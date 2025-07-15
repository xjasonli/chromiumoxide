use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration>
    class CssStyleDeclaration extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: [
            "CSSStyleDeclaration",
            "CSSPageDescriptors",
        ];

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssFloat>
            cssFloat: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText>
            cssText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length>
            length: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule>
            parentRule: Option<JsObject> [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority>
            getPropertyPriority(property: impl IntoJs<String>) -> String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue>
            getPropertyValue(property: impl IntoJs<String>) -> String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item>
            item(index: impl IntoJs<u32>) -> String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty>
            removeProperty(property: impl IntoJs<String>) -> String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty>
            setProperty(property: impl IntoJs<String>, value: impl IntoJs<String>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty>
            #[rename = + withPriority]
            setProperty(property: impl IntoJs<String>, value: impl IntoJs<String>, priority: impl IntoJs<String>) -> ();
        }
    }
} 

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors>
    class CssPageDescriptors extends CssStyleDeclaration inherits Object {
        static #class: "CSSPageDescriptors";
        
        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#margin>
            /// A string representing the margin property of the corresponding @page at-rule.
            margin: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#margin-top>
            /// A string representing the margin-top property of the corresponding @page at-rule.
            marginTop: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#margin-right>
            /// A string representing the margin-right property of the corresponding @page at-rule.
            marginRight: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#margin-bottom>
            /// A string representing the margin-bottom property of the corresponding @page at-rule.
            marginBottom: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#margin-left>
            /// A string representing the margin-left property of the corresponding @page at-rule.
            marginLeft: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#page-orientation>
            /// A string representing the page-orientation property of the corresponding @page at-rule.
            pageOrientation: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors#size>
            /// A string representing the size property of the corresponding @page at-rule.
            size: String;
        }
    }
}
