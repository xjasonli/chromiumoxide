use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement
    class HtmlTableSectionElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLTableSectionElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows
            rows: JsHtmlCollection [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow
            deleteRow(index: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow
            insertRow(index?: i32) -> JsHtmlTableRowElement;
        }
    }
);
