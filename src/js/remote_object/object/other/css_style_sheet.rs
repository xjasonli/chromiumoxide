use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet
    class StyleSheet extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: [
            "StyleSheet",
            "CSSStyleSheet"
        ];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled
            /// Gets or sets whether the stylesheet is applied to the document
            disabled: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href
            /// Returns the location of the stylesheet
            href: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media
            /// Returns a MediaList representing the intended destination medium for style information
            media: JsMediaList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode
            /// Returns the node that associates this style sheet with the document
            ownerNode: Option<JsNode> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet
            /// Returns the style sheet that imported this style sheet
            parentStyleSheet: Option<JsStyleSheet> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title
            /// Returns the advisory title of the style sheet
            title: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type
            /// Returns the type of the style sheet
            #[rename = typ]
            type: String [readonly];
        }
    }
}

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet
    class CssStyleSheet extends StyleSheet inherits Object {
        static #class: "CSSStyleSheet";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules
            /// Returns a live CSSRuleList of the CSS rules in the style sheet
            cssRules: JsCssRuleList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule
            /// Returns the CSSImportRule that imported this style sheet
            ownerRule: Option<JsCssImportRule> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/rules
            /// Returns a live CSSRuleList of the CSS rules in the style sheet
            rules: JsCssRuleList [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule
            /// Removes a rule from the style sheet
            deleteRule(index: impl IntoJs<u32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule
            /// Inserts a new rule into the style sheet
            insertRule(rule: impl IntoJs<String>, index: impl IntoJs<u32>) -> u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/removeRule
            /// Removes a rule from the style sheet
            removeRule(index: impl IntoJs<u32>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replace
            /// Replaces the content of the style sheet with new CSS rules
            replace(text: impl IntoJs<String>) -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replaceSync
            /// Synchronously replaces the content of the style sheet with new CSS rules
            replaceSync(text: impl IntoJs<String>) -> ();
        }
    }
}
