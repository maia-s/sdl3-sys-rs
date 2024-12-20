//! Platform-specific SDL API functions.

use super::stdinc::*;

use super::error::*;

use super::keyboard::*;

use super::video::*;

apply_cfg!(#[cfg(any(doc, windows))] => {
    apply_cfg!(#[cfg(feature = "use-windows-sys-v0-59")] => {
        #[cfg_attr(all(feature = "nightly", doc), doc(cfg(windows)))]
        /// (`sdl3-sys`) Enable a `use-windows-sys-*` feature to alias this to `MSG` from the `windows-sys` crate. Otherwise it's an opaque struct.
        pub type MSG = ::windows_sys_v0_59::Win32::UI::WindowsAndMessaging::MSG;
    });

    apply_cfg!(#[cfg(not(feature = "use-windows-sys-v0-59"))] => {
        #[cfg_attr(all(feature = "nightly", doc), doc(cfg(windows)))]
        /// (`sdl3-sys`) Enable a `use-windows-sys-*` feature to alias this to `MSG` from the `windows-sys` crate. Otherwise it's an opaque struct.
        pub type MSG = tagMSG;

    });

    /// A callback to be used with [`SDL_SetWindowsMessageHook`].
    ///
    /// This callback may modify the message, and should return true if the message
    /// should continue to be processed, or false to prevent further processing.
    ///
    /// As this is processing a message directly from the Windows event loop, this
    /// callback should do the minimum required work and return quickly.
    ///
    /// ### Parameters
    /// - `userdata`: the app-defined pointer provided to
    ///   [`SDL_SetWindowsMessageHook`].
    /// - `msg`: a pointer to a Win32 event structure to process.
    ///
    /// ### Return value
    /// Returns true to let event continue on, false to drop it.
    ///
    /// ### Thread safety
    /// This may only be called (by SDL) from the thread handling the
    ///   Windows event loop.
    ///
    /// ### Availability
    /// This datatype is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetWindowsMessageHook`]
    /// - [`SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP`]
    pub type SDL_WindowsMessageHook = ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, msg: *mut MSG) -> ::core::primitive::bool>;

    extern "C" {
        /// Set a callback for every Windows message, run before TranslateMessage().
        ///
        /// The callback may modify the message, and should return true if the message
        /// should continue to be processed, or false to prevent further processing.
        ///
        /// ### Parameters
        /// - `callback`: the [`SDL_WindowsMessageHook`] function to call.
        /// - `userdata`: a pointer to pass to every iteration of `callback`.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_WindowsMessageHook`]
        /// - [`SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP`]
        pub fn SDL_SetWindowsMessageHook(callback: SDL_WindowsMessageHook, userdata: *mut ::core::ffi::c_void);
    }

    #[doc(hidden)]
    #[repr(C)]
    pub struct tagMSG { _opaque: [::core::primitive::u8; 0] }

});

apply_cfg!(#[cfg(any(any(doc, windows), any(/* always disabled: SDL_PLATFORM_WINGDK */)))] => {
    extern "C" {
        /// Get the D3D9 adapter index that matches the specified display.
        ///
        /// The returned adapter index can be passed to `IDirect3D9::CreateDevice` and
        /// controls on which monitor a full screen application will appear.
        ///
        /// ### Parameters
        /// - `displayID`: the instance of the display to query.
        ///
        /// ### Return value
        /// Returns the D3D9 adapter index on success or -1 on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GetDirect3D9AdapterIndex(displayID: SDL_DisplayID) -> ::core::ffi::c_int;
    }

    extern "C" {
        /// Get the DXGI Adapter and Output indices for the specified display.
        ///
        /// The DXGI Adapter and Output indices can be passed to `EnumAdapters` and
        /// `EnumOutputs` respectively to get the objects required to create a DX10 or
        /// DX11 device and swap chain.
        ///
        /// ### Parameters
        /// - `displayID`: the instance of the display to query.
        /// - `adapterIndex`: a pointer to be filled in with the adapter index.
        /// - `outputIndex`: a pointer to be filled in with the output index.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GetDXGIOutputInfo(displayID: SDL_DisplayID, adapterIndex: *mut ::core::ffi::c_int, outputIndex: *mut ::core::ffi::c_int) -> ::core::primitive::bool;
    }

});

apply_cfg!(#[cfg(feature = "use-x11-v2")] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable either a `use-x11-*` or a `use-x11-dl-*` feature to alias this to `XEvent` from the `x11` or `x11-dl` crates, respectively. Otherwise it's an opaque struct.
    pub type XEvent = ::x11_v2::xlib::XEvent;
});

apply_cfg!(#[cfg(all(not(feature = "use-x11-v2"), feature = "use-x11-dl-v2"))] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable either a `use-x11-*` or a `use-x11-dl-*` feature to alias this to `XEvent` from the `x11` or `x11-dl` crates, respectively. Otherwise it's an opaque struct.
    pub type XEvent = ::x11_dl_v2::xlib::XEvent;
});

apply_cfg!(#[cfg(not(any(feature = "use-x11-dl-v2", feature = "use-x11-v2")))] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable either a `use-x11-*` or a `use-x11-dl-*` feature to alias this to `XEvent` from the `x11` or `x11-dl` crates, respectively. Otherwise it's an opaque struct.
    pub type XEvent = _XEvent;

});

pub type SDL_X11EventHook = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        xevent: *mut XEvent,
    ) -> ::core::primitive::bool,
>;

extern "C" {
    /// Set a callback for every X11 event.
    ///
    /// The callback may modify the event, and should return true if the event
    /// should continue to be processed, or false to prevent further processing.
    ///
    /// ### Parameters
    /// - `callback`: the [`SDL_X11EventHook`] function to call.
    /// - `userdata`: a pointer to pass to every iteration of `callback`.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetX11EventHook(callback: SDL_X11EventHook, userdata: *mut ::core::ffi::c_void);
}

apply_cfg!(#[cfg(any(doc, target_os = "linux"))] => {
    extern "C" {
        /// Sets the UNIX nice value for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// ### Parameters
        /// - `threadID`: the Unix thread ID to change priority of.
        /// - `priority`: the new, Unix-specific, priority value.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_SetLinuxThreadPriority(threadID: Sint64, priority: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Sets the priority (not nice level) and scheduling policy for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// ### Parameters
        /// - `threadID`: the Unix thread ID to change priority of.
        /// - `sdlPriority`: the new [`SDL_ThreadPriority`] value.
        /// - `schedPolicy`: the new scheduling policy (SCHED_FIFO, SCHED_RR,
        ///   SCHED_OTHER, etc...).
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_SetLinuxThreadPriorityAndPolicy(threadID: Sint64, sdlPriority: ::core::ffi::c_int, schedPolicy: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

});

apply_cfg!(#[cfg(any(doc, target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "watchos"))] => {
    /// The prototype for an Apple iOS animation callback.
    ///
    /// This datatype is only useful on Apple iOS.
    ///
    /// After passing a function pointer of this type to
    /// [`SDL_SetiOSAnimationCallback`], the system will call that function pointer at
    /// a regular interval.
    ///
    /// ### Parameters
    /// - `userdata`: what was passed as `callbackParam` to
    ///   [`SDL_SetiOSAnimationCallback`] as `callbackParam`.
    ///
    /// ### Availability
    /// This datatype is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetiOSAnimationCallback`]
    pub type SDL_iOSAnimationCallback = ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void)>;

    extern "C" {
        /// Use this function to set the animation callback on Apple iOS.
        ///
        /// The function prototype for `callback` is:
        ///
        /// ```c
        /// void callback(void *callbackParam);
        /// ```
        ///
        /// Where its parameter, `callbackParam`, is what was passed as `callbackParam`
        /// to [`SDL_SetiOSAnimationCallback()`].
        ///
        /// This function is only available on Apple iOS.
        ///
        /// For more information see:
        ///
        /// <https://wiki.libsdl.org/SDL3/README/ios>
        ///
        /// Note that if you use the "main callbacks" instead of a standard C `main`
        /// function, you don't have to use this API, as SDL will manage this for you.
        ///
        /// Details on main callbacks are here:
        ///
        /// <https://wiki.libsdl.org/SDL3/README/main-functions>
        ///
        /// ### Parameters
        /// - `window`: the window for which the animation callback should be set.
        /// - `interval`: the number of frames after which **callback** will be
        ///   called.
        /// - `callback`: the function to call for every frame.
        /// - `callbackParam`: a pointer that is passed to `callback`.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_SetiOSEventPump`]
        pub fn SDL_SetiOSAnimationCallback(window: *mut SDL_Window, interval: ::core::ffi::c_int, callback: SDL_iOSAnimationCallback, callbackParam: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Use this function to enable or disable the SDL event pump on Apple iOS.
        ///
        /// This function is only available on Apple iOS.
        ///
        /// ### Parameters
        /// - `enabled`: true to enable the event pump, false to disable it.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_SetiOSAnimationCallback`]
        pub fn SDL_SetiOSEventPump(enabled: ::core::primitive::bool);
    }

});

apply_cfg!(#[cfg(any(doc, target_os = "android"))] => {
    extern "C" {
        /// Get the Android Java Native Interface Environment of the current thread.
        ///
        /// This is the JNIEnv one needs to access the Java virtual machine from native
        /// code, and is needed for many Android APIs to be usable from C.
        ///
        /// The prototype of the function in SDL's code actually declare a void* return
        /// type, even if the implementation returns a pointer to a JNIEnv. The
        /// rationale being that the SDL headers can avoid including jni.h.
        ///
        /// ### Return value
        /// Returns a pointer to Java native interface object (JNIEnv) to which the
        ///   current thread is attached, or NULL on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidActivity`]
        pub fn SDL_GetAndroidJNIEnv() -> *mut ::core::ffi::c_void;
    }

    extern "C" {
        /// Retrieve the Java instance of the Android activity class.
        ///
        /// The prototype of the function in SDL's code actually declares a void*
        /// return type, even if the implementation returns a jobject. The rationale
        /// being that the SDL headers can avoid including jni.h.
        ///
        /// The jobject returned by the function is a local reference and must be
        /// released by the caller. See the PushLocalFrame() and PopLocalFrame() or
        /// DeleteLocalRef() functions of the Java native interface:
        ///
        /// <https://docs.oracle.com/javase/1.5.0/docs/guide/jni/spec/functions.html>
        ///
        /// ### Return value
        /// Returns the jobject representing the instance of the Activity class of the
        ///   Android application, or NULL on failure; call [`SDL_GetError()`] for
        ///   more information.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidJNIEnv`]
        pub fn SDL_GetAndroidActivity() -> *mut ::core::ffi::c_void;
    }

    extern "C" {
        /// Query Android API level of the current device.
        ///
        /// - API level 35: Android 15 (VANILLA_ICE_CREAM)
        /// - API level 34: Android 14 (UPSIDE_DOWN_CAKE)
        /// - API level 33: Android 13 (TIRAMISU)
        /// - API level 32: Android 12L (S_V2)
        /// - API level 31: Android 12 (S)
        /// - API level 30: Android 11 (R)
        /// - API level 29: Android 10 (Q)
        /// - API level 28: Android 9 (P)
        /// - API level 27: Android 8.1 (O_MR1)
        /// - API level 26: Android 8.0 (O)
        /// - API level 25: Android 7.1 (N_MR1)
        /// - API level 24: Android 7.0 (N)
        /// - API level 23: Android 6.0 (M)
        /// - API level 22: Android 5.1 (LOLLIPOP_MR1)
        /// - API level 21: Android 5.0 (LOLLIPOP, L)
        /// - API level 20: Android 4.4W (KITKAT_WATCH)
        /// - API level 19: Android 4.4 (KITKAT)
        /// - API level 18: Android 4.3 (JELLY_BEAN_MR2)
        /// - API level 17: Android 4.2 (JELLY_BEAN_MR1)
        /// - API level 16: Android 4.1 (JELLY_BEAN)
        /// - API level 15: Android 4.0.3 (ICE_CREAM_SANDWICH_MR1)
        /// - API level 14: Android 4.0 (ICE_CREAM_SANDWICH)
        /// - API level 13: Android 3.2 (HONEYCOMB_MR2)
        /// - API level 12: Android 3.1 (HONEYCOMB_MR1)
        /// - API level 11: Android 3.0 (HONEYCOMB)
        /// - API level 10: Android 2.3.3 (GINGERBREAD_MR1)
        ///
        /// ### Return value
        /// Returns the Android API level.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GetAndroidSDKVersion() -> ::core::ffi::c_int;
    }

    extern "C" {
        /// Query if the application is running on a Chromebook.
        ///
        /// ### Return value
        /// Returns true if this is a Chromebook, false otherwise.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_IsChromebook() -> ::core::primitive::bool;
    }

    extern "C" {
        /// Query if the application is running on a Samsung DeX docking station.
        ///
        /// ### Return value
        /// Returns true if this is a DeX docking station, false otherwise.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_IsDeXMode() -> ::core::primitive::bool;
    }

    extern "C" {
        /// Trigger the Android system back button behavior.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_SendAndroidBackButton();
    }

    /// See the official Android developer guide for more information:
    /// <http://developer.android.com/guide/topics/data/data-storage.html>
    ///
    /// ### Availability
    /// This macro is available since SDL 3.1.3.
    pub const SDL_ANDROID_EXTERNAL_STORAGE_READ: Uint32 = (0x01 as Uint32);

    pub const SDL_ANDROID_EXTERNAL_STORAGE_WRITE: Uint32 = (0x02 as Uint32);

    extern "C" {
        /// Get the path used for internal storage for this Android application.
        ///
        /// This path is unique to your application and cannot be written to by other
        /// applications.
        ///
        /// Your internal storage path is typically:
        /// `/data/data/your.app.package/files`.
        ///
        /// This is a C wrapper over `android.content.Context.getFilesDir()`:
        ///
        /// <https://developer.android.com/reference/android/content/Context#getFilesDir()>
        ///
        /// ### Return value
        /// Returns the path used for internal storage or NULL on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidExternalStoragePath`]
        /// - [`SDL_GetAndroidCachePath`]
        pub fn SDL_GetAndroidInternalStoragePath() -> *const ::core::ffi::c_char;
    }

    extern "C" {
        /// Get the current state of external storage for this Android application.
        ///
        /// The current state of external storage, a bitmask of these values:
        /// [`SDL_ANDROID_EXTERNAL_STORAGE_READ`], [`SDL_ANDROID_EXTERNAL_STORAGE_WRITE`].
        ///
        /// If external storage is currently unavailable, this will return 0.
        ///
        /// ### Return value
        /// Returns the current state of external storage, or 0 if external storage is
        ///   currently unavailable.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidExternalStoragePath`]
        pub fn SDL_GetAndroidExternalStorageState() -> Uint32;
    }

    extern "C" {
        /// Get the path used for external storage for this Android application.
        ///
        /// This path is unique to your application, but is public and can be written
        /// to by other applications.
        ///
        /// Your external storage path is typically:
        /// `/storage/sdcard0/Android/data/your.app.package/files`.
        ///
        /// This is a C wrapper over `android.content.Context.getExternalFilesDir()`:
        ///
        /// <https://developer.android.com/reference/android/content/Context#getExternalFilesDir()>
        ///
        /// ### Return value
        /// Returns the path used for external storage for this application on success
        ///   or NULL on failure; call [`SDL_GetError()`] for more information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidExternalStorageState`]
        /// - [`SDL_GetAndroidInternalStoragePath`]
        /// - [`SDL_GetAndroidCachePath`]
        pub fn SDL_GetAndroidExternalStoragePath() -> *const ::core::ffi::c_char;
    }

    extern "C" {
        /// Get the path used for caching data for this Android application.
        ///
        /// This path is unique to your application, but is public and can be written
        /// to by other applications.
        ///
        /// Your cache path is typically: `/data/data/your.app.package/cache/`.
        ///
        /// This is a C wrapper over `android.content.Context.getCacheDir()`:
        ///
        /// <https://developer.android.com/reference/android/content/Context#getCacheDir()>
        ///
        /// ### Return value
        /// Returns the path used for caches for this application on success or NULL
        ///   on failure; call [`SDL_GetError()`] for more information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        ///
        /// ### See also
        /// - [`SDL_GetAndroidInternalStoragePath`]
        /// - [`SDL_GetAndroidExternalStoragePath`]
        pub fn SDL_GetAndroidCachePath() -> *const ::core::ffi::c_char;
    }

    pub type SDL_RequestAndroidPermissionCallback = ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, permission: *const ::core::ffi::c_char, granted: ::core::primitive::bool)>;

    extern "C" {
        /// Request permissions at runtime, asynchronously.
        ///
        /// You do not need to call this for built-in functionality of SDL; recording
        /// from a microphone or reading images from a camera, using standard SDL APIs,
        /// will manage permission requests for you.
        ///
        /// This function never blocks. Instead, the app-supplied callback will be
        /// called when a decision has been made. This callback may happen on a
        /// different thread, and possibly much later, as it might wait on a user to
        /// respond to a system dialog. If permission has already been granted for a
        /// specific entitlement, the callback will still fire, probably on the current
        /// thread and before this function returns.
        ///
        /// If the request submission fails, this function returns -1 and the callback
        /// will NOT be called, but this should only happen in catastrophic conditions,
        /// like memory running out. Normally there will be a yes or no to the request
        /// through the callback.
        ///
        /// For the `permission` parameter, choose a value from here:
        ///
        /// <https://developer.android.com/reference/android/Manifest.permission>
        ///
        /// ### Parameters
        /// - `permission`: the permission to request.
        /// - `cb`: the callback to trigger when the request has a response.
        /// - `userdata`: an app-controlled pointer that is passed to the callback.
        ///
        /// ### Return value
        /// Returns true if the request was submitted, false if there was an error
        ///   submitting. The result of the request is only ever reported
        ///   through the callback, not this return value.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_RequestAndroidPermission(permission: *const ::core::ffi::c_char, cb: SDL_RequestAndroidPermissionCallback, userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Shows an Android toast notification.
        ///
        /// Toasts are a sort of lightweight notification that are unique to Android.
        ///
        /// <https://developer.android.com/guide/topics/ui/notifiers/toasts>
        ///
        /// Shows toast in UI thread.
        ///
        /// For the `gravity` parameter, choose a value from here, or -1 if you don't
        /// have a preference:
        ///
        /// <https://developer.android.com/reference/android/view/Gravity>
        ///
        /// ### Parameters
        /// - `message`: text message to be shown.
        /// - `duration`: 0=short, 1=long.
        /// - `gravity`: where the notification should appear on the screen.
        /// - `xoffset`: set this parameter only when gravity >=0.
        /// - `yoffset`: set this parameter only when gravity >=0.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_ShowAndroidToast(message: *const ::core::ffi::c_char, duration: ::core::ffi::c_int, gravity: ::core::ffi::c_int, xoffset: ::core::ffi::c_int, yoffset: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Send a user command to SDLActivity.
        ///
        /// Override "boolean onUnhandledMessage(Message msg)" to handle the message.
        ///
        /// ### Parameters
        /// - `command`: user command that must be greater or equal to 0x8000.
        /// - `param`: user parameter.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_SendAndroidMessage(command: Uint32, param: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

});

extern "C" {
    /// Query if the current device is a tablet.
    ///
    /// If SDL can't determine this, it will return false.
    ///
    /// ### Return value
    /// Returns true if the device is a tablet, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_IsTablet() -> ::core::primitive::bool;
}

extern "C" {
    /// Query if the current device is a TV.
    ///
    /// If SDL can't determine this, it will return false.
    ///
    /// ### Return value
    /// Returns true if the device is a TV, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_IsTV() -> ::core::primitive::bool;
}

/// Application sandbox environment.
///
/// ### Availability
/// This enum is available since SDL 3.2.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_Sandbox::NONE) | [`SDL_SANDBOX_NONE`] | |
/// | [`UNKNOWN_CONTAINER`](SDL_Sandbox::UNKNOWN_CONTAINER) | [`SDL_SANDBOX_UNKNOWN_CONTAINER`] | |
/// | [`FLATPAK`](SDL_Sandbox::FLATPAK) | [`SDL_SANDBOX_FLATPAK`] | |
/// | [`SNAP`](SDL_Sandbox::SNAP) | [`SDL_SANDBOX_SNAP`] | |
/// | [`MACOS`](SDL_Sandbox::MACOS) | [`SDL_SANDBOX_MACOS`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Sandbox(pub ::core::ffi::c_int);

impl From<SDL_Sandbox> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Sandbox) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_Sandbox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_SANDBOX_NONE",
            Self::UNKNOWN_CONTAINER => "SDL_SANDBOX_UNKNOWN_CONTAINER",
            Self::FLATPAK => "SDL_SANDBOX_FLATPAK",
            Self::SNAP => "SDL_SANDBOX_SNAP",
            Self::MACOS => "SDL_SANDBOX_MACOS",

            _ => return write!(f, "SDL_Sandbox({})", self.0),
        })
    }
}

impl SDL_Sandbox {
    pub const NONE: Self = Self(0);
    pub const UNKNOWN_CONTAINER: Self = Self(1);
    pub const FLATPAK: Self = Self(2);
    pub const SNAP: Self = Self(3);
    pub const MACOS: Self = Self(4);
}

pub const SDL_SANDBOX_NONE: SDL_Sandbox = SDL_Sandbox::NONE;
pub const SDL_SANDBOX_UNKNOWN_CONTAINER: SDL_Sandbox = SDL_Sandbox::UNKNOWN_CONTAINER;
pub const SDL_SANDBOX_FLATPAK: SDL_Sandbox = SDL_Sandbox::FLATPAK;
pub const SDL_SANDBOX_SNAP: SDL_Sandbox = SDL_Sandbox::SNAP;
pub const SDL_SANDBOX_MACOS: SDL_Sandbox = SDL_Sandbox::MACOS;

extern "C" {
    /// Get the application sandbox environment, if any.
    ///
    /// ### Return value
    /// Returns the application sandbox environment or [`SDL_SANDBOX_NONE`] if the
    ///   application is not running in a sandbox environment.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetSandbox() -> SDL_Sandbox;
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationWillTerminate.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationWillTerminate();
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationDidReceiveMemoryWarning.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationDidReceiveMemoryWarning();
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationWillResignActive.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationWillEnterBackground();
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationDidEnterBackground.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationDidEnterBackground();
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationWillEnterForeground.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationWillEnterForeground();
}

extern "C" {
    /// Let iOS apps with external event handling report
    /// onApplicationDidBecomeActive.
    ///
    /// This functions allows iOS apps that have their own event handling to hook
    /// into SDL to generate SDL events. This maps directly to an iOS-specific
    /// event, but since it doesn't do anything iOS-specific internally, it is
    /// available on all platforms, in case it might be useful for some specific
    /// paradigm. Most apps do not need to use this directly; SDL's internal event
    /// code will handle all this for windows created by SDL_CreateWindow!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OnApplicationDidEnterForeground();
}

apply_cfg!(#[cfg(any(doc, target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "watchos"))] => {
    extern "C" {
        /// Let iOS apps with external event handling report
        /// onApplicationDidChangeStatusBarOrientation.
        ///
        /// This functions allows iOS apps that have their own event handling to hook
        /// into SDL to generate SDL events. This maps directly to an iOS-specific
        /// event, but since it doesn't do anything iOS-specific internally, it is
        /// available on all platforms, in case it might be useful for some specific
        /// paradigm. Most apps do not need to use this directly; SDL's internal event
        /// code will handle all this for windows created by SDL_CreateWindow!
        ///
        /// ### Thread safety
        /// It is safe to call this function from any thread.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_OnApplicationDidChangeStatusBarOrientation();
    }

});

apply_cfg!(#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))] => {
    pub type XTaskQueueHandle = *mut XTaskQueueObject;

    pub type XUserHandle = *mut XUser;

    extern "C" {
        /// Gets a reference to the global async task queue handle for GDK,
        /// initializing if needed.
        ///
        /// Once you are done with the task queue, you should call
        /// XTaskQueueCloseHandle to reduce the reference count to avoid a resource
        /// leak.
        ///
        /// ### Parameters
        /// - `outTaskQueue`: a pointer to be filled in with task queue handle.
        ///
        /// ### Return value
        /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GetGDKTaskQueue(outTaskQueue: *mut XTaskQueueHandle) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Gets a reference to the default user handle for GDK.
        ///
        /// This is effectively a synchronous version of XUserAddAsync, which always
        /// prefers the default user and allows a sign-in UI.
        ///
        /// ### Parameters
        /// - `outUserHandle`: a pointer to be filled in with the default user
        ///   handle.
        ///
        /// ### Return value
        /// Returns true if success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// ### Availability
        /// This function is available since SDL 3.1.3.
        pub fn SDL_GetGDKDefaultUser(outUserHandle: *mut XUserHandle) -> ::core::primitive::bool;
    }

    #[repr(C)]
    pub struct XTaskQueueObject { _opaque: [::core::primitive::u8; 0] }

    #[repr(C)]
    pub struct XUser { _opaque: [::core::primitive::u8; 0] }

});

#[doc(hidden)]
#[repr(C)]
pub struct _XEvent {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
