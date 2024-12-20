//! SDL pen event handling.
//!
//! SDL provides an API for pressure-sensitive pen (stylus and/or eraser)
//! handling, e.g., for input and drawing tablets or suitably equipped mobile /
//! tablet devices.
//!
//! To get started with pens, simply handle SDL_EVENT_PEN_* events. When a pen
//! starts providing input, SDL will assign it a unique [`SDL_PenID`], which will
//! remain for the life of the process, as long as the pen stays connected.
//!
//! Pens may provide more than simple touch input; they might have other axes,
//! such as pressure, tilt, rotation, etc.

use super::stdinc::*;

/// SDL pen instance IDs.
///
/// Zero is used to signify an invalid/null device.
///
/// These show up in pen events when SDL sees input from them. They remain
/// consistent as long as SDL can recognize a tool to be the same pen; but if a
/// pen physically leaves the area and returns, it might get a new ID.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_PenID = Uint32;

/// Pen input flags, as reported by various pen events' `pen_state` field.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`SDL_PEN_INPUT_DOWN`] | pen is pressed down |
/// | [`SDL_PEN_INPUT_BUTTON_1`] | button 1 is pressed |
/// | [`SDL_PEN_INPUT_BUTTON_2`] | button 2 is pressed |
/// | [`SDL_PEN_INPUT_BUTTON_3`] | button 3 is pressed |
/// | [`SDL_PEN_INPUT_BUTTON_4`] | button 4 is pressed |
/// | [`SDL_PEN_INPUT_BUTTON_5`] | button 5 is pressed |
/// | [`SDL_PEN_INPUT_ERASER_TIP`] | eraser tip is used |
pub type SDL_PenInputFlags = Uint32;

/// pen is pressed down
pub const SDL_PEN_INPUT_DOWN: SDL_PenInputFlags = ((1_u32) as SDL_PenInputFlags);

/// button 1 is pressed
pub const SDL_PEN_INPUT_BUTTON_1: SDL_PenInputFlags = ((2_u32) as SDL_PenInputFlags);

/// button 2 is pressed
pub const SDL_PEN_INPUT_BUTTON_2: SDL_PenInputFlags = ((4_u32) as SDL_PenInputFlags);

/// button 3 is pressed
pub const SDL_PEN_INPUT_BUTTON_3: SDL_PenInputFlags = ((8_u32) as SDL_PenInputFlags);

/// button 4 is pressed
pub const SDL_PEN_INPUT_BUTTON_4: SDL_PenInputFlags = ((16_u32) as SDL_PenInputFlags);

/// button 5 is pressed
pub const SDL_PEN_INPUT_BUTTON_5: SDL_PenInputFlags = ((32_u32) as SDL_PenInputFlags);

/// eraser tip is used
pub const SDL_PEN_INPUT_ERASER_TIP: SDL_PenInputFlags = ((1073741824_u32) as SDL_PenInputFlags);

/// Pen axis indices.
///
/// These are the valid values for the `axis` field in [`SDL_PenAxisEvent`]. All
/// axes are either normalised to 0..1 or report a (positive or negative) angle
/// in degrees, with 0.0 representing the centre. Not all pens/backends support
/// all axes: unsupported axes are always zero.
///
/// To convert angles for tilt and rotation into vector representation, use
/// [`SDL_sinf`] on the XTILT, YTILT, or ROTATION component, for example:
///
/// `SDL_sinf(xtilt * SDL_PI_F / 180.0)`.
///
/// ### Availability
/// This enum is available since SDL 3.1.3
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`PRESSURE`](SDL_PenAxis::PRESSURE) | [`SDL_PEN_AXIS_PRESSURE`] | Pen pressure.  Unidirectional: 0 to 1.0 |
/// | [`XTILT`](SDL_PenAxis::XTILT) | [`SDL_PEN_AXIS_XTILT`] | Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right). |
/// | [`YTILT`](SDL_PenAxis::YTILT) | [`SDL_PEN_AXIS_YTILT`] | Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down). |
/// | [`DISTANCE`](SDL_PenAxis::DISTANCE) | [`SDL_PEN_AXIS_DISTANCE`] | Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0 |
/// | [`ROTATION`](SDL_PenAxis::ROTATION) | [`SDL_PEN_AXIS_ROTATION`] | Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down). |
/// | [`SLIDER`](SDL_PenAxis::SLIDER) | [`SDL_PEN_AXIS_SLIDER`] | Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0 |
/// | [`TANGENTIAL_PRESSURE`](SDL_PenAxis::TANGENTIAL_PRESSURE) | [`SDL_PEN_AXIS_TANGENTIAL_PRESSURE`] | Pressure from squeezing the pen ("barrel pressure"). |
/// | [`COUNT`](SDL_PenAxis::COUNT) | [`SDL_PEN_AXIS_COUNT`] | Total known pen axis types in this version of SDL. This number may grow in future releases! |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PenAxis(pub ::core::ffi::c_int);

impl From<SDL_PenAxis> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PenAxis) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PenAxis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::PRESSURE => "SDL_PEN_AXIS_PRESSURE",
            Self::XTILT => "SDL_PEN_AXIS_XTILT",
            Self::YTILT => "SDL_PEN_AXIS_YTILT",
            Self::DISTANCE => "SDL_PEN_AXIS_DISTANCE",
            Self::ROTATION => "SDL_PEN_AXIS_ROTATION",
            Self::SLIDER => "SDL_PEN_AXIS_SLIDER",
            Self::TANGENTIAL_PRESSURE => "SDL_PEN_AXIS_TANGENTIAL_PRESSURE",
            Self::COUNT => "SDL_PEN_AXIS_COUNT",

            _ => return write!(f, "SDL_PenAxis({})", self.0),
        })
    }
}

impl SDL_PenAxis {
    /// Pen pressure.  Unidirectional: 0 to 1.0
    pub const PRESSURE: Self = Self(0);
    /// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
    pub const XTILT: Self = Self(1);
    /// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
    pub const YTILT: Self = Self(2);
    /// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
    pub const DISTANCE: Self = Self(3);
    /// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).
    pub const ROTATION: Self = Self(4);
    /// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
    pub const SLIDER: Self = Self(5);
    /// Pressure from squeezing the pen ("barrel pressure").
    pub const TANGENTIAL_PRESSURE: Self = Self(6);
    /// Total known pen axis types in this version of SDL. This number may grow in future releases!
    pub const COUNT: Self = Self(7);
}

/// Pen pressure.  Unidirectional: 0 to 1.0
pub const SDL_PEN_AXIS_PRESSURE: SDL_PenAxis = SDL_PenAxis::PRESSURE;
/// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
pub const SDL_PEN_AXIS_XTILT: SDL_PenAxis = SDL_PenAxis::XTILT;
/// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
pub const SDL_PEN_AXIS_YTILT: SDL_PenAxis = SDL_PenAxis::YTILT;
/// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
pub const SDL_PEN_AXIS_DISTANCE: SDL_PenAxis = SDL_PenAxis::DISTANCE;
/// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).
pub const SDL_PEN_AXIS_ROTATION: SDL_PenAxis = SDL_PenAxis::ROTATION;
/// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
pub const SDL_PEN_AXIS_SLIDER: SDL_PenAxis = SDL_PenAxis::SLIDER;
/// Pressure from squeezing the pen ("barrel pressure").
pub const SDL_PEN_AXIS_TANGENTIAL_PRESSURE: SDL_PenAxis = SDL_PenAxis::TANGENTIAL_PRESSURE;
/// Total known pen axis types in this version of SDL. This number may grow in future releases!
pub const SDL_PEN_AXIS_COUNT: SDL_PenAxis = SDL_PenAxis::COUNT;

#[cfg(doc)]
use crate::everything::*;
