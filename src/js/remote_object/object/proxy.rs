use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy
    class Proxy extends Object {
        static #subtype: "proxy";
    }
);
