use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData>
    class FontData extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "FontData";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData/family>
            family: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData/fullName>
            fullName: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData/postscriptName>
            postscriptName: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData/style>
            style: String [readonly];
        }
        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/FontData/blob>
            blob() -> JsBlob;
        }
    }
} 
