use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

use chromiumoxide_cdp::cdp::js_protocol::runtime::{
    CallFunctionOnParams, EvaluateParams, RemoteObject,
};

use crate::utils::is_likely_js_function;

pub(crate) mod helper;
pub mod de;
pub mod any;
pub mod traits;
pub mod remote_object;
pub mod bigint;
pub mod primitives;
pub mod expr;
pub mod function_invoker;
pub mod optional;
pub mod undefined;
pub mod exposed_function;
pub mod execution_context;

pub use any::*;
pub use traits::*;
pub use remote_object::*;
pub use bigint::*;
//pub use primitives::*;
pub use expr::*;
pub use function_invoker::*;
pub use optional::*;
pub use undefined::*;
pub use exposed_function::*;
pub use execution_context::*;

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

#[derive(Debug, Clone)]
pub struct EvalParams<'a> {
    pub expr: JsExpr<'a>,
    pub options: EvalOptions,
}

impl<'a> EvalParams<'a> {
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), options: EvalOptions::default() }
    }

    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }

    pub fn into_scoped(self) -> ScopedEvalParams<'a> {
        ScopedEvalParams::new(self.expr).options(self.options)
    }

    pub fn into_global(self) -> GlobalEvalParams<'a> {
        GlobalEvalParams::new(self.expr).options(self.options)
    }
}

impl<'a, T: Into<JsExpr<'a>>> From<T> for EvalParams<'a> {
    fn from(expr: T) -> Self {
        EvalParams::new(expr)
    }
}

#[derive(Debug, Clone)]
pub struct ScopedEvalParams<'a> {
    pub expr: JsExpr<'a>,
    pub expr_context: Option<JsExprContext>,
    pub options: EvalOptions,
}

impl<'a> ScopedEvalParams<'a> {
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), expr_context: None, options: EvalOptions::default() }
    }

    pub fn expr_this<T: AsJs<JsRemoteObject>>(self, this: T) -> Self {
        let context = JsExprContext::new_with_this(this);
        Self { expr_context: Some(context), ..self }
    }

    pub fn expr_execution_context<T: Into<ExecutionContext>>(self, context: T) -> Self {
        let context = JsExprContext::new_with_context(context);
        Self { expr_context: Some(context), ..self }
    }

    pub fn expr_execution_context_object<T: AsJs<JsRemoteObject>>(self, context: T) -> Self {
        let context = JsExprContext::new_with_context_object(context);
        Self { expr_context: Some(context), ..self }
    }

    pub fn expr_context<T: Into<JsExprContext>>(self, context: T) -> Self {
        Self { expr_context: Some(context.into()), ..self }
    }

    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }
}

impl<'a, T: Into<JsExpr<'a>>> From<T> for ScopedEvalParams<'a> {
    fn from(expr: T) -> Self {
        ScopedEvalParams::new(expr)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalEvalParams<'a> {
    pub expr: JsExpr<'a>,
    pub execution_context: Option<ExecutionContext>,
    pub options: EvalOptions,
}

impl<'a> GlobalEvalParams<'a> {
    pub fn new(expr: impl Into<JsExpr<'a>>) -> Self {
        Self { expr: expr.into(), execution_context: None, options: EvalOptions::default() }
    }

    pub fn execution_context<T: Into<ExecutionContext>>(self, context: T) -> Self {
        Self { execution_context: Some(context.into()), ..self }
    }

    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }

    pub fn into_scoped(self) -> ScopedEvalParams<'a> {
        let scoped = ScopedEvalParams::new(self.expr)
            .options(self.options);
        if let Some(execution_context) = self.execution_context {
            scoped.expr_execution_context(execution_context)
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

mod private {
    #![allow(unused)]

    pub trait Sealed {}
    impl<'a, T: ?Sized + Sealed> Sealed for &'a T {}
    impl<'a, T: ?Sized + Sealed> Sealed for &'a mut T {}
}
