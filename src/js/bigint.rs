use super::helper;

#[derive(Debug, Clone)]
pub struct JsBigInt(pub String);

impl JsBigInt {
    pub fn new(big_int: String) -> Self {
        Self(big_int)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


impl serde::Serialize for JsBigInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("JsBigInt", 1)?;
        s.serialize_field(helper::JS_BIGINT_KEY, &self.0)?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for JsBigInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Key;
        impl<'de> serde::Deserialize<'de> for Key {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct KeyVisitor;
                impl<'de> serde::de::Visitor<'de> for KeyVisitor {
                    type Value = Key;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(helper::JS_BIGINT_KEY)
                    }

                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == helper::JS_BIGINT_KEY {
                            Ok(Key)
                        } else {
                            Err(E::unknown_field(value, &[helper::JS_BIGINT_KEY]))
                        }
                    }   
                }
                deserializer.deserialize_identifier(KeyVisitor)
            }
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct JsBigInt")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                while let Some(_) = map.next_key::<Key>()? {
                    let val = map.next_value()?;
                    return Ok(val);
                }
                use serde::de::Error as _;
                Err(A::Error::missing_field(helper::JS_BIGINT_KEY))
            }
        }
        let value = deserializer.deserialize_struct(
            helper::JS_BIGINT_KEY,
            &[helper::JS_BIGINT_KEY],
            Visitor
        )?;
        Ok(JsBigInt(value))
    }
}

impl schemars::JsonSchema for JsBigInt {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("JsBigInt")
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(std::concat!(
            ::core::module_path!(),
            "::",
            "JsBigInt"
        ))
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let schema = schemars::json_schema!({
            "type": "object",
            "properties": {
                helper::JS_BIGINT_KEY: { "type": "string" },
            },
            "required": [helper::JS_BIGINT_KEY],
        });
        schema
    }
}

impl From<String> for JsBigInt {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<JsBigInt> for String {
    fn from(big_int: JsBigInt) -> Self {
        big_int.0
    }
}

impl AsRef<str> for JsBigInt {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for JsBigInt {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}
