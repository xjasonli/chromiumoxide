use std::borrow::Cow;

use super::*;

#[macro_export]
macro_rules! js_expr_str {
    ($($js:tt)+) => {
        stringify!($($js)+)
    }
}
#[macro_export]
macro_rules! js_expr {
    ($($js:tt)+) => {
        crate::js::JsExpr::const_new(stringify!($($js)+))
    }
}
pub use js_expr;
pub use js_expr_str;

pub(crate) const JS_EXPR_KEY  : &str = "$chromiumoxide::js::expr";

#[derive(Debug, Clone)]
pub struct JsExpr<'a>(pub Cow<'a, str>);
impl<'a> JsExpr<'a> {
    pub const fn const_new(expr: &'a str) -> Self {
        JsExpr(Cow::Borrowed(expr))
    }

    pub fn new<T: Into<Cow<'a, str>>>(expr: T) -> Self {
        JsExpr(expr.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a, T> From<T> for JsExpr<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

impl<'a> std::fmt::Display for JsExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl<'a> serde::Serialize for JsExpr<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("JsExpr", 1)?;
        s.serialize_field(JS_EXPR_KEY , self.0.as_ref())?;
        s.end()
    }
}

impl<'a, 'de> serde::Deserialize<'de> for JsExpr<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Key;
        impl<'de> serde::Deserialize<'de> for Key {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct KeyVisitor;
                impl<'de> serde::de::Visitor<'de> for KeyVisitor {
                    type Value = Key;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(JS_EXPR_KEY)
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == JS_EXPR_KEY {
                            Ok(Key)
                        } else {
                            Err(E::unknown_field(value, &[JS_EXPR_KEY]))
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
                Err(A::Error::missing_field(JS_EXPR_KEY))
            }
        }

        let value = deserializer.deserialize_struct(
            JS_EXPR_KEY,
            &[JS_EXPR_KEY],
            Visitor
        )?;

        Ok(JsExpr::new(value))
    }
}


/// The context for the expression
/// 
/// This enum provides three ways to identify and scope the execution context for the expression:
/// - By the `this` value, which also identifies the execution context
/// - By the execution context, identified by an object
/// - By the execution context, identified by an ExecutionContextId or UniqueId
#[derive(Debug, Clone)]
pub enum JsExprContext {
    /// The `this` value for the expression, also used as the execution context
    This(JsRemoteObject),

    /// The execution context for the expression, without this value
    Context(ExecutionContext),

    /// The execution context (identified by an object) for the expression, without this value
    ContextObject(JsRemoteObject),
}

impl JsExprContext {
    pub fn new_with_this<T: AsJs<JsRemoteObject>>(this: T) -> Self {
        Self::This(this.as_js().clone())
    }

    pub fn new_with_context<T: Into<ExecutionContext>>(context: T) -> Self {
        Self::Context(context.into())
    }

    pub fn new_with_context_object<T: AsJs<JsRemoteObject>>(context: T) -> Self {
        Self::ContextObject(context.as_js().clone())
    }
}

impl<T: Into<ExecutionContext>> From<T> for JsExprContext {
    fn from(context: T) -> Self {
        Self::Context(context.into())
    }
}


// implement IntoJs<AnyType> for JsExpr<'a>
impl<'a> IntoJs<str> for JsExpr<'a> {
    type FromJs = String;
}

impl<'a> IntoJs<JsRemoteObject> for JsExpr<'a> {
    type FromJs = JsRemoteObject;
}

impl<'a> IntoJs<JsFunction> for JsExpr<'a> {
    type FromJs = JsFunction;
}
