use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap>
    class ImageBitmap extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "ImageBitmap";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height>
            height: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width>
            width: u32 [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close>
            close() -> ();
        }
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapSource>
/// 
/// ImageBitmapSource is a union type, which means it can be any of the following types:
/// - HTMLImageElement
/// - SVGImageElement
/// - HTMLVideoElement
/// - HTMLCanvasElement
/// - ImageBitmap
/// - OffscreenCanvas
/// - VideoFrame
/// 
/// In our case, we'll use JsObject to represent any of these types.
pub type JsImageBitmapSource = JsObject; 
 