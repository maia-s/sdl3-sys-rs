//! Functionality to query the current SDL version, both as headers the app was
//! compiled against, and a library the app is linked to.

use super::stdinc::*;

/// The current major version of SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 3.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
pub const SDL_MAJOR_VERSION: ::core::primitive::i32 = 3;

/// The current minor version of the SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 2.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
pub const SDL_MINOR_VERSION: ::core::primitive::i32 = 1;

/// The current micro (or patchlevel) version of the SDL headers.
///
/// If this were SDL version 3.2.1, this value would be 1.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
pub const SDL_MICRO_VERSION: ::core::primitive::i32 = 6;

/// This macro turns the version numbers into a numeric value.
///
/// (1,2,3) becomes 1002003.
///
/// ### Parameters
/// - `major`: the major version number.
/// - `minor`: the minorversion number.
/// - `patch`: the patch version number.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_VERSIONNUM(
    major: ::core::primitive::i32,
    minor: ::core::primitive::i32,
    patch: ::core::primitive::i32,
) -> ::core::primitive::i32 {
    (((major * 1000000_i32) + (minor * 1000_i32)) + patch)
}

/// This macro extracts the major version from a version number
///
/// 1002003 becomes 1.
///
/// ### Parameters
/// - `version`: the version number.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_VERSIONNUM_MAJOR(version: ::core::primitive::i32) -> ::core::primitive::i32 {
    (version / 1000000_i32)
}

/// This macro extracts the minor version from a version number
///
/// 1002003 becomes 2.
///
/// ### Parameters
/// - `version`: the version number.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_VERSIONNUM_MINOR(version: ::core::primitive::i32) -> ::core::primitive::i32 {
    ((version / 1000_i32) % 1000_i32)
}

/// This macro extracts the micro version from a version number
///
/// 1002003 becomes 3.
///
/// ### Parameters
/// - `version`: the version number.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_VERSIONNUM_MICRO(version: ::core::primitive::i32) -> ::core::primitive::i32 {
    (version % 1000_i32)
}

/// This is the version number macro for the current SDL version.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetVersion`]
pub const SDL_VERSION: ::core::primitive::i32 =
    SDL_VERSIONNUM(SDL_MAJOR_VERSION, SDL_MINOR_VERSION, SDL_MICRO_VERSION);

/// This macro will evaluate to true if compiled with SDL at least X.Y.Z.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_VERSION_ATLEAST(
    X: ::core::primitive::i32,
    Y: ::core::primitive::i32,
    Z: ::core::primitive::i32,
) -> ::core::primitive::bool {
    (SDL_VERSION >= SDL_VERSIONNUM(X, Y, Z))
}

extern "C" {
    /// Get the version of SDL that is linked against your program.
    ///
    /// If you are linking to SDL dynamically, then it is possible that the current
    /// version will be different than the version you compiled against. This
    /// function returns the current version, while [`SDL_VERSION`] is the version you
    /// compiled with.
    ///
    /// This function may be called safely at any time, even before [`SDL_Init()`].
    ///
    /// ### Return value
    /// Returns the version of the linked library.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetRevision`]
    pub fn SDL_GetVersion() -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the code revision of SDL that is linked against your program.
    ///
    /// This value is the revision of the code you are linked with and may be
    /// different from the code you are compiling with, which is found in the
    /// constant [`SDL_REVISION`].
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
    /// ### Return value
    /// Returns an arbitrary string, uniquely identifying the exact revision of
    ///   the SDL library in use.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetVersion`]
    pub fn SDL_GetRevision() -> *const ::core::ffi::c_char;
}

#[cfg(doc)]
use crate::everything::*;
