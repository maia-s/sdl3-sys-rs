//! SDL realtime clock and date/time routines.

use super::error::*;

use super::stdinc::*;

/// A structure holding a calendar date and time broken down into its
/// components.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_DateTime {
    /// Year
    pub year: ::core::ffi::c_int,
    /// Month \[01-12\]
    pub month: ::core::ffi::c_int,
    /// Day of the month \[01-31\]
    pub day: ::core::ffi::c_int,
    /// Hour \[0-23\]
    pub hour: ::core::ffi::c_int,
    /// Minute \[0-59\]
    pub minute: ::core::ffi::c_int,
    /// Seconds \[0-60\]
    pub second: ::core::ffi::c_int,
    /// Nanoseconds \[0-999999999\]
    pub nanosecond: ::core::ffi::c_int,
    /// Day of the week \[0-6\] (0 being Sunday)
    pub day_of_week: ::core::ffi::c_int,
    /// Seconds east of UTC
    pub utc_offset: ::core::ffi::c_int,
}

/// The preferred date format of the current system locale.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetDateTimeLocalePreferences`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`YYYYMMDD`](SDL_DateFormat::YYYYMMDD) | [`SDL_DATE_FORMAT_YYYYMMDD`] | Year/Month/Day |
/// | [`DDMMYYYY`](SDL_DateFormat::DDMMYYYY) | [`SDL_DATE_FORMAT_DDMMYYYY`] | Day/Month/Year |
/// | [`MMDDYYYY`](SDL_DateFormat::MMDDYYYY) | [`SDL_DATE_FORMAT_MMDDYYYY`] | Month/Day/Year |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_DateFormat(pub ::core::ffi::c_int);

impl From<SDL_DateFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_DateFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_DateFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::YYYYMMDD => "SDL_DATE_FORMAT_YYYYMMDD",
            Self::DDMMYYYY => "SDL_DATE_FORMAT_DDMMYYYY",
            Self::MMDDYYYY => "SDL_DATE_FORMAT_MMDDYYYY",

            _ => return write!(f, "SDL_DateFormat({})", self.0),
        })
    }
}

impl SDL_DateFormat {
    /// Year/Month/Day
    pub const YYYYMMDD: Self = Self(0);
    /// Day/Month/Year
    pub const DDMMYYYY: Self = Self(1);
    /// Month/Day/Year
    pub const MMDDYYYY: Self = Self(2);
}

/// Year/Month/Day
pub const SDL_DATE_FORMAT_YYYYMMDD: SDL_DateFormat = SDL_DateFormat::YYYYMMDD;
/// Day/Month/Year
pub const SDL_DATE_FORMAT_DDMMYYYY: SDL_DateFormat = SDL_DateFormat::DDMMYYYY;
/// Month/Day/Year
pub const SDL_DATE_FORMAT_MMDDYYYY: SDL_DateFormat = SDL_DateFormat::MMDDYYYY;

/// The preferred time format of the current system locale.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetDateTimeLocalePreferences`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`_24HR`](SDL_TimeFormat::_24HR) | [`SDL_TIME_FORMAT_24HR`] | 24 hour time |
/// | [`_12HR`](SDL_TimeFormat::_12HR) | [`SDL_TIME_FORMAT_12HR`] | 12 hour time |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_TimeFormat(pub ::core::ffi::c_int);

impl From<SDL_TimeFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_TimeFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_TimeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::_24HR => "SDL_TIME_FORMAT_24HR",
            Self::_12HR => "SDL_TIME_FORMAT_12HR",

            _ => return write!(f, "SDL_TimeFormat({})", self.0),
        })
    }
}

impl SDL_TimeFormat {
    /// 24 hour time
    pub const _24HR: Self = Self(0);
    /// 12 hour time
    pub const _12HR: Self = Self(1);
}

/// 24 hour time
pub const SDL_TIME_FORMAT_24HR: SDL_TimeFormat = SDL_TimeFormat::_24HR;
/// 12 hour time
pub const SDL_TIME_FORMAT_12HR: SDL_TimeFormat = SDL_TimeFormat::_12HR;

extern "C" {
    /// Gets the current preferred date and time format for the system locale.
    ///
    /// This might be a "slow" call that has to query the operating system. It's
    /// best to ask for this once and save the results. However, the preferred
    /// formats can change, usually because the user has changed a system
    /// preference outside of your program.
    ///
    /// ### Parameters
    /// - `dateFormat`: a pointer to the [`SDL_DateFormat`] to hold the returned date
    ///   format, may be NULL.
    /// - `timeFormat`: a pointer to the [`SDL_TimeFormat`] to hold the returned time
    ///   format, may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetDateTimeLocalePreferences(
        dateFormat: *mut SDL_DateFormat,
        timeFormat: *mut SDL_TimeFormat,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Gets the current value of the system realtime clock in nanoseconds since
    /// Jan 1, 1970 in Universal Coordinated Time (UTC).
    ///
    /// ### Parameters
    /// - `ticks`: the [`SDL_Time`] to hold the returned tick count.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetCurrentTime(ticks: *mut SDL_Time) -> ::core::primitive::bool;
}

extern "C" {
    /// Converts an [`SDL_Time`] in nanoseconds since the epoch to a calendar time in
    /// the [`SDL_DateTime`] format.
    ///
    /// ### Parameters
    /// - `ticks`: the [`SDL_Time`] to be converted.
    /// - `dt`: the resulting [`SDL_DateTime`].
    /// - `localTime`: the resulting [`SDL_DateTime`] will be expressed in local time
    ///   if true, otherwise it will be in Universal Coordinated
    ///   Time (UTC).
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_TimeToDateTime(
        ticks: SDL_Time,
        dt: *mut SDL_DateTime,
        localTime: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Converts a calendar time to an [`SDL_Time`] in nanoseconds since the epoch.
    ///
    /// This function ignores the day_of_week member of the [`SDL_DateTime`] struct, so
    /// it may remain unset.
    ///
    /// ### Parameters
    /// - `dt`: the source [`SDL_DateTime`].
    /// - `ticks`: the resulting [`SDL_Time`].
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_DateTimeToTime(
        dt: *const SDL_DateTime,
        ticks: *mut SDL_Time,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Converts an SDL time into a Windows FILETIME (100-nanosecond intervals
    /// since January 1, 1601).
    ///
    /// This function fills in the two 32-bit values of the FILETIME structure.
    ///
    /// ### Parameters
    /// - `ticks`: the time to convert.
    /// - `dwLowDateTime`: a pointer filled in with the low portion of the
    ///   Windows FILETIME value.
    /// - `dwHighDateTime`: a pointer filled in with the high portion of the
    ///   Windows FILETIME value.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_TimeToWindows(
        ticks: SDL_Time,
        dwLowDateTime: *mut Uint32,
        dwHighDateTime: *mut Uint32,
    );
}

extern "C" {
    /// Converts a Windows FILETIME (100-nanosecond intervals since January 1,
    /// 1601) to an SDL time.
    ///
    /// This function takes the two 32-bit values of the FILETIME structure as
    /// parameters.
    ///
    /// ### Parameters
    /// - `dwLowDateTime`: the low portion of the Windows FILETIME value.
    /// - `dwHighDateTime`: the high portion of the Windows FILETIME value.
    ///
    /// ### Return value
    /// Returns the converted SDL time.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_TimeFromWindows(dwLowDateTime: Uint32, dwHighDateTime: Uint32) -> SDL_Time;
}

extern "C" {
    /// Get the number of days in a month for a given year.
    ///
    /// ### Parameters
    /// - `year`: the year.
    /// - `month`: the month \[1-12\].
    ///
    /// ### Return value
    /// Returns the number of days in the requested month or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetDaysInMonth(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the day of year for a calendar date.
    ///
    /// ### Parameters
    /// - `year`: the year component of the date.
    /// - `month`: the month component of the date.
    /// - `day`: the day component of the date.
    ///
    /// ### Return value
    /// Returns the day of year \[0-365\] if the date is valid or -1 on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetDayOfYear(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
        day: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the day of week for a calendar date.
    ///
    /// ### Parameters
    /// - `year`: the year component of the date.
    /// - `month`: the month component of the date.
    /// - `day`: the day component of the date.
    ///
    /// ### Return value
    /// Returns a value between 0 and 6 (0 being Sunday) if the date is valid or
    ///   -1 on failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetDayOfWeek(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
        day: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[cfg(doc)]
use crate::everything::*;
