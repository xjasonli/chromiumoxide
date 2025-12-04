use super::*;

pub mod shadow_root;

pub use shadow_root::*;

js_remote_object!(
    /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment>
    class DocumentFragment extends Node inherits Object {
        static #class: ["DocumentFragment", "ShadowRoot"];

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount>
            childElementCount: usize [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children>
            children: JsHtmlCollection [readonly];

            /// Extension property
            childrenVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.children);
                }
            }

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild>
            firstElementChild: Option<JsElement> [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild>
            lastElementChild: Option<JsElement> [readonly];
        }
        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append>
            append<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append>
            #[rename = +text]
            append<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend>
            prepend<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend>
            #[rename = +text]
            prepend<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector>
            querySelector(selectors: impl IntoJs<String>) -> Option<JsElement>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll>
            querySelectorAll(selectors: impl IntoJs<String>) -> Vec<JsElement> {
                const result = this.querySelectorAll(selectors);
                return Array.from(result);
            }

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren>
            replaceChildren<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren>
            #[rename = +text]
            replaceChildren<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById>
            getElementById(id: impl IntoJs<String>) -> Option<JsElement>;

            /// Extension
            queryXpath(xpath: impl IntoJs<String>) -> Option<JsNode> {
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
            queryXpathAll(xpath: impl IntoJs<String>) -> Vec<JsNode> {
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
