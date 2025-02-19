use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map
    class Map extends Object {
        static #type: "object";
        static #subtype: "map";

        properties: {
            size: usize [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/clear
            clear() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/delete
            delete<K: IntoJs>(key: K) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach
            forEach(callback: impl IntoJs<JsFunction>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach
            #[rename = + withThis]
            forEach(callback: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/get
            get<K: IntoJs, V: FromJs>(key: K) -> Optional<V>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/has
            has<K: IntoJs>(key: K) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/set
            set<K: IntoJs, V: IntoJs>(key: K, value: V) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/values
            values() -> JsIterator;
        }
    }
);
