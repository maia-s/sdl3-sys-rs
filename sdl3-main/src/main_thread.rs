use core::{
    cell::UnsafeCell,
    ffi::c_void,
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::addr_of_mut,
    sync::atomic::{AtomicBool, Ordering},
};
use sdl3_sys::{
    init::SDL_RunOnMainThread,
    stdinc::{SDL_aligned_alloc, SDL_aligned_free},
    thread::{SDL_GetCurrentThreadID, SDL_ThreadID},
};

#[cfg(doc)]
use crate::{app_init, main};

/// Zero sized token that can only exist on the main thread.
///
/// Call [`MainThreadToken::get()`] or [`MainThreadToken::assert()`] to get one.
#[derive(Clone, Copy)]
pub struct MainThreadToken(PhantomData<*const ()>);

static MAIN_THREAD_ID: MainOnceLock<SDL_ThreadID> = MainOnceLock::new();

impl MainThreadToken {
    /// Get `Some(MainThreadToken)` if called on the main thread, or `None` otherwise.
    /// Returns `None` if `MainThreadToken` hasn't been inited.
    ///
    /// On targets that don't support threads, this will always succeed if `MainThreadToken`
    /// has been inited.
    ///
    /// See also [`MainThreadToken::assert()`]
    pub fn get() -> Option<Self> {
        MAIN_THREAD_ID.get().and_then(|tid| {
            (*tid == unsafe { SDL_GetCurrentThreadID() }).then_some(Self(PhantomData))
        })
    }

    /// Get `MainThreadToken` if called on the main thread, or panic otherwise.
    /// Panics if `MainThreadToken` hasn't been inited.
    ///
    /// On targets that don't support threads, this will always succeed if `MainThreadToken`
    /// has been inited.
    ///
    /// See also [`MainThreadToken::get()`]
    pub fn assert() -> Self {
        Self::get().expect("This operation can only be performed on the main thread")
    }

    /// Init and declare the current thread as the main thread.
    ///
    /// This doesn't change what's the actual main thread, and it's UB to call this on
    /// any thread other than the main thread. [`MainThreadToken::get()`] and
    /// [`MainThreadToken::assert()`] will fail on any thread including the main thread
    /// before this function is called.
    ///
    /// The [`main`] and [`app_init`] macros will call this for you.
    ///
    /// # Safety
    /// It's undefined behaviour to call this on any thread other than the main thread.
    /// On targets that don't support threads, this is always safe.
    pub unsafe fn init() {
        unsafe { MAIN_THREAD_ID.set_once(SDL_GetCurrentThreadID()) };
    }
}

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
    #[inline(always)]
    pub fn new(_: MainThreadToken, data: T) -> Self {
        Self(data)
    }

    #[inline(always)]
    pub fn get(&self, _: MainThreadToken) -> &T {
        &self.0
    }

    #[inline(always)]
    pub fn get_mut(&mut self, _: MainThreadToken) -> &mut T {
        &mut self.0
    }

    /// Get shared access to this data in a callback that's run on the main thread.
    /// This method waits for the callback to complete before returning.
    ///
    /// If this is called on a thread other than the main thread, it requires the SDL
    /// event loop to run. See [`SDL_RunOnMainThread`] for details.
    ///
    /// Returns false if the callback failed to run.
    ///
    /// See also [`run_sync_on_main_thread()`].
    #[must_use]
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
    /// See also [`run_sync_on_main_thread()`].
    #[must_use]
    pub fn get_mut_on_main_thread(&mut self, callback: impl FnOnce(&mut T) + Send) -> bool {
        run_sync_on_main_thread(move || callback(&mut self.0))
    }
}

struct CallOnceContainer<F>(Option<F>);

trait CallOnce {
    fn call_once(&mut self);
}

impl<F: FnOnce()> CallOnce for CallOnceContainer<F> {
    fn call_once(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
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
    unsafe extern "C" fn main_thread_fn(userdata: *mut c_void) {
        let call_once = unsafe { &mut *((*(userdata as *mut MainThreadCallHeader)).0) };
        call_once.call_once();
        unsafe { SDL_aligned_free(userdata) };
    }
    let f = CallOnceContainer(Some(callback));
    let data_ptr = unsafe {
        SDL_aligned_alloc(
            align_of::<MainThreadCall<CallOnceContainer<F>>>(),
            size_of::<MainThreadCall<CallOnceContainer<F>>>(),
        )
    } as *mut MainThreadCall<CallOnceContainer<F>>;
    if data_ptr.is_null() {
        return false;
    }
    let data = unsafe { addr_of_mut!((*data_ptr).data) };
    unsafe {
        addr_of_mut!((*data_ptr).header.0).write(data as *mut dyn CallOnce);
        addr_of_mut!((*data_ptr).data).write(f);
    }
    unsafe { SDL_RunOnMainThread(Some(main_thread_fn), data as *mut c_void, false) }
}

#[repr(transparent)]
struct SyncUnsafeCell<T: ?Sized>(UnsafeCell<T>);

unsafe impl<T: ?Sized + Sync> Sync for SyncUnsafeCell<T> {}

// OnceLock-ish struct that works with no_std
// (Copy for !Drop)
struct MainOnceLock<T: Copy> {
    is_set: AtomicBool,
    data: SyncUnsafeCell<MaybeUninit<T>>,
}

impl<T: Copy> MainOnceLock<T> {
    const fn new() -> Self {
        Self {
            is_set: AtomicBool::new(false),
            data: SyncUnsafeCell(UnsafeCell::new(MaybeUninit::uninit())),
        }
    }

    fn get(&self) -> Option<&T> {
        self.is_set
            .load(Ordering::Acquire)
            .then(|| unsafe { (*self.data.0.get()).assume_init_ref() })
    }

    /// # Safety
    /// This does *not* protect against a data race if called concurrently from multiple threads
    unsafe fn set_once(&self, data: T) {
        if !self.is_set.load(Ordering::Relaxed) {
            unsafe { self.data.0.get().write(MaybeUninit::new(data)) };
            self.is_set.store(true, Ordering::Release);
        }
    }
}
