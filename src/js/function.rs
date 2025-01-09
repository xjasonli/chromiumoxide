use std::marker::PhantomData;
use std::sync::Arc;
use schemars::Schema;
use serde::{Deserialize, Serialize, de::DeserializeSeed};
use serde_json::Value as JsonValue;
use crate::cdp::js_protocol::runtime::{
    CallArgument, CallFunctionOnParams, ExecutionContextId, GetPropertiesParams, RemoteObjectId
};
use crate::error::{CdpError, Result};
use crate::handler::PageInner;
use super::JsObject;
use super::{native::{FunctionNativeArgs, NativeValueFromJs}, object::OBJECT_ID_KEY};
use super::de::PageDeserializeSeed;

type JsonObject = serde_json::Map<String, JsonValue>;

/// Represents a JavaScript function that can be executed in the browser's runtime.
/// 
/// This struct provides a way to declare and execute JavaScript functions in the browser context.
/// It supports configuring the execution context and binding the function to specific objects
/// before calling it.
/// 
/// # Example
/// ```no_run
/// # use chromiumoxide::Page;
/// # let page: Page = unimplemented!();
/// let func = Function::new(page.into_inner(), "(x, y) => x + y")
///     .with_context(execution_context_id)
///     .call((1, 2))
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct Function {
    /// The page context where the function will be executed
    page: Arc<PageInner>,
    /// The JavaScript function declaration as a string
    function_declaration: String,
    /// The context configuration for function execution
    context: CallContext,
}

impl Function {
    /// Creates a new Function with the given page context and function declaration.
    pub(crate) fn new(page: Arc<PageInner>, function_declaration: impl Into<String>) -> Self {
        Self {
            page,
            function_declaration: function_declaration.into(),
            context: CallContext::default(),
        }
    }

    /// Sets the execution context for the function.
    /// 
    /// This method allows you to specify the context in which the function will be executed.
    /// The context can be default, a specific execution context ID, or a JavaScript object.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::Page;
    /// # let page: Page = unimplemented!();
    /// let func = Function::new(page.into_inner(), "(x, y) => x + y")
    ///     .with_context(execution_context_id)
    ///     .call((1, 2))
    ///     .await?;
    /// ```
    pub fn with_context(self, context: impl Into<CallContext>) -> Self {
        Self {
            page: self.page,
            function_declaration: self.function_declaration,
            context: context.into(),
        }
    }

    /// Binds the function to a specific JavaScript object.
    /// 
    /// This method sets the `this` context of the function to the specified object.
    /// It's useful when the function needs to be executed in the context of a particular object.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::Page;
    /// # let page: Page = unimplemented!();
    /// let func = Function::new(page.into_inner(), "function() { return this.value; }")
    ///     .with_object(object_id)
    ///     .call(())
    ///     .await?;
    /// ```
    pub fn with_object(self, object: impl Into<RemoteObjectId>) -> Self {
        Self {
            page: self.page,
            function_declaration: self.function_declaration,
            context: CallContext::Object(object.into()),
        }
    }

    /// Sets the function to run in a specific execution context.
    /// 
    /// This method allows you to specify which execution context the function should run in.
    /// An execution context represents an isolated JavaScript environment.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::Page;
    /// # let page: Page = unimplemented!();
    /// let func = Function::new(page.into_inner(), "(x, y) => x + y")
    ///     .with_execution_context(context_id)
    ///     .call((1, 2))
    ///     .await?;
    /// ```
    pub fn with_execution_context(self, execution_context: impl Into<ExecutionContextId>) -> Self {
        Self {
            page: self.page,
            function_declaration: self.function_declaration,
            context: CallContext::ExecutionContext(execution_context.into()),
        }
    }

    /// Executes the function with the given arguments and returns the result.
    /// 
    /// This method executes the JavaScript function in the browser and converts the result
    /// to a Rust type. It supports asynchronous operations and can handle complex argument
    /// and return value types.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::Page;
    /// # let page: Page = unimplemented!();
    /// let sum: i32 = Function::new(page.into_inner(), "(x, y) => x + y")
    ///     .call((1, 2))
    ///     .await?;
    /// ```
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
        let seed = PageDeserializeSeed::new(self.page.clone(), PhantomData);
        let result = seed.deserialize(json)?;
        Ok(result)
    }

    pub(super) async fn call_impl(&self, args: Vec<JsonValue>, schema: Schema) -> Result<JsonValue> {
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
                CallContext::ExecutionContext(id) => {
                    params.execution_context_id = Some(*id);
                }
                CallContext::Object(id) => {
                    params.object_id = Some(id.clone());
                }
                CallContext::Default => {
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

        let descriptor: JsonDescriptor = {
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
            descriptor.json
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

            let mut objects = Vec::new();
            objects.reserve(length);

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
                objects.push(RemoteObjectId::new(object_id));
            }

            JsonWithRemoteObjects::new(descriptor, objects).into_json()
        };

        Ok(json)
    }
}

/// Represents the execution context in which a JavaScript function will be called.
/// 
/// This enum defines three possible contexts for function execution:
/// - `Default`: Uses the default execution context of the page
/// - `ExecutionContext`: Uses a specific execution context identified by its ID
/// - `Object`: Binds the function to a specific JavaScript object as its `this` context
/// 
/// # Example
/// ```no_run
/// # use chromiumoxide::Page;
/// # let page: Page = unimplemented!();
/// let func = Function::new(page.into_inner(), "(x, y) => x + y")
///     .with_context(CallContext::Default)
///     .call((1, 2))
///     .await?;
/// ```
#[derive(Default, Debug, Clone)]
pub enum CallContext {
    /// The default execution context of the page.
    #[default]
    Default,
    /// A specific execution context identified by its ID.
    ExecutionContext(ExecutionContextId),
    /// A specific JavaScript object to be used as the `this` context.
    Object(RemoteObjectId),
}

impl From<ExecutionContextId> for CallContext {
    fn from(id: ExecutionContextId) -> Self {
        Self::ExecutionContext(id)
    }
}
impl From<RemoteObjectId> for CallContext {
    fn from(id: RemoteObjectId) -> Self {
        Self::Object(id)
    }
}
impl From<&RemoteObjectId> for CallContext {
    fn from(id: &RemoteObjectId) -> Self {
        Self::Object(id.clone())
    }
}
impl From<JsObject> for CallContext {
    fn from(object: JsObject) -> Self {
        Self::from(&object)
    }
}
impl From<&JsObject> for CallContext {
    fn from(object: &JsObject) -> Self {
        Self::from(object.id())
    }
}
impl From<()> for CallContext {
    fn from(_: ()) -> Self {
        Self::Default
    }
}
impl From<Option<ExecutionContextId>> for CallContext {
    fn from(execution_context_id: Option<ExecutionContextId>) -> Self {
        match execution_context_id {
            Some(id) => Self::from(id),
            None => Self::Default,
        }
    }
}

const REAL_FUNCTION: &str = "async function() {
    const userFunction = (__FUNCTION__);

    var arguments = Array.from(arguments);
    let { jsonDescriptors, resultPatterns } = arguments.pop();

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
    for (let i = 0; i < jsonDescriptors.length; i++) {
        const paths = jsonDescriptors[i].paths;
        for (let j = 0; j < paths.length; j++) {
            const path = paths[j];
            const object = arguments[offset + j];
            jsonDescriptors[i].json = mergeObject(jsonDescriptors[i].json, path, object);
            offset += 1;
        }
        args.push(jsonDescriptors[i].json);
    }

    var json = await Promise.resolve(userFunction.apply(this, args));
    let paths = [];
    let objects = [];
    for (let i = 0; i < resultPatterns.length; i++) {
        const pattern = resultPatterns[i];
        json = splitObject(json, pattern, [], paths, objects);
    }
    return {
        descriptor: {
            json: json,
            paths: paths,
        },
        objects: objects,
    };
}";

#[derive(Debug)]
struct JsonWithRemoteObjects {
    descriptor: JsonDescriptor,
    objects: Vec<RemoteObjectId>,
}

impl JsonWithRemoteObjects {
    fn new(descriptor: JsonDescriptor, objects: Vec<RemoteObjectId>) -> Self {
        Self {
            descriptor,
            objects,
        }
    }

    fn from_json(mut json: JsonValue) -> Self {
        let (paths, objects) = split_from_json(&mut json);
        Self {
            descriptor: JsonDescriptor {
                json,
                paths,
            },
            objects,
        }
    }

    fn into_json(mut self) -> JsonValue {
        merge_into_json(&mut self.descriptor.json, self.descriptor.paths, self.objects);
        self.descriptor.json
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
pub(crate) struct JsonDescriptor {
    #[serde(default)]
    pub json: JsonValue,
    #[serde(default)]
    pub paths: Vec<JsonPath>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CallDescriptor {
    #[serde(default)]
    json_descriptors: Vec<JsonDescriptor>,
    #[serde(default)]
    result_patterns: Vec<JsonPattern>,
}

impl CallDescriptor {
    fn new(args: Vec<JsonValue>, schema: &Schema) -> (Self, Vec<RemoteObjectId>) {
        let patterns = parse_json_schema(&schema);
        let mut descriptor = Self {
            json_descriptors: vec![],
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
        self.json_descriptors.push(json_with_remote_objects.descriptor);
        json_with_remote_objects.objects
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
                        // https://json-schema.org/understanding-json-schema/reference/array
                        let prefix_items = obj.get("prefixItems").unwrap_or(&default_null);
                        if prefix_items.is_array() {
                            for (idx, item) in prefix_items.as_array().unwrap().iter().enumerate() {
                                let mut new_path = current.clone();
                                new_path.push(JsonPatternSegment::Index(Some(idx)));
                                parse_impl(item, new_path, patterns);
                            }
                        }

                        let items = obj.get("items").unwrap_or(&default_null);
                        if items.is_object() {
                            let mut new_path = current.clone();
                            new_path.push(JsonPatternSegment::Index(None));
                            parse_impl(items, new_path, patterns);
                        }
                    }
                    "object" => {
                        // https://json-schema.org/understanding-json-schema/reference/object
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
                    // TODO: allOf/anyOf/oneOf
                    // https://json-schema.org/understanding-json-schema/reference/combining
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
