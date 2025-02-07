use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Document
    class HtmlDocument extends Document inherits Node, Object {
        static #class: "HTMLDocument";

        // https://developer.mozilla.org/en-US/docs/Web/API/Document#extensions_for_htmldocument
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie
            cookie: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView
            defaultView: Option<JsWindow> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/designMode
            designMode: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/dir
            dir: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/fullScreenEnabled
            fullScreenEnabled: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified
            lastModified: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/location
            location: Option<JsLocation> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState
            readyState: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer
            referrer: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/title
            title: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/URL
            #[rename = url]
            URL: String [readonly];

            /// Overrides Document properties

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement
            documentElement: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild
            firstElementChild: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild
            lastElementChild: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureElement
            pictureInPictureElement: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement
            pointerLockElement: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement
            scrollingElement: Option<JsHtmlElement> [readonly];
        }

        // https://developer.mozilla.org/en-US/docs/Web/API/Document#extension_for_html_documents
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/close
            close() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName
            getElementsByName<T: Class<str>>(name: T) -> JsNodeList;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus
            hasFocus() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/open
            open() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/write
            write<T: Class<str>>(text: T) -> ();
        }
    }
);
