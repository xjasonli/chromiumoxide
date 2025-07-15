use super::*;

js_remote_object!(
    /// <https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal>
    class AbortSignal extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "AbortSignal";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted>
            aborted: bool [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/reason>
            reason: Optional<JsObject> [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/throwIfAborted>
            throwIfAborted() -> ();
        }
    }
);
