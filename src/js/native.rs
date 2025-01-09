use std::sync::Arc;
use chromiumoxide_cdp::cdp::js_protocol::runtime::RemoteObjectId;
use serde_json::Value as JsonValue;

pub use schemars::JsonSchema;
pub use serde::{Serialize, Deserialize};

use super::object::JsObject;
use crate::handler::PageInner;


pub trait NativeValueIntoJs: Serialize {}
impl<T: Serialize> NativeValueIntoJs for T {}

pub trait NativeValueFromJs: serde::de::DeserializeOwned + JsonSchema {}
impl<T: serde::de::DeserializeOwned + JsonSchema> NativeValueFromJs for T {}

pub trait FunctionNativeArgs {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error>;
}
impl FunctionNativeArgs for () {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![])
    }
}
impl<A1: NativeValueIntoJs> FunctionNativeArgs for (A1,) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs> FunctionNativeArgs for (A1, A2) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs, A6: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs, A6: NativeValueIntoJs, A7: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs, A6: NativeValueIntoJs, A7: NativeValueIntoJs, A8: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs, A6: NativeValueIntoJs, A7: NativeValueIntoJs, A8: NativeValueIntoJs, A9: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?, serde_json::to_value(self.8)?])
    }
}
impl<A1: NativeValueIntoJs, A2: NativeValueIntoJs, A3: NativeValueIntoJs, A4: NativeValueIntoJs, A5: NativeValueIntoJs, A6: NativeValueIntoJs, A7: NativeValueIntoJs, A8: NativeValueIntoJs, A9: NativeValueIntoJs, A10: NativeValueIntoJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?, serde_json::to_value(self.8)?, serde_json::to_value(self.9)?])
    }
}

pub trait CallbackNativeArgs {}
impl CallbackNativeArgs for () {}
impl<A1: NativeValueFromJs> CallbackNativeArgs for (A1,) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs> CallbackNativeArgs for (A1, A2) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs, A6: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5, A6) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs, A6: NativeValueFromJs, A7: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5, A6, A7) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs, A6: NativeValueFromJs, A7: NativeValueFromJs, A8: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs, A6: NativeValueFromJs, A7: NativeValueFromJs, A8: NativeValueFromJs, A9: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9) {}
impl<A1: NativeValueFromJs, A2: NativeValueFromJs, A3: NativeValueFromJs, A4: NativeValueFromJs, A5: NativeValueFromJs, A6: NativeValueFromJs, A7: NativeValueFromJs, A8: NativeValueFromJs, A9: NativeValueFromJs, A10: NativeValueFromJs> CallbackNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) {}


pub(super) struct PageSeed<T> {
    page: Arc<PageInner>,
    inner: T,
}

impl<T> PageSeed<T> {
    pub(crate) fn new(page: Arc<PageInner>, inner: T) -> Self {
        Self { page, inner }
    }
}

impl<'de, T: serde::de::DeserializeSeed<'de>> serde::de::DeserializeSeed<'de> for PageSeed<T> {
    type Value = T::Value;

    fn deserialize<D: serde::de::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let deserializer = PageDeserializer {
            inner: deserializer,
            page: self.page.clone(),
        };
        if deserializer.is_js_object::<Self::Value>() {
            deserializer.deserialize_js_object()
        } else {
            self.inner.deserialize(deserializer)
        }
    }
}

struct PageDeserializer<D> {
    inner: D,
    page: Arc<PageInner>,
}

impl<'de, D: serde::de::Deserializer<'de>> PageDeserializer<D> {
    fn is_js_object<T>(&self) -> bool {
        try_specialize::type_eq::<T, JsObject>()
    }

    fn deserialize_js_object<T>(self) -> Result<T, D::Error> {
        assert!(self.is_js_object::<T>());

        let proxy = super::object::JsObjectSerdeProxy::deserialize(self.inner)?;
        let object_id: RemoteObjectId = proxy.into_inner().into();

        let js_object = JsObject::new(object_id, self.page.clone());
        Ok(try_specialize::TrySpecialize::try_specialize_from(js_object).unwrap())
    }
}

impl<'de, D: serde::de::Deserializer<'de>> serde::de::Deserializer<'de> for PageDeserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_any(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_bool<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_bool(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_i8<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_i8(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_i16<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_i16(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_i32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_i32(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_i64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_i64(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_i128<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_i128(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_u8<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_u8(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_u16<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_u16(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_u32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_u32(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_u64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_u64(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_u128<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_u128(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_f32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_f32(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_f64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_f64(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_char<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_char(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_str(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_string<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_string(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_bytes(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_byte_buf<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_byte_buf(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_option<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_option(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_unit<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_unit(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_unit_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_unit_struct(name, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_newtype_struct(name, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_seq<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_seq(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_tuple<V: serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_tuple(len, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_tuple_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_tuple_struct(name, len, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_map<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_map(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_struct(name, fields, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_enum<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_enum(name, variants, PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_identifier<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_identifier(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }

    fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_js_object::<V::Value>() {
            self.deserialize_js_object()
        } else {
            self.inner.deserialize_ignored_any(PageVisitor {
                inner: visitor,
                page: self.page,
            })
        }
    }
}


struct PageVisitor<V> {
    inner: V,
    page: Arc<PageInner>,
}

impl<'de, V: serde::de::Visitor<'de>> serde::de::Visitor<'de> for PageVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_i64(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_u64(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_string(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_byte_buf(v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: serde::de::Deserializer<'de> {
        let deserializer = PageDeserializer {
            inner: deserializer,
            page: self.page.clone(),
        };
        self.inner.visit_some(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where E: serde::de::Error {
        self.inner.visit_unit()
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where A: serde::de::SeqAccess<'de> {
        struct SeqAccess<A> {
            inner: A,
            page: Arc<PageInner>,
        }
        impl<'de, A: serde::de::SeqAccess<'de>> serde::de::SeqAccess<'de> for SeqAccess<A> {
            type Error = A::Error;

            fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
                let seed = PageSeed::new(self.page.clone(), seed);
                self.inner.next_element_seed(seed)
            }

            fn size_hint(&self) -> Option<usize> {
                self.inner.size_hint()
            }
        }

        self.inner.visit_seq(SeqAccess {
            inner: seq,
            page: self.page.clone(),
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where A: serde::de::MapAccess<'de> {
        struct MapAccess<A> {
            inner: A,
            page: Arc<PageInner>,
        }
        impl<'de, A: serde::de::MapAccess<'de>> serde::de::MapAccess<'de> for MapAccess<A> {
            type Error = A::Error;

            fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
                self.inner.next_key_seed(seed)
            }
            fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
                let seed = PageSeed::new(self.page.clone(), seed);
                self.inner.next_value_seed(seed)
            }
            fn size_hint(&self) -> Option<usize> {
                self.inner.size_hint()
            }
        }
        self.inner.visit_map(MapAccess {
            inner: map,
            page: self.page.clone(),
        })
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where A: serde::de::EnumAccess<'de> {
        struct VariantAccess<A> {
            inner: A,
            page: Arc<PageInner>,
        }
        impl<'de, A: serde::de::VariantAccess<'de>> serde::de::VariantAccess<'de> for VariantAccess<A> {
            type Error = A::Error;

            fn unit_variant(self) -> Result<(), Self::Error> {
                self.inner.unit_variant()
            }

            fn newtype_variant_seed<T: serde::de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
                let seed = PageSeed::new(self.page.clone(), seed);
                self.inner.newtype_variant_seed(seed)
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where V: serde::de::Visitor<'de> {
                self.inner.tuple_variant(len, PageVisitor {
                    inner: visitor,
                    page: self.page.clone(),
                })
            }

            fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
            where V: serde::de::Visitor<'de> {
                self.inner.struct_variant(fields, PageVisitor {
                    inner: visitor,
                    page: self.page.clone(),
                })
            }
        }

        struct EnumAccess<A> {
            inner: A,
            page: Arc<PageInner>,
        }
        impl<'de, A: serde::de::EnumAccess<'de>> serde::de::EnumAccess<'de> for EnumAccess<A> {
            type Error = A::Error;
            type Variant = VariantAccess<A::Variant>;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where V: serde::de::DeserializeSeed<'de> {
                let seed = PageSeed::new(self.page.clone(), seed);
                let (value, variant) = self.inner.variant_seed(seed)?;
                Ok((
                    value,
                    VariantAccess {
                        inner: variant,
                        page: self.page.clone(),
                    },
                ))
            }
        }

        self.inner.visit_enum(EnumAccess {
            inner: data,
            page: self.page.clone(),
        })
    }
}

unsafe impl try_specialize::LifetimeFree for JsObject {}
