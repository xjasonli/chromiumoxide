use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator
    class Generator extends Object {
        static #subtype: "generator";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next
            #[rename = +withValue]
            next<T: NativeValueFromJs, A: NativeValueIntoJs>(value: A) -> JsIteratorResult<T>;
        }
    }
);
