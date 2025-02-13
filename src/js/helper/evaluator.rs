use std::marker::PhantomData;
use std::{borrow::Cow, sync::Arc};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallArgument, GetPropertiesParams, ReleaseObjectParams};
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
    pub fn new(
        page: Arc<PageInner>,
        target: EvalTarget<'a>,
        options: EvalOptions,
    ) -> Self {
        Self { page, target, options }
    }

    pub fn new_remote(
        page: Arc<PageInner>,
        object: impl AsJs<JsRemoteObject>,
        options: EvalOptions,
    ) -> Self {
        Self::new(page, EvalTarget::Object(object.as_js().clone()), options)
    }

    pub fn new_expr(
        page: Arc<PageInner>,
        expr: impl Into<JsExpr<'a>>,
        expr_context: Option<JsExprContext>,
        options: EvalOptions,
    ) -> Self {
        Self::new(page, EvalTarget::Expr(
            ExprTarget::new(expr.into(), expr_context)
        ), options)
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

        let value = serde::de::DeserializeSeed::deserialize(
            de::PageDeserializeSeed::new(self.page.clone(), PhantomData),
            self.eval_with_schema(schema).await?
        )?;
        Ok(value)
    }

    pub async fn eval_with_schema(self, schema: Schema) -> Result<JsonValue> {
        let params = self.target.into_params(
            self.page.clone(),
            None,
            schema,
            self.options
        ).await?;
        execute(self.page, params).await
    }

    pub fn invoke(self) -> FunctionInvoker<'a> {
        FunctionInvoker::new(self.page, self.target, self.options)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EvalTarget<'a> {
    Object(JsRemoteObject),
    Expr(ExprTarget<'a>),
}

#[derive(Debug, Clone)]
pub(crate) struct ExprTarget<'a> {
    // the expression to be evaluated
    pub(crate) expr: JsExpr<'a>,

    // the execution context for the expression
    pub(crate) context: Option<JsExprContext>,
}

impl<'a> ExprTarget<'a> {
    pub fn new(
        expr: JsExpr<'a>,
        context: Option<JsExprContext>,
    ) -> Self {
        Self { expr, context }
    }
}

const EVALUATOR: &'static str = include_str!("evaluator.js");
fn generate_function(expr_func: &str, expr_list: &[(JsonPointer, String)]) -> String {
    let expr_list = expr_list.into_iter()
        .map(|(path, expr)| {
            let path_json = serde_json::to_string(path).unwrap();
            format!(
                "{{ path: {}, value: (()=>({}))() }}",
                path_json,
                expr
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    EVALUATOR.replace(
        "__EXPR_FUNC__",
        expr_func
    ).replace(
        "__EXPR_LIST__",
        &expr_list
    )
}

impl<'a> EvalTarget<'a> {
    pub(crate) async fn into_params(
        self,
        page: Arc<PageInner>,
        invoke: Option<(Option<JsRemoteObject>, InvokeParams)>,
        schema: Schema,
        options: EvalOptions
    ) -> Result<CallFunctionOnParams> {
        let raw_params = match self {
            EvalTarget::Object(remote) => {
                let mut remotes = vec![remote.clone()];
                let mut expr_list = vec![];

                let (func_args, expr_func) = if let Some((this, params)) = invoke {
                    let func_args = params.into_arguments(this, &mut expr_list, &mut remotes)?;
                    let expr_func = "this".into();
                    (func_args, expr_func)
                } else {
                    let func_args = InvokeParams::default().into_arguments(None, &mut expr_list, &mut remotes)?;
                    let expr_func = "() => this".into();
                    (func_args, expr_func)
                };
                let expr_this = CallArgument::builder()
                    .object_id(remote.remote_id())
                    .build();
                let context = remote.into();

                RawParams {
                    expr_func,
                    expr_list,
                    expr_this,
                    func_args,
                    context,
                    remotes,
                }
            }
            EvalTarget::Expr(expr_target) => {
                let mut remotes = vec![];
                let mut expr_list = vec![];
                match &expr_target.context {
                    Some(JsExprContext::This(remote)) => {
                        remotes.push(remote.clone());
                    }
                    Some(JsExprContext::ContextObject(remote)) => {
                        remotes.push(remote.clone());
                    }
                    _ => (),
                }

                let (func_args, expr_func) = {
                    if let Some((this, params)) = invoke {
                        let func_args = params.into_arguments(this, &mut expr_list, &mut remotes)?;
                        let expr_func = expr_target.expr.into_inner();
                        (func_args, expr_func)
                    } else {
                        let func_args = InvokeParams::default().into_arguments(None, &mut expr_list, &mut remotes)?;
                        let expr_func = format!("()=>({})", expr_target.expr.into_inner()).into();
                        (func_args, expr_func)
                    }
                };
                let expr_this = expr_target.context.as_ref().map(|context| {
                    match context {
                        JsExprContext::This(remote) => {
                            CallArgument::builder()
                                .object_id(remote.remote_id())
                                .build()
                        }
                        _ => Default::default(),
                    }
                }).unwrap_or_default();
                let context = expr_target.context.into();
                RawParams {
                    expr_func,
                    expr_list,
                    expr_this,
                    func_args,
                    context,
                    remotes,
                }
            }
        };
        raw_params.into_params(page, schema, options).await
    }
}

enum EvalExecutionContext {
    None,
    Context(ExecutionContext),
    ContextObject(JsRemoteObject),
}

impl EvalExecutionContext {
    fn apply(self, mut params: CallFunctionOnParams) -> CallFunctionOnParams {
        match self {
            Self::None => params,
            Self::Context(ExecutionContext::Id(id)) => {
                params.execution_context_id = Some(id);
                params
            }
            Self::Context(ExecutionContext::UniqueId(unique_id)) => {
                params.unique_context_id = Some(unique_id);
                params
            }
            Self::ContextObject(remote) => {
                params.object_id = Some(remote.remote_id());
                params
            }
        }
    }

    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl From<JsRemoteObject> for EvalExecutionContext {
    fn from(remote: JsRemoteObject) -> Self {
        Self::ContextObject(remote)
    }
}

impl From<JsExprContext> for EvalExecutionContext {
    fn from(context: JsExprContext) -> Self {
        match context {
            JsExprContext::This(remote) | JsExprContext::ContextObject(remote) => {
                Self::from(remote)
            }
            JsExprContext::Context(context) => {
                Self::Context(context)
            }
        }
    }
}
impl From<Option<JsExprContext>> for EvalExecutionContext {
    fn from(context: Option<JsExprContext>) -> Self {
        match context {
            Some(context) => context.into(),
            None => Self::None,
        }
    }
}

struct RawParams<'a> {
    expr_func: Cow<'a, str>,
    expr_list: Vec<(JsonPointer, String)>,
    expr_this: CallArgument,
    func_args: Vec<CallArgument>,
    context: EvalExecutionContext,
    remotes: Vec<JsRemoteObject>,
}

impl<'a> RawParams<'a> {
    async fn into_params(
        self,
        page: Arc<PageInner>,
        schema: Schema,
        options: EvalOptions,
    ) -> Result<CallFunctionOnParams> {
        let context = {
            if self.context.is_none() {
                if let Some(remote_object) = self.remotes.first() {
                    EvalExecutionContext::ContextObject(remote_object.clone())
                } else {
                    let context_id = page.execution_context().await?
                        .ok_or_else(|| CdpError::msg("No execution context found"))?;
                    EvalExecutionContext::Context(context_id.into())
                }
            } else {
                self.context
            }
        };

        let params = CallFunctionOnParams::builder()
            .function_declaration(
                generate_function(
                    &self.expr_func,
                    &self.expr_list
                )
            )
            .return_by_value(false)
            .arguments(self.func_args)
            .argument(self.expr_this)
            .argument(
                CallArgument::builder()
                    .value(serde_json::to_value(options.await_promise)?)
                    .build()
            )
            .argument(
                CallArgument::builder()
                    .value(serde_json::to_value(schema)?)
                    .build()
            )
            .await_promise(options.await_promise)
            .user_gesture(options.user_gesture)
            .build()
            .unwrap();
        Ok(context.apply(params))
    }
}

pub(crate) async fn execute(page: Arc<PageInner>, params: CallFunctionOnParams) -> Result<JsonValue> {
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
            if let Some(id) = special.remote_id() {
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
