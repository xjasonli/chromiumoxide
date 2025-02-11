use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/CDATASection
    class CdataSection extends Text inherits CharacterData, Node, Object {
        static #class: "CDATASection";
    }
);
