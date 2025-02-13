//! Provides functionality for exposing Rust functions to JavaScript.
//! 
//! This module allows registering Rust functions that can be called from JavaScript code
//! running in the browser. The exposed functions are added to the global scope (`window`)
//! and can be called like regular JavaScript functions.
//!
//! # Example
//! ```no_run
//! # use chromiumoxide::page::Page;
//! # use chromiumoxide::error::Result;
//! # async fn example(page: &Page) -> Result<()> {
//! // Expose a Rust function to JavaScript
//! page.expose_function(
//!     "add",
//!     |a: i32, b: i32| -> Result<i32, std::convert::Infallible> {
//!         Ok(a + b)
//!     }
//! ).await?;
//!
//! // Call it from JavaScript
//! let result = page.eval::<i32>("add(2, 3)").await?;
//! assert_eq!(result, 5);
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;
use chromiumoxide_cdp::cdp::browser_protocol::page::{AddScriptToEvaluateOnNewDocumentParams, RemoveScriptToEvaluateOnNewDocumentParams, ScriptIdentifier};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{AddBindingParams, EventBindingCalled, ExecutionContextId};
use rand::Rng;
use schemars::Schema;
use serde_json::Value as JsonValue;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize, de::DeserializeSeed};

use crate::error::{CdpError, Result};
use crate::listeners::EventStream;
use crate::utils::evaluation_string;
use crate::handler::PageInner;
use crate::page::Page;
use crate::js::de::PageDeserializeSeed;
use crate::js::{FromJs, IntoJs, FromJsArgs, ScopedEvalParams, js_expr_str};

/// Trait for functions that can be exposed to JavaScript.
///
/// This trait is implemented for Rust functions that can be called from JavaScript.
/// The implementation handles:
/// - Converting JavaScript arguments to Rust types
/// - Converting Rust return values to JavaScript
/// - Error handling and propagation
/// - Async/sync function support
///
/// # Function Signature
/// ```ignore
/// async? fn exposable_fn<M, E, R, A1, A2, ..., AN>(
///     arg1: A1,
///     arg2: A2,
///     ...
///     argN: AN,
/// ) -> Result<R, E>
/// where
///     M: Marker for both sync and async functions
///     E: Display + Debug + Send + Sync
///     R: NativeValueIntoJs
///     A1: NativeValueFromJs
///     A2: NativeValueFromJs
///     ...
///     AN: NativeValueFromJs
/// ```
///
/// # Type Parameters
/// - `M`: Marker type for function kind:
///   - `()` for synchronous functions
///   - `ExposableFnAsyncMarker` for async functions
/// - `E`: Error type that implements `Display + Debug + Send + Sync`
/// - `R`: Return type that implements `NativeValueIntoJs`
/// - `Args`: Argument types that implement `NativeValueFromJs`
///
/// # Requirements
/// - Function must be `Send + Sync` for thread safety
/// - Arguments must implement `NativeValueFromJs` for safe deserialization from JavaScript values
/// - Return type must be `Result<T, E>` where:
///   - `T` implements `NativeValueIntoJs` for serialization to JavaScript
///   - `E` implements `ExposableFnError` for error handling
/// 
pub trait ExposableFn<M, E, R, A>: private::Sealed<M, E, R, A> + Send + Sync {}
impl<F, M, E, R, A> ExposableFn<M, E, R, A> for F
where F: private::Sealed<M, E, R, A> + Send + Sync {}

/// A trait for error types that can be safely propagated to JavaScript.
/// 
/// This trait is automatically implemented for any type that implements:
/// - `std::fmt::Display` for formatting the error message
/// - `std::fmt::Debug` for debug information
/// - `Send + Sync` for thread safety
/// 
/// When a function exposed to JavaScript returns an `Err`, the error message
/// will be converted to a JavaScript Error object.
pub trait ExposableFnError: std::fmt::Display + std::fmt::Debug + Send + Sync {}
impl<E> ExposableFnError for E where E: std::fmt::Display + std::fmt::Debug + Send + Sync {}

/// Represents a Rust function that has been exposed to the JavaScript runtime.
///
/// This type serves as a handle to a Rust function that has been registered in the JavaScript
/// environment. It manages the function's lifecycle in the JavaScript runtime, including:
/// - Registration and initialization in the JavaScript global scope
/// - Handling incoming function calls from JavaScript
/// - Automatic cleanup when the handle is dropped
///
/// The exposed function remains callable from JavaScript as long as this handle is alive.
/// Once the handle is dropped, the function is automatically unregistered from the JavaScript
/// environment.
///
/// # Lifetime
/// The lifetime parameter `'f` represents how long the underlying Rust function
/// must remain valid. The function must live at least as long as this handle.
///
/// # Example
/// ```no_run
/// # use chromiumoxide::page::Page;
/// # use chromiumoxide::error::Result;
/// # async fn example(page: &Page) -> Result<()> {
/// let handle = page.expose_function(
///     "add",
///     |a: i32, b: i32| -> Result<i32, std::convert::Infallible> {
///         Ok(a + b)
///     }
/// ).await?;
/// 
/// // Function is available in JavaScript while `handle` is alive
/// // When `handle` is dropped, the function is removed from JavaScript
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ExposedFunction<'f>(Arc<ExposedFunctionInner<'f>>);

impl<'f> ExposedFunction<'f> {
    /// Returns the name of the callback as registered in the browser.
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub(crate) async fn new<T, M, E, R, A>(name: String, page: Arc<PageInner>, callback: T) -> Result<Self>
    where
        T: ExposableFn<M, E, R, A> + 'f,
        M: 'f,
        E: ExposableFnError + 'f,
        R: IntoJs + 'f,
        for<'a> A: FromJsArgs + 'a,
    {
        let inner = ExposedFunctionInner::new(name, page, callback).await?;
        Ok(Self(Arc::new(inner)))
    }
}

impl<'f> AsRef<str> for ExposedFunction<'f> {
    fn as_ref(&self) -> &str {
        self.0.name()
    }
}

#[cfg(feature = "tokio-runtime")]
type Scope<'f, T = ()> = async_scoped::TokioScope<'f, T>;
#[cfg(feature = "async-std-runtime")]
type Scope<'f, T = ()> = async_scoped::AsyncStdScope<'f, T>;

struct ExposedFunctionInner<'f> {
    shared: Arc<Shared<'f>>,
    _scope: Scope<'f>,
}

impl<'f> ExposedFunctionInner<'f> {
    async fn new<F, M, E, R, A>(name: String, page: Arc<PageInner>, callback: F) -> Result<Self>
    where
        F: ExposableFn<M, E, R, A> + 'f,
        M: 'f,
        E: ExposableFnError + 'f,
        R: IntoJs + 'f,
        for<'a> A: FromJsArgs + 'a,
    {
        // Ensure the CDP binding is available
        ensure_binding(&page, &*CDP_BINDING_NAME).await?;

        // Set up event listener for function calls
        let listener = page
            .event_listener::<EventBindingCalled>()
            .await?;

        // Register the function in JavaScript
        let script = page.execute(
            AddScriptToEvaluateOnNewDocumentParams::builder()
            .source(add_function_script(&name, &*CDP_BINDING_NAME)?)
            .run_immediately(true)
            .build().unwrap()
        ).await?.result.identifier;

        // Create the shared state
        let adapter = Box::new(BoxedFn(Box::new(callback)));
        let shared = Arc::new(Shared {
            name,
            page,
            script,
            adapter,
        });

        // Start the event handling scope
        let (scope, _) = {
            let shared = shared.clone();
            unsafe {
                Scope::scope(move |s| {
                    s.spawn_cancellable(shared.run(listener), || ())
                })
            }
        };

        Ok(Self { shared, _scope: scope })
    }

    fn name(&self) -> &str {
        &self.shared.name
    }
}

/// Automatically removes the function from JavaScript when dropped.
impl<'f> Drop for ExposedFunctionInner<'f> {
    fn drop(&mut self) {
        let _ = self.shared.page.execute_no_wait(
            RemoveScriptToEvaluateOnNewDocumentParams::builder()
                .identifier(self.shared.script.clone())
                .build().unwrap()
        );
    }
}

impl<'f> std::fmt::Debug for ExposedFunctionInner<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExposedFunction {{ name: {}, function: <function> }}", self.shared.name)
    }
}

/// Internal shared state for the callback implementation.
/// 
/// This struct holds the data needed to manage the callback's lifecycle
/// and handle invocations from the browser.
struct Shared<'f> {
    /// The name of the callback as registered in JavaScript
    name: String,
    /// The page context where the callback is registered
    page: Arc<PageInner>,
    /// The adapter that wraps the actual callback function
    adapter: BoxedErasedFn<'f>,
    /// The script identifier for the callback
    script: ScriptIdentifier,
}

impl<'f> Shared<'f> {
    /// Runs the event loop that handles callback invocations from the browser.
    async fn run(self: Arc<Self>, listener: EventStream<EventBindingCalled>) {
        use futures::StreamExt as _;
        let mut listener = listener.fuse();
        loop {
            futures::select! {
                event = listener.next() => {
                    if let Some(event) = event {
                        self.on_binding_called(&event).await;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    /// Handles a binding call event from the browser.
    async fn on_binding_called(&self, event: &EventBindingCalled)
    {
        if event.name != *CDP_BINDING_NAME {
            return;
        }

        match serde_json::from_str::<CallbackPayload>(&event.payload) {
            Ok(payload) => {
                if payload.name != self.name {
                    return;
                }
                match self.invoke(payload.seq, event.execution_context_id).await {
                    Ok(result) => {
                        let _ = set_result(
                            self.page.clone(),
                            &self.name,
                            payload.seq,
                            event.execution_context_id,
                            Some(result),
                            None
                        ).await;
                    }
                    Err(errmsg) => {
                        let _ = set_result(
                            self.page.clone(),
                            &self.name,
                            payload.seq,
                            event.execution_context_id,
                            None,
                            Some(errmsg)
                        ).await;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to parse payload for callback `{}`: {}", self.name, e);
            }
        }
    }

    /// Invokes the callback with arguments from JavaScript.
    async fn invoke(&self, seq: u64, execution_context_id: ExecutionContextId) -> Result<JsonValue, String> {
        let args = get_arguments(
            self.page.clone(),
            &self.name,
            seq,
            execution_context_id,
            self.adapter.arguments_schema()
        ).await.map_err(|e| e.to_string())?;

        let result = self.adapter.invoke_from_javascript(self.page.clone(), args).await;
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl std::fmt::Debug for Shared<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback {{ name: {}, function: <function> }}", self.name)
    }
}

/// JavaScript code for function registration and invocation
const ADD_FUNCTION: &str = include_str!("exposed_function/addFunction.js");
const GET_ARGUMENTS: &str = include_str!("exposed_function/getArguments.js");
const SET_RESULT: &str = include_str!("exposed_function/setResult.js");

/// Creates the JavaScript code to register a function.
fn add_function_script(name: &str, binding: &str) -> Result<String, serde_json::Error> {
    evaluation_string(ADD_FUNCTION, (name, binding))
}

/// Gets the arguments passed to a function call from JavaScript.
async fn get_arguments(
    page: Arc<PageInner>,
    name: &str,
    seq: u64,
    execution_context_id: ExecutionContextId,
    schema: Schema,
) -> Result<Vec<JsonValue>> {
    let params = ScopedEvalParams::new(GET_ARGUMENTS)
        .expr_execution_context(execution_context_id);

    let JsonValue::Array(args) = page.invoke_function(params)
        .arguments((name, seq))?
        .invoke_with_schema(schema).await? else {
        return Err(CdpError::UnexpectedValue("args is not an array".to_string()));
    };

    Ok(args)
}

/// Sets the result of a function call back to JavaScript.
async fn set_result(
    page: Arc<PageInner>,
    name: &str,
    seq: u64,
    execution_context_id: ExecutionContextId,
    result: Option<JsonValue>,
    errmsg: Option<String>
) -> Result<()> {
    let params = ScopedEvalParams::new(SET_RESULT)
        .expr_execution_context(execution_context_id);

    page.invoke_function(params)
        .arguments((name, seq, result, errmsg))?
        .invoke::<()>().await?;
    Ok(())
}

/// Payload for callback invocations from JavaScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CallbackPayload {
    name: String,
    seq: u64,
}

/// Internal trait for type-erased callback adapters.
#[async_trait::async_trait]
trait ErasedFn: Send + Sync {
    async fn invoke_from_javascript(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, String>;
    fn arguments_schema(&self) -> Schema;
}
type BoxedErasedFn<'f> = Box<dyn ErasedFn + 'f>;

/// Wrapper that adapts a concrete callback type to the erased trait.
struct BoxedFn<'f, M, E, R, A>(
    Box<dyn ExposableFn<M, E, R, A> + 'f>
);

#[async_trait::async_trait]
impl<'f, M, E, R, A> ErasedFn for BoxedFn<'f, M, E, R, A>
where
    A: FromJsArgs,
{
    async fn invoke_from_javascript(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, String> {
        self.0.invoke_from_javascript(page.into(), args).await
    }
    fn arguments_schema(&self) -> Schema {
        self.0.arguments_schema()
    }
}

/// Marker type for exposable async functions.
#[derive(Debug, Clone, Copy)]
pub struct ExposableFnAsyncMarker;

macro_rules! impl_exposable_fn {
    (
        $($ty:ident),*
    ) => {
        paste::paste!{
            #[allow(unused_variables, unused_mut)]
            #[async_trait::async_trait]
            impl<'f, F, E, R, $($ty,)*> private::Sealed<(), E, R, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Result<R, E>) + Send + Sync + 'f,
                E: ExposableFnError + 'f,
                R: IntoJs + 'f,
                $( for<'a> $ty: FromJs + 'a,)*
            {
                async fn invoke_from_javascript(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, String> {
                    let page: Arc<PageInner> = page.into();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageDeserializeSeed::new(page.clone(), PhantomData);
                        let [< $ty:lower >] = seed.deserialize(json).map_err(|e| e.to_string())?;
                    )*

                    let result = self($([< $ty:lower >],)*).map_err(|e| e.to_string())?;
                    let json = serde_json::to_value(result).map_err(|e| e.to_string())?;
                    Ok(json)
                }
                fn arguments_schema(&self) -> Schema {
                    let schema = {
                        let mut settings = schemars::generate::SchemaSettings::default();
                        settings.inline_subschemas = true;
                        settings.into_generator().into_root_schema_for::<($($ty,)*)>()
                    };
                    schema
                }
            }
        }
    };
}

impl_exposable_fn!();
impl_exposable_fn!(A1);
impl_exposable_fn!(A1, A2);
impl_exposable_fn!(A1, A2, A3);
impl_exposable_fn!(A1, A2, A3, A4);
impl_exposable_fn!(A1, A2, A3, A4, A5);
impl_exposable_fn!(A1, A2, A3, A4, A5, A6);
impl_exposable_fn!(A1, A2, A3, A4, A5, A6, A7);
impl_exposable_fn!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_exposable_fn!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_exposable_fn!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

macro_rules! impl_exposable_fn_async {
    (
        $($ty:ident),*
    ) => {
        paste::paste!{
            #[allow(unused_variables, unused_mut)]
            #[async_trait::async_trait]
            impl<'f, F, Fut, E, R, $($ty,)*> private::Sealed<ExposableFnAsyncMarker, E, R, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Fut) + Send + Sync + 'f,
                Fut: futures::Future<Output = Result<R, E>> + Send + 'f,
                E: ExposableFnError + 'f,
                R: IntoJs + 'f,
                $( for<'a> $ty: FromJs + 'a,)*
            {
                async fn invoke_from_javascript(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, String> {
                    let page: Arc<PageInner> = page.into();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageDeserializeSeed::new(page.clone(), PhantomData);
                        let [< $ty:lower >] = seed.deserialize(json).map_err(|e| e.to_string())?;
                    )*

                    let result = self($([< $ty:lower >],)*).await.map_err(|e| e.to_string())?;
                    let json = serde_json::to_value(result).map_err(|e| e.to_string())?;
                    Ok(json)
                }
                fn arguments_schema(&self) -> Schema {
                    let schema = {
                        let mut settings = schemars::generate::SchemaSettings::default();
                        settings.inline_subschemas = true;
                        settings.into_generator().into_root_schema_for::<($($ty,)*)>()
                    };
                    schema
                }

            }
        }
    };
}

impl_exposable_fn_async!();
impl_exposable_fn_async!(A1);
impl_exposable_fn_async!(A1, A2);
impl_exposable_fn_async!(A1, A2, A3);
impl_exposable_fn_async!(A1, A2, A3, A4);
impl_exposable_fn_async!(A1, A2, A3, A4, A5);
impl_exposable_fn_async!(A1, A2, A3, A4, A5, A6);
impl_exposable_fn_async!(A1, A2, A3, A4, A5, A6, A7);
impl_exposable_fn_async!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_exposable_fn_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_exposable_fn_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

static CDP_BINDING_NAME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    random_string(rand::thread_rng().gen_range(30..50))
});

fn random_string(len: usize) -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}

async fn ensure_binding(page: &Arc<PageInner>, binding_name: &str) -> Result<()> {
    #[derive(Debug, Deserialize, schemars::JsonSchema)]
    #[serde(rename_all = "camelCase")]
    #[allow(unused)]
    struct PropertyDescriptor {
        configurable: bool,
        enumerable: bool,
        writable: bool,
        value: String,
    }

    const GET_BINDING_DESC: &'static str = js_expr_str!(
        (name) => {
            let descriptor = Object.getOwnPropertyDescriptor(globalThis, name);
            if (descriptor) {
                descriptor.value = typeof descriptor.value;
                return descriptor;
            }
            return null;
        }
    );
    let descriptor: Option<PropertyDescriptor> = page.invoke_function(GET_BINDING_DESC)
        .argument(binding_name)?
        .invoke().await?;

    let descriptor = if descriptor.is_none() || descriptor.as_ref().unwrap().value != "function" {
        // Add the cdp binding to the global object
        page.execute(
            AddBindingParams::builder()
                .name(binding_name)
                .build()
                .unwrap(),
        ).await?;
        PropertyDescriptor {
            configurable: true,
            enumerable: true,
            writable: true,
            value: "function".to_string(),
        }
    } else {
        descriptor.unwrap()
    };

    if descriptor.enumerable {
        const SET_BINDING_ENUMERABLE: &'static str = js_expr_str!(
            (name) => {
                Object.defineProperty(globalThis, name, {
                    enumerable: false,
                });
            }
        );
        // Make the cdp binding non-enumerable on the main frame
        page.invoke_function(SET_BINDING_ENUMERABLE)
            .argument(binding_name)?
            .invoke::<()>().await?;
    }
    Ok(())
}

mod private {
    use super::*;

    #[async_trait::async_trait]
    pub trait Sealed<M, E, R, A> {
        async fn invoke_from_javascript(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, String>;
        fn arguments_schema(&self) -> Schema;
    }
}
