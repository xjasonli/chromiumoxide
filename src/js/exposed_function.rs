use std::sync::Arc;
use chromiumoxide_cdp::cdp::browser_protocol::page::{AddScriptToEvaluateOnNewDocumentParams, RemoveScriptToEvaluateOnNewDocumentParams, ScriptIdentifier};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{AddBindingParams, EventBindingCalled, ExecutionContextId};
use rand::Rng;
use schemars::Schema;
use serde_json::Value as JsonValue;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize, de::DeserializeSeed};

use crate::error::CdpError;
use crate::listeners::EventStream;
use crate::utils::evaluation_string;
use crate::{error::Result, Page};
use crate::handler::PageInner;
use super::native_value::{NativeValueFromJs, NativeValueIntoJs, FunctionNativeArgsFromJs};
use crate::js::de::PageDeserializeSeed;
use super::*;

#[cfg(feature = "tokio-runtime")]
type Scope<'a, T = ()> = async_scoped::TokioScope<'a, T>;
#[cfg(feature = "async-std-runtime")]
type Scope<'a, T = ()> = async_scoped::AsyncStdScope<'a, T>;

pub struct ExposedFunction<'a> {
    shared: Arc<Shared<'a>>,
    script_id: ScriptIdentifier,
    _scope: Scope<'a>,
}

impl<'a> ExposedFunction<'a> {
    pub(crate) async fn new<T, K, E, R, A>(
        name: String,
        page: Arc<PageInner>,
        callback: T
    ) -> Result<Self>
    where
        T: CallbackAdapter<K, E, R, A> + 'a,
        K: 'static,
        E: JsCallbackError,
        R: NativeValueIntoJs + 'a,
        A: FunctionNativeArgsFromJs + 'a,
    {
        ensure_binding(&page, &*CDP_BINDING_NAME).await?;

        let listener = page
            .event_listener::<EventBindingCalled>()
            .await?;

        let script_id = page.execute(
            AddScriptToEvaluateOnNewDocumentParams::builder()
            .source(add_function_script(&name, &*CDP_BINDING_NAME)?)
            .run_immediately(true)
            .build().unwrap()
        ).await?.result.identifier;

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

        Ok(Self { shared, script_id, _scope: scope })
    }

    /// Returns the name of the callback as registered in the browser.
    /// 
    /// This method returns the string identifier that was used to register
    /// the callback in the JavaScript environment.
    pub fn name(&self) -> &str {
        &self.shared.name
    }
}

impl<'a> Drop for ExposedFunction<'a> {
    fn drop(&mut self) {
        let _ = self.shared.page.execute_no_wait(
            RemoveScriptToEvaluateOnNewDocumentParams::builder()
                .identifier(self.script_id.clone())
                .build().unwrap()
        );
    }
}

impl<'a> std::fmt::Debug for ExposedFunction<'a> {
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
    page: Arc<PageInner>,
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
                        let _ = set_result(
                            self.page.clone(),
                            &self.name,
                            payload.seq,
                            event.execution_context_id,
                            Some(result),
                            None
                        ).await;
                    }
                    Err(e) => {
                        let _ = set_result(
                            self.page.clone(),
                            &self.name,
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

    async fn invoke(&self, seq: u64, execution_context_id: ExecutionContextId) -> Result<JsonValue, JsCallbackErrorWrapper> {
        let args = get_arguments(
            self.page.clone(),
            &self.name,
            seq,
            execution_context_id,
            self.adapter.args_schema()
        ).await?;

        let result = self.adapter.call(
            self.page.clone(),
            args
        ).await?;
        Ok(result)
    }
}

impl std::fmt::Debug for Shared<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback {{ name: {}, function: <function> }}", self.name)
    }
}

const ADD_FUNCTION: &str = include_str!("exposed_function/addFunction.js");
const GET_ARGUMENTS: &str = include_str!("exposed_function/getArguments.js");
const SET_RESULT: &str = include_str!("exposed_function/setResult.js");

fn add_function_script(name: &str, binding: &str) -> Result<String, serde_json::Error> {
    evaluation_string(ADD_FUNCTION, (name, binding))
}

async fn get_arguments(
    page: Arc<PageInner>,
    name: &str,
    seq: u64,
    execution_context_id: ExecutionContextId,
    schema: Schema,
) -> Result<Vec<JsonValue>> {
    let params = EvalParams::new(GET_ARGUMENTS)
        .with_context(Some(ScopedExecutionContext::Id(execution_context_id.into())));

    let JsonValue::Array(args) = page.invoke_function(params, None)
        .arguments((name, seq))?
        .invoke_with_schema(schema).await? else {
        return Err(CdpError::UnexpectedValue("args is not an array".to_string()));
    };

    Ok(args)
}

async fn set_result(
    page: Arc<PageInner>,
    name: &str,
    seq: u64,
    execution_context_id: ExecutionContextId,
    result: Option<JsonValue>,
    errmsg: Option<String>
) -> Result<()> {
    let params = EvalParams::new(SET_RESULT)
        .with_context(Some(ScopedExecutionContext::Id(execution_context_id.into())));

    page.invoke_function(params, None)
        .arguments((name, seq, result, errmsg))?
        .invoke::<()>().await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CallbackPayload {
    name: String,
    seq: u64,
}

pub trait JsCallbackError: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static {}
impl<T: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static> JsCallbackError for T {}

#[derive(Debug)]
pub struct JsCallbackErrorWrapper(Box<dyn JsCallbackError>);
impl<T> From<T> for JsCallbackErrorWrapper
where
    T: JsCallbackError
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[async_trait::async_trait]
pub trait CallbackAdapter<K, E, R, A>: Send + Sync {
    async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsCallbackErrorWrapper>;
    fn args_schema(&self) -> Schema;
}

#[async_trait::async_trait]
trait ErasedAdapter: Send + Sync {
    async fn call(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, JsCallbackErrorWrapper>;
    fn args_schema(&self) -> Schema;
}
type BoxedAdapter<'a> = Box<dyn ErasedAdapter + 'a>;

struct WrappedAdapter<'a, K, E, R, A>(
    Box<dyn CallbackAdapter<K, E, R, A> + 'a>
);

#[async_trait::async_trait]
impl<'a, K, E, R, A> ErasedAdapter for WrappedAdapter<'a, K, E, R, A> {
    async fn call(&self, page: Arc<PageInner>, args: Vec<JsonValue>) -> Result<JsonValue, JsCallbackErrorWrapper> {
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
            impl<F, E, R, $($ty,)*> CallbackAdapter<CallKindSync, E, R, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Result<R, E>) + Send + Sync,
                E: JsCallbackError,
                R: NativeValueIntoJs,
                $( $ty: NativeValueFromJs,)*
            {
                async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsCallbackErrorWrapper> {
                    let page_inner = page.into_inner();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageDeserializeSeed::new(page_inner.clone(), PhantomData);
                        let [< $ty:lower >] = seed.deserialize(json)?;
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
            impl<F, Fut, E, R, $($ty,)*> CallbackAdapter<CallKindAsync, E, R, ($($ty,)*)> for F
            where
                F: (Fn($($ty,)*) -> Fut) + Send + Sync,
                Fut: futures::Future<Output = Result<R, E>> + Send,
                E: JsCallbackError,
                R: NativeValueIntoJs,
                $( $ty: NativeValueFromJs,)*
            {
                async fn call(&self, page: Page, args: Vec<JsonValue>) -> Result<JsonValue, JsCallbackErrorWrapper> {
                    let page_inner = page.into_inner();
                    let mut _iter = args.into_iter();
                    $(
                        let json = _iter.next().unwrap_or_default();
                        let seed = PageDeserializeSeed::new(page_inner.clone(), PhantomData);
                        let [< $ty:lower >] = seed.deserialize(json)?;
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


    const GET_BINDING_DESC: &'static str = js!(
        (name) => {
            let descriptor = Object.getOwnPropertyDescriptor(globalThis, name);
            if (descriptor) {
                descriptor.value = typeof descriptor.value;
                return descriptor;
            }
            return null;
        }
    );
    let descriptor: Option<PropertyDescriptor> = page.invoke_function(GET_BINDING_DESC, None)
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
        const SET_BINDING_ENUMERABLE: &'static str = js!(
            (name) => {
                Object.defineProperty(globalThis, name, {
                    enumerable: false,
                });
            }
        );
        // Make the cdp binding non-enumerable on the main frame
        page.invoke_function(SET_BINDING_ENUMERABLE, None)
            .argument(binding_name)?
            .invoke::<()>().await?;
    }
    Ok(())
}
