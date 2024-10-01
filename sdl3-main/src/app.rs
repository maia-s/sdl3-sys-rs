use crate::{
    AppResult, AppResultWithState, AppState, BorrowMut, BorrowRef, BorrowVal, ConsumeMut,
    ConsumeRef, ConsumeVal, IntoAppResult, MainThreadToken,
};
use core::ffi::{c_char, c_int, c_void};
use sdl3_sys::{events::SDL_Event, init::SDL_AppResult};
#[cfg(feature = "std")]
use std::process::Termination;

const AC_NONE: u8 = 0;
const AC_VAL: u8 = 1;
const AC_REF: u8 = 2;
const AC_MUT: u8 = 3;
const AC_OPT: u8 = 4;
const AC_OPT_VAL: u8 = AC_OPT | AC_VAL;
const AC_OPT_REF: u8 = AC_OPT | AC_REF;
const AC_OPT_MUT: u8 = AC_OPT | AC_MUT;

#[inline(always)]
unsafe fn consume_opt_val<S: AppState + ConsumeVal<T>, T>(
    raw: *mut c_void,
    f: impl FnOnce(Option<T>),
) {
    if !raw.is_null() {
        unsafe { <S as ConsumeVal<T>>::consume_val(raw, |b| f(Some(b))) }
    } else {
        f(None)
    }
}

#[inline(always)]
unsafe fn consume_opt_ref<S: AppState + ConsumeRef<T>, T>(
    raw: *mut c_void,
    f: impl FnOnce(Option<&T>),
) {
    if !raw.is_null() {
        unsafe { <S as ConsumeRef<T>>::consume_ref(raw, |b| f(Some(b))) }
    } else {
        f(None)
    }
}

#[inline(always)]
unsafe fn consume_opt_mut<S: AppState + ConsumeMut<T>, T>(
    raw: *mut c_void,
    f: impl FnOnce(Option<&mut T>),
) {
    if !raw.is_null() {
        unsafe { <S as ConsumeMut<T>>::consume_mut(raw, |b| f(Some(b))) }
    } else {
        f(None)
    }
}

/// Pass an event by value
pub trait PassEventVal: Sized {
    /// Pass an event by value
    fn pass_event_val<R>(event: &mut SDL_Event, f: impl FnOnce(Self) -> R) -> R;
}

/// Pass an event by reference
pub trait PassEventRef {
    /// Pass an event by reference
    fn pass_event_ref<R>(event: &mut SDL_Event, f: impl FnOnce(&Self) -> R) -> R;
}

/// Pass an event by mutable reference
pub trait PassEventMut {
    /// Pass an event by mutable reference
    fn pass_event_mut<R>(event: &mut SDL_Event, f: impl FnOnce(&mut Self) -> R) -> R;
}

impl PassEventVal for *const SDL_Event {
    #[inline(always)]
    fn pass_event_val<R>(event: &mut SDL_Event, f: impl FnOnce(Self) -> R) -> R {
        f(event)
    }
}

impl PassEventVal for *mut SDL_Event {
    #[inline(always)]
    fn pass_event_val<R>(event: &mut SDL_Event, f: impl FnOnce(Self) -> R) -> R {
        f(event)
    }
}

impl PassEventVal for SDL_Event {
    #[inline(always)]
    fn pass_event_val<R>(event: &mut SDL_Event, f: impl FnOnce(Self) -> R) -> R {
        f(*event)
    }
}

impl PassEventRef for SDL_Event {
    #[inline(always)]
    fn pass_event_ref<R>(event: &mut SDL_Event, f: impl FnOnce(&SDL_Event) -> R) -> R {
        f(event)
    }
}

impl PassEventMut for SDL_Event {
    #[inline(always)]
    fn pass_event_mut<R>(event: &mut SDL_Event, f: impl FnOnce(&mut SDL_Event) -> R) -> R {
        f(event)
    }
}

mod sealed {
    macro_rules! traits {
        ($($trait:ident $(($($gen:tt)*))?),* $(,)?) => {
            $( #[allow(non_snake_case)] pub(crate) mod $trait { pub trait Sealed $(<$($gen)*>)?: Sized {} } )*
        };
    }

    traits!(
        AppMain,
        AppMainWithResult,
        AppInit(S),
        AppIterate(S, const STATE_AC: u8),
        AppEvent(S, const STATE_AC: u8, const EVENT_AC: u8),
        AppQuit(S, const STATE_AC: u8),
    );
}

macro_rules! impl_app {
    (
        $(
            $(#[$attr:meta])* impl $(<
                $($generic:ident $(:$bound:tt $(<$boundgen:tt>)?)?),*
            >)? $trait:ident $(<$($traitarg:ident),*>)?
            for fn ($($args:tt)*) $(-> $rt:ty)? { $($block:tt)* }
        )*
    ) => {
        $(
            $(#[$attr])*
            impl $(<$($generic $(: $bound $(<$boundgen>)?)?),*>)?
                sealed::$trait::Sealed $(<$($traitarg),*>)?
                for fn($($args)*) $(-> $rt)? {}

            $(#[$attr])*
            #[allow(unused_unsafe)]
            impl $(<$($generic $(: $bound $(<$boundgen>)?)?),*>)?
                $trait $(<$($traitarg),*>)?
                for fn($($args)*) $(-> $rt)?
            {
                $($block)*
            }

            $(#[$attr])*
            impl $(<$($generic $(: $bound $(<$boundgen>)?)?),*>)?
                sealed::$trait::Sealed $(<$($traitarg),*>)?
                for unsafe fn($($args)*) $(-> $rt)? {}

            $(#[$attr])*
            #[allow(unused_unsafe)]
            impl $(<$($generic $(: $bound $(<$boundgen>)?)?),*>)?
                $trait $(<$($traitarg),*>)?
                for unsafe fn($($args)*) $(-> $rt)?
            {
                $($block)*
            }
        )*
    };
}

#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `main`: `{Self}`",
    label = "for the function attached to this `main` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppMain: sealed::AppMain::Sealed {
    #[doc(hidden)] // not currently public
    /// # Safety
    /// `argv` must be null or point to a valid array of length `argc` containing valid c strings
    unsafe fn main(self, _: MainThreadToken, argc: c_int, argv: *mut *mut c_char) -> c_int;
}

impl_app! {
    impl AppMain for fn(c_int, *mut *mut c_char) -> c_int {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, argc: c_int, argv: *mut *mut c_char) -> c_int {
            unsafe { self(argc, argv) }
        }
    }

    impl AppMain for fn(c_int, *mut *mut c_char) -> bool {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, argc: c_int, argv: *mut *mut c_char) -> c_int {
            1 - (unsafe { self(argc, argv) } as c_int)
        }
    }

    impl AppMain for fn(c_int, *mut *mut c_char) {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, argc: c_int, argv: *mut *mut c_char) -> c_int {
            unsafe { self(argc, argv) };
            0
        }
    }

    impl AppMain for fn() -> c_int {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, _: c_int, _: *mut *mut c_char) -> c_int {
            unsafe { self() }
        }
    }

    impl AppMain for fn() -> bool {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, _: c_int, _: *mut *mut c_char) -> c_int {
            1 - (unsafe { self() } as c_int)
        }
    }

    impl AppMain for fn() {
        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, _: c_int, _: *mut *mut c_char) -> c_int {
            unsafe { self() };
            0
        }
    }
}

#[cfg(feature = "std")]
#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `main`: `{Self}`",
    label = "for the function attached to this `main` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppMainWithResult: sealed::AppMainWithResult::Sealed {
    #[doc(hidden)] // not currently public
    /// The result of the main function
    type Result: Termination;

    #[doc(hidden)] // not currently public
    /// # Safety
    /// `argv` must be null or point to a valid array of length `argc` containing valid c strings
    unsafe fn main(self, _: MainThreadToken, argc: c_int, argv: *mut *mut c_char) -> Self::Result;
}

impl_app! {
    #[cfg(feature = "std")]
    impl<R: Termination> AppMainWithResult for fn() -> R {
        type Result = R;

        #[inline(always)]
        unsafe fn main(self, _: MainThreadToken, _: c_int, _: *mut *mut c_char) -> Self::Result {
            unsafe { self() }
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `app_init`: `{Self}`",
    label = "for the function attached to this `app_init` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppInit<S>: sealed::AppInit::Sealed<S> {
    #[doc(hidden)] // not currently public
    /// # Safety
    /// - See the documentation for [`sdl3_sys::main::SDL_AppInit`]
    /// - `appstate` must point to memory suitable for writing a pointer to `S`
    /// - The implementation must accept `argc == 0` and `argv == null`
    unsafe fn init(
        self,
        _: MainThreadToken,
        appstate: *mut *mut c_void,
        argc: c_int,
        argv: *mut *mut c_char,
    ) -> SDL_AppResult;
}

impl_app! {
    impl<S: AppState> AppInit<S> for fn(*mut *mut c_void, c_int, *mut *mut c_char) -> SDL_AppResult
    {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(appstate, argc, argv) }
        }
    }

    impl<S: AppState> AppInit<S> for fn(*mut *mut c_void, c_int, *mut *mut c_char) -> AppResult
    {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(appstate, argc, argv) }.into()
        }
    }

    impl<S: AppState> AppInit<S> for fn(*mut *mut c_void) -> SDL_AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(appstate) }
        }
    }

    impl<S: AppState> AppInit<S> for fn(*mut *mut c_void) -> AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(appstate) }.into()
        }
    }

    impl<S: AppState> AppInit<S> for fn(c_int, *mut *mut c_char) -> SDL_AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            _appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(argc, argv) }
        }
    }

    impl<S: AppState> AppInit<S> for fn(c_int, *mut *mut c_char) -> AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            _appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self(argc, argv) }.into()
        }
    }

    impl<S: AppState> AppInit<S> for fn() -> SDL_AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            _appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self() }
        }
    }

    impl<S: AppState> AppInit<S> for fn() -> AppResult {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            _appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            unsafe { self() }.into()
        }
    }

    impl<S: AppState> AppInit<S> for fn() -> Option<S> {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            match unsafe { self() } {
                Some(s) => {
                    unsafe { appstate.write(s.into_raw()) };
                    SDL_AppResult::CONTINUE
                }
                None => SDL_AppResult::FAILURE,
            }
        }
    }

    impl<S: AppState> AppInit<S> for fn() -> AppResultWithState<S> {
        #[inline(always)]
        unsafe fn init(
            self,
            _: MainThreadToken,
            appstate: *mut *mut c_void,
            _argc: c_int,
            _argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            let (r, s) = unsafe { self() }.into_raw();
            unsafe { appstate.write(s) };
            r
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `app_iterate`: `{Self}`",
    label = "for the function attached to this `app_iterate` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppIterate<S, const STATE_AC: u8>: sealed::AppIterate::Sealed<S, STATE_AC> {
    #[doc(hidden)] // not currently public
    /// # Safety
    /// - See the documentation for [`sdl3_sys::main::SDL_AppIterate`]
    /// - `appstate` must point to a valid `S`
    unsafe fn iterate(self, appstate: *mut c_void) -> SDL_AppResult;
}

impl_app! {
    impl<S: BorrowVal<()>, R: IntoAppResult> AppIterate<S, AC_NONE> for fn() -> R {
        #[inline(always)]
        unsafe fn iterate(self, _: *mut c_void) -> SDL_AppResult {
            unsafe { self() }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<T>, T, R: IntoAppResult> AppIterate<S, AC_VAL> for fn(T) -> R {
        #[inline(always)]
        unsafe fn iterate(self, appstate: *mut c_void) -> SDL_AppResult {
            unsafe { S::borrow_val(appstate, |b| unsafe { self(b) }) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowRef<T>, T, R: IntoAppResult> AppIterate<S, AC_REF> for fn(&T) -> R {
        #[inline(always)]
        unsafe fn iterate(self, appstate: *mut c_void) -> SDL_AppResult {
            unsafe { S::borrow_ref(appstate, |b| unsafe { self(b) }) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowMut<T>, T, R: IntoAppResult> AppIterate<S, AC_MUT> for fn(&mut T) -> R {
        #[inline(always)]
        unsafe fn iterate(self, appstate: *mut c_void) -> SDL_AppResult {
            unsafe { S::borrow_mut(appstate, |b| unsafe { self(b) }) }.into_sdl_app_result()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `app_event`: `{Self}`",
    label = "for the function attached to this `app_event` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppEvent<S, const STATE_AC: u8, const EVENT_AC: u8>:
    sealed::AppEvent::Sealed<S, STATE_AC, EVENT_AC>
{
    #[doc(hidden)] // not currently public
    /// # Safety
    /// - See the documentation for [`sdl3_sys::main::SDL_AppEvent`]
    /// - `appstate` must point to a valid `S`
    /// - `event` must point to a valid [`SDL_Event`]
    unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult;
}

impl_app! {
    impl<S: AppState> AppEvent<S, AC_NONE, AC_NONE> for fn() {
        #[inline(always)]
        unsafe fn event(self, _: *mut c_void, _: *mut SDL_Event) -> SDL_AppResult {
            unsafe { self() }
            SDL_AppResult::CONTINUE
        }
    }

    impl<S: BorrowVal<()>, E: PassEventVal, R: IntoAppResult> AppEvent<S, AC_NONE, AC_VAL> for fn(E) -> R {
        #[inline(always)]
        unsafe fn event(self, _: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { E::pass_event_val(&mut *event, |e| unsafe { self(e) }) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<()>, E: PassEventRef, R: IntoAppResult> AppEvent<S, AC_NONE, AC_REF> for fn(&E) -> R {
        #[inline(always)]
        unsafe fn event(self, _: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { E::pass_event_ref(&mut *event, |e| unsafe { self(e) }) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<()>, E: PassEventMut, R: IntoAppResult> AppEvent<S, AC_NONE, AC_MUT> for fn(&mut E) -> R {
        #[inline(always)]
        unsafe fn event(self, _: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { E::pass_event_mut(&mut *event, |e| unsafe { self(e) }) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<T>, T, E: PassEventVal, R: IntoAppResult> AppEvent<S, AC_VAL, AC_VAL> for fn(T, E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_val(appstate, |s| E::pass_event_val(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<T>, T, E: PassEventRef, R: IntoAppResult> AppEvent<S, AC_VAL, AC_REF> for fn(T, &E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_val(appstate, |s| E::pass_event_ref(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowVal<T>, T, E: PassEventMut, R: IntoAppResult> AppEvent<S, AC_VAL, AC_MUT> for fn(T, &mut E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_val(appstate, |s| E::pass_event_mut(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowRef<T>, T, E: PassEventVal, R: IntoAppResult> AppEvent<S, AC_REF, AC_VAL> for fn(&T, E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_ref(appstate, |s| E::pass_event_val(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowRef<T>, T, E: PassEventRef, R: IntoAppResult> AppEvent<S, AC_REF, AC_REF> for fn(&T, &E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_ref(appstate, |s| E::pass_event_ref(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowRef<T>, T, E: PassEventMut, R: IntoAppResult> AppEvent<S, AC_REF, AC_MUT> for fn(&T, &mut E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_ref(appstate, |s| E::pass_event_mut(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowMut<T>, T, E: PassEventVal, R: IntoAppResult> AppEvent<S, AC_MUT, AC_VAL> for fn(&mut T, E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_mut(appstate, |s| E::pass_event_val(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowMut<T>, T, E: PassEventRef, R: IntoAppResult> AppEvent<S, AC_MUT, AC_REF> for fn(&mut T, &E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_mut(appstate, |s| E::pass_event_ref(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }

    impl<S: BorrowMut<T>, T, E: PassEventMut, R: IntoAppResult> AppEvent<S, AC_MUT, AC_MUT> for fn(&mut T, &mut E) -> R
    {
        #[inline(always)]
        unsafe fn event(self, appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
            unsafe { S::borrow_mut(appstate, |s| E::pass_event_mut(&mut *event, |e| unsafe { self(s, e) })) }.into_sdl_app_result()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "unsupported function signature for `app_quit`: `{Self}`",
    label = "for the function attached to this `app_quit` attribute",
    note = "see the `sdl3-main` crate documentation for supported signatures"
)]
pub trait AppQuit<S, const STATE_AC: u8>: sealed::AppQuit::Sealed<S, STATE_AC> {
    #[doc(hidden)] // not currently public
    /// # Safety
    /// - See the documentation for [`sdl3_sys::main::SDL_AppQuit`]
    /// - `appstate` must either be null, or point to a valid `S`
    unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult);
}

impl_app! {
    impl<S: AppState> AppQuit<S, AC_NONE> for fn() {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                self();
                let _ = S::from_raw(appstate);
            };
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_VAL> for fn(T) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                S::consume_val(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_REF> for fn(&T) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                S::consume_ref(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_MUT> for fn(&mut T) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                S::consume_mut(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_VAL> for fn(T, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_val(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_REF> for fn(&T, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_ref(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_MUT> for fn(&mut T, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_mut(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_VAL> for fn(T, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_val(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_REF> for fn(&T, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_ref(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_MUT> for fn(&mut T, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                S::consume_mut(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_OPT_VAL> for fn(Option<T>) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                consume_opt_val::<S, T>(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_OPT_REF> for fn(Option<&T>) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                consume_opt_ref::<S, T>(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_OPT_MUT> for fn(Option<&mut T>) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, _result: SDL_AppResult) {
            unsafe {
                consume_opt_mut::<S, T>(appstate, |s| unsafe { self(s) });
            }
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_OPT_VAL> for fn(Option<T>, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_val::<S, T>(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_OPT_REF> for fn(Option<&T>, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_ref::<S, T>(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_OPT_MUT> for fn(Option<&mut T>, SDL_AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_mut::<S, T>(appstate, |s| unsafe { self(s, result) });
            }
        }
    }

    impl<S: ConsumeVal<T>, T> AppQuit<S, AC_OPT_VAL> for fn(Option<T>, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_val::<S, T>(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }

    impl<S: ConsumeRef<T>, T> AppQuit<S, AC_OPT_REF> for fn(Option<&T>, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_ref::<S, T>(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }

    impl<S: ConsumeMut<T>, T> AppQuit<S, AC_OPT_MUT> for fn(Option<&mut T>, AppResult) {
        #[inline(always)]
        unsafe fn quit(self, appstate: *mut c_void, result: SDL_AppResult) {
            unsafe {
                consume_opt_mut::<S, T>(appstate, |s| unsafe { self(s, result.into()) });
            }
        }
    }
}
