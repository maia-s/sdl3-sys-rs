//! SDL power management routines.
//!
//! There is a single function in this category: [`SDL_GetPowerInfo()`].
//!
//! This function is useful for games on the go. This allows an app to know if
//! it's running on a draining battery, which can be useful if the app wants to
//! reduce processing, or perhaps framerate, to extend the duration of the
//! battery's charge. Perhaps the app just wants to show a battery meter when
//! fullscreen, or alert the user when the power is getting extremely low, so
//! they can save their game.

use super::stdinc::*;

use super::error::*;

/// The basic state for the system's power supply.
///
/// These are results returned by [`SDL_GetPowerInfo()`].
///
/// ### Availability
/// This enum is available since SDL 3.2.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`ERROR`](SDL_PowerState::ERROR) | [`SDL_POWERSTATE_ERROR`] | error determining power status |
/// | [`UNKNOWN`](SDL_PowerState::UNKNOWN) | [`SDL_POWERSTATE_UNKNOWN`] | cannot determine power status |
/// | [`ON_BATTERY`](SDL_PowerState::ON_BATTERY) | [`SDL_POWERSTATE_ON_BATTERY`] | Not plugged in, running on the battery |
/// | [`NO_BATTERY`](SDL_PowerState::NO_BATTERY) | [`SDL_POWERSTATE_NO_BATTERY`] | Plugged in, no battery available |
/// | [`CHARGING`](SDL_PowerState::CHARGING) | [`SDL_POWERSTATE_CHARGING`] | Plugged in, charging battery |
/// | [`CHARGED`](SDL_PowerState::CHARGED) | [`SDL_POWERSTATE_CHARGED`] | Plugged in, battery charged |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PowerState(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_PowerState {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_PowerState> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_PowerState) -> bool {
        self == &other.0
    }
}

impl From<SDL_PowerState> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PowerState) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::ERROR => "SDL_POWERSTATE_ERROR",
            Self::UNKNOWN => "SDL_POWERSTATE_UNKNOWN",
            Self::ON_BATTERY => "SDL_POWERSTATE_ON_BATTERY",
            Self::NO_BATTERY => "SDL_POWERSTATE_NO_BATTERY",
            Self::CHARGING => "SDL_POWERSTATE_CHARGING",
            Self::CHARGED => "SDL_POWERSTATE_CHARGED",

            _ => return write!(f, "SDL_PowerState({})", self.0),
        })
    }
}

impl SDL_PowerState {
    /// error determining power status
    pub const ERROR: Self = Self((-1_i32 as ::core::ffi::c_int));
    /// cannot determine power status
    pub const UNKNOWN: Self = Self((0_i32 as ::core::ffi::c_int));
    /// Not plugged in, running on the battery
    pub const ON_BATTERY: Self = Self((1_i32 as ::core::ffi::c_int));
    /// Plugged in, no battery available
    pub const NO_BATTERY: Self = Self((2_i32 as ::core::ffi::c_int));
    /// Plugged in, charging battery
    pub const CHARGING: Self = Self((3_i32 as ::core::ffi::c_int));
    /// Plugged in, battery charged
    pub const CHARGED: Self = Self((4_i32 as ::core::ffi::c_int));
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

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_PowerState {
    const GROUP_METADATA: &sdl3_sys::metadata::Group =
        &crate::metadata::GROUPS[crate::metadata::GROUP_OFFSET_power + 0];
}

extern "C" {
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
    /// On some platforms, retrieving power supply details might be expensive. If
    /// you want to display continuous status you could call this function every
    /// minute or so.
    ///
    /// ### Parameters
    /// - `seconds`: a pointer filled in with the seconds of battery life left,
    ///   or NULL to ignore. This will be filled in with -1 if we
    ///   can't determine a value or there is no battery.
    /// - `percent`: a pointer filled in with the percentage of battery life
    ///   left, between 0 and 100, or NULL to ignore. This will be
    ///   filled in with -1 we can't determine a value or there is no
    ///   battery.
    ///
    /// ### Return value
    /// Returns the current battery state or [`SDL_POWERSTATE_ERROR`] on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetPowerInfo(
        seconds: *mut ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> SDL_PowerState;
}

#[cfg(doc)]
use crate::everything::*;
