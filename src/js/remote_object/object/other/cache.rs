use super::*;

js_remote_object!(
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage>
    class CacheStorage extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "CacheStorage";

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete>
            delete(cache_name: impl IntoJs<String>) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has>
            has(cache_name: impl IntoJs<String>) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys>
            keys() -> Vec<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match>
            #[rename = + byUrl]
            match(request: impl IntoJs<String>) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match>
            #[rename = + byUrlWithOptions]
            match(request: impl IntoJs<String>, options: JsCacheQueryOptions) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open>
            open(cache_name: String) -> JsCache;
        }
    }
);

/// <https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match#options>
/// 
/// An object whose properties control how matching is done in the match operation. The available options are:
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsCacheQueryOptions {
    #[allow(rustdoc::bare_urls)]
    /// A boolean value that specifies whether the matching process should ignore the query string in the URL. For example, if set to true, the ?value=bar part of http://foo.com/?value=bar would be ignored when performing a match. It defaults to false.
    #[serde(default)]
    ignore_search: bool,
    
    /// A boolean value that, when set to true, prevents matching operations from validating the Request http method (normally only GET and HEAD are allowed.) It defaults to false.
    #[serde(default)]
    ignore_method: bool,

    /// A boolean value that, when set to true, tells the matching operation not to perform VARY header matching. In other words, if the URL matches you will get a match regardless of whether the Response object has a VARY header or not. It defaults to false.
    #[serde(default)]
    ignore_vary: bool,

    /// A string that represents a specific cache to search within.
    #[serde(default)]
    cache_name: Option<String>,
}

js_remote_object!(
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache>
    class Cache extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Cache";

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/add>
            add(request: impl IntoJs<JsRequest>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll>
            addAll(requests: impl IntoJs<Vec<JsRequest>>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete>
            delete(request: impl IntoJs<JsRequest>, options?: JsCacheQueryOptions) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys>
            keys() -> Vec<JsRequest>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys>
            #[rename = + withOptions]
            keys(options?: JsCacheQueryOptions) -> Vec<JsRequest>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/match>
            #[rename = + byUrl]
            match(request: impl IntoJs<String>) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/match>
            #[rename = + byUrlWithOptions]
            match(request: impl IntoJs<String>, options?: JsCacheQueryOptions) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/match>
            #[rename = + byRequest]
            match(request: impl IntoJs<JsRequest>) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/match>
            #[rename = + byRequestWithOptions]
            match(request: impl IntoJs<JsRequest>, options?: JsCacheQueryOptions) -> Option<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll>
            matchAll() -> Vec<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll>
            #[rename = + withRequest]
            matchAll(request: impl IntoJs<JsRequest>) -> Vec<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll>
            #[rename = + withRequestAndOptions]
            matchAll(request: impl IntoJs<JsRequest>, options?: JsCacheQueryOptions) -> Vec<JsResponse>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Cache/put>
            put(request: impl IntoJs<JsRequest>, response: impl IntoJs<JsResponse>) -> ();
        }
    }
);
