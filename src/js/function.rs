//! Provides types and functionality for working with JavaScript functions in the Chrome DevTools Protocol.
//! 
//! This module contains types that represent JavaScript functions and expressions that can be
//! evaluated in the browser context, as well as configuration options for their evaluation.

use super::*;

/// Represents a JavaScript function that can be invoked in the browser context.
///
/// This enum provides two ways to reference a JavaScript function:
/// - As a remote function object that already exists in the browser (`Func`)
/// - As a JavaScript expression that evaluates to a function (`Expr`)
///
/// When using `Expr`, the evaluated expression must return a function, otherwise
/// invoking it will fail. This is commonly used with function expressions like:
/// ```javascript
/// "(x) => x + 1"                    // Arrow function
/// "function(x) { return x + 1; }"   // Function expression
/// "Math.max"                        // Reference to existing function
/// ```
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Function {
    /// A reference to a JavaScript function object that exists in the browser context
    /// as a CDP RemoteObject of type "function"
    Func(JsFunction),
    
    /// A JavaScript expression that must evaluate to a function.
    /// The expression will be evaluated in the browser context when the function
    /// is invoked.
    Expr(JsExpr),
}

/// Converts a `JsFunction` into a `Function` as a remote function reference.
impl From<JsFunction> for Function {
    fn from(function: JsFunction) -> Self {
        Self::Func(function)
    }
}

/// Converts a reference to a `JsFunction` into a `Function` as a remote function reference.
impl From<&JsFunction> for Function {
    fn from(function: &JsFunction) -> Self {
        Self::Func(function.clone())
    }
}

/// Converts a `JsExpr` into a `Function` as a function expression.
impl From<JsExpr> for Function {
    fn from(expr: JsExpr) -> Self {
        Self::Expr(expr)
    }
}

/// Converts a `String` into a `Function` by creating a new function expression.
/// The string must evaluate to a function when executed in the browser context.
impl From<String> for Function {
    fn from(expr: String) -> Self {
        Self::Expr(JsExpr::new(expr))
    }
}

/// Converts a reference to a `String` into a `Function` by creating a new function expression.
/// The string must evaluate to a function when executed in the browser context.
impl From<&String> for Function {
    fn from(expr: &String) -> Self {
        Self::Expr(JsExpr::new(expr.to_string()))
    }
}

/// Converts a string slice into a `Function` by creating a new function expression.
/// The string must evaluate to a function when executed in the browser context.
impl From<&str> for Function {
    fn from(expr: &str) -> Self {
        Self::Expr(JsExpr::new(expr.to_string()))
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