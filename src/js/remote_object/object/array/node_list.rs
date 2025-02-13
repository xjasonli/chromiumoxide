use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList
    class NodeList extends ArrayLike inherits Object {
        static #class: "NodeList";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item
            item(index: usize) -> Option<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/forEach
            forEach(callback_fn: impl IntoJs<JsFunction>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/forEach
            #[rename = + withThis]
            forEach(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/NodeList/values
            values() -> JsIterator;
        }
    }
);
