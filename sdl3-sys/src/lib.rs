#![no_std]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(c_variadic))]

// This macro is used to apply attributes (like cfg) to a group of items. Wrap
// the items in a call to this macro and apply the attributes to the macro call
macro_rules! emit {
    ($($tt:tt)*) => { $($tt)* };
}

// The calling convention for SDL on non-GNU Windows uses "cdecl" instead of "C"
#[cfg(all(target_family = "windows", not(target_env = "gnu")))]
macro_rules! extern_sdlcall { ($($tt:tt)*) => { extern "cdecl" $($tt)* }; }
#[cfg(not(all(target_family = "windows", not(target_env = "gnu"))))]
macro_rules! extern_sdlcall { ($($tt:tt)*) => { extern "C" $($tt)* }; }

pub mod generated;

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
