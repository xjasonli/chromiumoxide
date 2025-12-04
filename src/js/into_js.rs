use std::collections::VecDeque;
use super::*;

pub trait JsType: IntoJsAny + FromJsAny {}
impl<T: IntoJsAny + FromJsAny> JsType for T {}

pub trait IntoJs<T: JsType>: IntoJsAny {
    type FromJs: IntoJs<T> + FromJsAny;
}

macro_rules! impl_into_js {
    (
        $(<$($generics:tt),+>)?
        IntoJs<$js_type:ty> for $ty:ty => $from:ty
        $(where $($where:tt)+)?
    ) => {
        impl_into_js!{@impl
            $(<$($generics),+>)?
            IntoJs<$js_type> for $ty => $from
            $(where $($where)+)?
        }
        impl_into_js!{@impl
            $(<$($generics),+>)?
            IntoJs<Optional<$js_type>> for $ty => Optional<$from>
            $(where $($where)+)?
        }
        impl_into_js!{@impl
            $(<$($generics),+>)?
            IntoJs<Nullable<$js_type>> for $ty => Nullable<$from>
            $(where $($where)+)?
        }
        impl_into_js!{@impl
            $(<$($generics),+>)?
            IntoJs<Optional<Nullable<$js_type>>> for $ty => Optional<Nullable<$from>>
            $(where $($where)+)?
        }
        impl_into_js!{@impl
            $(<$($generics),+>)?
            IntoJs<Nullable<Optional<$js_type>>> for $ty => Nullable<Optional<$from>>
            $(where $($where)+)?
        }
    };

    (
        $(<$($generics:tt),+>)?
        IntoJs<$js_type:ty> for $ty:ty
        $(where $($where:tt)+)?
    ) => {
        impl_into_js!{
            $(<$($generics),+>)?
            IntoJs<$js_type> for $ty => $js_type
            $(where $($where)+)?
        }
    };


    (
        $ty:ty
    ) => {
        impl_into_js!{
            IntoJs<$ty> for $ty
        }
    };

    (@impl
        $(<$($generics:tt),+>)?
        IntoJs<$js_type:ty> for $ty:ty => $from:ty
        $(where $($where:tt)+)?
    ) => {
        impl$(<$($generics),+>)? IntoJs<$js_type> for $ty
        $(where
            $($where)+
        )?
        {
            type FromJs = $from;
        }
    };
}
pub(crate) use impl_into_js;

// implement IntoJs<T> for &'a U
impl_into_js!{@impl
    <'a, T, U>
    IntoJs<T> for &'a U => U::FromJs
    where
        T: JsType,
        U: ?Sized + IntoJs<T>
}

// implement IntoJs<T> for &'a mut U
impl_into_js!{@impl
    <'a, T, U>
    IntoJs<T> for &'a mut U => U::FromJs
    where
        T: JsType,
        U: ?Sized + IntoJs<T>
}

/*
 * Javascript String representation
 */
impl_into_js!(String);

// implement IntoJs<String> for str
impl_into_js!(IntoJs<String> for str);

// implement IntoJs<String> for Cow<'a, str>
impl_into_js!{
    <'a>
    IntoJs<String> for std::borrow::Cow<'a, str>
}

/*
 * Javascript Boolean representation
 */
impl_into_js!(bool);

/*
 * Javascript Number representation
 */
impl_into_js!(f64);

// implement IntoJs<f64> for f32
impl_into_js!(IntoJs<f64> for f32);

// implement IntoJs<f64> for i64
impl_into_js!(IntoJs<f64> for i64);

// implement IntoJs<f64> for u64
impl_into_js!(IntoJs<f64> for u64);

// implement IntoJs<f64> for i32
impl_into_js!(IntoJs<f64> for i32);

// implement IntoJs<f64> for u32
impl_into_js!(IntoJs<f64> for u32);

// implement IntoJs<f64> for i16
impl_into_js!(IntoJs<f64> for i16);

// implement IntoJs<f64> for u16
impl_into_js!(IntoJs<f64> for u16);

// implement IntoJs<f64> for i8
impl_into_js!(IntoJs<f64> for i8);

// implement IntoJs<f64> for u8
impl_into_js!(IntoJs<f64> for u8);

// implement IntoJs<f64> for usize
impl_into_js!(IntoJs<f64> for usize);

// implement IntoJs<f64> for isize
impl_into_js!(IntoJs<f64> for isize);

/*
 * Javascript signed integer representation
 */
impl_into_js!(i64);

// implement IntoJs<i64> for i32
impl_into_js!(IntoJs<i64> for i32);

// implement IntoJs<i64> for i16
impl_into_js!(IntoJs<i64> for i16);

// implement IntoJs<i64> for i8
impl_into_js!(IntoJs<i64> for i8);

// implement IntoJs<i64> for u64
impl_into_js!(IntoJs<i64> for u64);

// implement IntoJs<i64> for u32
impl_into_js!(IntoJs<i64> for u32);

// implement IntoJs<i64> for u16
impl_into_js!(IntoJs<i64> for u16);

// implement IntoJs<i64> for u8
impl_into_js!(IntoJs<i64> for u8);

// implement IntoJs<i64> for usize
impl_into_js!(IntoJs<i64> for usize);

// implement IntoJs<i64> for isize
impl_into_js!(IntoJs<i64> for isize);

/*
 * Javascript unsigned integer representation
 */
impl_into_js!(u64);

// implement IntoJs<u64> for u32
impl_into_js!(IntoJs<u64> for u32);

// implement IntoJs<u64> for u16
impl_into_js!(IntoJs<u64> for u16);

// implement IntoJs<u64> for u8
impl_into_js!(IntoJs<u64> for u8);

// implement IntoJs<u64> for usize
impl_into_js!(IntoJs<u64> for usize);

// implement IntoJs<u64> for isize
impl_into_js!(IntoJs<u64> for isize);

/*
 * Javascript bigint representation
 */
impl_into_js!(JsBigInt);

/*
 * Javascript Array representation
 */
impl_into_js!{
    <T>
    IntoJs<JsArray> for Vec<T>
    where
        T: IntoJsAny
}

impl_into_js!{
    <T>
    IntoJs<JsArray> for VecDeque<T>
    where
        T: IntoJsAny
}

impl_into_js!{
    <T>
    IntoJs<JsArray> for [T]
    where
        T: IntoJsAny
}

macro_rules! impl_into_js_array {
    (
        $len:literal =>
        {
            $(<$($generics:tt),*>)?
            IntoJs<$js_type:ty> for [$ty:ty; _] => $from:ty
            $(where $($where:tt)*)?
        }
    ) => {
        impl_into_js!{
            $(<$($generics),*>)?
            IntoJs<$js_type> for [$ty; $len] => $from
            $(where $($where)*)?
        }
    };
    (
        $($lens:literal)+ =>
        $tt:tt
    ) => {
        $(
            impl_into_js_array!{
                $lens =>
                $tt
            }
        )+
    }
}

impl_into_js_array!{
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32 => {
        <T>
        IntoJs<JsArray> for [T; _] => JsArray
        where
            T: JsType
    }
}

// IntoJs<Vec<T>> for Vec<U>
impl_into_js!{
    <T, U>
    IntoJs<Vec<T>> for Vec<U> => Vec<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{
    <T, U>
    IntoJs<Vec<T>> for VecDeque<U> => Vec<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{
    <T, U>
    IntoJs<Vec<T>> for [U] => Vec<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js_array!{
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32 => {
        <T, U>
        IntoJs<Vec<T>> for [U; _] => Vec<U::FromJs>
        where
            T: JsType + IntoJs<T>,
            U: IntoJs<T>
    }
}

impl_into_js!{
    <T>
    IntoJs<Vec<T>> for JsArray => Vec<T::FromJs>
    where
        T: JsType + IntoJs<T>
}

// IntoJs<VecDeque<T>> for VecDeque<U>
impl_into_js!{
    <T, U>
    IntoJs<VecDeque<T>> for VecDeque<U> => VecDeque<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{
    <T, U>
    IntoJs<VecDeque<T>> for Vec<U> => VecDeque<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{
    <T, U>
    IntoJs<VecDeque<T>> for [U] => VecDeque<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js_array!{
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32 => {
        <T, U>
        IntoJs<VecDeque<T>> for [U; _] => VecDeque<U::FromJs>
        where
            T: JsType + IntoJs<T>,
            U: IntoJs<T>
    }
}

impl_into_js!{
    <T>
    IntoJs<VecDeque<T>> for JsArray => VecDeque<T::FromJs>
    where
        T: JsType + IntoJs<T>
}


/*
 * Javascript Optional representation
 */
impl_into_js!{@impl
    <T, U>
    IntoJs<Optional<T>> for Optional<U> => Optional<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{@impl
    <T>
    IntoJs<Optional<T>> for JsUndefined => Optional<T::FromJs>
    where
        T: FromJsAny + IntoJs<T>
}

/*
 * Javascript Nullable representation
 */
impl_into_js!{@impl
    <T, U>
    IntoJs<Nullable<T>> for Nullable<U> => Nullable<U::FromJs>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{@impl
    <T>
    IntoJs<Nullable<T>> for JsNull => Nullable<T::FromJs>
    where
        T: FromJsAny + IntoJs<T>
}

/*
 * Javascript Optional<Nullable<T>> representation
 */
impl_into_js!{@impl
    <T, U>
    IntoJs<Optional<Nullable<T>>> for Nullable<U> => Optional<Nullable<U::FromJs>>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}
impl_into_js!{@impl
    <T>
    IntoJs<Optional<Nullable<T>>> for JsNull => Optional<Nullable<T::FromJs>>
    where
        T: FromJsAny + IntoJs<T>
}

/*
 * Javascript Nullable<Optional<T>> representation
 */
impl_into_js!{@impl
    <T, U>
    IntoJs<Nullable<Optional<T>>> for Optional<U> => Nullable<Optional<U::FromJs>>
    where
        T: JsType + IntoJs<T>,
        U: IntoJs<T>
}

impl_into_js!{@impl
    <T>
    IntoJs<Nullable<Optional<T>>> for JsUndefined => Nullable<Optional<T::FromJs>>
    where
        T: FromJsAny + IntoJs<T>
}


#[cfg(test)]
mod test {
    #![allow(unused)]

    use super::*;

    fn string<T: IntoJs<String>>(t: T) -> T::FromJs {
        todo!()
    }
    fn optional_nullable<T: IntoJs<Optional<Nullable<JsNode>>>>(t: T) -> T::FromJs {
        todo!()
    }

    fn optional<T: IntoJs<Optional<JsNode>>>(t: T) -> T::FromJs {
        todo!()
    }

    fn nullable<T: IntoJs<Nullable<JsNode>>>(t: T) -> T::FromJs {
        todo!()
    }

    /// Test type inference
    /// 
    /// This function tests the type inference of the `IntoJs` trait.
    fn test_type_inference(n: JsHtmlElement) {
        let x = string(&String::from("hello"));
        let x = string(String::from("hello"));
        let x = string("hello");
        let x = string(std::borrow::Cow::Borrowed("hello"));
        let x = string(&std::borrow::Cow::Borrowed("hello"));

        let x = optional_nullable(&Optional::Some(&n));
        let x = optional_nullable(&Optional::Some(n.clone()));
        let x = optional_nullable(Nullable::Some(n.clone()));
        let x = optional_nullable(&n);
        let x = optional_nullable(JsUndefined);
        let x = optional_nullable(JsNull);

        let x = optional(n.clone());
        let x = optional(&n);
        let x = optional(Optional::Some(&n));
        let x = optional(Optional::Some(n.clone()));
        let x = optional(JsUndefined);
        //let x = optional(JsNull);

        let x = nullable(n.clone());
        let x = nullable(&n);
        let x = nullable(Nullable::Some(&n));
        let x = nullable(JsNull);
    }
}
