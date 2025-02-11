use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/arguments#properties
    class Arguments extends ArrayLike inherits Object {
        static #class: "Arguments";
    }
);

