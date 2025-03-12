use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement
    class HtmlTextAreaElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLTextAreaElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete
            autocomplete: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols
            cols: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue
            defaultValue: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled
            disabled: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form
            form: Option<JsHtmlFormElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength
            maxLength: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength
            minLength: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder
            placeholder: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly
            readOnly: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required
            required: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows
            rows: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection
            selectionDirection: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd
            selectionEnd: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart
            selectionStart: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength
            textLength: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type
            #[rename = typ]
            type: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage
            validationMessage: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity
            validity: JsValidityState [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value
            value: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate
            willValidate: bool [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity
            checkValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity
            reportValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select
            select() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity
            setCustomValidity(message: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText
            setRangeText(replacement: impl IntoJs<String>, start: impl IntoJs<u32>, end: impl IntoJs<u32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText
            #[rename = + withMode]
            setRangeText(replacement: impl IntoJs<String>, start: impl IntoJs<u32>, end: impl IntoJs<u32>, select_mode: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange
            setSelectionRange(start: impl IntoJs<u32>, end: impl IntoJs<u32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange
            #[rename = + withDirection]
            setSelectionRange(start: impl IntoJs<u32>, end: impl IntoJs<u32>, direction: impl IntoJs<String>) -> ();
        }
    }
}
