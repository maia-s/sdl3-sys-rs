#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryPen
//!
//! SDL pen event handling.
//!
//! SDL provides an API for pressure-sensitive pen (stylus and/or eraser)
//! handling, e.g., for input and drawing tablets or suitably equipped mobile /
//! tablet devices.
//!
//! To get started with pens, simply handle SDL_EVENT_PEN_* events. When a pen
//! starts providing input, SDL will assign it a unique SDL_PenID, which will
//! remain for the life of the process, as long as the pen stays connected.
//!
//! Pens may provide more than simple touch input; they might have other axes,
//! such as pressure, tilt, rotation, etc.

use super::error::*;

/// SDL pen instance IDs.
///
/// Zero is used to signify an invalid/null device.
///
/// These show up in pen events when SDL sees input from them. They remain
/// consistent as long as SDL can recognize a tool to be the same pen; but if a
/// pen physically leaves the area and returns, it might get a new ID.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_PenID = Uint32;

/// Pen input flags, as reported by various pen events' `pen_state` field.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_PenInputFlags = Uint32;

/// pen is pressed down
pub const SDL_PEN_INPUT_DOWN: ::core::primitive::u32 = 1_u32;

/// button 1 is pressed
pub const SDL_PEN_INPUT_BUTTON_1: ::core::primitive::u32 = 2_u32;

/// button 2 is pressed
pub const SDL_PEN_INPUT_BUTTON_2: ::core::primitive::u32 = 4_u32;

/// button 3 is pressed
pub const SDL_PEN_INPUT_BUTTON_3: ::core::primitive::u32 = 8_u32;

/// button 4 is pressed
pub const SDL_PEN_INPUT_BUTTON_4: ::core::primitive::u32 = 16_u32;

/// button 5 is pressed
pub const SDL_PEN_INPUT_BUTTON_5: ::core::primitive::u32 = 32_u32;

/// eraser tip is used
pub const SDL_PEN_INPUT_ERASER_TIP: ::core::primitive::u32 = 1073741824_u32;

/// Pen axis indices.
///
/// These are the valid values for the `axis` field in SDL_PenAxisEvent. All
/// axes are either normalised to 0..1 or report a (positive or negative) angle
/// in degrees, with 0.0 representing the centre. Not all pens/backends support
/// all axes: unsupported axes are always zero.
///
/// To convert angles for tilt and rotation into vector representation, use
/// SDL_sinf on the XTILT, YTILT, or ROTATION component, for example:
///
/// `SDL_sinf(xtilt * SDL_PI_F / 180.0)`.
///
/// \since This enum is available since SDL 3.0.0
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PEN_AXIS_PRESSURE`], [`SDL_PEN_AXIS_XTILT`], [`SDL_PEN_AXIS_YTILT`], [`SDL_PEN_AXIS_DISTANCE`], [`SDL_PEN_AXIS_ROTATION`], [`SDL_PEN_AXIS_SLIDER`], [`SDL_PEN_AXIS_TANGENTIAL_PRESSURE`], [`SDL_PEN_NUM_AXES`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenAxis(pub ::core::ffi::c_int);
impl SDL_PenAxis {
    /// Pen pressure.  Unidirectional: 0 to 1.0
    pub const AXIS_PRESSURE: Self = Self(0);
    /// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
    ///                                  The physical max/min tilt may be smaller than -90.0 / 90.0, check SDL_PenCapabilityInfo
    pub const AXIS_XTILT: Self = Self(1);
    /// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
    ///                                  The physical max/min tilt may be smaller than -90.0 / 90.0 check SDL_PenCapabilityInfo
    pub const AXIS_YTILT: Self = Self(2);
    /// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
    pub const AXIS_DISTANCE: Self = Self(3);
    /// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).
    pub const AXIS_ROTATION: Self = Self(4);
    /// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
    pub const AXIS_SLIDER: Self = Self(5);
    /// Pressure from squeezing the pen ("barrel pressure").
    pub const AXIS_TANGENTIAL_PRESSURE: Self = Self(6);
    /// Total known pen axis types in this version of SDL. This number may grow in future releases!
    pub const NUM_AXES: Self = Self(7);
}
/// Pen pressure.  Unidirectional: 0 to 1.0
pub const SDL_PEN_AXIS_PRESSURE: SDL_PenAxis = SDL_PenAxis::AXIS_PRESSURE;
/// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
///                                  The physical max/min tilt may be smaller than -90.0 / 90.0, check SDL_PenCapabilityInfo
pub const SDL_PEN_AXIS_XTILT: SDL_PenAxis = SDL_PenAxis::AXIS_XTILT;
/// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
///                                  The physical max/min tilt may be smaller than -90.0 / 90.0 check SDL_PenCapabilityInfo
pub const SDL_PEN_AXIS_YTILT: SDL_PenAxis = SDL_PenAxis::AXIS_YTILT;
/// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
pub const SDL_PEN_AXIS_DISTANCE: SDL_PenAxis = SDL_PenAxis::AXIS_DISTANCE;
/// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).
pub const SDL_PEN_AXIS_ROTATION: SDL_PenAxis = SDL_PenAxis::AXIS_ROTATION;
/// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
pub const SDL_PEN_AXIS_SLIDER: SDL_PenAxis = SDL_PenAxis::AXIS_SLIDER;
/// Pressure from squeezing the pen ("barrel pressure").
pub const SDL_PEN_AXIS_TANGENTIAL_PRESSURE: SDL_PenAxis = SDL_PenAxis::AXIS_TANGENTIAL_PRESSURE;
/// Total known pen axis types in this version of SDL. This number may grow in future releases!
pub const SDL_PEN_NUM_AXES: SDL_PenAxis = SDL_PenAxis::NUM_AXES;

