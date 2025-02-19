use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error
    class Error extends Object {
        static #type: "object";
        static #subtype: "error";
        
        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name
            name: String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message
            message: String;
        }
    }
);

