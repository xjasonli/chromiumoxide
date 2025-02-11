use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType
    class DocumentType extends Node inherits Object {
        static #class: "DocumentType";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/name
            name: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/publicId
            publicId: String [readonly];

            // https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/systemId
            systemId: String [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after
            after<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after
            #[rename = +text]
            after<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before
            before<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before
            #[rename = +text]
            before<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/remove
            remove() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith
            replaceWith<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith
            #[rename = +text]
            replaceWith<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;
        }
    }
);
