use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory>
    class IDBFactory extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBFactory";

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp>
            cmp(first: impl IntoJsAny, second: impl IntoJsAny) -> i32;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/databases>
            databases() -> Vec<JsIDBDatabaseInfo>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase>
            deleteDatabase(name: impl IntoJs<String>) -> JsIDBOpenDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open>
            open(name: impl IntoJs<String>) -> JsIDBOpenDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open>
            #[rename = + withVersion]
            open(name: impl IntoJs<String>, version: u64) -> JsIDBOpenDBRequest;
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/databases#return_value>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsIDBDatabaseInfo {
    /// The name of the database
    pub name: String,

    /// The version of the database
    pub version: u64,
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest>
    class IDBOpenDBRequest extends IDBRequest inherits EventTarget, Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBOpenDBRequest";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked>
            onblocked: Option<JsFunction>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded>
            onupgradeneeded: Option<JsFunction>;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest>
    class IDBRequest extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDB*Request";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error>
            error: Option<JsObject> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror>
            onerror: Option<JsFunction>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess>
            onsuccess: Option<JsFunction>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState>
            readyState: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result>
            result: JsObject [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source>
            source: JsObject [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction>
            transaction: Option<JsIDBTransaction> [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction>
    class IDBTransaction extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBTransaction";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db>
            db: JsIDBDatabase [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/durability>
            durability: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error>
            error: Option<JsObject> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode>
            mode: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames>
            objectStoreNames: Vec<String> [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort>
            abort() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/commit>
            commit() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore>
            objectStore(name: impl IntoJs<String>) -> JsIDBObjectStore;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase>
    class IDBDatabase extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBDatabase";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name>
            name: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames>
            objectStoreNames: Vec<String> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version>
            version: u64 [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close>
            close() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore>
            createObjectStore(name: impl IntoJs<String>) -> JsIDBObjectStore;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore>
            #[rename = + withOptions]
            createObjectStore(name: impl IntoJs<String>, options: JsIDBObjectStoreParameters) -> JsIDBObjectStore;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore>
            deleteObjectStore(name: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction>
            transaction(storeNames: impl IntoJs<Vec<String>>) -> JsIDBTransaction;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction>
            #[rename = + withMode]
            transaction(storeNames: impl IntoJs<Vec<String>>, mode: impl IntoJs<String>) -> JsIDBTransaction;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction>
            #[rename = + withModeAndOptions]
            transaction(storeNames: impl IntoJs<Vec<String>>, mode: impl IntoJs<String>, options: JsIDBTransactionOptions) -> JsIDBTransaction;
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsIDBObjectStoreParameters {
    /// The key path for the object store
    #[serde(default)]
    pub key_path: Option<String>,

    /// Whether to create an auto-incrementing key
    #[serde(default)]
    pub auto_increment: Option<bool>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsIDBTransactionOptions {
    /// The durability hint for the transaction
    #[serde(default)]
    pub durability: Option<String>,
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore>
    class IDBObjectStore extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBObjectStore";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement>
            autoIncrement: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames>
            indexNames: Vec<String> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath>
            keyPath: Option<String> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name>
            name: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction>
            transaction: JsIDBTransaction [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add>
            add(value: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add>
            #[rename = + withKey]
            add(value: impl IntoJsAny, key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear>
            clear() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count>
            count() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count>
            #[rename = + withKey]
            count(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex>
            createIndex(name: impl IntoJs<String>, keyPath: impl IntoJsAny) -> JsIDBIndex;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex>
            #[rename = + withOptions]
            createIndex(name: impl IntoJs<String>, keyPath: impl IntoJsAny, options: JsIDBIndexParameters) -> JsIDBIndex;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete>
            delete(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex>
            deleteIndex(name: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get>
            get(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll>
            getAll() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll>
            #[rename = + withKey]
            getAll(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll>
            #[rename = + withKeyAndCount]
            getAll(key: impl IntoJsAny, count: u32) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys>
            getAllKeys() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys>
            #[rename = + withKey]
            getAllKeys(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys>
            #[rename = + withKeyAndCount]
            getAllKeys(key: impl IntoJsAny, count: u32) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey>
            getKey(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index>
            index(name: impl IntoJs<String>) -> JsIDBIndex;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor>
            openCursor() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor>
            #[rename = + withKey]
            openCursor(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor>
            #[rename = + withKeyAndDirection]
            openCursor(key: impl IntoJsAny, direction: impl IntoJs<String>) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor>
            openKeyCursor() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor>
            #[rename = + withKey]
            openKeyCursor(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor>
            #[rename = + withKeyAndDirection]
            openKeyCursor(key: impl IntoJsAny, direction: impl IntoJs<String>) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put>
            put(value: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put>
            #[rename = + withKey]
            put(value: impl IntoJsAny, key: impl IntoJsAny) -> JsIDBRequest;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex>
    class IDBIndex extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "IDBIndex";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/keyPath>
            keyPath: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry>
            multiEntry: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name>
            name: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore>
            objectStore: JsIDBObjectStore [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique>
            unique: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count>
            count() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count>
            #[rename = + withKey]
            count(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/get>
            get(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll>
            getAll() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll>
            #[rename = + withKey]
            getAll(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll>
            #[rename = + withKeyAndCount]
            getAll(key: impl IntoJsAny, count: u32) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys>
            getAllKeys() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys>
            #[rename = + withKey]
            getAllKeys(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys>
            #[rename = + withKeyAndCount]
            getAllKeys(key: impl IntoJsAny, count: u32) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getKey>
            getKey(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor>
            openCursor() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor>
            #[rename = + withKey]
            openCursor(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor>
            #[rename = + withKeyAndDirection]
            openCursor(key: impl IntoJsAny, direction: impl IntoJs<String>) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor>
            openKeyCursor() -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor>
            #[rename = + withKey]
            openKeyCursor(key: impl IntoJsAny) -> JsIDBRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor>
            #[rename = + withKeyAndDirection]
            openKeyCursor(key: impl IntoJsAny, direction: impl IntoJs<String>) -> JsIDBRequest;
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsIDBIndexParameters {
    /// Whether the index enforces unique values
    #[serde(default)]
    pub unique: Option<bool>,

    /// Whether the index allows multiple entries for a key
    #[serde(default)]
    pub multi_entry: Option<bool>,
} 