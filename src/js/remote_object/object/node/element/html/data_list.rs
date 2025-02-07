use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement
    class HtmlDataListElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLDataListElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement/options
            options: JsHtmlCollection [readonly];
        }
    }
);
