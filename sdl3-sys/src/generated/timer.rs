#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryTimer
//!
//! SDL time management routines.

use super::stdinc::*;

use super::error::*;

pub const SDL_MS_PER_SECOND: ::core::primitive::i32 = 1000;

pub const SDL_US_PER_SECOND: ::core::primitive::i32 = 1000000;

pub const SDL_NS_PER_SECOND: ::core::primitive::i64 = 1000000000_i64;

pub const SDL_NS_PER_MS: ::core::primitive::i32 = 1000000;

pub const SDL_NS_PER_US: ::core::primitive::i32 = 1000;

extern_sdlcall! {{
    /// Get the number of milliseconds since SDL library initialization.
    ///
    /// \returns an unsigned 64-bit value representing the number of milliseconds
    ///          since the SDL library initialized.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTicks() -> Uint64;
}}

extern_sdlcall! {{
    /// Get the number of nanoseconds since SDL library initialization.
    ///
    /// \returns an unsigned 64-bit value representing the number of nanoseconds
    ///          since the SDL library initialized.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTicksNS() -> Uint64;
}}

extern_sdlcall! {{
    /// Get the current value of the high resolution counter.
    ///
    /// This function is typically used for profiling.
    ///
    /// The counter values are only meaningful relative to each other. Differences
    /// between values can be converted to times by using
    /// SDL_GetPerformanceFrequency().
    ///
    /// \returns the current counter value.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPerformanceFrequency
    pub fn SDL_GetPerformanceCounter() -> Uint64;
}}

extern_sdlcall! {{
    /// Get the count per second of the high resolution counter.
    ///
    /// \returns a platform-specific count per second.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPerformanceCounter
    pub fn SDL_GetPerformanceFrequency() -> Uint64;
}}

extern_sdlcall! {{
    /// Wait a specified number of milliseconds before returning.
    ///
    /// This function waits a specified number of milliseconds before returning. It
    /// waits at least the specified time, but possibly longer due to OS
    /// scheduling.
    ///
    /// \param ms the number of milliseconds to delay.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_Delay(ms: Uint32);
}}

extern_sdlcall! {{
    /// Wait a specified number of nanoseconds before returning.
    ///
    /// This function waits a specified number of nanoseconds before returning. It
    /// will attempt to wait as close to the requested time as possible, busy
    /// waiting if necessary, but could return later due to OS scheduling.
    ///
    /// \param ns the number of nanoseconds to delay.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DelayNS(ns: Uint64);
}}

/// Definition of the timer ID type.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_TimerID = Uint32;

/// Function prototype for the millisecond timer callback function.
///
/// The callback function is passed the current timer interval and returns the
/// next timer interval, in milliseconds. If the returned value is the same as
/// the one passed in, the periodic alarm continues, otherwise a new alarm is
/// scheduled. If the callback returns 0, the periodic alarm is canceled and
/// will be removed.
///
/// \param userdata an arbitrary pointer provided by the app through
///                 SDL_AddTimer, for its own use.
/// \param timerID the current timer being processed.
/// \param interval the current callback time interval.
/// \returns the new callback time interval, or 0 to disable further runs of
///          the callback.
///
/// \threadsafety SDL may call this callback at any time from a background
///               thread; the application is responsible for locking resources
///               the callback touches that need to be protected.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_AddTimer
pub type SDL_TimerCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, timerID: SDL_TimerID, interval: Uint32) -> Uint32)>;

extern_sdlcall! {{
    /// Call a callback function at a future time.
    ///
    /// If you use this function, you must pass `SDL_INIT_TIMER` to SDL_Init().
    ///
    /// The callback function is passed the current timer interval and the user
    /// supplied parameter from the SDL_AddTimer() call and should return the next
    /// timer interval. If the value returned from the callback is 0, the timer is
    /// canceled and will be removed.
    ///
    /// The callback is run on a separate thread, and for short timeouts can
    /// potentially be called before this function returns.
    ///
    /// Timers take into account the amount of time it took to execute the
    /// callback. For example, if the callback took 250 ms to execute and returned
    /// 1000 (ms), the timer would only wait another 750 ms before its next
    /// iteration.
    ///
    /// Timing may be inexact due to OS scheduling. Be sure to note the current
    /// time with SDL_GetTicksNS() or SDL_GetPerformanceCounter() in case your
    /// callback needs to adjust for variances.
    ///
    /// \param interval the timer delay, in milliseconds, passed to `callback`.
    /// \param callback the SDL_TimerCallback function to call when the specified
    ///                 `interval` elapses.
    /// \param userdata a pointer that is passed to `callback`.
    /// \returns a timer ID or 0 on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddTimerNS
    /// \sa SDL_RemoveTimer
    pub fn SDL_AddTimer(interval: Uint32, callback: SDL_TimerCallback, userdata: *mut ::core::ffi::c_void) -> SDL_TimerID;
}}

/// Function prototype for the nanosecond timer callback function.
///
/// The callback function is passed the current timer interval and returns the
/// next timer interval, in nanoseconds. If the returned value is the same as
/// the one passed in, the periodic alarm continues, otherwise a new alarm is
/// scheduled. If the callback returns 0, the periodic alarm is canceled and
/// will be removed.
///
/// \param userdata an arbitrary pointer provided by the app through
///                 SDL_AddTimer, for its own use.
/// \param timerID the current timer being processed.
/// \param interval the current callback time interval.
/// \returns the new callback time interval, or 0 to disable further runs of
///          the callback.
///
/// \threadsafety SDL may call this callback at any time from a background
///               thread; the application is responsible for locking resources
///               the callback touches that need to be protected.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_AddTimerNS
pub type SDL_NSTimerCallback = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, timerID: SDL_TimerID, interval: Uint64) -> Uint64)>;

extern_sdlcall! {{
    /// Call a callback function at a future time.
    ///
    /// If you use this function, you must pass `SDL_INIT_TIMER` to SDL_Init().
    ///
    /// The callback function is passed the current timer interval and the user
    /// supplied parameter from the SDL_AddTimerNS() call and should return the
    /// next timer interval. If the value returned from the callback is 0, the
    /// timer is canceled and will be removed.
    ///
    /// The callback is run on a separate thread, and for short timeouts can
    /// potentially be called before this function returns.
    ///
    /// Timers take into account the amount of time it took to execute the
    /// callback. For example, if the callback took 250 ns to execute and returned
    /// 1000 (ns), the timer would only wait another 750 ns before its next
    /// iteration.
    ///
    /// Timing may be inexact due to OS scheduling. Be sure to note the current
    /// time with SDL_GetTicksNS() or SDL_GetPerformanceCounter() in case your
    /// callback needs to adjust for variances.
    ///
    /// \param interval the timer delay, in nanoseconds, passed to `callback`.
    /// \param callback the SDL_TimerCallback function to call when the specified
    ///                 `interval` elapses.
    /// \param userdata a pointer that is passed to `callback`.
    /// \returns a timer ID or 0 on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddTimer
    /// \sa SDL_RemoveTimer
    pub fn SDL_AddTimerNS(interval: Uint64, callback: SDL_NSTimerCallback, userdata: *mut ::core::ffi::c_void) -> SDL_TimerID;
}}

extern_sdlcall! {{
    /// Remove a timer created with SDL_AddTimer().
    ///
    /// \param id the ID of the timer to remove.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddTimer
    pub fn SDL_RemoveTimer(id: SDL_TimerID) -> SDL_bool;
}}

