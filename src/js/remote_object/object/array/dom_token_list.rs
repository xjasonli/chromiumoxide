use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList
    class DomTokenList extends ArrayLike inherits Object {
        static #class: "DOMTokenList";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value
            value: String;
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/item
            item(index: usize) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains
            contains(token: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add
            add<'a, I>(...tokens: I) -> ()
            where
                I: IntoIterator<Item = &'a str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove
            remove<'a, I>(...tokens: I) -> ()
            where
                I: IntoIterator<Item = &'a str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace
            replace(old_token: &str, new_token: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports
            supports(token: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle
            toggle(token: &str, force?: bool) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/forEach
            forEach(callback_fn: &Function, this?: &JsObject) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/values
            values() -> JsIterator;
        }
    }
);
