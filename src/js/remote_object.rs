use std::{ops::Deref, sync::Arc};
use serde::de::Error as _;
use chromiumoxide_cdp::cdp::js_protocol::runtime::{ReleaseObjectParams, RemoteObjectId};
use crate::{error::Result, handler::PageInner, js::de::PageDeserializer};
use super::*;


mod object;
mod function;
mod symbol;

pub use object::*;
pub use function::*;
pub use symbol::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsRemoteObject(Arc<JsRemoteObjectInner>);

impl JsRemoteObject {
    pub fn object_id(&self) -> RemoteObjectId {
        self.0.data.id.clone()
    }
    pub fn object_type(&self) -> &JsRemoteObjectType {
        &self.0.data.r#type
    }
    pub fn object_class(&self) -> &str {
        &self.0.data.class
    }

    pub fn is<T: Subclass<Self>>(&self) -> bool {
        T::is_instance(self)
    }

    pub fn downcast<T: Subclass<Self>>(&self) -> Option<<T as Class<Self>>::Owned> {
        T::try_from_super(self.clone())
    }

    pub async fn eval<T: NativeValueFromJs>(
        &self,
        expr: impl Into<String>,
        options: EvalOptions
    ) -> Result<T> {
        let params = EvalParams::new(expr)
            .with_this(Some(self))
            .with_options(options);

        self.page().eval(params).await
    }

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

    pub fn invoke_method(&self, name: impl Into<String>, options: EvalOptions) -> FunctionInvoker {
        let expr = format!("this['{}']", name.into());
        self.invoke_function(expr, options)
    }

    pub async fn get_property<T>(&self, name: impl Into<String>) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        let expr = format!("this['{}']", name.into());
        self.eval(expr, EvalOptions::default()).await
    }

    pub async fn set_property<T>(&self, name: impl Into<String>, value: T) -> Result<()>
    where
        T: NativeValueIntoJs,
    {
        let function = format!("(value) => {{ this['{}'] = value; }}", name.into());
        self.invoke_function(function, EvalOptions::default())
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
        value.object_id()
    }
}

impl From<&JsRemoteObject> for RemoteObjectId {
    fn from(value: &JsRemoteObject) -> Self {
        value.object_id()
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
        std::borrow::Cow::Borrowed("JsRemoteValue")
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(std::concat!(
            ::core::module_path!(),
            "::",
            "JsRemoteValue"
        ))
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        generator.subschema_for::<helper::JsRemote>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JsRemoteObjectType {
    Object(Option<JsObjectSubtype>),
    Function,
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
            *subtype
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsObjectTypeInfo {
    #[serde(flatten)]
    pub(crate) subtype: Option<JsObjectSubtype>,
    pub(crate) class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsFunctionTypeInfo {
    pub(crate) class: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "subtype")]
#[serde(rename_all = "lowercase")]
pub enum JsObjectSubtype {
    Array,
    Node {
        #[serde(rename = "nodeId")]
        node_id: i64,
        #[serde(rename = "backendNodeId")]
        backend_node_id: i64,
    },
    RegExp,
    Date,
    Map,
    Set,
    WeakMap,
    WeakSet,
    Iterator,
    Generator,
    Error,
    Proxy,
    Promise,
    TypedArray,
    ArrayBuffer,
    DataView,
    WebAssemblyMemory,
    WasmValue,
}

impl JsObjectSubtype {
    pub fn name(&self) -> &'static str {
        match self {
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


macro_rules! example {
    (
        $(#[$attrss:meta])*
        $v:vis fn hello();
    ) => {
        $(#[$attrss])*
        $v fn hello() {
            println!("Hello, World!");
        }
    };
}

example! {
    /// Says hello.
    /// vsdf
    pub fn hello();
}



macro_rules! define_js_properties {
    ({
        $($rules:tt)*
    }) => {
        define_js_properties!(@parse $($rules)*);
    };

    (@parse $($rules:tt)+) => {
        define_js_properties!(@header $($rules)+);
    };

    (@parse) => {};

    (@header
        $(#[doc = $doc:expr])*
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_properties!(@entry
                $(#[doc = $doc])*
                $name[[< $name:snake >]]
                $($rest)*
            );
        }
    };
    (@header
        $(#[doc = $doc:expr])*
        #[rename = $rename:ident $(+ $suffix:ident)?]
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_properties!(@entry
                $(#[doc = $doc])*
                $name[[< $rename:snake $( _ $suffix:snake)? >]]
                $($rest)*
            );
        }
    };
    (@header
        $(#[doc = $doc:expr])*
        #[rename = + $suffix:ident]
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_properties!(@entry
                $(#[doc = $doc])*
                $name[[< $name:snake _ $suffix:snake >]]
                $($rest)*
            );
        }
    };

    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty [readonly];
        $($rest:tt)*
    ) => {
        define_js_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty,);
        define_js_properties!(@parse $($rest)*);
    };
    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty [readonly] {
            $(get() {
                $($getter:tt)+
            })?
        }
        $($rest:tt)*
    ) => {
        define_js_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty, $($($getter)+)?);
        define_js_properties!(@parse $($rest)*);
    };

    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty;
        $($rest:tt)*
    ) => {
        define_js_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty,);
        define_js_properties!(@setter $(#[doc = $doc])* $name[$rename]: $ty,);
        define_js_properties!(@parse $($rest)*);
    };
    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty {
            $(get() {
                $($getter:tt)+
            })?
            $(set($var:ident) {
                $($setter:tt)+
            })?
        }
        $($rest:tt)*
    ) => {
        define_js_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty, $($($getter)+)?);
        define_js_properties!(@setter $(#[doc = $doc])* $name[$rename]: $ty, $($var, $($setter)+)?);
        define_js_properties!(@parse $($rest)*);
    };

    (@getter
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty,
    ) => {
        $(#[doc = $doc])*
        pub async fn $rename(&self) -> Result<$ty> {
            self.get_property(stringify!($name)).await
        }
    };
    (@getter
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty,
        $($js:tt)+
    ) => {
        $(#[doc = $doc])*
        pub async fn $rename(&self) -> Result<$ty> {
            const JS: &str = concat!(
                "() => {",
                    stringify!($($js)+),
                "}",
            );
            self.invoke_function(JS, EvalOptions::default())
                .invoke().await
        }
    };
    (@setter
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty,
    ) => {
        paste::paste! {
            $(#[doc = $doc])*
            pub async fn [< set_ $rename >](&self, value: $ty) -> Result<()> {
                self.set_property(stringify!($name), value).await
            }
        }
    };
    (@setter
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty,
        $var:ident,
        $($js:tt)+
    ) => {
        paste::paste! {
            $(#[doc = $doc])*
            pub async fn [< set_ $rename >](&self, [< $var:snake >]: $ty) -> Result<()> {
                const JS: &str = concat!(
                    "(", stringify!($var), ") => {",
                        stringify!($($js)+),
                    "}",
                );
                self.invoke_function(JS, EvalOptions::default())
                    .argument([< $var:snake >])?
                    .invoke().await
            }
        }
    };
}

macro_rules! define_js_methods {
    ({
        $($t:tt)*
    }) => {
        define_js_methods!(@parse $($t)*);
    };

    (@parse $($t:tt)+) => {
        define_js_methods!(@header $($t)+);
    };

    (@parse) => {};

    (@header
        $(#[doc = $doc:expr])*
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_methods!(@entry
                $(#[doc = $doc])*
                $name[[< $name:snake >]]
                $($rest)*
            );
        }
    };
    (@header
        $(#[doc = $doc:expr])*
        #[rename = $rename:ident $(+ $suffix:ident)?]
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_methods!(@entry
                $(#[doc = $doc])*
                $name[[< $rename:snake $( _ $suffix:snake)? >]]
                $($rest)*
            );
        }
    };
    (@header
        $(#[doc = $doc:expr])*
        #[rename = + $suffix:ident]
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            define_js_methods!(@entry
                $(#[doc = $doc])*
                $name[[< $name:snake _ $suffix:snake >]]
                $($rest)*
            );
        }
    };

    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]
        $(< $( $lt:tt $( : $clt:ty )? ),+ >)?
        (
            $(
                $(... $($spread_marker:block)?)?
                $arg:ident $(? $($optional:block)?)? : $arg_type:ty
            ),* $(,)?
        ) -> $ty:ty
        $(where
            $( $lt2:tt : $clt2:ty ),+
        )?
        ;
        $($rest:tt)*
    ) => {
        paste::paste! {
            $(#[doc = $doc])*
            pub async fn $rename$(< $( $lt $( : $clt )? ),+ >)?(
                &self,
                $(
                    [< $arg:snake >]:
                    $($($optional)? Optional<)?
                        $arg_type
                    $($($optional)? >)?
                    ,
                )*
            ) -> Result<$ty>
            $(where
                $( $lt2: $clt2 ),+
            )?
            {
                let invoker = self.invoke_method(stringify!($name), EvalOptions::default());
                $( define_js_methods!(@argument invoker $(... $($spread_marker)?)? [< $arg:snake >]); )*
                invoker.invoke().await
            }
        }
        define_js_methods!(@parse $($rest)*);
    };
    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]
        $(< $( $lt:tt $( : $clt:ty )? ),+ >)?
        (
            $(
                $(... $($spread_marker:block)?)?
                $arg:ident $(? $($optional:block)?)? : $arg_type:ty
            ),* $(,)?
        ) -> $ty:ty
        $(where
            $( $lt2:tt : $clt2:ty ),+
        )?
        {
            $($js:tt)*
        }
        $($rest:tt)*
    ) => {
        paste::paste! {
            $(#[doc = $doc])*
            pub async fn $rename$(< $( $lt $( : $clt )? ),+ >)?(
                &self,
                $(
                    [< $arg:snake >]:
                    $($($optional)? Optional<)?
                        $arg_type
                    $($($optional)? >)?
                    ,
                )*
            ) -> Result<$ty>
            $(where
                $( $lt2: $clt2 ),+
            )?
            {
                const FUNCTION: &str = concat!(
                    "(", $(stringify!($arg),)* ") => {",
                        stringify!($($js)*),
                    "}",
                );
                let invoker = self.invoke_function(FUNCTION, EvalOptions::default());
                $( define_js_methods!(@argument invoker $(... $($spread_marker)?)? [< $arg:snake >]); )*
                invoker.invoke().await
            }
        }
        define_js_methods!(@parse $($rest)*);
    };

    (@argument $var:ident ... $arg:ident) => {
        let $var = $var.arguments_spread($arg)?;
    };
    (@argument $var:ident $arg:ident) => {
        let $var = $var.argument($arg)?;
    };
}

macro_rules! define_js_remote_object {
    (
        $(#[$meta:meta])*
        class $t:ident { $($body:tt)+ }
    ) => {
        define_js_remote_object!(
            @
            $(#[$meta])*
            class $t extends RemoteObject { $($body)+ }
        );
    };
    (
        $(#[$meta:meta])*
        class $t:ident extends $parent:ident { $($body:tt)+ }
    ) => {
        define_js_remote_object!(
            @
            $(#[$meta])*
            class $t extends $parent inherits RemoteObject { $($body)+ }
        );
    };
    (
        $(#[$meta:meta])*
        class $t:ident extends $parent:ident inherits $($ancestor:ident),+ { $($body:tt)+ }
    ) => {
        define_js_remote_object!(
            @
            $(#[$meta])*
            class $t extends $parent inherits $($ancestor,)+ RemoteObject { $($body)+ }
        );
    };

    (
        @
        $(#[$meta:meta])*
        class $t:ident
        extends $parent:ident
        $(inherits $($ancestor:ident),+)? {
            $(static #type: $type:literal;)?
            $(static #subtype: $subtype:expr;)?
            $(static #class: $class:tt;)?

            $(
                properties: $properties:tt
            )?
            $(
                methods: $methods:tt
            )?
        }
    ) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            $(#[$meta])*
            pub struct [< Js $t >]([< Js $parent >]);

            impl [< Js $t >] {
                pub fn is<T: Subclass<[< Js $t >]>>(&self) -> bool {
                    T::is_instance(self)
                }

                pub fn downcast<T: Subclass<[< Js $t >]>>(&self) -> Option<<T as Class<[< Js $t >]>>::Owned> {
                    T::try_from_super(self.clone())
                }
            }

            // implement Deref<Target = Parent> for Self
            impl Deref for [< Js $t >] {
                type Target = [< Js $parent >];
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            // implement Class<Self> for Self
            impl private::Sealed for [< Js $t >] {}
            impl Class<[< Js $t >]> for [< Js $t >] {
                type Owned = Self;
                fn as_ref(&self) -> &[< Js $t >] {
                    self
                }
            }

            // implement Class<Parent> for Self
            impl Class<[< Js $parent >]> for [< Js $t >] {
                type Owned = Self;
                fn as_ref(&self) -> &[< Js $parent >] {
                    &self.0
                }
            }
            impl From<[< Js $t >]> for [< Js $parent >] {
                fn from(value: [< Js $t >]) -> Self {
                    value.0
                }
            }

            // implement Subclass<Parent> for Self
            impl Subclass<[< Js $parent >]> for [< Js $t >] {
                fn is_instance(value: &[< Js $parent >]) -> bool {
                    $(
                        if value.object_type().name() != $type {
                            return false;
                        }
                    )?
                    $(
                        let subtype = value.object_type()
                            .object_subtype()
                            .map(|subtype| subtype.name());
                        if !helper::SubtypePattern::matches($subtype, subtype) {
                            return false;
                        }
                    )?
                    $(
                        let class = value.object_class();
                        if !helper::ClassPattern::matches($class, class) {
                            return false;
                        }
                    )?
                    true
                }

                fn from_super(value: [< Js $parent >]) -> <Self as Class<[< Js $parent >]>>::Owned {
                    Self(value)
                }
            }

            impl TryFrom<[< Js $parent >]> for [< Js $t >] {
                type Error = [< Js $parent >];
                fn try_from(value: [< Js $parent >]) -> Result<Self, Self::Error> {
                    Self::try_from_super(value.clone())
                        .ok_or(value)
                }
            }

            $($(
                // implement Class<Ancestor> for Self
                impl Class<[< Js $ancestor >]> for [< Js $t >] {
                    type Owned = Self;
                    fn as_ref(&self) -> &[< Js $ancestor >] {
                        &self.0
                    }
                }
                impl From<[< Js $t >]> for [< Js $ancestor >] {
                    fn from(value: [< Js $t >]) -> Self {
                        [< Js $parent >]::from(value).into()
                    }
                }

                // implement Subclass<Ancestor> for Self
                impl Subclass<[< Js $ancestor >]> for [< Js $t >] {
                    fn is_instance(value: &[< Js $ancestor >]) -> bool {
                        [< Js $parent >]::is_instance(value)
                    }
                    fn from_super(value: [< Js $ancestor >]) -> <Self as Class<[< Js $ancestor >]>>::Owned {
                        Self([< Js $parent >]::from_super(value))
                    }
                }
                impl TryFrom<[< Js $ancestor >]> for [< Js $t >] {
                    type Error = [< Js $ancestor >];
                    fn try_from(value: [< Js $ancestor >]) -> Result<Self, Self::Error> {
                        Self::try_from_super(value.clone())
                            .ok_or(value)
                    }
                }
            )+)?

            // implement Serialize and Deserialize for Self
            impl serde::Serialize for [< Js $t >] {
                fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    self.0.serialize(serializer)
                }
            }
            impl<'de> serde::Deserialize<'de> for [< Js $t >] {
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let parent = [< Js $parent >]::deserialize(deserializer)?;
                    let this = Self::try_from(parent)
                        .map_err(|_| 
                            D::Error::custom(format!("Failed to convert {} to {}", stringify!([< Js $parent>]), stringify!([< Js $t >])))
                        )?;
                    Ok(this)
                }
            }

            // implement JsonSchema for Self
            impl schemars::JsonSchema for [< Js $t >] {
                fn schema_name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(stringify!([< Js $t >]))
                }
                fn schema_id() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(::core::concat!(
                        ::core::module_path!(),
                        "::",
                        stringify!([< Js $t >])
                    ))
                }
                fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                    #[allow(unused_mut)]
                    let mut parent_schema = [< Js $parent >]::json_schema(generator);

                    #[allow(unused_mut)]
                    let mut properties = parent_schema.ensure_object()
                        ["properties"]
                        [helper::JS_REMOTE_KEY]
                        ["properties"]
                        .as_object_mut()
                        .unwrap();
                    $(
                        properties["type"]["enum"] = serde_json::json!([$type]);
                    )?
                    $(
                        properties["subtype"] = helper::SubtypePattern::to_schema($subtype);
                    )?

                    let remove_node_id = {
                        if properties["type"]["enum"].as_array_mut()
                            .unwrap()
                            .contains(&serde_json::json!("object")) {
                            if let Some(subtype) = properties["subtype"]["enum"].as_array() {
                                !subtype.contains(&serde_json::json!("node"))
                            } else {
                                true
                            }
                        } else {
                            false
                        }
                    };
                    if remove_node_id {
                        properties.remove("nodeId");
                        properties.remove("backendNodeId");
                    }
                    parent_schema
                }
            }

            

            // implement javascript properties
            $(
                impl [< Js $t >] {
                    define_js_properties!($properties);
                }
            )?

            // implement javascript methods
            $(
                impl [< Js $t >] {
                    define_js_methods!($methods);
                }
            )?
        }
    };
}

pub(crate) use define_js_remote_object;
pub(crate) use define_js_properties;
pub(crate) use define_js_methods;

