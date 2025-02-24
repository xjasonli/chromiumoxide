use super::*;

/// A trait for function argument tuples that can be converted from JavaScript values.
/// 
/// This is an internal trait used to implement function argument deserialization.
/// It is implemented for tuples of up to 10 elements, where each element implements
/// [`NativeValueFromJs`].
pub trait FromJsArgs: private::from_js::Sealed {}

macro_rules! impl_from_js_args {
    ($($($name:ident),+)?) => {
        impl$(<$($name: FromJsAny),+>)? FromJsArgs for ($($($name,)+)?) {}
        impl$(<$($name: FromJsAny),+>)? private::from_js::Sealed for ($($($name,)+)?) {}
    };
}

impl_from_js_args!();
impl_from_js_args!(A1);
impl_from_js_args!(A1, A2);
impl_from_js_args!(A1, A2, A3);
impl_from_js_args!(A1, A2, A3, A4);
impl_from_js_args!(A1, A2, A3, A4, A5);
impl_from_js_args!(A1, A2, A3, A4, A5, A6);
impl_from_js_args!(A1, A2, A3, A4, A5, A6, A7);
impl_from_js_args!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_from_js_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_from_js_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

/// A trait for function argument tuples that can be converted into JavaScript values.
/// 
/// This is an internal trait used to implement function argument serialization.
/// It is implemented for tuples of up to 10 elements, where each element implements
/// [`NativeValueIntoJs`].
pub trait IntoJsArgs<'a>: private::into_js::Sealed<'a> {}

macro_rules! impl_into_js_args {
    (
        $($($name:ident),+)?
    ) => {
        paste::paste!{
            impl<'a, $($($name),+)?> IntoJsArgs<'a> for ($($($name,)+)?)
            $(
                where
                    $(
                        $name: IntoJsAny + 'a,
                    )+
            )?
            {}

            impl<'a, $($($name),+)?> private::into_js::Sealed<'a> for ($($($name,)+)?)
            $(
                where
                    $(
                        $name: IntoJsAny + 'a,
                    )+
            )?
            {
                //fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error> {
                //    $(
                //        let ($([< $name:lower >],)+) = self;
                //    )?
                //    Ok(vec![
                //        $(
                //            $(
                //                serde_json::to_value([< $name:lower >])?,
                //            )+
                //        )?
                //    ])
                //}
                fn into_vec(self) -> Vec<DynIntoJsAny<'a>> {
                    $(
                        let ($([< $name:lower >],)+) = self;
                    )?
                    vec![
                        $(
                            $(
                                std::sync::Arc::new([< $name:lower >]),
                            )+
                        )?
                    ]
                }
            }
        }
    };
}

impl_into_js_args!();
impl_into_js_args!(A1);
impl_into_js_args!(A1, A2);
impl_into_js_args!(A1, A2, A3);
impl_into_js_args!(A1, A2, A3, A4);
impl_into_js_args!(A1, A2, A3, A4, A5);
impl_into_js_args!(A1, A2, A3, A4, A5, A6);
impl_into_js_args!(A1, A2, A3, A4, A5, A6, A7);
impl_into_js_args!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_into_js_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_into_js_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);


mod private {
    use super::*;

    pub mod from_js {
        pub trait Sealed: Send + Sync {}
    }
    pub mod into_js {
        use super::*;

        pub trait Sealed<'a>: Send + Sync {
            //fn into_json_values(self) -> Result<Vec<JsonValue>, serde_json::Error>;
            fn into_vec(self) -> Vec<DynIntoJsAny<'a>>;
        }
    }
}