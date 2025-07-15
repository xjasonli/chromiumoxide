use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule>
    class CssRule extends Object {
        static #class: [
            "CSSRule",
            "CSSStyleRule",
            "CSSImportRule",
            "CSSMediaRule",
            "CSSFontFaceRule",
            "CSSPageRule",
            "CSSNamespaceRule",
            "CSSKeyframesRule",
            "CSSKeyframeRule",
            "CSSCounterStyleRule",
            "CSSSupportsRule",
            "CSSFontFeatureValuesRule",
            "CSSFontPaletteValuesRule",
            "CSSLayerStatementRule",
            "CSSLayerBlockRule",
            "CSSPropertyRule",
            "CSSNestedDeclarations",
            "CSSStartingStyleRule",
            "CSSConditionRule",
            "CSSContainerRule"
        ];

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText>
            /// Represents the textual representation of the rule
            cssText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentRule>
            /// Returns the containing rule, otherwise null
            parentRule: Option<JsCssRule> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentStyleSheet>
            /// Returns the CSSStyleSheet object for the style sheet that contains this rule
            parentStyleSheet: Option<JsCssStyleSheet> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type>
            /// Returns one of the Type constants to determine which type of rule is represented
            #[rename = typ]
            type: JsCssRuleType [readonly];
        }
    }
}

/// Constants for CSSRule.type
/// 
/// <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, schemars::JsonSchema_repr)]
#[repr(u16)]
pub enum JsCssRuleType {
    /// 1: A style rule - CSSStyleRule, the most common kind of rule
    Style = 1,
    
    /// 3: An @import rule - CSSImportRule
    Import = 3,
    
    /// 4: An @media rule - CSSMediaRule
    Media = 4,
    
    /// 5: An @font-face rule - CSSFontFaceRule
    FontFace = 5,
    
    /// 6: An @page rule - CSSPageRule
    Page = 6,
    
    /// 7: An @keyframes rule - CSSKeyframesRule
    Keyframes = 7,
    
    /// 8: An @keyframe rule - CSSKeyframeRule
    Keyframe = 8,
    
    /// 10: An @namespace rule - CSSNamespaceRule
    Namespace = 10,
    
    /// 11: An @counter-style rule - CSSCounterStyleRule
    CounterStyle = 11,
    
    /// 12: An @supports rule - CSSSupportsRule
    Supports = 12,
    
    /// 14: An @font-feature-values rule - CSSFontFeatureValuesRule
    FontFeatureValues = 14,
    
    // Note: The following values are not mentioned in the current MDN documentation
    // but may be needed for newer CSS features or browser-specific implementations
    
    /// 15: An @font-palette-values rule
    FontPaletteValues = 15,
    
    /// 16: An @layer rule (statement)
    LayerStatement = 16,
    
    /// 17: An @layer rule (block)
    LayerBlock = 17,
    
    /// 18: An @property rule
    Property = 18,
    
    /// 19: An @starting-style rule
    StartingStyle = 19,
    
    /// 20: An @container rule
    Container = 20,
    
    /// 21: An @nested-declarations rule
    NestedDeclarations = 21,
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule>
    class CssGroupingRule extends CssRule inherits Object {
        static #class: [
            "CSSGroupingRule",
            "CSSMediaRule",
            "CSSPageRule",
            "CSSLayerBlockRule",
            "CSSStartingStyleRule",
            "CSSConditionRule",
            "CSSSupportsRule",
            "CSSContainerRule"
        ];

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules>
            /// Returns a live CSSRuleList, listing the CSS rules contained in the grouping rule
            cssRules: JsCssRuleList [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule>
            /// Removes a rule from the grouping rule
            deleteRule(index: impl IntoJs<u32>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule>
            /// Inserts a new rule into the grouping rule
            insertRule(rule: impl IntoJs<String>, index: impl IntoJs<u32>) -> u32;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule>
    class CssStyleRule extends CssRule inherits Object {
        static #class: "CSSStyleRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText>
            /// Returns or sets the textual representation of the selector for this rule
            selectorText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style>
            /// Returns the CSSStyleDeclaration object for the rule
            style: JsCssStyleDeclaration [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/styleMap>
            styleMap: JsObject [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule>
    class CssImportRule extends CssRule inherits Object {
        static #class: "CSSImportRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href>
            href: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/layerName>
            layerName: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media>
            media: JsMediaList;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet>
            styleSheet: JsCssStyleSheet [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/supportsText>
            supportsText: Option<String> [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule>
    class CssMediaRule extends CssGroupingRule inherits CssRule, Object {
        static #class: "CSSMediaRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/media>
            /// Returns a MediaList representing the intended destination medium for style information
            media: JsMediaList [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule>
    class CssFontFaceRule extends CssRule inherits Object {
        static #class: "CSSFontFaceRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style>
            /// Returns a CSSStyleDeclaration object for the rule
            style: JsCssStyleDeclaration [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule>
    class CssPageRule extends CssGroupingRule inherits CssRule, Object {
        static #class: "CSSPageRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/selectorText>
            /// Returns or sets the selector text of the rule
            selectorText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style>
            /// Returns the declaration block of this rule
            style: JsCssPageDescriptors [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule>
    class CssNamespaceRule extends CssRule inherits Object {
        static #class: "CSSNamespaceRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI>
            /// Returns the namespace name
            namespaceURI: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix>
            /// Returns the prefix associated to this namespace
            prefix: String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule>
    class CssKeyframesRule extends CssRule inherits Object {
        static #class: "CSSKeyframesRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name>
            /// Returns or sets the name of the animation
            name: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules>
            /// Returns a CSSRuleList of the keyframe rules
            cssRules: JsCssRuleList [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/length>
            length: u32 [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule>
            /// Inserts a new keyframe rule into the rule
            appendRule(rule: String) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule>
            /// Deletes a keyframe rule from the rule
            deleteRule(key: String) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule>
            /// Returns the keyframe rule corresponding to the specified key
            findRule(key: String) -> Option<JsCssKeyframeRule>;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule>
    class CssKeyframeRule extends CssRule inherits Object {
        static #class: "CSSKeyframeRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText>
            /// Returns or sets the keyframe selector as a string
            keyText: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style>
            /// Returns the declaration block of this rule
            style: JsCssStyleDeclaration [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule>
    class CssCounterStyleRule extends CssRule inherits Object {
        static #class: "CSSCounterStyleRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name>
            name: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system>
            system: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols>
            symbols: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols>
            additiveSymbols: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative>
            negative: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix>
            prefix: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix>
            suffix: String;

            /// <https://developer.org/en-US/docs/Web/API/CSSCounterStyleRule/range>
            range: String;

            /// <https://developer.org/en-US/docs/Web/API/CSSCounterStyleRule/pad>
            pad: String;

            /// <https://developer.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs>
            speakAs: String;

            /// <https://developer.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback>
            fallback: String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule>
    class CssConditionRule extends CssGroupingRule inherits CssRule, Object {
        static #class: ["CSSConditionRule", "CSSSupportsRule", "CSSContainerRule"];

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText>
            /// Returns the condition of the rule
            conditionText: String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule>
    class CssSupportsRule extends CssConditionRule inherits CssGroupingRule, CssRule, Object {
        static #class: "CSSSupportsRule";
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule>
    class CssFontFeatureValuesRule extends CssRule inherits Object {
        static #class: "CSSFontFeatureValuesRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily>
            /// Returns or sets the font family name for the rule
            fontFamily: String;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule>
    class CssFontPaletteValuesRule extends CssRule inherits Object {
        static #class: "CSSFontPaletteValuesRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/name>
            name: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/fontFamily>
            fontFamily: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/overrideColors>
            overrideColors: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/basePalette>
            basePalette: String [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerStatementRule>
    class CssLayerStatementRule extends CssRule inherits Object {
        static #class: "CSSLayerStatementRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerStatementRule/nameList>
            /// Returns the names of the declared layers
            nameList: Vec<String> [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerBlockRule>
    class CssLayerBlockRule extends CssGroupingRule inherits CssRule, Object {
        static #class: "CSSLayerBlockRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerBlockRule/name>
            /// Returns the name of the layer
            name: String [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule>
    class CssPropertyRule extends CssRule inherits Object {
        static #class: "CSSPropertyRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/name>
            /// Returns the name of the custom property
            name: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/syntax>
            /// Returns the syntax of the custom property
            syntax: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/inherits>
            /// Returns whether the custom property is inheritable
            inherits: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/initialValue>
            /// Returns the initial value of the custom property
            initialValue: String [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSNestedDeclarations>
    class CssNestedDeclarations extends CssRule inherits Object {
        static #class: "CSSNestedDeclarations";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSNestedDeclarations/style>
            style: JsObject [readonly];
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSStartingStyleRule>
    class CssStartingStyleRule extends CssGroupingRule inherits CssRule, Object {
        static #class: "CSSStartingStyleRule";
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule>
    class CssContainerRule extends CssConditionRule inherits CssGroupingRule, CssRule, Object {
        static #class: "CSSContainerRule";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule/containerName>
            containerName: String [readonly];
            
            /// <https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule/containerQuery>
            containerQuery: String [readonly];
        }
    }
}
