use chromiumoxide_cdp::cdp::browser_protocol::dom::ResolveNodeParams;
use super::*;

pub mod other;
pub mod array;
pub mod node;
pub mod reg_exp;
pub mod date;
pub mod map;
pub mod set;
pub mod weak_map;
pub mod weak_set;
pub mod iterator;
pub mod generator;
pub mod error;
pub mod proxy;
pub mod promise;
pub mod typed_array;
pub mod array_buffer;
pub mod data_view;
pub mod wasm_memory;
pub mod wasm_value;

pub use other::*;
pub use array::*;
pub use node::*;
pub use reg_exp::*;
pub use date::*;
pub use map::*;
pub use set::*;
pub use weak_map::*;
pub use weak_set::*;
pub use iterator::*;
pub use generator::*;
pub use error::*;
pub use proxy::*;
pub use promise::*;
pub use typed_array::*;
pub use array_buffer::*;
pub use data_view::*;
pub use wasm_memory::*;
pub use wasm_value::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    class Object {
        static #type: ["object", "function"];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor
            constructor: JsFunction [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign
            objectAssign<I, T>(...sources: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: IntoJsAny {
                return Object.assign(this, ...sources);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create
            objectCreate() -> Self {
                return Object.create(this);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create
            /// 
            #[rename = + withProperties]
            objectCreate<T: IntoJsAny>(properties: T) -> Self {
                return Object.create(this, properties);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperties
            /// 
            objectDefineProperties<T: IntoJsAny>(properties: T) -> Self {
                return Object.defineProperties(this, properties);
            }


            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
            hasOwnProperty(key: String) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
            isPrototypeOf(value: &JsObject) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
            propertyIsEnumerable(key: JsPropertyName) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
            toString() -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf
            valueOf() -> Self;
        }
    }
);

impl JsObject {
    pub fn remote_object_subtype(&self) -> JsObjectSubtype {
        self.remote_object_type()
            .object_subtype()
            .expect("JsObject is not an object")
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum JsPropertyName {
    String(String),
    Symbol(JsSymbol),
}


#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum JsPropertyDescriptor<T> {
    Data(JsDataDescriptor<T>),
    Accessor(JsAccessorDescriptor),
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsDataDescriptor<T> {
    value: Option<T>,
    writable: bool,
    configurable: bool,
    enumerable: bool,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsAccessorDescriptor {    
    get: JsFunction,
    set: JsFunction,
    configurable: bool,
    enumerable: bool,
}

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget
    class EventTarget extends Object {
        static #type: "object";
        static #subtypes: ["node", "other"];
        static #classes: [
            "*",
            [
                "EventTarget",
                "Window",
                "IDB*Request",
                "IDBTransaction",
                "IDBDatabase",
                "AbortSignal",
                "Performance",
                "MediaQueryList",
            ]
        ];

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener
            addEventListener<T>(name: impl IntoJs<String>, listener: T, options?: &JsAddEventListenerOptions) -> ()
            where
                T: IntoJs<JsFunction>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener
            removeEventListener<T>(name: impl IntoJs<String>, listener: T, options?: &JsRemoveEventListenerOptions) -> ()
            where
                T: IntoJs<JsFunction>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent
            dispatchEvent(event: impl IntoJs<JsObject>) -> bool;
        }
    }
}

impl JsEventTarget {
    /// Get all event listeners for the target.
    /// 
    /// depth:
    ///  The maximum depth at which Node children should be retrieved,
    ///  defaults to 1. Use -1 for the entire subtree or provide an integer
    ///  larger than 0.
    /// 
    /// pierce:
    ///  Whether or not iframes and shadow roots should be traversed when
    ///  returning the subtree (default is false). Reports listeners for
    ///  all contexts if pierce is enabled.
    /// 
    /// Returns:
    ///  List of relevant listeners.
    /// 
    pub async fn get_event_listeners(&self, depth: i32, pierce: bool) -> Result<Vec<JsEventListener>> {
        use chromiumoxide_cdp::cdp::browser_protocol::dom_debugger::GetEventListenersParams;
        let params = GetEventListenersParams::builder()
            .object_id(self.remote_object_id())
            .depth(depth)
            .pierce(pierce)
            .build()
            .expect("infallible");

        let result = self.ctx().page.execute(params).await?.result;
        let listeners = JsEventListener::from_cdp_listeners(result.listeners, self.ctx()).await?;
        Ok(listeners)
    }
}

/// https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#type-EventListener
#[derive(Debug, Clone)]
pub struct JsEventListener {
    /// The event listener's type.
    pub r#type: String,

    /// The event listener's useCapture.
    pub use_capture: bool,
    
    /// The event listener's passive flag.
    pub passive: bool,

    /// The event listener's once flag.
    pub once: bool,

    /// The event listener's handler.
    pub handler: Option<JsObject>,

    /// The event listener's originalHandler.
    pub original_handler: Option<JsObject>,

    /// The event listener's backendNodeId.
    pub node: Option<JsNode>,
}

impl JsEventListener {
    async fn from_cdp_listeners(
        cdp_listeners: Vec<crate::cdp::browser_protocol::dom_debugger::EventListener>,
        ctx: JsRemoteObjectCtx,
    ) -> Result<Vec<Self>> {
        let mut listeners = Vec::new();
        for cdp_listener in cdp_listeners {
            listeners.push(Self::from_cdp_listener(cdp_listener, ctx.clone()).await?);
        }
        Ok(listeners)
    }

    async fn from_cdp_listener(
        cdp_listener: crate::cdp::browser_protocol::dom_debugger::EventListener,
        ctx: JsRemoteObjectCtx,
    ) -> Result<Self> {
        let handler = if let Some(handler) = cdp_listener.handler {
            let val = helper::JsRemoteVal::from_remote_object(&ctx.page, handler).await?;
            let object = JsRemoteObject::new(ctx.clone(), val)
                .downcast_unchecked::<JsObject>();
            Some(object)
        } else {
            None
        };

        let original_handler = if let Some(original_handler) = cdp_listener.original_handler {
            let val = helper::JsRemoteVal::from_remote_object(&ctx.page, original_handler).await?;
            let object = JsRemoteObject::new(ctx.clone(), val)
                .downcast_unchecked::<JsObject>();
            Some(object)
        } else {
            None
        };

        let node = if let Some(backend_node_id) = cdp_listener.backend_node_id {
            let params = ResolveNodeParams::builder()
                .backend_node_id(backend_node_id)
                .execution_context_id(ctx.execution_context_id)
                .build();
            let result = ctx.page.execute(params).await?.result;
            let remote_object = result.object;
            let val = helper::JsRemoteVal::from_remote_object(&ctx.page, remote_object).await?;
            let object = JsRemoteObject::new(ctx.clone(), val)
                .downcast_unchecked::<JsNode>();
            Some(object)
        } else {
            None
        };

        Ok(Self {
            r#type: cdp_listener.r#type,
            use_capture: cdp_listener.use_capture,
            passive: cdp_listener.passive,
            once: cdp_listener.once,
            handler,
            original_handler,
            node,
        })
    }
}

/// An object that specifies characteristics about the event listener.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#options
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsAddEventListenerOptions {
    /// A boolean value indicating that events of this type will be dispatched to the registered 
    /// listener before being dispatched to any EventTarget beneath it in the DOM tree. If not 
    /// specified, defaults to false.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#capture
    #[serde(default)]
    capture: bool,

    /// A boolean value indicating that the listener should be invoked at most once after being 
    /// added. If true, the listener would be automatically removed when invoked. If not specified, 
    /// defaults to false.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#once
    #[serde(default)]
    once: bool,

    /// A boolean value that, if true, indicates that the function specified by listener will never 
    /// call preventDefault(). If a passive listener calls preventDefault(), nothing will happen and 
    /// a console warning may be generated.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#passive
    #[serde(default)]
    passive: bool,

    /// An AbortSignal. The listener will be removed when the abort() method of the AbortController 
    /// which owns the AbortSignal is called. If not specified, no AbortSignal is associated with 
    /// the listener.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#signal
    #[serde(default)]
    signal: Optional<JsAbortSignal>,
}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsRemoveEventListenerOptions {
    #[serde(default)]
    capture: bool,
}
