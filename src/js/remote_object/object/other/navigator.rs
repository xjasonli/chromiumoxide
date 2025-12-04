use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator>
    class Navigator extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Navigator";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/cookieEnabled>
            cookieEnabled: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency>
            hardwareConcurrency: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language>
            language: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages>
            languages: Vec<String> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints>
            maxTouchPoints: i32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine>
            onLine: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform>
            platform: String [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent>
            userAgent: String [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate>
            vibrate(pattern: impl IntoJsAny) -> bool;
        }
    }
} 