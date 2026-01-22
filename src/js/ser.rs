use std::{cell::RefCell, rc::Rc};

use super::JsRemoteObject;


#[derive(Debug, Clone)]
pub struct Ctx {
    objects: Rc<RefCell<Vec<JsRemoteObject>>>,
}

impl Ctx {
    pub fn new() -> Self {
        Self { objects: Rc::new(RefCell::new(vec![])) }
    }

    pub fn add(&self, obj: &JsRemoteObject) {
        self.objects.borrow_mut().push(obj.clone());
    }

    pub fn get(&self) -> Vec<JsRemoteObject> {
        self.objects.borrow().clone()
    }
}

thread_local! {
    static CURRENT_CTX: RefCell<Option<Ctx>> = RefCell::new(None);
}

/// Guard that clears the thread_local context when dropped
#[derive(Debug, Clone)]
struct CtxGuard(#[allow(dead_code)] Rc<CtxGuardInner>);

#[derive(Debug)]
struct CtxGuardInner(());

impl CtxGuardInner {
    fn new(ctx: Ctx) -> Self {
        CURRENT_CTX.with(|current| {
            *current.borrow_mut() = Some(ctx);
        });
        Self(())
    }
}

impl Drop for CtxGuardInner {
    fn drop(&mut self) {
        CURRENT_CTX.with(|current| {
            *current.borrow_mut() = None;
        });
    }
}

impl CtxGuard {
    fn new(ctx: Ctx) -> Self {
        Self(Rc::new(CtxGuardInner::new(ctx)))
    }
}

/// Add a RemoteObject to the current serialization context (if any)
pub fn add_to_current_ctx(obj: &JsRemoteObject) {
    CURRENT_CTX.with(|current| {
        if let Some(ctx) = current.borrow().as_ref() {
            ctx.add(obj);
        }
    });
}

pub type JsSerializerCtx = Ctx;
pub type JsJsonSerializer = JsSerializer<serde_json::value::Serializer>;
unsafe impl try_specialize::LifetimeFree for JsJsonSerializer {}

#[derive(Debug)]
pub struct JsSerializer<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T: serde::ser::Serializer> JsSerializer<T> {
    pub fn new(inner: T) -> (Self, Ctx) {
        let ctx = Ctx::new();
        let guard = CtxGuard::new(ctx.clone());
        (Self { inner, _guard: guard }, ctx)
    }
}

impl JsSerializer<serde_json::value::Serializer> {
    pub fn new_json_serializer() -> (Self, JsSerializerCtx) {
        Self::new(serde_json::value::Serializer)
    }
}

impl<T> serde::ser::Serializer for JsSerializer<T>
where
    T: serde::ser::Serializer,
{
    type Ok = <T as serde::ser::Serializer>::Ok;
    type Error = <T as serde::ser::Serializer>::Error;

    type SerializeSeq = JsSerializeSeq<T::SerializeSeq>;
    type SerializeTuple = JsSerializeTuple<T::SerializeTuple>;
    type SerializeTupleStruct = JsSerializeTupleStruct<T::SerializeTupleStruct>;
    type SerializeTupleVariant = JsSerializeTupleVariant<T::SerializeTupleVariant>;
    type SerializeMap = JsSerializeMap<T::SerializeMap>;
    type SerializeStruct = JsSerializeStruct<T::SerializeStruct>;
    type SerializeStructVariant = JsSerializeStructVariant<T::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<V: ?Sized>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_some(&value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<V: ?Sized>(self, name: &'static str, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_newtype_struct(name, &value)
    }

    fn serialize_newtype_variant<V: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_newtype_variant(name, variant_index, variant, &value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let seq = JsSerializeSeq {
            inner: self.inner.serialize_seq(len)?,
            _guard: self._guard.clone(),
        };
        Ok(seq)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let tuple = JsSerializeTuple {
            inner: self.inner.serialize_tuple(len)?,
            _guard: self._guard.clone(),
        };
        Ok(tuple)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let tuple_struct = JsSerializeTupleStruct {
            inner: self.inner.serialize_tuple_struct(name, len)?,
            _guard: self._guard.clone(),
        };
        Ok(tuple_struct)
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let tuple_variant = JsSerializeTupleVariant {
            inner: self.inner.serialize_tuple_variant(name, variant_index, variant, len)?,
            _guard: self._guard.clone(),
        };
        Ok(tuple_variant)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let map = JsSerializeMap {
            inner: self.inner.serialize_map(len)?,
            _guard: self._guard.clone(),
        };
        Ok(map)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        let struct_ = JsSerializeStruct {
            inner: self.inner.serialize_struct(name, len)?,
            _guard: self._guard.clone(),
        };
        Ok(struct_)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        let struct_variant = JsSerializeStructVariant {
            inner: self.inner.serialize_struct_variant(name, variant_index, variant, len)?,
            _guard: self._guard.clone(),
        };
        Ok(struct_variant)
    }
}

#[derive(Debug)]
pub struct JsSerialize<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::Serialize for JsSerialize<T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

#[derive(Debug)]
pub struct JsSerializeSeq<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::SerializeSeq for JsSerializeSeq<T>
where
    T: serde::ser::SerializeSeq,
{
    type Ok = <T as serde::ser::SerializeSeq>::Ok;
    type Error = <T as serde::ser::SerializeSeq>::Error;

    fn serialize_element<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_element(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeTuple<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::SerializeTuple for JsSerializeTuple<T>
where
    T: serde::ser::SerializeTuple,
{
    type Ok = <T as serde::ser::SerializeTuple>::Ok;
    type Error = <T as serde::ser::SerializeTuple>::Error;

    fn serialize_element<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_element(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeTupleStruct<T> {
    inner: T,
    _guard: CtxGuard,
}


impl<T> serde::ser::SerializeTupleStruct for JsSerializeTupleStruct<T>
where
    T: serde::ser::SerializeTupleStruct,
{
    type Ok = <T as serde::ser::SerializeTupleStruct>::Ok;
    type Error = <T as serde::ser::SerializeTupleStruct>::Error;

    fn serialize_field<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_field(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeTupleVariant<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::SerializeTupleVariant for JsSerializeTupleVariant<T>
where
    T: serde::ser::SerializeTupleVariant,
{
    type Ok = <T as serde::ser::SerializeTupleVariant>::Ok;
    type Error = <T as serde::ser::SerializeTupleVariant>::Error;

    fn serialize_field<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_field(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeMap<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::SerializeMap for JsSerializeMap<T>
where
    T: serde::ser::SerializeMap,
{
    type Ok = <T as serde::ser::SerializeMap>::Ok;
    type Error = <T as serde::ser::SerializeMap>::Error;

    fn serialize_key<V: ?Sized>(&mut self, key: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let key = JsSerialize { inner: key, _guard: self._guard.clone() };
        self.inner.serialize_key(&key)
    }

    fn serialize_value<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_value(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeStruct<T> {
    inner: T,
    _guard: CtxGuard,
}

impl<T> serde::ser::SerializeStruct for JsSerializeStruct<T>
where
    T: serde::ser::SerializeStruct,
{
    type Ok = <T as serde::ser::SerializeStruct>::Ok;
    type Error = <T as serde::ser::SerializeStruct>::Error;

    fn serialize_field<V: ?Sized>(&mut self, key: &'static str, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_field(key, &value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

#[derive(Debug)]
pub struct JsSerializeStructVariant<T> {
    inner: T,
    _guard: CtxGuard,
}


impl<T> serde::ser::SerializeStructVariant for JsSerializeStructVariant<T>
where
    T: serde::ser::SerializeStructVariant,
{
    type Ok = <T as serde::ser::SerializeStructVariant>::Ok;
    type Error = <T as serde::ser::SerializeStructVariant>::Error;

    fn serialize_field<V: ?Sized>(&mut self, key: &'static str, value: &V) -> Result<(), Self::Error>
    where
        V: serde::Serialize,
    {
        let value = JsSerialize { inner: value, _guard: self._guard.clone() };
        self.inner.serialize_field(key, &value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
