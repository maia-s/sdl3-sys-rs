#[cfg(doc)]
emit! {
    /// This macro is a string describing the source at a particular point in
    /// development.
    ///
    /// This string is often generated from revision control's state at build time.
    ///
    /// This string can be quite complex and does not follow any standard. For
    /// example, it might be something like "SDL-prerelease-3.1.1-47-gf687e0732".
    /// It might also be user-defined at build time, so it's best to treat it as a
    /// clue in debugging forensics and not something the app will parse in any
    /// way.
    ///
    /// ### Availability
    /// This macro is available since SDL 3.0.0.
    pub const SDL_REVISION: *const ::core::ffi::c_char = c"Some arbitrary string decided at SDL build time".as_ptr();

}

#[cfg(not(doc))]
emit! {
    pub const SDL_REVISION: *const ::core::ffi::c_char = c"SDL-preview-3.1.3-220-g679dd4b18".as_ptr();

}

#[cfg(doc)]
use crate::everything::*;
