use std::sync::Arc;

use chromiumoxide_cdp::cdp::{browser_protocol::dom::DescribeNodeParams, js_protocol::runtime::{CallArgument, RemoteObjectSubtype, RemoteObjectType}};

use crate::{error::{CdpError, Result}, handler::PageInner};
use super::*;

pub(crate) const JS_REMOTE_KEY: &str = "$chromiumoxide::js::remote";
pub(crate) const JS_BIGINT_KEY: &str = "$chromiumoxide::js::bigint";
pub(crate) const JS_UNDEFINED_KEY: &str = "$chromiumoxide::js::undefined";

#[derive(Debug, Clone)]
pub(crate) enum SpecialValue {
    Remote(JsRemoteVal),
    BigInt(JsBigInt),
    Undefined(JsUndefined),
}

impl SpecialValue {
    pub async fn from_remote_object(page: &Arc<PageInner>, remote_object: RemoteObject) -> Result<Self> {
        if remote_object.object_id.is_some() {
            return Ok(Self::Remote(JsRemoteVal::from_remote_object(page, remote_object).await?));
        }
        if remote_object.r#type == RemoteObjectType::Bigint {
            let mut value: String = remote_object.unserializable_value.unwrap().into();
            assert!(value.ends_with("n"));
            value.pop();
            return Ok(Self::BigInt(JsBigInt(value)));
        }
        if remote_object.r#type == RemoteObjectType::Undefined {
            return Ok(Self::Undefined(JsUndefined));
        }
        Err(CdpError::UnexpectedValue(format!("Unsupported remote object: {remote_object:?}")))
    }

    pub fn into_call_argument(self) -> CallArgument {
        let argument = match self {
            SpecialValue::Remote(remote) => {
                CallArgument::builder()
                    .object_id(remote.id)
                    .build()
            }
            SpecialValue::BigInt(big_int) => {
                let mut value = big_int.0;
                value.push('n');
                CallArgument::builder()
                    .unserializable_value(value)
                    .build()
            }
            SpecialValue::Undefined(_) => CallArgument::default(),
        };
        argument
    }

    pub fn from_json(json: &JsonObject) -> Option<Self> {
        if let Ok(remote) = JsRemoteVal::deserialize(json) {
            return Some(SpecialValue::Remote(remote));
        }
        if let Ok(big_int) = JsBigInt::deserialize(json) {
            return Some(SpecialValue::BigInt(big_int));
        }
        if let Ok(undefined) = JsUndefined::deserialize(json) {
            return Some(SpecialValue::Undefined(undefined));
        }
        None
    }

    pub fn into_json(&self) -> Result<JsonValue, serde_json::Error> {
        let serializer = serde_json::value::Serializer;
        let value = match self {
            SpecialValue::Remote(data) => data.serialize(serializer)?,
            SpecialValue::BigInt(big_int) => big_int.serialize(serializer)?,
            SpecialValue::Undefined(undefined) => undefined.serialize(serializer)?,
        };
        Ok(value)
    }

    pub fn remote_object_id(&self) -> Option<RemoteObjectId> {
        match self {
            SpecialValue::Remote(remote) => Some(remote.id.clone()),
            _ => None,
        }
    }
}

impl From<JsRemoteVal> for SpecialValue {
    fn from(data: JsRemoteVal) -> Self {
        SpecialValue::Remote(data)
    }
}

impl From<JsRemoteObject> for SpecialValue {
    fn from(remote_object: JsRemoteObject) -> Self {
        let data = remote_object.val();
        SpecialValue::Remote(data.clone())
    }
}

impl From<JsBigInt> for SpecialValue {
    fn from(big_int: JsBigInt) -> Self {
        SpecialValue::BigInt(big_int)
    }
}

//impl From<JsExpr> for SpecialValue {
//    fn from(expr: JsExpr) -> Self {
//        SpecialValue::Expr(expr)
//    }
//}


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct JsRemoteVal {
    pub(crate) id: RemoteObjectId,
    pub(crate) r#type: JsRemoteObjectType,
    pub(crate) class: String,
}

impl JsRemoteVal {
    pub(crate) async fn from_remote_object(page: &Arc<PageInner>, remote_object: RemoteObject) -> Result<Self> {
        let Some(object_id) = remote_object.object_id else {
            return Err(CdpError::UnexpectedValue(format!("Remote object has no object id: {remote_object:?}")));
        };

        let r#type = match remote_object.r#type {
            RemoteObjectType::Object => {
                let subtype = match remote_object.subtype {
                    Some(subtype) => {
                        match subtype {
                            RemoteObjectSubtype::Array => JsObjectSubtype::Array,
                            RemoteObjectSubtype::Node => {
                                let node = {
                                    let params = DescribeNodeParams::builder()
                                        .object_id(object_id.clone())
                                        .build();
                                    let response = page.execute(params).await?;
                                    response.result.node
                                };
                                let node_id = if *node.node_id.inner() == 0 {
                                    None
                                } else {
                                    Some(*node.node_id.inner())
                                };
                                let backend_node_id = *node.backend_node_id.inner();

                                JsObjectSubtype::Node {
                                    node_id,
                                    backend_node_id,
                                }
                            }
                            RemoteObjectSubtype::Regexp => JsObjectSubtype::RegExp,
                            RemoteObjectSubtype::Date => JsObjectSubtype::Date,
                            RemoteObjectSubtype::Map => JsObjectSubtype::Map,
                            RemoteObjectSubtype::Set => JsObjectSubtype::Set,
                            RemoteObjectSubtype::Weakmap => JsObjectSubtype::WeakMap,
                            RemoteObjectSubtype::Weakset => JsObjectSubtype::WeakSet,
                            RemoteObjectSubtype::Iterator => JsObjectSubtype::Iterator,
                            RemoteObjectSubtype::Generator => JsObjectSubtype::Generator,
                            RemoteObjectSubtype::Error => JsObjectSubtype::Error,
                            RemoteObjectSubtype::Proxy => JsObjectSubtype::Proxy,
                            RemoteObjectSubtype::Promise => JsObjectSubtype::Promise,
                            RemoteObjectSubtype::Typedarray => JsObjectSubtype::TypedArray,
                            RemoteObjectSubtype::Arraybuffer => JsObjectSubtype::ArrayBuffer,
                            RemoteObjectSubtype::Dataview => JsObjectSubtype::DataView,
                            RemoteObjectSubtype::Webassemblymemory => JsObjectSubtype::WebAssemblyMemory,
                            RemoteObjectSubtype::Wasmvalue => JsObjectSubtype::WasmValue,
                            RemoteObjectSubtype::Null => {
                                return Err(CdpError::UnexpectedValue(format!("Unsupported remote object subtype: {subtype:?}")));
                            }
                        }
                    }
                    None => JsObjectSubtype::None,
                };
                JsRemoteObjectType::Object(subtype)
            }
            RemoteObjectType::Function => {
                JsRemoteObjectType::Function
            }
            RemoteObjectType::Symbol => {
                JsRemoteObjectType::Symbol
            }
            ty => {
                return Err(CdpError::UnexpectedValue(format!("Unsupported remote object type: {ty:?}")));
            }
        };
        let this = JsRemoteVal {
            id: object_id.clone(),
            r#type: r#type,
            class: remote_object.class_name.unwrap_or_default(),
        };
        Ok(this)
    }
}

impl serde::Serialize for JsRemoteVal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(serde::Serialize)]
        struct Proxy<'a> {
            id: &'a str,
            #[serde(flatten)]
            r#type: &'a JsRemoteObjectType,
            class: &'a str,
        }

        let proxy = Proxy {
            id: &self.id.as_ref(),
            r#type: &self.r#type,
            class: &self.class,
        };

        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("JsRemoteObject", 1)?;
        s.serialize_field(JS_REMOTE_KEY, &proxy)?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for JsRemoteVal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Proxy {
            id: String,
            #[serde(flatten)]
            r#type: JsRemoteObjectType,
            class: String,
        }

        use serde::de::Error as _;

        struct Key;
        impl<'de> serde::Deserialize<'de> for Key {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct KeyVisitor;
                impl<'de> serde::de::Visitor<'de> for KeyVisitor {
                    type Value = Key;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(JS_REMOTE_KEY)
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == JS_REMOTE_KEY {
                            Ok(Key)
                        } else {
                            Err(E::unknown_field(value, &[JS_REMOTE_KEY]))
                        }
                    }   
                }
                deserializer.deserialize_identifier(KeyVisitor)
            }
        }

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Proxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct JsRemoteValue")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                while let Some(_) = map.next_key::<Key>()? {
                    let val = map.next_value()?;
                    return Ok(val);
                }
                Err(A::Error::missing_field(JS_REMOTE_KEY))
            }
        }

        let proxy = deserializer.deserialize_struct(
            "JsRemoteObject",
            &[JS_REMOTE_KEY],
            Visitor
        )?;
        Ok(Self {
            id: RemoteObjectId::new(proxy.id),
            r#type: proxy.r#type,
            class: proxy.class,
        })
    }
}

impl schemars::JsonSchema for JsRemoteVal {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("JsRemoteValueData")
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(std::concat!(
            ::core::module_path!(),
            "::",
            "JsRemoteValueData"
        ))
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let schema = schemars::json_schema!({
            "type": "object",
            "properties": {
                JS_REMOTE_KEY: {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "type": {
                            "type": "string",
                            "enum": ["object", "function", "symbol"],
                        },
                        "subtype": {
                            "type": "string",
                            "enum": [
                                "array",
                                "node",
                                "regexp",
                                "date",
                                "map",
                                "set",
                                "weakmap",
                                "weakset",
                                "iterator",
                                "generator",
                                "error",
                                "proxy",
                                "promise",
                                "typedarray",
                                "arraybuffer",
                                "dataview",
                                "wasmmemory",
                                "wasmvalue",
                            ]
                        },
                        "class": { "type": "string" },
                        "nodeId": { "type": "integer" },
                        "backendNodeId": { "type": "integer" },
                    },
                    "required": ["id", "type"],
                }
            },
            "required": [JS_REMOTE_KEY],
        });
        schema
    }

    fn always_inline_schema() -> bool { true }
}
