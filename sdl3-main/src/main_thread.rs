use core::{
    cell::Cell,
    ffi::c_void,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::{addr_of_mut, NonNull},
    sync::atomic::Ordering,
};
use sdl3_sys::{
    init::{SDL_IsMainThread, SDL_RunOnMainThread},
    stdinc::{SDL_aligned_alloc, SDL_aligned_free},
    thread::{SDL_GetCurrentThreadID, SDL_ThreadID},
    timer::SDL_Delay,
};

#[cfg(target_has_atomic = "8")]
type AtomicLeastU8 = core::sync::atomic::AtomicU8;
#[cfg(all(not(target_has_atomic = "8"), target_has_atomic = "16"))]
type AtomicLeastU8 = core::sync::atomic::AtomicU16;
#[cfg(all(
    not(any(target_has_atomic = "8", target_has_atomic = "16")),
    target_has_atomic = "32"
))]
type AtomicLeastU8 = core::sync::atomic::AtomicU32;
#[cfg(all(
    not(any(
        target_has_atomic = "8",
        target_has_atomic = "16",
        target_has_atomic = "32"
    )),
    target_has_atomic = "64"
))]
type AtomicLeastU8 = core::sync::atomic::AtomicU64;
#[cfg(all(
    not(any(
        target_has_atomic = "8",
        target_has_atomic = "16",
        target_has_atomic = "32",
        target_has_atomic = "64"
    )),
    target_has_atomic = "128"
))]
type AtomicLeastU8 = core::sync::atomic::AtomicU128;
#[cfg(not(any(
    target_has_atomic = "8",
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
    target_has_atomic = "128"
)))]
compile_error!("no supported atomic integer type");

/// Zero sized token that can only exist on the main thread.
///
/// Call [`MainThreadToken::get()`] or [`MainThreadToken::assert()`] to get one.
///
/// As of `sdl3-main` 0.6, it's not required to call [`MainThreadToken::init()`].
/// If SDL has been inited properly, only the thread that SDL considers the main thread
/// can get a `MainThreadToken`. Otherwise, the first call to either of [`MainThreadToken::get()`]
/// or [`MainThreadToken::assert()`] will determine which thread `MainThreadToken` considers
/// the main thread for the lifetime of the process. [`MainThreadToken::init()`] is equivalent
/// to [`MainThreadToken::assert()`] and is retained for backwards compatibility.
#[derive(Clone, Copy)]
pub struct MainThreadToken(PhantomData<*const ()>);

impl MainThreadToken {
    /// Get `Some(MainThreadToken)` if called on the main thread, or `None` otherwise.
    ///
    /// On targets that don't support threads, this will always succeed.
    ///
    /// See also [`MainThreadToken::assert()`]
    pub fn get() -> Option<Self> {
        struct ThreadId(Cell<SDL_ThreadID>);
        unsafe impl Sync for ThreadId {}

        static MAIN_THREAD_ID: ThreadId = ThreadId(Cell::new(SDL_ThreadID(0)));
        static MAIN_THREAD_ID_STATUS: AtomicLeastU8 = AtomicLeastU8::new(0);

        loop {
            match MAIN_THREAD_ID_STATUS.load(Ordering::Acquire) {
                0 => {
                    // if this returns false we know it's not the main thread, but true may be
                    // unreliable depending on how/if SDL was inited and the version of SDL
                    if !unsafe { SDL_IsMainThread() } {
                        return None;
                    }
                    if MAIN_THREAD_ID_STATUS
                        .compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed)
                        .is_ok()
                    {
                        MAIN_THREAD_ID.0.set(SDL_GetCurrentThreadID());
                        MAIN_THREAD_ID_STATUS.store(2, Ordering::Release);
                        return Some(Self(PhantomData));
                    }
                }
                1 => (),
                _ => {
                    return (MAIN_THREAD_ID.0.get() == SDL_GetCurrentThreadID())
                        .then_some(Self(PhantomData));
                }
            }
            unsafe { SDL_Delay(0) }
        }
    }

    /// Get `MainThreadToken` if called on the main thread, or panic otherwise.
    ///
    /// On targets that don't support threads, this will always succeed.
    ///
    /// See also [`MainThreadToken::get()`]
    #[track_caller]
    pub fn assert() -> Self {
        Self::get().expect("This operation can only be performed on the main thread")
    }

    /// Initialize `MainThreadToken` (if it wasn't already) if called on the main thread,
    /// or panic otherwise.
    ///
    /// As of `sdl3-main` 0.6, this function is equivalent to [`MainThreadToken::assert()`].
    ///
    /// On targets that don't support threads, this will always succeed.
    #[track_caller]
    #[inline(always)]
    pub fn init() {
        Self::assert();
    }
}

/// Data that can only be accessed from the main thread. Accessors take a [`MainThreadToken`].
///
/// This can be moved freely between threads, but the Drop implementation will panic if it's
/// dropped from a thread other than the main thread.
#[repr(transparent)]
pub struct MainThreadData<T>(T);

unsafe impl<T> Send for MainThreadData<T> {}
unsafe impl<T> Sync for MainThreadData<T> {}

impl<T> Drop for MainThreadData<T> {
    fn drop(&mut self) {
        MainThreadToken::assert();
    }
}

impl<T> MainThreadData<T> {
    /// Create a new `MainThreadData`.
    ///
    /// See also [`Self::assert_new()`]
    #[inline(always)]
    pub fn new(_: MainThreadToken, data: T) -> Self {
        Self(data)
    }

    /// Get shared access to this data.
    ///
    /// See also [`Self::assert_get()`], [`Self::get_on_main_thread()`]
    #[inline(always)]
    pub fn get(&self, _: MainThreadToken) -> &T {
        &self.0
    }

    /// Get exclusive access to this data.
    ///
    /// See also [`Self::assert_get_mut()`], [`Self::get_mut_on_main_thread()`]
    #[inline(always)]
    pub fn get_mut(&mut self, _: MainThreadToken) -> &mut T {
        &mut self.0
    }

    /// Create a new `MainThreadData`. Panic if not called on the main thread.
    ///
    /// See also [`Self::new()`]
    #[track_caller]
    #[inline(always)]
    pub fn assert_new(data: T) -> Self {
        Self::new(MainThreadToken::assert(), data)
    }

    /// Get shared access to this data. Panic if not called on the main thread.
    ///
    /// See also [`Self::get()`], [`Self::get_on_main_thread()`]
    #[track_caller]
    #[inline(always)]
    pub fn assert_get(&self) -> &T {
        self.get(MainThreadToken::assert())
    }

    /// Get shared access to this data. Panic if not called on the main thread.
    ///
    /// See also [`Self::get_mut()`], [`Self::get_mut_on_main_thread()`]
    #[track_caller]
    #[inline(always)]
    pub fn assert_get_mut(&mut self) -> &mut T {
        self.get_mut(MainThreadToken::assert())
    }

    /// Get shared access to this data in a callback that's run on the main thread.
    /// This method waits for the callback to complete before returning.
    ///
    /// If this is called on a thread other than the main thread, it requires the SDL
    /// event loop to run. See [`SDL_RunOnMainThread`] for details.
    ///
    /// Returns false if the callback failed to run.
    ///
    /// See also [`run_sync_on_main_thread()`], [`Self::get()`], [`Self::assert_get()`]
    #[must_use]
    #[inline(always)]
    pub fn get_on_main_thread(&self, callback: impl FnOnce(&T) + Send) -> bool {
        run_sync_on_main_thread(move || callback(&self.0))
    }

    /// Get exclusive access to this data in a callback that's run on the main thread.
    /// This method waits for the callback to complete before returning.
    ///
    /// If this is called on a thread other than the main thread, it requires the SDL
    /// event loop to run. See [`SDL_RunOnMainThread`] for details.
    ///
    /// Returns false if the callback failed to run.
    ///
    /// See also [`run_sync_on_main_thread()`], [`Self::get_mut()`], [`Self::assert_get_mut()`]
    #[must_use]
    #[inline(always)]
    pub fn get_mut_on_main_thread(&mut self, callback: impl FnOnce(&mut T) + Send) -> bool {
        run_sync_on_main_thread(move || callback(&mut self.0))
    }
}

struct CallOnceContainer<F>(Option<F>);

trait CallOnce {
    fn call_once(&mut self);
    fn discard(&mut self);
}

impl<F: FnOnce()> CallOnce for CallOnceContainer<F> {
    fn call_once(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }

    fn discard(&mut self) {
        self.0.take();
    }
}

#[repr(transparent)]
struct MainThreadCallHeader(*mut dyn CallOnce);

#[repr(C)]
struct MainThreadCall<T> {
    header: MainThreadCallHeader,
    data: T,
}

/// Run a callback on the main thread and wait for it to complete before returning.
///
/// If this is called on a thread other than the main thread, it requires the SDL
/// event loop to run. See [`SDL_RunOnMainThread`] for details.
///
/// Returns false if the callback failed to run.
///
/// See also:
/// - [`run_async_on_main_thread()`]
/// - [`MainThreadData::get_on_main_thread()`]
#[must_use]
pub fn run_sync_on_main_thread<F: FnOnce() + Send>(callback: F) -> bool {
    unsafe extern "C" fn main_thread_fn(userdata: *mut c_void) {
        let call_once = unsafe { &mut *(userdata as *mut &mut dyn CallOnce) };
        call_once.call_once();
    }
    let mut f = CallOnceContainer(Some(callback));
    let mut f: &mut dyn CallOnce = &mut f;
    let f = &mut f as *mut &mut dyn CallOnce as *mut c_void;
    unsafe { SDL_RunOnMainThread(Some(main_thread_fn), f, true) }
}

/// Schedule a callback to run on the main thread and immediately return without waiting for it.
///
/// If this is called on a thread other than the main thread, it requires the SDL
/// event loop to run. See [`SDL_RunOnMainThread`] for details.
///
/// Returns false if the callback failed to run.
///
/// See also [`run_sync_on_main_thread()`].
#[must_use]
pub fn run_async_on_main_thread<F: FnOnce() + Send + 'static>(callback: F) -> bool {
    // we can't copy uninit bytes such as padding, because that's unsound,
    // and we can't know there's no uninit bytes unless the size is zero
    if const { size_of::<F>() == 0 } {
        // callback is zero sized; we don't need to allocate
        unsafe extern "C" fn main_thread_fn<F: FnOnce() + Send + 'static>(userdata: *mut c_void) {
            unsafe { (userdata as *mut F).read()() }
        }
        let callback = ManuallyDrop::new(callback);
        let userdata: *mut F = NonNull::<F>::dangling().as_ptr();
        if unsafe { SDL_RunOnMainThread(Some(main_thread_fn::<F>), userdata as *mut c_void, false) }
        {
            true
        } else {
            let _ = ManuallyDrop::into_inner(callback);
            false
        }
    } else {
        // have to allocate for callback
        unsafe extern "C" fn main_thread_fn(userdata: *mut c_void) {
            defer!(unsafe { SDL_aligned_free(userdata) });
            unsafe { &mut *((*(userdata as *mut MainThreadCallHeader)).0) }.call_once();
        }
        let f = CallOnceContainer(Some(callback));
        let userdata = unsafe {
            SDL_aligned_alloc(
                align_of::<MainThreadCall<CallOnceContainer<F>>>(),
                size_of::<MainThreadCall<CallOnceContainer<F>>>(),
            )
        } as *mut MainThreadCall<CallOnceContainer<F>>;
        if userdata.is_null() {
            return false;
        }
        let payload = unsafe { addr_of_mut!((*userdata).data) };
        unsafe {
            addr_of_mut!((*userdata).header.0).write(payload as *mut dyn CallOnce);
            (payload as *mut ManuallyDrop<CallOnceContainer<F>>).write(ManuallyDrop::new(f));
        }
        let userdata = userdata as *mut c_void;
        if unsafe { SDL_RunOnMainThread(Some(main_thread_fn), userdata, false) } {
            true
        } else {
            defer!(unsafe { SDL_aligned_free(userdata) });
            unsafe { &mut *((*(userdata as *mut MainThreadCallHeader)).0) }.discard();
            false
        }
    }
}
