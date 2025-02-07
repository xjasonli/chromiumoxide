//! Traits for implementing JavaScript class inheritance in Rust.
//!
//! This module provides traits that allow modeling JavaScript's class-based inheritance
//! system in Rust. It enables type-safe downcasting and property access for JavaScript
//! objects exposed through the Chrome DevTools Protocol.

use std::borrow::Cow;
use super::*;

/// A trait for types that can be used as JavaScript class types.
///
/// This trait represents a type that maps to a JavaScript class and provides
/// access to its properties and methods. It is used as a base trait for all
/// JavaScript object types in the library.
///
/// # Type Parameters
/// * `T` - The parent class type this class inherits from
///
/// # Safety
/// This trait is sealed and can only be implemented by types in this crate
/// to ensure type safety of JavaScript class operations.
pub trait Class<T: ?Sized> : private::Sealed + NativeValueIntoJs {
    /// The owned version of this class type
    type Owned: Class<T> + NativeValueFromJs;
    
    /// Returns a reference to the parent class
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

/// A trait for types that can be used as JavaScript subclasses.
///
/// This trait extends `Class` to add support for type checking and downcasting,
/// similar to JavaScript's `instanceof` operator. It allows checking if a JavaScript
/// object is an instance of a particular class and safely converting between class types.
///
/// # Type Parameters
/// * `T` - The parent class type this class inherits from
///
/// # Example
/// ```no_run
/// # use chromiumoxide::js::{JsElement, JsNode};
/// # fn example(node: JsNode) {
/// // Check if node is an Element
/// if JsElement::is_instance(&node) {
///     // Safely downcast to Element
///     let element = JsElement::from_super(node);
///     // Use element-specific methods...
/// }
/// # }
/// ```
pub trait Subclass<T> : Class<T> {
    /// Checks if a value is an instance of this class
    fn is_instance(value: &T) -> bool;

    /// Converts a parent class value to this class type
    fn from_super(value: T) -> <Self as Class<T>>::Owned;

    /// Attempts to convert a parent class value to this class type
    ///
    /// Returns None if the value is not an instance of this class.
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

// Built-in implementations for string types

/// Implementation of `Class<str>` for `String`
impl private::Sealed for String {}
impl Class<str> for String {
    type Owned = Self;
    fn as_ref(&self) -> &str {
        String::as_str(self)
    }
}

/// Implementation of `Class<str>` for `str`
impl private::Sealed for str {}
impl Class<str> for str {
    type Owned = String;
    fn as_ref(&self) -> &str {
        self
    }
}

/// Implementation of `Class<str>` for `Cow<str>`
impl<'a> private::Sealed for Cow<'a, str> {}
impl<'a> Class<str> for Cow<'a, str> {
    type Owned = String;
    fn as_ref<'b>(&'b self) -> &'b str {
        AsRef::as_ref(self)
    }
}
