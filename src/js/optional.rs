use serde::{Serialize, Serializer, Deserialize, Deserializer};
use super::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(schemars::JsonSchema)]
#[schemars(untagged)]
pub enum Optional<T> {
    #[schemars(with = "JsUndefined")]
    #[default]
    Undefined,

    Value(T),
}

pub use Optional::Undefined;
pub use Optional::Value;

impl<T: Serialize> Serialize for Optional<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value(value) => {
                value.serialize(serializer)
            }
            Undefined => {
                JsUndefined.serialize(serializer)
            }
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Optional<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::IntoDeserializer as _;

        fn de<'de, T, F, D, E>(deserializer: F) -> Result<Optional<T>, E>
        where
            T: Deserialize<'de>,
            F: Fn() -> D,
            D: Deserializer<'de>,
            E: serde::de::Error,
        {
            if let Ok(_) = JsUndefined::deserialize(deserializer()) {
                return Ok(Undefined);
            }
            if let Ok(value) = T::deserialize(deserializer()) {
                return Ok(Value(value));
            }
            Err(E::custom("data did not match Optional"))
        }

        let page = de::PageDeserializer::get(&deserializer);
        let content = serde_content::Value::deserialize(deserializer)?;
        if let Some(page) = page {
            de(||
                de::PageDeserializer::new(
                    content.clone()
                        .into_deserializer()
                        .human_readable(),
                    page.clone(),
                )
            )
        } else {
            de(||
                content.clone()
                    .into_deserializer()
                    .human_readable()
            )
        }
    }
}

impl<T> Optional<T> {
    /// Returns true if the optional contains a value
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Returns true if the optional is undefined
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// Returns a reference to the contained value as a new Optional
    pub fn as_ref(&self) -> Optional<&T> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => Optional::Value(t),
        }
    }

    /// Returns a mutable reference to the contained value as a new Optional
    pub fn as_mut(&mut self) -> Optional<&mut T> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => Optional::Value(t),
        }
    }

    /// Converts to Option<&T>
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Undefined => None,
            Self::Value(t) => Some(t),
        }
    }

    /// Converts to Option<T>
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Undefined => None,
            Self::Value(t) => Some(t),
        }
    }

    /// Returns the contained value or a default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Undefined => default,
            Self::Value(t) => t,
        }
    }

    /// Returns the contained value or computes it from a closure
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::Undefined => f(),
            Self::Value(t) => t,
        }
    }

    /// Returns the contained value or panics
    pub fn unwrap(self) -> T {
        match self {
            Self::Undefined => panic!("called `Optional::unwrap()` on an `Undefined` value"),
            Self::Value(t) => t,
        }
    }

    /// Maps an Optional<T> to Optional<U> by applying a function to the contained value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Optional<U> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => Optional::Value(f(t)),
        }
    }

    /// Maps an Optional<T> to Optional<U> by applying a function to a reference of the contained value
    pub fn map_ref<U, F: FnOnce(&T) -> U>(&self, f: F) -> Optional<U> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => Optional::Value(f(t)),
        }
    }

    /// Returns None if the optional is None, otherwise calls f with the wrapped value and returns the result
    pub fn and_then<U, F: FnOnce(T) -> Optional<U>>(self, f: F) -> Optional<U> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => f(t),
        }
    }

    /// Returns the optional if it contains a value, otherwise returns other
    pub fn or(self, other: Optional<T>) -> Optional<T> {
        match self {
            Self::Undefined => other,
            Self::Value(_) => self,
        }
    }

    /// Returns the optional if it contains a value, otherwise calls f and returns the result
    pub fn or_else<F: FnOnce() -> Optional<T>>(self, f: F) -> Optional<T> {
        match self {
            Self::Undefined => f(),
            Self::Value(_) => self,
        }
    }

    /// Returns Optional::Value if the option contains a value that matches the predicate
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Optional<T> {
        match self {
            Self::Undefined => Optional::Undefined,
            Self::Value(t) => if predicate(&t) {
                Optional::Value(t)
            } else {
                Optional::Undefined
            },
        }
    }

    /// Transforms the Optional<T> into a Result<T, E>
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Self::Undefined => Err(err),
            Self::Value(t) => Ok(t),
        }
    }

    /// Transforms the Optional<T> into a Result<T, E> by mapping the error
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Self::Undefined => Err(err()),
            Self::Value(t) => Ok(t),
        }
    }

    /// Returns the contained value or panics with a custom message
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Undefined => panic!("{}", msg),
            Self::Value(t) => t,
        }
    }

    /// Zips two optionals together into an optional tuple
    pub fn zip<U>(self, other: Optional<U>) -> Optional<(T, U)> {
        match (self, other) {
            (Self::Value(t), Optional::Value(u)) => Optional::Value((t, u)),
            _ => Optional::Undefined,
        }
    }
}
