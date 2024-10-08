//! Platform-specific SDL API functions.

use super::stdinc::*;

use super::error::*;

use super::keyboard::*;

use super::video::*;

#[cfg(windows)]
emit! {
    pub type MSG = tagMSG;

    /// A callback to be used with [`SDL_SetWindowsMessageHook`].
    ///
    /// This callback may modify the message, and should return true if the message
    /// should continue to be processed, or false to prevent further processing.
    ///
    /// As this is processing a message directly from the Windows event loop, this
    /// callback should do the minimum required work and return quickly.
    ///
    /// - `userdata`: the app-defined pointer provided to
    ///   [`SDL_SetWindowsMessageHook`].
    /// - `msg`: a pointer to a Win32 event structure to process.
    /// - Returns true to let event continue on, false to drop it.
    ///
    /// Thread safety: This may only be called (by SDL) from the thread handling the
    ///   Windows event loop.
    ///
    /// This datatype is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetWindowsMessageHook`]<br>
    /// See also [`SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP`]<br>
    pub type SDL_WindowsMessageHook = ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, msg: *mut MSG) -> ::core::primitive::bool>;

    extern "C" {
        /// Set a callback for every Windows message, run before TranslateMessage().
        ///
        /// The callback may modify the message, and should return true if the message
        /// should continue to be processed, or false to prevent further processing.
        ///
        /// - `callback`: the [`SDL_WindowsMessageHook`] function to call.
        /// - `userdata`: a pointer to pass to every iteration of `callback`.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_WindowsMessageHook`]<br>
        /// See also [`SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP`]<br>
        pub fn SDL_SetWindowsMessageHook(callback: SDL_WindowsMessageHook, userdata: *mut ::core::ffi::c_void);
    }

    #[repr(C)]
    #[non_exhaustive]
    pub struct tagMSG { _opaque: [::core::primitive::u8; 0] }

}

#[cfg(any(windows, any(/* always disabled: SDL_PLATFORM_WINGDK */)))]
emit! {
    extern "C" {
        /// Get the D3D9 adapter index that matches the specified display.
        ///
        /// The returned adapter index can be passed to `IDirect3D9::CreateDevice` and
        /// controls on which monitor a full screen application will appear.
        ///
        /// - `displayID`: the instance of the display to query.
        /// - Returns the D3D9 adapter index on success or -1 on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_GetDirect3D9AdapterIndex(displayID: SDL_DisplayID) -> ::core::ffi::c_int;
    }

    extern "C" {
        /// Get the DXGI Adapter and Output indices for the specified display.
        ///
        /// The DXGI Adapter and Output indices can be passed to `EnumAdapters` and
        /// `EnumOutputs` respectively to get the objects required to create a DX10 or
        /// DX11 device and swap chain.
        ///
        /// - `displayID`: the instance of the display to query.
        /// - `adapterIndex`: a pointer to be filled in with the adapter index.
        /// - `outputIndex`: a pointer to be filled in with the output index.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_GetDXGIOutputInfo(displayID: SDL_DisplayID, adapterIndex: *mut ::core::ffi::c_int, outputIndex: *mut ::core::ffi::c_int) -> ::core::primitive::bool;
    }

}

pub type XEvent = _XEvent;

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
    /// - `callback`: the [`SDL_X11EventHook`] function to call.
    /// - `userdata`: a pointer to pass to every iteration of `callback`.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_SetX11EventHook(callback: SDL_X11EventHook, userdata: *mut ::core::ffi::c_void);
}

#[cfg(target_os = "linux")]
emit! {
    extern "C" {
        /// Sets the UNIX nice value for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// - `threadID`: the Unix thread ID to change priority of.
        /// - `priority`: the new, Unix-specific, priority value.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_SetLinuxThreadPriority(threadID: Sint64, priority: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Sets the priority (not nice level) and scheduling policy for a thread.
        ///
        /// This uses setpriority() if possible, and RealtimeKit if available.
        ///
        /// - `threadID`: the Unix thread ID to change priority of.
        /// - `sdlPriority`: the new [`SDL_ThreadPriority`] value.
        /// - `schedPolicy`: the new scheduling policy (SCHED_FIFO, SCHED_RR,
        ///   SCHED_OTHER, etc...).
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_SetLinuxThreadPriorityAndPolicy(threadID: Sint64, sdlPriority: ::core::ffi::c_int, schedPolicy: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
emit! {
    /// The prototype for an Apple iOS animation callback.
    ///
    /// This datatype is only useful on Apple iOS.
    ///
    /// After passing a function pointer of this type to
    /// [`SDL_SetiOSAnimationCallback`], the system will call that function pointer at
    /// a regular interval.
    ///
    /// - `userdata`: what was passed as `callbackParam` to
    ///   [`SDL_SetiOSAnimationCallback`] as `callbackParam`.
    ///
    /// This datatype is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetiOSAnimationCallback`]<br>
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
        /// https://wiki.libsdl.org/SDL3/README/ios
        ///
        /// Note that if you use the "main callbacks" instead of a standard C `main`
        /// function, you don't have to use this API, as SDL will manage this for you.
        ///
        /// Details on main callbacks are here:
        ///
        /// https://wiki.libsdl.org/SDL3/README/main-functions
        ///
        /// - `window`: the window for which the animation callback should be set.
        /// - `interval`: the number of frames after which **callback** will be
        ///   called.
        /// - `callback`: the function to call for every frame.
        /// - `callbackParam`: a pointer that is passed to `callback`.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_SetiOSEventPump`]<br>
        pub fn SDL_SetiOSAnimationCallback(window: *mut SDL_Window, interval: ::core::ffi::c_int, callback: SDL_iOSAnimationCallback, callbackParam: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Use this function to enable or disable the SDL event pump on Apple iOS.
        ///
        /// This function is only available on Apple iOS.
        ///
        /// - `enabled`: true to enable the event pump, false to disable it.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_SetiOSAnimationCallback`]<br>
        pub fn SDL_SetiOSEventPump(enabled: ::core::primitive::bool);
    }

}

#[cfg(target_os = "android")]
emit! {
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
        /// - Returns a pointer to Java native interface object (JNIEnv) to which the
        ///   current thread is attached, or NULL on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_GetAndroidActivity`]<br>
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
        /// https://docs.oracle.com/javase/1.5.0/docs/guide/jni/spec/functions.html
        ///
        /// - Returns the jobject representing the instance of the Activity class of the
        ///   Android application, or NULL on failure; call [`SDL_GetError()`] for
        ///   more information.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_GetAndroidJNIEnv`]<br>
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
        /// - Returns the Android API level.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_GetAndroidSDKVersion() -> ::core::ffi::c_int;
    }

    extern "C" {
        /// Query if the application is running on a Chromebook.
        ///
        /// - Returns true if this is a Chromebook, false otherwise.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_IsChromebook() -> ::core::primitive::bool;
    }

    extern "C" {
        /// Query if the application is running on a Samsung DeX docking station.
        ///
        /// - Returns true if this is a DeX docking station, false otherwise.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_IsDeXMode() -> ::core::primitive::bool;
    }

    extern "C" {
        /// Trigger the Android system back button behavior.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_SendAndroidBackButton();
    }

    /// See the official Android developer guide for more information:
    /// http://developer.android.com/guide/topics/data/data-storage.html
    ///
    /// This macro is available since SDL 3.0.0.
    pub const SDL_ANDROID_EXTERNAL_STORAGE_READ: ::core::primitive::i32 = 1;

    pub const SDL_ANDROID_EXTERNAL_STORAGE_WRITE: ::core::primitive::i32 = 2;

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
        /// https://developer.android.com/reference/android/content/Context#getFilesDir()
        ///
        /// - Returns the path used for internal storage or NULL on failure; call
        ///   [`SDL_GetError()`] for more information.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_GetAndroidExternalStorageState`]<br>
        pub fn SDL_GetAndroidInternalStoragePath() -> *const ::core::ffi::c_char;
    }

    extern "C" {
        /// Get the current state of external storage for this Android application.
        ///
        /// The current state of external storage, a bitmask of these values:
        /// `SDL_ANDROID_EXTERNAL_STORAGE_READ`, `SDL_ANDROID_EXTERNAL_STORAGE_WRITE`.
        ///
        /// If external storage is currently unavailable, this will return 0.
        ///
        /// - Returns the current state of external storage, or 0 if external storage is
        ///   currently unavailable.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_GetAndroidExternalStoragePath`]<br>
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
        /// https://developer.android.com/reference/android/content/Context#getExternalFilesDir()
        ///
        /// - Returns the path used for external storage for this application on success
        ///   or NULL on failure; call [`SDL_GetError()`] for more information.
        ///
        /// This function is available since SDL 3.0.0.
        ///
        /// See also [`SDL_GetAndroidExternalStorageState`]<br>
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
        /// https://developer.android.com/reference/android/content/Context#getCacheDir()
        ///
        /// - Returns the path used for caches for this application on success or NULL
        ///   on failure; call [`SDL_GetError()`] for more information.
        ///
        /// This function is available since SDL 3.0.0.
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
        /// - `permission`: the permission to request.
        /// - `cb`: the callback to trigger when the request has a response.
        /// - `userdata`: an app-controlled pointer that is passed to the callback.
        /// - Returns true if the request was submitted, false if there was an error
        ///   submitting. The result of the request is only ever reported
        ///   through the callback, not this return value.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_RequestAndroidPermission(permission: *const ::core::ffi::c_char, cb: SDL_RequestAndroidPermissionCallback, userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool;
    }

    extern "C" {
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
        /// - `message`: text message to be shown.
        /// - `duration`: 0=short, 1=long.
        /// - `gravity`: where the notification should appear on the screen.
        /// - `xoffset`: set this parameter only when gravity >=0.
        /// - `yoffset`: set this parameter only when gravity >=0.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_ShowAndroidToast(message: *const ::core::ffi::c_char, duration: ::core::ffi::c_int, gravity: ::core::ffi::c_int, xoffset: ::core::ffi::c_int, yoffset: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Send a user command to SDLActivity.
        ///
        /// Override "boolean onUnhandledMessage(Message msg)" to handle the message.
        ///
        /// - `command`: user command that must be greater or equal to 0x8000.
        /// - `param`: user parameter.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_SendAndroidMessage(command: Uint32, param: ::core::ffi::c_int) -> ::core::primitive::bool;
    }

}

extern "C" {
    /// Query if the current device is a tablet.
    ///
    /// If SDL can't determine this, it will return false.
    ///
    /// - Returns true if the device is a tablet, false otherwise.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_IsTablet() -> ::core::primitive::bool;
}

extern "C" {
    /// Query if the current device is a TV.
    ///
    /// If SDL can't determine this, it will return false.
    ///
    /// - Returns true if the device is a TV, false otherwise.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_IsTV() -> ::core::primitive::bool;
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
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
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_OnApplicationDidEnterForeground();
}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
emit! {
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
        /// Thread safety: It is safe to call this function from any thread.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_OnApplicationDidChangeStatusBarOrientation();
    }

}

#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))]
emit! {
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
        /// - `outTaskQueue`: a pointer to be filled in with task queue handle.
        /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_GetGDKTaskQueue(outTaskQueue: *mut XTaskQueueHandle) -> ::core::primitive::bool;
    }

    extern "C" {
        /// Gets a reference to the default user handle for GDK.
        ///
        /// This is effectively a synchronous version of XUserAddAsync, which always
        /// prefers the default user and allows a sign-in UI.
        ///
        /// - `outUserHandle`: a pointer to be filled in with the default user
        ///   handle.
        /// - Returns true if success or false on failure; call [`SDL_GetError()`] for more
        ///   information.
        ///
        /// This function is available since SDL 3.0.0.
        pub fn SDL_GetGDKDefaultUser(outUserHandle: *mut XUserHandle) -> ::core::primitive::bool;
    }

    #[repr(C)]
    #[non_exhaustive]
    pub struct XTaskQueueObject { _opaque: [::core::primitive::u8; 0] }

    #[repr(C)]
    #[non_exhaustive]
    pub struct XUser { _opaque: [::core::primitive::u8; 0] }

}

#[repr(C)]
#[non_exhaustive]
pub struct _XEvent {
    _opaque: [::core::primitive::u8; 0],
}
