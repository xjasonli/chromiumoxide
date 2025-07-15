use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen>
    class Screen extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Screen";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight>
            availHeight: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth>
            availWidth: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth>
            colorDepth: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/height>
            height: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation>
            orientation: JsObject [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth>
            pixelDepth: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Screen/width>
            width: i32 [readonly];
        }
    }
} 
 