use super::*;

// JsString
macro_rules! impl_into_js {
    ($into:ty : $from:ty,  $ty:ty $(where $($where:tt)*)?) => {
        impl$(<$($where)*>)? IntoJs<$into> for $ty {
            type FromJs = $from;
        }
    };
}

impl_into_js!{str : String, String }
impl_into_js!{str : String, str }
impl_into_js!{str : String, std::borrow::Cow<'a, str> where 'a }
impl_into_js!{str : T::FromJs, &'a T where 'a, T: ?Sized + IntoJs<str> }
impl_into_js!{str : T::FromJs, &'a mut T where 'a, T: ?Sized + IntoJs<str> }

impl AsJs<str> for String {
    fn as_js(&self) -> &str { self }
}

impl AsJs<str> for str {
    fn as_js(&self) -> &str { self }
}

impl<'a> AsJs<str> for std::borrow::Cow<'a, str> {
    fn as_js(&self) -> &str { self.as_ref() }
}


