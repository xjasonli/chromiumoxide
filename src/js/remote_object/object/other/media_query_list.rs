use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList>
    class MediaQueryList extends EventTarget inherits Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "MediaQueryList";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/matches>
            matches: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/media>
            media: String [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener>
            addListener(callback: impl IntoJs<JsFunction>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener>
            removeListener(callback: impl IntoJs<JsFunction>) -> ();
        }
    }
} 