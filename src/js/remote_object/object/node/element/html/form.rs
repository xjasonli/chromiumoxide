use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement
    class HtmlFormElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLFormElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements
            elements: JsHtmlCollection [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length
            length: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method
            method: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target
            target: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action
            action: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding
            encoding: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype
            enctype: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset
            acceptCharset: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete
            autocomplete: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate
            noValidate: bool;
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity
            checkValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity
            reportValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit
            requestSubmit() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit
            #[rename = +withSubmitter]
            requestSubmit<T: Class<JsHtmlElement>>(submitter?: T) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset
            reset() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit
            submit() -> ();
        }
    }
);

