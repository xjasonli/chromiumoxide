use serde_json::Value as JsonValue;

pub trait NativeValue: NativeValueIntoJs + NativeValueFromJs {}

impl<T> NativeValue for T
where T
    : NativeValueIntoJs + NativeValueFromJs
{}

pub trait NativeValueIntoJs
    : serde::Serialize
    + std::fmt::Debug + Send + Sync {}

impl<T> NativeValueIntoJs for T
where T
    : serde::Serialize
    + std::fmt::Debug + Send + Sync + ?Sized
{}

pub trait NativeValueFromJs
    : serde::de::DeserializeOwned + schemars::JsonSchema
    + std::fmt::Debug + Send + Sync {}

impl<T> NativeValueFromJs for T
where T
    : serde::de::DeserializeOwned + schemars::JsonSchema
    + std::fmt::Debug + Send + Sync
{}

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
