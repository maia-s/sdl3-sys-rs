//! Simple error message routines for SDL.

use super::stdinc::*;

extern "C" {
    /// Set the SDL error message for the current thread.
    ///
    /// Calling this function will replace any previous error message that was set.
    ///
    /// This function always returns false, since SDL frequently uses false to
    /// signify a failing result, leading to this idiom:
    ///
    /// ```c
    /// if (error_code) {
    ///     return SDL_SetError("This operation has failed: %d", error_code);
    /// }
    /// ```
    ///
    /// - `fmt`: a printf()-style message format string.
    /// - `...`: additional parameters matching % tokens in the `fmt` string, if
    ///   any.
    /// - Returns false.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ClearError`]<br>
    /// See also [`SDL_GetError`]<br>
    pub fn SDL_SetError(fmt: *const ::core::ffi::c_char, ...) -> ::core::primitive::bool;
}

extern "C" {
    /// Set an error indicating that memory allocation failed.
    ///
    /// This function does not do any memory allocation.
    ///
    /// - Returns false.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_OutOfMemory() -> ::core::primitive::bool;
}

extern "C" {
    /// Retrieve a message about the last error that occurred on the current
    /// thread.
    ///
    /// It is possible for multiple errors to occur before calling SDL_GetError().
    /// Only the last error is returned.
    ///
    /// The message is only applicable when an SDL function has signaled an error.
    /// You must check the return values of SDL function calls to determine when to
    /// appropriately call SDL_GetError(). You should *not* use the results of
    /// SDL_GetError() to decide if an error has occurred! Sometimes SDL will set
    /// an error string even when reporting success.
    ///
    /// SDL will *not* clear the error string for successful API calls. You *must*
    /// check return values for failure cases before you can assume the error
    /// string applies.
    ///
    /// Error strings are set per-thread, so an error set in a different thread
    /// will not interfere with the current thread's operation.
    ///
    /// The returned value is a thread-local string which will remain valid until
    /// the current thread's error string is changed. The caller should make a copy
    /// if the value is needed after the next SDL API call.
    ///
    /// - Returns a message with information about the specific error that occurred,
    ///   or an empty string if there hasn't been an error message set since
    ///   the last call to SDL_ClearError().
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ClearError`]<br>
    /// See also [`SDL_SetError`]<br>
    pub fn SDL_GetError() -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Clear any previous error message for this thread.
    ///
    /// - Returns true.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetError`]<br>
    /// See also [`SDL_SetError`]<br>
    pub fn SDL_ClearError() -> ::core::primitive::bool;
}

#[inline(always)]
pub unsafe fn SDL_Unsupported() -> ::core::primitive::bool {
    unsafe { SDL_SetError(c"That operation is not supported".as_ptr()) }
}
