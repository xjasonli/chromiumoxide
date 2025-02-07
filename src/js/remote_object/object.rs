use super::*;

pub mod array;
pub mod node;
pub mod reg_exp;
pub mod date;
pub mod map;
pub mod set;
pub mod weak_map;
pub mod weak_set;
pub mod iterator;
pub mod generator;
pub mod error;
pub mod proxy;
pub mod promise;
pub mod typed_array;
pub mod array_buffer;
pub mod data_view;
pub mod wasm_memory;
pub mod wasm_value;

pub use array::*;
pub use node::*;
pub use reg_exp::*;
pub use date::*;
pub use map::*;
pub use set::*;
pub use weak_map::*;
pub use weak_set::*;
pub use iterator::*;
pub use generator::*;
pub use error::*;
pub use proxy::*;
pub use promise::*;
pub use typed_array::*;
pub use array_buffer::*;
pub use data_view::*;
pub use wasm_memory::*;
pub use wasm_value::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    class Object {
        static #type: "object";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto
            #[rename = proto]
            __proto__: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor
            constructor: JsFunction [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
            hasOwnProperty(key: String) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
            isPrototypeOf(value: &JsObject) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
            propertyIsEnumerable(key: AnyOf2<&str, &JsSymbol>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
            toString() -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf
            valueOf() -> Self;
        }
    }
);

impl JsObject {
    pub fn object_subtype(&self) -> Option<JsObjectSubtype> {
        self.object_type().object_subtype()
    }
}

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window
    class Window extends Object {
        static #subtype: ();
        static #class: "Window";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Window/document
            document: JsDocument [readonly];

            // todo
        }
        methods: {
            // todo
        }
    }
);

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Location
    class Location extends Object {
        static #subtype: ();
        static #class: "Location";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/ancestorOrigins
            ancestorOrigins: Vec<String> [readonly] {
                get() {
                    return Array.from(this.ancestorOrigins);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/href
            href: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol
            protocol: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/host
            host: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname
            hostname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/port
            port: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname
            pathname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/search
            search: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/hash
            hash: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/origin
            origin: String [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/assign
            assign(url: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/reload
            reload() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Location/replace
            replace<T: NativeValueIntoJs>(url: T) -> ();
        }
    }
);

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileList
    class FileList extends Object {
        static #subtype: ();
        static #class: "FileList";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileList/length
            length: u32 [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileList/item
            item(index: u32) -> Option<JsFile>;
        }
    }
);

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Blob
    class Blob extends Object {
        static #subtype: ();
        static #class: ["Blob", "File"];
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/size
            size: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/type
            #[rename = typ]
            type: String [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer
            arrayBuffer() -> JsArrayBuffer;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/bytes
            bytes() -> JsTypedArray;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice
            slice(start?: isize, end?: isize, content_type?: &str) -> JsBlob;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/stream
            stream() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Blob/text
            text() -> String;
        }
    }
);

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/File
    class File extends Blob inherits Object {
        static #subtype: ();
        static #class: "File";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified
            lastModified: u64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/File/name
            name: String [readonly];

            // https://developer.mozilla.org/en-US/docs/Web/API/File/webkitRelativePath
            webkitRelativePath: String [readonly];
        }
    }
);
