//! System-dependent library loading routines.
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
    /// - `sofile`: a system-dependent name of the object file.
    /// - Returns an opaque pointer to the object handle or NULL on failure; call
    ///   SDL_GetError() for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_LoadFunction`]<br>
    /// See also [`SDL_UnloadObject`]<br>
    pub fn SDL_LoadObject(sofile: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Look up the address of the named function in a shared object.
    ///
    /// This function pointer is no longer valid after calling SDL_UnloadObject().
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
    /// - `handle`: a valid shared object handle returned by SDL_LoadObject().
    /// - `name`: the name of the function to look up.
    /// - Returns a pointer to the function or NULL on failure; call SDL_GetError()
    ///   for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_LoadObject`]<br>
    pub fn SDL_LoadFunction(
        handle: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
    ) -> SDL_FunctionPointer;
}

extern "C" {
    /// Unload a shared object from memory.
    ///
    /// - `handle`: a valid shared object handle returned by SDL_LoadObject().
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_LoadObject`]<br>
    pub fn SDL_UnloadObject(handle: *mut ::core::ffi::c_void);
}
