/// A trait for types that can be converted to and from JavaScript values.
/// 
/// This is a convenience trait that combines [`NativeValueIntoJs`] and [`NativeValueFromJs`].
/// Types implementing this trait can be:
/// - Passed as arguments to JavaScript functions
/// - Returned from JavaScript functions
/// - Used as property values in JavaScript objects
pub trait NativeValue: IntoJs + FromJs {}
impl<T: IntoJs + FromJs> NativeValue for T {}

/// A marker type to indicate `any` JavaScript value.
/// 
/// This type can be used to indicate that a type can be converted to any JavaScript value.
/// It is used as the default type for the `IntoJs` trait.
/// 
/// This type ONLY implements `FromJs`, not `IntoJs`.
/// When used as the return type of `eval` or `invoke`, it will discard the return value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct JsAny;

//pub trait IntoJsValue<T: FromJs = JsAny>: serde::Serialize + std::fmt::Debug + Send + Sync {
//    type FromJs: FromJs;
//    fn into_js(&self) -> T;
//}
//
//impl<T> IntoJsValue<JsAny> for T
//where T: serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized {
//    type FromJs = JsAny;
//    fn into_js(&self) -> JsAny { JsAny }
//}

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
pub trait IntoJs<T: ?Sized = JsAny> : serde::Serialize + std::fmt::Debug + Send + Sync {
    type FromJs: FromJs;
}

/// Blanket implementation for types that implement `serde::Serialize + std::fmt::Debug + Send + Sync`
/// 
/// This implementation uses `JsAny` as the type of the JavaScript value.
/// which means any rust type that implements `serde::Serialize + std::fmt::Debug + Send + Sync`
/// can be converted to a JavaScript value.
impl<T: serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized> IntoJs for T {
    type FromJs = JsAny;
}

pub trait ErasedIntoJs: erased_serde::Serialize + std::fmt::Debug + Send + Sync {}
impl<T: erased_serde::Serialize + std::fmt::Debug + Send + Sync + ?Sized> ErasedIntoJs for T {}

pub type BoxedIntoJs<'a> = Box<dyn ErasedIntoJs + 'a>;
impl<'a> serde::Serialize for dyn ErasedIntoJs + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let erased: &dyn erased_serde::Serialize = self;
        erased.serialize(serializer)
    }
}

/// A trait for types that can be converted to a JavaScript T value reference.
/// 
/// This trait is implemented for references to types that implement `IntoJs<T>`.
/// 
/// The type must be:
/// - `IntoJs<T>`: Can be converted to a JavaScript value
/// - `Debug`: Can be formatted for debugging
/// - `Send + Sync`: Thread-safe
pub trait AsJs<T: ?Sized>: IntoJs<T> {
    fn as_js(&self) -> &T;
}

/// Blanket implementation for references to types that implement `IntoJs<T>`.
/// 
/// This implementation uses the `as_js` method of the type to get the JavaScript value.
impl<'a, T: ?Sized, U: AsJs<T> + ?Sized> AsJs<T> for &'a U
where &'a U: IntoJs<T> {
    fn as_js(&self) -> &T { (**self).as_js() }
}

/// Blanket implementation for mutable references to types that implement `IntoJs<T>`.
/// 
/// This implementation uses the `as_js` method of the type to get the JavaScript value.
impl<'a, T: ?Sized, U: AsJs<T> + ?Sized> AsJs<T> for &'a mut U
where &'a mut U: IntoJs<T> {
    fn as_js(&self) -> &T { (**self).as_js() }
}

/// A trait for types that can be derived from a base type.
/// 
/// This trait is implemented for types that can be converted to a JavaScript value
/// and for which a base type can be derived.
pub trait DerivedJs<BaseT>: AsJs<BaseT> {
    /// Checks whether the given base value is an instance of this type
    fn is_instance(base: &BaseT) -> bool;

    /// Downcasts the given base value to this type without checking
    fn downcast_from_unchecked(base: BaseT) -> Self::FromJs;

    /// Downcasts the given base value to this type if it is an instance of this type
    fn downcast_from(base: BaseT) -> Option<Self::FromJs> {
        if Self::is_instance(&base) {
            Some(Self::downcast_from_unchecked(base))
        } else {
            None
        }
    }
}

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
pub trait FromJs : serde::de::DeserializeOwned + schemars::JsonSchema
    + std::fmt::Debug + Send + Sync {}
impl<T: serde::de::DeserializeOwned + schemars::JsonSchema> FromJs for T
where T: std::fmt::Debug + Send + Sync {}


/// A trait for function argument tuples that can be converted from JavaScript values.
/// 
/// This is an internal trait used to implement function argument deserialization.
/// It is implemented for tuples of up to 10 elements, where each element implements
/// [`NativeValueFromJs`].
pub trait FromJsArgs: private::from_js::Sealed {}

macro_rules! impl_from_js_args {
    ($($($name:ident),+)?) => {
        impl$(<$($name: FromJs),+>)? FromJsArgs for ($($($name,)+)?) {}
        impl$(<$($name: FromJs),+>)? private::from_js::Sealed for ($($($name,)+)?) {}
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
                        $name: IntoJs + 'a,
                    )+
            )?
            {}

            impl<'a, $($($name),+)?> private::into_js::Sealed<'a> for ($($($name,)+)?)
            $(
                where
                    $(
                        $name: IntoJs + 'a,
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
                fn into_vec(self) -> Vec<BoxedIntoJs<'a>> {
                    $(
                        let ($([< $name:lower >],)+) = self;
                    )?
                    vec![
                        $(
                            $(
                                Box::new([< $name:lower >]),
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
            fn into_vec(self) -> Vec<BoxedIntoJs<'a>>;
        }
    }
}
