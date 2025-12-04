//! Rust representation of JavaScript's `null` value.
//! 
//! This module provides a type alias for `Option<T>` called `Nullable<T>`.


#[allow(non_upper_case_globals)]
/// Represents JavaScript's `null` value.
/// 
/// This type is used to handle JavaScript's nullable values in Rust.
/// 
/// # Example
/// ```no_run
/// use chromiumoxide::js::JsNull;
/// 
/// let null: JsNull = JsNull;
/// ```
pub const JsNull: JsNull = ();

/// Represents JavaScript's `null` value.
/// 
/// This type is used to handle JavaScript's nullable values in Rust.
/// 
/// # Example
/// ```no_run
/// use chromiumoxide::js::JsNull;
/// 
/// let null: JsNull = JsNull;
/// ```
pub type JsNull = ();

/// Represents a value that can be either a value of type `T` or JavaScript's `null`.
/// 
/// This type is used to handle JavaScript's nullable values in Rust.
/// 
/// # Example
/// ```no_run
/// use chromiumoxide::js::Nullable;
/// 
/// let value: Nullable<i32> = Nullable::Some(42);
/// let null: Nullable<i32> = Nullable::None;
/// ```
pub type Nullable<T = ()> = Option<T>;
