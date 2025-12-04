use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory>
    class TrustedTypePolicyFactory extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "TrustedTypePolicyFactory";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/defaultPolicy>
            defaultPolicy: Option<JsObject> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/emptyHTML>
            emptyHTML: JsObject [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/emptyScript>
            emptyScript: JsObject [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/createPolicy>
            createPolicy(policyName: impl IntoJs<String>, policyOptions: impl IntoJsAny) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getAttributeType>
            getAttributeType(tagName: impl IntoJs<String>, attribute: impl IntoJs<String>, elementNs?: impl IntoJs<String>, attrNs?: impl IntoJs<String>) -> Option<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getPropertyType>
            getPropertyType(tagName: impl IntoJs<String>, property: impl IntoJs<String>, elementNs?: impl IntoJs<String>) -> Option<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isHTML>
            isHTML(value: impl IntoJsAny) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isScript>
            isScript(value: impl IntoJsAny) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isScriptURL>
            isScriptURL(value: impl IntoJsAny) -> bool;
        }
    }
} 