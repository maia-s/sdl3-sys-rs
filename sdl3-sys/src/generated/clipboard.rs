//! SDL provides access to the system clipboard, both for reading information
//! from other processes and publishing information of its own.
//!
//! This is not just text! SDL apps can access and publish data by mimetype.

use super::stdinc::*;

use super::error::*;

extern "C" {
    /// Put UTF-8 text into the clipboard.
    ///
    /// - `text`: the text to store in the clipboard.
    /// - Returns true on success or false on failure; call SDL_GetError() for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetClipboardText`]<br>
    /// See also [`SDL_HasClipboardText`]<br>
    pub fn SDL_SetClipboardText(text: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Get UTF-8 text from the clipboard.
    ///
    /// This functions returns empty string if there was not enough memory left for
    /// a copy of the clipboard's content.
    ///
    /// - Returns the clipboard text on success or an empty string on failure; call
    ///   SDL_GetError() for more information. This should be freed with
    ///   SDL_free() when it is no longer needed.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_HasClipboardText`]<br>
    /// See also [`SDL_SetClipboardText`]<br>
    pub fn SDL_GetClipboardText() -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Query whether the clipboard exists and contains a non-empty text string.
    ///
    /// - Returns true if the clipboard has text, or false if it does not.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetClipboardText`]<br>
    /// See also [`SDL_SetClipboardText`]<br>
    pub fn SDL_HasClipboardText() -> ::core::primitive::bool;
}

extern "C" {
    /// Put UTF-8 text into the primary selection.
    ///
    /// - `text`: the text to store in the primary selection.
    /// - Returns true on success or false on failure; call SDL_GetError() for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetPrimarySelectionText`]<br>
    /// See also [`SDL_HasPrimarySelectionText`]<br>
    pub fn SDL_SetPrimarySelectionText(text: *const ::core::ffi::c_char)
        -> ::core::primitive::bool;
}

extern "C" {
    /// Get UTF-8 text from the primary selection.
    ///
    /// This functions returns empty string if there was not enough memory left for
    /// a copy of the primary selection's content.
    ///
    /// - Returns the primary selection text on success or an empty string on
    ///   failure; call SDL_GetError() for more information. This should be
    ///   freed with SDL_free() when it is no longer needed.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_HasPrimarySelectionText`]<br>
    /// See also [`SDL_SetPrimarySelectionText`]<br>
    pub fn SDL_GetPrimarySelectionText() -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Query whether the primary selection exists and contains a non-empty text
    /// string.
    ///
    /// - Returns true if the primary selection has text, or false if it does not.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetPrimarySelectionText`]<br>
    /// See also [`SDL_SetPrimarySelectionText`]<br>
    pub fn SDL_HasPrimarySelectionText() -> ::core::primitive::bool;
}

/// Callback function that will be called when data for the specified mime-type
/// is requested by the OS.
///
/// The callback function is called with NULL as the mime_type when the
/// clipboard is cleared or new data is set. The clipboard is automatically
/// cleared in SDL_Quit().
///
/// - `userdata`: a pointer to provided user data.
/// - `mime_type`: the requested mime-type.
/// - `size`: a pointer filled in with the length of the returned data.
/// - Returns a pointer to the data for the provided mime-type. Returning NULL
///   or setting length to 0 will cause no data to be sent to the
///   "receiver". It is up to the receiver to handle this. Essentially
///   returning no data is more or less undefined behavior and may cause
///   breakage in receiving applications. The returned data will not be
///   freed so it needs to be retained and dealt with internally.
///
/// This function is available since SDL 3.0.0.
///
/// See also [`SDL_SetClipboardData`]<br>
pub type SDL_ClipboardDataCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        mime_type: *const ::core::ffi::c_char,
        size: *mut ::core::primitive::usize,
    ) -> *const ::core::ffi::c_void,
>;

/// Callback function that will be called when the clipboard is cleared, or new
/// data is set.
///
/// - `userdata`: a pointer to provided user data.
///
/// This function is available since SDL 3.0.0.
///
/// See also [`SDL_SetClipboardData`]<br>
pub type SDL_ClipboardCleanupCallback =
    ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void)>;

extern "C" {
    /// Offer clipboard data to the OS.
    ///
    /// Tell the operating system that the application is offering clipboard data
    /// for each of the proivded mime-types. Once another application requests the
    /// data the callback function will be called allowing it to generate and
    /// respond with the data for the requested mime-type.
    ///
    /// The size of text data does not include any terminator, and the text does
    /// not need to be null terminated (e.g. you can directly copy a portion of a
    /// document)
    ///
    /// - `callback`: a function pointer to the function that provides the
    ///   clipboard data.
    /// - `cleanup`: a function pointer to the function that cleans up the
    ///   clipboard data.
    /// - `userdata`: an opaque pointer that will be forwarded to the callbacks.
    /// - `mime_types`: a list of mime-types that are being offered.
    /// - `num_mime_types`: the number of mime-types in the mime_types list.
    /// - Returns true on success or false on failure; call SDL_GetError() for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ClearClipboardData`]<br>
    /// See also [`SDL_GetClipboardData`]<br>
    /// See also [`SDL_HasClipboardData`]<br>
    pub fn SDL_SetClipboardData(
        callback: SDL_ClipboardDataCallback,
        cleanup: SDL_ClipboardCleanupCallback,
        userdata: *mut ::core::ffi::c_void,
        mime_types: *mut *const ::core::ffi::c_char,
        num_mime_types: ::core::primitive::usize,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Clear the clipboard data.
    ///
    /// - Returns true on success or false on failure; call SDL_GetError() for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetClipboardData`]<br>
    pub fn SDL_ClearClipboardData() -> ::core::primitive::bool;
}

extern "C" {
    /// Get the data from clipboard for a given mime type.
    ///
    /// The size of text data does not include the terminator, but the text is
    /// guaranteed to be null terminated.
    ///
    /// - `mime_type`: the mime type to read from the clipboard.
    /// - `size`: a pointer filled in with the length of the returned data.
    /// - Returns the retrieved data buffer or NULL on failure; call SDL_GetError()
    ///   for more information. This should be freed with SDL_free() when it
    ///   is no longer needed.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_HasClipboardData`]<br>
    /// See also [`SDL_SetClipboardData`]<br>
    pub fn SDL_GetClipboardData(
        mime_type: *const ::core::ffi::c_char,
        size: *mut ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Query whether there is data in the clipboard for the provided mime type.
    ///
    /// - `mime_type`: the mime type to check for data for.
    /// - Returns true if there exists data in clipboard for the provided mime type,
    ///   false if it does not.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetClipboardData`]<br>
    /// See also [`SDL_GetClipboardData`]<br>
    pub fn SDL_HasClipboardData(mime_type: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Retrieve the list of mime types available in the clipboard.
    ///
    /// - `num_mime_types`: a pointer filled with the number of mime types, may
    ///   be NULL.
    /// - Returns a null terminated array of strings with mime types, or NULL on
    ///   failure; call SDL_GetError() for more information. This should be
    ///   freed with SDL_free() when it is no longer needed.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetClipboardData`]<br>
    pub fn SDL_GetClipboardMimeTypes(
        num_mime_types: *mut ::core::primitive::usize,
    ) -> *mut *mut ::core::ffi::c_char;
}
