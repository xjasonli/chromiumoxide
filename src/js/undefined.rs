use super::helper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsUndefined;

impl serde::Serialize for JsUndefined {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("JsUndefined", 1)?;
        s.serialize_field(helper::JS_UNDEFINED_KEY, &())?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for JsUndefined {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Key;
        impl<'de> serde::Deserialize<'de> for Key {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct KeyVisitor;
                impl<'de> serde::de::Visitor<'de> for KeyVisitor {
                    type Value = Key;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(helper::JS_UNDEFINED_KEY)
                    }

                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == helper::JS_UNDEFINED_KEY {
                            Ok(Key)
                        } else {
                            Err(E::unknown_field(value, &[helper::JS_UNDEFINED_KEY]))
                        }
                    }
                }

                deserializer.deserialize_identifier(KeyVisitor)
            }
        }

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ();

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct JsUndefined")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                while let Some(_) = map.next_key::<Key>()? {
                    let val = map.next_value()?;
                    return Ok(val);
                }
                use serde::de::Error as _;
                Err(A::Error::missing_field(helper::JS_UNDEFINED_KEY))
            }
        }
        deserializer.deserialize_struct(
            "JsUndefined",
            &[helper::JS_UNDEFINED_KEY],
            Visitor
        )?;
        Ok(JsUndefined)
    }
}

impl schemars::JsonSchema for JsUndefined {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("JsUndefined")
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(std::concat!(
            ::core::module_path!(),
            "::",
            "JsUndefined"
        ))
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let schema = schemars::json_schema!({
            "type": "object",
            "properties": {
                helper::JS_UNDEFINED_KEY: { "type": "null" },
            },
            "required": [helper::JS_UNDEFINED_KEY],
        });
        schema
    }
}