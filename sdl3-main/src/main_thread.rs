use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};
use sdl3_sys::thread::{SDL_GetCurrentThreadID, SDL_ThreadID};

#[cfg(doc)]
use crate::{app_init, main};

/// Zero sized token that can only exist on the main thread.
///
/// Call [`MainThreadToken::get()`] or [`MainThreadToken::assert()`] to get one.
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
