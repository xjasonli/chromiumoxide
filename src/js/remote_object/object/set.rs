use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set
    class Set extends Object {
        static #type: "object";
        static #subtype: "set";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/size
            size: u64 [readonly];
        }

        methods: {
            //TODO
        }
    }
);

