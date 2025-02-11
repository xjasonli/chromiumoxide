use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol
    class Symbol {
        static #type: "symbol";

        properties: {
            constructor: JsFunction [readonly];
            description: String [readonly];
        }

        methods: {
            toString() -> String;
        }
    }
);
