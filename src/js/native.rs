use std::sync::Arc;
use chromiumoxide_cdp::cdp::js_protocol::runtime::RemoteObjectId;
use serde_json::Value as JsonValue;

pub use schemars::JsonSchema;
pub use serde::{Serialize, Deserialize};

use super::object::JsObject;
use crate::handler::PageInner;


pub trait FunctionNativeArgs {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error>;
}
impl FunctionNativeArgs for () {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![])
    }
}
impl<A1: NativeValueToJs> FunctionNativeArgs for (A1,) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs> FunctionNativeArgs for (A1, A2) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs, A6: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs, A6: NativeValueToJs, A7: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs, A6: NativeValueToJs, A7: NativeValueToJs, A8: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs, A6: NativeValueToJs, A7: NativeValueToJs, A8: NativeValueToJs, A9: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?, serde_json::to_value(self.8)?])
    }
}
impl<A1: NativeValueToJs, A2: NativeValueToJs, A3: NativeValueToJs, A4: NativeValueToJs, A5: NativeValueToJs, A6: NativeValueToJs, A7: NativeValueToJs, A8: NativeValueToJs, A9: NativeValueToJs, A10: NativeValueToJs> FunctionNativeArgs for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
        Ok(vec![serde_json::to_value(self.0)?, serde_json::to_value(self.1)?, serde_json::to_value(self.2)?, serde_json::to_value(self.3)?, serde_json::to_value(self.4)?, serde_json::to_value(self.5)?, serde_json::to_value(self.6)?, serde_json::to_value(self.7)?, serde_json::to_value(self.8)?, serde_json::to_value(self.9)?])
    }
}


pub trait NativeValueToJs: Serialize {}
impl<T: Serialize> NativeValueToJs for T {}

pub trait NativeValueFromJs: serde::de::DeserializeOwned + JsonSchema {}
impl<T: serde::de::DeserializeOwned + JsonSchema> NativeValueFromJs for T {}


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

    serde::forward_to_deserialize_any!(
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    );
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
        struct EnumAccess<A> {
            inner: A,
            page: Arc<PageInner>,
        }
        impl<'de, A: serde::de::EnumAccess<'de>> serde::de::EnumAccess<'de> for EnumAccess<A> {
            type Error = A::Error;
            type Variant = A::Variant;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where V: serde::de::DeserializeSeed<'de> {
                let seed = PageSeed::new(self.page.clone(), seed);
                self.inner.variant_seed(seed)
            }
        }

        self.inner.visit_enum(EnumAccess {
            inner: data,
            page: self.page.clone(),
        })
    }
}

unsafe impl try_specialize::LifetimeFree for JsObject {}
