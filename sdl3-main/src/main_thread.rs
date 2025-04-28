use core::marker::PhantomData;
use sdl3_sys::init::SDL_IsMainThread;

#[cfg(doc)]
use crate::{app_init, main};

/// Zero sized token that can only exist on the main thread.
///
/// Call [`MainThreadToken::get()`] or [`MainThreadToken::assert()`] to get one.
#[derive(Clone, Copy)]
pub struct MainThreadToken(PhantomData<*const ()>);

impl MainThreadToken {
    /// Get `Some(MainThreadToken)` if called on the main thread, or `None` otherwise.
    ///
    /// On targets that don't support threads, this will always succeed.
    ///
    /// See also [`MainThreadToken::assert()`]
    pub fn get() -> Option<Self> {
        unsafe { SDL_IsMainThread() }.then_some(Self(PhantomData))
    }

    /// Get `MainThreadToken` if called on the main thread, or panic otherwise.
    ///
    /// On targets that don't support threads, this will always succeed.
    ///
    /// See also [`MainThreadToken::get()`]
    pub fn assert() -> Self {
        Self::get().expect("This operation can only be performed on the main thread")
    }
}

/// Data that can only be accessed from the main thread. Accessors take a [`MainThreadToken`].
///
/// This can be moved freely between threads, but the Drop implementation will panic if it's
/// dropped from another thread than the main thread.
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
    /// Create a new MainThreadData.
    #[inline(always)]
    pub fn new(_: MainThreadToken, data: T) -> Self {
        Self(data)
    }

    /// Get shared access to this data.
    #[inline(always)]
    pub fn get(&self, _: MainThreadToken) -> &T {
        &self.0
    }

    /// Get exclusive access to this data.
    #[inline(always)]
    pub fn get_mut(&mut self, _: MainThreadToken) -> &mut T {
        &mut self.0
    }
}
