//! SDL provides a means to identify the app's platform, both at compile time
//! and runtime.

extern "C" {
    /// Get the name of the platform.
    ///
    /// Here are the names returned for some (but not all) supported platforms:
    ///
    /// - "Windows"
    /// - "macOS"
    /// - "Linux"
    /// - "iOS"
    /// - "Android"
    ///
    /// ### Return value
    /// Returns the name of the platform. If the correct platform name is not
    ///   available, returns a string beginning with the text "Unknown".
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetPlatform() -> *const ::core::ffi::c_char;
}

#[cfg(doc)]
use crate::everything::*;
