use chromiumoxide_cdp::cdp::browser_protocol::dom::ResolveNodeParams;
use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget
    class EventTarget extends Object {
        static #type: "object";
        static #subtypes: ["none", "node"];
        static #classes: [["EventTarget", "Window"], "*"];

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener
            addEventListener<T>(name: impl IntoJs<str>, listener: T, options?: &JsAddEventListenerOptions) -> ()
            where
                T: IntoJs<JsFunction>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener
            removeEventListener<T>(name: impl IntoJs<str>, listener: T, options?: &JsRemoveEventListenerOptions) -> ()
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

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal
    class AbortSignal extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "AbortSignal";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted
            aborted: bool [readonly];
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/reason
            reason: Optional<JsObject> [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/throwIfAborted
            throwIfAborted() -> ();
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Event
    class Event extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: ["Event",
            "AnimationEvent", "AudioProcessingEvent", "BeforeUnloadEvent",
            "BlobEvent", "ClipboardEvent", "CloseEvent", "CompositionEvent",
            "CustomEvent", "DeviceMotionEvent", "DeviceOrientationEvent",
            "DragEvent", "ErrorEvent", "FetchEvent", "FocusEvent",
            "FontFaceSetLoadEvent", "FormDataEvent", "GamepadEvent",
            "HashChangeEvent", "HIDInputReportEvent", "IDBVersionChangeEvent",
            "InputEvent", "KeyboardEvent", "MediaStreamEvent", "MessageEvent",
            "MouseEvent", "MutationEvent", "OfflineAudioCompletionEvent",
            "PageTransitionEvent", "PaymentRequestUpdateEvent", "PointerEvent",
            "PopStateEvent", "ProgressEvent", "RTCDataChannelEvent",
            "RTCPeerConnectionIceEvent", "StorageEvent", "SubmitEvent",
            "SVGEvent", "TimeEvent", "TouchEvent", "TrackEvent",
            "TransitionEvent", "UIEvent", "WebGLContextEvent", "WheelEvent",
        ];

        // https://developer.mozilla.org/en-US/docs/Web/API/Event#instance_properties
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles
            bubbles: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable
            cancelable: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/composed
            composed: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget
            currentTarget: Option<JsEventTarget> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented
            defaultPrevented: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase
            eventPhase: u16 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/target
            target: Option<JsEventTarget> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp
            timeStamp: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/type
            #[rename = typ]
            type: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted
            isTrusted: bool [readonly];
        }

        // https://developer.mozilla.org/en-US/docs/Web/API/Event#instance_methods
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath
            composedPath() -> Vec<JsEventTarget>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault
            preventDefault() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation
            stopImmediatePropagation() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation
            stopPropagation() -> ();
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window
    class Window extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "Window";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/document
            document: JsDocument [readonly];

            // todo
        }
        methods: {
            // todo
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Location
    class Location extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "Location";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/ancestorOrigins
            ancestorOrigins: Vec<String> [readonly] {
                get() {
                    return Array.from(this.ancestorOrigins);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/href
            href: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol
            protocol: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/host
            host: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname
            hostname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/port
            port: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname
            pathname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/search
            search: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/hash
            hash: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/origin
            origin: String [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/assign
            assign(url: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/reload
            reload() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/replace
            replace<T: IntoJs>(url: T) -> ();
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileList
    class FileList extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "FileList";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileList/length
            length: u32 [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileList/item
            item(index: u32) -> Option<JsFile>;
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Blob
    class Blob extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: ["Blob", "File"];
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/size
            size: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/type
            #[rename = typ]
            type: String [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer
            arrayBuffer() -> JsArrayBuffer;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/bytes
            bytes() -> JsTypedArray;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice
            slice(start?: isize, end?: isize, content_type?: &str) -> JsBlob;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/stream
            stream() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/text
            text() -> String;
        }
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/File
    class File extends Blob inherits Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "File";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified
            lastModified: u64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/File/name
            name: String [readonly];

            // https://developer.mozilla.org/en-US/docs/Web/API/File/webkitRelativePath
            webkitRelativePath: String [readonly];
        }
    }
);