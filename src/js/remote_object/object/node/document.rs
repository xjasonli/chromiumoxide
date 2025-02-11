use super::*;

pub mod html;

pub use html::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Document
    class Document extends Node inherits Object {
        static #class: ["HTMLDocument", "XMLDocument"];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/activeElement
            activeElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/body
            body: Option<JsHtmlElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet
            characterSet: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount
            childElementCount: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/children
            children: JsHtmlCollection [readonly];

            /// Extension property
            childrenVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.children);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode
            compatMode: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType
            contentType: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript
            currentScript: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype
            doctype: Option<JsDocumentType> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement
            documentElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI
            #[rename = document_uri]
            documentURI: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds
            embeds: JsHtmlCollection [readonly];

            /// Extension property
            embedsVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.embeds);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild
            firstElementChild: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts
            fonts: JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/forms
            forms: JsHtmlCollection [readonly];

            /// Extension property
            formsVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.forms);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/fragmentDirective
            fragmentDirective: Option<JsObject> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenElement
            fullscreenElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/head
            head: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden
            hidden: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/images
            images: JsHtmlCollection [readonly];

            /// Extension property
            imagesVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.images);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation
            implementation: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild
            lastElementChild: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/links
            links: JsHtmlCollection [readonly];

            /// Extension property
            linksVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.links);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureElement
            pictureInPictureElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureEnabled
            pictureInPictureEnabled: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins
            plugins: JsHtmlCollection [readonly];

            /// Extension property
            pluginsVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.plugins);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement
            pointerLockElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/prerendering
            prerendering: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts
            scripts: JsHtmlCollection [readonly];

            /// Extension property
            scriptsVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.scripts);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement
            scrollingElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/stylesheets
            styleSheets: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline
            timeline: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState
            visibilityState: String [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode
            adoptNode<T: Class<JsNode>>(node: T) -> T::Owned;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/append
            append<'a, I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/append
            #[rename = +text]
            append<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint
            caretPositionFromPoint(x: f64, y: f64) -> Option<JsObject>;
            #[rename = +withShadowRoots]
            caretPositionFromPoint(x: f64, y: f64, shadow_roots: &[JsShadowRoot]) -> Option<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute
            createAttribute(name: &str) -> JsAttr;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS
            #[rename = create_attribute_ns]
            createAttributeNS(namespace_uri: &str, qualified_name: &str) -> JsAttr;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection
            #[rename = create_cdata_section]
            createCDATASection(data: &str) -> JsCdataSection;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment
            createComment(data: &str) -> JsComment;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment
            createDocumentFragment() -> JsDocumentFragment;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement
            createElement(tag_name: &str) -> JsElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS
            #[rename = create_element_ns]
            createElementNS(namespace_uri: &str, qualified_name: &str) -> JsElement;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator
            createNodeIterator<T: Class<JsNode>>(root: T, what_to_show: Option<JsNodeFilter>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction
            createProcessingInstruction(target: &str, data: &str) -> JsProcessingInstruction;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange
            createRange() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode
            createTextNode(data: &str) -> JsText;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker
            createTreeWalker<T: Class<JsNode>>(root: T, what_to_show: Option<JsNodeFilter>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint
            elementFromPoint(x: u32, y: u32) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint
            elementsFromPoint(x: u32, y: u32) -> Vec<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen
            exitFullscreen() -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPictureInPicture
            exitPictureInPicture() -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock
            exitPointerLock() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations
            getAnimations() -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById
            getElementById(id: &str) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName
            getElementsByClassName(name: &str) -> JsHtmlCollection;

            /// Extension method
            getElementsByClassNameVec(name: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByClassName(name));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName
            getElementsByTagName(name: &str) -> JsHtmlCollection;

            /// Extension method
            getElementsByTagNameVec(name: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByTagName(name));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS
            #[rename = get_elements_by_tag_name_ns]
            getElementsByTagNameNS(namespace: &str, name: &str) -> JsHtmlCollection;

            /// Extension method
            #[rename = get_elements_by_tag_name_ns_vec]
            getElementsByTagNameNSVec(namespace: &str, name: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByTagNameNS(namespace, name));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection
            getSelection() -> Option<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/hasStorageAccess
            hasStorageAccess() -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/hasUnpartitionedCookieAccess
            hasUnpartitionedCookieAccess() -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode
            importNode<T: Class<JsNode>>(node: T, deep: bool) -> T::Owned;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend
            prepend<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend
            #[rename = +text]
            prepend<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector
            querySelector(selectors: &str) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll
            querySelectorAll(selectors: &str) -> Vec<JsElement> {
                const result = this.querySelectorAll(selectors);
                return Array.from(result);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren
            replaceChildren<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren
            #[rename = +text]
            replaceChildren<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: Class<str>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccess
            requestStorageAccess(options?: &JsRequestStorageAccessOptions) -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccessFor
            requestStorageAccessFor(origin: &str) -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition
            startViewTransition(callback: Option<&JsFunction>) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression
            createExpression(xpath: &str) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate
            evaluate<T: Class<JsNode>>(
                xpath: &str,
                context_node: T,
                namespace_resolver: Option<JsFunction>,
                result_type: JsXpathResultType,
                result: Option<JsObject>,
            ) -> JsObject;

            /// Extension method
            queryXpath(xpath: &str) -> Option<JsNode> {
                let result = this.evaluate(xpath, null, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                for (let i = 0; i < result.snapshotLength; i++) {
                    let node = result.snapshotItem(i);
                    if (node) {
                        return node;
                    }
                }
                return null;
            }

            /// Extension method
            queryXpathAll(xpath: &str) -> Vec<JsNode> {
                let result = this.evaluate(xpath, null, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
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

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(transparent)]
    pub struct JsNodeFilter: u32 {
        const SHOW_ALL = 0xFFFFFFFF;
        const SHOW_ATTRIBUTE = 2;
        const SHOW_CDATA_SECTION = 8;
        const SHOW_COMMENT = 128;
        const SHOW_DOCUMENT = 256;
        const SHOW_DOCUMENT_FRAGMENT = 1024;
        const SHOW_DOCUMENT_TYPE = 512;
        const SHOW_ELEMENT = 1;
        const SHOW_ENTITY = 32;
        const SHOW_ENTITY_REFERENCE = 16;
        const SHOW_NOTATION = 2048;
        const SHOW_PROCESSING_INSTRUCTION = 64;
        const SHOW_TEXT = 4;
    }
}

impl schemars::JsonSchema for JsNodeFilter {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "JsNodeFilter".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::JsNodeFilter").into()
    }
    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        gen.subschema_for::<u32>()
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccess#types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsRequestStorageAccessOptions {
    /// A boolean specifying all possible unpartitioned states should be made accessible.
    pub all: Option<bool>,

    /// A boolean specifying third-party cookies should be made accessible.
    pub cookies: Option<bool>,

    /// A boolean specifying StorageAccessHandle.sessionStorage should be made accessible.
    #[serde(rename = "sessionStorage")]
    pub session_storage: Option<bool>,

    /// A boolean specifying StorageAccessHandle.localStorage should be made accessible.
    #[serde(rename = "localStorage")]
    pub local_storage: Option<bool>,

    /// A boolean specifying StorageAccessHandle.indexedDB should be made accessible.
    #[serde(rename = "indexedDB")]
    pub indexed_db: Option<bool>,

    /// A boolean specifying StorageAccessHandle.locks should be made accessible.
    pub locks: Option<bool>,

    /// A boolean specifying StorageAccessHandle.caches should be made accessible.
    pub caches: Option<bool>,

    /// A boolean specifying StorageAccessHandle.getDirectory() should be made accessible.
    #[serde(rename = "getDirectory")]
    pub get_directory: Option<bool>,

    /// A boolean specifying StorageAccessHandle.estimate() should be made accessible.
    pub estimate: Option<bool>,

    /// A boolean specifying StorageAccessHandle.createObjectURL() should be made accessible.
    #[serde(rename = "createObjectURL")]
    pub create_object_url: Option<bool>,

    /// A boolean specifying StorageAccessHandle.revokeObjectURL() should be made accessible.
    #[serde(rename = "revokeObjectURL")]
    pub revoke_object_url: Option<bool>,

    /// A boolean specifying StorageAccessHandle.BroadcastChannel() should be made accessible.
    #[serde(rename = "BroadcastChannel")]
    pub broadcast_channel: Option<bool>,

    /// A boolean specifying StorageAccessHandle.SharedWorker() should be made accessible.
    #[serde(rename = "SharedWorker")]
    pub shared_worker: Option<bool>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/XPathResult#constants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, schemars::JsonSchema_repr)]
#[repr(u8)]
pub enum JsXpathResultType {
    /// A result set containing whatever type naturally results from evaluation
    /// of the expression. Note that if the result is a node-set then
    /// UNORDERED_NODE_ITERATOR_TYPE is always the resulting type.
    Any = 0,

    /// A result containing a single number. This is useful for example, in an
    /// XPath expression using the count() function.
    Number = 1,

    /// A result containing a single string.
    String = 2,

    /// A result containing a single boolean value. This is useful for example, in
    /// an XPath expression using the not() function.
    Boolean = 3,

    /// A result node-set containing all the nodes matching the expression. The nodes
    /// may not necessarily be in the same order that they appear in the document.
    UnorderedNodeIterator = 4,

    /// A result node-set containing all the nodes matching the expression. The nodes
    /// are in the same order that they appear in the document.
    OrderedNodeIterator = 5,

    /// A result node-set containing snapshots of all the nodes matching the expression.
    /// The nodes may not necessarily be in the same order that they appear in the document.
    UnorderedNodeSnapshot = 6,

    /// A result node-set containing snapshots of all the nodes matching the expression.
    /// The nodes are in the same order that they appear in the document.
    OrderedNodeSnapshot = 7,

    /// A result node-set containing any single node that matches the expression. The node
    /// is not necessarily the first node in the document that matches the expression.
    AnyUnorderedNode = 8,

    /// A result node-set containing the first node in the document that matches the expression.
    FirstOrderedNode = 9,
}
