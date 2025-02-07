use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement
    class HtmlLabelElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLLabelElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control
            control: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form
            form: Option<JsHtmlFormElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor
            htmlFor: String;
        }
    }
);
