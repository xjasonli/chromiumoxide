use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot
    class ShadowRoot extends DocumentFragment inherits Node, Object {
        static #class: "ShadowRoot";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/activeElement
            activeElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/adoptedStyleSheets
            adoptedStyleSheets: JsArray [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/clonable
            clonable: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/delegatesFocus
            delegatesFocus: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/fullscreenElement
            fullscreenElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host
            host: JsElement [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML
            #[rename = inner_html]
            innerHTML: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode
            mode: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pictureInPictureElement
            pictureInPictureElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pointerLockElement
            pointerLockElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/serializable
            serializable: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/slotAssignment
            slotAssignment: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/styleSheets
            styleSheets: JsObject [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getAnimations
            getAnimations() -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/setHTMLUnsafe
            #[rename = set_html_unsafe]
            setHTMLUnsafe<T: IntoJs<str>>(html: T) -> ();
        }
    }
);
