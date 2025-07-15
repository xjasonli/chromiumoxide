use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection>
    class Selection extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Selection";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode>
            anchorNode: Option<JsNode> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset>
            anchorOffset: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode>
            focusNode: Option<JsNode> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset>
            focusOffset: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed>
            isCollapsed: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount>
            rangeCount: u32 [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/type>
            #[rename = typ]
            type: String [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange>
            addRange(range: impl IntoJs<JsObject>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse>
            collapse(node: impl IntoJs<JsNode>, offset?: u32) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd>
            collapseToEnd() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart>
            collapseToStart() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode>
            containsNode(node: impl IntoJs<JsNode>, allowPartialContainment?: bool) -> bool;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument>
            deleteFromDocument() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend>
            extend(node: impl IntoJs<JsNode>, offset?: u32) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt>
            getRangeAt(index: u32) -> JsObject;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges>
            removeAllRanges() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange>
            removeRange(range: impl IntoJs<JsObject>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren>
            selectAllChildren(node: impl IntoJs<JsNode>) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent>
            setBaseAndExtent(anchorNode: impl IntoJs<JsNode>, anchorOffset: u32, focusNode: impl IntoJs<JsNode>, focusOffset: u32) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Selection/toString>
            toString() -> String;
        }
    }
} 