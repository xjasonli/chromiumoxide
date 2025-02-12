//! Provides functionality for invoking JavaScript functions in the browser context.
//! 
//! This module contains types and utilities for executing JavaScript functions with arguments
//! and handling their return values through the Chrome DevTools Protocol.

use std::marker::PhantomData;
use std::sync::Arc;

use chromiumoxide_cdp::cdp::js_protocol::runtime::CallArgument;
use schemars::Schema;

use crate::handler::PageInner;
use crate::error::Result;
use super::*;

/// A builder for invoking JavaScript functions with arguments.
/// 
/// This type provides a fluent interface for:
/// - Adding arguments to the function call
/// - Setting the function context (`this` value)
/// - Executing the function and handling its return value
/// 
/// # Example
/// ```no_run
/// # use chromiumoxide::js::FunctionInvoker;
/// # async fn example(invoker: FunctionInvoker) -> Result<(), Box<dyn std::error::Error>> {
/// let result = invoker
///     .argument(42)?           // Add a single argument
///     .argument("hello")?      // Add another argument
///     .invoke::<String>()      // Execute and get result as String
///     .await?;
/// # Ok(())
/// # }
/// ```
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

    /// Adds a single argument to the function call.
    /// 
    /// The argument must implement `NativeValueIntoJs` to be convertible to a JavaScript value.
    /// 
    /// # Arguments
    /// * `argument` - The value to pass as an argument
    /// 
    /// # Returns
    /// Returns self for method chaining
    pub fn argument<T: NativeValueIntoJs>(mut self, argument: T) -> Result<Self> {
        self.params.argument(argument)?;
        Ok(self)
    }

    /// Adds multiple arguments to the function call from a tuple.
    /// 
    /// The arguments must implement `FunctionNativeArgsIntoJs` which is automatically
    /// implemented for tuples of up to 10 elements where each element implements `NativeValueIntoJs`.
    /// 
    /// # Arguments
    /// * `arguments` - Tuple of arguments to pass to the function
    /// 
    /// # Returns
    /// Returns self for method chaining
    pub fn arguments<Args>(mut self, arguments: Args) -> Result<Self>
    where
        Args: FunctionNativeArgsIntoJs,
    {
        self.params.arguments(arguments)?;
        Ok(self)
    }

    /// Adds multiple arguments to the function call from an iterator, similar to JavaScript's spread syntax.
    /// 
    /// This method is analogous to JavaScript's spread operator (`...`) in function calls.
    /// For example, in JavaScript:
    /// ```javascript
    /// const args = [1, 2, 3];
    /// myFunction(...args);  // Spreads array into individual arguments
    /// ```
    /// 
    /// This method provides the same functionality in Rust:
    /// ```no_run
    /// # use chromiumoxide::js::FunctionInvoker;
    /// # async fn example(invoker: FunctionInvoker) -> Result<(), Box<dyn std::error::Error>> {
    /// let args = vec![1, 2, 3];
    /// invoker.arguments_spread(args)?  // Spreads iterator into individual arguments
    ///     .invoke::<()>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Arguments
    /// * `arguments` - Iterator of values to spread as individual arguments
    /// 
    /// # Returns
    /// Returns self for method chaining
    pub fn arguments_spread<I, T>(mut self, arguments: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: NativeValueIntoJs,
    {
        self.params.arguments_spread(arguments)?;
        Ok(self)
    }

    /// Executes the function with the configured arguments and converts the result.
    /// 
    /// The return type must implement `NativeValueFromJs` to be convertible from
    /// the JavaScript return value.
    /// 
    /// # Type Parameters
    /// * `T` - The expected return type
    /// 
    /// # Returns
    /// Returns the function's return value converted to type `T`
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
    // `this` context for the function
    pub(crate) this: Option<JsRemoteObject>,

    // Arguments to pass to the function
    pub(crate) arguments: Vec<JsonValue>,

    // Remote object references used in the call
    pub(crate) remotes: Vec<JsRemoteObject>,
}

impl InvokeParams {
    /// Creates a new parameter set with the given `this` context.
    pub fn new(this: Option<JsRemoteObject>) -> Self {
        let mut remotes = vec![];
        if let Some(this) = &this {
            remotes.push(this.clone());
        }
        Self { this, arguments: vec![], remotes }
    }

    /// Adds a single argument value.
    pub fn argument<T: NativeValueIntoJs>(&mut self, value: T) -> Result<()> {
        self.arguments.push(serde_json::to_value(value)?);
        Ok(())
    }

    /// Adds multiple arguments from a tuple.
    pub fn arguments<Args>(&mut self, arguments: Args) -> Result<()>
    where
        Args: FunctionNativeArgsIntoJs,
    {
        self.arguments.extend(Args::into_json_values(arguments)?);
        Ok(())
    }

    /// Adds multiple arguments from an iterator.
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

    /// Converts the parameters into CDP call arguments.
    pub fn into_arguments(self, expr_list: &mut Vec<(helper::JsonPointer, String)>, remotes: &mut Vec<JsRemoteObject>) -> Result<Vec<CallArgument>> {
        let mut args = vec![];

        let mut descriptors = vec![];
        let mut specials = vec![];
        for (i, arg) in self.arguments.into_iter().enumerate() {
            let prefix = [helper::JsonPointerSegment::Index(i)];
            let (descriptor, special) = helper::ValueDescriptor::parse_with_expr(
                arg,
                &prefix,
                expr_list
            );
            descriptors.push(descriptor);
            specials.extend(special);
        }
        for special in specials {
            args.push(special.into_call_argument());
        }
        args.push(CallArgument{
            value: Some(serde_json::to_value(descriptors)?),
            ..Default::default()
        });
        args.push(CallArgument{
            object_id: self.this.map(|v| v.remote_id()),
            ..Default::default()
        });

        remotes.extend(self.remotes);
        Ok(args)
    }
}
