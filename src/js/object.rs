use std::borrow::Cow;
use std::sync::Arc;

use schemars::json_schema;

use crate::cdp::js_protocol::runtime::RemoteObjectId;
use crate::handler::PageInner;
use crate::Page;
use super::function::Function;

#[derive(Debug, Clone)]
pub struct JsObject {
    pub(crate) object_id: RemoteObjectId,
    pub(crate) page: Arc<PageInner>,
}

impl JsObject {
    pub(crate) fn new(object_id: RemoteObjectId, page: Arc<PageInner>) -> Self {
        Self { object_id, page }
    }

    pub fn page(&self) -> Page {
        self.page.clone().into()
    }

    pub fn id(&self) -> &RemoteObjectId {
        &self.object_id
    }

    pub fn function(&self, function_declaration: impl Into<String>) -> Function {
        Function::new(self.page.clone(), function_declaration.into())
            .with_object(self.object_id.clone())
    }
}


impl From<JsObject> for RemoteObjectId {
    fn from(object: JsObject) -> Self {
        object.object_id
    }
}

impl From<&JsObject> for RemoteObjectId {
    fn from(object: &JsObject) -> Self {
        object.object_id.clone()
    }
}

pub(crate) const OBJECT_ID_KEY: &str = "$chromiumoxide::objectId";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub(super) struct JsObjectSerdeProxy {
    #[serde(rename = "$chromiumoxide::objectId")]
    object_id: String,
}

impl JsObjectSerdeProxy {
    pub(super) fn into_inner(self) -> RemoteObjectId {
        self.object_id.into()
    }
}

impl serde::Serialize for JsObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let proxy = JsObjectSerdeProxy {
            object_id: self.object_id.clone().into(),
        };
        proxy.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for JsObject {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        panic!("JsObject::deserialize() should never be called directly!")
    }
}

impl schemars::JsonSchema for JsObject {
    fn schema_name() -> Cow<'static, str> {
        "JsObject".into()
    }

    fn schema_id() -> Cow<'static, str> {
        concat!(module_path!(), "::JsObject").into()
    }

    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = json_schema!({
            "type": "object",
        });
        {
            schemars::_private::insert_object_property(
                &mut schema,
                OBJECT_ID_KEY,
                false,
                gen.subschema_for::<String>(),
            );
        }
        schemars::_private::insert_metadata_property_if_nonempty(
            &mut schema,
            "title",
            "JsObject",
        );
        schemars::_private::insert_metadata_property_if_nonempty(
            &mut schema,
            "description",
            "Remote object id",
        );
        schema
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use schemars::*;

    #[test]
    fn test_schema() {
        let settings = generate::SchemaSettings::default();   
        let mut generator = SchemaGenerator::new(settings);
        let schema = generator.root_schema_for::<JsObject>();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());

        #[derive(JsonSchema)]
        struct Test {
            obj: JsObject,
        }
        let schema = generator.root_schema_for::<Test>();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn test_vec_schema() {
        let mut generator = SchemaGenerator::default();
        let schema = generator.root_schema_for::<Vec<JsObject>>();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

}
