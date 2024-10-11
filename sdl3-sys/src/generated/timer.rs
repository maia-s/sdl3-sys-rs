//! SDL time management routines.

use super::stdinc::*;

use super::error::*;

pub const SDL_MS_PER_SECOND: ::core::primitive::i32 = 1000;

pub const SDL_US_PER_SECOND: ::core::primitive::i32 = 1000000;

pub const SDL_NS_PER_SECOND: ::core::primitive::i64 = 1000000000_i64;

pub const SDL_NS_PER_MS: ::core::primitive::i32 = 1000000;

pub const SDL_NS_PER_US: ::core::primitive::i32 = 1000;

#[inline(always)]
pub const fn SDL_SECONDS_TO_NS(S: Uint64) -> Uint64 {
    (((S) as Uint64) * 1000000000_u64)
}

#[inline(always)]
pub const fn SDL_NS_TO_SECONDS(NS: ::core::primitive::i64) -> ::core::primitive::i64 {
    (NS / 1000000000_i64)
}

#[inline(always)]
pub const fn SDL_MS_TO_NS(MS: Uint64) -> Uint64 {
    (((MS) as Uint64) * 1000000_u64)
}

#[inline(always)]
pub const fn SDL_NS_TO_MS(NS: ::core::primitive::i32) -> ::core::primitive::i32 {
    (NS / 1000000_i32)
}

#[inline(always)]
pub const fn SDL_US_TO_NS(US: Uint64) -> Uint64 {
    (((US) as Uint64) * 1000_u64)
}

#[inline(always)]
pub const fn SDL_NS_TO_US(NS: ::core::primitive::i32) -> ::core::primitive::i32 {
    (NS / 1000_i32)
}

extern "C" {
    /// Get the number of milliseconds since SDL library initialization.
    ///
    /// - Returns an unsigned 64-bit value representing the number of milliseconds
    ///   since the SDL library initialized.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetTicks() -> Uint64;
}

extern "C" {
    /// Get the number of nanoseconds since SDL library initialization.
    ///
    /// - Returns an unsigned 64-bit value representing the number of nanoseconds
    ///   since the SDL library initialized.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetTicksNS() -> Uint64;
}

extern "C" {
    /// Get the current value of the high resolution counter.
    ///
    /// This function is typically used for profiling.
    ///
    /// The counter values are only meaningful relative to each other. Differences
    /// between values can be converted to times by using
    /// [`SDL_GetPerformanceFrequency()`].
    ///
    /// - Returns the current counter value.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetPerformanceFrequency`]<br>
    pub fn SDL_GetPerformanceCounter() -> Uint64;
}

extern "C" {
    /// Get the count per second of the high resolution counter.
    ///
    /// - Returns a platform-specific count per second.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetPerformanceCounter`]<br>
    pub fn SDL_GetPerformanceFrequency() -> Uint64;
}

extern "C" {
    /// Wait a specified number of milliseconds before returning.
    ///
    /// This function waits a specified number of milliseconds before returning. It
    /// waits at least the specified time, but possibly longer due to OS
    /// scheduling.
    ///
    /// - `ms`: the number of milliseconds to delay.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_Delay(ms: Uint32);
}

extern "C" {
    /// Wait a specified number of nanoseconds before returning.
    ///
    /// This function waits a specified number of nanoseconds before returning. It
    /// waits at least the specified time, but possibly longer due to OS
    /// scheduling.
    ///
    /// - `ns`: the number of nanoseconds to delay.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_DelayNS(ns: Uint64);
}

extern "C" {
    /// Wait a specified number of nanoseconds before returning.
    ///
    /// This function waits a specified number of nanoseconds before returning. It
    /// will attempt to wait as close to the requested time as possible, busy
    /// waiting if necessary, but could return later due to OS scheduling.
    ///
    /// - `ns`: the number of nanoseconds to delay.
    ///
    /// This function is available since SDL 3.1.4.
    pub fn SDL_DelayPrecise(ns: Uint64);
}

/// Definition of the timer ID type.
///
/// This datatype is available since SDL 3.0.0.
pub type SDL_TimerID = Uint32;

/// Function prototype for the millisecond timer callback function.
///
/// The callback function is passed the current timer interval and returns the
/// next timer interval, in milliseconds. If the returned value is the same as
/// the one passed in, the periodic alarm continues, otherwise a new alarm is
/// scheduled. If the callback returns 0, the periodic alarm is canceled and
/// will be removed.
///
/// - `userdata`: an arbitrary pointer provided by the app through
///   [`SDL_AddTimer`], for its own use.
/// - `timerID`: the current timer being processed.
/// - `interval`: the current callback time interval.
/// - Returns the new callback time interval, or 0 to disable further runs of
///   the callback.
///
/// Thread safety: SDL may call this callback at any time from a background
///   thread; the application is responsible for locking resources
///   the callback touches that need to be protected.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_AddTimer`]<br>
pub type SDL_TimerCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        timerID: SDL_TimerID,
        interval: Uint32,
    ) -> Uint32,
>;

extern "C" {
    /// Call a callback function at a future time.
    ///
    /// The callback function is passed the current timer interval and the user
    /// supplied parameter from the [`SDL_AddTimer()`] call and should return the next
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
    /// time with [`SDL_GetTicksNS()`] or [`SDL_GetPerformanceCounter()`] in case your
    /// callback needs to adjust for variances.
    ///
    /// - `interval`: the timer delay, in milliseconds, passed to `callback`.
    /// - `callback`: the [`SDL_TimerCallback`] function to call when the specified
    ///   `interval` elapses.
    /// - `userdata`: a pointer that is passed to `callback`.
    /// - Returns a timer ID or 0 on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_AddTimerNS`]<br>
    /// See also [`SDL_RemoveTimer`]<br>
    pub fn SDL_AddTimer(
        interval: Uint32,
        callback: SDL_TimerCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> SDL_TimerID;
}

/// Function prototype for the nanosecond timer callback function.
///
/// The callback function is passed the current timer interval and returns the
/// next timer interval, in nanoseconds. If the returned value is the same as
/// the one passed in, the periodic alarm continues, otherwise a new alarm is
/// scheduled. If the callback returns 0, the periodic alarm is canceled and
/// will be removed.
///
/// - `userdata`: an arbitrary pointer provided by the app through
///   [`SDL_AddTimer`], for its own use.
/// - `timerID`: the current timer being processed.
/// - `interval`: the current callback time interval.
/// - Returns the new callback time interval, or 0 to disable further runs of
///   the callback.
///
/// Thread safety: SDL may call this callback at any time from a background
///   thread; the application is responsible for locking resources
///   the callback touches that need to be protected.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_AddTimerNS`]<br>
pub type SDL_NSTimerCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        timerID: SDL_TimerID,
        interval: Uint64,
    ) -> Uint64,
>;

extern "C" {
    /// Call a callback function at a future time.
    ///
    /// The callback function is passed the current timer interval and the user
    /// supplied parameter from the [`SDL_AddTimerNS()`] call and should return the
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
    /// time with [`SDL_GetTicksNS()`] or [`SDL_GetPerformanceCounter()`] in case your
    /// callback needs to adjust for variances.
    ///
    /// - `interval`: the timer delay, in nanoseconds, passed to `callback`.
    /// - `callback`: the [`SDL_TimerCallback`] function to call when the specified
    ///   `interval` elapses.
    /// - `userdata`: a pointer that is passed to `callback`.
    /// - Returns a timer ID or 0 on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_AddTimer`]<br>
    /// See also [`SDL_RemoveTimer`]<br>
    pub fn SDL_AddTimerNS(
        interval: Uint64,
        callback: SDL_NSTimerCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> SDL_TimerID;
}

extern "C" {
    /// Remove a timer created with [`SDL_AddTimer()`].
    ///
    /// - `id`: the ID of the timer to remove.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_AddTimer`]<br>
    pub fn SDL_RemoveTimer(id: SDL_TimerID) -> ::core::primitive::bool;
}
