use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/Storage
    class Storage extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Storage";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/length
            length: u32 [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear
            clear() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem
            getItem(key: impl IntoJs<String>) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/key
            key(index: u32) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem
            removeItem(key: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem
            setItem(key: impl IntoJs<String>, value: impl IntoJs<String>) -> ();
        }
    }
}
