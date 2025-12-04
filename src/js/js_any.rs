/// A trait for types that can be converted from JavaScript values.
/// 
/// Types implementing this trait can be:
/// - Returned from JavaScript functions
/// - Read from JavaScript object properties
/// - Parsed from JavaScript expressions
/// 
/// The type must be:
/// - `DeserializeOwned`: Can be deserialized from JSON without borrowing
/// - `JsonSchema`: Used to determine how to handle special JavaScript values (like undefined, functions, etc.)
///    during type conversion, rather than for validation
/// - `Debug`: Can be formatted for debugging
/// - `Send + Sync`: Thread-safe
pub trait FromJsAny : serde::de::DeserializeOwned + schemars::JsonSchema
    + std::fmt::Debug + Send + Sync {}

/// Blanket implementation for types that implement `serde::de::DeserializeOwned + schemars::JsonSchema`
/// 
/// This implementation uses `()` as the type of the JavaScript value.
/// which means any rust type that implements `serde::de::DeserializeOwned + schemars::JsonSchema`
/// can be converted to a JavaScript value.
impl<T: serde::de::DeserializeOwned + schemars::JsonSchema> FromJsAny for T
where T: std::fmt::Debug + Send + Sync {}

/// A trait for types that can be converted into JavaScript values.
/// 
/// Types implementing this trait can be:
/// - Passed as arguments to JavaScript functions
/// - Set as property values in JavaScript objects
/// - Used in JavaScript expressions
/// 
/// The type must be:
/// - `Serialize`: Can be serialized into JSON
/// - `Debug`: Can be formatted for debugging
/// - `Send + Sync`: Thread-safe
pub trait IntoJsAny: serde::Serialize + std::fmt::Debug + Send + Sync {}

/// Blanket implementation for types that implement `serde::Serialize + std::fmt::Debug + Send + Sync`
/// 
/// This implementation uses `()` as the type of the JavaScript value.
/// which means any rust type that implements `serde::Serialize + std::fmt::Debug + Send + Sync`
/// can be converted to a JavaScript value.
impl<T: serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized> IntoJsAny for T {}

/// A trait for types that can be converted into JavaScript values.
/// 
/// This trait is implemented for all types that implement `serde::Serialize + std::fmt::Debug + Send + Sync`.
/// It provides an erased implementation of `serde::Serialize` for dynamic types.
pub trait IntoJsAnyErased : erased_serde::Serialize + std::fmt::Debug + Send + Sync {}
impl<T: erased_serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized> IntoJsAnyErased for T {}

// implement serde::Serialize for erased types
impl<'a> serde::Serialize for dyn IntoJsAnyErased + 'a {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let erased: &dyn erased_serde::Serialize = self;
        erased.serialize(serializer)
    }
}

/// A type alias for a dynamically typed `IntoJsAnyErased` reference.
/// 
/// This type allows for storing and passing around types that implement `IntoJsAnyErased`
/// in a type-erased manner. It is a thread-safe reference to a dynamically typed value
/// that can be converted to a JavaScript value.
pub type DynIntoJsAny<'a> = std::sync::Arc<dyn IntoJsAnyErased + 'a>;
