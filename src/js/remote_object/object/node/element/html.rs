use super::*;

mod anchor;
mod button;
mod data_list;
mod form;
mod input;
mod image;
mod iframe;
mod label;
mod table;
mod table_cell;
mod table_row;
mod table_section;

pub use anchor::*;
pub use button::*;
pub use data_list::*; 
pub use form::*;
pub use input::*;
pub use image::*;
pub use iframe::*;
pub use label::*;
pub use table::*;
pub use table_cell::*;
pub use table_row::*;
pub use table_section::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement
    class HtmlElement extends Element inherits Node, Object {
        static #class: "HTML*";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey
            accessKey: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel
            accessKeyLabel: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/attributeStyleMap
            attributeStyleMap: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocapitalize
            autocapitalize: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autofocus
            autofocus: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocorrect
            autocorrect: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable
            contentEditable: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset
            dataset: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir
            dir: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable
            draggable: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/editContext
            editContext: Option<JsObject> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/enterKeyHint
            enterKeyHint: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden
            hidden: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/inert
            inert: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText
            innerText: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/inputMode
            inputMode: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/isContentEditable
            isContentEditable: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang
            lang: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/nonce
            nonce: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetHeight
            offsetHeight: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft
            offsetLeft: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetParent
            offsetParent: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetTop
            offsetTop: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetWidth
            offsetWidth: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/outerText
            outerText: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/popover
            popover: String;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck
            spellcheck: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style
            style: JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex
            tabIndex: i32;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title
            title: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/translate
            translate: bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/virtualKeyboardPolicy
            virtualKeyboardPolicy: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/writingSuggestions
            writingSuggestions: String;
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/attachInternals
            attachInternals() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur
            blur() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click
            click() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus
            focus(options?: JsFocusOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidePopover
            hidePopover() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/showPopover
            showPopover() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/togglePopover
            togglePopover(force: bool) -> bool;
        }
    }
);

/// https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus#options
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsFocusOptions {
    /// Prevent the browser from scrolling the document to bring the
    /// newly-focused element into view. A value of false for preventScroll
    /// (the default) means that the browser will scroll the element into
    /// view after focusing it. If preventScroll is set to true, no scrolling
    /// will occur.
    #[serde(rename = "preventScroll")]
    pub prevent_scroll: Option<bool>,

    /// A boolean value that should be set to true to force, or false to
    /// prevent visible indication that the element is focused. If the
    /// property is not specified, a browser will provide visible indication
    /// if it determines that this would improve accessibility for users.
    #[serde(rename = "focusVisible")]
    pub focus_visible: Option<bool>,
}
