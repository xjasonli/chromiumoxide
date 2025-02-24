use super::*;
use serde_json::Value as JsonValue;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/Performance
    class Performance extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Performance";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/eventCounts
            eventCounts: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/memory
            memory: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/navigation
            navigation: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
            timeOrigin: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/timing
            timing: JsObject [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks
            clearMarks(markName?: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures
            clearMeasures(measureName?: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearResourceTimings
            clearResourceTimings() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntries
            getEntries() -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName
            getEntriesByName(name: impl IntoJs<String>, type_?: impl IntoJs<String>) -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByType
            getEntriesByType(type_: impl IntoJs<String>) -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark
            mark(markName: impl IntoJs<String>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure
            measure(measureName: impl IntoJs<String>, startMark?: impl IntoJs<String>, endMark?: impl IntoJs<String>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
            now() -> f64;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/setResourceTimingBufferSize
            setResourceTimingBufferSize(maxSize: u32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/toJSON
            #[rename = toJson]
            toJSON() -> JsonValue;
        }
    }
} 