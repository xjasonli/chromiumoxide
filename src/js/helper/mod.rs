use chromiumoxide_cdp::cdp::js_protocol::runtime::RemoteObjectId;
use serde::{Serialize, Deserialize};

use super::*;

mod special_value;
mod evaluator;
mod global;
mod patterns;

pub(crate) use special_value::*;
pub(crate) use evaluator::*;
pub(crate) use global::*;
pub(crate) use patterns::*;

type JsonObject = serde_json::Map<String, JsonValue>;

pub(crate) type JsonPointer = Vec<JsonPointerSegment>;
pub(crate) type JsonPointerRef<'a> = &'a [JsonPointerSegment];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum JsonPointerSegment {
    Field(String),
    Index(usize),
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

/// A descriptor of a JSON object with special values.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ValueDescriptor {
    // The JSON value.
    #[serde(default)]
    pub value: JsonValue,

    // The paths of the special values.
    #[serde(default)]
    pub paths: Vec<JsonPointer>,
}

impl ValueDescriptor {
    #[allow(dead_code)]
    pub fn parse(json: JsonValue) -> (Self, Vec<SpecialValue>) {
        Self::parse_with_expr(json, &mut vec![], &[])
    }

    pub fn parse_with_expr(
        mut json: JsonValue,
        exprs: &mut Vec<(JsonPointer, String)>,
        expr_prefix: JsonPointerRef<'_>,
    ) -> (Self, Vec<SpecialValue>) {
        let (paths, specials) = utils::split_from_json(&mut json, exprs, expr_prefix);
        (
            Self {
                value: json,
                paths,
            },
            specials,
        )
    }

    pub fn merge(mut self, specials: Vec<SpecialValue>) -> crate::error::Result<JsonValue> {
        utils::merge_into_json(&mut self.value, self.paths, specials)?;
        Ok(self.value)
    }
}

mod utils {
    use super::*;

    pub(super) fn split_from_json(
        json: &mut JsonValue,
        exprs: &mut Vec<(JsonPointer, String)>,
        expr_prefix: JsonPointerRef<'_>,
    ) -> (Vec<JsonPointer>, Vec<SpecialValue>) {
        fn split_impl(
            json: &mut JsonValue,
            current: JsonPointer,
            paths: &mut Vec<JsonPointer>,
            specials: &mut Vec<SpecialValue>,
            exprs: &mut Vec<(JsonPointer, String)>,
            expr_prefix: JsonPointerRef<'_>,
        ) {
            match json {
                JsonValue::Object(obj) => {
                    if let Some(expr) = JsExpr::deserialize(&*obj).ok() {
                        let mut path= expr_prefix.to_owned();
                        path.extend(current);

                        exprs.push((path, expr.into_inner().into()));
                    } else if let Some(special) = SpecialValue::from_json(obj) {
                        paths.push(current);
                        specials.push(special);
                        *obj = JsonObject::new();
                    } else {
                        for (key, val) in obj.iter_mut() {
                            let mut new_path = current.clone();
                            new_path.push(JsonPointerSegment::Field(key.clone()));
                            split_impl(val, new_path, paths, specials, exprs, expr_prefix);
                        }
                    }
                }
                JsonValue::Array(arr) => {
                    for (idx, val) in arr.iter_mut().enumerate() {
                        let mut new_path = current.clone();
                        new_path.push(JsonPointerSegment::Index(idx));
                        split_impl(val, new_path, paths, specials, exprs, expr_prefix);
                    }
                }
                _ => (),
            }
        }

        let mut paths = Vec::new();
        let mut specials = Vec::new();
        split_impl(json, vec![], &mut paths, &mut specials, exprs, expr_prefix);
        (paths, specials)
    }

    pub(super) fn merge_into_json(
        json: &mut JsonValue,
        paths: Vec<JsonPointer>,
        specials: Vec<SpecialValue>
    ) -> crate::error::Result<()> {
        fn merge_impl(
            json: &mut JsonValue,
            path: JsonPointerRef<'_>,
            special: SpecialValue
        ) -> crate::error::Result<()> {
            if path.is_empty() {
                *json = special.into_json()?;
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
                        merge_impl(prop, &path[1..], special)?;
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
                        merge_impl(item, &path[1..], special)?;
                    }
                }
            }
            Ok(())
        }

        for (path, special) in paths.into_iter().zip(specials.into_iter()) {
            merge_impl(json, &path[..], special)?;
        }
        Ok(())
    }
}
