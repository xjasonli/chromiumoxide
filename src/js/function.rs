use std::marker::PhantomData;
use std::sync::Arc;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::cdp::js_protocol::runtime::{
    CallArgument, CallFunctionOnParams, ExecutionContextId, GetPropertiesParams, RemoteObjectId
};
use crate::error::{CdpError, Result};
use crate::handler::PageInner;
use super::{native::{FunctionNativeArgs, NativeValueFromJs, PageSeed}, object::OBJECT_ID_KEY};

type JsonObject = serde_json::Map<String, JsonValue>;

#[derive(Debug, Clone)]
enum CallContext {
    ExecutionContext(ExecutionContextId),
    Object(RemoteObjectId),
}

#[derive(Debug)]
pub struct Function {
    page: Arc<PageInner>,
    function_declaration: String,
    context: Option<CallContext>,
}

impl Function {
    pub(crate) fn new(page: Arc<PageInner>, function: impl Into<String>) -> Self {
        Self {
            page,
            function_declaration: function.into(),
            context: None,
        }
    }

    pub fn with_object(self, object: impl Into<RemoteObjectId>) -> Self {
        Self {
            page: self.page,
            function_declaration: self.function_declaration,
            context: Some(CallContext::Object(object.into())),
        }
    }

    pub fn with_execution_context(self, execution_context: impl Into<ExecutionContextId>) -> Self {
        Self {
            page: self.page,
            function_declaration: self.function_declaration,
            context: Some(CallContext::ExecutionContext(execution_context.into())),
        }
    }

    async fn call_impl(&self, args: Vec<JsonValue>, schema: Schema) -> Result<JsonValue> {
        let function_declaration = REAL_FUNCTION.replace(
            "__FUNCTION__",
            self.function_declaration.as_str(),
        );

        let mut params = CallFunctionOnParams::builder()
            .function_declaration(function_declaration)
            .await_promise(true)
            .return_by_value(false)
            .build().unwrap();
        {
            match &self.context {
                Some(CallContext::ExecutionContext(id)) => {
                    params.execution_context_id = Some(*id);
                }
                Some(CallContext::Object(id)) => {
                    params.object_id = Some(id.clone());
                }
                None => {
                    let context_id = self.page
                        .execution_context()
                        .await?
                        .ok_or(CdpError::NotFound)?;
                    params.execution_context_id = Some(context_id);
                }
            }

            params.arguments = Some({
                let (descriptor, object_ids) = CallDescriptor::new(args, &schema);
                let mut arguments = object_ids.into_iter()
                    .map(|id| CallArgument::builder().object_id(id).build())
                    .collect::<Vec<_>>();
                arguments.push(CallArgument::builder().value(serde_json::to_value(descriptor)?).build());
                arguments
            });
        }

        let result_id = {
            let resp = self.page.execute(params).await?.result;
            if let Some(exception) = resp.exception_details {
                return Err(CdpError::JavascriptException(Box::new(exception)));
            }
            resp.result.object_id.ok_or(CdpError::NotFound)?
        };

        let descriptor: ValueDescriptor = {
            let params = CallFunctionOnParams::builder()
                .function_declaration("function() { return this.descriptor; }")
                .object_id(result_id.clone())
                .return_by_value(true)
                .build().unwrap();
            let resp = self.page.execute(params).await?.result;
            if let Some(exception) = resp.exception_details {
                return Err(CdpError::JavascriptException(Box::new(exception)));
            }
            let json = resp.result.value.ok_or(CdpError::NotFound)?;
            serde_json::from_value(json)?
        };

        let json = if descriptor.paths.is_empty() {
            descriptor.value
        } else {
            let array_id = {
                let params = CallFunctionOnParams::builder()
                    .function_declaration("function() { return this.objects; }")
                    .object_id(result_id)
                    .return_by_value(false)
                    .build().unwrap();
                let resp = self.page.execute(params).await?.result;
                if let Some(exception) = resp.exception_details {
                    return Err(CdpError::JavascriptException(Box::new(exception)));
                }
                resp.result.object_id.ok_or(CdpError::NotFound)?
            };
            let mut properties = {
                let params = GetPropertiesParams::builder()
                    .object_id(array_id)
                    .own_properties(true)
                    .build().unwrap();
                let resp = self.page.execute(params).await?.result;
                if let Some(exception) = resp.exception_details {
                    return Err(CdpError::JavascriptException(Box::new(exception)));
                }
                resp.result
            };

            let length = {
                let length_index = properties.iter().enumerate()
                    .find(|(_, p)| p.name == "length")
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);
                properties.remove(length_index)
                    .value
                    .map(|v| v.value.unwrap_or_default())
                    .unwrap_or_default()
                    .as_u64()
                    .unwrap_or(0) as usize
            };

            if length != descriptor.paths.len() {
                return Err(CdpError::NotFound);
            }

            let mut object_ids = Vec::new();
            object_ids.reserve(length);

            for idx in 0..length {
                let property_index = properties.iter().enumerate()
                    .find(|(_, p)| p.name == format!("{}", idx))
                    .map(|(idx, _)| idx)
                    .ok_or(CdpError::NotFound)?;
                let property = properties.remove(property_index);
                let object_id = property.value
                    .ok_or(CdpError::NotFound)?
                    .object_id
                    .ok_or(CdpError::NotFound)?;
                object_ids.push(RemoteObjectId::new(object_id));
            }

            JsonWithRemoteObjects {
                json: descriptor.value,
                object_paths: descriptor.paths,
                object_ids,
            }.into_json()
        };

        Ok(json)
    }

    pub async fn call<R: NativeValueFromJs, A: FunctionNativeArgs>(
        &self,
        args: A,
    ) -> Result<R, CdpError> {
        let schema = {
            let mut settings = schemars::generate::SchemaSettings::default();
            settings.inline_subschemas = true;
            settings.into_generator().into_root_schema_for::<R>()
        };

        let args = A::into_json_values(args)?;
        let json = self.call_impl(args, schema).await?;
        let seed = PageSeed::new(self.page.clone(), PhantomData);
        let result = serde::de::DeserializeSeed::deserialize(seed, json)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;
    use crate::error::Result;
    use crate::js::object::JsObject;
    use crate::JsonSchema;

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct TestObject {
        a: i32,
        val: String,

        obj: JsObject,
    }

    async fn test_function(func: Function, js_object: JsObject) -> Result<()> {
        let result: Vec<TestObject> = func.call((&js_object, 5, "str_value", ["sfffff", "sdfsf"])).await?;

        Ok(())
    }
}


const REAL_FUNCTION: &str = "async function() {
    const userFunction = (__FUNCTION__);

    var arguments = Array.from(arguments);
    let { valueDescriptors, resultPatterns } = arguments.pop();

    let mergeObject = function(value, path, object) {
        if (path.length === 0) {
            return object;
        }

        const segment = path[0];
        if (typeof segment === 'string' && typeof value !== 'object') {
            value = {};
        } else if (typeof segment === 'number' && !Array.isArray(value)) {
            value = [];
        }
        value[segment] = mergeObject(value[segment], path.slice(1), object);
        return value;
    };

    let splitObject = function(value, pattern, path, paths, objects) {
        if (pattern.length === 0) {
            if (typeof value === 'object') {
                paths.push(path);
                objects.push(value);
                value = {};
            }
            return value;
        }

        const segment = pattern[0];
        if ('property' in segment && typeof value === 'object') {
            const property = segment.property;
            if (property === null) {
                // wildcard
                for (const [key, val] of Object.entries(value)) {
                    value[key] = splitObject(val, pattern.slice(1), path.concat([key]), paths, objects);
                }
            } else if (property in value) {
                value[property] = splitObject(value[property], pattern.slice(1), path.concat([property]), paths, objects);
            }
        } else if ('index' in segment && Array.isArray(value)) {
            const index = segment.index;
            if (index === null) {
                // wildcard
                for (let i = 0; i < value.length; i++) {
                    value[i] = splitObject(value[i], pattern.slice(1), path.concat([i]), paths, objects);
                }
            } else if (index < value.length) {
                value[index] = splitObject(value[index], pattern.slice(1), path.concat([index]), paths, objects);
            }
        }

        return value;
    }

    let offset = 0;
    let args = [];
    for (let i = 0; i < valueDescriptors.length; i++) {
        const paths = valueDescriptors[i].paths;
        for (let j = 0; j < paths.length; j++) {
            const path = paths[j];
            const object = arguments[offset + j];
            valueDescriptors[i].value = mergeObject(valueDescriptors[i].value, path, object);
            offset += 1;
        }
        args.push(valueDescriptors[i].value);
    }

    var value = await Promise.resolve(userFunction.apply(this, args));
    let paths = [];
    let objects = [];
    for (let i = 0; i < resultPatterns.length; i++) {
        const pattern = resultPatterns[i];
        value = splitObject(value, pattern, [], paths, objects);
    }
    return {
        descriptor: {
            value: value,
            paths: paths,
        },
        objects: objects,
    };
}";

#[derive(Debug)]
pub struct JsonWithRemoteObjects {
    json: JsonValue,
    object_paths: Vec<JsonPath>,
    object_ids: Vec<RemoteObjectId>,
}

impl JsonWithRemoteObjects {
    fn from_json(mut json: JsonValue) -> Self {
        let (object_paths, object_ids) = split_from_json(&mut json);
        Self {
            json,
            object_paths,
            object_ids,
        }
    }

    fn into_json(mut self) -> JsonValue {
        merge_into_json(&mut self.json, self.object_paths, self.object_ids);
        self.json
    }

}

fn split_from_json(json: &mut JsonValue) -> (Vec<JsonPath>, Vec<RemoteObjectId>) {
    fn split_impl(
        json: &mut JsonValue,
        current: JsonPath,
        object_paths: &mut Vec<JsonPath>,
        object_ids: &mut Vec<RemoteObjectId>
    ) {
        match json {
            JsonValue::Object(obj) => {
                if let Some(JsonValue::String(id)) = obj.remove(OBJECT_ID_KEY) {
                    object_paths.push(current);
                    object_ids.push(RemoteObjectId::new(id));
                    *obj = JsonObject::new();
                } else {
                    for (key, val) in obj.iter_mut() {
                        let mut new_path = current.clone();
                        new_path.push(JsonPathSegment::Property(key.clone()));
                        split_impl(val, new_path, object_paths, object_ids);
                    }
                }
            }
            JsonValue::Array(arr) => {
                for (idx, val) in arr.iter_mut().enumerate() {
                    let mut new_path = current.clone();
                    new_path.push(JsonPathSegment::Index(idx));
                    split_impl(val, new_path, object_paths, object_ids);
                }
            }
            _ => (),
        }
    }

    let mut object_paths = Vec::new();
    let mut object_ids = Vec::new();
    split_impl(json, vec![], &mut object_paths, &mut object_ids);
    (object_paths, object_ids)
}

fn merge_into_json(
    json: &mut JsonValue,
    object_paths: Vec<JsonPath>,
    object_ids: Vec<RemoteObjectId>
) {
    fn merge_impl(json: &mut JsonValue, path: JsonPathRef<'_>, id: RemoteObjectId) {
        use std::ops::IndexMut;

        if path.is_empty() {
            *json = serde_json::json!({ OBJECT_ID_KEY: String::from(id) });
        } else {
            match &path[0] {
                JsonPathSegment::Property(s) => {
                    if !json.is_object() {
                        *json = serde_json::json!({});
                    }
                    let attr = json.index_mut(s);
                    merge_impl(attr, &path[1..], id);
                }
                JsonPathSegment::Index(n) => {
                    if !json.is_array() {
                        *json = serde_json::json!([]);
                    }
                    let item = json.index_mut(n);
                    merge_impl(item, &path[1..], id);
                }
            }
        }
    }

    for (path, id) in object_paths.into_iter().zip(object_ids.into_iter()) {
        merge_impl(json, &path[..], id);
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueDescriptor {
    value: JsonValue,
    paths: Vec<JsonPath>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CallDescriptor {
    value_descriptors: Vec<ValueDescriptor>,
    result_patterns: Vec<JsonPattern>,
}

impl CallDescriptor {
    fn new(args: Vec<JsonValue>, schema: &Schema) -> (Self, Vec<RemoteObjectId>) {
        let patterns = parse_json_schema(&schema);
        let mut descriptor = Self {
            value_descriptors: vec![],
            result_patterns: patterns,
        };
        let mut object_ids = Vec::new();
        for arg in args {
            object_ids.extend(descriptor.push(arg));
        }
        (descriptor, object_ids)
    }

    fn push(&mut self, arg: JsonValue) -> Vec<RemoteObjectId> {
        let json_with_remote_objects = JsonWithRemoteObjects::from_json(arg);
        self.value_descriptors.push(ValueDescriptor {
            value: json_with_remote_objects.json,
            paths: json_with_remote_objects.object_paths,
        });
        json_with_remote_objects.object_ids
    }
}

type JsonPath = Vec<JsonPathSegment>;
type JsonPathRef<'a> = &'a [JsonPathSegment];
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonPathSegment {
    Property(String),
    Index(usize),
}

impl From<String> for JsonPathSegment {
    fn from(s: String) -> Self {
        JsonPathSegment::Property(s)
    }
}
impl From<usize> for JsonPathSegment {
    fn from(n: usize) -> Self {
        JsonPathSegment::Index(n)
    }
}
impl From<&str> for JsonPathSegment {
    fn from(s: &str) -> Self {
        JsonPathSegment::Property(s.to_string())
    }
}

type JsonPattern = Vec<JsonPatternSegment>;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JsonPatternSegment {
    Property(Option<String>),
    Index(Option<usize>),
}

fn parse_json_schema(schema: &schemars::Schema) -> Vec<JsonPattern> {
    fn parse_impl(schema: &JsonValue, current: JsonPattern, patterns: &mut Vec<JsonPattern>) {
        let default_null = serde_json::json!(null);
        match schema {
            JsonValue::Object(obj) => {
                match obj.get("type").unwrap_or(&default_null).as_str().unwrap_or_default() {
                    "array" => {
                        let items = obj.get("items").unwrap_or(&default_null);
                        if items.is_object() {
                            let mut new_path = current.clone();
                            new_path.push(JsonPatternSegment::Index(None));
                            parse_impl(items, new_path, patterns);
                        } else if items.is_array(){
                            for (idx, item) in items.as_array().unwrap().iter().enumerate() {
                                let mut new_path = current.clone();
                                new_path.push(JsonPatternSegment::Index(Some(idx)));
                                parse_impl(item, new_path, patterns);
                            }
                        }
                    }
                    "object" => {
                        let properties = obj.get("properties").unwrap_or(&default_null);
                        if properties.is_object() {
                            let properties = properties.as_object().unwrap();
                            if properties.get(OBJECT_ID_KEY).is_some() {
                                patterns.push(current);
                                return;
                            }

                            for (key, val) in properties.iter() {
                                let mut new_path = current.clone();
                                new_path.push(JsonPatternSegment::Property(Some(key.clone())));
                                parse_impl(val, new_path, patterns);
                            }
                        }
                        let additional_properties = obj.get("additionalProperties").unwrap_or(&default_null);
                        if additional_properties.is_boolean() {
                            let mut new_path = current.clone();
                            new_path.push(JsonPatternSegment::Property(None));
                            parse_impl(additional_properties, new_path, patterns);
                        }
                    }
                    _ => {
                    }
                }
            }
            _ => (),
        }
    }

    if let Some(_) = schema.as_object() {
        let mut patterns = Vec::new();
        parse_impl(schema.as_value(), vec![], &mut patterns);
        patterns
    } else {
        vec![]
    }
}
