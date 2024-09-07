#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategorySystem
//!
//! Platform-specific SDL API functions.

use super::stdinc::*;

use super::error::*;

use super::keyboard::*;

use super::render::*;

use super::video::*;

#[cfg(windows)]
emit! {
    pub type MSG = tagMSG;

    /// A callback to be used with SDL_SetWindowsMessageHook.
    ///
    /// This callback may modify the message, and should return SDL_TRUE if the
    /// message should continue to be processed, or SDL_FALSE to prevent further
    /// processing.
    ///
    /// As this is processing a message directly from the Windows event loop, this
    /// callback should do the minimum required work and return quickly.
    ///
    /// \param userdata the app-defined pointer provided to
    ///                 SDL_SetWindowsMessageHook.
    /// \param msg a pointer to a Win32 event structure to process.
    /// \returns SDL_TRUE to let event continue on, SDL_FALSE to drop it.
    ///
    /// \threadsafety This may only be called (by SDL) from the thread handling the
    ///               Windows event loop.
    ///
    /// \since This datatype is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowsMessageHook
    /// \sa SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP
    pub type SDL_WindowsMessageHook = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, msg: *mut MSG) -> SDL_bool)>;

    extern_sdlcall! {{
        /// Set a callback for every Windows message, run before TranslateMessage().
        ///
        /// The callback may modify the message, and should return SDL_TRUE if the
        /// message should continue to be processed, or SDL_FALSE to prevent further
        /// processing.
        ///
        /// \param callback the SDL_WindowsMessageHook function to call.
        /// \param userdata a pointer to pass to every iteration of `callback`.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_WindowsMessageHook
        /// \sa SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP
        pub fn SDL_SetWindowsMessageHook(callback: SDL_WindowsMessageHook, userdata: *mut ::core::ffi::c_void);
    }}

}

#[cfg(any(windows, any(/* always disabled: SDL_PLATFORM_WINGDK */)))]
emit! {
    extern_sdlcall! {{
        /// Get the D3D9 adapter index that matches the specified display.
        ///
        /// The returned adapter index can be passed to `IDirect3D9::CreateDevice` and
        /// controls on which monitor a full screen application will appear.
        ///
        /// \param displayID the instance of the display to query.
        /// \returns the D3D9 adapter index on success or -1 on failure; call
        ///          SDL_GetError() for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetDirect3D9AdapterIndex(displayID: SDL_DisplayID) -> ::core::ffi::c_int;
    }}

    extern_sdlcall! {{
        /// Get the DXGI Adapter and Output indices for the specified display.
        ///
        /// The DXGI Adapter and Output indices can be passed to `EnumAdapters` and
        /// `EnumOutputs` respectively to get the objects required to create a DX10 or
        /// DX11 device and swap chain.
        ///
        /// \param displayID the instance of the display to query.
        /// \param adapterIndex a pointer to be filled in with the adapter index.
        /// \param outputIndex a pointer to be filled in with the output index.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetDXGIOutputInfo(displayID: SDL_DisplayID, adapterIndex: *mut ::core::ffi::c_int, outputIndex: *mut ::core::ffi::c_int) -> SDL_bool;
    }}

}

pub type XEvent = _XEvent;

pub type SDL_X11EventHook = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, xevent: *mut XEvent) -> SDL_bool)>;

extern_sdlcall! {{
    /// Set a callback for every X11 event.
    ///
    /// The callback may modify the event, and should return SDL_TRUE if the event
    /// should continue to be processed, or SDL_FALSE to prevent further
    /// processing.
    ///
    /// \param callback the SDL_X11EventHook function to call.
    /// \param userdata a pointer to pass to every iteration of `callback`.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetX11EventHook(callback: SDL_X11EventHook, userdata: *mut ::core::ffi::c_void);
}}

#[cfg(target_os = "linux")]
emit! {
    extern_sdlcall! {{
        /// Sets the UNIX nice value for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// \param threadID the Unix thread ID to change priority of.
        /// \param priority the new, Unix-specific, priority value.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_SetLinuxThreadPriority(threadID: Sint64, priority: ::core::ffi::c_int) -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Sets the priority (not nice level) and scheduling policy for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// \param threadID the Unix thread ID to change priority of.
        /// \param sdlPriority the new SDL_ThreadPriority value.
        /// \param schedPolicy the new scheduling policy (SCHED_FIFO, SCHED_RR,
        ///                    SCHED_OTHER, etc...).
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_SetLinuxThreadPriorityAndPolicy(threadID: Sint64, sdlPriority: ::core::ffi::c_int, schedPolicy: ::core::ffi::c_int) -> SDL_bool;
    }}

}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
emit! {
    /// The prototype for an Apple iOS animation callback.
    ///
    /// This datatype is only useful on Apple iOS.
    ///
    /// After passing a function pointer of this type to
    /// SDL_SetiOSAnimationCallback, the system will call that function pointer at
    /// a regular interval.
    ///
    /// \param userdata what was passed as `callbackParam` to
    ///                 SDL_SetiOSAnimationCallback as `callbackParam`.
    ///
    /// \since This datatype is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetiOSAnimationCallback
    pub type SDL_iOSAnimationCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void))>;

    extern_sdlcall! {{
        /// Use this function to set the animation callback on Apple iOS.
        ///
        /// The function prototype for `callback` is:
        ///
        /// ```c
        /// void callback(void *callbackParam);
        /// ```
        ///
        /// Where its parameter, `callbackParam`, is what was passed as `callbackParam`
        /// to SDL_SetiOSAnimationCallback().
        ///
        /// This function is only available on Apple iOS.
        ///
        /// For more information see:
        ///
        /// https://wiki.libsdl.org/SDL3/README/ios
        ///
        /// Note that if you use the "main callbacks" instead of a standard C `main`
        /// function, you don't have to use this API, as SDL will manage this for you.
        ///
        /// Details on main callbacks are here:
        ///
        /// https://wiki.libsdl.org/SDL3/README/main-functions
        ///
        /// \param window the window for which the animation callback should be set.
        /// \param interval the number of frames after which **callback** will be
        ///                 called.
        /// \param callback the function to call for every frame.
        /// \param callbackParam a pointer that is passed to `callback`.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_SetiOSEventPump
        pub fn SDL_SetiOSAnimationCallback(window: *mut SDL_Window, interval: ::core::ffi::c_int, callback: SDL_iOSAnimationCallback, callbackParam: *mut ::core::ffi::c_void) -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Use this function to enable or disable the SDL event pump on Apple iOS.
        ///
        /// This function is only available on Apple iOS.
        ///
        /// \param enabled SDL_TRUE to enable the event pump, SDL_FALSE to disable it.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_SetiOSAnimationCallback
        pub fn SDL_SetiOSEventPump(enabled: SDL_bool);
    }}

}

#[cfg(target_os = "android")]
emit! {
    extern_sdlcall! {{
        /// Get the Android Java Native Interface Environment of the current thread.
        ///
        /// This is the JNIEnv one needs to access the Java virtual machine from native
        /// code, and is needed for many Android APIs to be usable from C.
        ///
        /// The prototype of the function in SDL's code actually declare a void* return
        /// type, even if the implementation returns a pointer to a JNIEnv. The
        /// rationale being that the SDL headers can avoid including jni.h.
        ///
        /// \returns a pointer to Java native interface object (JNIEnv) to which the
        ///          current thread is attached, or NULL on failure; call
        ///          SDL_GetError() for more information.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_GetAndroidActivity
        pub fn SDL_GetAndroidJNIEnv() -> *mut ::core::ffi::c_void;
    }}

    extern_sdlcall! {{
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
        /// https://docs.oracle.com/javase/1.5.0/docs/guide/jni/spec/functions.html
        ///
        /// \returns the jobject representing the instance of the Activity class of the
        ///          Android application, or NULL on failure; call SDL_GetError() for
        ///          more information.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_GetAndroidJNIEnv
        pub fn SDL_GetAndroidActivity() -> *mut ::core::ffi::c_void;
    }}

    extern_sdlcall! {{
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
        /// \returns the Android API level.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetAndroidSDKVersion() -> ::core::ffi::c_int;
    }}

    extern_sdlcall! {{
        /// Query if the application is running on Android TV.
        ///
        /// \returns SDL_TRUE if this is Android TV, SDL_FALSE otherwise.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_IsAndroidTV() -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Query if the application is running on a Chromebook.
        ///
        /// \returns SDL_TRUE if this is a Chromebook, SDL_FALSE otherwise.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_IsChromebook() -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Query if the application is running on a Samsung DeX docking station.
        ///
        /// \returns SDL_TRUE if this is a DeX docking station, SDL_FALSE otherwise.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_IsDeXMode() -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Trigger the Android system back button behavior.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_SendAndroidBackButton();
    }}

    /// See the official Android developer guide for more information:
    /// http://developer.android.com/guide/topics/data/data-storage.html
    ///
    /// \since This macro is available since SDL 3.0.0.
    pub const SDL_ANDROID_EXTERNAL_STORAGE_READ: ::core::primitive::i32 = 1;

    pub const SDL_ANDROID_EXTERNAL_STORAGE_WRITE: ::core::primitive::i32 = 2;

    extern_sdlcall! {{
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
        /// https://developer.android.com/reference/android/content/Context#getFilesDir()
        ///
        /// \returns the path used for internal storage or NULL on failure; call
        ///          SDL_GetError() for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_GetAndroidExternalStorageState
        pub fn SDL_GetAndroidInternalStoragePath() -> *const ::core::ffi::c_char;
    }}

    extern_sdlcall! {{
        /// Get the current state of external storage for this Android application.
        ///
        /// The current state of external storage, a bitmask of these values:
        /// `SDL_ANDROID_EXTERNAL_STORAGE_READ`, `SDL_ANDROID_EXTERNAL_STORAGE_WRITE`.
        ///
        /// If external storage is currently unavailable, this will return 0.
        ///
        /// \returns the current state of external storage, or 0 if external storage is
        ///          currently unavailable.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_GetAndroidExternalStoragePath
        pub fn SDL_GetAndroidExternalStorageState() -> Uint32;
    }}

    extern_sdlcall! {{
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
        /// https://developer.android.com/reference/android/content/Context#getExternalFilesDir()
        ///
        /// \returns the path used for external storage for this application on success
        ///          or NULL on failure; call SDL_GetError() for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_GetAndroidExternalStorageState
        pub fn SDL_GetAndroidExternalStoragePath() -> *const ::core::ffi::c_char;
    }}

    extern_sdlcall! {{
        /// Get the path used for caching data for this Android application.
        ///
        /// This path is unique to your application, but is public and can be written
        /// to by other applications.
        ///
        /// Your cache path is typically: `/data/data/your.app.package/cache/`.
        ///
        /// This is a C wrapper over `android.content.Context.getCacheDir()`:
        ///
        /// https://developer.android.com/reference/android/content/Context#getCacheDir()
        ///
        /// \returns the path used for caches for this application on success or NULL
        ///          on failure; call SDL_GetError() for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetAndroidCachePath() -> *const ::core::ffi::c_char;
    }}

    pub type SDL_RequestAndroidPermissionCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, permission: *const ::core::ffi::c_char, granted: SDL_bool))>;

    extern_sdlcall! {{
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
        /// \param permission the permission to request.
        /// \param cb the callback to trigger when the request has a response.
        /// \param userdata an app-controlled pointer that is passed to the callback.
        /// \returns SDL_TRUE if the request was submitted, SDL_FALSE if there was an
        ///          error submitting. The result of the request is only ever reported
        ///          through the callback, not this return value.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_RequestAndroidPermission(permission: *const ::core::ffi::c_char, cb: SDL_RequestAndroidPermissionCallback, userdata: *mut ::core::ffi::c_void) -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Shows an Android toast notification.
        ///
        /// Toasts are a sort of lightweight notification that are unique to Android.
        ///
        /// https://developer.android.com/guide/topics/ui/notifiers/toasts
        ///
        /// Shows toast in UI thread.
        ///
        /// For the `gravity` parameter, choose a value from here, or -1 if you don't
        /// have a preference:
        ///
        /// https://developer.android.com/reference/android/view/Gravity
        ///
        /// \param message text message to be shown.
        /// \param duration 0=short, 1=long.
        /// \param gravity where the notification should appear on the screen.
        /// \param xoffset set this parameter only when gravity >=0.
        /// \param yoffset set this parameter only when gravity >=0.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_ShowAndroidToast(message: *const ::core::ffi::c_char, duration: ::core::ffi::c_int, gravity: ::core::ffi::c_int, xoffset: ::core::ffi::c_int, yoffset: ::core::ffi::c_int) -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Send a user command to SDLActivity.
        ///
        /// Override "boolean onUnhandledMessage(Message msg)" to handle the message.
        ///
        /// \param command user command that must be greater or equal to 0x8000.
        /// \param param user parameter.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_SendAndroidMessage(command: Uint32, param: ::core::ffi::c_int) -> SDL_bool;
    }}

}

extern_sdlcall! {{
    /// Query if the current device is a tablet.
    ///
    /// If SDL can't determine this, it will return SDL_FALSE.
    ///
    /// \returns SDL_TRUE if the device is a tablet, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_IsTablet() -> SDL_bool;
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationWillTerminate();
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationDidReceiveMemoryWarning();
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationWillEnterBackground();
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationDidEnterBackground();
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationWillEnterForeground();
}}

extern_sdlcall! {{
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
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationDidEnterForeground();
}}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
emit! {
    extern_sdlcall! {{
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
        /// \threadsafety It is safe to call this function from any thread.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_OnApplicationDidChangeStatusBarOrientation();
    }}

}

#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))]
emit! {
    pub type XTaskQueueHandle = *mut XTaskQueueObject;

    pub type XUserHandle = *mut XUser;

    extern_sdlcall! {{
        /// Gets a reference to the global async task queue handle for GDK,
        /// initializing if needed.
        ///
        /// Once you are done with the task queue, you should call
        /// XTaskQueueCloseHandle to reduce the reference count to avoid a resource
        /// leak.
        ///
        /// \param outTaskQueue a pointer to be filled in with task queue handle.
        /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetGDKTaskQueue(outTaskQueue: *mut XTaskQueueHandle) -> SDL_bool;
    }}

    extern_sdlcall! {{
        /// Gets a reference to the default user handle for GDK.
        ///
        /// This is effectively a synchronous version of XUserAddAsync, which always
        /// prefers the default user and allows a sign-in UI.
        ///
        /// \param outUserHandle a pointer to be filled in with the default user
        ///                      handle.
        /// \returns SDL_TRUE if success or SDL_FALSE on failure; call SDL_GetError()
        ///          for more information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_GetGDKDefaultUser(outUserHandle: *mut XUserHandle) -> SDL_bool;
    }}

}

#[repr(C)]
#[non_exhaustive]
pub struct XTaskQueueObject { _opaque: [::core::primitive::u8; 0] }

#[repr(C)]
#[non_exhaustive]
pub struct XUser { _opaque: [::core::primitive::u8; 0] }

#[repr(C)]
#[non_exhaustive]
pub struct tagMSG { _opaque: [::core::primitive::u8; 0] }

#[repr(C)]
#[non_exhaustive]
pub struct _XEvent { _opaque: [::core::primitive::u8; 0] }
