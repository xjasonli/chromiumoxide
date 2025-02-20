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

        /*
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/caches
            caches: JsCacheStorage [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/closed
            closed: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/cookieStore
            cookieStore: JsCookieStore [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/credentialless
            credentialless: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated
            crossOriginIsolated: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto
            crypto: JsCrypto [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements
            customElements: JsCustomElementRegistry [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio
            devicePixelRatio: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/document
            document: JsDocument [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/documentPictureInPicture
            documentPictureInPicture: JsDocumentPictureInPicture [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fence
            fence: JsFenceObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement
            frameElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/frames
            frames: JsWindow [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/history
            history: JsHistory [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/indexedDB
            indexedDB: JsIDBFactory [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight
            innerHeight: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth
            innerWidth: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/isSecureContext
            isSecureContext: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/launchQueue
            launchQueue: JsLaunchQueue [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/length
            length: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
            localStorage: JsStorage [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/location
            location: JsLocation;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar
            locationbar: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar
            menubar: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/navigation
            navigation: JsNavigation [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator
            navigator: JsNavigator [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/opener
            opener: Option<JsWindow>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/origin
            origin: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/originAgentCluster
            originAgentCluster: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight
            outerHeight: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth
            outerWidth: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/parent
            parent: JsWindow [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/performance
            performance: JsPerformance [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar
            personalbar: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scheduler
            scheduler: JsScheduler [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/screen
            screen: JsScreen [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/screenLeft
            screenLeft: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/screenTop
            screenTop: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX
            screenX: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY
            screenY: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars
            scrollbars: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX
            scrollX: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY
            scrollY: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/self
            self: JsWindow [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
            sessionStorage: JsStorage [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/sharedStorage
            sharedStorage: JsSharedStorage [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis
            speechSynthesis: JsSpeechSynthesis [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar
            statusbar: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar
            toolbar: JsBarProp [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/top
            top: JsWindow [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/trustedTypes
            trustedTypes: JsTrustedTypePolicyFactory [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/visualViewport
            visualViewport: JsVisualViewport [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/window
            window: JsWindow [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/alert
            alert(message: Option<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/atob
            atob(data: String) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa
            btoa(data: String) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame
            cancelAnimationFrame(handle: u32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback
            cancelIdleCallback(handle: u32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval
            clearInterval(handle: Option<i32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout
            clearTimeout(handle: Option<i32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/close
            close() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm
            confirm(message: Option<String>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap
            createImageBitmap(image: JsImageBitmapSource) -> JsPromise<JsImageBitmap>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch
            fetch(input: AnyOf2<String, JsRequest>) -> JsPromise<JsResponse>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/focus
            focus() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle
            getComputedStyle(element: &JsElement) -> JsCssStyleDeclaration;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle
            #[rename = + withPseudoElement]
            getComputedStyle(element: &JsElement, pseudo_element: Option<String>) -> JsCssStyleDeclaration;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getScreenDetails
            getScreenDetails() -> JsPromise<JsScreenDetails>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection
            getSelection() -> Option<JsSelection>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia
            matchMedia(query: String) -> JsMediaQueryList;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy
            moveBy(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo
            moveTo(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/open
            open(url: Option<String>, target: Option<String>, features: Option<String>) -> Option<JsWindow>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage
            postMessage<T: IntoJs>(message: T, target_origin: String, transfer: Option<Vec<JsTransferable>>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/print
            print() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt
            prompt(message: Option<String>, default: Option<String>) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts
            queryLocalFonts() -> JsPromise<Vec<JsLocalFontAccess>>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/queueMicrotask
            queueMicrotask(callback: &JsFunction) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame
            requestAnimationFrame(callback: &JsFunction) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
            requestIdleCallback(callback: &JsFunction) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
            #[rename = + withOptions]
            requestIdleCallback(callback: &JsFunction, options: JsIdleRequestOptions) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy
            resizeBy(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo
            resizeTo(width: f64, height: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll
            scroll(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll
            #[rename = + withOptions]
            scroll(options: JsScrollToOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy
            scrollBy(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy
            #[rename = + withOptions]
            scrollBy(options: JsScrollToOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo
            scrollTo(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo
            #[rename = + withOptions]
            scrollTo(options: JsScrollToOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval
            setInterval(handler: &JsFunction, timeout: Option<i32>) -> i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout
            setTimeout(handler: &JsFunction, timeout: Option<i32>) -> i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker
            showDirectoryPicker(options: Option<JsDirectoryPickerOptions>) -> JsPromise<JsFileSystemDirectoryHandle>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker
            showOpenFilePicker(options: Option<JsOpenFilePickerOptions>) -> JsPromise<Vec<JsFileSystemFileHandle>>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker
            showSaveFilePicker(options: Option<JsSaveFilePickerOptions>) -> JsPromise<JsFileSystemFileHandle>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/stop
            stop() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/structuredClone
            structuredClone<T: IntoJs>(value: T, options: Option<JsStructuredSerializeOptions>) -> T::FromJs;
        }
        */
    }
);

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage
    class CacheStorage extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "CacheStorage";

        /*
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete
            delete(cache_name: impl IntoJs<str>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has
            has(cache_name: impl IntoJs<str>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys
            keys() -> Vec<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match
            #[rename = + byUrl]
            match(request: impl IntoJs<str>) -> Option<JsResponse>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match
            #[rename = + byUrlWithOptions]
            match(request: impl IntoJs<str>, options: JsCacheQueryOptions) -> Option<JsResponse>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open
            open(cache_name: String) -> JsPromise<JsCache>;
        }
        */
    }
);

/// https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match#options
/// 
/// An object whose properties control how matching is done in the match operation. The available options are:
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsCacheQueryOptions {
    /// A boolean value that specifies whether the matching process should ignore the query string in the URL. For example, if set to true, the ?value=bar part of http://foo.com/?value=bar would be ignored when performing a match. It defaults to false.
    #[serde(default)]
    ignore_search: bool,
    
    /// A boolean value that, when set to true, prevents matching operations from validating the Request http method (normally only GET and HEAD are allowed.) It defaults to false.
    #[serde(default)]
    ignore_method: bool,

    /// A boolean value that, when set to true, tells the matching operation not to perform VARY header matching. In other words, if the URL matches you will get a match regardless of whether the Response object has a VARY header or not. It defaults to false.
    #[serde(default)]
    ignore_vary: bool,

    /// A string that represents a specific cache to search within.
    #[serde(default)]
    cache_name: Option<String>,
}

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