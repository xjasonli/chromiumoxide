use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/History
    class History extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "History";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/History/length
            length: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration
            scrollRestoration: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/state
            state: JsonValue [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/History/back
            back() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/forward
            forward() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/go
            go(delta: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/pushState
            pushState(state: impl IntoJsAny, title: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/pushState
            #[rename = + withUrl]
            pushState(state: impl IntoJsAny, title: impl IntoJs<String>, url: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState
            replaceState(state: impl IntoJsAny, title: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState
            #[rename = + withUrl]
            replaceState(state: impl IntoJsAny, title: impl IntoJs<String>, url: impl IntoJs<String>) -> ();
        }
    }
}