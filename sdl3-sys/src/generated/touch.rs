//! SDL offers touch input, on platforms that support it. It can manage
//! multiple touch devices and track multiple fingers on those devices.
//!
//! Touches are mostly dealt with through the event system, in the
//! [`SDL_EVENT_FINGER_DOWN`], [`SDL_EVENT_FINGER_MOTION`], and [`SDL_EVENT_FINGER_UP`]
//! events, but there are also functions to query for hardware details, etc.
//!
//! The touch system, by default, will also send virtual mouse events; this can
//! be useful for making a some desktop apps work on a phone without
//! significant changes. For apps that care about mouse and touch input
//! separately, they should ignore mouse events that have a `which` field of
//! [`SDL_TOUCH_MOUSEID`].

use super::stdinc::*;

use super::error::*;

use super::mouse::*;

/// A unique ID for a touch device.
///
/// This ID is valid for the time the device is connected to the system, and is
/// never reused for the lifetime of the application.
///
/// The value 0 is an invalid ID.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TouchID(pub Uint64);

impl ::core::cmp::PartialEq<Uint64> for SDL_TouchID {
    #[inline(always)]
    fn eq(&self, other: &Uint64) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_TouchID> for Uint64 {
    #[inline(always)]
    fn eq(&self, other: &SDL_TouchID) -> bool {
        self == &other.0
    }
}

impl From<SDL_TouchID> for Uint64 {
    #[inline(always)]
    fn from(value: SDL_TouchID) -> Self {
        value.0
    }
}

#[cfg(feature = "display-impls")]
impl ::core::fmt::Display for SDL_TouchID {
    #[inline(always)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <Uint64 as ::core::fmt::Display>::fmt(&self.0, f)
    }
}

impl SDL_TouchID {
    /// Initialize a `SDL_TouchID` from a raw value.
    #[inline(always)]
    pub const fn new(value: Uint64) -> Self {
        Self(value)
    }
}

impl SDL_TouchID {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> Uint64 {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_TouchID {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::touch::METADATA_SDL_TouchID;
}

/// A unique ID for a single finger on a touch device.
///
/// This ID is valid for the time the finger (stylus, etc) is touching and will
/// be unique for all fingers currently in contact, so this ID tracks the
/// lifetime of a single continuous touch. This value may represent an index, a
/// pointer, or some other unique ID, depending on the platform.
///
/// The value 0 is an invalid ID.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FingerID(pub Uint64);

impl ::core::cmp::PartialEq<Uint64> for SDL_FingerID {
    #[inline(always)]
    fn eq(&self, other: &Uint64) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_FingerID> for Uint64 {
    #[inline(always)]
    fn eq(&self, other: &SDL_FingerID) -> bool {
        self == &other.0
    }
}

impl From<SDL_FingerID> for Uint64 {
    #[inline(always)]
    fn from(value: SDL_FingerID) -> Self {
        value.0
    }
}

#[cfg(feature = "display-impls")]
impl ::core::fmt::Display for SDL_FingerID {
    #[inline(always)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <Uint64 as ::core::fmt::Display>::fmt(&self.0, f)
    }
}

impl SDL_FingerID {
    /// Initialize a `SDL_FingerID` from a raw value.
    #[inline(always)]
    pub const fn new(value: Uint64) -> Self {
        Self(value)
    }
}

impl SDL_FingerID {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> Uint64 {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_FingerID {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::touch::METADATA_SDL_FingerID;
}

/// An enum that describes the type of a touch device.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_TouchDeviceType::INVALID) | [`SDL_TOUCH_DEVICE_INVALID`] | |
/// | [`DIRECT`](SDL_TouchDeviceType::DIRECT) | [`SDL_TOUCH_DEVICE_DIRECT`] | touch screen with window-relative coordinates |
/// | [`INDIRECT_ABSOLUTE`](SDL_TouchDeviceType::INDIRECT_ABSOLUTE) | [`SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE`] | trackpad with absolute device coordinates |
/// | [`INDIRECT_RELATIVE`](SDL_TouchDeviceType::INDIRECT_RELATIVE) | [`SDL_TOUCH_DEVICE_INDIRECT_RELATIVE`] | trackpad with screen cursor-relative coordinates |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_TouchDeviceType(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_TouchDeviceType {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_TouchDeviceType> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_TouchDeviceType) -> bool {
        self == &other.0
    }
}

impl From<SDL_TouchDeviceType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_TouchDeviceType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_TouchDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_TOUCH_DEVICE_INVALID",
            Self::DIRECT => "SDL_TOUCH_DEVICE_DIRECT",
            Self::INDIRECT_ABSOLUTE => "SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE",
            Self::INDIRECT_RELATIVE => "SDL_TOUCH_DEVICE_INDIRECT_RELATIVE",

            _ => return write!(f, "SDL_TouchDeviceType({})", self.0),
        })
    }
}

impl SDL_TouchDeviceType {
    pub const INVALID: Self = Self((-1_i32 as ::core::ffi::c_int));
    /// touch screen with window-relative coordinates
    pub const DIRECT: Self = Self((0_i32 as ::core::ffi::c_int));
    /// trackpad with absolute device coordinates
    pub const INDIRECT_ABSOLUTE: Self = Self((1_i32 as ::core::ffi::c_int));
    /// trackpad with screen cursor-relative coordinates
    pub const INDIRECT_RELATIVE: Self = Self((2_i32 as ::core::ffi::c_int));
}

pub const SDL_TOUCH_DEVICE_INVALID: SDL_TouchDeviceType = SDL_TouchDeviceType::INVALID;
/// touch screen with window-relative coordinates
pub const SDL_TOUCH_DEVICE_DIRECT: SDL_TouchDeviceType = SDL_TouchDeviceType::DIRECT;
/// trackpad with absolute device coordinates
pub const SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE: SDL_TouchDeviceType =
    SDL_TouchDeviceType::INDIRECT_ABSOLUTE;
/// trackpad with screen cursor-relative coordinates
pub const SDL_TOUCH_DEVICE_INDIRECT_RELATIVE: SDL_TouchDeviceType =
    SDL_TouchDeviceType::INDIRECT_RELATIVE;

impl SDL_TouchDeviceType {
    /// Initialize a `SDL_TouchDeviceType` from a raw value.
    #[inline(always)]
    pub const fn new(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }
}

impl SDL_TouchDeviceType {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_TouchDeviceType {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::touch::METADATA_SDL_TouchDeviceType;
}

/// Data about a single finger in a multitouch event.
///
/// Each touch event is a collection of fingers that are simultaneously in
/// contact with the touch device (so a "touch" can be a "multitouch," in
/// reality), and this struct reports details of the specific fingers.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetTouchFingers`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Finger {
    /// the finger ID
    pub id: SDL_FingerID,
    /// the x-axis location of the touch event, normalized (0...1)
    pub x: ::core::ffi::c_float,
    /// the y-axis location of the touch event, normalized (0...1)
    pub y: ::core::ffi::c_float,
    /// the quantity of pressure applied, normalized (0...1)
    pub pressure: ::core::ffi::c_float,
}

/// The [`SDL_MouseID`] for mouse events simulated with touch input.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub const SDL_TOUCH_MOUSEID: SDL_MouseID = SDL_MouseID((-1_i32 as Uint32));

/// The [`SDL_TouchID`] for touch events simulated with mouse input.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub const SDL_MOUSE_TOUCHID: SDL_TouchID = SDL_TouchID((-1_i32 as Uint64));

unsafe extern "C" {
    /// Get a list of registered touch devices.
    ///
    /// On some platforms SDL first sees the touch device if it was actually used.
    /// Therefore the returned list might be empty, although devices are available.
    /// After using all devices at least once the number will be correct.
    ///
    /// ## Parameters
    /// - `count`: a pointer filled in with the number of devices returned, may
    ///   be NULL.
    ///
    /// ## Return value
    /// Returns a 0 terminated array of touch device IDs or NULL on failure; call
    ///   [`SDL_GetError()`] for more information. This should be freed with
    ///   [`SDL_free()`] when it is no longer needed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetTouchDevices(count: *mut ::core::ffi::c_int) -> *mut SDL_TouchID;
}

unsafe extern "C" {
    /// Get the touch device name as reported from the driver.
    ///
    /// ## Parameters
    /// - `touchID`: the touch device instance ID.
    ///
    /// ## Return value
    /// Returns touch device name, or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetTouchDeviceName(touchID: SDL_TouchID) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get the type of the given touch device.
    ///
    /// ## Parameters
    /// - `touchID`: the ID of a touch device.
    ///
    /// ## Return value
    /// Returns touch device type.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetTouchDeviceType(touchID: SDL_TouchID) -> SDL_TouchDeviceType;
}

unsafe extern "C" {
    /// Get a list of active fingers for a given touch device.
    ///
    /// ## Parameters
    /// - `touchID`: the ID of a touch device.
    /// - `count`: a pointer filled in with the number of fingers returned, can
    ///   be NULL.
    ///
    /// ## Return value
    /// Returns a NULL terminated array of [`SDL_Finger`] pointers or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This is a single
    ///   allocation that should be freed with [`SDL_free()`] when it is no
    ///   longer needed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetTouchFingers(
        touchID: SDL_TouchID,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut SDL_Finger;
}

#[cfg(doc)]
use crate::everything::*;
