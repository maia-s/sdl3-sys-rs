#![no_std]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(c_variadic))]
#![doc = include_str!("../README.md")]

use core::mem::size_of;

// This macro is used to apply cfg attributes to a group of items. Wrap the items
// in a call to this macro and apply the attributes to the macro call
macro_rules! emit {
    ($($tt:tt)*) => { $($tt)* };
}

// Get the size of a field of a struct or union
macro_rules! size_of_field {
    ($struct:ty, $field:ident) => {
        $crate::size_of_return_value(&|s: $struct| unsafe {
            // safety: this is never evaluated
            s.$field
        })
    };
}
pub(crate) use size_of_field;

#[allow(unused)] // incorrectly detected as unused
const fn size_of_return_value<T, R>(_: &impl FnOnce(T) -> R) -> usize {
    size_of::<R>()
}

macro_rules! ptr_read_field {
    ($ptr:expr, $struct:ty, $field:ident, $field_ty:ty) => {{
        let ptr: *const _ = $ptr;
        ptr.cast::<u8>()
            .add(::core::mem::offset_of!($struct, $field))
            .cast::<$field_ty>()
            .read()
    }};
}
pub(crate) use ptr_read_field;

macro_rules! ptr_write_field {
    ($ptr:expr, $struct:ty, $field:ident, $field_ty:ty, $value:expr) => {{
        let (ptr, value): (*mut _, $field_ty) = ($ptr, $value);
        ptr.cast::<u8>()
            .add(::core::mem::offset_of!($struct, $field))
            .cast::<$field_ty>()
            .write(value);
        value
    }};
}
pub(crate) use ptr_write_field;

#[doc(hidden)] // for internal use only
#[macro_export]
macro_rules! __static_c_str {
    ($cstr:ident = $str:expr) => {
        static $cstr: [::core::ffi::c_char; $str.len() + 1] = {
            const BYTES: &[::core::primitive::u8] = $str.as_bytes();
            let mut cstr = [0 as ::core::ffi::c_char; BYTES.len() + 1];
            let mut i = 0;
            while i < BYTES.len() {
                assert!(BYTES[i] != 0, "zero byte in string");
                cstr[i] = BYTES[i] as ::core::ffi::c_char;
                i += 1;
            }
            cstr
        };
    };
}

mod generated;
pub use generated::*;

pub mod ffi {
    #[cfg(windows)]
    pub type c_wchar_t = u16;
    #[cfg(not(windows))]
    pub type c_wchar_t = u32;

    #[cfg(feature = "nightly")]
    pub use core::ffi::VaList;
    #[cfg(not(feature = "nightly"))]
    pub enum VaList {}
}

pub(crate) mod sealed_interface {
    pub trait Sealed {}
}

/// Implemented for SDL interface types
///
/// # Safety
/// Only implement this for types that conform to the SDL interface spec:
/// - Must be a `repr(C)` struct
/// - The first field of the struct must be of type `u32` and contain the size
///   of the struct in bytes (typically named `version`)
/// - The rest of the struct must be valid when initialized as all zero bytes
pub unsafe trait Interface: sealed_interface::Sealed + Sized {}
