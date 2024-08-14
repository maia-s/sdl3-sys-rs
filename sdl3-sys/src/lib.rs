#![no_std]

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
