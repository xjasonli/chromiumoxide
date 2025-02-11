use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement
    class HtmlIFrameElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLIFrameElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allow
            allow: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen
            allowFullscreen: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument
            contentDocument: Option<JsDocument> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow
            contentWindow: Option<JsWindow> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/credentialless
            credentialless: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/csp
            csp: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height
            height: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/loading
            loading: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy
            referrerPolicy: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox
            sandbox: JsDomTokenList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src
            src: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc
            srcdoc: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width
            width: String;
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument
            getSVGDocument() -> Option<JsDocument>;
        }
    }
);
