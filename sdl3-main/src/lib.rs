#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = include_str!("../README.md")]
#![cfg_attr(all(feature = "nightly", doc), feature(doc_auto_cfg))] // https://github.com/rust-lang/rust/issues/43781

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(doc)]
use state::SyncPtr;

/// Use this attribute macro if you want to provide your own main, but run it through SDL.
///
/// Supported signatures: (all of these can be safe or unsafe)
/// ```custom,{.rust}
/// fn()
/// fn() -> c_int
/// fn() -> bool
/// fn(argc: c_int, argv: *mut *mut c_char)
/// fn(argc: c_int, argv: *mut *mut c_char) -> c_int
/// fn(argc: c_int, argv: *mut *mut c_char) -> bool
/// fn() -> impl Termination
/// ```
///
/// Example:
/// ```rust
/// #[sdl3_main::main]
/// fn my_main() {
///     println!("main called through SDL!");
/// }
/// ```
pub use sdl3_main_macros::main;

/// This attribute macro can be applied to an `impl` block for a type to assign the associated
/// functions or methods named `app_init`, `app_iterate`, `app_event` and `app_quit` to the
/// respective sdl main callbacks, as if the corresponding attribute macros were used.
/// All four must be defined in a single impl block, but `app_quit` is optional and will be
/// defined as an empty function if omitted.
pub use sdl3_main_macros::app_impl;

/// The function tagged with `app_init` is called by SDL at the start of the program on the main thread.
///
/// See the SDL documentation for this function:
/// [`SDL_AppInit`](https://docs.rs/sdl3-sys/latest/sdl3_sys/main/fn.SDL_AppInit.html)
///
/// The function tagged with this attribute determines the type of the app state, if any.
/// You can use a raw signature that doesn't handle the app state for you (safe or unsafe):
/// ```custom,{.rust}
/// fn(appstate: *mut *mut c_void, argc: c_int, argv: *mut *mut c_char) -> SDL_AppResult
/// fn(appstate: *mut *mut c_void, argc: c_int, argv: *mut *mut c_char) -> sdl3_main::AppResult
/// fn(argc: c_int, argv: *mut *mut c_char) -> SDL_AppResult
/// fn(argc: c_int, argv: *mut *mut c_char) -> sdl3_main::AppResult
/// fn(appstate: *mut *mut c_void) -> SDL_AppResult
/// fn(appstate: *mut *mut c_void) -> sdl3_main::AppResult
/// fn() -> SDL_AppResult
/// fn() -> sdl3_main::AppResult
/// ```
///
/// Or you can use one of these signatures with an app state type `S` (safe or unsafe):
/// ```custom,{.rust}
/// fn() -> Option<S>
/// fn() -> sdl3_main::AppResultWithState<S>
/// ```
///
/// The app state type must implement `Send` and `Sync`. You can define your own app state type
/// by implementing the `sdl3_main::AppState` trait, or use one of these predefined types:
/// ```custom,{.rust}
/// ()
/// sdl3_main::state::SyncPtr<_>
/// Box<_>
/// Arc<_>
/// ```
///
/// App states can be borrowed as many different types, depending on the app state. Everything
/// can be borrowed as `()` or `*mut c_void`.
///
/// | AppState | Can be borrowed as |
/// | -------- | ------------------ |
/// | [`SyncPtr<T>`] | <ul><li>`SyncPtr<T>`</li><li>`NonNull<T>` (may panic)</li><li>`Option<NonNull<T>>`</li></ul> |
/// | `Box<T>` | <ul><li>`&T`</li><li>`NonNull<T>`</li></ul> |
/// | `Box<Mutex<T>>` | <ul><li>`&T`</li><li>`&mut T`</li></ul> |
/// | `Box<RwLock<T>>` | <ul><li>`&T`</li><li>`&mut T`</li></ul> |
/// | `Arc<T>` | <ul><li>`Arc<T>`</li><li>`&Arc<T>`</li><li>`&T`</li><li>`NonNull<T>`</li></ul> |
/// | `Arc<Mutex<T>>` | <ul><li>`&T`</li><li>`&mut T`</li></ul> |
/// | `Arc<RwLock<T>>` | <ul><li>`&T`</li><li>`&mut T`</li></ul> |
pub use sdl3_main_macros::app_init;

/// The function tagged with `app_iterate` is called continuously by SDL on the main thread while the app is running.
///
/// It will only be called if `app_init` returned continue status, and keep getting called
/// for as long as `app_iterate` and `app_event` return continue status.
///
/// See the SDL documentation for this function:
/// [`SDL_AppIterate`](https://docs.rs/sdl3-sys/latest/sdl3_sys/main/fn.SDL_AppIterate.html)
///
/// Supported signatures (safe or unsafe), where `B` is a borrowed app state as defined in [`app_init`]:
/// ```custom,{.rust}
/// fn() -> impl sdl3_main::IntoAppResult
/// fn(B) -> impl sdl3_main::IntoAppResult
/// ```
pub use sdl3_main_macros::app_iterate;

/// The function tagged with `app_event` is called by SDL when an event is delivered. This may get called on the main thread
/// or on another thread.
///
/// See the SDL documentation for this function:
/// [`SDL_AppEvent`](https://docs.rs/sdl3-sys/latest/sdl3_sys/main/fn.SDL_AppEvent.html)
///
/// Supported signatures (safe or unsafe), where `B` is a borrowed app state as defined in [`app_init`],
/// and `E` is any supported event type:
/// ```custom,{.rust}
/// fn() -> impl sdl3_main::IntoAppResult
/// fn(E) -> impl sdl3_main::IntoAppResult
/// fn(B, E) -> impl sdl3_main::IntoAppResult
/// ```
///
/// Predefined supported event types:
/// ```custom,{.rust}
/// SDL_Event
/// &SDL_Event
/// &mut SDL_Event
/// *const SDL_Event
/// *mut SDL_Event
/// ```
/// You can add support for your own event types by implementing the `PassEventVal`, `PassEventRef` and/or `PassEventMut` traits.
pub use sdl3_main_macros::app_event;

/// The function tagged with `app_quit` is called by SDL on the main thread when the app quits.
///
/// This will get called regardless of the return status of `app_init`, so if that fails the
/// app state may not be available. If you're using an app state type you should take it as
/// an `Option` to avoid a panic in that case.
///
/// See the SDL documentation for this function:
/// [`SDL_AppQuit`](https://docs.rs/sdl3-sys/latest/sdl3_sys/main/fn.SDL_AppQuit.html)
///
/// Supported signatures (safe or unsafe), where `B` is a borrowed app state, and `S` is the app state
/// itself as defined in [`app_init`]:
/// ```custom,{.rust}
/// fn()
/// fn(Option<B>)
/// fn(Option<B>, SDL_AppResult)
/// fn(Option<B>, sdl3_main::AppResult)
/// fn(Option<S>)
/// fn(Option<S>, SDL_AppResult)
/// fn(Option<S>, sdl3_main::AppResult)
/// fn(B)
/// fn(B, SDL_AppResult)
/// fn(B, sdl3_main::AppResult)
/// fn(S)
/// fn(S, SDL_AppResult)
/// fn(S, sdl3_main::AppResult)
/// ```
pub use sdl3_main_macros::app_quit;

use core::{
    ffi::{c_int, c_void},
    ptr,
};
use sdl3_sys::{
    init::SDL_AppResult,
    log::{SDL_LogCategory, SDL_LogCritical},
};

pub mod app;
mod main_thread;
pub mod state;

pub use main_thread::{MainThreadData, MainThreadToken};
use state::{AppState, BorrowMut, BorrowRef, BorrowVal, ConsumeMut, ConsumeRef, ConsumeVal};

#[doc(hidden)]
pub mod __internal {
    pub use ::sdl3_sys;

    #[cfg(feature = "alloc")]
    pub use ::alloc::{boxed::Box, sync::Arc};

    use core::{
        ffi::{c_char, c_int},
        ptr,
    };
    use sdl3_sys::main::SDL_RunApp;
    #[cfg(feature = "std")]
    use std::{
        any::Any,
        cell::UnsafeCell,
        mem::MaybeUninit,
        panic::{catch_unwind, resume_unwind, UnwindSafe},
    };

    #[cfg(feature = "std")]
    pub struct Shuttle<T>(UnsafeCell<MaybeUninit<Result<T, Box<dyn Any + Send>>>>);

    #[cfg(feature = "std")]
    unsafe impl<T> Sync for Shuttle<T> {}

    #[cfg(feature = "std")]
    impl<T> Shuttle<T> {
        #[allow(clippy::new_without_default)]
        pub const fn new() -> Self {
            Self(UnsafeCell::new(MaybeUninit::uninit()))
        }

        /// # Safety
        /// - This is not thread safe!
        /// - Calling this after a previous capture without a corresponding resume will
        ///   leak/forget the previous capture
        pub unsafe fn capture(&self, f: impl FnOnce() -> T + UnwindSafe) {
            match catch_unwind(f) {
                Ok(value) => {
                    unsafe { self.0.get().write(MaybeUninit::new(Ok(value))) };
                }
                Err(unwind) => {
                    unsafe { self.0.get().write(MaybeUninit::new(Err(unwind))) };
                }
            }
        }

        /// # Safety
        /// - This is not thread safe!
        /// - It's UB to call this without having called `capture`
        /// - It's UB to call this more than once after each call to `capture`
        pub unsafe fn resume(&self) -> T {
            match unsafe { (*self.0.get()).assume_init_read() } {
                Ok(value) => value,
                Err(unwind) => resume_unwind(unwind),
            }
        }
    }

    #[cfg(feature = "std")]
    impl Shuttle<()> {
        /// # Safety
        /// - This is not thread safe!
        /// - It's UB to call this more than once before each call to `resume`
        pub unsafe fn capture_and_continue<T>(
            &self,
            unwind_val: T,
            f: impl FnOnce() -> T + UnwindSafe,
        ) -> T {
            match catch_unwind(f) {
                Ok(value) => {
                    unsafe { self.0.get().write(MaybeUninit::new(Ok(()))) };
                    value
                }
                Err(unwind) => {
                    unsafe { self.0.get().write(MaybeUninit::new(Err(unwind))) };
                    unwind_val
                }
            }
        }
    }

    #[inline(always)]
    pub unsafe fn run_app(
        main_fn: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    ) -> c_int {
        unsafe { SDL_RunApp(0, ptr::null_mut(), Some(main_fn), ptr::null_mut()) }
    }
}

/// This trait is used for converting a type into an [`SDL_AppResult`] or [`AppResult`].
///
/// `()` implements this trait and turns into [`SDL_AppResult::CONTINUE`]
pub trait IntoAppResult: Sized {
    fn into_sdl_app_result(self) -> SDL_AppResult;

    #[inline(always)]
    fn into_app_result(self) -> AppResult {
        self.into_sdl_app_result().into()
    }
}

impl IntoAppResult for () {
    #[inline(always)]
    fn into_sdl_app_result(self) -> SDL_AppResult {
        SDL_AppResult::CONTINUE
    }

    #[inline(always)]
    fn into_app_result(self) -> AppResult {
        AppResult::Continue
    }
}

impl IntoAppResult for SDL_AppResult {
    #[inline(always)]
    fn into_sdl_app_result(self) -> SDL_AppResult {
        self
    }

    #[inline(always)]
    fn into_app_result(self) -> AppResult {
        self.into()
    }
}

impl IntoAppResult for AppResult {
    #[inline(always)]
    fn into_sdl_app_result(self) -> SDL_AppResult {
        self.into()
    }

    #[inline(always)]
    fn into_app_result(self) -> AppResult {
        self
    }
}

/// This is the Rust enum equivalent to [`SDL_AppResult`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppResult {
    /// Continue running
    Continue,

    /// Quit with success status
    Success,

    /// Quit with failure status
    Failure,
}

impl From<AppResult> for SDL_AppResult {
    #[inline(always)]
    fn from(value: AppResult) -> Self {
        match value {
            AppResult::Continue => Self::CONTINUE,
            AppResult::Success => Self::SUCCESS,
            AppResult::Failure => Self::FAILURE,
        }
    }
}

impl From<SDL_AppResult> for AppResult {
    #[inline]
    fn from(value: SDL_AppResult) -> Self {
        match value {
            SDL_AppResult::CONTINUE => Self::Continue,
            SDL_AppResult::SUCCESS => Self::Success,
            SDL_AppResult::FAILURE => Self::Failure,
            SDL_AppResult(value) => {
                #[cold]
                #[inline(never)]
                fn unknown_app_result_value(result: c_int) -> AppResult {
                    unsafe {
                        SDL_LogCritical(
                            SDL_LogCategory::APPLICATION.into(),
                            c"Unrecognized app result value: %d".as_ptr(),
                            result,
                        )
                    };
                    AppResult::Failure
                }
                unknown_app_result_value(value)
            }
        }
    }
}

/// An [`AppResult`] with an app state, for returning from the function tagged with [`app_init`].
#[derive(Debug)]
pub enum AppResultWithState<S: AppState> {
    /// Continue running
    Continue(S),

    /// Quit with success status
    Success(Option<S>),

    /// Quit with failure status
    Failure(Option<S>),
}

impl<S: AppState> AppResultWithState<S> {
    pub fn into_raw(self) -> (SDL_AppResult, *mut c_void) {
        match self {
            Self::Continue(s) => (SDL_AppResult::CONTINUE, s.into_raw()),
            Self::Success(s) => (
                SDL_AppResult::SUCCESS,
                s.map(|s| s.into_raw()).unwrap_or(ptr::null_mut()),
            ),
            Self::Failure(s) => (
                SDL_AppResult::FAILURE,
                s.map(|s| s.into_raw()).unwrap_or(ptr::null_mut()),
            ),
        }
    }
}
