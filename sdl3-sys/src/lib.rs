#![no_std]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(c_variadic))] // https://github.com/rust-lang/rust/issues/44930
#![cfg_attr(all(feature = "nightly", doc), feature(doc_auto_cfg))] // https://github.com/rust-lang/rust/issues/43781
#![cfg_attr(all(feature = "nightly", doc), feature(doc_cfg))] // https://github.com/rust-lang/rust/issues/43781
#![doc = include_str!("../README.md")]

extern crate self as sdl3_sys;

// This macro is used to apply a cfg attribute to multiple items
// e.g. `apply_cfg!(#[cfg(feature = "nightly")] => { type VaList = ::core::ffi::VaList; })`
macro_rules! apply_cfg {
    (#[cfg $cfg:tt] => { $($item:item)* }) => { $( #[cfg $cfg] $item )* };
}

// Get the size of a field of a struct or union
macro_rules! size_of_field {
    ($struct:ty, $field:ident) => {
        $crate::size_of_return_value(&|s: $struct| unsafe {
            // safety: this is never evaluated
            #[allow(deprecated)]
            s.$field
        })
    };
}

pub(crate) use size_of_field;

#[allow(unused)] // incorrectly detected as unused
const fn size_of_return_value<T, R>(_: &impl FnOnce(T) -> R) -> usize {
    size_of::<R>()
}

#[doc(hidden)] // for internal use only
#[macro_export]
macro_rules! __const_c_str {
    ($cstr:ident = $str:expr) => {
        const $cstr: [::core::ffi::c_char; $str.len() + 1] = {
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

#[doc(hidden)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonExhaustive(());

#[cfg(feature = "debug-impls")]
impl core::fmt::Debug for NonExhaustive {
    #[inline(always)]
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

const _: () = assert!(size_of::<NonExhaustive>() == 0);

mod generated;
pub use generated::*;

/// You can set a breakpoint on this function to break into the debugger when asserts
/// want to trigger a breakpoint.
///
/// If the `nightly` feature is enabled, this calls [`core::arch::breakpoint()`],
/// so you don't have to set a breakpoint manually in that case.
#[cold]
#[inline(never)]
pub fn breakpoint() {
    #[cfg(feature = "nightly")]
    core::arch::breakpoint();

    #[cfg(not(feature = "nightly"))]
    {
        // raise a double panic to abort (this is no-std so we can't use std::process::abort)
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!("breakpoint");
            }
        }
        let _abort = Abort;
        panic!("breakpoint");
    }
}

/// Extra ffi types for `sdl3-sys`
pub mod ffi {
    #[cfg(doc)]
    /// Equivalent to C's `wchar_t` type. This is `u16` on Windows and `u32` otherwise.
    /// Enable a `use-libc-*` feature to make this an alias of `libc::wchar_t`.
    pub type c_wchar_t = u32;
    #[cfg(all(not(doc), feature = "use-libc-v0-2"))]
    pub type c_wchar_t = ::libc_v0_2::wchar_t;
    #[cfg(all(not(any(doc, feature = "use-libc-v0-2")), windows))]
    pub type c_wchar_t = u16;
    #[cfg(all(not(any(doc, feature = "use-libc-v0-2")), not(windows)))]
    pub type c_wchar_t = u32;

    #[cfg(doc)]
    /// Equivalent to C's `va_list` type. Enable the `nightly` feature and compile with
    /// the nightly compiler to make this an alias of [`core::ffi::VaList`]. Otherwise,
    /// this type can't be instantiated.
    pub enum VaList {}
    #[cfg(all(not(doc), feature = "nightly"))]
    pub use core::ffi::VaList;
    #[cfg(all(not(doc), not(feature = "nightly")))]
    pub enum VaList {}
}

pub mod metadata;
