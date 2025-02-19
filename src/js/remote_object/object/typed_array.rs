use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray
    class TypedArray extends Object {
        static #type: "object";
        static #subtype: "typedarray";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/buffer
            buffer: JsArrayBuffer [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteLength
            byteLength: u64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteOffset
            byteOffset: u64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/length
            length: u64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/BYTES_PER_ELEMENT
            #[rename = bytes_per_element]
            BYTES_PER_ELEMENT: u64 [readonly];
        }
        methods: {
            //TODO
        }
    }
);
