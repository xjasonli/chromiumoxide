use std::{collections::{BTreeMap, HashMap}, sync::Arc};
use chromiumoxide_cdp::cdp::js_protocol::runtime::RemoteObjectId;
use serde::{de::DeserializeOwned, Deserialize};
use serde::Serialize;

use crate::error::{CdpError, Result};
use crate::Page;

pub use serde_json::{Value as JsonValue, Number as JsonNumber};

pub trait NativeValue: Serialize + DeserializeOwned {
}

#[derive(Debug, Clone)]
pub enum Value {
    // Trivial values
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),

    // Unserializable values
    RemoteObject(RemoteObjectId),

    // Containers
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self { Value::Null }
}

impl Value {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DomNode {
    remote_object_id: RemoteObjectId,

    #[serde(skip)]
    page: Page,
}

impl NativeRemoteValue for DomNode {
    fn new(remote_object_id: RemoteObjectId, page: Page) -> Self {
        Self { remote_object_id, page }
    }
}

//impl NativeValue for Vec<DomNode> {
//    fn to_json(&self) -> Result<JsonValue> {
//        let v = self.iter()
//            .map(|node| HashMap::default())
//            .collect::<Vec<_>>();
//        Ok(serde_json::to_value(v)?)
//    }
//
//    fn to_remote(&self) -> Option<RemoteValue> {
//        Some(
//            RemoteValue::Array(
//                self.iter()
//                    .map(|node| RemoteValue::Remote(node.0.clone()))
//                    .collect()
//            )
//        )
//    }
//}


#[derive(Debug)]
pub enum RemoteValue {
    Object(RemoteObjectId),
    Properties(BTreeMap<JsonPathSegment, RemoteValue>),
}

impl RemoteValue {
    pub fn iter(&self) -> impl Iterator<Item = (Vec<&JsonPathSegment>, &RemoteObjectId)> {
        RemoteValueIter {
            value: self,
            stack: vec![],
            ended: false,
        }
    }
    pub fn get(&self, path: &JsonPath) -> Result<&RemoteValue> {
        let mut path = &path[..];
        let mut current = self;
        while let Some((head, tail)) = path.split_first() {
            let properties = current.as_properties()
                .ok_or(CdpError::NotFound)?;
            let value = properties.get(head)
                .ok_or(CdpError::NotFound)?;
            path = tail;
            current = value;
        }
        Ok(current)
    }

    pub fn get_object(&self, path: &JsonPath) -> Result<&RemoteObjectId> {
        self.get(path).and_then(|v| v.as_object().ok_or(CdpError::NotFound))
    }

    fn into_object(self) -> Option<RemoteObjectId> {
        match self {
            RemoteValue::Object(id) => Some(id),
            _ => None,
        }
    }
    fn into_properties(self) -> Option<BTreeMap<JsonPathSegment, RemoteValue>> {
        match self {
            RemoteValue::Properties(props) => Some(props),
            _ => None,
        }
    }
    fn is_object(self) -> bool {
        matches!(self, RemoteValue::Object(_))
    }
    fn is_properties(self) -> bool {
        matches!(self, RemoteValue::Properties(_))
    }
    fn as_object(&self) -> Option<&RemoteObjectId> {
        match self {
            RemoteValue::Object(id) => Some(id),
            _ => None,
        }
    }
    fn as_properties(&self) -> Option<&BTreeMap<JsonPathSegment, RemoteValue>> {
        match self {
            RemoteValue::Properties(props) => Some(props),
            _ => None,
        }
    }
}

struct RemoteValueIter<'a> {
    value: &'a RemoteValue,
    stack: Vec<(Vec<&'a JsonPathSegment>, std::collections::btree_map::Iter<'a, JsonPathSegment, RemoteValue>)>,
    ended: bool,
}

impl<'a> Iterator for RemoteValueIter<'a> {
    type Item = (Vec<&'a JsonPathSegment>, &'a RemoteObjectId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        if self.stack.is_empty() {
            match &self.value {
                RemoteValue::Object(id) => {
                    self.ended = true;
                    return Some((vec![], id));
                }
                RemoteValue::Properties(props) => {
                    self.stack.push((vec![], props.iter()));
                }
            }
        }

        while let Some((path, iter)) = self.stack.last_mut() {
            if let Some((key, value)) = iter.next() {
                let mut new_path = path.clone();
                new_path.push(key);

                match value {
                    RemoteValue::Object(id) => {
                        return Some((new_path, id));
                    }
                    RemoteValue::Properties(props) => {
                        self.stack.push((new_path, props.iter()));
                    }
                }
            } else {
                self.stack.pop();
            }
        }

        self.ended = true;
        None
    }
}

impl RemoteValue {
}

pub trait NativeRemoteValue {
    fn new(remote_object_id: RemoteObjectId, page: Page) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonPathSegment {
    String(String),
    Number(usize),
}
impl From<String> for JsonPathSegment {
    fn from(s: String) -> Self {
        JsonPathSegment::String(s)
    }
}
impl From<usize> for JsonPathSegment {
    fn from(n: usize) -> Self {
        JsonPathSegment::Number(n)
    }
}
impl From<&str> for JsonPathSegment {
    fn from(s: &str) -> Self {
        JsonPathSegment::String(s.to_string())
    }
}

pub type JsonPath = Vec<JsonPathSegment>;

pub trait NativeJsonValue<T> : DeserializeOwned + Serialize {
    type Iterator: Iterator<Item = JsonPath>;
    fn remote_objects(&self) -> Self::Iterator;
}

pub trait NativeValue: Sized {
    type JsonValue: NativeJsonValue<Self>;

    fn from_js(json: Self::JsonValue, remote: Option<(RemoteValue, Page)>) -> Result<Self>;
    fn into_js(self) -> Result<(Self::JsonValue, Option<RemoteValue>)>;
}

impl<T: Serialize + DeserializeOwned> NativeJsonValue<T> for T {
    type Iterator = std::vec::IntoIter<JsonPath>;
    fn remote_objects(&self) -> Self::Iterator {
        vec![vec![]].into_iter()
    }
}

impl<T: NativeJsonValue<T>> NativeValue for T {
    default type JsonValue = T;

    default fn from_js(json: Self::JsonValue, _remote: Option<(RemoteValue, Page)>) -> Result<Self> {
        Ok(json)
    }
    default fn into_js(self) -> Result<(Self::JsonValue, Option<RemoteValue>)> {
        Ok((self, None))
    }
}

impl NativeJsonValue<DomNode> for () {
    type Iterator = std::vec::IntoIter<JsonPath>;
    fn remote_objects(&self) -> Self::Iterator {
        vec![vec![]].into_iter()
    }
}

impl NativeValue for DomNode {
    type JsonValue = ();

    fn from_js(_json: Self::JsonValue, remote: Option<(RemoteValue, Page)>) -> Result<Self> {
        let remote = remote.ok_or_else(|| CdpError::NotFound)?;
        let id = remote.0.into_object().ok_or_else(|| CdpError::NotFound)?;
        Ok(DomNode { remote_object_id: id, page: remote.1 })
    }

    fn into_js(self) -> Result<(Self::JsonValue, Option<RemoteValue>)> {
        Ok((Default::default(), Some(RemoteValue::Object(self.remote_object_id))))
    }
}

impl NativeJsonValue<Vec<DomNode>> for Vec<()> {
    type Iterator = std::vec::IntoIter<JsonPath>;
    fn remote_objects(&self) -> Self::Iterator {
        vec![vec![]].into_iter()
    }
}


impl NativeValue for Vec<DomNode> {
    type JsonValue = Vec<()>;

    fn from_js(_json: Self::JsonValue, remote: Option<(RemoteValue, Page)>) -> Result<Self> {
        Ok(vec![])
    }
    fn into_js(self) -> Result<(Self::JsonValue, Option<RemoteValue>)> {
        Ok((Default::default(), None))
    }
}

//impl<T: NativeRemoteValue> NativeJsonValue<Vec<T>> for Vec<()> {
//    type Iterator = Box<dyn Iterator<Item = JsonPath> + 'static>;
//    fn remote_objects(&self) -> Self::Iterator {
//        Box::new(self.clone().into_iter()
//            .enumerate()
//            .map(|(idx, _)| vec![idx.into()])
//        )
//    }
//}
//
//impl<T: NativeRemoteValue> NativeValue for Vec<T> {
//    type JsonValue = Vec<()>;
//
//    fn from_js(json: Self::JsonValue, remote: Option<(RemoteValue, Page)>) -> Result<Self> {
//        todo!()
//        //let mut result = Vec::new();
//        //let x = json.remote_objects()
//        //    .map(|v| T::new(remote))
//        //    .collect::<Result<Vec<_>>>();
//        //Ok(json.into_iter().map(|v| T::from_js(v, remote)).collect::<Result<Vec<_>>>()?)
//    }
//
//    fn into_js(self) -> Result<(Self::JsonValue, Option<RemoteValue>)> {
//        todo!()
//        //let items = self.into_iter()
//        //    .map(|v| v.into_js())
//        //    .collect::<Result<Vec<_>>>()?;
//        //Ok((items.into_iter().map(|(v, _)| v).collect::<Vec<_>>(), None))
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_value() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Test {
            a: String,
            b: Option<i32>,
        }

        let s = "test".to_string();
        let value = String::from_js(s, None).unwrap();
        let (json, remote) = value.into_js().unwrap();
        println!("{:?}", json);
        println!("{:#?}", remote);
    }

    fn test_dom_node(page: Page) {
        let remote_object_id = RemoteObjectId::new("ssss");
        let node = DomNode::from_js(Default::default(), Some((RemoteValue::Object(remote_object_id), page))).unwrap();
        let (json, remote) = node.into_js().unwrap();
        println!("{:?}", json);
        println!("{:#?}", remote);
    }
}