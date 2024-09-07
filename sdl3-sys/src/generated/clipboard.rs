#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryClipboard
//!
//! SDL provides access to the system clipboard, both for reading information
//! from other processes and publishing information of its own.
//!
//! This is not just text! SDL apps can access and publish data by mimetype.

use super::stdinc::*;

use super::error::*;

extern_sdlcall! {{
    /// Put UTF-8 text into the clipboard.
    ///
    /// \param text the text to store in the clipboard.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetClipboardText
    /// \sa SDL_HasClipboardText
    pub fn SDL_SetClipboardText(text: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get UTF-8 text from the clipboard.
    ///
    /// This functions returns empty string if there was not enough memory left for
    /// a copy of the clipboard's content.
    ///
    /// \returns the clipboard text on success or an empty string on failure; call
    ///          SDL_GetError() for more information. This should be freed with
    ///          SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasClipboardText
    /// \sa SDL_SetClipboardText
    pub fn SDL_GetClipboardText() -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Query whether the clipboard exists and contains a non-empty text string.
    ///
    /// \returns SDL_TRUE if the clipboard has text, or SDL_FALSE if it does not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetClipboardText
    /// \sa SDL_SetClipboardText
    pub fn SDL_HasClipboardText() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Put UTF-8 text into the primary selection.
    ///
    /// \param text the text to store in the primary selection.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPrimarySelectionText
    /// \sa SDL_HasPrimarySelectionText
    pub fn SDL_SetPrimarySelectionText(text: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get UTF-8 text from the primary selection.
    ///
    /// This functions returns empty string if there was not enough memory left for
    /// a copy of the primary selection's content.
    ///
    /// \returns the primary selection text on success or an empty string on
    ///          failure; call SDL_GetError() for more information. This should be
    ///          freed with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasPrimarySelectionText
    /// \sa SDL_SetPrimarySelectionText
    pub fn SDL_GetPrimarySelectionText() -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Query whether the primary selection exists and contains a non-empty text
    /// string.
    ///
    /// \returns SDL_TRUE if the primary selection has text, or SDL_FALSE if it
    ///          does not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPrimarySelectionText
    /// \sa SDL_SetPrimarySelectionText
    pub fn SDL_HasPrimarySelectionText() -> SDL_bool;
}}

/// Callback function that will be called when data for the specified mime-type
/// is requested by the OS.
///
/// The callback function is called with NULL as the mime_type when the
/// clipboard is cleared or new data is set. The clipboard is automatically
/// cleared in SDL_Quit().
///
/// \param userdata a pointer to provided user data.
/// \param mime_type the requested mime-type.
/// \param size a pointer filled in with the length of the returned data.
/// \returns a pointer to the data for the provided mime-type. Returning NULL
///          or setting length to 0 will cause no data to be sent to the
///          "receiver". It is up to the receiver to handle this. Essentially
///          returning no data is more or less undefined behavior and may cause
///          breakage in receiving applications. The returned data will not be
///          freed so it needs to be retained and dealt with internally.
///
/// \since This function is available since SDL 3.0.0.
///
/// \sa SDL_SetClipboardData
pub type SDL_ClipboardDataCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, mime_type: *const ::core::ffi::c_char, size: *mut ::core::primitive::usize) -> *const ::core::ffi::c_void)>;

/// Callback function that will be called when the clipboard is cleared, or new
/// data is set.
///
/// \param userdata a pointer to provided user data.
///
/// \since This function is available since SDL 3.0.0.
///
/// \sa SDL_SetClipboardData
pub type SDL_ClipboardCleanupCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void))>;

extern_sdlcall! {{
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
    /// \param callback a function pointer to the function that provides the
    ///                 clipboard data.
    /// \param cleanup a function pointer to the function that cleans up the
    ///                clipboard data.
    /// \param userdata an opaque pointer that will be forwarded to the callbacks.
    /// \param mime_types a list of mime-types that are being offered.
    /// \param num_mime_types the number of mime-types in the mime_types list.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ClearClipboardData
    /// \sa SDL_GetClipboardData
    /// \sa SDL_HasClipboardData
    pub fn SDL_SetClipboardData(callback: SDL_ClipboardDataCallback, cleanup: SDL_ClipboardCleanupCallback, userdata: *mut ::core::ffi::c_void, mime_types: *mut *const ::core::ffi::c_char, num_mime_types: ::core::primitive::usize) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Clear the clipboard data.
    ///
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetClipboardData
    pub fn SDL_ClearClipboardData() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the data from clipboard for a given mime type.
    ///
    /// The size of text data does not include the terminator, but the text is
    /// guaranteed to be null terminated.
    ///
    /// \param mime_type the mime type to read from the clipboard.
    /// \param size a pointer filled in with the length of the returned data.
    /// \returns the retrieved data buffer or NULL on failure; call SDL_GetError()
    ///          for more information. This should be freed with SDL_free() when it
    ///          is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasClipboardData
    /// \sa SDL_SetClipboardData
    pub fn SDL_GetClipboardData(mime_type: *const ::core::ffi::c_char, size: *mut ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    /// Query whether there is data in the clipboard for the provided mime type.
    ///
    /// \param mime_type the mime type to check for data for.
    /// \returns SDL_TRUE if there exists data in clipboard for the provided mime
    ///          type, SDL_FALSE if it does not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetClipboardData
    /// \sa SDL_GetClipboardData
    pub fn SDL_HasClipboardData(mime_type: *const ::core::ffi::c_char) -> SDL_bool;
}}

