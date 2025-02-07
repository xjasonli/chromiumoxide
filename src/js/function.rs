use super::*;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Function {
    Func(JsFunction),
    Expr(JsExpr),
}

impl From<JsFunction> for Function {
    fn from(function: JsFunction) -> Self {
        Self::Func(function)
    }
}

impl From<&JsFunction> for Function {
    fn from(function: &JsFunction) -> Self {
        Self::Func(function.clone())
    }
}

impl From<JsExpr> for Function {
    fn from(expr: JsExpr) -> Self {
        Self::Expr(expr)
    }
}

impl From<String> for Function {
    fn from(expr: String) -> Self {
        Self::Expr(JsExpr::new(expr))
    }
}

impl From<&String> for Function {
    fn from(expr: &String) -> Self {
        Self::Expr(JsExpr::new(expr.to_string()))
    }
}

impl From<&str> for Function {
    fn from(expr: &str) -> Self {
        Self::Expr(JsExpr::new(expr.to_string()))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EvalOptions {
    pub await_promise: bool,
    pub user_gesture: bool,
}

impl EvalOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn await_promise(mut self, await_promise: bool) -> Self {
        self.await_promise = await_promise;
        self
    }

    pub fn user_gesture(mut self, user_gesture: bool) -> Self {
        self.user_gesture = user_gesture;
        self
    }
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            await_promise: true,
            user_gesture: true,
        }
    }
}
