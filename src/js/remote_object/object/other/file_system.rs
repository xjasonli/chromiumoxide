use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle
    class FileSystemHandle extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "FileSystemHandle";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind
            kind: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name
            name: String [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/isSameEntry
            isSameEntry(other: impl IntoJs<JsFileSystemHandle>) -> bool;
        }
    }
}

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle
    class FileSystemFileHandle extends FileSystemHandle inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "FileSystemFileHandle";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile
            getFile() -> JsFile;

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable
            createWritable() -> JsWritableStream;
        }
    }
}

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle
    class FileSystemDirectoryHandle extends FileSystemHandle inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "FileSystemDirectoryHandle";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle
            getDirectoryHandle(name: impl IntoJs<String>) -> JsFileSystemDirectoryHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle
            #[rename = + withOptions]
            getDirectoryHandle(name: impl IntoJs<String>, options: JsFileSystemGetHandleOptions) -> JsFileSystemDirectoryHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle
            getFileHandle(name: impl IntoJs<String>) -> JsFileSystemFileHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle
            #[rename = + withOptions]
            getFileHandle(name: impl IntoJs<String>, options: JsFileSystemGetHandleOptions) -> JsFileSystemFileHandle;

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry
            removeEntry(name: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry
            #[rename = + withOptions]
            removeEntry(name: impl IntoJs<String>, options: JsFileSystemRemoveOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve
            resolve(possibleDescendant: impl IntoJs<JsFileSystemHandle>) -> Option<Vec<String>>;
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsFileSystemGetHandleOptions {
    /// Whether to create the entry if it does not exist
    #[serde(default)]
    pub create: Option<bool>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry#parameters
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JsFileSystemRemoveOptions {
    /// Whether to remove the entry recursively
    pub recursive: bool,
} 
 