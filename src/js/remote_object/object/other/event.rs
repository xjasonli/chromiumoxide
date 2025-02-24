use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Event
    class Event extends Object {
        static #type: "object";
        static #subtype: "other";
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
