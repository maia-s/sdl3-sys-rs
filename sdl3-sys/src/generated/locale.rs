//! SDL locale services.

use super::stdinc::*;

use super::error::*;

/// A struct to provide locale data.
///
/// Locale data is split into a spoken language, like English, and an optional
/// country, like Canada. The language will be in ISO-639 format (so English
/// would be "en"), and the country, if not NULL, will be an ISO-3166 country
/// code (so Canada would be "CA").
///
/// ### Availability
/// This function is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetPreferredLocales`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Locale {
    /// A language name, like "en" for English.
    pub language: *const ::core::ffi::c_char,
    /// A country, like "US" for America. Can be NULL.
    pub country: *const ::core::ffi::c_char,
}

extern "C" {
    /// Report the user's preferred locale.
    ///
    /// Returned language strings are in the format xx, where 'xx' is an ISO-639
    /// language specifier (such as "en" for English, "de" for German, etc).
    /// Country strings are in the format YY, where "YY" is an ISO-3166 country
    /// code (such as "US" for the United States, "CA" for Canada, etc). Country
    /// might be NULL if there's no specific guidance on them (so you might get {
    /// "en", "US" } for American English, but { "en", NULL } means "English
    /// language, generically"). Language strings are never NULL, except to
    /// terminate the array.
    ///
    /// Please note that not all of these strings are 2 characters; some are three
    /// or more.
    ///
    /// The returned list of locales are in the order of the user's preference. For
    /// example, a German citizen that is fluent in US English and knows enough
    /// Japanese to navigate around Tokyo might have a list like: { "de", "en_US",
    /// "jp", NULL }. Someone from England might prefer British English (where
    /// "color" is spelled "colour", etc), but will settle for anything like it: {
    /// "en_GB", "en", NULL }.
    ///
    /// This function returns NULL on error, including when the platform does not
    /// supply this information at all.
    ///
    /// This might be a "slow" call that has to query the operating system. It's
    /// best to ask for this once and save the results. However, this list can
    /// change, usually because the user has changed a system preference outside of
    /// your program; SDL will send an [`SDL_EVENT_LOCALE_CHANGED`] event in this case,
    /// if possible, and you can call this function again to get an updated copy of
    /// preferred locales.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of locales returned, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns a NULL terminated array of locale pointers, or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This is a single
    ///   allocation that should be freed with [`SDL_free()`] when it is no
    ///   longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetPreferredLocales(count: *mut ::core::ffi::c_int) -> *mut *mut SDL_Locale;
}

#[cfg(doc)]
use crate::everything::*;
