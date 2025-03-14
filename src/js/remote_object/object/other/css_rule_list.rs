use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList
    class CssRuleList extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "CSSRuleList";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/length
            /// Returns the number of CSS rules in the list
            length: u32 [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/item
            /// Returns a CSS rule from the list
            item(index: impl IntoJs<u32>) -> Option<JsCssRule>;
        }
    }
}
