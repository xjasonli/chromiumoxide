use super::*;

pub trait Class<T: ?Sized> : private::Sealed + NativeValueIntoJs {
    type Owned: Class<T> + NativeValueFromJs;
    fn as_ref(&self) -> &T;
}

impl<'a, T: ?Sized, U: Class<T> + ?Sized> Class<T> for &'a U {
    type Owned = <U as Class<T>>::Owned;
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}
impl<'a, T: ?Sized, U: Class<T> + ?Sized> Class<T> for &'a mut U {
    type Owned = <U as Class<T>>::Owned;
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}


pub trait Subclass<T> : Class<T> {
    fn is_instance(value: &T) -> bool;
    fn from_super(value: T) -> <Self as Class<T>>::Owned;

    fn try_from_super(value: T) -> Option<<Self as Class<T>>::Owned> {
        if Self::is_instance(&value) {
            Some(Self::from_super(value))
        } else {
            None
        }
    }
}

impl<'a, T, U: Subclass<T>> Subclass<T> for &'a U {
    fn is_instance(value: &T) -> bool {
        U::is_instance(value)
    }
    fn from_super(value: T) -> <Self as Class<T>>::Owned {
        U::from_super(value)
    }
}
impl<'a, T, U: Subclass<T>> Subclass<T> for &'a mut U {
    fn is_instance(value: &T) -> bool {
        U::is_instance(value)
    }
    fn from_super(value: T) -> <Self as Class<T>>::Owned {
        U::from_super(value)
    }
}
