use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement
    class HtmlImageElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLImageElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt
            alt: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/attributionSrc
            attributionSrc: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete
            complete: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin
            crossOrigin: Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/currentSrc
            currentSrc: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding
            decoding: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/fetchPriority
            fetchPriority: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height
            height: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap
            isMap: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading
            loading: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight
            naturalHeight: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth
            naturalWidth: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy
            referrerPolicy: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes
            sizes: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src
            src: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset
            srcset: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap
            useMap: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width
            width: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/x
            x: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/y
            y: u32 [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decode
            decode() -> ();
        }
    }
);
