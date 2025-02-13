use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement
    class HtmlButtonElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLButtonElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled
            disabled: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form
            form: Option<JsHtmlFormElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction
            formAction: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype
            formEnctype: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod
            formMethod: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate
            formNoValidate: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget
            formTarget: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels
            labels: JsNodeList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction
            popoverTargetAction: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement
            popoverTargetElement: Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type
            #[rename = typ]
            type: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage
            validationMessage: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity
            validity: JsValidityState [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value
            value: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate
            willValidate: bool [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity
            checkValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity
            reportValidity() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity
            setCustomValidity<T: IntoJs<str>>(message: T) -> ();
        }
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsValidityState {
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput
    #[serde(rename = "badInput")]
    pub bad_input: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError
    #[serde(rename = "customError")]
    pub custom_error: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch
    #[serde(rename = "patternMismatch")]
    pub pattern_mismatch: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow
    #[serde(rename = "rangeOverflow")]
    pub range_overflow: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow
    #[serde(rename = "rangeUnderflow")]
    pub range_underflow: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch
    #[serde(rename = "stepMismatch")]
    pub step_mismatch: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong
    #[serde(rename = "tooLong")]
    pub too_long: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort
    #[serde(rename = "tooShort")]
    pub too_short: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch
    #[serde(rename = "typeMismatch")]
    pub type_mismatch: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid
    pub valid: bool,
    
    /// https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing
    #[serde(rename = "valueMissing")]
    pub value_missing: bool,
}
