//! Provides functionality for invoking JavaScript functions in the browser context.
//! 
//! This module contains types and utilities for executing JavaScript functions with arguments
//! and handling their return values through the Chrome DevTools Protocol.

use std::marker::PhantomData;
use std::sync::Arc;

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
#[derive(Debug)]
pub struct FunctionInvoker<'a> {
    page: Arc<PageInner>,
    target: helper::EvalTarget<'a>,
    params: InvokeParams<'a>,
    options: EvalOptions,
}

impl<'a> FunctionInvoker<'a> {
    pub(crate) fn new(
        page: Arc<PageInner>,
        target: helper::EvalTarget<'a>,
        options: EvalOptions
    ) -> Self {
        Self { page, target, params: InvokeParams::new(), options }
    }

    pub fn this<T: IntoJs + 'a>(mut self, this: T) -> Self {
        self.params.this(this);
        self
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
    pub fn argument<T: IntoJs + 'a>(mut self, argument: T) -> Self {
        self.params.argument(argument);
        self
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
    pub fn arguments<Args>(mut self, arguments: Args) -> Self
    where
        Args: IntoJsArgs<'a>,
    {
        self.params.arguments(arguments);
        self
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
    pub fn arguments_spread<I, T>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: IntoJs + 'a,
    {
        self.params.arguments_spread(arguments);
        self
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
        T: FromJs,
    {
        let schema = {
            let mut settings = schemars::generate::SchemaSettings::default();
            settings.inline_subschemas = true;
            settings.into_generator().into_root_schema_for::<T>()
        };
        let page = self.page.clone();
        let (value, execution_context_id) = self.invoke_with_schema(schema).await?;
        let value = serde::de::DeserializeSeed::deserialize(
            de::JsDeserializeSeed::new(
                JsRemoteObjectCtx::new(page, execution_context_id),
                PhantomData,
            ),
            value
        )?;
        Ok(value)
    }

    pub(crate) async fn invoke_with_schema(self, schema: Schema) -> Result<(JsonValue, ExecutionContextId)> {
        let (value, execution_context_id) = helper::evaluate(
            self.page.clone(),
            self.target,
            Some(self.params),
            schema,
            self.options
        ).await?;
        Ok((value, execution_context_id))
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(crate) struct InvokeParams<'a>(Vec<BoxedIntoJs<'a>>);

impl<'a> InvokeParams<'a> {
    /// Creates a new parameter set.
    pub fn new() -> Self {
        Self(vec![Box::new(())])
    }

    pub fn this<T: IntoJs + 'a>(&mut self, this: T) {
        self.0[0] = Box::new(this);
    }

    /// Adds a single argument value.
    pub fn argument<T: IntoJs + 'a>(&mut self, value: T) {
        self.0.push(Box::new(value));
    }

    /// Adds multiple arguments from a tuple.
    pub fn arguments<Args>(&mut self, arguments: Args)
    where
        Args: IntoJsArgs<'a>,
    {
        self.0.extend(Args::into_vec(arguments));
    }

    /// Adds multiple arguments from an iterator.
    pub fn arguments_spread<I, T>(&mut self, arguments: I)
    where
        I: IntoIterator<Item = T>,
        T: IntoJs + 'a,
    {
        self.0.extend(
            arguments.into_iter().map(|arg| -> BoxedIntoJs<'a> { Box::new(arg) })
        );
    }
}
