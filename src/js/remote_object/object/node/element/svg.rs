use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement
    class SvgElement extends Element inherits Node, Object {
        static #class: "SVG*";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/attributeStyleMap
            attributeStyleMap: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset
            dataset: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/nonce
            nonce: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement
            ownerSvgElement: Option<JsSvgElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style
            style: JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex
            tabIndex: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement
            viewportElement: Option<JsSvgElement> [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur
            blur() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus
            focus() -> ();
            #[rename = +withOptions]
            focus(options: JsFocusOptions) -> ();
        }
    }
);
