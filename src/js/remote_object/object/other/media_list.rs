use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList>
    class MediaList extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "MediaList";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText>
            mediaText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length>
            length: u32 [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium>
            appendMedium(medium: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium>
            deleteMedium(medium: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item>
            item(index: impl IntoJs<u32>) -> Option<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/MediaList/toString>
            toString() -> String;
        }
    }
}
