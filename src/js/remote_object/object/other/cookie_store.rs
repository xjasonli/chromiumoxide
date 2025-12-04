use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore>
    class CookieStore extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "CookieStore";

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete>
            delete(name: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete>
            #[rename = + withOptions]
            delete(options: JsCookieStoreDeleteOptions) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get>
            get(name: impl IntoJs<String>) -> Option<JsCookie>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get>
            #[rename = + withOptions]
            get(options: JsCookieStoreGetOptions) -> Option<JsCookie>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll>
            getAll() -> Vec<JsCookie>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll>
            #[rename = + withName]
            getAll(name: impl IntoJs<String>) -> Vec<JsCookie>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll>
            #[rename = + withOptions]
            getAll(options: JsCookieStoreGetOptions) -> Vec<JsCookie>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set>
            set(name: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set>
            #[rename = + withOptions]
            set(options: JsCookieStoreSetOptions) -> ();
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsCookieStoreDeleteOptions {
    /// The name of the cookie to delete
    pub name: String,

    /// The path of the cookie to delete
    #[serde(default)]
    pub path: Option<String>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsCookieStoreGetOptions {
    /// The name of the cookie to get
    pub name: String,

    /// The URL path of the cookie to get
    #[serde(default)]
    pub path: Option<String>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsCookieStoreSetOptions {
    /// The name of the cookie
    pub name: String,

    /// The value of the cookie
    pub value: String,

    /// The path of the cookie
    #[serde(default)]
    pub path: Option<String>,

    /// The expiration date of the cookie as the number of milliseconds since the Unix epoch
    #[serde(default)]
    pub expires: Option<f64>,

    /// The domain of the cookie
    #[serde(default)]
    pub domain: Option<String>,

    /// Whether the cookie is secure
    #[serde(default)]
    pub secure: Option<bool>,

    /// The same-site policy of the cookie
    #[serde(default)]
    pub same_site: Option<String>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/Cookie>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsCookie {
    /// The name of the cookie
    pub name: String,

    /// The value of the cookie
    pub value: String,

    /// The domain of the cookie
    #[serde(default)]
    pub domain: Option<String>,

    /// The path of the cookie
    #[serde(default)]
    pub path: Option<String>,

    /// The expiration date of the cookie as the number of milliseconds since the Unix epoch
    #[serde(default)]
    pub expires: Option<f64>,

    /// Whether the cookie is secure
    #[serde(default)]
    pub secure: bool,

    /// The same-site policy of the cookie
    #[serde(default)]
    pub same_site: Option<String>,
} 