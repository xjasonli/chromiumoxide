use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection
    class HtmlCollection extends ArrayLike inherits Object {
        static #class: "HTMLCollection";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item
            item(index: usize) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem
            namedItem(name: &str) -> Option<JsElement>;
        }
    }
);
