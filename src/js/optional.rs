use chromiumoxide_cdp::cdp::js_protocol::runtime::{RemoteObject, RemoteObjectType};
use super::*;

/// Represents a value that can be either a value of type `T` or JavaScript's `undefined`.
/// 
/// This type is used to handle JavaScript's optional values in Rust. While similar to Rust's
/// `Option` type, there is an important distinction:
/// 
/// - `Optional::Undefined` represents JavaScript's `undefined` value
/// - `Option::None` represents JavaScript's `null` value
/// 
/// This separation is important because JavaScript treats `undefined` and `null` as distinct values,
/// even though they are both "empty" values. When working with JavaScript interop:
/// 
/// - Use `Optional<T>` when you need to handle a value that might be `undefined`
/// - Use `Option<T>` when you need to handle a value that might be `null`
/// - Use `Optional<Option<T>>` when you need to handle both cases
/// 
/// # Example
/// ```no_run
/// use chromiumoxide::js::Optional;
/// 
/// // A value that might be undefined in JavaScript
/// let value: Optional<i32> = Optional::Value(42);
/// let undefined: Optional<i32> = Optional::Undefined;
/// 
/// // Handling both undefined and null
/// let value: Optional<Option<i32>> = Optional::Value(Some(42));     // normal value
/// let null: Optional<Option<i32>> = Optional::Value(None);          // null
/// let undefined: Optional<Option<i32>> = Optional::Undefined;       // undefined
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Optional<T = JsUndefined> {
    /// Represents JavaScript's `undefined` value
    #[serde(serialize_with = "undefined::serialize")]
    #[serde(deserialize_with = "undefined::deserialize")]
    #[schemars(with = "JsUndefined")]
    None,

    /// Contains an actual value of type `T`
    Some(T),
}

impl<T: IntoJsAny> From<T> for Optional<T> {
    fn from(value: T) -> Self {
        use try_specialize::Specialization;
        if let Some(_) = Specialization::<T, JsUndefined>::try_new() {
            return Self::None;
        }
        Self::Some(value)
    }
}

impl<T> Optional<T> {
    /// Returns true if the optional contains a value (is not undefined)
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Returns true if the optional is undefined
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns a reference to the contained value as a new Optional
    pub fn as_ref(&self) -> Optional<&T> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => Optional::Some(t),
        }
    }

    /// Returns a mutable reference to the contained value as a new Optional
    pub fn as_mut(&mut self) -> Optional<&mut T> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => Optional::Some(t),
        }
    }

    /// Converts to Option<&T>
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::None => None,
            Self::Some(t) => Some(t),
        }
    }

    /// Converts to Option<T>
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Some(t) => Some(t),
        }
    }

    /// Returns the contained value or a default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::None => default,
            Self::Some(t) => t,
        }
    }

    /// Returns the contained value or computes it from a closure
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::None => f(),
            Self::Some(t) => t,
        }
    }

    /// Returns the contained value or panics
    pub fn unwrap(self) -> T {
        match self {
            Self::None => panic!("called `Optional::unwrap()` on an `Undefined` value"),
            Self::Some(t) => t,
        }
    }

    /// Returns the contained value or panics with a custom message
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::None => panic!("{}", msg),
            Self::Some(t) => t,
        }
    }

    /// Maps an Optional<T> to Optional<U> by applying a function to the contained value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Optional<U> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => Optional::Some(f(t)),
        }
    }

    /// Maps an Optional<T> to Optional<U> by applying a function to a reference of the contained value
    pub fn map_ref<U, F: FnOnce(&T) -> U>(&self, f: F) -> Optional<U> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => Optional::Some(f(t)),
        }
    }

    /// Returns None if the optional is None, otherwise calls f with the wrapped value and returns the result
    pub fn and_then<U, F: FnOnce(T) -> Optional<U>>(self, f: F) -> Optional<U> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => f(t),
        }
    }

    /// Returns the optional if it contains a value, otherwise returns other
    pub fn or(self, other: Optional<T>) -> Optional<T> {
        match self {
            Self::None => other,
            Self::Some(_) => self,
        }
    }

    /// Returns the optional if it contains a value, otherwise calls f and returns the result
    pub fn or_else<F: FnOnce() -> Optional<T>>(self, f: F) -> Optional<T> {
        match self {
            Self::None => f(),
            Self::Some(_) => self,
        }
    }

    /// Returns Optional::Value if the option contains a value that matches the predicate
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Optional<T> {
        match self {
            Self::None => Optional::None,
            Self::Some(t) => if predicate(&t) {
                Optional::Some(t)
            } else {
                Optional::None
            },
        }
    }

    /// Transforms the Optional<T> into a Result<T, E>
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Self::None => Err(err),
            Self::Some(t) => Ok(t),
        }
    }

    /// Transforms the Optional<T> into a Result<T, E> by mapping the error
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Self::None => Err(err()),
            Self::Some(t) => Ok(t),
        }
    }

    /// Zips two optionals together into an optional tuple
    pub fn zip<U>(self, other: Optional<U>) -> Optional<(T, U)> {
        match (self, other) {
            (Self::Some(t), Optional::Some(u)) => Optional::Some((t, u)),
            _ => Optional::None,
        }
    }
}

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self::None
    }
}

/// Represents JavaScript's `undefined` value.
/// 
/// This type is used to represent the JavaScript `undefined` value in Rust code.
/// It is primarily used for JavaScript interop within the ChromiumOxide library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsUndefined;

impl serde::Serialize for JsUndefined {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        undefined::serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for JsUndefined {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        undefined::deserialize(deserializer)?;
        Ok(Self)
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

impl JsUndefined {
    #[allow(unused)]
    pub(crate) fn from_remote_object(remote_object: &RemoteObject) -> Option<Self> {
        if remote_object.r#type != RemoteObjectType::Undefined {
            return None;
        }
        Some(Self)
    }
}

mod undefined {
    use super::*;

    unsafe impl try_specialize::LifetimeFree for JsUndefined {}

    pub(super) fn serialize<S: serde::Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("JsUndefined", 1)?;
        s.serialize_field(helper::JS_UNDEFINED_KEY, &())?;
        s.end()
    }

    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
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
        Ok(())
    }
}

