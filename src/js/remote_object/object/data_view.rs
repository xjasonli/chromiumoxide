use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView
    class DataView extends Object {
        static #subtype: "dataview";


        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/buffer
            buffer: JsArrayBuffer [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteLength
            byteLength: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteOffset
            byteOffset: usize [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64
            getBigInt64(byte_offset: usize, little_endian: bool) -> JsBigInt;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigUint64
            getBigUint64(byte_offset: usize, little_endian: bool) -> JsBigInt;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat16
            getFloat16(byte_offset: usize, little_endian: bool) -> f32;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32
            getFloat32(byte_offset: usize, little_endian: bool) -> f32;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64
            getFloat64(byte_offset: usize, little_endian: bool) -> f64;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16
            getInt16(byte_offset: usize, little_endian: bool) -> i16;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32
            getInt32(byte_offset: usize, little_endian: bool) -> i32;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8
            getInt8(byte_offset: usize) -> i8;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16
            getUint16(byte_offset: usize, little_endian: bool) -> u16;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32
            getUint32(byte_offset: usize, little_endian: bool) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint8
            getUint8(byte_offset: usize) -> u8;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigInt64
            setBigInt64(byte_offset: usize, value: JsBigInt, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigUint64
            setBigUint64(byte_offset: usize, value: JsBigInt, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat16
            setFloat16(byte_offset: usize, value: f32, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat32
            setFloat32(byte_offset: usize, value: f32, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat64
            setFloat64(byte_offset: usize, value: f64, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt16
            setInt16(byte_offset: usize, value: i16, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt32
            setInt32(byte_offset: usize, value: i32, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt8
            setInt8(byte_offset: usize, value: i8) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint16
            setUint16(byte_offset: usize, value: u16, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint32
            setUint32(byte_offset: usize, value: u32, little_endian: bool) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint8
            setUint8(byte_offset: usize, value: u8) -> ();
        }
    }
);
