use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Comment
    class Comment extends CharacterData inherits Node, Object {
        static #class: "Comment";
    }
);
