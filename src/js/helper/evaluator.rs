use std::marker::PhantomData;
use std::{borrow::Cow, sync::Arc};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallArgument, GetPropertiesParams, ReleaseObjectParams};
use schemars::Schema;

use crate::handler::PageInner;
use crate::error::{CdpError, Result};
use super::*;

pub(crate) struct Evaluator {
    page: Arc<PageInner>,
    target: EvalTarget,
    options: EvalOptions,
}

impl Evaluator {
    pub fn new(
        page: Arc<PageInner>,
        target: EvalTarget,
        options: EvalOptions,
    ) -> Self {
        Self { page, target, options }
    }

    pub fn new_remote(
        page: Arc<PageInner>,
        remote: impl Into<JsRemoteObject>,
        options: EvalOptions,
    ) -> Self {
        Self::new(page, EvalTarget::Remote(remote.into()), options)
    }

    pub fn new_expr(
        page: Arc<PageInner>,
        expr: impl Into<JsExpr>,
        this: Option<JsRemoteObject>,
        context: Option<ExecutionContext>,
        options: EvalOptions,
    ) -> Self {
        Self::new(page, EvalTarget::Expr(
            Expr::new(expr.into().into(), this, context)
        ), options)
    }

    pub async fn eval<T>(self) -> Result<T>
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

    pub fn invoke(self, this: Option<&JsRemoteObject>) -> FunctionInvoker {
        FunctionInvoker::new(self.page, self.target, this.map(|t| t.clone()), self.options)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EvalTarget {
    Remote(JsRemoteObject),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub(crate) struct Expr {
    // the expression to be evaluated
    pub(crate) expr: String,

    // the `this` value for the expression
    pub(crate) this: Option<JsRemoteObject>,

    // execution context for the expression
    pub(crate) context: Option<ExecutionContext>,

}
impl Expr {
    pub fn new(
        expr: String,
        this: Option<JsRemoteObject>,
        context: Option<ExecutionContext>,
    ) -> Self {
        Self { expr, this, context }
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

impl EvalTarget {
    pub(crate) async fn into_params(
        self,
        page: Arc<PageInner>,
        invoke: Option<InvokeParams>,
        schema: Schema,
        options: EvalOptions
    ) -> Result<CallFunctionOnParams> {
        let raw_params = match self {
            EvalTarget::Remote(remote) => {
                let mut remotes = vec![remote.clone()];
                let mut expr_list = vec![];

                let (func_args, expr_func) = if let Some(invoke) = invoke {
                    let func_args = invoke.into_arguments(&mut expr_list, &mut remotes)?;
                    let expr_func = "this".into();
                    (func_args, expr_func)
                } else {
                    let func_args = InvokeParams::default().into_arguments(&mut expr_list, &mut remotes)?;
                    let expr_func = "() => this".into();
                    (func_args, expr_func)
                };
                let expr_this = CallArgument::builder()
                    .object_id(remote.remote_id())
                    .build();
                let context = Some(remote.into());

                RawParams {
                    expr_func,
                    expr_list,
                    expr_this,
                    func_args,
                    context,
                    remotes,
                }
            }
            EvalTarget::Expr(expr) => {
                let mut remotes = vec![];
                let mut expr_list = vec![];
                if let Some(ExecutionContext::RemoteObject(remote_object)) = &expr.context {
                    remotes.push(remote_object.clone());
                }
                if let Some(this) = &expr.this {
                    remotes.push(this.clone());
                }

                let (func_args, expr_func) = if let Some(invoke) = invoke {
                    let func_args = invoke.into_arguments(&mut expr_list, &mut remotes)?;
                    let expr_func = expr.expr.into();
                    (func_args, expr_func)
                } else {
                    let func_args = InvokeParams::default().into_arguments(&mut expr_list, &mut remotes)?;
                    let expr_func = format!("()=>({})", expr.expr).into();
                    (func_args, expr_func)
                };
                let expr_this = expr.this.as_ref().map(|this| {
                    CallArgument::builder()
                        .object_id(this.remote_id())
                        .build()
                }).unwrap_or_default();
                let context = expr.context;
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

struct RawParams {
    expr_func: Cow<'static, str>,
    expr_list: Vec<(JsonPointer, String)>,
    expr_this: CallArgument,
    func_args: Vec<CallArgument>,
    context: Option<ExecutionContext>,
    remotes: Vec<JsRemoteObject>,
}

impl RawParams {
    async fn into_params(
        self,
        page: Arc<PageInner>,
        schema: Schema,
        options: EvalOptions,
    ) -> Result<CallFunctionOnParams> {
        let context = {
            if let Some(context) = self.context {
                context
            } else if let Some(remote_object) = self.remotes.first() {
                ExecutionContext::RemoteObject(remote_object.clone())
            } else {
                let context_id = page.execution_context().await?
                    .ok_or_else(|| CdpError::msg("No execution context found"))?;
                ExecutionContext::Id(context_id)
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
