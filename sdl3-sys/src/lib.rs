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

#[allow(unused)] // incorrectly detected as unused
const fn size_of_return_value<T, R>(_: &impl FnOnce(T) -> R) -> usize {
    size_of::<R>()
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
