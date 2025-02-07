use std::marker::PhantomData;
use std::sync::Arc;

use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallArgument, RemoteObjectId};
use schemars::Schema;

use crate::handler::PageInner;
use crate::error::Result;
use super::*;

#[derive(Debug, Clone)]
pub struct FunctionInvoker {
    page: Arc<PageInner>,
    target: helper::EvalTarget,
    params: InvokeParams,
    options: EvalOptions,
}

impl FunctionInvoker {
    pub(crate) fn new(
        page: Arc<PageInner>,
        target: helper::EvalTarget,
        this: Option<JsRemoteObject>,
        options: EvalOptions
    ) -> Self {
        Self { page, target, params: InvokeParams::new(this), options }
    }

    pub fn argument<T: NativeValueIntoJs>(mut self, argument: T) -> Result<Self> {
        self.params.argument(argument)?;
        Ok(self)
    }

    pub fn arguments<Args>(mut self, arguments: Args) -> Result<Self>
    where
        Args: FunctionNativeArgsIntoJs,
    {
        self.params.arguments(arguments)?;
        Ok(self)
    }

    pub fn arguments_spread<I, T>(mut self, arguments: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: NativeValueIntoJs,
    {
        self.params.arguments_spread(arguments)?;
        Ok(self)
    }

    pub async fn invoke<T>(self) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        let schema = {
            let mut settings = schemars::generate::SchemaSettings::default();
            settings.inline_subschemas = true;
            settings.into_generator().into_root_schema_for::<T>()
        };

        let value = serde::de::DeserializeSeed::deserialize(
            de::PageDeserializeSeed::new(self.page.clone(), PhantomData),
            self.invoke_with_schema(schema).await?
        )?;
        Ok(value)
    }

    pub(crate) async fn invoke_with_schema(self, schema: Schema) -> Result<JsonValue> {
        let params = self.target.into_params(
            self.page.clone(),
            Some(self.params),
            schema,
            self.options
        ).await?;
        helper::execute(self.page, params).await
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct InvokeParams {
    // `this` for the function
    pub(crate) this: Option<JsRemoteObject>,

    // arguments for the function
    pub(crate) arguments: Vec<JsonValue>,

    // remote objects in arguments
    pub(crate) remotes: Vec<RemoteObjectId>,
}

impl InvokeParams {
    pub fn new(this: Option<JsRemoteObject>) -> Self {
        let mut remotes = vec![];
        if let Some(this) = &this {
            remotes.push(this.object_id());
        }
        Self { this, arguments: vec![], remotes }
    }

    pub fn argument<T: NativeValueIntoJs>(&mut self, value: T) -> Result<()> {
        self.arguments.push(serde_json::to_value(value)?);
        Ok(())
    }

    pub fn arguments<Args>(&mut self, arguments: Args) -> Result<()>
    where
        Args: FunctionNativeArgsIntoJs,
    {
        self.arguments.extend(Args::into_json_values(arguments)?);
        Ok(())
    }

    pub fn arguments_spread<I, T>(&mut self, arguments: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: NativeValueIntoJs,
    {
        self.arguments.extend(
            arguments.into_iter()
                .map(|arg| -> Result<_> { Ok(serde_json::to_value(arg)?) } )
                .collect::<Result<Vec<_>>>()?
        );
        Ok(())
    }

    pub fn into_arguments(self, expr_list: &mut Vec<String>, remotes: &mut Vec<RemoteObjectId>) -> Result<Vec<CallArgument>> {
        let mut args = vec![];

        let mut descriptors = vec![];
        let mut specials = vec![];
        for arg in self.arguments {
            let (descriptor, special) = helper::ValueDescriptor::parse(arg);
            descriptors.push(descriptor);
            specials.extend(special);
        }
        for special in specials {
            args.push(special.into_call_argument(expr_list));
        }
        args.push(CallArgument{
            value: Some(serde_json::to_value(descriptors)?),
            ..Default::default()
        });
        args.push(CallArgument{
            object_id: self.this.map(|v| v.object_id()),
            ..Default::default()
        });

        remotes.extend(self.remotes);
        Ok(args)
    }
}
