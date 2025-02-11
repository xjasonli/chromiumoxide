
macro_rules! js_remote_object {
    (
        $(#[$meta:meta])*
        class $t:ident { $($body:tt)+ }
    ) => {
        js_remote_object!(
            @
            $(#[$meta])*
            class $t extends RemoteObject { $($body)+ }
        );
    };
    (
        $(#[$meta:meta])*
        class $t:ident extends $parent:ident { $($body:tt)+ }
    ) => {
        js_remote_object!(
            @
            $(#[$meta])*
            class $t extends $parent inherits RemoteObject { $($body)+ }
        );
    };
    (
        $(#[$meta:meta])*
        class $t:ident extends $parent:ident inherits $($ancestor:ident),+ { $($body:tt)+ }
    ) => {
        js_remote_object!(
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
            $(static #type: $type:expr;)?
            $(static #subtype: $subtype:expr;)?
            $(static #class: $class:expr;)?

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
                pub fn is_instance_of<T: Subclass<[< Js $t >]>>(&self) -> bool {
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
                        if value.remote_type().name() != $type {
                            return false;
                        }
                    )?
                    $(
                        let subtype = value.remote_type()
                            .object_subtype()
                            .map(|subtype| subtype.name());
                        if !helper::SubtypePattern::matches($subtype, subtype) {
                            return false;
                        }
                    )?
                    $(
                        let class = value.remote_class();
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
                    js_remote_object_properties!($properties);
                }
            )?

            // implement javascript methods
            $(
                impl [< Js $t >] {
                    js_remote_object_methods!($methods);
                }
            )?
        }
    };
}

macro_rules! js_remote_object_properties {
    ({
        $($rules:tt)*
    }) => {
        js_remote_object_properties!(@parse $($rules)*);
    };

    (@parse $($rules:tt)+) => {
        js_remote_object_properties!(@header $($rules)+);
    };

    (@parse) => {};

    (@header
        $(#[doc = $doc:expr])*
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            js_remote_object_properties!(@entry
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
            js_remote_object_properties!(@entry
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
            js_remote_object_properties!(@entry
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
        js_remote_object_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty,);
        js_remote_object_properties!(@parse $($rest)*);
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
        js_remote_object_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty, $($($getter)+)?);
        js_remote_object_properties!(@parse $($rest)*);
    };

    (@entry
        $(#[doc = $doc:expr])*
        $name:ident[$rename:ident]: $ty:ty;
        $($rest:tt)*
    ) => {
        js_remote_object_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty,);
        js_remote_object_properties!(@setter $(#[doc = $doc])* $name[$rename]: $ty,);
        js_remote_object_properties!(@parse $($rest)*);
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
        js_remote_object_properties!(@getter $(#[doc = $doc])* $name[$rename]: $ty, $($($getter)+)?);
        js_remote_object_properties!(@setter $(#[doc = $doc])* $name[$rename]: $ty, $($var, $($setter)+)?);
        js_remote_object_properties!(@parse $($rest)*);
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

macro_rules! js_remote_object_methods {
    ({
        $($t:tt)*
    }) => {
        js_remote_object_methods!(@parse $($t)*);
    };

    (@parse $($t:tt)+) => {
        js_remote_object_methods!(@header $($t)+);
    };

    (@parse) => {};

    (@header
        $(#[doc = $doc:expr])*
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste! {
            js_remote_object_methods!(@entry
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
            js_remote_object_methods!(@entry
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
            js_remote_object_methods!(@entry
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
                $( js_remote_object_methods!(@argument invoker $(... $($spread_marker)?)? [< $arg:snake >]); )*
                invoker.invoke().await
            }
        }
        js_remote_object_methods!(@parse $($rest)*);
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
                $( js_remote_object_methods!(@argument invoker $(... $($spread_marker)?)? [< $arg:snake >]); )*
                invoker.invoke().await
            }
        }
        js_remote_object_methods!(@parse $($rest)*);
    };

    (@argument $var:ident ... $arg:ident) => {
        let $var = $var.arguments_spread($arg)?;
    };
    (@argument $var:ident $arg:ident) => {
        let $var = $var.argument($arg)?;
    };
}

pub(in super) use js_remote_object;
pub(in super) use js_remote_object_properties;
pub(in super) use js_remote_object_methods;
