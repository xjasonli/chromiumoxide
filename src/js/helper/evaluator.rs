use std::marker::PhantomData;
use std::sync::Arc;
use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallArgument, CallFunctionOnReturns, GetPropertiesParams, ReleaseObjectParams};
use schemars::Schema;

use crate::handler::PageInner;
use crate::error::{CdpError, Result};
use super::*;

pub(crate) struct Evaluator<'a> {
    page: Arc<PageInner>,
    target: EvalTarget<'a>,
    options: EvalOptions,
}

impl<'a> Evaluator<'a> {
    pub fn new_object(
        page: Arc<PageInner>,
        object: impl AsJs<JsRemoteObject>,
        options: EvalOptions,
    ) -> Self {
        let target = EvalTarget::new_with_object(object);
        Self { page, target, options }
    }

    pub fn new_expr(
        page: Arc<PageInner>,
        expr: impl Into<JsExpr<'a>>,
        execution_context_id: Option<ExecutionContextId>,
        execution_context_object: Option<BoxedIntoJs<'a>>,
        this: Option<BoxedIntoJs<'a>>,
        options: EvalOptions,
    ) -> Self {
        let target = EvalTarget::new(expr.into(), execution_context_id, execution_context_object, this);
        Self { page, target, options }
    }

    pub async fn eval<T>(self) -> Result<T>
    where
        T: FromJs,
    {
        let schema = {
            let mut settings = schemars::generate::SchemaSettings::default();
            settings.inline_subschemas = true;
            settings.into_generator().into_root_schema_for::<T>()
        };
        let (value, execution_context_id) = evaluate(
            self.page.clone(),
            self.target,
            None,
            schema,
            self.options
        ).await?;

        let value = serde::de::DeserializeSeed::deserialize(
            de::JsDeserializeSeed::new(
                JsRemoteObjectCtx::new(self.page.clone(), execution_context_id),
                PhantomData,
            ),
            value
        )?;
        Ok(value)
    }

    pub fn invoke(self) -> FunctionInvoker<'a> {
        FunctionInvoker::new(self.page, self.target, self.options)
    }
}

#[derive(Debug)]
pub(crate) struct EvalTarget<'a> {
    // the expression to be evaluated
    pub(crate) expr: JsExpr<'a>,

    // the execution context for the expression
    pub(crate) execution_context_id: Option<ExecutionContextId>,

    // the execution context (identified by a remote object) for the expression
    pub(crate) execution_context_object: Option<BoxedIntoJs<'a>>,

    // the this value for the expression
    pub(crate) this: Option<BoxedIntoJs<'a>>,
}

impl<'a> EvalTarget<'a> {
    pub fn new_with_object(
        object: impl AsJs<JsRemoteObject>,
    ) -> Self {
        let object = object.as_js().clone();
        Self {
            expr: "this".into(),
            execution_context_id: Some(object.execution_context_id()),
            execution_context_object: None,
            this: Some(Box::new(object)),
        }
    }

    pub fn new(
        expr: JsExpr<'a>,
        execution_context_id: Option<ExecutionContextId>,
        execution_context_object: Option<BoxedIntoJs<'a>>,
        this: Option<BoxedIntoJs<'a>>,
    ) -> Self {
        Self { expr, execution_context_id, execution_context_object, this }
    }

    pub(crate) async fn into_params(
        mut self,
        page: Arc<PageInner>,
        invoke: Option<InvokeParams<'a>>,
        schema: Schema,
        mode: ReturnMode,
        options: EvalOptions
    ) -> Result<(CallFunctionOnParams, ExecutionContextId)> {
        let mut objects = vec![];
        let mut call_arguments = vec![];

        call_arguments.push(CallArgument::builder()
            .value(serde_json::to_value(EvaluateConfig {
                return_mode: mode,
                await_promise: options.await_promise,
                schema,
            })?)
            .build()
        );

        // extracts the execution context id from the execution context object
        if let Some(execution_context_object) = self.execution_context_object {
            let object = Argument::new(execution_context_object)?;
            if let Some(execution_context_id) = object.execution_context_id {
                self.execution_context_id.get_or_insert(execution_context_id);
            } else {
                // Should we return an error here?
            }
        }

        let this_exprs = {
            let this = Argument::new(self.this)?;
            call_arguments.push(CallArgument::builder()
                .value(serde_json::to_value(this.descriptor)?)
                .build()
            );
            this.specials.into_iter().for_each(|special| {
                if let Some(id) = special.remote_object_id() {
                    objects.push(id);
                }
                call_arguments.push(special.into_call_argument());
            });
            if let Some(execution_context_id) = this.execution_context_id {
                self.execution_context_id.get_or_insert(execution_context_id);
            }
            this.exprs
        };

        let args_exprs = {
            if let Some(invoke) = invoke {
                let args = Argument::new(invoke)?;
                call_arguments.push(CallArgument::builder()
                    .value(serde_json::to_value(args.descriptor)?)
                    .build()
                );
                args.specials.into_iter().for_each(|special| {
                    if let Some(id) = special.remote_object_id() {
                        objects.push(id);
                    }
                    call_arguments.push(special.into_call_argument());
                });
                if let Some(execution_context_id) = args.execution_context_id {
                    self.execution_context_id.get_or_insert(execution_context_id);
                }
                args.exprs
            } else {
                Vec::new()
            }
        };

        let execution_context_id = if let Some(execution_context_id) = self.execution_context_id {
            execution_context_id
        } else {
            page.execution_context().await?
                .ok_or(CdpError::msg("No execution context found"))?
        };

        let function = generate_function(&this_exprs, self.expr.as_str(), &args_exprs);

        let return_by_value = match mode {
            ReturnMode::Null => true,
            ReturnMode::Undefined => true,
            ReturnMode::ByValue => true,
            _ => false,
        };

        let params = CallFunctionOnParams::builder()
            .function_declaration(function)
            .execution_context_id(execution_context_id)
            .await_promise(options.await_promise)
            .user_gesture(options.user_gesture)
            .return_by_value(return_by_value)
            .arguments(call_arguments)
            .build()
            .expect("build CallFunctionOnParams should be infallible");

        Ok((params, execution_context_id))
    }
}


const EVALUATOR: &'static str = include_str!("evaluator.js");
fn generate_function(
    this_exprs: &[(JsonPointer, String)],
    expr: &str,
    args_exprs: &[(JsonPointer, String)],
) -> String {
    fn expr_to_code(path: &JsonPointer, expr: &str) -> String {
        let path_json = serde_json::to_string(path)
            .expect("string serializing is infallible");
        format!(
            "{{ path: {}, value: (()=>({}))() }}",
            path_json,
            expr
        )
    }
    fn exprs_to_code(exprs: &[(JsonPointer, String)]) -> String {
        exprs.into_iter()
            .map(|(path, expr)| expr_to_code(path, expr))
            .collect::<Vec<_>>()
            .join(",")
    }

    let this_exprs = exprs_to_code(this_exprs);
    let args_exprs = exprs_to_code(args_exprs);

    EVALUATOR.replace("__THIS_EXPRS__", &this_exprs)
        .replace("__EXPR__", expr)
        .replace("__ARGS_EXPRS__", &args_exprs)
}

#[derive(Debug, Clone, Default)]
pub struct Argument {
    pub descriptor: ValueDescriptor,
    pub specials: Vec<SpecialValue>,
    pub exprs: Vec<(JsonPointer, String)>,
    pub execution_context_id: Option<ExecutionContextId>,
}

impl Argument {
    pub fn new<T: IntoJs>(value: T) -> Result<Self> {
        let ctx = ser::JsSerializerCtx::default();
        let serializer = ser::JsJsonSerializer::new_json_serializer(ctx.clone());
        let mut exprs = vec![];
        let (descriptor, specials) = ValueDescriptor::parse_with_expr(
            value.serialize(serializer)?,
            &mut exprs,
            &[],
        );
        let execution_context_id = ctx.borrow()
            .first()
            .map(|object| object.execution_context_id());
        Ok(Self { descriptor, specials, exprs, execution_context_id })
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ReturnMode {
    // Simple case, only return null
    //
    // all other result will be discarded and converted to null
    // this is a special case of `ByValue`, with pre-process logic in javascript
    Null,

    // Simple case, only return undefined
    //
    // all other result will be discarded and converted to undefined
    // this is a special case of `ByValue`, with pre-process logic in javascript
    Undefined,

    // Simple case, return the result by value
    //
    // Maybe:
    // * RemoteObject.type == "object" with RemoteObject.value for json
    //
    // * RemoteObject.type == "undefined" for undefined
    // * RemoteObject.type == "bigint" with RemoteObject.unserializedValue for bigint
    ByValue,

    // Simple case, return the result by id
    //
    // Maybe:
    // * RemoteObject.type == "object" with RemoteObject.objectId
    // * RemoteObject.type == "function" with RemoteObject.objectId
    // * RemoteObject.type == "symbol" with RemoteObject.objectId
    //
    // * RemoteObject.type == "undefined" for undefined
    // * RemoteObject.type == "bigint" with RemoteObject.unserializedValue for bigint
    ById,

    // return the result as a Descriptor
    Complex,
}

impl ReturnMode {
    fn from_schema(schema: &Schema) -> Self {
        schema::parse_json_schema(schema)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateConfig {
    /// Whether to return a complex value.
    return_mode: ReturnMode,

    /// Whether to wait for the promise to resolve.
    await_promise: bool,

    /// The schema to use for the result.
    schema: Schema,
}

pub(crate) async fn evaluate<'a>(
    page: Arc<PageInner>,
    target: EvalTarget<'a>,
    invoke: Option<InvokeParams<'a>>,
    schema: Schema,
    options: EvalOptions,
) -> Result<(JsonValue, ExecutionContextId)> {
    let mode = ReturnMode::from_schema(&schema);
    let (params, execution_context_id) = target.into_params(
        page.clone(),
        invoke,
        schema,
        mode,
        options,
    ).await?;

    let value = if let ReturnMode::Complex = mode {
        execute_complex(page, params).await?
    } else {
        execute_simple(page, params).await?
    };
    Ok((value, execution_context_id))
}

async fn execute_simple(page: Arc<PageInner>, params: CallFunctionOnParams) -> Result<JsonValue> {
    let CallFunctionOnReturns {
        exception_details,
        result,
    } = page.execute(params).await?.result;

    if let Some(exception) = exception_details {
        return Err(CdpError::JavascriptException(Box::new(exception)));
    }
    parse_remote_object(page, result).await
}

async fn execute_complex(page: Arc<PageInner>, params: CallFunctionOnParams) -> Result<JsonValue> {
    let mut guard = RemoteValuesGuard::new(page.clone());
    let resp = page.execute(params).await?.result;
    if let Some(exception) = resp.exception_details {
        return Err(CdpError::JavascriptException(Box::new(exception)));
    }
    let Some(result_id) = resp.result.object_id else {
        return Err(CdpError::UnexpectedValue(format!("Invalid result: {:#?}", resp.result)));
    };

    guard.add(result_id.clone());

    let descriptor: ValueDescriptor = {
        let params = CallFunctionOnParams::builder()
            .function_declaration("function() { return this.descriptor; }")
            .object_id(result_id.clone())
            .return_by_value(true)
            .build().unwrap();
        let resp = page.execute(params).await?.result;
        if let Some(exception) = resp.exception_details {
            return Err(CdpError::JavascriptException(Box::new(exception)));
        }
        let Some(value) = resp.result.value else {
            return Err(CdpError::UnexpectedValue(format!("Invalid descriptor: {:#?}", resp.result)));
        };
        serde_json::from_value(value)
            .map_err(|e| CdpError::UnexpectedValue(format!("Invalid descriptor: {:#?}", e)))?
    };

    if descriptor.paths.is_empty() {
        Ok(descriptor.value)
    } else {
        let array_id = {
            let params = CallFunctionOnParams::builder()
                .function_declaration("function() { return this.specials; }")
                .object_id(result_id)
                .return_by_value(false)
                .build().unwrap();
            let resp = page.execute(params).await?.result;
            if let Some(exception) = resp.exception_details {
                return Err(CdpError::JavascriptException(Box::new(exception)));
            }
            let Some(array_id) = resp.result.object_id else {
                return Err(CdpError::UnexpectedValue(format!("Invalid specials: {:#?}", resp.result)));
            };
            array_id
        };
        guard.add(array_id.clone());

        let mut properties = {
            let params = GetPropertiesParams::builder()
                .object_id(array_id)
                .own_properties(true)
                .build().unwrap();
            let resp = page.execute(params).await?.result;
            if let Some(exception) = resp.exception_details {
                return Err(CdpError::JavascriptException(Box::new(exception)));
            }
            resp.result
        };

        let length = {
            let length_index = properties.iter().enumerate()
                .find(|(_, p)| p.name == "length")
                .map(|(idx, _)| idx)
                .unwrap_or(0);
            properties.remove(length_index)
                .value
                .map(|v| v.value.unwrap_or_default())
                .unwrap_or_default()
                .as_u64()
                .unwrap_or(0) as usize
        };

        if length != descriptor.paths.len() {
            return Err(CdpError::UnexpectedValue(format!("Invalid descriptor paths length: {:#?}", descriptor)));
        }

        let mut specials = Vec::new();
        specials.reserve(length);
        let mut items_guard = RemoteValuesGuard::new(page.clone());

        for idx in 0..length {
            let property_index = properties.iter().enumerate()
                .find(|(_, p)| p.name == format!("{}", idx))
                .map(|(idx, _)| idx)
                .ok_or(CdpError::UnexpectedValue(format!("Invalid properties: {:#?}", properties)))?;
            let property = properties.remove(property_index);
            let Some(remote_object) = property.value else {
                return Err(CdpError::UnexpectedValue(format!("Invalid properties: {:#?}", properties)));
            };
            let special = SpecialValue::from_remote_object(&page, remote_object).await?;
            if let Some(id) = special.remote_object_id() {
                items_guard.add(id.clone());
            }
            specials.push(special);
        }
        let value = descriptor.merge(specials)?;
        items_guard.clear();
        Ok(value)
    }
}

struct RemoteValuesGuard {
    page: Arc<PageInner>,
    values: Vec<RemoteObjectId>,
}

impl RemoteValuesGuard {
    pub fn new(page: Arc<PageInner>) -> Self {
        Self { page, values: vec![] }
    }

    pub fn add(&mut self, value: RemoteObjectId) {
        self.values.push(value);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl Drop for RemoteValuesGuard {
    fn drop(&mut self) {
        for value in self.values.drain(..) {
            let _ = self.page.execute_no_wait(
                ReleaseObjectParams::builder()
                    .object_id(value)
                    .build().unwrap()
            );
        }
    }
}
