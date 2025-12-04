use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement>
    class HtmlSelectElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLSelectElement";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled>
            disabled: bool;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form>
            form: Option<JsHtmlFormElement> [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels>
            labels: JsNodeList [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length>
            length: u32;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple>
            multiple: bool;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name>
            name: String;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options>
            options: JsHtmlCollection [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required>
            required: bool;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex>
            selectedIndex: i32;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions>
            selectedOptions: JsHtmlCollection [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size>
            size: u32;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type>
            #[rename = type_]
            type: String [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage>
            validationMessage: String [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity>
            validity: JsValidityState [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value>
            value: String;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate>
            willValidate: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add>
            add(item: impl IntoJs<JsHtmlOptionElement>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add>
            #[rename = + before]
            add(item: impl IntoJs<JsHtmlOptionElement>, before: impl IntoJs<JsHtmlOptionElement>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add>
            #[rename = + beforeIndex]
            add(item: impl IntoJs<JsHtmlOptionElement>, before: impl IntoJs<i32>) -> ();
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity>
            checkValidity() -> bool;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item>
            item(index: impl IntoJs<u32>) -> Option<JsHtmlOptionElement>;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem>
            namedItem(name: impl IntoJs<String>) -> Option<JsHtmlOptionElement>;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove>
            remove(index: impl IntoJs<i32>) -> ();
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity>
            reportValidity() -> bool;
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity>
            setCustomValidity(message: impl IntoJs<String>) -> ();
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/showPicker>
            showPicker() -> ();
        }
    }
}
