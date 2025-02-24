use super::*;

js_remote_object!{
    /// https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails
    class ScreenDetails extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "ScreenDetails";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/currentScreen
            currentScreen: JsScreen [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/screens
            screens: Vec<JsScreen> [readonly];
        }
    }
}
 