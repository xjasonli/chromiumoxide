use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map
    class Map extends Object {
        static #subtype: "map";

        properties: {
            size: usize [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/clear
            clear() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/delete
            delete<K: NativeValueIntoJs>(key: K) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach
            forEach(callback: Function, this_arg?: &JsRemoteObject) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/get
            get<K: NativeValueIntoJs, V: NativeValueFromJs>(key: K) -> Optional<V>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/has
            has<K: NativeValueIntoJs>(key: K) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/set
            set<K: NativeValueIntoJs, V: NativeValueIntoJs>(key: K, value: V) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/values
            values() -> JsIterator;
        }
    }
);
