use std::borrow::Cow;

use super::*;

// implement Class<str> for String
impl private::Sealed for String {}
impl Class<str> for String {
    type Owned = Self;
    fn as_ref(&self) -> &str {
        String::as_str(self)
    }
}

// implement Class<str> for str
impl private::Sealed for str {}
impl Class<str> for str {
    type Owned = String;
    fn as_ref(&self) -> &str {
        self
    }
}

// implement Class<str> for Cow<'a, str>
impl<'a> private::Sealed for Cow<'a, str> {}
impl<'a> Class<str> for Cow<'a, str> {
    type Owned = String;
    fn as_ref<'b>(&'b self) -> &'b str {
        AsRef::as_ref(self)
    }
}
