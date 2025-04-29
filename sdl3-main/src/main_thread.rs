use core::{ffi::c_void, marker::PhantomData, ptr::addr_of_mut};
use sdl3_sys::{
    init::{SDL_IsMainThread, SDL_RunOnMainThread},
    stdinc::{SDL_aligned_alloc, SDL_aligned_free},
};

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

    /// Get shared access to this data in a callback that's run on the main thread.
    /// This method waits for the callback to complete before returning.
    ///
    /// If this is called on a thread other than the main thread, it requires the SDL
    /// event loop to run. See [`SDL_RunOnMainThread`] for details.
    ///
    /// Returns false if the callback failed to run.
    ///
    /// See also [`run_on_main_thread_sync()`].
    #[must_use]
    pub fn get_on_main_thread(&self, callback: impl FnOnce(&T) + Send) -> bool {
        run_on_main_thread_sync(move || callback(&self.0))
    }

    /// Get exclusive access to this data in a callback that's run on the main thread.
    /// This method waits for the callback to complete before returning.
    ///
    /// If this is called on a thread other than the main thread, it requires the SDL
    /// event loop to run. See [`SDL_RunOnMainThread`] for details.
    ///
    /// Returns false if the callback failed to run.
    ///
    /// See also [`run_on_main_thread_sync()`].
    #[must_use]
    pub fn get_mut_on_main_thread(&mut self, callback: impl FnOnce(&mut T) + Send) -> bool {
        run_on_main_thread_sync(move || callback(&mut self.0))
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
/// - [`run_on_main_thread_async()`]
/// - [`MainThreadData::get_on_main_thread()`]
#[must_use]
pub fn run_on_main_thread_sync<F: FnOnce() + Send>(callback: F) -> bool {
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
/// See also [`run_on_main_thread_sync()`].
#[must_use]
pub fn run_on_main_thread_async<F: FnOnce() + Send + 'static>(callback: F) -> bool {
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
