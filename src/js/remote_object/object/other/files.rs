use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileList
    class FileList extends Object {
        static #type: "object";
        static #subtype: "other";
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

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Blob
    class Blob extends Object {
        static #type: "object";
        static #subtype: "other";
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

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/File
    class File extends Blob inherits Object {
        static #type: "object";
        static #subtype: "other";
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
