//! Types for representing JavaScript union types in Rust.
//!
//! This module provides the `AnyOf` type and related macros that allow representing
//! JavaScript values that can be one of several different types. This is similar to
//! TypeScript's union types (e.g., `string | number`).
//!
//! # Example
//! ```no_run
//! use chromiumoxide::js::AnyOf;
//!
//! // A value that can be either a string or a number
//! type StringOrNumber = AnyOf!(String, i32);
//!
//! // A value that can be one of three types
//! type ComplexType = AnyOf!(String, i32, bool);
//! ```

use std::borrow::Cow;
use frunk::{
    Coprod,
    coproduct::{
        CoproductSelector,
        CoproductTaker,
        CoprodUninjector,
        CoproductEmbedder,
        CoprodInjector,
    },
};
use super::*;

/// Macro for creating union types using the `AnyOf` type.
///
/// This macro provides a convenient way to create types that can hold values
/// of different types. It supports up to 20 different type parameters.
///
/// # Example
/// ```no_run
/// # use chromiumoxide::js::AnyOf;
/// // A value that can be either a string or a number
/// let value: AnyOf!(String, i32) = AnyOf::new("hello");
///
/// // Check and extract the value
/// if let Ok(string) = value.try_extract::<String, _, _>() {
///     println!("Got string: {}", string);
/// }
/// ```
#[macro_export]
macro_rules! AnyOf {
    ($t1:ty, $t2:ty) => {
        $crate::js::any::AnyOf2<$t1, $t2>
    };
    ($t1:ty, $t2:ty, $t3:ty) => {
        $crate::js::any::AnyOf3<$t1, $t2, $t3>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        $crate::js::any::AnyOf4<$t1, $t2, $t3, $t4>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        $crate::js::any::AnyOf5<$t1, $t2, $t3, $t4, $t5>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
        $crate::js::any::AnyOf6<$t1, $t2, $t3, $t4, $t5, $t6>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
        $crate::js::any::AnyOf7<$t1, $t2, $t3, $t4, $t5, $t6, $t7>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
        $crate::js::any::AnyOf8<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty) => {
        $crate::js::any::AnyOf9<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty) => {
        $crate::js::any::AnyOf10<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty) => {
        $crate::js::any::AnyOf11<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty) => {
        $crate::js::any::AnyOf12<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty) => {
        $crate::js::any::AnyOf13<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty) => {
        $crate::js::any::AnyOf14<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty) => {
        $crate::js::any::AnyOf15<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty) => {
        $crate::js::any::AnyOf16<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15, $t16>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty) => {
        $crate::js::any::AnyOf17<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15, $t16, $t17>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty) => {
        $crate::js::any::AnyOf18<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15, $t16, $t17, $t18>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty) => {
        $crate::js::any::AnyOf19<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15, $t16, $t17, $t18, $t19>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty) => {
        $crate::js::any::AnyOf20<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12, $t13, $t14, $t15, $t16, $t17, $t18, $t19, $t20>
    };
}

pub use AnyOf;

/// Internal macro for defining AnyOf types.
///
/// This macro generates the implementation of an AnyOf type with the specified
/// number of type parameters. It implements:
/// - Constructor and accessor methods
/// - Serialization and deserialization
/// - JSON schema generation
macro_rules! define_any_of {
    ($n:tt, $($t:ty),+) => {
        paste::paste! {
            type [< AnyOf $n Inner >]<$($t,)+> = Coprod!($($t),+);

            /// A type that can hold a value of one of several different types.
            ///
            /// This type is used to represent JavaScript values that can be one of
            /// multiple types, similar to TypeScript's union types.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct [< AnyOf $n >]<$($t,)+>([< AnyOf $n Inner >]<$($t,)+>);

            impl<$($t,)+> [< AnyOf $n >]<$($t,)+> {
                /// Creates a new instance containing the given value.
                pub fn new<T, I>(value: T) -> Self
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoprodInjector<T, I>,
                {
                    Self(<[< AnyOf $n Inner >]<$($t,)+>>::inject(value))
                }
                
                /// Gets a reference to the contained value if it is of type T.
                pub fn get<T, I>(&self) -> Option<&T>
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoproductSelector<T, I>,
                {
                    self.0.get::<T, I>()
                }

                /// Sets the contained value.
                pub fn set<T, I>(&mut self, value: T)
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoprodInjector<T, I>,
                {
                    self.0 = <[< AnyOf $n Inner >]<$($t,)+>>::inject(value);
                }

                /// Checks if the contained value is of type T.
                pub fn is<T, I>(&self) -> bool
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoproductSelector<T, I>,
                {
                    self.0.get::<T, I>().is_some()
                }

                /// Takes ownership of the contained value if it is of type T.
                pub fn take<T, I>(self) -> Option<T>
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoproductTaker<T, I>,
                {
                    self.0.take::<T, I>()
                }

                /// Attempts to extract a value of type T, returning the original
                /// value if extraction fails.
                pub fn try_extract<T, I, II>(self) -> Result<T, Self>
                where
                    [< AnyOf $n Inner >]<$($t,)+>: CoprodUninjector<T, I>,
                    <[< AnyOf $n Inner >]<$($t,)+> as CoprodUninjector<T, I>>::Remainder: CoproductEmbedder<[< AnyOf $n Inner >]<$($t,)+>, II>
                {
                    self.0.uninject::<T, I>()
                        .map_err(|remain| Self(remain.embed()))
                }
            }

            // Implement serialization
            impl<$($t,)+> serde::Serialize for [< AnyOf $n >]<$($t,)+>
            where
                $($t: serde::Serialize),+
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    $(
                        if let Some(v) = self.get::<$t, _>() {
                            return v.serialize(serializer);
                        }
                    )+
                    Err(serde::ser::Error::custom("no value"))
                }
            }

            // Implement deserialization
            impl<'de, $($t,)+> serde::Deserialize<'de> for [< AnyOf $n >]<$($t,)+>
            where
                $($t: serde::Deserialize<'de>),+
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use serde::de::IntoDeserializer as _;

                    fn de<'de, $($t,)+ F, D, E>(deserializer: F) -> Result<[< AnyOf $n >]<$($t,)+>, E>
                    where
                        F: Fn() -> D,
                        D: serde::Deserializer<'de>,
                        E: serde::de::Error,
                        $($t: serde::Deserialize<'de>),+
                    {
                        $(
                            if let Ok(v) = $t::deserialize(deserializer()) {
                                return Ok([< AnyOf $n >]::new(v));
                            }
                        )+
                        Err(E::custom(concat!("data did not match AnyOf", stringify!($n))))
                    }

                    let page = de::PageDeserializer::get(&deserializer);
                    let content = serde_content::Value::deserialize(deserializer)?;
                    if let Some(page) = page {
                        de(||
                            de::PageDeserializer::new(
                                content.clone()
                                    .into_deserializer()
                                    .human_readable(),
                                page.clone(),
                            )
                        )
                    } else {
                        de(||
                            content.clone()
                                .into_deserializer()
                                .human_readable()
                        )
                    }
                }
            }

            // Implement JSON schema generation
            impl<$($t,)+> schemars::JsonSchema for [< AnyOf $n >]<$($t,)+>
            where
                $($t: schemars::JsonSchema),+
            {
                fn schema_name() -> Cow<'static, str> {
                    format!(
                        "{}<{}>",
                        stringify!([< AnyOf $n >]),
                        vec![$($t::schema_name()),+].join(", "),
                    ).into()
                }
                fn schema_id() -> Cow<'static, str> {
                    format!("{}::{}", ::core::module_path!(), Self::schema_name(),)
                        .into()
                }
                fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                    let mut map = serde_json::Map::new();
                    map.insert(
                        "anyOf".into(),
                        serde_json::Value::Array(vec![
                            $(
                                generator.subschema_for::<$t>().to_value(),
                            )+
                        ]),
                    );
                    schemars::Schema::from(map)
                }
            }
        }
    };
}

// Generate AnyOf types for different numbers of type parameters
define_any_of!(2, T1, T2);
define_any_of!(3, T1, T2, T3);
define_any_of!(4, T1, T2, T3, T4);
define_any_of!(5, T1, T2, T3, T4, T5);
define_any_of!(6, T1, T2, T3, T4, T5, T6);
define_any_of!(7, T1, T2, T3, T4, T5, T6, T7);
define_any_of!(8, T1, T2, T3, T4, T5, T6, T7, T8);
define_any_of!(9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
define_any_of!(10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
define_any_of!(11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
define_any_of!(12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
define_any_of!(13, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
define_any_of!(14, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
define_any_of!(15, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
define_any_of!(16, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
define_any_of!(17, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
define_any_of!(18, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
define_any_of!(19, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
define_any_of!(20, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    fn test() {
        let any = <AnyOf!(i32, String)>::new(1);
        let x: Result<i32, AnyOf2<i32, String>> = any.try_extract::<i32, _, _>();
        println!("{:?}", x.unwrap());
    }
}
