use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Attr
    class Attr extends Node inherits Object {
        static #class: "Attr";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/localName
            localName: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/name
            name: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/namespaceURI
            #[rename = namespace_uri]
            namespaceURI: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/ownerElement
            ownerElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/prefix
            prefix: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/specified
            specified: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Attr/value
            value: String;
        }
    }
);
