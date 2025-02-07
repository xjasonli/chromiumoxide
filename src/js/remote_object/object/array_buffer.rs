use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer
    class ArrayBuffer extends Object {
        static #subtype: "arraybuffer";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/byteLength
            byteLength: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/detached
            detached: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/maxByteLength
            maxByteLength: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resizeable
            resizeable: bool [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resize
            resize(new_length: usize) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice
            slice(start?: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer
            transfer(new_byte_length: Optional<usize>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transferToFixedLength
            transferToFixedLength(new_byte_length: Optional<usize>) -> Self;
        }
    }
);
