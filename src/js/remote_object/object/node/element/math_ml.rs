use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement
    class MathMLElement extends Element inherits Node, Object {
        static #class: "MathML*";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/attributeStyleMap
            attributeStyleMap: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/style
            style: JsObject;
        }
    }
);
