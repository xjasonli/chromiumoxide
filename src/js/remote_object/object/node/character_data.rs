use super::*;

pub mod text;
pub mod comment;
pub mod processing_instruction;

pub use text::*;
pub use comment::*;
pub use processing_instruction::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData
    class CharacterData extends Node inherits Object {
        static #class: [
            "CharacterData",
            "Text",
            "Comment",
            "CDATASection",
            "ProcessingInstruction",
        ];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data
            data: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length
            length: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling
            next_element_sibling: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling
            previous_element_sibling: Option<JsElement> [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after
            after<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after
            #[rename = +text]
            after<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData
            appendData(data: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before
            before<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before
            #[rename = +text]
            before<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData
            deleteData(offset: usize, count: usize) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData
            insertData(offset: usize, data: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove
            remove() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData
            replaceData(offset: usize, count: usize, data: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith
            replaceWith<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith
            #[rename = +text]
            replaceWith<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData
            substringData(offset: usize, count: usize) -> String;
        }
    }
);
