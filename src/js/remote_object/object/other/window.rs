use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window
    class Window extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Window";

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
            crypto: JsObject /* JsCrypto */ [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements
            customElements: JsObject /* JsCustomElementRegistry */ [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio
            devicePixelRatio: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/document
            document: JsDocument [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/documentPictureInPicture
            documentPictureInPicture: JsObject /* JsDocumentPictureInPicture */ [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fence
            fence: JsObject /* JsFenceObject */ [readonly];

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
            launchQueue: JsObject /* JsLaunchQueue */ [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/length
            length: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
            localStorage: JsStorage [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/location
            location: JsLocation [readonly];

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
            #[rename = + window]
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
            createImageBitmap(image: JsImageBitmapSource) -> JsImageBitmap;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch
            fetch(url: impl IntoJs<String>) -> JsResponse;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch
            #[rename = + withOptions]
            fetch<T: IntoJsAny>(url: impl IntoJs<String>, options: JsRequestInit<T>) -> JsResponse;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch
            #[rename = + byRequest]
            fetch(request: impl IntoJs<JsRequest>) -> JsResponse;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch
            #[rename = + byRequestWithOptions]
            fetch<T: IntoJsAny>(request: impl IntoJs<JsRequest>, options: JsRequestInit<T>) -> JsResponse;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/focus
            focus() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle
            getComputedStyle(element: &JsElement) -> JsCssStyleDeclaration;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle
            #[rename = + withPseudoElement]
            getComputedStyle(element: &JsElement, pseudo_element: Option<String>) -> JsCssStyleDeclaration;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getScreenDetails
            getScreenDetails() -> JsScreenDetails;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection
            getSelection() -> Option<JsSelection>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia
            matchMedia(query: String) -> JsMediaQueryList;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy
            moveBy(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo
            moveTo(x: f64, y: f64) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/open
            open(url?: Option<String>, target?: Option<String>, features?: Option<String>) -> Option<JsWindow>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage
            postMessage<T: IntoJsAny>(message: T, target_origin: String, transfer: Option<Vec<JsTransferable>>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/print
            print() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt
            prompt(message: Option<String>, default: Option<String>) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts
            queryLocalFonts() -> Vec<JsFontData>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/queueMicrotask
            queueMicrotask(callback: impl IntoJs<JsFunction>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame
            requestAnimationFrame(callback: impl IntoJs<JsFunction>) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
            requestIdleCallback(callback: impl IntoJs<JsFunction>) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
            #[rename = + withOptions]
            requestIdleCallback(callback: impl IntoJs<JsFunction>, options: JsIdleRequestOptions) -> u32;

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
            setInterval(handler: impl IntoJs<JsFunction>, timeout?: i32) -> i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout
            setTimeout(handler: impl IntoJs<JsFunction>, timeout?: i32) -> i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker
            showDirectoryPicker(options?: JsDirectoryPickerOptions) -> JsFileSystemDirectoryHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker
            showOpenFilePicker(options?: JsOpenFilePickerOptions) -> Vec<JsFileSystemFileHandle>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker
            showSaveFilePicker(options?: JsSaveFilePickerOptions) -> JsFileSystemFileHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/stop
            stop() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/structuredClone
            structuredClone<T: FromJsAny>(value: impl IntoJsAny, options?: JsStructuredSerializeOptions) -> T;
        }
    }
);
