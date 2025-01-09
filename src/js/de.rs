use std::sync::Arc;
use chromiumoxide_cdp::cdp::js_protocol::runtime::RemoteObjectId;
use serde::Deserialize;

use super::object::JsObject;
use crate::handler::PageInner;


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


macro_rules! impl_deserialize_method {
    ($method:ident) => {
        impl_deserialize_method!($method +);
    };
    
    ($method:ident + $($name:ident: $type:ty),*) => {
        paste::paste! {
            fn [< deserialize_ $method >]<V: serde::de::Visitor<'de>>(self, $($name: $type,)* visitor: V) -> Result<V::Value, Self::Error> {
                if self.is_js_object::<V::Value>() {
                    self.deserialize_js_object()
                } else {
                    self.inner.[< deserialize_ $method >](
                        $($name,)*
                        PageVisitor {
                            inner: visitor,
                            page: self.page,
                        }
                    )
                }
            }
        }
    };
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

    impl_deserialize_method!(any);
    impl_deserialize_method!(bool);
    impl_deserialize_method!(i8);
    impl_deserialize_method!(i16);
    impl_deserialize_method!(i32);
    impl_deserialize_method!(i64);
    impl_deserialize_method!(i128);
    impl_deserialize_method!(u8);
    impl_deserialize_method!(u16);
    impl_deserialize_method!(u32);
    impl_deserialize_method!(u64);
    impl_deserialize_method!(u128);
    impl_deserialize_method!(f32);
    impl_deserialize_method!(f64);
    impl_deserialize_method!(char);
    impl_deserialize_method!(str);
    impl_deserialize_method!(string);
    impl_deserialize_method!(bytes);
    impl_deserialize_method!(byte_buf);
    impl_deserialize_method!(option);
    impl_deserialize_method!(unit);
    impl_deserialize_method!(map);
    impl_deserialize_method!(unit_struct + name: &'static str);
    impl_deserialize_method!(newtype_struct + name: &'static str);
    impl_deserialize_method!(seq);
    impl_deserialize_method!(tuple + len: usize);
    impl_deserialize_method!(tuple_struct + name: &'static str, len: usize);
    impl_deserialize_method!(struct + name: &'static str, fields: &'static [&'static str]);
    impl_deserialize_method!(enum + name: &'static str, variants: &'static [&'static str]);
    impl_deserialize_method!(identifier);
    impl_deserialize_method!(ignored_any);
}

macro_rules! impl_visit_method {
    ($method:ident) => {
        impl_visit_method!($method + );
    };
    ($method:ident + $($name:ident: $type:ty $({ $($stmt:stmt)* })?)?) => {
        paste::paste! {
            fn [< visit_ $method >]<E>(self, $($name: $type)?) -> Result<Self::Value, E>
            where E: serde::de::Error {
                $(
                    $($stmt)*
                )?
                self.inner.[< visit_ $method >]($($name)?)
            }
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

    impl_visit_method!(bool + v: bool);
    impl_visit_method!(i8 + v: i8);
    impl_visit_method!(i16 + v: i16);
    impl_visit_method!(i32 + v: i32);
    impl_visit_method!(i64 + v: i64);
    impl_visit_method!(u8 + v: u8);
    impl_visit_method!(u16 + v: u16);
    impl_visit_method!(u32 + v: u32);
    impl_visit_method!(u64 + v: u64);
    impl_visit_method!(f32 + v: f32);
    impl_visit_method!(f64 + v: f64);
    impl_visit_method!(char + v: char);
    impl_visit_method!(str + v: &str);
    impl_visit_method!(string + v: String);
    impl_visit_method!(bytes + v: &[u8]);
    impl_visit_method!(byte_buf + v: Vec<u8>);
    impl_visit_method!(none);
    impl_visit_method!(unit);

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: serde::de::Deserializer<'de> {
        let deserializer = PageDeserializer {
            inner: deserializer,
            page: self.page.clone(),
        };
        self.inner.visit_some(deserializer)
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