use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement
    class HtmlTableElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLTableElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption
            caption: Option<JsHtmlElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead
            tHead: Option<JsHtmlTableSectionElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot
            tFoot: Option<JsHtmlTableSectionElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows
            rows: JsHtmlCollection [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies
            tBodies: JsHtmlCollection [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead
            #[rename = create_thead]
            createTHead() -> JsHtmlTableSectionElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead
            #[rename = delete_thead]
            deleteTHead() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot
            #[rename = create_tfoot]
            createTFoot() -> JsHtmlTableSectionElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot
            #[rename = delete_tfoot]
            deleteTFoot() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody
            #[rename = create_tbody]
            createTBody() -> JsHtmlTableSectionElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption
            createCaption() -> JsHtmlElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption
            deleteCaption() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow
            insertRow(index?: i32) -> JsHtmlTableRowElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow
            deleteRow(index: i32) -> ();
        }
    }
);
