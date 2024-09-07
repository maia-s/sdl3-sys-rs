#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryPower
//!
//! SDL power management routines.

use super::stdinc::*;

use super::error::*;

/// The basic state for the system's power supply.
///
/// These are results returned by SDL_GetPowerInfo().
///
/// \since This enum is available since SDL 3.0.0
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_POWERSTATE_ERROR`], [`SDL_POWERSTATE_UNKNOWN`], [`SDL_POWERSTATE_ON_BATTERY`], [`SDL_POWERSTATE_NO_BATTERY`], [`SDL_POWERSTATE_CHARGING`], [`SDL_POWERSTATE_CHARGED`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PowerState(pub ::core::ffi::c_int);
impl SDL_PowerState {
    /// error determining power status
    pub const ERROR: Self = Self(-1_i32);
    /// cannot determine power status
    pub const UNKNOWN: Self = Self(0_i32);
    /// Not plugged in, running on the battery
    pub const ON_BATTERY: Self = Self(1_i32);
    /// Plugged in, no battery available
    pub const NO_BATTERY: Self = Self(2_i32);
    /// Plugged in, charging battery
    pub const CHARGING: Self = Self(3_i32);
    /// Plugged in, battery charged
    pub const CHARGED: Self = Self(4_i32);
}
/// error determining power status
pub const SDL_POWERSTATE_ERROR: SDL_PowerState = SDL_PowerState::ERROR;
/// cannot determine power status
pub const SDL_POWERSTATE_UNKNOWN: SDL_PowerState = SDL_PowerState::UNKNOWN;
/// Not plugged in, running on the battery
pub const SDL_POWERSTATE_ON_BATTERY: SDL_PowerState = SDL_PowerState::ON_BATTERY;
/// Plugged in, no battery available
pub const SDL_POWERSTATE_NO_BATTERY: SDL_PowerState = SDL_PowerState::NO_BATTERY;
/// Plugged in, charging battery
pub const SDL_POWERSTATE_CHARGING: SDL_PowerState = SDL_PowerState::CHARGING;
/// Plugged in, battery charged
pub const SDL_POWERSTATE_CHARGED: SDL_PowerState = SDL_PowerState::CHARGED;

extern_sdlcall! {{
    /// Get the current power supply details.
    ///
    /// You should never take a battery status as absolute truth. Batteries
    /// (especially failing batteries) are delicate hardware, and the values
    /// reported here are best estimates based on what that hardware reports. It's
    /// not uncommon for older batteries to lose stored power much faster than it
    /// reports, or completely drain when reporting it has 20 percent left, etc.
    ///
    /// Battery status can change at any time; if you are concerned with power
    /// state, you should call this function frequently, and perhaps ignore changes
    /// until they seem to be stable for a few seconds.
    ///
    /// It's possible a platform can only report battery percentage or time left
    /// but not both.
    ///
    /// \param seconds a pointer filled in with the seconds of battery life left,
    ///                or NULL to ignore. This will be filled in with -1 if we
    ///                can't determine a value or there is no battery.
    /// \param percent a pointer filled in with the percentage of battery life
    ///                left, between 0 and 100, or NULL to ignore. This will be
    ///                filled in with -1 we can't determine a value or there is no
    ///                battery.
    /// \returns the current battery state or `SDL_POWERSTATE_ERROR` on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetPowerInfo(seconds: *mut ::core::ffi::c_int, percent: *mut ::core::ffi::c_int) -> SDL_PowerState;
}}

