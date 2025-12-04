use super::*;
use serde_json::Value as JsonValue;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation>
    class Navigation extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Navigation";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/currentEntry>
            currentEntry: JsObject [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/transition>
            transition: Option<JsObject> [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back>
            back() -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/entries>
            entries() -> Vec<JsObject>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward>
            forward() -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate>
            navigate(url: impl IntoJs<String>) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate>
            #[rename = + withOptions]
            navigate(url: impl IntoJs<String>, options: JsNavigateOptions) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload>
            reload() -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo>
            traverseTo(key: impl IntoJs<String>) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/updateCurrentEntry>
            updateCurrentEntry(options: JsNavigationUpdateCurrentEntryOptions) -> ();
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate#options>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsNavigateOptions {
    /// Info to be added to the navigation entry.
    #[serde(default)]
    pub state: Option<JsonValue>,

    /// The history handling behavior ("push" or "replace").
    #[serde(default)]
    pub history: Option<String>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/Navigation/updateCurrentEntry#parameters>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsNavigationUpdateCurrentEntryOptions {
    /// Info to be added to the navigation entry.
    pub state: JsonValue,
} 