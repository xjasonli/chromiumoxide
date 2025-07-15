use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage>
    class SharedStorage extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "SharedStorage";

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/clear>
            clear() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/delete>
            delete(key: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/get>
            get(key: impl IntoJs<String>) -> Option<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/run>
            run(url: impl IntoJs<String>, data: impl IntoJsAny) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/set>
            set(key: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/worklet>
            worklet() -> JsObject;
        }
    }
} 