use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator
    class Iterator extends Object {
        static #subtype: ["iterator", "generator"];

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#next
            next<T: NativeValueFromJs>() -> JsIteratorResult<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/drop
            drop(limit: u32) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/every
            every(callback: Function) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/filter
            filter(callback: Function) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/find
            find<T: NativeValueFromJs>(callback: Function) -> Optional<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/flatMap
            flatMap(callback: Function) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/forEach
            forEach(callback: Function) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/map
            map(callback: Function) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/reduce
            reduce<T>(callback: Function, initial_value?: T) -> Optional<T>
            where
                T: NativeValue;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/some
            some(callback: Function) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/take
            take(limit: u32) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/toArray
            toArray() -> JsArray;

            /// Extension method
            toVec<T: NativeValueFromJs>() -> Vec<T> {
                return this.toArray();
            }
        }
    }
);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsIteratorResult<T> {
    pub value: Optional<T>,
    pub done: Optional<bool>,
}
