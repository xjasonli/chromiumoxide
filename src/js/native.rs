use serde_json::Value as JsonValue;

/// A trait for Rust types that can be serialized into JavaScript values.
/// 
/// This trait is automatically implemented for all types that implement `serde::Serialize`.
/// It is used to convert Rust values into JavaScript values that can be used in the browser.
pub trait NativeValueIntoJs: serde::Serialize {}
impl<T: serde::Serialize> NativeValueIntoJs for T {}

/// A trait for Rust types that can be deserialized from JavaScript values.
/// 
/// This trait is automatically implemented for all types that implement both
/// `serde::de::DeserializeOwned` and `schemars::JsonSchema`. It is used to convert
/// JavaScript values received from the browser into Rust types.
pub trait NativeValueFromJs: serde::de::DeserializeOwned + schemars::JsonSchema {}
impl<T: serde::de::DeserializeOwned + schemars::JsonSchema> NativeValueFromJs for T {}

/// A trait for Rust types that can be used as callback function arguments.
/// 
/// This trait defines which types can be used as arguments for callback functions
/// that are called from JavaScript. It provides implementations for tuples of
/// different sizes, supporting up to 10 arguments.
pub trait CallbackNativeArgs: private::callback::Sealed {}

macro_rules! impl_callback_native_args {
    ($($($name:ident),+)?) => {
        impl$(<$($name: NativeValueFromJs),+>)? CallbackNativeArgs for ($($($name,)+)?) {}
        impl$(<$($name: NativeValueFromJs),+>)? private::callback::Sealed for ($($($name,)+)?) {}
    };
}

impl_callback_native_args!();
impl_callback_native_args!(A1);
impl_callback_native_args!(A1, A2);
impl_callback_native_args!(A1, A2, A3);
impl_callback_native_args!(A1, A2, A3, A4);
impl_callback_native_args!(A1, A2, A3, A4, A5);
impl_callback_native_args!(A1, A2, A3, A4, A5, A6);
impl_callback_native_args!(A1, A2, A3, A4, A5, A6, A7);
impl_callback_native_args!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_callback_native_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_callback_native_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

/// A trait for function arguments that can be converted into a list of JSON values.
/// 
/// This trait is used to convert Rust function arguments into a list of arguments
/// that can be passed to JavaScript functions. It provides implementations for
/// tuples of different sizes, supporting up to 10 arguments.
pub trait FunctionNativeArgs: private::function::Sealed {
    fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error>;
}

macro_rules! impl_function_native_args {
    ($($($name:ident),+)?) => {
        paste::paste!{
            impl$(<$($name: NativeValueIntoJs),+>)? FunctionNativeArgs for ($($($name,)+)?) {
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
            impl$(<$($name: NativeValueIntoJs),+>)? private::function::Sealed for ($($($name,)+)?) {}
        }
    };
}

impl_function_native_args!();
impl_function_native_args!(A1);
impl_function_native_args!(A1, A2);
impl_function_native_args!(A1, A2, A3);
impl_function_native_args!(A1, A2, A3, A4);
impl_function_native_args!(A1, A2, A3, A4, A5);
impl_function_native_args!(A1, A2, A3, A4, A5, A6);
impl_function_native_args!(A1, A2, A3, A4, A5, A6, A7);
impl_function_native_args!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_function_native_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_function_native_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);


mod private {
    pub mod callback {
        pub trait Sealed {}
    }
    pub mod function {
        pub trait Sealed {}
    }
}
