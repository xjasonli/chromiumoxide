use serde::de::Error;
use super::JsRemoteObjectCtx;

pub(crate) struct JsDeserializeSeed<T> {
    inner: T,
    ctx: JsRemoteObjectCtx,
}

impl<T> JsDeserializeSeed<T> {
    pub(crate) fn new(ctx: JsRemoteObjectCtx, inner: T) -> Self {
        Self { inner, ctx }
    }
}

impl<'de, T: serde::de::DeserializeSeed<'de>> serde::de::DeserializeSeed<'de> for JsDeserializeSeed<T> {
    type Value = T::Value;

    fn deserialize<D: serde::de::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let deserializer = JsDeserializer::new(deserializer, self.ctx.clone());

        self.inner.deserialize(deserializer)
            .map_err(|e| D::Error::custom(e.to_string()))
    }
}

pub(crate) struct JsDeserializer<'de, 'a> {
    inner: Box<dyn erased_serde::Deserializer<'de> + 'a>,
    ctx: JsRemoteObjectCtx,
}

impl<'de, 'a> JsDeserializer<'de, 'a> {
    pub fn get<D: serde::de::Deserializer<'de> + 'a>(
        deserializer: &D,
    ) -> Option<JsRemoteObjectCtx> {
        Self::try_get(deserializer)
            .ok()
    }

    pub fn try_get<D: serde::de::Deserializer<'de> + 'a>(
        deserializer: &D,
    ) -> Result<JsRemoteObjectCtx, D::Error> {
        use try_specialize::TrySpecialize as _;

        let this = unsafe {
            deserializer.try_specialize_ref_ignore_lifetimes::<Self>()
                .ok_or_else(|| D::Error::custom(
                    "Deserializer is not a `JsDeserializer`"
                ))?
        };
        Ok(this.ctx())
    }

    pub fn new<T: serde::de::Deserializer<'de> + 'a>(
        inner: T,
        ctx: JsRemoteObjectCtx,
    ) -> Self
    where 'de: 'a
    {
        let inner = Box::new(<dyn erased_serde::Deserializer<'de>>::erase(inner));
        Self {
            inner,
            ctx,
        }
    }

    pub fn ctx(&self) -> JsRemoteObjectCtx {
        self.ctx.clone()
    }
}

macro_rules! impl_deserialize_method {
    ($method:ident $(($($name:ident: $type:ty),+))?) => {
        paste::paste! {
            fn [< deserialize_ $method >]<V: serde::de::Visitor<'de>>(self, $($($name: $type,)+)? visitor: V) -> Result<V::Value, Self::Error> {
                self.inner.[< deserialize_ $method >](
                    $(
                        $($name,)*
                    )?
                    JsVisitor {
                        inner: visitor,
                        ctx: self.ctx,
                    }
                )
            }
        }
    };
}

impl<'de, 'a> serde::de::Deserializer<'de> for JsDeserializer<'de, 'a> {
    type Error = erased_serde::Error;

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
    impl_deserialize_method!(unit_struct(name: &'static str));
    impl_deserialize_method!(newtype_struct(name: &'static str));
    impl_deserialize_method!(seq);
    impl_deserialize_method!(tuple(len: usize));
    impl_deserialize_method!(tuple_struct(name: &'static str, len: usize));
    impl_deserialize_method!(struct(name: &'static str, fields: &'static [&'static str]));
    impl_deserialize_method!(enum(name: &'static str, variants: &'static [&'static str]));
    impl_deserialize_method!(identifier);
    impl_deserialize_method!(ignored_any);
}

struct JsVisitor<V> {
    inner: V,
    ctx: JsRemoteObjectCtx,
}

macro_rules! impl_visit_method {
    ($method:ident$(($name:ident: $type:ty) $({ $($stmt:stmt)* })?)?) => {
        paste::paste! {
            fn [< visit_ $method >]<E>(self, $($name: $type)?) -> Result<Self::Value, E>
            where E: serde::de::Error {
                $(
                    $($stmt)*
                )?
                let result = self.inner.[< visit_ $method >]($($name)?);
                result
            }
        }
    };
    ($method:ident<$type:ty: $trait:ty>($name:ident : $type1:ty) -> $type2:ty $(;$( $stmt:tt )+)?) => {
        paste::paste! {
            fn [< visit_ $method >]<$type>(self, $name: $type1) -> Result<Self::Value, $type::Error>
            where $type1: $trait {
                $(
                    $($stmt)+
                )?
                let $name = $type2 {
                    inner: $name,
                    ctx: self.ctx.clone(),
                };
                let result = self.inner.[< visit_ $method >]($name)
                    .map_err(|e| $type::Error::custom(e.to_string()));
                result
            }
        }
    };
}

impl<'de, V: serde::de::Visitor<'de>> serde::de::Visitor<'de> for JsVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    impl_visit_method!(none);
    impl_visit_method!(unit);
    impl_visit_method!(bool(v: bool));
    impl_visit_method!(i8(v: i8));
    impl_visit_method!(i16(v: i16));
    impl_visit_method!(i32(v: i32));
    impl_visit_method!(i64(v: i64));
    impl_visit_method!(i128(v: i128));
    impl_visit_method!(u8(v: u8));
    impl_visit_method!(u16(v: u16));
    impl_visit_method!(u32(v: u32));
    impl_visit_method!(u64(v: u64));
    impl_visit_method!(u128(v: u128));
    impl_visit_method!(f32(v: f32));
    impl_visit_method!(f64(v: f64));
    impl_visit_method!(char(v: char));
    impl_visit_method!(str(v: &str));
    impl_visit_method!(borrowed_str(v: &'de str));
    impl_visit_method!(string(v: String));
    impl_visit_method!(bytes(v: &[u8]));
    impl_visit_method!(borrowed_bytes(v: &'de [u8]));
    impl_visit_method!(byte_buf(v: Vec<u8>));
    impl_visit_method!(
        some<D: serde::de::Deserializer<'de>>(deserializer: D) -> JsDeserializer;

        let deserializer = Box::new(<dyn erased_serde::Deserializer<'de>>::erase(deserializer));
    );
    impl_visit_method!(
        newtype_struct<D: serde::de::Deserializer<'de>>(deserializer: D) -> JsDeserializer;

        let deserializer = Box::new(<dyn erased_serde::Deserializer<'de>>::erase(deserializer));
    );
    impl_visit_method!(
        seq<A: serde::de::SeqAccess<'de>>(seq: A) -> SeqAccess;

        struct SeqAccess<A> {
            inner: A,
            ctx: JsRemoteObjectCtx,
        }
        impl<'de, A: serde::de::SeqAccess<'de>> serde::de::SeqAccess<'de> for SeqAccess<A> {
            type Error = A::Error;

            fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
                let seed = JsDeserializeSeed::new(self.ctx.clone(), seed);
                self.inner.next_element_seed(seed)
            }

            fn size_hint(&self) -> Option<usize> {
                self.inner.size_hint()
            }
        }
    );
    impl_visit_method!(
        map<A: serde::de::MapAccess<'de>>(map: A) -> MapAccess;

        struct MapAccess<A> {
            inner: A,
            ctx: JsRemoteObjectCtx,
        }
        impl<'de, A: serde::de::MapAccess<'de>> serde::de::MapAccess<'de> for MapAccess<A> {
            type Error = A::Error;

            fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
                self.inner.next_key_seed(seed)
            }
            fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
                let seed = JsDeserializeSeed::new(self.ctx.clone(), seed);
                self.inner.next_value_seed(seed)
            }
            fn size_hint(&self) -> Option<usize> {
                self.inner.size_hint()
            }
        }
    );
    impl_visit_method!(
        enum<A: serde::de::EnumAccess<'de>>(data: A) -> EnumAccess;

        struct VariantAccess<A> {
            inner: A,
            ctx: JsRemoteObjectCtx,
        }
        impl<'de, A: serde::de::VariantAccess<'de>> serde::de::VariantAccess<'de> for VariantAccess<A> {
            type Error = A::Error;

            fn unit_variant(self) -> Result<(), Self::Error> {
                self.inner.unit_variant()
            }

            fn newtype_variant_seed<T: serde::de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
                let seed = JsDeserializeSeed::new(self.ctx.clone(), seed);
                self.inner.newtype_variant_seed(seed)
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where V: serde::de::Visitor<'de> {
                self.inner.tuple_variant(len, JsVisitor {
                    inner: visitor,
                    ctx: self.ctx.clone(),
                })
            }

            fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
            where V: serde::de::Visitor<'de> {
                self.inner.struct_variant(fields, JsVisitor {
                    inner: visitor,
                    ctx: self.ctx.clone(),
                })
            }
        }

        struct EnumAccess<A> {
            inner: A,
            ctx: JsRemoteObjectCtx,
        }
        impl<'de, A: serde::de::EnumAccess<'de>> serde::de::EnumAccess<'de> for EnumAccess<A> {
            type Error = A::Error;
            type Variant = VariantAccess<A::Variant>;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where V: serde::de::DeserializeSeed<'de> {
                let seed = JsDeserializeSeed::new(self.ctx.clone(), seed);
                let (value, variant) = self.inner.variant_seed(seed)?;
                Ok((
                    value,
                    VariantAccess {
                        inner: variant,
                        ctx: self.ctx.clone(),
                    },
                ))
            }
        }
    );
}
