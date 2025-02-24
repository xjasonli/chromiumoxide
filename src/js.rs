use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

use chromiumoxide_cdp::cdp::js_protocol::runtime::{
    CallFunctionOnParams, EvaluateParams, ExecutionContextId, RemoteObject
};

use crate::utils::is_likely_js_function;

pub(crate) mod helper;
pub mod ser;
pub mod de;
pub mod js_any;
pub mod js_args;
pub mod into_js;
pub mod nullable;
pub mod optional;
pub mod remote_object;
pub mod bigint;
pub mod expr;
pub mod function_invoker;
pub mod exposed_function;

pub use js_any::*;
pub use js_args::*;
pub use into_js::*;
pub use nullable::*;
pub use optional::*;
pub use remote_object::*;
pub use bigint::*;
pub use expr::*;
pub use function_invoker::*;
pub use exposed_function::*;

#[derive(Debug, Clone)]
pub struct EvaluationResult {
    /// Mirror object referencing original JavaScript object
    inner: RemoteObject,
}

impl EvaluationResult {
    pub fn new(inner: RemoteObject) -> Self {
        Self { inner }
    }

    pub fn object(&self) -> &RemoteObject {
        &self.inner
    }

    pub fn into_object(self) -> RemoteObject {
        self.inner
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.object().value.as_ref()
    }

    /// Attempts to deserialize the value into the given type
    pub fn into_value<T: DeserializeOwned>(self) -> serde_json::Result<T> {
        let value = self
            .inner
            .value
            .ok_or_else(|| serde::de::Error::custom("No value found"))?;
        serde_json::from_value(value)
    }
}

#[derive(Debug, Clone)]
pub enum Evaluation {
    Expression(EvaluateParams),
    Function(CallFunctionOnParams),
}

impl From<&str> for Evaluation {
    fn from(expression: &str) -> Self {
        if is_likely_js_function(expression) {
            CallFunctionOnParams::from(expression).into()
        } else {
            EvaluateParams::from(expression).into()
        }
    }
}

impl From<String> for Evaluation {
    fn from(expression: String) -> Self {
        expression.as_str().into()
    }
}

impl From<EvaluateParams> for Evaluation {
    fn from(params: EvaluateParams) -> Self {
        Evaluation::Expression(params)
    }
}

impl From<CallFunctionOnParams> for Evaluation {
    fn from(params: CallFunctionOnParams) -> Self {
        Evaluation::Function(params)
    }
}


/// Configuration options for evaluating JavaScript code in the browser context.
///
/// This struct provides options that control how JavaScript code is executed,
/// including handling of promises and user gestures.
#[derive(Debug, Copy, Clone)]
pub struct EvalOptions {
    /// Whether to wait for any returned Promise to resolve before completing the evaluation.
    /// When true, if the evaluated code returns a Promise, the evaluation will wait for it to resolve.
    pub await_promise: bool,

    /// Whether to treat the evaluation as triggered by a user gesture.
    /// Some browser APIs require user gestures to work (like requestFullscreen).
    pub user_gesture: bool,
}

impl EvalOptions {
    /// Creates a new `EvalOptions` with default settings.
    ///
    /// By default:
    /// - `await_promise` is true (waits for Promises to resolve)
    /// - `user_gesture` is true (simulates user gesture)
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to wait for any returned Promise to resolve.
    ///
    /// # Arguments
    /// * `await_promise` - If true, waits for any returned Promise to resolve
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn await_promise(mut self, await_promise: bool) -> Self {
        self.await_promise = await_promise;
        self
    }

    /// Sets whether to treat the evaluation as triggered by a user gesture.
    ///
    /// # Arguments
    /// * `user_gesture` - If true, simulates a user gesture context
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn user_gesture(mut self, user_gesture: bool) -> Self {
        self.user_gesture = user_gesture;
        self
    }
}

impl Default for EvalOptions {
    /// Creates a new `EvalOptions` with default values:
    /// - `await_promise`: true
    /// - `user_gesture`: true
    fn default() -> Self {
        Self {
            await_promise: true,
            user_gesture: true,
        }
    }
}

/// Configuration options for evaluating JavaScript code.
///
/// This struct provides options that control how JavaScript code is executed,
/// including handling of promises and user gestures.
/// 
/// This struct is used to evaluate JavaScript code.
/// 
/// The `expr` is the JavaScript expression to evaluate.
/// 
/// The `options` are the options for the evaluation.
#[derive(Debug, Clone)]
pub struct EvalParams<'a> {
    /// The expression to evaluate
    pub expr: JsExpr<'a>,

    /// The options for the evaluation
    pub options: EvalOptions,
}

impl<'a> EvalParams<'a> {
    /// Creates a new `EvalParams` with default settings.
    ///
    /// By default:
    /// - `options` are the default evaluation options
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), options: EvalOptions::default() }
    }

    /// Sets the options for the evaluation
    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }

    /// Converts the `EvalParams` into a `GlobalEvalParams`
    pub fn into_global(self) -> GlobalEvalParams<'a> {
        GlobalEvalParams::new(self.expr).options(self.options)
    }

    /// Converts the `EvalParams` into a `ScopedEvalParams`
    pub fn into_scoped(self) -> ScopedEvalParams<'a> {
        ScopedEvalParams::new(self.expr).options(self.options)
    }
}

impl<'a, T: Into<JsExpr<'a>>> From<T> for EvalParams<'a> {
    fn from(expr: T) -> Self {
        EvalParams::new(expr)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalEvalParams<'a> {
    /// The expression to evaluate
    pub expr: JsExpr<'a>,

    /// The options for the evaluation
    pub options: EvalOptions,

    /// The execution context for the evaluation
    pub execution_context_id: Option<ExecutionContextId>,
}

impl<'a> GlobalEvalParams<'a> {
    /// Creates a new `GlobalEvalParams` with default settings.
    ///
    /// By default:
    /// - `execution_context_id` is None (global execution context)
    /// - `options` are the default evaluation options
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), execution_context_id: None, options: EvalOptions::default() }
    }

    /// Sets the options for the evaluation
    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }

    /// Sets the execution context (identified by an id) for the evaluation
    pub fn execution_context_id<T: Into<ExecutionContextId>>(self, execution_context_id: T) -> Self {
        Self { execution_context_id: Some(execution_context_id.into()), ..self }
    }

    /// Sets the execution context (identified by a remote object) for the evaluation
    pub fn execution_context_object<T: Into<JsRemoteObject>>(self, execution_context_object: T) -> Self {
        self.execution_context_id(execution_context_object.into().execution_context_id())
    }

    /// Converts the `GlobalEvalParams` into a `ScopedEvalParams`
    pub fn into_scoped(self) -> ScopedEvalParams<'a> {
        let scoped = ScopedEvalParams::new(self.expr)
            .options(self.options);
        if let Some(execution_context_id) = self.execution_context_id {
            scoped.execution_context_id(execution_context_id)
        } else {
            scoped
        }
    }
}

impl<'a, T: Into<JsExpr<'a>>> From<T> for GlobalEvalParams<'a> {
    fn from(expr: T) -> Self {
        GlobalEvalParams::new(expr)
    }
}

/// Configuration options for evaluating JavaScript code in a specific execution context.
///
/// This struct provides options that control how JavaScript code is executed,
/// including handling of promises and user gestures.
/// 
/// This struct is used to evaluate JavaScript code in a specific execution context.
/// 
/// The `execution_context_id` is the id of the execution context to evaluate the code in.
/// 
/// The `this` value is the value of the `this` keyword in the evaluated code.
/// 
/// The `options` are the options for the evaluation.
#[derive(Debug, Clone)]
pub struct ScopedEvalParams<'a> {
    /// The expression to evaluate
    pub expr: JsExpr<'a>,

    /// The options for the evaluation
    pub options: EvalOptions,

    /// The execution context for the evaluation
    pub execution_context_id: Option<ExecutionContextId>,

    /// The `this` value for the evaluation
    pub this: Option<DynIntoJsAny<'a>>,
}

impl<'a> ScopedEvalParams<'a> {
    /// Creates a new `ScopedEvalParams` with default settings.
    ///
    /// By default:
    /// - `this` is None (no `this` value)
    /// - `execution_context_id` is None (global execution context)
    /// - `options` are the default evaluation options
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), this: None, execution_context_id: None, options: EvalOptions::default() }
    }

    /// Sets the options for the evaluation
    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }

    /// Sets the execution context (identified by an id) for the evaluation
    pub fn execution_context_id<U: Into<ExecutionContextId>>(self, execution_context_id: U) -> Self {
        Self { execution_context_id: Some(execution_context_id.into()), ..self }
    }

    /// Sets the execution context (identified by a remote object) for the evaluation
    pub fn execution_context_object<T: Into<JsRemoteObject>>(self, execution_context_object: T) -> Self {
        self.execution_context_id(execution_context_object.into().execution_context_id())
    }

    /// Sets the `this` value for the evaluation
    pub fn this<T: IntoJsAny + 'a>(self, this: T) -> Self {
        Self { this: Some(std::sync::Arc::new(this)), ..self }
    }
}

impl<'a, T: Into<JsExpr<'a>>> From<T> for ScopedEvalParams<'a> {
    fn from(expr: T) -> Self {
        ScopedEvalParams::new(expr)
    }
}
