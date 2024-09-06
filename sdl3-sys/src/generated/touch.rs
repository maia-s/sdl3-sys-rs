#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryTouch
//!
//! SDL touch management.

use super::stdinc::*;

use super::error::*;

use super::mouse::*;

use super::video::*;

pub type SDL_TouchID = Uint64;

pub type SDL_FingerID = Uint64;

/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_TOUCH_DEVICE_INVALID`], [`SDL_TOUCH_DEVICE_DIRECT`], [`SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE`], [`SDL_TOUCH_DEVICE_INDIRECT_RELATIVE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TouchDeviceType(pub ::core::ffi::c_int);
impl SDL_TouchDeviceType {
    pub const INVALID: Self = Self(-1_i32);
    pub const DIRECT: Self = Self(0_i32);
    pub const INDIRECT_ABSOLUTE: Self = Self(1_i32);
    pub const INDIRECT_RELATIVE: Self = Self(2_i32);
}
pub const SDL_TOUCH_DEVICE_INVALID: SDL_TouchDeviceType = SDL_TouchDeviceType::INVALID;
pub const SDL_TOUCH_DEVICE_DIRECT: SDL_TouchDeviceType = SDL_TouchDeviceType::DIRECT;
pub const SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE: SDL_TouchDeviceType = SDL_TouchDeviceType::INDIRECT_ABSOLUTE;
pub const SDL_TOUCH_DEVICE_INDIRECT_RELATIVE: SDL_TouchDeviceType = SDL_TouchDeviceType::INDIRECT_RELATIVE;

/// Data about a single finger in a multitouch event.
///
/// Each touch even is a collection of fingers that are simultaneously in
/// contact with the touch device (so a "touch" can be a "multitouch," in
/// reality), and this struct reports details of the specific fingers.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetTouchFingers
#[repr(C)]
#[derive(Clone, Copy)]
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

pub const SDL_TOUCH_MOUSEID: SDL_MouseID = (-1_i32) as SDL_MouseID;

pub const SDL_MOUSE_TOUCHID: SDL_TouchID = (-1_i32) as SDL_TouchID;

extern_sdlcall! {{
    /// Get a list of registered touch devices.
    ///
    /// On some platforms SDL first sees the touch device if it was actually used.
    /// Therefore the returned list might be empty, although devices are available.
    /// After using all devices at least once the number will be correct.
    ///
    /// \param count a pointer filled in with the number of devices returned, may
    ///              be NULL.
    /// \returns a 0 terminated array of touch device IDs or NULL on failure; call
    ///          SDL_GetError() for more information. This should be freed with
    ///          SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTouchDevices(count: *mut ::core::ffi::c_int) -> *mut SDL_TouchID;
}}

extern_sdlcall! {{
    /// Get the touch device name as reported from the driver.
    ///
    /// \param touchID the touch device instance ID.
    /// \returns touch device name, or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTouchDeviceName(touchID: SDL_TouchID) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the type of the given touch device.
    ///
    /// \param touchID the ID of a touch device.
    /// \returns touch device type.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTouchDeviceType(touchID: SDL_TouchID) -> SDL_TouchDeviceType;
}}

extern_sdlcall! {{
    /// Get a list of active fingers for a given touch device.
    ///
    /// \param touchID the ID of a touch device.
    /// \param count a pointer filled in with the number of fingers returned, can
    ///              be NULL.
    /// \returns a NULL terminated array of SDL_Finger pointers or NULL on failure;
    ///          call SDL_GetError() for more information. This is a single
    ///          allocation that should be freed with SDL_free() when it is no
    ///          longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTouchFingers(touchID: SDL_TouchID, count: *mut ::core::ffi::c_int) -> *mut *mut SDL_Finger;
}}
