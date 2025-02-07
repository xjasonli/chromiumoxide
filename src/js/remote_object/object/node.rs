use chromiumoxide_cdp::cdp::browser_protocol::dom::{BackendNodeId, NodeId};

use super::*;
use crate::error::{CdpError, Result};

mod element;
mod document;
mod document_type;
mod document_fragment;
mod character_data;
mod attr;

pub use element::*;
pub use document::*;
pub use document_type::*;
pub use document_fragment::*;
pub use character_data::*;
pub use attr::*;

// https://developer.mozilla.org/en-US/docs/Web/API/Node
//
// Node (nodeType = 1-12)
// ├── Attr (nodeType = 2)
// ├── Document (nodeType = 9)
// │   ├── HTMLDocument
// │   └── XMLDocument
// ├── DocumentFragment (nodeType = 11)
// ├── DocumentType (nodeType = 10)
// ├── Element (nodeType = 1)
// │   ├── HTMLElement
// │   │   ├── HTMLAnchorElement (<a>)
// │   │   ├── HTMLButtonElement (<button>)
// │   │   ├── HTMLDivElement (<div>)
// │   │   ├── HTMLFormElement (<form>)
// │   │   ├── HTMLInputElement (<input>)
// │   │   ├── HTMLImageElement (<img>)
// │   │   ├── HTMLParagraphElement (<p>)
// │   │   ├── HTMLSpanElement (<span>)
// │   │   ├── HTMLTableElement (<table>)
// │   │   ├── HTMLSelectElement (<select>)
// │   │   ├── HTMLTextAreaElement (<textarea>)
// │   │   ├── HTMLHeadingElement (<h1>-<h6>)
// │   │   ├── HTMLListElement (<ul>, <ol>)
// │   │   ├── HTMLMediaElement
// │   │   │   ├── HTMLVideoElement
// │   │   │   └── HTMLAudioElement
// │   │   └── ... (other HTML elements)
// │   ├── SVGElement
// │   │   ├── SVGCircleElement
// │   │   ├── SVGRectElement
// │   │   ├── SVGPathElement
// │   │   ├── SVGLineElement
// │   │   ├── SVGTextElement
// │   │   ├── SVGPolygonElement
// │   │   ├── SVGEllipseElement
// │   │   └── ... (other SVG elements)
// │   └── MathMLElement
// └── CharacterData (nodeType = 3)
//     ├── Text (nodeType = 3)
//     ├── Comment (nodeType = 8)
//     ├── CDATASection (nodeType = 4)
//     └── ProcessingInstruction (nodeType = 7)

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Node
    class Node extends Object {
        static #subtype: "node";


        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI
            #[rename = base_uri]
            baseURI: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes
            childNodes: JsNodeList [readonly];

            /// Extension property
            childNodesVec: Vec<JsNode> [readonly] {
                get() {
                    return Array.from(this.childNodes);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild
            firstChild: JsNode [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected
            isConnected: bool [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild
            lastChild: JsNode [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling
            nextSibling: Option<JsNode> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName
            nodeName: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
            nodeType: JsNodeType [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue
            nodeValue: Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument
            ownerDocument: Option<JsDocument> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode
            parentNode: Option<JsNode> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement
            parentElement: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling
            previousSibling: Option<JsNode> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent
            textContent: Option<String>;
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild
            appendChild<T: Class<JsNode>>(child: T) -> T::Owned;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode
            cloneNode(deep: bool) -> JsNode;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition
            compareDocumentPosition<T: Class<JsNode>>(other: T) -> JsDocumentPosition;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/contains
            contains<T: Class<JsNode>>(other: T) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode
            getRootNode(composed: bool) -> JsNode;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes
            hasChildNodes() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore
            insertBefore<T1, T2>(new_node: T1, reference_node: T2) -> T1
            where
                T1: Class<JsNode>,
                T2: Class<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace
            isDefaultNamespace(namespace_uri: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode
            isEqualNode<T: Class<JsNode>>(other: T) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode
            isSameNode<T: Class<JsNode>>(other: T) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI
            #[rename = lookup_namespace_uri]
            lookupNamespaceURI(prefix: &str) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix
            lookupPrefix(namespace_uri: &str) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize
            normalize() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild
            removeChild<T: Class<JsNode>>(child: T) -> T;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild
            replaceChild<T1, T2>(new_child: T1, old_child: T2) -> T1
            where
                T1: Class<JsNode>,
                T2: Class<JsNode>;
        }
    }
);

impl JsNode {
    pub fn node_id(&self) -> NodeId {
        match self.object_subtype() {
            Some(JsObjectSubtype::Node {
                node_id,
                ..
            }) => NodeId::new(node_id),
            _ => panic!("JsNode is not a node"),
        }
    }
    pub fn backend_node_id(&self) -> BackendNodeId {
        match self.object_subtype() {
            Some(JsObjectSubtype::Node {
                backend_node_id,
                ..
            }) => BackendNodeId::new(backend_node_id),
            _ => panic!("JsNode is not a node"),
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(transparent)]
    pub struct JsDocumentPosition: u32 {
        const DISCONNECTED = 1;
        const PRECEDING = 2;
        const FOLLOWING = 4;
        const CONTAINS = 8;
        const CONTAINED_BY = 16;
        const IMPLEMENTATION_SPECIFIC = 32;
    }
}

impl schemars::JsonSchema for JsDocumentPosition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "JsDocumentPosition".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::JsDocumentPosition").into()
    }
    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        gen.subschema_for::<u32>()
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, schemars::JsonSchema_repr)]
#[repr(u8)]
pub enum JsNodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CDataSection = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
}

impl JsNodeType {
    /// Convert from JavaScript node type number
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Element),
            2 => Ok(Self::Attribute),
            3 => Ok(Self::Text),
            4 => Ok(Self::CDataSection),
            5 => Ok(Self::EntityReference),
            6 => Ok(Self::Entity),
            7 => Ok(Self::ProcessingInstruction),
            8 => Ok(Self::Comment),
            9 => Ok(Self::Document),
            10 => Ok(Self::DocumentType),
            11 => Ok(Self::DocumentFragment),
            12 => Ok(Self::Notation),
            _ => Err(CdpError::UnexpectedValue(format!("Invalid node type: {}", value))),
        }
    }

    /// Convert to JavaScript node type number
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for JsNodeType {
    type Error = CdpError;
    /// Attempt to convert from u8 to JsNodeType
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
    }
}

impl From<JsNodeType> for u8 {
    /// Convert JsNodeType to u8
    fn from(node_type: JsNodeType) -> Self {
        node_type.to_u8()
    }
}
