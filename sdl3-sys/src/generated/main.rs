//! Redefine main() on some platforms so that it is called by SDL.
//!
//! For details on how SDL_main works, and how to use it, please refer to:
//!
//! https://wiki.libsdl.org/SDL3/README/main-functions
//!
//! (or docs/README-main-functions.md in the SDL source tree)

use super::stdinc::*;

use super::error::*;

use super::events::*;

#[cfg(doc)]
emit! {}

use super::init::*;

/// The prototype for the application's main() function
///
/// \param argc an ANSI-C style main function's argc.
/// \param argv an ANSI-C style main function's argv.
/// \returns an ANSI-C main return code; generally 0 is considered successful
///          program completion, and small non-zero values are considered
///          errors.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_main_func = ::core::option::Option<
    unsafe extern "C" fn(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

extern "C" {
    /// An app-supplied function for program entry.
    ///
    /// Apps do not directly create this function; they should create a standard
    /// ANSI-C `main` function instead. If SDL needs to insert some startup code
    /// before `main` runs, or the platform doesn't actually _use_ a function
    /// called "main", SDL will do some macro magic to redefine `main` to
    /// `SDL_main` and provide its own `main`.
    ///
    /// Apps should include `SDL_main.h` in the same file as their `main` function,
    /// and they should not use that symbol for anything else in that file, as it
    /// might get redefined.
    ///
    /// This function is only provided by the app if it isn't using
    /// SDL_MAIN_USE_CALLBACKS.
    ///
    /// Program startup is a surprisingly complex topic. Please see
    /// [README/main-functions](README/main-functions), (or
    /// docs/README-main-functions.md in the source tree) for a more detailed
    /// explanation.
    ///
    /// \param argc an ANSI-C style main function's argc.
    /// \param argv an ANSI-C style main function's argv.
    /// \returns an ANSI-C main return code; generally 0 is considered successful
    ///          program completion, and small non-zero values are considered
    ///          errors.
    ///
    /// \threadsafety This is the program entry point.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_main(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Circumvent failure of SDL_Init() when not using SDL_main() as an entry
    /// point.
    ///
    /// This function is defined in SDL_main.h, along with the preprocessor rule to
    /// redefine main() as SDL_main(). Thus to ensure that your main() function
    /// will not be changed it is necessary to define SDL_MAIN_HANDLED before
    /// including SDL.h.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Init
    pub fn SDL_SetMainReady();
}

extern "C" {
    /// Initializes and launches an SDL application, by doing platform-specific
    /// initialization before calling your mainFunction and cleanups after it
    /// returns, if that is needed for a specific platform, otherwise it just calls
    /// mainFunction.
    ///
    /// You can use this if you want to use your own main() implementation without
    /// using SDL_main (like when using SDL_MAIN_HANDLED). When using this, you do
    /// *not* need SDL_SetMainReady().
    ///
    /// \param argc the argc parameter from the application's main() function, or 0
    ///             if the platform's main-equivalent has no argc.
    /// \param argv the argv parameter from the application's main() function, or
    ///             NULL if the platform's main-equivalent has no argv.
    /// \param mainFunction your SDL app's C-style main(). NOT the function you're
    ///                     calling this from! Its name doesn't matter; it doesn't
    ///                     literally have to be `main`.
    /// \param reserved should be NULL (reserved for future use, will probably be
    ///                 platform-specific then).
    /// \returns the return value from mainFunction: 0 on success, otherwise
    ///          failure; SDL_GetError() might have more information on the
    ///          failure.
    ///
    /// \threadsafety Generally this is called once, near startup, from the
    ///               process's initial thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RunApp(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        mainFunction: SDL_main_func,
        reserved: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// An entry point for SDL's use in SDL_MAIN_USE_CALLBACKS.
    ///
    /// Generally, you should not call this function directly. This only exists to
    /// hand off work into SDL as soon as possible, where it has a lot more control
    /// and functionality available, and make the inline code in SDL_main.h as
    /// small as possible.
    ///
    /// Not all platforms use this, it's actual use is hidden in a magic
    /// header-only library, and you should not call this directly unless you
    /// _really_ know what you're doing.
    ///
    /// \param argc standard Unix main argc.
    /// \param argv standard Unix main argv.
    /// \param appinit the application's SDL_AppInit function.
    /// \param appiter the application's SDL_AppIterate function.
    /// \param appevent the application's SDL_AppEvent function.
    /// \param appquit the application's SDL_AppQuit function.
    /// \returns standard Unix main return value.
    ///
    /// \threadsafety It is not safe to call this anywhere except as the only
    ///               function call in SDL_main.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EnterAppMainCallbacks(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        appinit: SDL_AppInit_func,
        appiter: SDL_AppIterate_func,
        appevent: SDL_AppEvent_func,
        appquit: SDL_AppQuit_func,
    ) -> ::core::ffi::c_int;
}

#[cfg(windows)]
emit! {
    extern "C" {
        /// Register a win32 window class for SDL's use.
        ///
        /// This can be called to set the application window class at startup. It is
        /// safe to call this multiple times, as long as every call is eventually
        /// paired with a call to SDL_UnregisterApp, but a second registration attempt
        /// while a previous registration is still active will be ignored, other than
        /// to increment a counter.
        ///
        /// Most applications do not need to, and should not, call this directly; SDL
        /// will call it when initializing the video subsystem.
        ///
        /// \param name the window class name, in UTF-8 encoding. If NULL, SDL
        ///             currently uses "SDL_app" but this isn't guaranteed.
        /// \param style the value to use in WNDCLASSEX::style. If `name` is NULL, SDL
        ///              currently uses `(CS_BYTEALIGNCLIENT | CS_OWNDC)` regardless of
        ///              what is specified here.
        /// \param hInst the HINSTANCE to use in WNDCLASSEX::hInstance. If zero, SDL
        ///              will use `GetModuleHandle(NULL)` instead.
        /// \returns true on success or false on failure; call SDL_GetError() for more
        ///          information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_RegisterApp(name: *const ::core::ffi::c_char, style: Uint32, hInst: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Deregister the win32 window class from an SDL_RegisterApp call.
        ///
        /// This can be called to undo the effects of SDL_RegisterApp.
        ///
        /// Most applications do not need to, and should not, call this directly; SDL
        /// will call it when deinitializing the video subsystem.
        ///
        /// It is safe to call this multiple times, as long as every call is eventually
        /// paired with a prior call to SDL_RegisterApp. The window class will only be
        /// deregistered when the registration counter in SDL_RegisterApp decrements to
        /// zero through calls to this function.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_UnregisterApp();
    }

}

#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))]
emit! {
    extern "C" {
        /// Callback from the application to let the suspend continue.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GDKSuspendComplete();
    }

}
