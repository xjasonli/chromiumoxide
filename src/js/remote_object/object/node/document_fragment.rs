use super::*;

pub mod shadow_root;

pub use shadow_root::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment
    class DocumentFragment extends Node inherits Object {
        static #class: ["DocumentFragment", "ShadowRoot"];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount
            childElementCount: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children
            children: JsHtmlCollection [readonly];

            /// Extension property
            childrenVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.children);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild
            first_element_child: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild
            last_element_child: Option<JsElement> [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append
            append<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append
            #[rename = +text]
            append<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend
            prepend<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend
            #[rename = +text]
            prepend<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector
            querySelector(selectors: &str) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll
            querySelectorAll(selectors: &str) -> Vec<JsElement> {
                const result = this.querySelectorAll(selectors);
                return Array.from(result);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren
            replaceChildren<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren
            #[rename = +text]
            replaceChildren<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById
            getElementById(id: &str) -> Option<JsElement>;

            /// Extension
            queryXpath(xpath: &str) -> Option<JsNode> {
                let document = this.ownerDocument;
                let result = document.evaluate(xpath, this, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                for (let i = 0; i < result.snapshotLength; i++) {
                    let node = result.snapshotItem(i);
                    if (node) {
                        return node;
                    }
                }
                return null;
            }

            /// Extension
            queryXpathAll(xpath: &str) -> Vec<JsNode> {
                let document = this.ownerDocument;
                let result = document.evaluate(xpath, this, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                let nodes = [];
                for (let i = 0; i < result.snapshotLength; i++) {
                    let node = result.snapshotItem(i);
                    if (node) {
                        nodes.push(node);
                    }
                }
                return nodes;
            }
        }
    }
);
