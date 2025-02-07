use crate::js::helper;

#[derive(Debug, Clone)]
pub struct JsExpr(pub String);

impl JsExpr {
    pub fn new<S: Into<String>>(expr: S) -> Self {
        JsExpr(expr.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for JsExpr {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for JsExpr {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

impl From<JsExpr> for String {
    fn from(expr: JsExpr) -> Self {
        expr.0
    }
}

impl std::fmt::Display for JsExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::Serialize for JsExpr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("JsExpr", 1)?;
        s.serialize_field(helper::JS_EXPR_KEY , &self.0)?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for JsExpr {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Key;
        impl<'de> serde::Deserialize<'de> for Key {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct KeyVisitor;
                impl<'de> serde::de::Visitor<'de> for KeyVisitor {
                    type Value = Key;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(helper::JS_EXPR_KEY)
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == helper::JS_EXPR_KEY {
                            Ok(Key)
                        } else {
                            Err(E::unknown_field(value, &[helper::JS_EXPR_KEY]))
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
                formatter.write_str("struct JsExpr")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                while let Some(_) = map.next_key::<Key>()? {
                    let val = map.next_value()?;
                    return Ok(val);
                }
                use serde::de::Error as _;
                Err(A::Error::missing_field(helper::JS_EXPR_KEY))
            }
        }

        let value = deserializer.deserialize_struct(
            helper::JS_EXPR_KEY,
            &[helper::JS_EXPR_KEY],
            Visitor
        )?;
        Ok(JsExpr(value))
    }
}
