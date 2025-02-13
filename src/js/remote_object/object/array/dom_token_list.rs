use super::*;

js_remote_object!(
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
            contains(token: impl IntoJs<str>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add
            add<'a, I, T>(...tokens: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove
            remove<'a, I, T>(...tokens: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace
            replace(old_token: impl IntoJs<str>, new_token: impl IntoJs<str>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports
            supports(token: impl IntoJs<str>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle
            toggle(token: impl IntoJs<str>, force?: bool) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/forEach
            forEach(callback_fn: impl IntoJs<JsFunction>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/forEach
            #[rename = + withThis]
            forEach(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/values
            values() -> JsIterator;
        }
    }
);
