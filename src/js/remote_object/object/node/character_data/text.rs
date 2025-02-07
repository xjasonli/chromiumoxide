use super::*;

mod cdata_section;
pub use cdata_section::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Text
    class Text extends CharacterData inherits Node, Object {
        static #class: "Text";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Text/assignedSlot
            assignedSlot: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Text/wholeText
            wholeText: String [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Text/splitText
            splitText(offset: usize) -> Self;
        }
    }
);
