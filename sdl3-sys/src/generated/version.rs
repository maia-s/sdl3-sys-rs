//! Functionality to query the current SDL version, both as headers the app was
//! compiled against, and a library the app is linked to.

use super::stdinc::*;

/// The current major version of SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 3.
///
/// \since This macro is available since SDL 3.0.0.
pub const SDL_MAJOR_VERSION: ::core::primitive::i32 = 3;

/// The current minor version of the SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 2.
///
/// \since This macro is available since SDL 3.0.0.
pub const SDL_MINOR_VERSION: ::core::primitive::i32 = 1;

/// The current micro (or patchlevel) version of the SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 1.
///
/// \since This macro is available since SDL 3.0.0.
pub const SDL_MICRO_VERSION: ::core::primitive::i32 = 2;

// [sdl3-sys-gen] skipped function-like define `SDL_VERSIONNUM`

// [sdl3-sys-gen] skipped function-like define `SDL_VERSIONNUM_MAJOR`

// [sdl3-sys-gen] skipped function-like define `SDL_VERSIONNUM_MINOR`

// [sdl3-sys-gen] skipped function-like define `SDL_VERSIONNUM_MICRO`

// [sdl3-sys-gen] skipped function-like define `SDL_VERSION_ATLEAST`

extern "C" {
    /// Get the version of SDL that is linked against your program.
    ///
    /// If you are linking to SDL dynamically, then it is possible that the current
    /// version will be different than the version you compiled against. This
    /// function returns the current version, while SDL_VERSION is the version you
    /// compiled with.
    ///
    /// This function may be called safely at any time, even before SDL_Init().
    ///
    /// \returns the version of the linked library.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRevision
    pub fn SDL_GetVersion() -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the code revision of SDL that is linked against your program.
    ///
    /// This value is the revision of the code you are linked with and may be
    /// different from the code you are compiling with, which is found in the
    /// constant SDL_REVISION.
    ///
    /// The revision is arbitrary string (a hash value) uniquely identifying the
    /// exact revision of the SDL library in use, and is only useful in comparing
    /// against other revisions. It is NOT an incrementing number.
    ///
    /// If SDL wasn't built from a git repository with the appropriate tools, this
    /// will return an empty string.
    ///
    /// You shouldn't use this function for anything but logging it for debugging
    /// purposes. The string is not intended to be reliable in any way.
    ///
    /// \returns an arbitrary string, uniquely identifying the exact revision of
    ///          the SDL library in use.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetVersion
    pub fn SDL_GetRevision() -> *const ::core::ffi::c_char;
}
