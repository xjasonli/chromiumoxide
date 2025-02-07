use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

use chromiumoxide_cdp::cdp::js_protocol::runtime::{
    CallFunctionOnParams, EvaluateParams, RemoteObject,
};

use crate::utils::is_likely_js_function;

pub(crate) mod helper;
pub mod de;
pub mod any;
pub mod class;
pub mod native_value;
pub mod remote_object;
pub mod bigint;
pub mod expr;
pub mod function;
pub mod function_invoker;
pub mod optional;
pub mod undefined;
pub mod exposed_function;
pub mod execution_context;

pub use any::*;
pub use class::*;
pub use native_value::*;
pub use remote_object::*;
pub use bigint::*;
pub use expr::*;
pub use function::*;
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

#[derive(Debug, Clone)]
pub struct EvalParams {
    pub expr: String,
    pub this: Option<JsRemoteObject>,
    pub context: Option<ScopedExecutionContext>,
    pub options: EvalOptions,
}

impl EvalParams {
    pub fn new(expr: impl Into<String>) -> Self {
        Self { expr: expr.into(), this: None, context: None, options: EvalOptions::default() }
    }

    pub fn this<T: Class<JsRemoteObject>>(self, this: T) -> Self {
        let this = Class::<JsRemoteObject>::as_ref(&this).clone();
        Self { this: Some(this), ..self }
    }

    pub fn context<T: Into<ScopedExecutionContext>>(self, context: T) -> Self {
        Self { context: Some(context.into()), ..self }
    }

    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }
}

impl<T: Into<String>> From<T> for EvalParams {
    fn from(expr: T) -> Self {
        EvalParams::new(expr)
    }
}

#[derive(Debug, Clone)]
pub struct EvalGlobalParams {
    pub expr: String,
    pub context: Option<ExecutionContext>,
    pub options: EvalOptions,
}

impl EvalGlobalParams {
    pub fn new(expr: impl Into<String>) -> Self {
        Self { expr: expr.into(), context: None, options: EvalOptions::default() }
    }

    pub fn context<T: Into<ExecutionContext>>(self, context: T) -> Self {
        Self { context: Some(context.into()), ..self }
    }

    pub fn options(self, options: EvalOptions) -> Self {
        Self { options, ..self }
    }
}

impl<T: Into<String>> From<T> for EvalGlobalParams {
    fn from(expr: T) -> Self {
        EvalGlobalParams::new(expr)
    }
}

#[macro_export]
macro_rules! js {
    ($($js:tt)+) => {
        stringify!($($js)+)
    }
}
pub use js;

mod private {
    pub trait Sealed {}
    impl<'a, T: ?Sized + Sealed> Sealed for &'a T {}
    impl<'a, T: ?Sized + Sealed> Sealed for &'a mut T {}
}
