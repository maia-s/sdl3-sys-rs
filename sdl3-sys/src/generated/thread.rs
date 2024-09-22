//! SDL thread management routines.

use super::stdinc::*;

use super::error::*;

use super::properties::*;

use super::atomic::*;

#[cfg(windows)]
emit! {}

/// A unique numeric ID that identifies a thread.
///
/// These are different from SDL_Thread objects, which are generally what an
/// application will operate on, but having a way to uniquely identify a thread
/// can be useful at times.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_GetThreadID
/// \sa SDL_GetCurrentThreadID
pub type SDL_ThreadID = Uint64;

/// Thread local storage ID.
///
/// 0 is the invalid ID. An app can create these and then set data for these
/// IDs that is unique to each thread.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_GetTLS
/// \sa SDL_SetTLS
pub type SDL_TLSID = SDL_AtomicInt;

/// The SDL thread priority.
///
/// SDL will make system changes as necessary in order to apply the thread
/// priority. Code which attempts to control thread state related to priority
/// should be aware that calling SDL_SetThreadPriority may alter such state.
/// SDL_HINT_THREAD_PRIORITY_POLICY can be used to control aspects of this
/// behavior.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_THREAD_PRIORITY_LOW`], [`SDL_THREAD_PRIORITY_NORMAL`], [`SDL_THREAD_PRIORITY_HIGH`], [`SDL_THREAD_PRIORITY_TIME_CRITICAL`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_ThreadPriority(pub ::core::ffi::c_int);
impl SDL_ThreadPriority {
    pub const LOW: Self = Self(0);
    pub const NORMAL: Self = Self(1);
    pub const HIGH: Self = Self(2);
    pub const TIME_CRITICAL: Self = Self(3);
}
pub const SDL_THREAD_PRIORITY_LOW: SDL_ThreadPriority = SDL_ThreadPriority::LOW;
pub const SDL_THREAD_PRIORITY_NORMAL: SDL_ThreadPriority = SDL_ThreadPriority::NORMAL;
pub const SDL_THREAD_PRIORITY_HIGH: SDL_ThreadPriority = SDL_ThreadPriority::HIGH;
pub const SDL_THREAD_PRIORITY_TIME_CRITICAL: SDL_ThreadPriority = SDL_ThreadPriority::TIME_CRITICAL;

/// The function passed to SDL_CreateThread() as the new thread's entry point.
///
/// \param data what was passed as `data` to SDL_CreateThread().
/// \returns a value that can be reported through SDL_WaitThread().
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_ThreadFunction =
    ::core::option::Option<extern "C" fn(data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

#[cfg(doc)]
emit! {
    extern "C" {
        /// Create a new thread with a default stack size.
        ///
        /// This is a convenience function, equivalent to calling
        /// SDL_CreateThreadWithProperties with the following properties set:
        ///
        /// - `SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER`: `fn`
        /// - `SDL_PROP_THREAD_CREATE_NAME_STRING`: `name`
        /// - `SDL_PROP_THREAD_CREATE_USERDATA_POINTER`: `data`
        ///
        /// Note that this "function" is actually a macro that calls an internal
        /// function with two extra parameters not listed here; they are hidden through
        /// preprocessor macros and are needed to support various C runtimes at the
        /// point of the function call. Language bindings that aren't using the C
        /// headers will need to deal with this.
        ///
        /// Usually, apps should just call this function the same way on every platform
        /// and let the macros hide the details.
        ///
        /// \param fn the SDL_ThreadFunction function to call in the new thread.
        /// \param name the name of the thread.
        /// \param data a pointer that is passed to `fn`.
        /// \returns an opaque pointer to the new thread object on success, NULL if the
        ///          new thread could not be created; call SDL_GetError() for more
        ///          information.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_CreateThreadWithProperties
        /// \sa SDL_WaitThread
        pub fn SDL_CreateThread(r#fn: SDL_ThreadFunction, name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void) -> *mut SDL_Thread;
    }

    extern "C" {
        /// Create a new thread with with the specified properties.
        ///
        /// These are the supported properties:
        ///
        /// - `SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER`: an SDL_ThreadFunction
        ///   value that will be called at the start of the new thread's life.
        ///   Required.
        /// - `SDL_PROP_THREAD_CREATE_NAME_STRING`: the name of the new thread, which
        ///   might be available to debuggers. Optional, defaults to NULL.
        /// - `SDL_PROP_THREAD_CREATE_USERDATA_POINTER`: an arbitrary app-defined
        ///   pointer, which is passed to the entry function on the new thread, as its
        ///   only parameter. Optional, defaults to NULL.
        /// - `SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER`: the size, in bytes, of the new
        ///   thread's stack. Optional, defaults to 0 (system-defined default).
        ///
        /// SDL makes an attempt to report `SDL_PROP_THREAD_CREATE_NAME_STRING` to the
        /// system, so that debuggers can display it. Not all platforms support this.
        ///
        /// Thread naming is a little complicated: Most systems have very small limits
        /// for the string length (Haiku has 32 bytes, Linux currently has 16, Visual
        /// C++ 6.0 has _nine_!), and possibly other arbitrary rules. You'll have to
        /// see what happens with your system's debugger. The name should be UTF-8 (but
        /// using the naming limits of C identifiers is a better bet). There are no
        /// requirements for thread naming conventions, so long as the string is
        /// null-terminated UTF-8, but these guidelines are helpful in choosing a name:
        ///
        /// https://stackoverflow.com/questions/149932/naming-conventions-for-threads
        ///
        /// If a system imposes requirements, SDL will try to munge the string for it
        /// (truncate, etc), but the original string contents will be available from
        /// SDL_GetThreadName().
        ///
        /// The size (in bytes) of the new stack can be specified with
        /// `SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER`. Zero means "use the system
        /// default" which might be wildly different between platforms. x86 Linux
        /// generally defaults to eight megabytes, an embedded device might be a few
        /// kilobytes instead. You generally need to specify a stack that is a multiple
        /// of the system's page size (in many cases, this is 4 kilobytes, but check
        /// your system documentation).
        ///
        /// Note that this "function" is actually a macro that calls an internal
        /// function with two extra parameters not listed here; they are hidden through
        /// preprocessor macros and are needed to support various C runtimes at the
        /// point of the function call. Language bindings that aren't using the C
        /// headers will need to deal with this.
        ///
        /// The actual symbol in SDL is `SDL_CreateThreadWithPropertiesRuntime`, so
        /// there is no symbol clash, but trying to load an SDL shared library and look
        /// for "SDL_CreateThreadWithProperties" will fail.
        ///
        /// Usually, apps should just call this function the same way on every platform
        /// and let the macros hide the details.
        ///
        /// \param props the properties to use.
        /// \returns an opaque pointer to the new thread object on success, NULL if the
        ///          new thread could not be created; call SDL_GetError() for more
        ///          information.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_CreateThread
        /// \sa SDL_WaitThread
        pub fn SDL_CreateThreadWithProperties(props: SDL_PropertiesID) -> *mut SDL_Thread;
    }

    pub const SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.entry_function\0") };

    pub const SDL_PROP_THREAD_CREATE_NAME_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.name\0") };

    pub const SDL_PROP_THREAD_CREATE_USERDATA_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.userdata\0") };

    pub const SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.stacksize\0") };

}

#[cfg(not(doc))]
emit! {
    #[cfg(windows)]
    emit! {
    }

}

#[cfg(not(doc))]
emit! {}

#[cfg(not(doc))]
emit! {}

#[cfg(not(doc))]
emit! {
    extern "C" {
        /// The actual entry point for SDL_CreateThread.
        ///
        /// \param fn the SDL_ThreadFunction function to call in the new thread
        /// \param name the name of the thread
        /// \param data a pointer that is passed to `fn`
        /// \param pfnBeginThread the C runtime's _beginthreadex (or whatnot). Can be NULL.
        /// \param pfnEndThread the C runtime's _endthreadex (or whatnot). Can be NULL.
        /// \returns an opaque pointer to the new thread object on success, NULL if the
        ///          new thread could not be created; call SDL_GetError() for more
        ///          information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_CreateThreadRuntime(r#fn: SDL_ThreadFunction, name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void, pfnBeginThread: SDL_FunctionPointer, pfnEndThread: SDL_FunctionPointer) -> *mut SDL_Thread;
    }

    extern "C" {
        /// The actual entry point for SDL_CreateThreadWithProperties.
        ///
        /// \param props the properties to use
        /// \param pfnBeginThread the C runtime's _beginthreadex (or whatnot). Can be NULL.
        /// \param pfnEndThread the C runtime's _endthreadex (or whatnot). Can be NULL.
        /// \returns an opaque pointer to the new thread object on success, NULL if the
        ///          new thread could not be created; call SDL_GetError() for more
        ///          information.
        ///
        /// \since This function is available since SDL 3.0.0.
        pub fn SDL_CreateThreadWithPropertiesRuntime(props: SDL_PropertiesID, pfnBeginThread: SDL_FunctionPointer, pfnEndThread: SDL_FunctionPointer) -> *mut SDL_Thread;
    }

    #[cfg(all(not(doc), not(windows)))]
    pub const SDL_BeginThreadFunction: SDL_FunctionPointer = unsafe { ::core::mem::transmute::<*const ::core::ffi::c_void, SDL_FunctionPointer>(core::ptr::null()) };
    #[cfg(all(not(doc), not(windows)))]
    pub const SDL_EndThreadFunction: SDL_FunctionPointer = unsafe { ::core::mem::transmute::<*const ::core::ffi::c_void, SDL_FunctionPointer>(core::ptr::null()) };
    #[cfg(all(not(doc), windows))]
    extern "cdecl" {
        fn _beginthreadex(security: *mut ::core::ffi::c_void, stack_size: ::core::ffi::c_uint, start_address: Option<extern "stdcall" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_uint>, arglist: *mut ::core::ffi::c_void, initflag: ::core::ffi::c_uint, thrdaddr: ::core::ffi::c_uint) -> ::core::primitive::usize;
        fn _endthreadex(retval: ::core::ffi::c_uint);
    }
    #[cfg(all(not(doc), windows))]
    pub const SDL_BeginThreadFunction: SDL_FunctionPointer = unsafe { ::core::mem::transmute::<unsafe extern "cdecl" fn(*mut ::core::ffi::c_void, ::core::ffi::c_uint, Option<extern "stdcall" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_uint>, *mut ::core::ffi::c_void, ::core::ffi::c_uint, ::core::ffi::c_uint) -> ::core::primitive::usize, SDL_FunctionPointer>(_beginthreadex) };
    #[cfg(all(not(doc), windows))]
    pub const SDL_EndThreadFunction: SDL_FunctionPointer = unsafe { ::core::mem::transmute::<unsafe extern "cdecl" fn (::core::ffi::c_uint), SDL_FunctionPointer>(_endthreadex) };

    #[inline(always)]
    pub unsafe fn SDL_CreateThread(r#fn: SDL_ThreadFunction, name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void, ) -> *mut SDL_Thread {
        unsafe { SDL_CreateThreadRuntime((r#fn), (name), (data), ((SDL_BeginThreadFunction) as SDL_FunctionPointer), ((SDL_EndThreadFunction) as SDL_FunctionPointer)) }
    }


    #[inline(always)]
    pub unsafe fn SDL_CreateThreadWithProperties(props: SDL_PropertiesID, ) -> *mut SDL_Thread {
        unsafe { SDL_CreateThreadWithPropertiesRuntime((props), ((SDL_BeginThreadFunction) as SDL_FunctionPointer), ((SDL_EndThreadFunction) as SDL_FunctionPointer)) }
    }


    pub const SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.entry_function\0") };

    pub const SDL_PROP_THREAD_CREATE_NAME_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.name\0") };

    pub const SDL_PROP_THREAD_CREATE_USERDATA_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.userdata\0") };

    pub const SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.thread.create.stacksize\0") };

}

extern "C" {
    /// Get the thread name as it was specified in SDL_CreateThread().
    ///
    /// \param thread the thread to query.
    /// \returns a pointer to a UTF-8 string that names the specified thread, or
    ///          NULL if it doesn't have a name.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetThreadName(thread: *mut SDL_Thread) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the thread identifier for the current thread.
    ///
    /// This thread identifier is as reported by the underlying operating system.
    /// If SDL is running on a platform that does not support threads the return
    /// value will always be zero.
    ///
    /// This function also returns a valid thread ID when called from the main
    /// thread.
    ///
    /// \returns the ID of the current thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetThreadID
    pub fn SDL_GetCurrentThreadID() -> SDL_ThreadID;
}

extern "C" {
    /// Get the thread identifier for the specified thread.
    ///
    /// This thread identifier is as reported by the underlying operating system.
    /// If SDL is running on a platform that does not support threads the return
    /// value will always be zero.
    ///
    /// \param thread the thread to query.
    /// \returns the ID of the specified thread, or the ID of the current thread if
    ///          `thread` is NULL.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetCurrentThreadID
    pub fn SDL_GetThreadID(thread: *mut SDL_Thread) -> SDL_ThreadID;
}

extern "C" {
    /// Set the priority for the current thread.
    ///
    /// Note that some platforms will not let you alter the priority (or at least,
    /// promote the thread to a higher priority) at all, and some require you to be
    /// an administrator account. Be prepared for this to fail.
    ///
    /// \param priority the SDL_ThreadPriority to set.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetThreadPriority(priority: SDL_ThreadPriority) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait for a thread to finish.
    ///
    /// Threads that haven't been detached will remain (as a "zombie") until this
    /// function cleans them up. Not doing so is a resource leak.
    ///
    /// Once a thread has been cleaned up through this function, the SDL_Thread
    /// that references it becomes invalid and should not be referenced again. As
    /// such, only one thread may call SDL_WaitThread() on another.
    ///
    /// The return code for the thread function is placed in the area pointed to by
    /// `status`, if `status` is not NULL.
    ///
    /// You may not wait on a thread that has been used in a call to
    /// SDL_DetachThread(). Use either that function or this one, but not both, or
    /// behavior is undefined.
    ///
    /// It is safe to pass a NULL thread to this function; it is a no-op.
    ///
    /// Note that the thread pointer is freed by this function and is not valid
    /// afterward.
    ///
    /// \param thread the SDL_Thread pointer that was returned from the
    ///               SDL_CreateThread() call that started this thread.
    /// \param status pointer to an integer that will receive the value returned
    ///               from the thread function by its 'return', or NULL to not
    ///               receive such value back.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateThread
    /// \sa SDL_DetachThread
    pub fn SDL_WaitThread(thread: *mut SDL_Thread, status: *mut ::core::ffi::c_int);
}

extern "C" {
    /// Let a thread clean up on exit without intervention.
    ///
    /// A thread may be "detached" to signify that it should not remain until
    /// another thread has called SDL_WaitThread() on it. Detaching a thread is
    /// useful for long-running threads that nothing needs to synchronize with or
    /// further manage. When a detached thread is done, it simply goes away.
    ///
    /// There is no way to recover the return code of a detached thread. If you
    /// need this, don't detach the thread and instead use SDL_WaitThread().
    ///
    /// Once a thread is detached, you should usually assume the SDL_Thread isn't
    /// safe to reference again, as it will become invalid immediately upon the
    /// detached thread's exit, instead of remaining until someone has called
    /// SDL_WaitThread() to finally clean it up. As such, don't detach the same
    /// thread more than once.
    ///
    /// If a thread has already exited when passed to SDL_DetachThread(), it will
    /// stop waiting for a call to SDL_WaitThread() and clean up immediately. It is
    /// not safe to detach a thread that might be used with SDL_WaitThread().
    ///
    /// You may not call SDL_WaitThread() on a thread that has been detached. Use
    /// either that function or this one, but not both, or behavior is undefined.
    ///
    /// It is safe to pass NULL to this function; it is a no-op.
    ///
    /// \param thread the SDL_Thread pointer that was returned from the
    ///               SDL_CreateThread() call that started this thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateThread
    /// \sa SDL_WaitThread
    pub fn SDL_DetachThread(thread: *mut SDL_Thread);
}

extern "C" {
    /// Get the current thread's value associated with a thread local storage ID.
    ///
    /// \param id a pointer to the thread local storage ID, may not be NULL.
    /// \returns the value associated with the ID for the current thread or NULL if
    ///          no value has been set; call SDL_GetError() for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTLS
    pub fn SDL_GetTLS(id: *mut SDL_TLSID) -> *mut ::core::ffi::c_void;
}

/// The callback used to cleanup data passed to SDL_SetTLS.
///
/// This is called when a thread exits, to allow an app to free any resources.
///
/// \param value a pointer previously handed to SDL_SetTLS.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_SetTLS
pub type SDL_TLSDestructorCallback =
    ::core::option::Option<extern "C" fn(value: *mut ::core::ffi::c_void)>;

extern "C" {
    /// Set the current thread's value associated with a thread local storage ID.
    ///
    /// If the thread local storage ID is not initialized (the value is 0), a new
    /// ID will be created in a thread-safe way, so all calls using a pointer to
    /// the same ID will refer to the same local storage.
    ///
    /// Note that replacing a value from a previous call to this function on the
    /// same thread does _not_ call the previous value's destructor!
    ///
    /// `destructor` can be NULL; it is assumed that `value` does not need to be
    /// cleaned up if so.
    ///
    /// \param id a pointer to the thread local storage ID, may not be NULL.
    /// \param value the value to associate with the ID for the current thread.
    /// \param destructor a function called when the thread exits, to free the
    ///                   value, may be NULL.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTLS
    pub fn SDL_SetTLS(
        id: *mut SDL_TLSID,
        value: *const ::core::ffi::c_void,
        destructor: SDL_TLSDestructorCallback,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Cleanup all TLS data for this thread.
    ///
    /// If you are creating your threads outside of SDL and then calling SDL
    /// functions, you should call this function before your thread exits, to
    /// properly clean up SDL memory.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CleanupTLS();
}

/// The SDL thread object.
///
/// These are opaque data.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_CreateThread
/// \sa SDL_WaitThread
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Thread {
    _opaque: [::core::primitive::u8; 0],
}
