use std::{collections::{BTreeMap, HashSet}, sync::LazyLock};

use super::*;
use schemars::Schema;

pub(super) fn parse_json_schema(schema: &Schema) -> ReturnMode {
    let value = schema.as_value();
    let mut type_map = TypeMap::new();
    parse_schema(&value, &[], &mut type_map);
    type_map.into_mode()
}

struct TypeMap(BTreeMap<JsonPointer, HashSet<ValueType>>);

impl TypeMap {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn touch(&mut self, pointer: JsonPointerRef<'_>) {
        self.0.entry(pointer.to_owned())
            .or_insert(HashSet::new());
    }

    fn insert(&mut self, pointer: JsonPointerRef<'_>, value_type: impl Into<ValueType>) {
        self.0.entry(pointer.to_owned())
            .or_insert(HashSet::new())
            .insert(value_type.into());
    }

    fn insert_all(&mut self, pointer: JsonPointerRef<'_>, value_types: impl IntoIterator<Item = impl Into<ValueType>>) {
        self.0.entry(pointer.to_owned())
            .or_insert(HashSet::new())
            .extend(value_types.into_iter().map(|v| v.into()));
    }

    fn into_mode(mut self) -> ReturnMode {
        let root = self.0.remove(&vec![]).unwrap_or(HashSet::new());

        // Check if we could discard the result
        if root.is_empty()
            || root == HashSet::from([ValueType::Null])
            || root == HashSet::from([ValueType::Undefined])
        {
            let ty = root.iter().next().unwrap_or(&ValueType::Null);
            return match ty {
                ValueType::Null => ReturnMode::Null,
                ValueType::Undefined => ReturnMode::Undefined,
                _ => unreachable!(),
            };
        }

        // Object and Array may use ByValue mode, but not compatible with RemoteObject
        if root.contains(&ValueType::Object) || root.contains(&ValueType::Array) {
            // RemoteObject should use ById mode, which is not compatible with Object or Array
            // so we should use complex mode
            if root.contains(&ValueType::RemoteObject) {
                return ReturnMode::Complex;
            }

            let leaf = self.0.values()
                .fold(HashSet::new(), |mut acc, types| {
                    acc.extend(types.iter().map(|v| *v));
                    acc
                });
            
            // We have RemoteObject within object or array, use complex mode
            if leaf.contains(&ValueType::RemoteObject) {
                return ReturnMode::Complex;
            }

            // All other types can also use ByValue mode
            return ReturnMode::ByValue;
        }

        // Without Object and Array, all other types can use ById mode
        if root.contains(&ValueType::RemoteObject) {
            return ReturnMode::ById;
        }

        // all other cases, use ByValue mode
        ReturnMode::ByValue
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ValueType {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,

    RemoteObject,
    Undefined,
    BigInt,
}

impl From<&ValueType> for ValueType {
    fn from(value: &ValueType) -> Self {
        *value
    }
}

impl ValueType {
    fn standard_types() -> &'static [Self] {
        &[
            Self::Object,
            Self::Array,
            Self::String,
            Self::Number,
            Self::Integer,
            Self::Boolean,
            Self::Null,
        ]
    }
}

static DEFAULT_TYPES: LazyLock<Vec<&'static str>> = LazyLock::new(||{
    ["object", "array", "string", "number", "integer", "boolean", "null"]
        .into_iter()
        .collect::<Vec<_>>()
});

fn parse_schema(schema: &JsonValue, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    if let Some(boolean) = schema.as_bool() {
        if boolean {
            type_map.insert_all(current_path, ValueType::standard_types());
        } else {
            type_map.touch(current_path);
        }
    } else if let Some(object) = schema.as_object() {
        parse_simple_schema(object, current_path, type_map);
        if let Some(one_of) = object.get("oneOf").and_then(|v| v.as_array()) {
            parse_schema_list(one_of, current_path, type_map);
        }
        if let Some(any_of) = object.get("anyOf").and_then(|v| v.as_array()) {
            parse_schema_list(any_of, current_path, type_map);
        }
        if let Some(all_of) = object.get("allOf").and_then(|v| v.as_array()) {
            parse_schema_list(all_of, current_path, type_map);
        }
    }
}

fn parse_simple_schema(schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    let types = schema.get("type")
        .map(|v| match v {
            JsonValue::String(s) => vec![s.as_str()],
            JsonValue::Array(a) => {
                a.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
            }
            _ => Vec::new(),
        })
        .unwrap_or(DEFAULT_TYPES.clone());
    for typename in types {
        match typename {
            "object" => {
                parse_simple_schema_object(schema, current_path, type_map);
            }
            "array" => {
                parse_simple_schema_array(schema, current_path, type_map);
            }
            "string" => {
                parse_simple_schema_string(schema, current_path, type_map);
            }
            "number" => {
                parse_simple_schema_number(schema, current_path, type_map);
            }
            "integer" => {
                parse_simple_schema_integer(schema, current_path, type_map);
            }
            "boolean" => {
                parse_simple_schema_boolean(schema, current_path, type_map);
            }
            "null" => {
                parse_simple_schema_null(schema, current_path, type_map);
            }
            _ => (),
        }
    }
}


fn parse_schema_list(schema: &[JsonValue], current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    for schema in schema {
        parse_schema(schema, current_path, type_map);
    }
}

fn parse_simple_schema_object(schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
        if properties.contains_key(JS_REMOTE_OBJECT_KEY) {
            type_map.insert(current_path, ValueType::RemoteObject);
            return;
        }
        if properties.contains_key(JS_UNDEFINED_KEY) {
            type_map.insert(current_path, ValueType::Undefined);
            return;
        }
        if properties.contains_key(JS_BIGINT_KEY) {
            type_map.insert(current_path, ValueType::BigInt);
            return;
        }
    }
    type_map.insert(current_path, ValueType::Object);

    if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
        for (key, value) in properties {
            let mut path = current_path.to_owned();
            path.push(JsonPointerSegment::Field(key.to_string()));
            parse_schema(value, &path, type_map);
        }
    }

    let additional_properties = schema.get("additionalProperties").unwrap_or(&JsonValue::Bool(true));
    let mut path = current_path.to_owned();
    // use empty string for wildcard
    path.push(JsonPointerSegment::Field(String::new()));
    parse_schema(additional_properties, &path, type_map);
}

fn parse_simple_schema_array(schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::Array);
    if let Some(prefix_items) = schema.get("prefixItems").and_then(|v| v.as_array()) {
        for (i, item) in prefix_items.into_iter().enumerate() {
            let mut path = current_path.to_owned();
            path.push(JsonPointerSegment::Index(i));
            parse_schema(item, &path, type_map);
        }
    }
    let items = schema.get("items").unwrap_or(&JsonValue::Bool(true));
    let mut path = current_path.to_owned();
    // use usize::MAX for wildcard
    path.push(JsonPointerSegment::Index(usize::MAX));
    parse_schema(items, &path, type_map);
}

fn parse_simple_schema_string(_schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::String);
}

fn parse_simple_schema_number(_schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::Number);
}

fn parse_simple_schema_integer(_schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::Integer);
}

fn parse_simple_schema_boolean(_schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::Boolean);
}

fn parse_simple_schema_null(_schema: &JsonObject, current_path: JsonPointerRef<'_>, type_map: &mut TypeMap) {
    type_map.insert(current_path, ValueType::Null);
}
