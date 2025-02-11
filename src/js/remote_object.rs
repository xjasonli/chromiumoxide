//! Remote object types and traits for JavaScript interop.
//! 
//! This module provides types and traits for interacting with JavaScript objects through the Chrome DevTools Protocol.
 
use std::{ops::Deref, sync::Arc};
use serde::de::Error as _;
use chromiumoxide_cdp::cdp::js_protocol::runtime::{ReleaseObjectParams, RemoteObjectId};
use crate::{error::Result, handler::PageInner, js::de::PageDeserializer};
use super::*;

mod macros;

pub mod object;
pub mod function;
pub mod symbol;

use macros::*;

pub use object::*;
pub use function::*;
pub use symbol::*;

/// A wrapper around JavaScript objects that allows interacting with them from Rust.
/// 
/// This type represents a remote JavaScript object that exists in the browser's JavaScript engine.
/// It provides methods to:
/// - Access object properties and methods
/// - Evaluate JavaScript expressions in the context of this object
/// - Invoke functions with this object as `this` context
/// - Convert between JavaScript and Rust types
///
/// # Example
/// ```no_run
/// use chromiumoxide::js::JsRemoteObject;
/// 
/// # async fn example(obj: JsRemoteObject) {
/// // Get a property
/// let value = obj.get_property::<String>("propertyName").await?;
/// 
/// // Set a property
/// obj.set_property("propertyName", "new value").await?;
/// 
/// // Invoke a method
/// obj.invoke_method("methodName", Default::default())
///    .argument(42)?
///    .invoke::<String>()
///    .await?;
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsRemoteObject(Arc<JsRemoteObjectInner>);

/// Implementation of core functionality for JavaScript remote objects.
/// 
/// This implementation provides:
/// - Object identification and type information
/// - Type checking and downcasting
/// - JavaScript evaluation and function invocation
impl JsRemoteObject {
    /// Returns the unique identifier of this remote object.
    pub fn remote_id(&self) -> RemoteObjectId {
        self.0.data.id.clone()
    }

    /// Returns the JavaScript type of this remote object.
    pub fn remote_type(&self) -> &JsRemoteObjectType {
        &self.0.data.r#type
    }

    /// Returns the JavaScript class name of this remote object.
    /// 
    /// For objects, this is typically the constructor name (e.g., "Array", "Object", "HTMLElement").
    /// For functions, this is usually "Function".
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # fn example(obj: JsRemoteObject) {
    /// match obj.remote_class() {
    ///     "Array" => println!("This is an array"),
    ///     "HTMLElement" => println!("This is an HTML element"),
    ///     _ => println!("Other type: {}", obj.remote_class()),
    /// }
    /// # }
    /// ```
    pub fn remote_class(&self) -> &str {
        &self.0.data.class
    }

    /// Checks if this object is an instance of the specified type.
    /// 
    /// This is similar to JavaScript's `instanceof` operator. It checks whether
    /// the object matches the type constraints (type, subtype, and class name)
    /// of the specified type.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # use chromiumoxide::js::JsElement;
    /// # fn example(obj: JsRemoteObject) {
    /// if obj.is_instance_of::<JsElement>() {
    ///     // This object is an HTML element
    /// }
    /// # }
    /// ```
    pub fn is_instance_of<T: Subclass<Self>>(&self) -> bool {
        T::is_instance(self)
    }

    /// Attempts to downcast this object to a more specific type.
    /// 
    /// If the object matches the type constraints of the target type,
    /// returns a new handle to the object as that type. Otherwise returns None.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # use chromiumoxide::js::JsElement;
    /// # fn example(obj: JsRemoteObject) {
    /// if let Some(element) = obj.downcast::<JsElement>() {
    ///     // Use element-specific methods
    ///     // element.query_selector(".class")...
    /// }
    /// # }
    /// ```
    pub fn downcast<T: Subclass<Self>>(&self) -> Option<<T as Class<Self>>::Owned> {
        T::try_from_super(self.clone())
    }

    /// Evaluates a JavaScript expression in the context of this object.
    /// 
    /// This method executes JavaScript code with `this` bound to the current object.
    /// The expression result will be converted to the specified Rust type.
    /// 
    /// # Arguments
    /// * `expr` - The JavaScript expression to evaluate
    /// * `options` - Evaluation options like timeout and execution context
    /// 
    /// # Returns
    /// The result of evaluating the expression, converted to type `T`
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get object's internal value
    /// let value = obj.eval::<i32>("this.value * 2", Default::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn eval<T: NativeValueFromJs>(
        &self,
        expr: impl Into<String>,
        options: EvalOptions
    ) -> Result<T> {
        let params = EvalParams::new(expr)
            .this(self)
            .options(options);

        self.page().eval(params).await
    }

    /// Creates a function invoker for executing JavaScript functions with this object as `this`.
    /// 
    /// This method allows executing JavaScript functions in the context of the current object.
    /// The function can be specified either as a function object or as a string expression.
    /// 
    /// # Arguments
    /// * `function` - The function to invoke, either as a `JsFunction` or string expression
    /// * `options` - Evaluation options like timeout and execution context
    /// 
    /// # Returns
    /// A `FunctionInvoker` that can be used to add arguments and execute the function
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Call a function with arguments
    /// let result = obj.invoke_function("(x, y) => x + y", Default::default())
    ///     .argument(1)?
    ///     .argument(2)?
    ///     .invoke::<i32>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn invoke_function(&self, function: impl Into<Function>, options: EvalOptions) -> FunctionInvoker {
        let function = function.into();
        let evaluator = match function {
            Function::Func(function) => helper::Evaluator::new_remote(
                self.page(),
                function,
                options
            ),
            Function::Expr(expr) => helper::Evaluator::new_expr(
                self.page(),
                expr,
                Some(self.clone()),
                None,
                options
            ),
        };
        evaluator.invoke(Some(self))
    }

    /// Invokes a method on this object.
    /// 
    /// This is a convenience wrapper around `invoke_function` that looks up a method by name
    /// and invokes it with this object as `this`.
    /// 
    /// # Arguments
    /// * `name` - The name of the method to invoke
    /// * `options` - Evaluation options like timeout and execution context
    /// 
    /// # Returns
    /// A `FunctionInvoker` that can be used to add arguments and execute the method
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Call object's method
    /// let result = obj.invoke_method("calculate", Default::default())
    ///     .argument(42)?
    ///     .invoke::<String>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn invoke_method(&self, name: impl Into<String>, options: EvalOptions) -> FunctionInvoker {
        let function = "(name, ...args) => {{ return this[name](...args); }}";
        self.invoke_function(function, options)
            .argument(name.into())
            .unwrap() // String serializing is infallible
    }

    /// Invokes a well-known Symbol method on this object.
    /// 
    /// This method executes a method referenced by a well-known Symbol (like `Symbol.iterator`).
    /// The method is called with `this` bound to the current object.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Call Symbol.iterator method
    /// let iterator = obj.invoke_symbol_method("iterator", Default::default())
    ///     .invoke::<JsIterator>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn invoke_symbol_method(&self, symbol: impl Into<String>, options: EvalOptions) -> FunctionInvoker {
        let expr = format!("this[Symbol.{}](...arguments)", symbol.into());
        self.invoke_function(expr, options)
    }

    /// Invokes a Symbol method registered with `Symbol.for()` on this object.
    /// 
    /// This method executes a method referenced by a Symbol registered in the global Symbol registry.
    /// The method is called with `this` bound to the current object.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Call method referenced by a registered Symbol
    /// let result = obj.invoke_symbol_method_for("mySymbol", Default::default())
    ///     .invoke::<String>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn invoke_symbol_method_for(&self, symbol: impl Into<String>, options: EvalOptions) -> FunctionInvoker {
        let function = "(symbol, ...args) => {{ return this[Symbol.for(symbol)](...args); }}";
        self.invoke_function(function, options)
            .argument(symbol.into())
            .unwrap() // String serializing is infallible
    }

    /// Gets a property value from this object.
    /// 
    /// This method retrieves a property value by name and converts it to the specified Rust type.
    /// 
    /// # Arguments
    /// * `name` - The name of the property to get
    /// 
    /// # Returns
    /// The property value converted to type `T`
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// let title: String = obj.get_property("title").await?;
    /// let count: i32 = obj.get_property("count").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_property<T>(&self, name: impl Into<String>) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        let function = "(name) => {{ return this[name]; }}";
        self.invoke_function(function, EvalOptions::default())
            .argument(name.into())?
            .invoke::<T>().await
    }

    /// Gets a property value referenced by a well-known Symbol from this object.
    /// 
    /// This method retrieves a property value referenced by a well-known Symbol (like `Symbol.toStringTag`)
    /// and converts it to the specified Rust type.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get Symbol.toStringTag property
    /// let tag: String = obj.get_symbol_property("toStringTag").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbol_property<T>(&self, symbol: impl Into<String>) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        let expr = format!("this[Symbol.{}])", symbol.into());
        self.eval(expr, EvalOptions::default()).await
    }

    /// Gets a property value referenced by a Symbol from the global registry.
    /// 
    /// This method retrieves a property value referenced by a Symbol registered with `Symbol.for()`
    /// and converts it to the specified Rust type.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get property referenced by a registered Symbol
    /// let value: String = obj.get_symbol_property_for("mySymbol").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbol_property_for<T>(&self, symbol: impl Into<String>) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        let function = "(symbol) => {{ return this[Symbol.for(symbol)]; }}";
        self.invoke_function(function, EvalOptions::default())
            .argument(symbol.into())?
            .invoke::<T>().await
    }

    /// Sets a property value on this object.
    /// 
    /// This method sets a property value by name, converting the Rust value to JavaScript.
    /// 
    /// # Arguments
    /// * `name` - The name of the property to set
    /// * `value` - The value to set, which must be convertible to JavaScript
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// obj.set_property("title", "New Title").await?;
    /// obj.set_property("count", 42).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_property<T>(&self, name: impl Into<String>, value: T) -> Result<()>
    where
        T: NativeValueIntoJs,
    {
        let function = "(name, value) => {{ this[name] = value; }}";
        self.invoke_function(function, EvalOptions::default())
            .argument(name.into())?
            .argument(value)?
            .invoke().await
    }

    /// Sets a property value referenced by a well-known Symbol on this object.
    /// 
    /// This method sets a property value referenced by a well-known Symbol (like `Symbol.toStringTag`),
    /// converting the Rust value to JavaScript.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Set Symbol.toStringTag property
    /// obj.set_symbol_property("toStringTag", "MyObject").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_symbol_property<T>(&self, symbol: impl Into<String>, value: T) -> Result<()>
    where
        T: NativeValueIntoJs,
    {
        let function = format!("(value) => {{ this[Symbol.{} = value; }}", symbol.into());
        self.invoke_function(function, EvalOptions::default())
            .argument(value)?
            .invoke().await
    }

    /// Sets a property value referenced by a Symbol from the global registry.
    /// 
    /// This method sets a property value referenced by a Symbol registered with `Symbol.for()`,
    /// converting the Rust value to JavaScript.
    /// 
    /// # Example
    /// ```no_run
    /// # use chromiumoxide::js::JsRemoteObject;
    /// # async fn example(obj: JsRemoteObject) -> Result<(), Box<dyn std::error::Error>> {
    /// // Set property referenced by a registered Symbol
    /// obj.set_symbol_property_for("mySymbol", "value").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_symbol_property_for<T>(&self, symbol: impl Into<String>, value: T) -> Result<()>
    where
        T: NativeValueIntoJs,
    {
        let function = "(symbol, value) => {{ this[Symbol.for(symbol)] = value; }}";
        self.invoke_function(function, EvalOptions::default())
            .argument(symbol.into())?
            .argument(value)?
            .invoke().await
    }
}

impl JsRemoteObject {
    pub(crate) fn new(page: Arc<PageInner>, data: helper::JsRemote) -> Self {
        Self(Arc::new(JsRemoteObjectInner { page, data }))
    }

    pub(crate) fn page(&self) -> Arc<PageInner> {
        self.0.page.clone()
    }
}

impl From<JsRemoteObject> for RemoteObjectId {
    fn from(value: JsRemoteObject) -> Self {
        value.remote_id()
    }
}

impl From<&JsRemoteObject> for RemoteObjectId {
    fn from(value: &JsRemoteObject) -> Self {
        value.remote_id()
    }
}

impl serde::Serialize for JsRemoteObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for JsRemoteObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let inner = JsRemoteObjectInner::deserialize(deserializer)?;
        Ok(JsRemoteObject(Arc::new(inner)))
    }
}

impl schemars::JsonSchema for JsRemoteObject {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        JsRemoteObjectInner::schema_name()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        JsRemoteObjectInner::schema_id()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        JsRemoteObjectInner::json_schema(generator)
    }
}

impl private::Sealed for JsRemoteObject {}
impl Class<JsRemoteObject> for JsRemoteObject {
    type Owned = Self;
    fn as_ref(&self) -> &JsRemoteObject { self }
}

/// Internal storage for JsRemoteObject.
/// 
/// Contains the page reference and remote object data needed to interact with
/// the JavaScript object in the browser.
#[derive(Debug, Clone)]
pub(crate) struct JsRemoteObjectInner {
    pub(crate) page: Arc<PageInner>,
    pub(crate) data: helper::JsRemote,
}

impl PartialEq for JsRemoteObjectInner {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.page, &other.page)
            && self.data == other.data
    }
}
impl Eq for JsRemoteObjectInner {}

/// Automatically releases the remote object when it is dropped.
impl Drop for JsRemoteObjectInner {
    fn drop(&mut self) {
        let _ = self.page.execute_no_wait(
            ReleaseObjectParams::builder()
                .object_id(self.data.id.clone())
                .build().unwrap()
        );
    }
}

impl serde::Serialize for JsRemoteObjectInner {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.data.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for JsRemoteObjectInner {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(JsRemoteObjectInner {
            page: PageDeserializer::try_get(&deserializer)?,
            data: helper::JsRemote::deserialize(deserializer)?,
        })
    }
}

impl schemars::JsonSchema for JsRemoteObjectInner {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("JsRemoteObject")
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(std::concat!(
            ::core::module_path!(),
            "::",
            "JsRemoteObject"
        ))
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        generator.subschema_for::<helper::JsRemote>()
    }
}

/// The type of a JavaScript remote object.
/// 
/// This represents the basic JavaScript type system categories:
/// - Object (with optional subtypes for arrays, DOM nodes, etc)
/// - Function
/// - Symbol
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JsRemoteObjectType {
    /// A JavaScript object, with an optional subtype for specific object kinds
    Object(JsObjectSubtype),
    /// A JavaScript function
    Function,
    /// A JavaScript symbol
    Symbol,
}

impl JsRemoteObjectType {
    pub fn name(&self) -> &str {
        match self {
            Self::Object(..) => "object",
            Self::Function => "function",
            Self::Symbol => "symbol",
        }
    }
    pub fn is_object(&self) -> bool {
        matches!(self, JsRemoteObjectType::Object(_))
    }
    pub fn is_function(&self) -> bool {
        matches!(self, JsRemoteObjectType::Function)
    }
    pub fn is_symbol(&self) -> bool {
        matches!(self, JsRemoteObjectType::Symbol)
    }

    pub fn object_subtype(&self) -> Option<JsObjectSubtype> {
        if let JsRemoteObjectType::Object(subtype) = self {
            Some(*subtype)
        } else {
            None
        }
    }
}

/// Represents the subtype field of a Chrome DevTools Protocol RemoteObject.
/// 
/// This enum corresponds to the `subtype` field in CDP's RemoteObject type, which provides
/// more specific type information for JavaScript objects. It includes:
/// - Built-in JavaScript objects (Array, Date, RegExp)
/// - DOM objects (Node with associated IDs)
/// - Collection types (Map, Set, WeakMap, WeakSet)
/// - Special objects (Promise, Proxy, Error)
/// - Binary data objects (TypedArray, ArrayBuffer, DataView)
/// - WebAssembly-related objects
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "subtype")]
#[serde(rename_all = "lowercase")]
pub enum JsObjectSubtype {
    /// A JavaScript object that is not categorized into any specific subtype by CDP.
    /// This includes complex objects like window, Location, History, and other objects
    /// that don't fall into CDP's predefined subtypes.
    None,

    /// A JavaScript Array object
    Array,
    
    /// A DOM Node object with CDP-specific identifiers
    /// 
    /// Contains two types of node IDs used by CDP:
    /// - `node_id`: ID for use with the DOM domain
    /// - `backend_node_id`: Persistent backend node ID
    Node {
        #[serde(rename = "nodeId")]
        node_id: Option<i64>,
        #[serde(rename = "backendNodeId")]
        backend_node_id: i64,
    },

    /// A JavaScript RegExp (Regular Expression) object
    RegExp,

    /// A JavaScript Date object
    Date,

    /// A JavaScript Map collection
    Map,

    /// A JavaScript Set collection
    Set,

    /// A JavaScript WeakMap collection
    WeakMap,

    /// A JavaScript WeakSet collection
    WeakSet,

    /// A JavaScript Iterator object
    Iterator,

    /// A JavaScript Generator object
    Generator,

    /// A JavaScript Error object or its subclasses
    Error,

    /// A JavaScript Proxy object
    Proxy,

    /// A JavaScript Promise object
    Promise,

    /// A JavaScript TypedArray (Int8Array, Uint8Array, etc.)
    TypedArray,

    /// A JavaScript ArrayBuffer object
    ArrayBuffer,

    /// A JavaScript DataView object
    DataView,

    /// A WebAssembly.Memory object
    WebAssemblyMemory,

    /// A WebAssembly value type
    WasmValue,
}

impl JsObjectSubtype {
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Array => "array",
            Self::Node { .. } => "node",
            Self::RegExp => "regexp",
            Self::Date => "date",
            Self::Map => "map",
            Self::Set => "set",
            Self::WeakMap => "weakmap",
            Self::WeakSet => "weakset",
            Self::Iterator => "iterator",
            Self::Generator => "generator",
            Self::Error => "error",
            Self::Proxy => "proxy",
            Self::Promise => "promise",
            Self::TypedArray => "typedarray",
            Self::ArrayBuffer => "arraybuffer",
            Self::DataView => "dataview",
            Self::WebAssemblyMemory => "wasmmemory",
            Self::WasmValue => "wasmvalue",
        }
    }
    pub fn is_none(&self) -> bool {
        matches!(self, JsObjectSubtype::None)
    }
    pub fn is_array(&self) -> bool {
        matches!(self, JsObjectSubtype::Array)
    }
    pub fn is_node(&self) -> bool {
        matches!(self, JsObjectSubtype::Node { .. })
    }
    pub fn is_reg_exp(&self) -> bool {
        matches!(self, JsObjectSubtype::RegExp)
    }
    pub fn is_date(&self) -> bool {
        matches!(self, JsObjectSubtype::Date)
    }
    pub fn is_map(&self) -> bool {
        matches!(self, JsObjectSubtype::Map)
    }
    pub fn is_set(&self) -> bool {
        matches!(self, JsObjectSubtype::Set)
    }
    pub fn is_weak_map(&self) -> bool {
        matches!(self, JsObjectSubtype::WeakMap)
    }
    pub fn is_weak_set(&self) -> bool {
        matches!(self, JsObjectSubtype::WeakSet)
    }
    pub fn is_iterator(&self) -> bool {
        matches!(self, JsObjectSubtype::Iterator)
    }
    pub fn is_generator(&self) -> bool {
        matches!(self, JsObjectSubtype::Generator)
    }
    pub fn is_error(&self) -> bool {
        matches!(self, JsObjectSubtype::Error)
    }
    pub fn is_proxy(&self) -> bool {
        matches!(self, JsObjectSubtype::Proxy)
    }
    pub fn is_promise(&self) -> bool {
        matches!(self, JsObjectSubtype::Promise)
    }
    pub fn is_typed_array(&self) -> bool {
        matches!(self, JsObjectSubtype::TypedArray)
    }
    pub fn is_array_buffer(&self) -> bool {
        matches!(self, JsObjectSubtype::ArrayBuffer)
    }
    pub fn is_data_view(&self) -> bool {
        matches!(self, JsObjectSubtype::DataView)
    }
    pub fn is_wasm_memory(&self) -> bool {
        matches!(self, JsObjectSubtype::WebAssemblyMemory)
    }
    pub fn is_wasm_value(&self) -> bool {
        matches!(self, JsObjectSubtype::WasmValue)
    }
}


