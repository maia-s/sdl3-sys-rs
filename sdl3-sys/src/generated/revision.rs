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
    /// \since This macro is available since SDL 3.0.0.
    pub const SDL_REVISION: &::core::ffi::CStr = c"Some arbitrary string decided at SDL build time";

}

#[cfg(not(doc))]
emit! {
    pub const SDL_REVISION: &::core::ffi::CStr = c"prerelease-3.1.2-1876-g53bf2baac";

}
