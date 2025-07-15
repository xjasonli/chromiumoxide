use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/BarProp>
    class BarProp extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "BarProp";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible>
            visible: bool [readonly];
        }
    }
} 