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

use super::mouse::*;

use super::touch::*;

/// SDL pen instance IDs.
///
/// Zero is used to signify an invalid/null device.
///
/// These show up in pen events when SDL sees input from them. They remain
/// consistent as long as SDL can recognize a tool to be the same pen; but if a
/// pen physically leaves the area and returns, it might get a new ID.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenID(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_PenID {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_PenID> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_PenID) -> bool {
        self == &other.0
    }
}

impl From<SDL_PenID> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_PenID) -> Self {
        value.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_PenID {
    const GROUP_METADATA: &sdl3_sys::metadata::Group = &crate::metadata::pen::METADATA_SDL_PenID;
}

/// The [`SDL_MouseID`] for mouse events simulated with pen input.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub const SDL_PEN_MOUSEID: SDL_MouseID = SDL_MouseID((-2_i32 as Uint32));

/// The [`SDL_TouchID`] for touch events simulated with pen input.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub const SDL_PEN_TOUCHID: SDL_TouchID = SDL_TouchID((-2_i32 as Uint64));

/// Pen input flags, as reported by various pen events' `pen_state` field.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`DOWN`](SDL_PenInputFlags::DOWN) | [`SDL_PEN_INPUT_DOWN`] | pen is pressed down |
/// | [`BUTTON_1`](SDL_PenInputFlags::BUTTON_1) | [`SDL_PEN_INPUT_BUTTON_1`] | button 1 is pressed |
/// | [`BUTTON_2`](SDL_PenInputFlags::BUTTON_2) | [`SDL_PEN_INPUT_BUTTON_2`] | button 2 is pressed |
/// | [`BUTTON_3`](SDL_PenInputFlags::BUTTON_3) | [`SDL_PEN_INPUT_BUTTON_3`] | button 3 is pressed |
/// | [`BUTTON_4`](SDL_PenInputFlags::BUTTON_4) | [`SDL_PEN_INPUT_BUTTON_4`] | button 4 is pressed |
/// | [`BUTTON_5`](SDL_PenInputFlags::BUTTON_5) | [`SDL_PEN_INPUT_BUTTON_5`] | button 5 is pressed |
/// | [`ERASER_TIP`](SDL_PenInputFlags::ERASER_TIP) | [`SDL_PEN_INPUT_ERASER_TIP`] | eraser tip is used |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_PenInputFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_PenInputFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_PenInputFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_PenInputFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_PenInputFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_PenInputFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PenInputFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_PenInputFlags(")?;
        let all_bits = all_bits | Self::DOWN.0;
        if (Self::DOWN != 0 || self.0 == 0) && *self & Self::DOWN == Self::DOWN {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "DOWN")?;
        }
        let all_bits = all_bits | Self::BUTTON_1.0;
        if (Self::BUTTON_1 != 0 || self.0 == 0) && *self & Self::BUTTON_1 == Self::BUTTON_1 {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON_1")?;
        }
        let all_bits = all_bits | Self::BUTTON_2.0;
        if (Self::BUTTON_2 != 0 || self.0 == 0) && *self & Self::BUTTON_2 == Self::BUTTON_2 {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON_2")?;
        }
        let all_bits = all_bits | Self::BUTTON_3.0;
        if (Self::BUTTON_3 != 0 || self.0 == 0) && *self & Self::BUTTON_3 == Self::BUTTON_3 {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON_3")?;
        }
        let all_bits = all_bits | Self::BUTTON_4.0;
        if (Self::BUTTON_4 != 0 || self.0 == 0) && *self & Self::BUTTON_4 == Self::BUTTON_4 {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON_4")?;
        }
        let all_bits = all_bits | Self::BUTTON_5.0;
        if (Self::BUTTON_5 != 0 || self.0 == 0) && *self & Self::BUTTON_5 == Self::BUTTON_5 {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON_5")?;
        }
        let all_bits = all_bits | Self::ERASER_TIP.0;
        if (Self::ERASER_TIP != 0 || self.0 == 0) && *self & Self::ERASER_TIP == Self::ERASER_TIP {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "ERASER_TIP")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_PenInputFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_PenInputFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_PenInputFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_PenInputFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_PenInputFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_PenInputFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_PenInputFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_PenInputFlags {
    /// pen is pressed down
    pub const DOWN: Self = Self((1_u32 as Uint32));
    /// button 1 is pressed
    pub const BUTTON_1: Self = Self((2_u32 as Uint32));
    /// button 2 is pressed
    pub const BUTTON_2: Self = Self((4_u32 as Uint32));
    /// button 3 is pressed
    pub const BUTTON_3: Self = Self((8_u32 as Uint32));
    /// button 4 is pressed
    pub const BUTTON_4: Self = Self((16_u32 as Uint32));
    /// button 5 is pressed
    pub const BUTTON_5: Self = Self((32_u32 as Uint32));
    /// eraser tip is used
    pub const ERASER_TIP: Self = Self((1073741824_u32 as Uint32));
}

/// pen is pressed down
pub const SDL_PEN_INPUT_DOWN: SDL_PenInputFlags = SDL_PenInputFlags::DOWN;
/// button 1 is pressed
pub const SDL_PEN_INPUT_BUTTON_1: SDL_PenInputFlags = SDL_PenInputFlags::BUTTON_1;
/// button 2 is pressed
pub const SDL_PEN_INPUT_BUTTON_2: SDL_PenInputFlags = SDL_PenInputFlags::BUTTON_2;
/// button 3 is pressed
pub const SDL_PEN_INPUT_BUTTON_3: SDL_PenInputFlags = SDL_PenInputFlags::BUTTON_3;
/// button 4 is pressed
pub const SDL_PEN_INPUT_BUTTON_4: SDL_PenInputFlags = SDL_PenInputFlags::BUTTON_4;
/// button 5 is pressed
pub const SDL_PEN_INPUT_BUTTON_5: SDL_PenInputFlags = SDL_PenInputFlags::BUTTON_5;
/// eraser tip is used
pub const SDL_PEN_INPUT_ERASER_TIP: SDL_PenInputFlags = SDL_PenInputFlags::ERASER_TIP;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_PenInputFlags {
    const GROUP_METADATA: &sdl3_sys::metadata::Group =
        &crate::metadata::pen::METADATA_SDL_PenInputFlags;
}

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
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
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
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PenAxis(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_PenAxis {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_PenAxis> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_PenAxis) -> bool {
        self == &other.0
    }
}

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
    pub const PRESSURE: Self = Self((0 as ::core::ffi::c_int));
    /// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
    pub const XTILT: Self = Self((1 as ::core::ffi::c_int));
    /// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
    pub const YTILT: Self = Self((2 as ::core::ffi::c_int));
    /// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
    pub const DISTANCE: Self = Self((3 as ::core::ffi::c_int));
    /// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).
    pub const ROTATION: Self = Self((4 as ::core::ffi::c_int));
    /// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
    pub const SLIDER: Self = Self((5 as ::core::ffi::c_int));
    /// Pressure from squeezing the pen ("barrel pressure").
    pub const TANGENTIAL_PRESSURE: Self = Self((6 as ::core::ffi::c_int));
    /// Total known pen axis types in this version of SDL. This number may grow in future releases!
    pub const COUNT: Self = Self((7 as ::core::ffi::c_int));
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

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_PenAxis {
    const GROUP_METADATA: &sdl3_sys::metadata::Group = &crate::metadata::pen::METADATA_SDL_PenAxis;
}

#[cfg(doc)]
use crate::everything::*;
