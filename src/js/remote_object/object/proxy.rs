use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy
    class Proxy extends Object {
        static #type: "object";
        static #subtype: "proxy";
    }
);
