use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
    class CssStyleDeclaration extends Object {
        static #type: "object";
        static #subtype: "none";
        static #class: "CSSStyleDeclaration";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssFloat
            cssFloat: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText
            cssText: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length
            length: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule
            parentRule: Option<JsObject> [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority
            getPropertyPriority(property: impl IntoJs<String>) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue
            getPropertyValue(property: impl IntoJs<String>) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item
            item(index: u32) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty
            removeProperty(property: impl IntoJs<String>) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty
            setProperty(property: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty
            #[rename = + withPriority]
            setProperty(property: impl IntoJs<String>, value: impl IntoJs<String>, priority: impl IntoJs<String>) -> ();
        }
    }
} 
