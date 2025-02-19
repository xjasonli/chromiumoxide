use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator
    class Generator extends Object {
        static #type: "object";
        static #subtype: "generator";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next
            #[rename = +withValue]
            next<T: FromJs, A: IntoJs>(value: A) -> JsIteratorResult<T>;
        }
    }
);
