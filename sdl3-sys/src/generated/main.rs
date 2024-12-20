//! Redefine main() if necessary so that it is called by SDL.
//!
//! In order to make this consistent on all platforms, the application's main()
//! should look like this:
//!
//! ```c
//!  int main(int argc, char *argv[])
//!  {
//!  }
//! ```
//!
//! SDL will take care of platform specific details on how it gets called.
//!
//! For more information, see:
//!
//! <https://wiki.libsdl.org/SDL3/README/main-functions>

use super::stdinc::*;

use super::error::*;

use super::events::*;

apply_cfg!(#[cfg(doc)] => {
});

use super::init::*;

extern "C" {
    /// App-implemented initial entry point for SDL_MAIN_USE_CALLBACKS apps.
    ///
    /// Apps implement this function when using SDL_MAIN_USE_CALLBACKS. If using a
    /// standard "main" function, you should not supply this.
    ///
    /// This function is called by SDL once, at startup. The function should
    /// initialize whatever is necessary, possibly create windows and open audio
    /// devices, etc. The `argc` and `argv` parameters work like they would with a
    /// standard "main" function.
    ///
    /// This function should not go into an infinite mainloop; it should do any
    /// one-time setup it requires and then return.
    ///
    /// The app may optionally assign a pointer to `*appstate`. This pointer will
    /// be provided on every future call to the other entry points, to allow
    /// application state to be preserved between functions without the app needing
    /// to use a global variable. If this isn't set, the pointer will be NULL in
    /// future entry points.
    ///
    /// If this function returns [`SDL_APP_CONTINUE`], the app will proceed to normal
    /// operation, and will begin receiving repeated calls to [`SDL_AppIterate`] and
    /// [`SDL_AppEvent`] for the life of the program. If this function returns
    /// [`SDL_APP_FAILURE`], SDL will call [`SDL_AppQuit`] and terminate the process with
    /// an exit code that reports an error to the platform. If it returns
    /// [`SDL_APP_SUCCESS`], SDL calls [`SDL_AppQuit`] and terminates with an exit code
    /// that reports success to the platform.
    ///
    /// This function is called by SDL on the main thread.
    ///
    /// ### Parameters
    /// - `appstate`: a place where the app can optionally store a pointer for
    ///   future use.
    /// - `argc`: the standard ANSI C main's argc; number of elements in `argv`.
    /// - `argv`: the standard ANSI C main's argv; array of command line
    ///   arguments.
    ///
    /// ### Return value
    /// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
    ///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AppIterate`]
    /// - [`SDL_AppEvent`]
    /// - [`SDL_AppQuit`]
    pub fn SDL_AppInit(
        appstate: *mut *mut ::core::ffi::c_void,
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> SDL_AppResult;
}

extern "C" {
    /// App-implemented iteration entry point for SDL_MAIN_USE_CALLBACKS apps.
    ///
    /// Apps implement this function when using SDL_MAIN_USE_CALLBACKS. If using a
    /// standard "main" function, you should not supply this.
    ///
    /// This function is called repeatedly by SDL after [`SDL_AppInit`] returns 0. The
    /// function should operate as a single iteration the program's primary loop;
    /// it should update whatever state it needs and draw a new frame of video,
    /// usually.
    ///
    /// On some platforms, this function will be called at the refresh rate of the
    /// display (which might change during the life of your app!). There are no
    /// promises made about what frequency this function might run at. You should
    /// use SDL's timer functions if you need to see how much time has passed since
    /// the last iteration.
    ///
    /// There is no need to process the SDL event queue during this function; SDL
    /// will send events as they arrive in [`SDL_AppEvent`], and in most cases the
    /// event queue will be empty when this function runs anyhow.
    ///
    /// This function should not go into an infinite mainloop; it should do one
    /// iteration of whatever the program does and return.
    ///
    /// The `appstate` parameter is an optional pointer provided by the app during
    /// [`SDL_AppInit()`]. If the app never provided a pointer, this will be NULL.
    ///
    /// If this function returns [`SDL_APP_CONTINUE`], the app will continue normal
    /// operation, receiving repeated calls to [`SDL_AppIterate`] and [`SDL_AppEvent`] for
    /// the life of the program. If this function returns [`SDL_APP_FAILURE`], SDL will
    /// call [`SDL_AppQuit`] and terminate the process with an exit code that reports
    /// an error to the platform. If it returns [`SDL_APP_SUCCESS`], SDL calls
    /// [`SDL_AppQuit`] and terminates with an exit code that reports success to the
    /// platform.
    ///
    /// This function is called by SDL on the main thread.
    ///
    /// ### Parameters
    /// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
    ///
    /// ### Return value
    /// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
    ///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
    ///
    /// ### Thread safety
    /// This function may get called concurrently with [`SDL_AppEvent()`]
    ///   for events not pushed on the main thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AppInit`]
    /// - [`SDL_AppEvent`]
    pub fn SDL_AppIterate(appstate: *mut ::core::ffi::c_void) -> SDL_AppResult;
}

extern "C" {
    /// App-implemented event entry point for SDL_MAIN_USE_CALLBACKS apps.
    ///
    /// Apps implement this function when using SDL_MAIN_USE_CALLBACKS. If using a
    /// standard "main" function, you should not supply this.
    ///
    /// This function is called as needed by SDL after [`SDL_AppInit`] returns
    /// [`SDL_APP_CONTINUE`]. It is called once for each new event.
    ///
    /// There is (currently) no guarantee about what thread this will be called
    /// from; whatever thread pushes an event onto SDL's queue will trigger this
    /// function. SDL is responsible for pumping the event queue between each call
    /// to [`SDL_AppIterate`], so in normal operation one should only get events in a
    /// serial fashion, but be careful if you have a thread that explicitly calls
    /// [`SDL_PushEvent`]. SDL itself will push events to the queue on the main thread.
    ///
    /// Events sent to this function are not owned by the app; if you need to save
    /// the data, you should copy it.
    ///
    /// This function should not go into an infinite mainloop; it should handle the
    /// provided event appropriately and return.
    ///
    /// The `appstate` parameter is an optional pointer provided by the app during
    /// [`SDL_AppInit()`]. If the app never provided a pointer, this will be NULL.
    ///
    /// If this function returns [`SDL_APP_CONTINUE`], the app will continue normal
    /// operation, receiving repeated calls to [`SDL_AppIterate`] and [`SDL_AppEvent`] for
    /// the life of the program. If this function returns [`SDL_APP_FAILURE`], SDL will
    /// call [`SDL_AppQuit`] and terminate the process with an exit code that reports
    /// an error to the platform. If it returns [`SDL_APP_SUCCESS`], SDL calls
    /// [`SDL_AppQuit`] and terminates with an exit code that reports success to the
    /// platform.
    ///
    /// ### Parameters
    /// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
    /// - `event`: the new event for the app to examine.
    ///
    /// ### Return value
    /// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
    ///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
    ///
    /// ### Thread safety
    /// This function may get called concurrently with
    ///   [`SDL_AppIterate()`] or [`SDL_AppQuit()`] for events not pushed from
    ///   the main thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AppInit`]
    /// - [`SDL_AppIterate`]
    pub fn SDL_AppEvent(appstate: *mut ::core::ffi::c_void, event: *mut SDL_Event)
        -> SDL_AppResult;
}

extern "C" {
    /// App-implemented deinit entry point for SDL_MAIN_USE_CALLBACKS apps.
    ///
    /// Apps implement this function when using SDL_MAIN_USE_CALLBACKS. If using a
    /// standard "main" function, you should not supply this.
    ///
    /// This function is called once by SDL before terminating the program.
    ///
    /// This function will be called no matter what, even if [`SDL_AppInit`] requests
    /// termination.
    ///
    /// This function should not go into an infinite mainloop; it should
    /// deinitialize any resources necessary, perform whatever shutdown activities,
    /// and return.
    ///
    /// You do not need to call [`SDL_Quit()`] in this function, as SDL will call it
    /// after this function returns and before the process terminates, but it is
    /// safe to do so.
    ///
    /// The `appstate` parameter is an optional pointer provided by the app during
    /// [`SDL_AppInit()`]. If the app never provided a pointer, this will be NULL. This
    /// function call is the last time this pointer will be provided, so any
    /// resources to it should be cleaned up here.
    ///
    /// This function is called by SDL on the main thread.
    ///
    /// ### Parameters
    /// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
    /// - `result`: the result code that terminated the app (success or failure).
    ///
    /// ### Thread safety
    /// [`SDL_AppEvent()`] may get called concurrently with this function
    ///   if other threads that push events are still active.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AppInit`]
    pub fn SDL_AppQuit(appstate: *mut ::core::ffi::c_void, result: SDL_AppResult);
}

/// The prototype for the application's main() function
///
/// ### Parameters
/// - `argc`: an ANSI-C style main function's argc.
/// - `argv`: an ANSI-C style main function's argv.
///
/// ### Return value
/// Returns an ANSI-C main return code; generally 0 is considered successful
///   program completion, and small non-zero values are considered
///   errors.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
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
    /// [`SDL_main`] and provide its own `main`.
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
    /// ### Parameters
    /// - `argc`: an ANSI-C style main function's argc.
    /// - `argv`: an ANSI-C style main function's argv.
    ///
    /// ### Return value
    /// Returns an ANSI-C main return code; generally 0 is considered successful
    ///   program completion, and small non-zero values are considered
    ///   errors.
    ///
    /// ### Thread safety
    /// This is the program entry point.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_main(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Circumvent failure of [`SDL_Init()`] when not using [`SDL_main()`] as an entry
    /// point.
    ///
    /// This function is defined in SDL_main.h, along with the preprocessor rule to
    /// redefine main() as [`SDL_main()`]. Thus to ensure that your main() function
    /// will not be changed it is necessary to define SDL_MAIN_HANDLED before
    /// including SDL.h.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_Init`]
    pub fn SDL_SetMainReady();
}

extern "C" {
    /// Initializes and launches an SDL application, by doing platform-specific
    /// initialization before calling your mainFunction and cleanups after it
    /// returns, if that is needed for a specific platform, otherwise it just calls
    /// mainFunction.
    ///
    /// You can use this if you want to use your own main() implementation without
    /// using [`SDL_main`] (like when using SDL_MAIN_HANDLED). When using this, you do
    /// *not* need [`SDL_SetMainReady()`].
    ///
    /// ### Parameters
    /// - `argc`: the argc parameter from the application's main() function, or 0
    ///   if the platform's main-equivalent has no argc.
    /// - `argv`: the argv parameter from the application's main() function, or
    ///   NULL if the platform's main-equivalent has no argv.
    /// - `mainFunction`: your SDL app's C-style main(). NOT the function you're
    ///   calling this from! Its name doesn't matter; it doesn't
    ///   literally have to be `main`.
    /// - `reserved`: should be NULL (reserved for future use, will probably be
    ///   platform-specific then).
    ///
    /// ### Return value
    /// Returns the return value from mainFunction: 0 on success, otherwise
    ///   failure; [`SDL_GetError()`] might have more information on the
    ///   failure.
    ///
    /// ### Thread safety
    /// Generally this is called once, near startup, from the
    ///   process's initial thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
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
    /// ### Parameters
    /// - `argc`: standard Unix main argc.
    /// - `argv`: standard Unix main argv.
    /// - `appinit`: the application's [`SDL_AppInit`] function.
    /// - `appiter`: the application's [`SDL_AppIterate`] function.
    /// - `appevent`: the application's [`SDL_AppEvent`] function.
    /// - `appquit`: the application's [`SDL_AppQuit`] function.
    ///
    /// ### Return value
    /// Returns standard Unix main return value.
    ///
    /// ### Thread safety
    /// It is not safe to call this anywhere except as the only
    ///   function call in [`SDL_main`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_EnterAppMainCallbacks(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        appinit: SDL_AppInit_func,
        appiter: SDL_AppIterate_func,
        appevent: SDL_AppEvent_func,
        appquit: SDL_AppQuit_func,
    ) -> ::core::ffi::c_int;
}

apply_cfg!(#[cfg(any(doc, windows))] => {
    extern "C" {
        /// Register a win32 window class for SDL's use.
        ///
        /// This can be called to set the application window class at startup. It is
        /// safe to call this multiple times, as long as every call is eventually
        /// paired with a call to [`SDL_UnregisterApp`], but a second registration attempt
        /// while a previous registration is still active will be ignored, other than
        /// to increment a counter.
        ///
        /// Most applications do not need to, and should not, call this directly; SDL
        /// will call it when initializing the video subsystem.
        ///
        /// ### Parameters
        /// - `name`: the window class name, in UTF-8 encoding. If NULL, SDL
        ///   currently uses "SDL_app" but this isn't guaranteed.
        /// - `style`: the value to use in WNDCLASSEX::style. If `name` is NULL, SDL
        ///   currently uses `(CS_BYTEALIGNCLIENT | CS_OWNDC)` regardless of
        ///   what is specified here.
        /// - `hInst`: the HINSTANCE to use in WNDCLASSEX::hInstance. If zero, SDL
        ///   will use `GetModuleHandle(NULL)` instead.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_RegisterApp(name: *const ::core::ffi::c_char, style: Uint32, hInst: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Deregister the win32 window class from an [`SDL_RegisterApp`] call.
        ///
        /// This can be called to undo the effects of [`SDL_RegisterApp`].
        ///
        /// Most applications do not need to, and should not, call this directly; SDL
        /// will call it when deinitializing the video subsystem.
        ///
        /// It is safe to call this multiple times, as long as every call is eventually
        /// paired with a prior call to [`SDL_RegisterApp`]. The window class will only be
        /// deregistered when the registration counter in [`SDL_RegisterApp`] decrements to
        /// zero through calls to this function.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_UnregisterApp();
    }

});

apply_cfg!(#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))] => {
    extern "C" {
        /// Callback from the application to let the suspend continue.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GDKSuspendComplete();
    }

});

#[cfg(doc)]
use crate::everything::*;
