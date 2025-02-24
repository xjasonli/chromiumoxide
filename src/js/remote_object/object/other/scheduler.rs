use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/Scheduler
    class Scheduler extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Scheduler";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask
            postTask(callback: impl IntoJs<JsFunction>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask
            #[rename = + withOptions]
            postTask(callback: impl IntoJs<JsFunction>, options: JsSchedulerPostTaskOptions) -> JsObject;
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsSchedulerPostTaskOptions {
    /// The priority of the task
    #[serde(default)]
    pub priority: Option<String>,

    /// The signal to abort the task
    #[serde(default)]
    pub signal: Option<JsAbortSignal>,

    /// The delay in milliseconds before the task is executed
    #[serde(default)]
    pub delay: Option<f64>,
} 
 