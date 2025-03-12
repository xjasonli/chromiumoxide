use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement
    class HtmlOptionElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLOptionElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected
            defaultSelected: bool;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled
            disabled: bool;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form
            form: Option<JsHtmlFormElement> [readonly];
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index
            index: u32 [readonly];
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label
            label: String;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected
            selected: bool;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text
            text: String;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value
            value: String;
        }
    }
}
