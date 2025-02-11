use serde_json::Value as JsonValue;

/// A trait for types that can be converted to and from JavaScript values.
/// 
/// This is a convenience trait that combines [`NativeValueIntoJs`] and [`NativeValueFromJs`].
/// Types implementing this trait can be:
/// - Passed as arguments to JavaScript functions
/// - Returned from JavaScript functions
/// - Used as property values in JavaScript objects
pub trait NativeValue: NativeValueIntoJs + NativeValueFromJs {}
impl<T: NativeValueIntoJs + NativeValueFromJs> NativeValue for T {}

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
pub trait NativeValueIntoJs : serde::Serialize + std::fmt::Debug + Send + Sync {}
impl<T> NativeValueIntoJs for T
where T : serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized {}

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
pub trait NativeValueFromJs : serde::de::DeserializeOwned + schemars::JsonSchema
    + std::fmt::Debug + Send + Sync {}
impl<T: serde::de::DeserializeOwned + schemars::JsonSchema> NativeValueFromJs for T
where T: std::fmt::Debug + Send + Sync {}


/// A trait for function argument tuples that can be converted from JavaScript values.
/// 
/// This is an internal trait used to implement function argument deserialization.
/// It is implemented for tuples of up to 10 elements, where each element implements
/// [`NativeValueFromJs`].
pub trait FunctionNativeArgsFromJs: private::from_js::Sealed {}

macro_rules! impl_function_native_args_from_js {
    ($($($name:ident),+)?) => {
        impl$(<$($name: NativeValueFromJs),+>)? FunctionNativeArgsFromJs for ($($($name,)+)?) {}
        impl$(<$($name: NativeValueFromJs),+>)? private::from_js::Sealed for ($($($name,)+)?) {}
    };
}

impl_function_native_args_from_js!();
impl_function_native_args_from_js!(A1);
impl_function_native_args_from_js!(A1, A2);
impl_function_native_args_from_js!(A1, A2, A3);
impl_function_native_args_from_js!(A1, A2, A3, A4);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5, A6);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5, A6, A7);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_function_native_args_from_js!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

/// A trait for function argument tuples that can be converted into JavaScript values.
/// 
/// This is an internal trait used to implement function argument serialization.
/// It is implemented for tuples of up to 10 elements, where each element implements
/// [`NativeValueIntoJs`].
pub trait FunctionNativeArgsIntoJs: private::into_js::Sealed {}

macro_rules! impl_function_native_args_into_js {
    ($($($name:ident),+)?) => {
        paste::paste!{
            impl$(<$($name: NativeValueIntoJs),+>)? FunctionNativeArgsIntoJs for ($($($name,)+)?) {}
            impl$(<$($name: NativeValueIntoJs),+>)? private::into_js::Sealed for ($($($name,)+)?) {
                fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
                    $(
                        let ($([< $name:lower >],)+) = self;
                    )?
                    Ok(vec![
                        $(
                            $(
                                serde_json::to_value([< $name:lower >])?,
                            )+
                        )?
                    ])
                }
            }
        }
    };
}

impl_function_native_args_into_js!();
impl_function_native_args_into_js!(A1);
impl_function_native_args_into_js!(A1, A2);
impl_function_native_args_into_js!(A1, A2, A3);
impl_function_native_args_into_js!(A1, A2, A3, A4);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5, A6);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5, A6, A7);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_function_native_args_into_js!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

mod private {
    use super::*;

    pub mod from_js {
        pub trait Sealed: Send + Sync {}
    }
    pub mod into_js {
        use super::*;

        pub trait Sealed: Send + Sync {
            fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error>;
        }
    }
}
