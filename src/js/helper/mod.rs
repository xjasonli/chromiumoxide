use std::sync::Arc;

use chromiumoxide_cdp::cdp::js_protocol::runtime::{RemoteObjectId, RemoteObjectType};
use serde::{Serialize, Deserialize};

use crate::handler::PageInner;
use crate::error::{CdpError, Result};

use super::*;

mod remote_val;
mod evaluator;
mod global;
mod patterns;
mod schema;

pub(crate) use remote_val::*;
pub(crate) use evaluator::*;
pub(crate) use global::*;
pub(crate) use patterns::*;

pub(crate) const JS_REMOTE_OBJECT_KEY: &str = "$chromiumoxide::js::remote";
pub(crate) const JS_BIGINT_KEY: &str = "$chromiumoxide::js::bigint";
pub(crate) const JS_UNDEFINED_KEY: &str = "$chromiumoxide::js::undefined";

type JsonObject = serde_json::Map<String, JsonValue>;

pub(crate) type JsonPointer = Vec<JsonPointerSegment>;
pub(crate) type JsonPointerRef<'a> = &'a [JsonPointerSegment];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub(crate) enum JsonPointerSegment {
    Field(String),
    Index(usize),
}

impl std::cmp::PartialOrd for JsonPointerSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (JsonPointerSegment::Index(n1), JsonPointerSegment::Index(n2)) => Some(n1.cmp(n2)),
            (JsonPointerSegment::Field(s1), JsonPointerSegment::Field(s2)) => Some(s1.cmp(s2)),
            (JsonPointerSegment::Index(_), JsonPointerSegment::Field(_)) => Some(std::cmp::Ordering::Less),
            (JsonPointerSegment::Field(_), JsonPointerSegment::Index(_)) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl std::cmp::Ord for JsonPointerSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<String> for JsonPointerSegment {
    fn from(s: String) -> Self {
        JsonPointerSegment::Field(s)
    }
}
impl From<usize> for JsonPointerSegment {
    fn from(n: usize) -> Self {
        JsonPointerSegment::Index(n)
    }
}
impl From<&str> for JsonPointerSegment {
    fn from(s: &str) -> Self {
        JsonPointerSegment::Field(s.to_string())
    }
}

/// A descriptor of a JSON object with remote object paths.
/// 
/// This struct is used to describe a JSON object with remote object paths.
/// 
/// The `value` is the JSON value.
/// 
/// The `paths` is the paths of the remote objects.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ValueDescriptor {
    // The JSON value.
    #[serde(default)]
    pub value: JsonValue,

    // The paths of the remote object values.
    #[serde(default)]
    pub paths: Vec<JsonPointer>,
}

impl ValueDescriptor {
    #[allow(dead_code)]
    pub fn parse(json: JsonValue) -> (Self, Vec<JsRemoteVal>) {
        Self::parse_with_expr(json, &mut vec![], &[])
    }

    pub fn parse_with_expr(
        mut json: JsonValue,
        exprs: &mut Vec<(JsonPointer, String)>,
        expr_prefix: JsonPointerRef<'_>,
    ) -> (Self, Vec<JsRemoteVal>) {
        let (paths, values) = utils::split_from_json(&mut json, exprs, expr_prefix);
        (
            Self {
                value: json,
                paths,
            },
            values,
        )
    }

    pub fn merge(mut self, values: Vec<JsRemoteVal>) -> crate::error::Result<JsonValue> {
        utils::merge_into_json(&mut self.value, self.paths, values)?;
        Ok(self.value)
    }
}

mod utils {
    use super::*;

    /// Split `JsExpr` and `JsRemoteObject` out of the json value
    pub(super) fn split_from_json(
        json: &mut JsonValue,
        exprs: &mut Vec<(JsonPointer, String)>,
        expr_prefix: JsonPointerRef<'_>,
    ) -> (Vec<JsonPointer>, Vec<JsRemoteVal>) {
        fn split_impl(
            json: &mut JsonValue,
            current: JsonPointer,
            paths: &mut Vec<JsonPointer>,
            values: &mut Vec<JsRemoteVal>,
            exprs: &mut Vec<(JsonPointer, String)>,
            expr_prefix: JsonPointerRef<'_>,
        ) {
            match json {
                JsonValue::Object(obj) => {
                    if let Some(expr) = JsExpr::deserialize(&*obj).ok() {
                        let mut path= expr_prefix.to_owned();
                        path.extend(current);

                        exprs.push((path, expr.into_inner().into()));
                    } else if let Some(value) = JsRemoteVal::deserialize(&*obj).ok() {
                        paths.push(current);
                        values.push(value);
                        *obj = JsonObject::new();
                    } else if let Some(_) = JsUndefined::deserialize(&*obj).ok() {
                        // skipped
                    } else if let Some(_) = JsBigInt::deserialize(&*obj).ok() {
                        // skipped
                    } else {
                        for (key, val) in obj.iter_mut() {
                            let mut new_path = current.clone();
                            new_path.push(JsonPointerSegment::Field(key.clone()));
                            split_impl(val, new_path, paths, values, exprs, expr_prefix);
                        }
                    }
                }
                JsonValue::Array(arr) => {
                    for (idx, val) in arr.iter_mut().enumerate() {
                        let mut new_path = current.clone();
                        new_path.push(JsonPointerSegment::Index(idx));
                        split_impl(val, new_path, paths, values, exprs, expr_prefix);
                    }
                }
                _ => (),
            }
        }

        let mut paths = Vec::new();
        let mut values = Vec::new();
        split_impl(json, vec![], &mut paths, &mut values, exprs, expr_prefix);
        (paths, values)
    }

    // Merge ``
    pub(super) fn merge_into_json(
        json: &mut JsonValue,
        paths: Vec<JsonPointer>,
        values: Vec<JsRemoteVal>
    ) -> crate::error::Result<()> {
        fn merge_impl(
            json: &mut JsonValue,
            path: JsonPointerRef<'_>,
            value: JsRemoteVal
        ) -> crate::error::Result<()> {
            if path.is_empty() {
                *json = value.serialize(serde_json::value::Serializer)?;
            } else {
                match &path[0] {
                    JsonPointerSegment::Field(s) => {
                        if !json.is_object() {
                            *json = serde_json::json!({});
                        }
                        let object = json.as_object_mut().unwrap();
                        if !object.contains_key(s) {
                            object.insert(s.to_string(), serde_json::Value::Null);
                        }

                        let prop = object.get_mut(s).unwrap();
                        merge_impl(prop, &path[1..], value)?;
                    }
                    JsonPointerSegment::Index(n) => {
                        if !json.is_array() {
                            *json = serde_json::json!([]);
                        }

                        let array = json.as_array_mut().unwrap();
                        if array.len() <= *n {
                            array.resize(*n + 1, serde_json::Value::Null);
                        }

                        let item = array.get_mut(*n).unwrap();
                        merge_impl(item, &path[1..], value)?;
                    }
                }
            }
            Ok(())
        }

        for (path, value) in paths.into_iter().zip(values.into_iter()) {
            merge_impl(json, &path[..], value)?;
        }
        Ok(())
    }
}

async fn parse_remote_object(page: Arc<PageInner>, remote_object: RemoteObject) -> Result<JsonValue> {
    match remote_object.r#type {
        RemoteObjectType::Object | RemoteObjectType::Symbol | RemoteObjectType::Function => {
            if let Some(_) = &remote_object.object_id {
                let remote_val = JsRemoteVal::from_remote_object(&page, remote_object).await?;
                return Ok(serde_json::to_value(remote_val)?);
            }
        }
        RemoteObjectType::Undefined => {
            return Ok(serde_json::to_value(JsUndefined)?)
        }
        RemoteObjectType::Bigint => {
            let bigint = JsBigInt::from_remote_object(&remote_object)
                .ok_or(CdpError::UnexpectedValue(format!("Invalid bigint: {:#?}", remote_object)))?;
            return Ok(serde_json::to_value(bigint)?);
        }
        _ => (),
    }
    Ok(remote_object.value.unwrap_or_default())
}
