use super::*;
use serde_json::Value as JsonValue;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Request#instance_methods>
    class Request extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Request";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/body>
            body: Option<JsReadableStream> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed>
            bodyUsed: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/cache>
            cache: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials>
            credentials: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/destination>
            destination: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/duplex>
            duplex: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/headers>
            headers: JsHeaders [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity>
            integrity: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/isHistoryNavigation>
            isHistoryNavigation: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/isReloadNavigation>
            isReloadNavigation: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/keepalive>
            keepalive: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/method>
            method: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/mode>
            mode: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/priority>
            priority: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect>
            redirect: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer>
            referrer: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy>
            referrerPolicy: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/signal>
            signal: JsAbortSignal [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/url>
            url: String [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer>
            arrayBuffer() -> JsArrayBuffer;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/blob>
            blob() -> JsBlob;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/clone>
            clone() -> JsRequest;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/formData>
            formData() -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/json>
            json() -> JsonValue;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Request/text>
            text() -> String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Response>
    class Response extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Response";
        
        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/body>
            body: Option<JsReadableStream> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed>
            bodyUsed: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/headers>
            headers: JsHeaders [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/ok>
            ok: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected>
            redirected: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/status>
            status: u16 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText>
            statusText: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/type>
            #[rename = typ]
            type: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/url>
            url: String [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer>
            arrayBuffer() -> JsArrayBuffer;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/blob>
            blob() -> JsBlob;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/clone>
            clone() -> JsResponse;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/formData>
            formData() -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/json>
            json() -> JsonValue;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Response/text>
            text() -> String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers>
    class Headers extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Headers";
        
        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/append>
            append(name: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete>
            delete(name: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries>
            entries() -> JsIterator;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach>
            forEach(callback: impl IntoJs<JsFunction>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/get>
            get(name: impl IntoJs<String>) -> Option<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/getSetCookie>
            getSetCookie() -> Vec<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/has>
            has(name: impl IntoJs<String>) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys>
            keys() -> JsIterator;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/set>
            set(name: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Headers/values>
            values() -> JsIterator;
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#parameters>
/// 
/// RequestInit is a dictionary that contains any custom settings that you want to apply to the request.
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsRequestInit<T> {
    /// Indicates that you want the request's response to be able to register a JavaScript-based attribution source or attribution trigger.
    #[serde(default)]
    pub attribution_reporting: Option<JsAttributionReporting>,

    /// The request body contains content to send to the server.
    /// It can be a string, ArrayBuffer, Blob, DataView, File, FormData, TypedArray, URLSearchParams, or ReadableStream.
    #[serde(default)]
    pub body: Option<T>,

    /// A boolean specifying that the selected topics for the current user should be sent in a Sec-Browsing-Topics header.
    #[serde(default)]
    pub browsing_topics: Option<bool>,

    /// The cache mode.
    /// One of: "default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"
    #[serde(default)]
    pub cache: Option<String>,

    /// The request credentials mode.
    /// One of: "omit", "same-origin", "include"
    #[serde(default)]
    pub credentials: Option<String>,

    /// The duplex mode to use for the request.
    /// One of: "half"
    #[serde(default)]
    pub duplex: Option<String>,

    /// The request headers.
    #[serde(default)]
    pub headers: Option<JsHeaders>,

    /// A cryptographic hash of the resource to be fetched by request.
    #[serde(default)]
    pub integrity: Option<String>,

    /// A boolean to set request's keepalive.
    #[serde(default)]
    pub keepalive: Option<bool>,

    /// The request method, e.g., GET, POST.
    #[serde(default)]
    pub method: Option<String>,

    /// The mode for how to handle cross-origin requests.
    /// One of: "cors", "no-cors", "same-origin", "navigate"
    #[serde(default)]
    pub mode: Option<String>,

    /// The priority of the request.
    /// One of: "high", "low", "auto"
    #[serde(default)]
    pub priority: Option<String>,

    /// The redirect mode.
    /// One of: "follow", "error", "manual"
    #[serde(default)]
    pub redirect: Option<String>,

    /// A string specifying the referrer of the request.
    /// This can be a same-origin URL, "", or "about:client".
    #[serde(default)]
    pub referrer: Option<String>,

    /// Specifies the referrer policy to use for the request.
    /// One of: "no-referrer", "no-referrer-when-downgrade", "origin", "origin-when-cross-origin",
    /// "same-origin", "strict-origin", "strict-origin-when-cross-origin", "unsafe-url"
    #[serde(default)]
    pub referrer_policy: Option<String>,

    /// An AbortSignal object instance; allows you to communicate with a fetch request and abort it if desired.
    #[serde(default)]
    pub signal: Option<JsAbortSignal>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/RequestInit#attributionreporting>
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsAttributionReporting {
    /// If set to true, the request's response is eligible to register an attribution source.
    #[serde(default)]
    pub event_source_eligible: bool,

    /// If set to true, the request's response is eligible to register an attribution trigger.
    #[serde(default)]
    pub trigger_eligible: bool,
}
