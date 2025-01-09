use std::sync::Arc;
use chromiumoxide_cdp::cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams;
use chromiumoxide_cdp::cdp::js_protocol::runtime::{AddBindingParams, EventBindingCalled, ExecutionContextId};
use rand::Rng;
use schemars::Schema;
use serde_json::Value as JsonValue;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

use crate::listeners::EventStream;
use crate::utils::evaluation_string;
use crate::{error::Result, Page};
use crate::handler::PageInner;
use super::native::{CallbackNativeArgs, NativeValueFromJs, NativeValueIntoJs};
use crate::js::de::PageSeed;

#[cfg(feature = "tokio-runtime")]
type Scope<'a, T = ()> = async_scoped::TokioScope<'a, T>;
#[cfg(feature = "async-std-runtime")]
type Scope<'a, T = ()> = async_scoped::AsyncStdScope<'a, T>;

/// Represents a JavaScript callback function that can be registered and invoked from the browser.
/// 
/// This struct provides a way to register Rust functions as JavaScript callbacks in the browser context.
/// The callbacks can be invoked from JavaScript code and handle the communication between the browser
/// and Rust environments.
/// 
/// # Example
/// ```no_run
/// # use chromiumoxide::Page;
/// # let page: Page = unimplemented!();
/// let callback = Callback::new(
///     "myCallback".to_string(),
///     page,
///     |x: i32| Ok(x + 1)
/// ).await?;
/// ```
pub struct Callback<'a> {
    shared: Arc<Shared<'a>>,
    _scope: Scope<'a>,
}

impl<'a> Callback<'a> {
    /// Creates a new callback with the given name, page context, and Rust function.
    /// 
    /// This method registers a Rust function as a JavaScript callback in the browser context.
    /// The callback can then be invoked from JavaScript code running in the browser.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::Page;
    /// # let page: Page = unimplemented!();
    /// let callback = Callback::new(
    ///     "myCallback".to_string(),
    ///     page,
    ///     |x: i32| Ok(x + 1)
    /// ).await?;
    /// ```
    pub(crate) async fn new<T, K, R, A>(name: String, page: Page, callback: T) -> Result<Self>
    where
        T: CallbackAdapter<K, R, A> + 'a,
        K: 'static,
        R: NativeValueIntoJs + 'a,
        A: CallbackNativeArgs + 'a,
    {
        ensure_binding(&page, &*CDP_BINDING_NAME).await?;

        let listener = page
            .event_listener::<EventBindingCalled>()
            .await?;

        page.execute(
            AddScriptToEvaluateOnNewDocumentParams::builder()
            .source(evaluation_string(FUNCTION_ADD_CALLBACK, (&name, &*CDP_BINDING_NAME))?)
            .run_immediately(true)
            .build().unwrap()
        ).await?;

        let adapter = Box::new(WrappedAdapter(Box::new(callback)));
        let shared = Arc::new(Shared {
            name,
            page,
            adapter,
        });

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

    /// Returns the name of the callback as registered in the browser.
    /// 
    /// This method returns the string identifier that was used to register
    /// the callback in the JavaScript environment.
    pub fn name(&self) -> &str {
        &self.shared.name
    }
}

impl<'a> std::fmt::Debug for Callback<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback {{ name: {}, function: <function> }}", self.shared.name)
    }
}

/// Internal shared state for the callback implementation.
/// 
/// This struct holds the data needed to manage the callback's lifecycle
/// and handle invocations from the browser.
struct Shared<'a> {
    /// The name of the callback as registered in JavaScript
    name: String,
    /// The page context where the callback is registered
    page: Page,
    /// The adapter that wraps the actual callback function
    adapter: BoxedAdapter<'a>,
}

impl<'a> Shared<'a> {
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

    async fn on_binding_called(&self, event: &EventBindingCalled) {
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
                        let _ = feed_callback_result(
                            self.page.clone(),
                            self.name.clone(),
                            payload.seq,
                            event.execution_context_id,
                            Some(result),
                            None
                        ).await;
                    }
                    Err(e) => {
                        let _ = feed_callback_result(
                            self.page.clone(),
                            self.name.clone(),
                            payload.seq,
                            event.execution_context_id,
                            None,
                            Some(e.0.to_string())
                        ).await;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to parse payload for callback `{}`: {}", self.name, e);
            }
        }
    }

    async fn invoke(&self, seq: u64, execution_context_id: ExecutionContextId) -> Result<JsonValue, JsErrorWrapper> {
        let args = self.page.declare_function(FUNCTION_GET_CALLBACK_ARGS)
            .with_context(execution_context_id)
            .call_impl(
                vec![serde_json::to_value(&self.name)?, serde_json::to_value(seq)?],
                self.adapter.args_schema()
            )
            .await?;

        let JsonValue::Array(args) = args else {
            return Err("args is not an array".into());
        };

        let result = self.adapter.call(self.page.clone().into_inner(), args).await?;
        Ok(result)
    }
}

impl std::fmt::Debug for Shared<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback {{ name: {}, function: <function> }}", self.name)
    }
}

async fn feed_callback_result(
    page: Page,
    name: String,
    seq: u64,
    execution_context_id: ExecutionContextId,
    result: Option<JsonValue>,
    errmsg: Option<String>
) -> Result<()> {
    page.declare_function(FUNCTION_FEED_CALLBACK_RESULT)
        .with_context(execution_context_id)
        .call_impl(
            vec![
                serde_json::to_value(name)?,
                serde_json::to_value(seq)?,
                serde_json::to_value(result)?,
                serde_json::to_value(errmsg)?,
            ],
            Schema::default(),
        )
        .await?;
    Ok(())
}

const FUNCTION_ADD_CALLBACK: &str = "function addCallback(name, binding) {
    if (globalThis[name]) {
        return;
    }

    Object.defineProperty(globalThis, name, {
        value: function(...args) {
            const thisFunction = globalThis[name];
            thisFunction.args ??= new Map();
            thisFunction.callbacks ??= new Map();

            const seq = (thisFunction.lastSeq ?? 0) + 1;
            thisFunction.lastSeq = seq;
            thisFunction.args.set(seq, args);

            globalThis[binding](
                JSON.stringify({
                    name: name,
                    seq: seq,
                })
            );

            return new Promise(function(resolveFunc, rejectFunc) {
                thisFunction.callbacks.set(seq, {
                    resolve: function(value) {
                        thisFunction.args.delete(seq);
                        resolveFunc(value);
                    },
                    reject: function(error) {
                        thisFunction.args.delete(seq);
                        rejectFunc(error);
                    }
                });
            });
        },
        enumerable: false,
    });
}";

const FUNCTION_GET_CALLBACK_ARGS: &str = "function getCallbackArgs(name, seq) {
    const thisFunction = globalThis[name];
    const args = thisFunction.args.get(seq);
    return args;
}";

const FUNCTION_FEED_CALLBACK_RESULT: &str = "function feedCallbackResult(name, seq, value, errmsg) {
    const thisFunction = globalThis[name];
    const callback = thisFunction.callbacks.get(seq);
    if (errmsg !== undefined && errmsg !== null) {
        let error = new Error(errmsg);
        callback.reject(error);
    } else {
        callback.resolve(value);
    }
}";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CallbackPayload {
    name: String,
    seq: u64,
    //args: Vec<JsonDescriptor>,
}

pub trait JsError: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static {}
impl<T: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static> JsError for T {}

#[derive(Debug)]
pub struct JsErrorWrapper(Box<dyn JsError>);
impl<T> From<T> for JsErrorWrapper
where
    T: JsError
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[async_trait::async_trait]
pub trait CallbackAdapter<K, R, A>: Send + Sync {
    async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsErrorWrapper>;
    fn args_schema(&self) -> Schema;
}

#[async_trait::async_trait]
trait ErasedAdapter: Send + Sync {
    async fn call(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, JsErrorWrapper>;
    fn args_schema(&self) -> Schema;
}
type BoxedAdapter<'a> = Box<dyn ErasedAdapter + 'a>;

struct WrappedAdapter<'a, K, R, A>(
    Box<dyn CallbackAdapter<K, R, A> + 'a>
);

#[async_trait::async_trait]
impl<'a, K, R, A> ErasedAdapter for WrappedAdapter<'a, K, R, A> {
    async fn call(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, JsErrorWrapper> {
        self.0.call(page.into(), args).await
    }
    fn args_schema(&self) -> Schema {
        self.0.args_schema()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CallKindAsync;
#[derive(Debug, Clone, Copy)]
pub struct CallKindSync;

macro_rules! impl_callback_adapter {
    (
        $($ty:ident),*
    ) => {
        paste::paste!{
            #[allow(unused_variables, unused_mut)]
            #[async_trait::async_trait]
            impl<F, R, E, $($ty,)*> CallbackAdapter<CallKindSync, Result<R, E>, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Result<R, E>) + Send + Sync,
                R: NativeValueIntoJs,
                E: JsError,
                $( $ty: NativeValueFromJs,)*
            {
                async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsErrorWrapper> {
                    let page_inner = page.into_inner();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageSeed::new(page_inner.clone(), PhantomData);
                        let [< $ty:lower >] = serde::de::DeserializeSeed::deserialize(seed, json)?;
                    )*

                    let result = self($([< $ty:lower >],)*)?;
                    let json = serde_json::to_value(result)?;
                    Ok(json)
                }
                fn args_schema(&self) -> Schema {
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

impl_callback_adapter!();
impl_callback_adapter!(A1);
impl_callback_adapter!(A1, A2);
impl_callback_adapter!(A1, A2, A3);
impl_callback_adapter!(A1, A2, A3, A4);
impl_callback_adapter!(A1, A2, A3, A4, A5);
impl_callback_adapter!(A1, A2, A3, A4, A5, A6);
impl_callback_adapter!(A1, A2, A3, A4, A5, A6, A7);
impl_callback_adapter!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_callback_adapter!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_callback_adapter!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

macro_rules! impl_callback_adapter_async {
    (
        $($ty:ident),*
    ) => {
        paste::paste!{
            #[allow(unused_variables, unused_mut)]
            #[async_trait::async_trait]
            impl<F, Fut, R, E, $($ty,)*> CallbackAdapter<CallKindAsync, Result<R, E>, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Fut) + Send + Sync,
                Fut: futures::Future<Output = Result<R, E>> + Send,
                R: NativeValueIntoJs,
                E: JsError,
                $( $ty: NativeValueFromJs,)*
            {
                async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsErrorWrapper> {
                    let page_inner = page.into_inner();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageSeed::new(page_inner.clone(), PhantomData);
                        let [< $ty:lower >] = serde::de::DeserializeSeed::deserialize(seed, json)?;
                    )*

                    let result = self($([< $ty:lower >],)*).await?;
                    let json = serde_json::to_value(result)?;
                    Ok(json)
                }
                fn args_schema(&self) -> Schema {
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

impl_callback_adapter_async!();
impl_callback_adapter_async!(A1);
impl_callback_adapter_async!(A1, A2);
impl_callback_adapter_async!(A1, A2, A3);
impl_callback_adapter_async!(A1, A2, A3, A4);
impl_callback_adapter_async!(A1, A2, A3, A4, A5);
impl_callback_adapter_async!(A1, A2, A3, A4, A5, A6);
impl_callback_adapter_async!(A1, A2, A3, A4, A5, A6, A7);
impl_callback_adapter_async!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_callback_adapter_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_callback_adapter_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

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

async fn ensure_binding(page: &Page, name: &str) -> Result<()> {
    #[derive(Debug, Deserialize, schemars::JsonSchema)]
    #[serde(rename_all = "camelCase")]
    #[allow(unused)]
    struct PropertyDescriptor {
        configurable: bool,
        enumerable: bool,
        writable: bool,
        value: String,
    }

    let descriptor = page.invoke_function::<Option<PropertyDescriptor>>((),
        "(name) => {
            let descriptor = Object.getOwnPropertyDescriptor(globalThis, name);
            if (descriptor) {
                descriptor.value = typeof descriptor.value;
                return descriptor;
            }
            return null;
        }",
        (name,)
    ).await?;

    let descriptor = if descriptor.is_none() || descriptor.as_ref().unwrap().value != "function" {
        page.execute(
            AddBindingParams::builder()
                .name(name)
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
        page.invoke_function::<()>((),
            "(name) => {
                Object.defineProperty(globalThis, name, {
                    enumerable: false,
                });
            }",
            (name,),
        ).await?;
    }
    Ok(())
}
