use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction
    class ProcessingInstruction extends CharacterData inherits Node, Object {
        static #class: "ProcessingInstruction";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet
            sheet: Option<JsObject> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target
            target: String [readonly];
        }
    }
);
