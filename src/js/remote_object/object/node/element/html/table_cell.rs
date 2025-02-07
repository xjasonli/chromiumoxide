use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement
    class HtmlTableCellElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLTableCellElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/abbr
            abbr: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex
            cellIndex: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan
            colSpan: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers
            headers: JsDomTokenList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan
            rowSpan: u32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/scope
            scope: String;
        }
    }
);
