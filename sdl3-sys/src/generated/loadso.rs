//! System-dependent library loading routines.
//!
//! Shared objects are code that is programmatically loadable at runtime.
//! Windows calls these "DLLs", Linux calls them "shared libraries", etc.
//!
//! To use them, build such a library, then call [`SDL_LoadObject()`] on it. Once
//! loaded, you can use [`SDL_LoadFunction()`] on that object to find the address
//! of its exported symbols. When done with the object, call [`SDL_UnloadObject()`]
//! to dispose of it.
//!
//! Some things to keep in mind:
//!
//! - These functions only work on C function names. Other languages may have
//!   name mangling and intrinsic language support that varies from compiler to
//!   compiler.
//! - Make sure you declare your function pointers with the same calling
//!   convention as the actual library function. Your code will crash
//!   mysteriously if you do not do this.
//! - Avoid namespace collisions. If you load a symbol from the library, it is
//!   not defined whether or not it goes into the global symbol namespace for
//!   the application. If it does and it conflicts with symbols in your code or
//!   other shared libraries, you will not get the results you expect. :)

use super::stdinc::*;

use super::error::*;

extern "C" {
    /// Dynamically load a shared object.
    ///
    /// ### Parameters
    /// - `sofile`: a system-dependent name of the object file.
    ///
    /// ### Return value
    /// Returns an opaque pointer to the object handle or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LoadFunction`]
    /// - [`SDL_UnloadObject`]
    pub fn SDL_LoadObject(sofile: *const ::core::ffi::c_char) -> *mut SDL_SharedObject;
}

extern "C" {
    /// Look up the address of the named function in a shared object.
    ///
    /// This function pointer is no longer valid after calling [`SDL_UnloadObject()`].
    ///
    /// This function can only look up C function names. Other languages may have
    /// name mangling and intrinsic language support that varies from compiler to
    /// compiler.
    ///
    /// Make sure you declare your function pointers with the same calling
    /// convention as the actual library function. Your code will crash
    /// mysteriously if you do not do this.
    ///
    /// If the requested function doesn't exist, NULL is returned.
    ///
    /// ### Parameters
    /// - `handle`: a valid shared object handle returned by [`SDL_LoadObject()`].
    /// - `name`: the name of the function to look up.
    ///
    /// ### Return value
    /// Returns a pointer to the function or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LoadObject`]
    pub fn SDL_LoadFunction(
        handle: *mut SDL_SharedObject,
        name: *const ::core::ffi::c_char,
    ) -> SDL_FunctionPointer;
}

extern "C" {
    /// Unload a shared object from memory.
    ///
    /// Note that any pointers from this object looked up through
    /// [`SDL_LoadFunction()`] will no longer be valid.
    ///
    /// ### Parameters
    /// - `handle`: a valid shared object handle returned by [`SDL_LoadObject()`].
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LoadObject`]
    pub fn SDL_UnloadObject(handle: *mut SDL_SharedObject);
}

/// An opaque datatype that represents a loaded shared object.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_LoadObject`]
/// - [`SDL_LoadFunction`]
/// - [`SDL_UnloadObject`]
#[repr(C)]
pub struct SDL_SharedObject {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
