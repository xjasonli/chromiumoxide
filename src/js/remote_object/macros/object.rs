macro_rules! remote_object {
    ($($rules:tt)*) => {
        remote_object_parse_attr!{
            remote_object_parse_outer(
                remote_object_gen_impl(
                    remote_object_parse_property(),
                ),
            ),
            $($rules)*
        }
    };
}

macro_rules! remote_object_parse_attr {
    (
        $callback:ident($($forward:tt)*),
        $($rest:tt)*
    ) => {
        remote_object_parse_attr!{@step
            #callback $callback($($forward)*),
            #attr [],
            #type [],
            #subtype [],
            #class [],
            $($rest)*
        }
    };
    
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [],
        #subtype [$($subtype:expr)?],
        #class [$($class:expr)?],
        #[type = $type:expr]
        $($rest:tt)*
    ) => {
        remote_object_parse_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)*],
            #type [$type],
            #subtype [$($subtype)?],
            #class [$($class)?],
            $($rest)*
        }
    };

    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [],
        #class [$($class:expr)?],
        #[subtype = $subtype:expr]
        $($rest:tt)*
    ) => {
        remote_object_parse_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)*],
            #type [$($type)?],
            #subtype [$subtype],
            #class [$($class)?],
            $($rest)*
        }
    };

    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [$($subtype:expr)?],
        #class [],
        #[class = $class:expr]
        $($rest:tt)*
    ) => {
        remote_object_parse_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)*],
            #type [$($type)?],
            #subtype [$($subtype)?],
            #class [$class],
            $($rest)*
        }
    };

    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [$($subtype:expr)?],
        #class [$($class:expr)?],
        #[$meta:meta]
        $($rest:tt)*
    ) => {
        remote_object_parse_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)* #[$meta]],
            #type [$($type)?],
            #subtype [$($subtype)?],
            #class [$($class)?],
            $($rest)*
        }
    };

    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [$($subtype:expr)?],
        #class [$($class:expr)?],
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #attr [$($attr)*],
            #type [$($type)?],
            #subtype [$($subtype)?],
            #class [$($class)?],
            $($rest)*
        }
    };
}

macro_rules! remote_object_parse_outer {
    (
        $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [$($subtype:expr)?],
        #class [$($class:expr)?],
        $($rest:tt)*
    ) => {
        remote_object_parse_outer!{@parse
            #callback $callback(
                $($forward)*
                #attr [$($attr)*],
                #type [$($type)?],
                #subtype [$($subtype)?],
                #class [$($class)?],
            ),
            $($rest)*
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        class $self:ident {
            $($body:tt)*
        }
    ) => {
        remote_object_parse_outer!{@format
            #callback $callback($($forward)*),
            #self $self,
            #parent RemoteObject,
            #ancestors [],
            #body { $($body)* }
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        class $self:ident
        extends $parent:ident {
            $($body:tt)*
        }
    ) => {
        remote_object_parse_outer!{@format
            #callback $callback($($forward)*),
            #self $self,
            #parent $parent,
            #ancestors [RemoteObject],
            #body { $($body)* }
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        class $self:ident
        extends $parent:ident
        inherits $($ancestor:ident),+ {
            $($body:tt)*
        }
    ) => {
        remote_object_parse_outer!{@format
            #callback $callback($($forward)*),
            #self $self,
            #parent $parent,
            #ancestors [$($ancestor,)+ RemoteObject],
            #body { $($body)* }
        }
    };

    (@format
        #callback $callback:ident($($forward:tt)*),
        #self $self:ident,
        #parent $parent:ident,
        #ancestors [$($ancestor:ident),*],
        #body { $($body:tt)* }
    ) => {
        paste::paste! {
            $callback!{
                $($forward)*
                #self [< Js $self >],
                #parent [< Js $parent >],
                #ancestors [$([< Js $ancestor >]),*],
                #body { $($body)* }
            }
        }
    };
}

macro_rules! remote_object_gen_impl {
    (
        $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #type [$($type:expr)?],
        #subtype [$($subtype:expr)?],
        #class [$($class:expr)?],
        #self $self:ident,
        #parent $parent:ident,
        #ancestors [$($ancestor:ident),*],
        #body { $($body:tt)* }
    ) => {
        $($attr)*
        #[derive(Debug, Clone)]
        pub struct $self($parent);

        impl $self {
            pub fn is_instance_of<T: Subclass<$self>>(&self) -> bool {
                T::is_instance(self)
            }

            pub fn downcast<T: Subclass<$self>>(&self) -> Option<<T as Class<$self>>::Owned> {
                T::try_from_super(self.clone())
            }
        }

        // implement Deref<Target = Parent> for Self
        impl Deref for $self {
            type Target = $parent;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // implement Class<Self> for Self
        impl private::Sealed for $self {}
        impl Class<$self> for $self {
            type Owned = Self;
            fn as_ref(&self) -> &$self {
                self
            }
        }

        // implement Class<Parent> for Self
        impl Class<$parent> for $self {
            type Owned = Self;
            fn as_ref(&self) -> &$parent {
                &self.0
            }
        }
        impl From<$self> for $parent {
            fn from(value: $self) -> Self {
                value.0
            }
        }

        // implement Subclass<Parent> for Self
        impl Subclass<$parent> for $self {
            fn is_instance(value: &$parent) -> bool {
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

            fn from_super(value: $parent) -> <Self as Class<$parent>>::Owned {
                Self(value)
            }
        }

        impl TryFrom<$parent> for $self {
            type Error = $parent;
            fn try_from(value: $parent) -> Result<Self, Self::Error> {
                Self::try_from_super(value.clone())
                    .ok_or(value)
            }
        }

        $(
            // implement Class<Ancestor> for Self
            impl Class<$ancestor> for $self {
                type Owned = Self;
                fn as_ref(&self) -> &$ancestor {
                    &self.0
                }
            }
            impl From<$self> for $ancestor {
                fn from(value: $self) -> Self {
                    $parent::from(value).into()
                }
            }

            // implement Subclass<Ancestor> for Self
            impl Subclass<$ancestor> for $self {
                fn is_instance_of(value: &$ancestor) -> bool {
                    $parent::is_instance(value)
                }
                fn from_super(value: $ancestor) -> <Self as Class<$ancestor>>::Owned {
                    Self($parent::from_super(value))
                }
            }
            impl TryFrom<$ancestor> for $self {
                type Error = $ancestor;
                fn try_from(value: $ancestor) -> Result<Self, Self::Error> {
                    Self::try_from_super(value.clone())
                        .ok_or(value)
                }
            }
        )*

        // implement Serialize and Deserialize for Self
        impl serde::Serialize for $self {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $self {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<$self, D::Error> {
                let parent = $parent::deserialize(deserializer)?;
                let this = $self::try_from(parent)
                    .map_err(|_| 
                        D::Error::custom(format!("Failed to convert {} to {}", stringify!($parent), stringify!($self)))
                    )?;
                Ok(this)
            }
        }

        // implement JsonSchema for Self
        impl schemars::JsonSchema for $self {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($self))
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(::core::concat!(
                    ::core::module_path!(),
                    "::",
                    stringify!($self)
                ))
            }
            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                #[allow(unused_mut)]
                let mut parent_schema = $parent::json_schema(generator);

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

        // implmenet properties
        impl $self {
            $callback!{
                $($forward)*
                $($body)*
            }
        }
    };
}

pub(crate) use {
    remote_object_parse_attr,
    remote_object_parse_outer,
    remote_object_gen_impl,
};
