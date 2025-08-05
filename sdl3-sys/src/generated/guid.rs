//! A GUID is a 128-bit value that represents something that is uniquely
//! identifiable by this value: "globally unique."
//!
//! SDL provides functions to convert a GUID to/from a string.

use super::stdinc::*;

/// An [`SDL_GUID`] is a 128-bit identifier for an input device that identifies
/// that device across runs of SDL programs on the same platform.
///
/// If the device is detached and then re-attached to a different port, or if
/// the base system is rebooted, the device should still report the same GUID.
///
/// GUIDs are as precise as possible but are not guaranteed to distinguish
/// physically distinct but equivalent devices. For example, two game
/// controllers from the same vendor with the same product ID and revision may
/// have the same GUID.
///
/// GUIDs may be platform-dependent (i.e., the same device may report different
/// GUIDs on different operating systems).
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GUID {
    pub data: [Uint8; 16],
}

unsafe extern "C" {
    /// Get an ASCII string representation for a given [`SDL_GUID`].
    ///
    /// ## Parameters
    /// - `guid`: the [`SDL_GUID`] you wish to convert to string.
    /// - `pszGUID`: buffer in which to write the ASCII string.
    /// - `cbGUID`: the size of pszGUID, should be at least 33 bytes.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_StringToGUID`]
    pub fn SDL_GUIDToString(
        guid: SDL_GUID,
        pszGUID: *mut ::core::ffi::c_char,
        cbGUID: ::core::ffi::c_int,
    );
}

unsafe extern "C" {
    /// Convert a GUID string into a [`SDL_GUID`] structure.
    ///
    /// Performs no error checking. If this function is given a string containing
    /// an invalid GUID, the function will silently succeed, but the GUID generated
    /// will not be useful.
    ///
    /// ## Parameters
    /// - `pchGUID`: string containing an ASCII representation of a GUID.
    ///
    /// ## Return value
    /// Returns a [`SDL_GUID`] structure.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GUIDToString`]
    pub fn SDL_StringToGUID(pchGUID: *const ::core::ffi::c_char) -> SDL_GUID;
}

#[cfg(doc)]
use crate::everything::*;
