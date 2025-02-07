use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement
    class HtmlInputElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLInputElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue
            defaultValue: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/dirName
            dirName: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels
            labels: Option<JsNodeList> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/list
            list: Option<JsHtmlDataListElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple
            multiple: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetAction
            popoverTargetAction: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetElement
            popoverTargetElement: Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step
            step: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type
            #[rename = typ]
            type: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value
            value: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsDate
            valueAsDate: Option<JsDate>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber
            valueAsNumber: Option<f64>;


            // Instance properties related to the parent form

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/form
            form: Option<JsHtmlFormElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction
            formAction: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype
            formEnctype: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod
            formMethod: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate
            formNoValidate: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget
            formTarget: String;


            // Instance properties that apply to any type of input element that is not hidden

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled
            disabled: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required
            required: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validationMessage
            validationMessage: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validity
            validity: JsValidityState [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/willValidate
            willValidate: bool [readonly];


            // Instance properties that apply only to elements of type checkbox or radio

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked
            checked: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked
            defaultChecked: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate
            indeterminate: bool;


            // Instance properties that apply only to elements of type image

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt
            alt: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height
            height: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src
            src: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width
            width: u32;


            // Instance properties that apply only to elements of type file

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept
            accept: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/capture
            capture: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files
            files: Option<JsFileList> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory
            webkitdirectory: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitEntries
            webkitEntries: Option<JsArray>;


            // Instance properties that apply only to visible elements containing text or numbers

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete
            autocomplete: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max
            max: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength
            maxLength: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min
            min: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength
            minLength: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern
            pattern: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder
            placeholder: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly
            readOnly: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection
            selectionDirection: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd
            selectionEnd: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart
            selectionStart: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size
            size: u32;
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity
            checkValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity
            reportValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select
            select() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setCustomValidity
            setCustomValidity(message: String) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText
            setRangeText(replacement: String, start?: u32, end?: u32, select_mode?: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange
            setSelectionRange(start: u32, end: u32, direction?: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/showPicker
            showPicker() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepDown
            stepDown(step: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepUp
            stepUp(step?: i32) -> ();
        }
    }
);
