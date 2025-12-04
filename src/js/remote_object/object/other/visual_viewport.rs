use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport>
    class VisualViewport extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "VisualViewport";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/height>
            height: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetLeft>
            offsetLeft: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetTop>
            offsetTop: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageLeft>
            pageLeft: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageTop>
            pageTop: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/scale>
            scale: f64 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/width>
            width: f64 [readonly];
        }
    }
} 