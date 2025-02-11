use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement
    class HtmlTableRowElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLTableRowElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells
            cells: JsHtmlCollection [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex
            rowIndex: i32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex
            sectionRowIndex: i32 [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell
            deleteCell(index: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell
            insertCell(index?: i32) -> JsHtmlTableCellElement;
        }
    }
);
