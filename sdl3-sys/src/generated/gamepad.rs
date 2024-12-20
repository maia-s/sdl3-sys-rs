//! SDL provides a low-level joystick API, which just treats joysticks as an
//! arbitrary pile of buttons, axes, and hat switches. If you're planning to
//! write your own control configuration screen, this can give you a lot of
//! flexibility, but that's a lot of work, and most things that we consider
//! "joysticks" now are actually console-style gamepads. So SDL provides the
//! gamepad API on top of the lower-level joystick functionality.
//!
//! The difference betweena joystick and a gamepad is that a gamepad tells you
//! _where_ a button or axis is on the device. You don't speak to gamepads in
//! terms of arbitrary numbers like "button 3" or "axis 2" but in standard
//! locations: the d-pad, the shoulder buttons, triggers, A/B/X/Y (or
//! X/O/Square/Triangle, if you will).
//!
//! One turns a joystick into a gamepad by providing a magic configuration
//! string, which tells SDL the details of a specific device: when you see this
//! specific hardware, if button 2 gets pressed, this is actually D-Pad Up,
//! etc.
//!
//! SDL has many popular controllers configured out of the box, and users can
//! add their own controller details through an environment variable if it's
//! otherwise unknown to SDL.
//!
//! In order to use these functions, [`SDL_Init()`] must have been called with the
//! [`SDL_INIT_GAMEPAD`] flag. This causes SDL to scan the system for gamepads, and
//! load appropriate drivers.
//!
//! If you would like to receive gamepad updates while the application is in
//! the background, you should set the following hint before calling
//! [`SDL_Init()`]: [`SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS`]

use super::stdinc::*;

use super::error::*;

use super::guid::*;

use super::iostream::*;

use super::joystick::*;

use super::power::*;

use super::properties::*;

use super::sensor::*;

/// Standard gamepad types.
///
/// This type does not necessarily map to first-party controllers from
/// Microsoft/Sony/Nintendo; in many cases, third-party controllers can report
/// as these, either because they were designed for a specific console, or they
/// simply most closely match that console's controllers (does it have A/B/X/Y
/// buttons or X/O/Square/Triangle? Does it have a touchpad? etc).
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_GamepadType::UNKNOWN) | [`SDL_GAMEPAD_TYPE_UNKNOWN`] | |
/// | [`STANDARD`](SDL_GamepadType::STANDARD) | [`SDL_GAMEPAD_TYPE_STANDARD`] | |
/// | [`XBOX360`](SDL_GamepadType::XBOX360) | [`SDL_GAMEPAD_TYPE_XBOX360`] | |
/// | [`XBOXONE`](SDL_GamepadType::XBOXONE) | [`SDL_GAMEPAD_TYPE_XBOXONE`] | |
/// | [`PS3`](SDL_GamepadType::PS3) | [`SDL_GAMEPAD_TYPE_PS3`] | |
/// | [`PS4`](SDL_GamepadType::PS4) | [`SDL_GAMEPAD_TYPE_PS4`] | |
/// | [`PS5`](SDL_GamepadType::PS5) | [`SDL_GAMEPAD_TYPE_PS5`] | |
/// | [`NINTENDO_SWITCH_PRO`](SDL_GamepadType::NINTENDO_SWITCH_PRO) | [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO`] | |
/// | [`NINTENDO_SWITCH_JOYCON_LEFT`](SDL_GamepadType::NINTENDO_SWITCH_JOYCON_LEFT) | [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT`] | |
/// | [`NINTENDO_SWITCH_JOYCON_RIGHT`](SDL_GamepadType::NINTENDO_SWITCH_JOYCON_RIGHT) | [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT`] | |
/// | [`NINTENDO_SWITCH_JOYCON_PAIR`](SDL_GamepadType::NINTENDO_SWITCH_JOYCON_PAIR) | [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR`] | |
/// | [`COUNT`](SDL_GamepadType::COUNT) | [`SDL_GAMEPAD_TYPE_COUNT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GamepadType(pub ::core::ffi::c_int);

impl From<SDL_GamepadType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GamepadType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_GAMEPAD_TYPE_UNKNOWN",
            Self::STANDARD => "SDL_GAMEPAD_TYPE_STANDARD",
            Self::XBOX360 => "SDL_GAMEPAD_TYPE_XBOX360",
            Self::XBOXONE => "SDL_GAMEPAD_TYPE_XBOXONE",
            Self::PS3 => "SDL_GAMEPAD_TYPE_PS3",
            Self::PS4 => "SDL_GAMEPAD_TYPE_PS4",
            Self::PS5 => "SDL_GAMEPAD_TYPE_PS5",
            Self::NINTENDO_SWITCH_PRO => "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO",
            Self::NINTENDO_SWITCH_JOYCON_LEFT => "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT",
            Self::NINTENDO_SWITCH_JOYCON_RIGHT => "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT",
            Self::NINTENDO_SWITCH_JOYCON_PAIR => "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR",
            Self::COUNT => "SDL_GAMEPAD_TYPE_COUNT",

            _ => return write!(f, "SDL_GamepadType({})", self.0),
        })
    }
}

impl SDL_GamepadType {
    pub const UNKNOWN: Self = Self(0);
    pub const STANDARD: Self = Self(1);
    pub const XBOX360: Self = Self(2);
    pub const XBOXONE: Self = Self(3);
    pub const PS3: Self = Self(4);
    pub const PS4: Self = Self(5);
    pub const PS5: Self = Self(6);
    pub const NINTENDO_SWITCH_PRO: Self = Self(7);
    pub const NINTENDO_SWITCH_JOYCON_LEFT: Self = Self(8);
    pub const NINTENDO_SWITCH_JOYCON_RIGHT: Self = Self(9);
    pub const NINTENDO_SWITCH_JOYCON_PAIR: Self = Self(10);
    pub const COUNT: Self = Self(11);
}

pub const SDL_GAMEPAD_TYPE_UNKNOWN: SDL_GamepadType = SDL_GamepadType::UNKNOWN;
pub const SDL_GAMEPAD_TYPE_STANDARD: SDL_GamepadType = SDL_GamepadType::STANDARD;
pub const SDL_GAMEPAD_TYPE_XBOX360: SDL_GamepadType = SDL_GamepadType::XBOX360;
pub const SDL_GAMEPAD_TYPE_XBOXONE: SDL_GamepadType = SDL_GamepadType::XBOXONE;
pub const SDL_GAMEPAD_TYPE_PS3: SDL_GamepadType = SDL_GamepadType::PS3;
pub const SDL_GAMEPAD_TYPE_PS4: SDL_GamepadType = SDL_GamepadType::PS4;
pub const SDL_GAMEPAD_TYPE_PS5: SDL_GamepadType = SDL_GamepadType::PS5;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO: SDL_GamepadType =
    SDL_GamepadType::NINTENDO_SWITCH_PRO;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT: SDL_GamepadType =
    SDL_GamepadType::NINTENDO_SWITCH_JOYCON_LEFT;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT: SDL_GamepadType =
    SDL_GamepadType::NINTENDO_SWITCH_JOYCON_RIGHT;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR: SDL_GamepadType =
    SDL_GamepadType::NINTENDO_SWITCH_JOYCON_PAIR;
pub const SDL_GAMEPAD_TYPE_COUNT: SDL_GamepadType = SDL_GamepadType::COUNT;

/// The list of buttons available on a gamepad
///
/// For controllers that use a diamond pattern for the face buttons, the
/// south/east/west/north buttons below correspond to the locations in the
/// diamond pattern. For Xbox controllers, this would be A/B/X/Y, for Nintendo
/// Switch controllers, this would be B/A/Y/X, for PlayStation controllers this
/// would be Cross/Circle/Square/Triangle.
///
/// For controllers that don't use a diamond pattern for the face buttons, the
/// south/east/west/north buttons indicate the buttons labeled A, B, C, D, or
/// 1, 2, 3, 4, or for controllers that aren't labeled, they are the primary,
/// secondary, etc. buttons.
///
/// The activate action is often the south button and the cancel action is
/// often the east button, but in some regions this is reversed, so your game
/// should allow remapping actions based on user preferences.
///
/// You can query the labels for the face buttons using
/// [`SDL_GetGamepadButtonLabel()`]
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GamepadButton::INVALID) | [`SDL_GAMEPAD_BUTTON_INVALID`] | |
/// | [`SOUTH`](SDL_GamepadButton::SOUTH) | [`SDL_GAMEPAD_BUTTON_SOUTH`] | Bottom face button (e.g. Xbox A button) |
/// | [`EAST`](SDL_GamepadButton::EAST) | [`SDL_GAMEPAD_BUTTON_EAST`] | Right face button (e.g. Xbox B button) |
/// | [`WEST`](SDL_GamepadButton::WEST) | [`SDL_GAMEPAD_BUTTON_WEST`] | Left face button (e.g. Xbox X button) |
/// | [`NORTH`](SDL_GamepadButton::NORTH) | [`SDL_GAMEPAD_BUTTON_NORTH`] | Top face button (e.g. Xbox Y button) |
/// | [`BACK`](SDL_GamepadButton::BACK) | [`SDL_GAMEPAD_BUTTON_BACK`] | |
/// | [`GUIDE`](SDL_GamepadButton::GUIDE) | [`SDL_GAMEPAD_BUTTON_GUIDE`] | |
/// | [`START`](SDL_GamepadButton::START) | [`SDL_GAMEPAD_BUTTON_START`] | |
/// | [`LEFT_STICK`](SDL_GamepadButton::LEFT_STICK) | [`SDL_GAMEPAD_BUTTON_LEFT_STICK`] | |
/// | [`RIGHT_STICK`](SDL_GamepadButton::RIGHT_STICK) | [`SDL_GAMEPAD_BUTTON_RIGHT_STICK`] | |
/// | [`LEFT_SHOULDER`](SDL_GamepadButton::LEFT_SHOULDER) | [`SDL_GAMEPAD_BUTTON_LEFT_SHOULDER`] | |
/// | [`RIGHT_SHOULDER`](SDL_GamepadButton::RIGHT_SHOULDER) | [`SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER`] | |
/// | [`DPAD_UP`](SDL_GamepadButton::DPAD_UP) | [`SDL_GAMEPAD_BUTTON_DPAD_UP`] | |
/// | [`DPAD_DOWN`](SDL_GamepadButton::DPAD_DOWN) | [`SDL_GAMEPAD_BUTTON_DPAD_DOWN`] | |
/// | [`DPAD_LEFT`](SDL_GamepadButton::DPAD_LEFT) | [`SDL_GAMEPAD_BUTTON_DPAD_LEFT`] | |
/// | [`DPAD_RIGHT`](SDL_GamepadButton::DPAD_RIGHT) | [`SDL_GAMEPAD_BUTTON_DPAD_RIGHT`] | |
/// | [`MISC1`](SDL_GamepadButton::MISC1) | [`SDL_GAMEPAD_BUTTON_MISC1`] | Additional button (e.g. Xbox Series X share button, PS5 microphone button, Nintendo Switch Pro capture button, Amazon Luna microphone button, Google Stadia capture button) |
/// | [`RIGHT_PADDLE1`](SDL_GamepadButton::RIGHT_PADDLE1) | [`SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1`] | Upper or primary paddle, under your right hand (e.g. Xbox Elite paddle P1) |
/// | [`LEFT_PADDLE1`](SDL_GamepadButton::LEFT_PADDLE1) | [`SDL_GAMEPAD_BUTTON_LEFT_PADDLE1`] | Upper or primary paddle, under your left hand (e.g. Xbox Elite paddle P3) |
/// | [`RIGHT_PADDLE2`](SDL_GamepadButton::RIGHT_PADDLE2) | [`SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2`] | Lower or secondary paddle, under your right hand (e.g. Xbox Elite paddle P2) |
/// | [`LEFT_PADDLE2`](SDL_GamepadButton::LEFT_PADDLE2) | [`SDL_GAMEPAD_BUTTON_LEFT_PADDLE2`] | Lower or secondary paddle, under your left hand (e.g. Xbox Elite paddle P4) |
/// | [`TOUCHPAD`](SDL_GamepadButton::TOUCHPAD) | [`SDL_GAMEPAD_BUTTON_TOUCHPAD`] | PS4/PS5 touchpad button |
/// | [`MISC2`](SDL_GamepadButton::MISC2) | [`SDL_GAMEPAD_BUTTON_MISC2`] | Additional button |
/// | [`MISC3`](SDL_GamepadButton::MISC3) | [`SDL_GAMEPAD_BUTTON_MISC3`] | Additional button |
/// | [`MISC4`](SDL_GamepadButton::MISC4) | [`SDL_GAMEPAD_BUTTON_MISC4`] | Additional button |
/// | [`MISC5`](SDL_GamepadButton::MISC5) | [`SDL_GAMEPAD_BUTTON_MISC5`] | Additional button |
/// | [`MISC6`](SDL_GamepadButton::MISC6) | [`SDL_GAMEPAD_BUTTON_MISC6`] | Additional button |
/// | [`COUNT`](SDL_GamepadButton::COUNT) | [`SDL_GAMEPAD_BUTTON_COUNT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GamepadButton(pub ::core::ffi::c_int);

impl From<SDL_GamepadButton> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GamepadButton) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GAMEPAD_BUTTON_INVALID",
            Self::SOUTH => "SDL_GAMEPAD_BUTTON_SOUTH",
            Self::EAST => "SDL_GAMEPAD_BUTTON_EAST",
            Self::WEST => "SDL_GAMEPAD_BUTTON_WEST",
            Self::NORTH => "SDL_GAMEPAD_BUTTON_NORTH",
            Self::BACK => "SDL_GAMEPAD_BUTTON_BACK",
            Self::GUIDE => "SDL_GAMEPAD_BUTTON_GUIDE",
            Self::START => "SDL_GAMEPAD_BUTTON_START",
            Self::LEFT_STICK => "SDL_GAMEPAD_BUTTON_LEFT_STICK",
            Self::RIGHT_STICK => "SDL_GAMEPAD_BUTTON_RIGHT_STICK",
            Self::LEFT_SHOULDER => "SDL_GAMEPAD_BUTTON_LEFT_SHOULDER",
            Self::RIGHT_SHOULDER => "SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER",
            Self::DPAD_UP => "SDL_GAMEPAD_BUTTON_DPAD_UP",
            Self::DPAD_DOWN => "SDL_GAMEPAD_BUTTON_DPAD_DOWN",
            Self::DPAD_LEFT => "SDL_GAMEPAD_BUTTON_DPAD_LEFT",
            Self::DPAD_RIGHT => "SDL_GAMEPAD_BUTTON_DPAD_RIGHT",
            Self::MISC1 => "SDL_GAMEPAD_BUTTON_MISC1",
            Self::RIGHT_PADDLE1 => "SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1",
            Self::LEFT_PADDLE1 => "SDL_GAMEPAD_BUTTON_LEFT_PADDLE1",
            Self::RIGHT_PADDLE2 => "SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2",
            Self::LEFT_PADDLE2 => "SDL_GAMEPAD_BUTTON_LEFT_PADDLE2",
            Self::TOUCHPAD => "SDL_GAMEPAD_BUTTON_TOUCHPAD",
            Self::MISC2 => "SDL_GAMEPAD_BUTTON_MISC2",
            Self::MISC3 => "SDL_GAMEPAD_BUTTON_MISC3",
            Self::MISC4 => "SDL_GAMEPAD_BUTTON_MISC4",
            Self::MISC5 => "SDL_GAMEPAD_BUTTON_MISC5",
            Self::MISC6 => "SDL_GAMEPAD_BUTTON_MISC6",
            Self::COUNT => "SDL_GAMEPAD_BUTTON_COUNT",

            _ => return write!(f, "SDL_GamepadButton({})", self.0),
        })
    }
}

impl SDL_GamepadButton {
    pub const INVALID: Self = Self(-1_i32);
    /// Bottom face button (e.g. Xbox A button)
    pub const SOUTH: Self = Self(0_i32);
    /// Right face button (e.g. Xbox B button)
    pub const EAST: Self = Self(1_i32);
    /// Left face button (e.g. Xbox X button)
    pub const WEST: Self = Self(2_i32);
    /// Top face button (e.g. Xbox Y button)
    pub const NORTH: Self = Self(3_i32);
    pub const BACK: Self = Self(4_i32);
    pub const GUIDE: Self = Self(5_i32);
    pub const START: Self = Self(6_i32);
    pub const LEFT_STICK: Self = Self(7_i32);
    pub const RIGHT_STICK: Self = Self(8_i32);
    pub const LEFT_SHOULDER: Self = Self(9_i32);
    pub const RIGHT_SHOULDER: Self = Self(10_i32);
    pub const DPAD_UP: Self = Self(11_i32);
    pub const DPAD_DOWN: Self = Self(12_i32);
    pub const DPAD_LEFT: Self = Self(13_i32);
    pub const DPAD_RIGHT: Self = Self(14_i32);
    /// Additional button (e.g. Xbox Series X share button, PS5 microphone button, Nintendo Switch Pro capture button, Amazon Luna microphone button, Google Stadia capture button)
    pub const MISC1: Self = Self(15_i32);
    /// Upper or primary paddle, under your right hand (e.g. Xbox Elite paddle P1)
    pub const RIGHT_PADDLE1: Self = Self(16_i32);
    /// Upper or primary paddle, under your left hand (e.g. Xbox Elite paddle P3)
    pub const LEFT_PADDLE1: Self = Self(17_i32);
    /// Lower or secondary paddle, under your right hand (e.g. Xbox Elite paddle P2)
    pub const RIGHT_PADDLE2: Self = Self(18_i32);
    /// Lower or secondary paddle, under your left hand (e.g. Xbox Elite paddle P4)
    pub const LEFT_PADDLE2: Self = Self(19_i32);
    /// PS4/PS5 touchpad button
    pub const TOUCHPAD: Self = Self(20_i32);
    /// Additional button
    pub const MISC2: Self = Self(21_i32);
    /// Additional button
    pub const MISC3: Self = Self(22_i32);
    /// Additional button
    pub const MISC4: Self = Self(23_i32);
    /// Additional button
    pub const MISC5: Self = Self(24_i32);
    /// Additional button
    pub const MISC6: Self = Self(25_i32);
    pub const COUNT: Self = Self(26_i32);
}

pub const SDL_GAMEPAD_BUTTON_INVALID: SDL_GamepadButton = SDL_GamepadButton::INVALID;
/// Bottom face button (e.g. Xbox A button)
pub const SDL_GAMEPAD_BUTTON_SOUTH: SDL_GamepadButton = SDL_GamepadButton::SOUTH;
/// Right face button (e.g. Xbox B button)
pub const SDL_GAMEPAD_BUTTON_EAST: SDL_GamepadButton = SDL_GamepadButton::EAST;
/// Left face button (e.g. Xbox X button)
pub const SDL_GAMEPAD_BUTTON_WEST: SDL_GamepadButton = SDL_GamepadButton::WEST;
/// Top face button (e.g. Xbox Y button)
pub const SDL_GAMEPAD_BUTTON_NORTH: SDL_GamepadButton = SDL_GamepadButton::NORTH;
pub const SDL_GAMEPAD_BUTTON_BACK: SDL_GamepadButton = SDL_GamepadButton::BACK;
pub const SDL_GAMEPAD_BUTTON_GUIDE: SDL_GamepadButton = SDL_GamepadButton::GUIDE;
pub const SDL_GAMEPAD_BUTTON_START: SDL_GamepadButton = SDL_GamepadButton::START;
pub const SDL_GAMEPAD_BUTTON_LEFT_STICK: SDL_GamepadButton = SDL_GamepadButton::LEFT_STICK;
pub const SDL_GAMEPAD_BUTTON_RIGHT_STICK: SDL_GamepadButton = SDL_GamepadButton::RIGHT_STICK;
pub const SDL_GAMEPAD_BUTTON_LEFT_SHOULDER: SDL_GamepadButton = SDL_GamepadButton::LEFT_SHOULDER;
pub const SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER: SDL_GamepadButton = SDL_GamepadButton::RIGHT_SHOULDER;
pub const SDL_GAMEPAD_BUTTON_DPAD_UP: SDL_GamepadButton = SDL_GamepadButton::DPAD_UP;
pub const SDL_GAMEPAD_BUTTON_DPAD_DOWN: SDL_GamepadButton = SDL_GamepadButton::DPAD_DOWN;
pub const SDL_GAMEPAD_BUTTON_DPAD_LEFT: SDL_GamepadButton = SDL_GamepadButton::DPAD_LEFT;
pub const SDL_GAMEPAD_BUTTON_DPAD_RIGHT: SDL_GamepadButton = SDL_GamepadButton::DPAD_RIGHT;
/// Additional button (e.g. Xbox Series X share button, PS5 microphone button, Nintendo Switch Pro capture button, Amazon Luna microphone button, Google Stadia capture button)
pub const SDL_GAMEPAD_BUTTON_MISC1: SDL_GamepadButton = SDL_GamepadButton::MISC1;
/// Upper or primary paddle, under your right hand (e.g. Xbox Elite paddle P1)
pub const SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1: SDL_GamepadButton = SDL_GamepadButton::RIGHT_PADDLE1;
/// Upper or primary paddle, under your left hand (e.g. Xbox Elite paddle P3)
pub const SDL_GAMEPAD_BUTTON_LEFT_PADDLE1: SDL_GamepadButton = SDL_GamepadButton::LEFT_PADDLE1;
/// Lower or secondary paddle, under your right hand (e.g. Xbox Elite paddle P2)
pub const SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2: SDL_GamepadButton = SDL_GamepadButton::RIGHT_PADDLE2;
/// Lower or secondary paddle, under your left hand (e.g. Xbox Elite paddle P4)
pub const SDL_GAMEPAD_BUTTON_LEFT_PADDLE2: SDL_GamepadButton = SDL_GamepadButton::LEFT_PADDLE2;
/// PS4/PS5 touchpad button
pub const SDL_GAMEPAD_BUTTON_TOUCHPAD: SDL_GamepadButton = SDL_GamepadButton::TOUCHPAD;
/// Additional button
pub const SDL_GAMEPAD_BUTTON_MISC2: SDL_GamepadButton = SDL_GamepadButton::MISC2;
/// Additional button
pub const SDL_GAMEPAD_BUTTON_MISC3: SDL_GamepadButton = SDL_GamepadButton::MISC3;
/// Additional button
pub const SDL_GAMEPAD_BUTTON_MISC4: SDL_GamepadButton = SDL_GamepadButton::MISC4;
/// Additional button
pub const SDL_GAMEPAD_BUTTON_MISC5: SDL_GamepadButton = SDL_GamepadButton::MISC5;
/// Additional button
pub const SDL_GAMEPAD_BUTTON_MISC6: SDL_GamepadButton = SDL_GamepadButton::MISC6;
pub const SDL_GAMEPAD_BUTTON_COUNT: SDL_GamepadButton = SDL_GamepadButton::COUNT;

/// The set of gamepad button labels
///
/// This isn't a complete set, just the face buttons to make it easy to show
/// button prompts.
///
/// For a complete set, you should look at the button and gamepad type and have
/// a set of symbols that work well with your art style.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_GamepadButtonLabel::UNKNOWN) | [`SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN`] | |
/// | [`A`](SDL_GamepadButtonLabel::A) | [`SDL_GAMEPAD_BUTTON_LABEL_A`] | |
/// | [`B`](SDL_GamepadButtonLabel::B) | [`SDL_GAMEPAD_BUTTON_LABEL_B`] | |
/// | [`X`](SDL_GamepadButtonLabel::X) | [`SDL_GAMEPAD_BUTTON_LABEL_X`] | |
/// | [`Y`](SDL_GamepadButtonLabel::Y) | [`SDL_GAMEPAD_BUTTON_LABEL_Y`] | |
/// | [`CROSS`](SDL_GamepadButtonLabel::CROSS) | [`SDL_GAMEPAD_BUTTON_LABEL_CROSS`] | |
/// | [`CIRCLE`](SDL_GamepadButtonLabel::CIRCLE) | [`SDL_GAMEPAD_BUTTON_LABEL_CIRCLE`] | |
/// | [`SQUARE`](SDL_GamepadButtonLabel::SQUARE) | [`SDL_GAMEPAD_BUTTON_LABEL_SQUARE`] | |
/// | [`TRIANGLE`](SDL_GamepadButtonLabel::TRIANGLE) | [`SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GamepadButtonLabel(pub ::core::ffi::c_int);

impl From<SDL_GamepadButtonLabel> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GamepadButtonLabel) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadButtonLabel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN",
            Self::A => "SDL_GAMEPAD_BUTTON_LABEL_A",
            Self::B => "SDL_GAMEPAD_BUTTON_LABEL_B",
            Self::X => "SDL_GAMEPAD_BUTTON_LABEL_X",
            Self::Y => "SDL_GAMEPAD_BUTTON_LABEL_Y",
            Self::CROSS => "SDL_GAMEPAD_BUTTON_LABEL_CROSS",
            Self::CIRCLE => "SDL_GAMEPAD_BUTTON_LABEL_CIRCLE",
            Self::SQUARE => "SDL_GAMEPAD_BUTTON_LABEL_SQUARE",
            Self::TRIANGLE => "SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE",

            _ => return write!(f, "SDL_GamepadButtonLabel({})", self.0),
        })
    }
}

impl SDL_GamepadButtonLabel {
    pub const UNKNOWN: Self = Self(0);
    pub const A: Self = Self(1);
    pub const B: Self = Self(2);
    pub const X: Self = Self(3);
    pub const Y: Self = Self(4);
    pub const CROSS: Self = Self(5);
    pub const CIRCLE: Self = Self(6);
    pub const SQUARE: Self = Self(7);
    pub const TRIANGLE: Self = Self(8);
}

pub const SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN: SDL_GamepadButtonLabel =
    SDL_GamepadButtonLabel::UNKNOWN;
pub const SDL_GAMEPAD_BUTTON_LABEL_A: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::A;
pub const SDL_GAMEPAD_BUTTON_LABEL_B: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::B;
pub const SDL_GAMEPAD_BUTTON_LABEL_X: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::X;
pub const SDL_GAMEPAD_BUTTON_LABEL_Y: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::Y;
pub const SDL_GAMEPAD_BUTTON_LABEL_CROSS: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::CROSS;
pub const SDL_GAMEPAD_BUTTON_LABEL_CIRCLE: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::CIRCLE;
pub const SDL_GAMEPAD_BUTTON_LABEL_SQUARE: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::SQUARE;
pub const SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE: SDL_GamepadButtonLabel =
    SDL_GamepadButtonLabel::TRIANGLE;

/// The list of axes available on a gamepad
///
/// Thumbstick axis values range from [`SDL_JOYSTICK_AXIS_MIN`] to
/// [`SDL_JOYSTICK_AXIS_MAX`], and are centered within ~8000 of zero, though
/// advanced UI will allow users to set or autodetect the dead zone, which
/// varies between gamepads.
///
/// Trigger axis values range from 0 (released) to [`SDL_JOYSTICK_AXIS_MAX`] (fully
/// pressed) when reported by [`SDL_GetGamepadAxis()`]. Note that this is not the
/// same range that will be reported by the lower-level [`SDL_GetJoystickAxis()`].
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GamepadAxis::INVALID) | [`SDL_GAMEPAD_AXIS_INVALID`] | |
/// | [`LEFTX`](SDL_GamepadAxis::LEFTX) | [`SDL_GAMEPAD_AXIS_LEFTX`] | |
/// | [`LEFTY`](SDL_GamepadAxis::LEFTY) | [`SDL_GAMEPAD_AXIS_LEFTY`] | |
/// | [`RIGHTX`](SDL_GamepadAxis::RIGHTX) | [`SDL_GAMEPAD_AXIS_RIGHTX`] | |
/// | [`RIGHTY`](SDL_GamepadAxis::RIGHTY) | [`SDL_GAMEPAD_AXIS_RIGHTY`] | |
/// | [`LEFT_TRIGGER`](SDL_GamepadAxis::LEFT_TRIGGER) | [`SDL_GAMEPAD_AXIS_LEFT_TRIGGER`] | |
/// | [`RIGHT_TRIGGER`](SDL_GamepadAxis::RIGHT_TRIGGER) | [`SDL_GAMEPAD_AXIS_RIGHT_TRIGGER`] | |
/// | [`COUNT`](SDL_GamepadAxis::COUNT) | [`SDL_GAMEPAD_AXIS_COUNT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GamepadAxis(pub ::core::ffi::c_int);

impl From<SDL_GamepadAxis> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GamepadAxis) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadAxis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GAMEPAD_AXIS_INVALID",
            Self::LEFTX => "SDL_GAMEPAD_AXIS_LEFTX",
            Self::LEFTY => "SDL_GAMEPAD_AXIS_LEFTY",
            Self::RIGHTX => "SDL_GAMEPAD_AXIS_RIGHTX",
            Self::RIGHTY => "SDL_GAMEPAD_AXIS_RIGHTY",
            Self::LEFT_TRIGGER => "SDL_GAMEPAD_AXIS_LEFT_TRIGGER",
            Self::RIGHT_TRIGGER => "SDL_GAMEPAD_AXIS_RIGHT_TRIGGER",
            Self::COUNT => "SDL_GAMEPAD_AXIS_COUNT",

            _ => return write!(f, "SDL_GamepadAxis({})", self.0),
        })
    }
}

impl SDL_GamepadAxis {
    pub const INVALID: Self = Self(-1_i32);
    pub const LEFTX: Self = Self(0_i32);
    pub const LEFTY: Self = Self(1_i32);
    pub const RIGHTX: Self = Self(2_i32);
    pub const RIGHTY: Self = Self(3_i32);
    pub const LEFT_TRIGGER: Self = Self(4_i32);
    pub const RIGHT_TRIGGER: Self = Self(5_i32);
    pub const COUNT: Self = Self(6_i32);
}

pub const SDL_GAMEPAD_AXIS_INVALID: SDL_GamepadAxis = SDL_GamepadAxis::INVALID;
pub const SDL_GAMEPAD_AXIS_LEFTX: SDL_GamepadAxis = SDL_GamepadAxis::LEFTX;
pub const SDL_GAMEPAD_AXIS_LEFTY: SDL_GamepadAxis = SDL_GamepadAxis::LEFTY;
pub const SDL_GAMEPAD_AXIS_RIGHTX: SDL_GamepadAxis = SDL_GamepadAxis::RIGHTX;
pub const SDL_GAMEPAD_AXIS_RIGHTY: SDL_GamepadAxis = SDL_GamepadAxis::RIGHTY;
pub const SDL_GAMEPAD_AXIS_LEFT_TRIGGER: SDL_GamepadAxis = SDL_GamepadAxis::LEFT_TRIGGER;
pub const SDL_GAMEPAD_AXIS_RIGHT_TRIGGER: SDL_GamepadAxis = SDL_GamepadAxis::RIGHT_TRIGGER;
pub const SDL_GAMEPAD_AXIS_COUNT: SDL_GamepadAxis = SDL_GamepadAxis::COUNT;

/// Types of gamepad control bindings.
///
/// A gamepad is a collection of bindings that map arbitrary joystick buttons,
/// axes and hat switches to specific positions on a generic console-style
/// gamepad. This enum is used as part of [`SDL_GamepadBinding`] to specify those
/// mappings.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_GamepadBindingType::NONE) | [`SDL_GAMEPAD_BINDTYPE_NONE`] | |
/// | [`BUTTON`](SDL_GamepadBindingType::BUTTON) | [`SDL_GAMEPAD_BINDTYPE_BUTTON`] | |
/// | [`AXIS`](SDL_GamepadBindingType::AXIS) | [`SDL_GAMEPAD_BINDTYPE_AXIS`] | |
/// | [`HAT`](SDL_GamepadBindingType::HAT) | [`SDL_GAMEPAD_BINDTYPE_HAT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GamepadBindingType(pub ::core::ffi::c_int);

impl From<SDL_GamepadBindingType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GamepadBindingType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadBindingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_GAMEPAD_BINDTYPE_NONE",
            Self::BUTTON => "SDL_GAMEPAD_BINDTYPE_BUTTON",
            Self::AXIS => "SDL_GAMEPAD_BINDTYPE_AXIS",
            Self::HAT => "SDL_GAMEPAD_BINDTYPE_HAT",

            _ => return write!(f, "SDL_GamepadBindingType({})", self.0),
        })
    }
}

impl SDL_GamepadBindingType {
    pub const NONE: Self = Self(0);
    pub const BUTTON: Self = Self(1);
    pub const AXIS: Self = Self(2);
    pub const HAT: Self = Self(3);
}

pub const SDL_GAMEPAD_BINDTYPE_NONE: SDL_GamepadBindingType = SDL_GamepadBindingType::NONE;
pub const SDL_GAMEPAD_BINDTYPE_BUTTON: SDL_GamepadBindingType = SDL_GamepadBindingType::BUTTON;
pub const SDL_GAMEPAD_BINDTYPE_AXIS: SDL_GamepadBindingType = SDL_GamepadBindingType::AXIS;
pub const SDL_GAMEPAD_BINDTYPE_HAT: SDL_GamepadBindingType = SDL_GamepadBindingType::HAT;

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GamepadBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut s = f.debug_struct("SDL_GamepadBinding");
        s.field("input_type", &self.input_type);
        match self.input_type {
            SDL_GamepadBindingType::BUTTON => {
                s.field("input.button", unsafe { &self.input.button });
            }
            SDL_GamepadBindingType::AXIS => {
                s.field("input.axis", unsafe { &self.input.axis });
            }
            SDL_GamepadBindingType::HAT => {
                s.field("input.hat", unsafe { &self.input.hat });
            }
            _ => (),
        }
        s.field("output_type", &self.output_type);
        match self.output_type {
            SDL_GamepadBindingType::BUTTON => {
                s.field("output.button", unsafe { &self.output.button });
            }
            SDL_GamepadBindingType::AXIS => {
                s.field("output.axis", unsafe { &self.output.axis });
            }
            _ => (),
        }
        s.finish()
    }
}
/// A mapping between one joystick input to a gamepad control.
///
/// A gamepad has a collection of several bindings, to say, for example, when
/// joystick button number 5 is pressed, that should be treated like the
/// gamepad's "start" button.
///
/// SDL has these bindings built-in for many popular controllers, and can add
/// more with a simple text string. Those strings are parsed into a collection
/// of these structs to make it easier to operate on the data.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetGamepadBindings`]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_GamepadBinding {
    pub input_type: SDL_GamepadBindingType,
    pub input: SDL_GamepadBinding__AnonUnion1,
    pub output_type: SDL_GamepadBindingType,
    pub output: SDL_GamepadBinding__AnonUnion2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_GamepadBinding__AnonUnion1 {
    pub button: ::core::ffi::c_int,
    pub axis: SDL_GamepadBinding__AnonUnion1__AnonStruct1,
    pub hat: SDL_GamepadBinding__AnonUnion1__AnonStruct2,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadBinding__AnonUnion1__AnonStruct1 {
    pub axis: ::core::ffi::c_int,
    pub axis_min: ::core::ffi::c_int,
    pub axis_max: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadBinding__AnonUnion1__AnonStruct2 {
    pub hat: ::core::ffi::c_int,
    pub hat_mask: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_GamepadBinding__AnonUnion2 {
    pub button: SDL_GamepadButton,
    pub axis: SDL_GamepadBinding__AnonUnion2__AnonStruct1,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadBinding__AnonUnion2__AnonStruct1 {
    pub axis: SDL_GamepadAxis,
    pub axis_min: ::core::ffi::c_int,
    pub axis_max: ::core::ffi::c_int,
}

extern "C" {
    /// Add support for gamepads that SDL is unaware of or change the binding of an
    /// existing gamepad.
    ///
    /// The mapping string has the format "GUID,name,mapping", where GUID is the
    /// string value from [`SDL_GUIDToString()`], name is the human readable string for
    /// the device and mappings are gamepad mappings to joystick ones. Under
    /// Windows there is a reserved GUID of "xinput" that covers all XInput
    /// devices. The mapping format for joystick is:
    ///
    /// - `bX`: a joystick button, index X
    /// - `hX.Y`: hat X with value Y
    /// - `aX`: axis X of the joystick
    ///
    /// Buttons can be used as a gamepad axes and vice versa.
    ///
    /// If a device with this GUID is already plugged in, SDL will generate an
    /// [`SDL_EVENT_GAMEPAD_ADDED`] event.
    ///
    /// This string shows an example of a valid mapping for a gamepad:
    ///
    /// ```c
    /// "341a3608000000000000504944564944,Afterglow PS3 Controller,a:b1,b:b2,y:b3,x:b0,start:b9,guide:b12,back:b8,dpup:h0.1,dpleft:h0.8,dpdown:h0.4,dpright:h0.2,leftshoulder:b4,rightshoulder:b5,leftstick:b10,rightstick:b11,leftx:a0,lefty:a1,rightx:a2,righty:a3,lefttrigger:b6,righttrigger:b7"
    /// ```
    ///
    /// ### Parameters
    /// - `mapping`: the mapping string.
    ///
    /// ### Return value
    /// Returns 1 if a new mapping is added, 0 if an existing mapping is updated,
    ///   -1 on failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddGamepadMappingsFromFile`]
    /// - [`SDL_AddGamepadMappingsFromIO`]
    /// - [`SDL_GetGamepadMapping`]
    /// - [`SDL_GetGamepadMappingForGUID`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG_FILE`]
    /// - [`SDL_EVENT_GAMEPAD_ADDED`]
    pub fn SDL_AddGamepadMapping(mapping: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

extern "C" {
    /// Load a set of gamepad mappings from an [`SDL_IOStream`].
    ///
    /// You can call this function several times, if needed, to load different
    /// database files.
    ///
    /// If a new mapping is loaded for an already known gamepad GUID, the later
    /// version will overwrite the one currently loaded.
    ///
    /// Any new mappings for already plugged in controllers will generate
    /// [`SDL_EVENT_GAMEPAD_ADDED`] events.
    ///
    /// Mappings not belonging to the current platform or with no platform field
    /// specified will be ignored (i.e. mappings for Linux will be ignored in
    /// Windows, etc).
    ///
    /// This function will load the text database entirely in memory before
    /// processing it, so take this into consideration if you are in a memory
    /// constrained environment.
    ///
    /// ### Parameters
    /// - `src`: the data stream for the mappings to be added.
    /// - `closeio`: if true, calls [`SDL_CloseIO()`] on `src` before returning, even
    ///   in the case of an error.
    ///
    /// ### Return value
    /// Returns the number of mappings added or -1 on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddGamepadMapping`]
    /// - [`SDL_AddGamepadMappingsFromFile`]
    /// - [`SDL_GetGamepadMapping`]
    /// - [`SDL_GetGamepadMappingForGUID`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG_FILE`]
    /// - [`SDL_EVENT_GAMEPAD_ADDED`]
    pub fn SDL_AddGamepadMappingsFromIO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Load a set of gamepad mappings from a file.
    ///
    /// You can call this function several times, if needed, to load different
    /// database files.
    ///
    /// If a new mapping is loaded for an already known gamepad GUID, the later
    /// version will overwrite the one currently loaded.
    ///
    /// Any new mappings for already plugged in controllers will generate
    /// [`SDL_EVENT_GAMEPAD_ADDED`] events.
    ///
    /// Mappings not belonging to the current platform or with no platform field
    /// specified will be ignored (i.e. mappings for Linux will be ignored in
    /// Windows, etc).
    ///
    /// ### Parameters
    /// - `file`: the mappings file to load.
    ///
    /// ### Return value
    /// Returns the number of mappings added or -1 on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddGamepadMapping`]
    /// - [`SDL_AddGamepadMappingsFromIO`]
    /// - [`SDL_GetGamepadMapping`]
    /// - [`SDL_GetGamepadMappingForGUID`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG`]
    /// - [`SDL_HINT_GAMECONTROLLERCONFIG_FILE`]
    /// - [`SDL_EVENT_GAMEPAD_ADDED`]
    pub fn SDL_AddGamepadMappingsFromFile(file: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

extern "C" {
    /// Reinitialize the SDL mapping database to its initial state.
    ///
    /// This will generate gamepad events as needed if device mappings change.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReloadGamepadMappings() -> ::core::primitive::bool;
}

extern "C" {
    /// Get the current gamepad mappings.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of mappings returned, can
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns an array of the mapping strings, NULL-terminated, or NULL on
    ///   failure; call [`SDL_GetError()`] for more information. This is a
    ///   single allocation that should be freed with [`SDL_free()`] when it is
    ///   no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadMappings(count: *mut ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
}

extern "C" {
    /// Get the gamepad mapping string for a given GUID.
    ///
    /// ### Parameters
    /// - `guid`: a structure containing the GUID for which a mapping is desired.
    ///
    /// ### Return value
    /// Returns a mapping string or NULL on failure; call [`SDL_GetError()`] for more
    ///   information. This should be freed with [`SDL_free()`] when it is no
    ///   longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickGUIDForID`]
    /// - [`SDL_GetJoystickGUID`]
    pub fn SDL_GetGamepadMappingForGUID(guid: SDL_GUID) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Get the current mapping of a gamepad.
    ///
    /// Details about mappings are discussed with [`SDL_AddGamepadMapping()`].
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad you want to get the current mapping for.
    ///
    /// ### Return value
    /// Returns a string that has the gamepad's mapping or NULL if no mapping is
    ///   available; call [`SDL_GetError()`] for more information. This should
    ///   be freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddGamepadMapping`]
    /// - [`SDL_GetGamepadMappingForID`]
    /// - [`SDL_GetGamepadMappingForGUID`]
    /// - [`SDL_SetGamepadMapping`]
    pub fn SDL_GetGamepadMapping(gamepad: *mut SDL_Gamepad) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Set the current mapping of a joystick or gamepad.
    ///
    /// Details about mappings are discussed with [`SDL_AddGamepadMapping()`].
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    /// - `mapping`: the mapping to use for this device, or NULL to clear the
    ///   mapping.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddGamepadMapping`]
    /// - [`SDL_GetGamepadMapping`]
    pub fn SDL_SetGamepadMapping(
        instance_id: SDL_JoystickID,
        mapping: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether a gamepad is currently connected.
    ///
    /// ### Return value
    /// Returns true if a gamepad is connected, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepads`]
    pub fn SDL_HasGamepad() -> ::core::primitive::bool;
}

extern "C" {
    /// Get a list of currently connected gamepads.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of gamepads returned, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns a 0 terminated array of joystick instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasGamepad`]
    /// - [`SDL_OpenGamepad`]
    pub fn SDL_GetGamepads(count: *mut ::core::ffi::c_int) -> *mut SDL_JoystickID;
}

extern "C" {
    /// Check if the given joystick is supported by the gamepad interface.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns true if the given joystick is supported by the gamepad interface,
    ///   false if it isn't or it's an invalid index.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoysticks`]
    /// - [`SDL_OpenGamepad`]
    pub fn SDL_IsGamepad(instance_id: SDL_JoystickID) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the implementation dependent name of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the name of the selected gamepad. If no name can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadName`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadNameForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the implementation dependent path of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the path of the selected gamepad. If no path can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadPath`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadPathForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the player index of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the player index of a gamepad, or -1 if it's not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadPlayerIndex`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadPlayerIndexForID(instance_id: SDL_JoystickID) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the implementation-dependent GUID of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the GUID of the selected gamepad. If called on an invalid index,
    ///   this function returns a zero GUID.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GUIDToString`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadGUIDForID(instance_id: SDL_JoystickID) -> SDL_GUID;
}

extern "C" {
    /// Get the USB vendor ID of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the vendor ID isn't
    /// available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the USB vendor ID of the selected gamepad. If called on an invalid
    ///   index, this function returns zero.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadVendor`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadVendorForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the USB product ID of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the product ID isn't
    /// available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the USB product ID of the selected gamepad. If called on an
    ///   invalid index, this function returns zero.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadProduct`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadProductForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the product version of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the product version
    /// isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the product version of the selected gamepad. If called on an
    ///   invalid index, this function returns zero.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadProductVersion`]
    /// - [`SDL_GetGamepads`]
    pub fn SDL_GetGamepadProductVersionForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the type of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the gamepad type.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadType`]
    /// - [`SDL_GetGamepads`]
    /// - [`SDL_GetRealGamepadTypeForID`]
    pub fn SDL_GetGamepadTypeForID(instance_id: SDL_JoystickID) -> SDL_GamepadType;
}

extern "C" {
    /// Get the type of a gamepad, ignoring any mapping override.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the gamepad type.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadTypeForID`]
    /// - [`SDL_GetGamepads`]
    /// - [`SDL_GetRealGamepadType`]
    pub fn SDL_GetRealGamepadTypeForID(instance_id: SDL_JoystickID) -> SDL_GamepadType;
}

extern "C" {
    /// Get the mapping of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the mapping string. Returns NULL if no mapping is available. This
    ///   should be freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepads`]
    /// - [`SDL_GetGamepadMapping`]
    pub fn SDL_GetGamepadMappingForID(instance_id: SDL_JoystickID) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Open a gamepad for use.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns a gamepad identifier or NULL if an error occurred; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseGamepad`]
    /// - [`SDL_IsGamepad`]
    pub fn SDL_OpenGamepad(instance_id: SDL_JoystickID) -> *mut SDL_Gamepad;
}

extern "C" {
    /// Get the [`SDL_Gamepad`] associated with a joystick instance ID, if it has been
    /// opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID of the gamepad.
    ///
    /// ### Return value
    /// Returns an [`SDL_Gamepad`] on success or NULL on failure or if it hasn't been
    ///   opened yet; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadFromID(instance_id: SDL_JoystickID) -> *mut SDL_Gamepad;
}

extern "C" {
    /// Get the [`SDL_Gamepad`] associated with a player index.
    ///
    /// ### Parameters
    /// - `player_index`: the player index, which different from the instance ID.
    ///
    /// ### Return value
    /// Returns the [`SDL_Gamepad`] associated with a player index.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadPlayerIndex`]
    /// - [`SDL_SetGamepadPlayerIndex`]
    pub fn SDL_GetGamepadFromPlayerIndex(player_index: ::core::ffi::c_int) -> *mut SDL_Gamepad;
}

extern "C" {
    /// Get the properties associated with an opened gamepad.
    ///
    /// These properties are shared with the underlying joystick object.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - [`SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN`]: true if this gamepad has an LED
    ///   that has adjustable brightness
    /// - [`SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN`]: true if this gamepad has an LED
    ///   that has adjustable color
    /// - [`SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN`]: true if this gamepad has a
    ///   player LED
    /// - [`SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN`]: true if this gamepad has
    ///   left/right rumble
    /// - [`SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN`]: true if this gamepad has
    ///   simple trigger rumble
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadProperties(gamepad: *mut SDL_Gamepad) -> SDL_PropertiesID;
}

pub const SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN: *const ::core::ffi::c_char =
    SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN;

pub const SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN: *const ::core::ffi::c_char =
    SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN;

pub const SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN: *const ::core::ffi::c_char =
    SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN;

pub const SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN: *const ::core::ffi::c_char =
    SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN;

pub const SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN: *const ::core::ffi::c_char =
    SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN;

extern "C" {
    /// Get the instance ID of an opened gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Return value
    /// Returns the instance ID of the specified gamepad on success or 0 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadID(gamepad: *mut SDL_Gamepad) -> SDL_JoystickID;
}

extern "C" {
    /// Get the implementation-dependent name for an opened gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Return value
    /// Returns the implementation dependent name for the gamepad, or NULL if
    ///   there is no name or the identifier passed is invalid.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadNameForID`]
    pub fn SDL_GetGamepadName(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the implementation-dependent path for an opened gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Return value
    /// Returns the implementation dependent path for the gamepad, or NULL if
    ///   there is no path or the identifier passed is invalid.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadPathForID`]
    pub fn SDL_GetGamepadPath(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the type of an opened gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the gamepad type, or [`SDL_GAMEPAD_TYPE_UNKNOWN`] if it's not
    ///   available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadTypeForID`]
    pub fn SDL_GetGamepadType(gamepad: *mut SDL_Gamepad) -> SDL_GamepadType;
}

extern "C" {
    /// Get the type of an opened gamepad, ignoring any mapping override.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the gamepad type, or [`SDL_GAMEPAD_TYPE_UNKNOWN`] if it's not
    ///   available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetRealGamepadTypeForID`]
    pub fn SDL_GetRealGamepadType(gamepad: *mut SDL_Gamepad) -> SDL_GamepadType;
}

extern "C" {
    /// Get the player index of an opened gamepad.
    ///
    /// For XInput gamepads this returns the XInput user index.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the player index for gamepad, or -1 if it's not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetGamepadPlayerIndex`]
    pub fn SDL_GetGamepadPlayerIndex(gamepad: *mut SDL_Gamepad) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set the player index of an opened gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to adjust.
    /// - `player_index`: player index to assign to this gamepad, or -1 to clear
    ///   the player index and turn off player LEDs.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadPlayerIndex`]
    pub fn SDL_SetGamepadPlayerIndex(
        gamepad: *mut SDL_Gamepad,
        player_index: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the USB vendor ID of an opened gamepad, if available.
    ///
    /// If the vendor ID isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the USB vendor ID, or zero if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadVendorForID`]
    pub fn SDL_GetGamepadVendor(gamepad: *mut SDL_Gamepad) -> Uint16;
}

extern "C" {
    /// Get the USB product ID of an opened gamepad, if available.
    ///
    /// If the product ID isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the USB product ID, or zero if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadProductForID`]
    pub fn SDL_GetGamepadProduct(gamepad: *mut SDL_Gamepad) -> Uint16;
}

extern "C" {
    /// Get the product version of an opened gamepad, if available.
    ///
    /// If the product version isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the USB product version, or zero if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadProductVersionForID`]
    pub fn SDL_GetGamepadProductVersion(gamepad: *mut SDL_Gamepad) -> Uint16;
}

extern "C" {
    /// Get the firmware version of an opened gamepad, if available.
    ///
    /// If the firmware version isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the gamepad firmware version, or zero if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadFirmwareVersion(gamepad: *mut SDL_Gamepad) -> Uint16;
}

extern "C" {
    /// Get the serial number of an opened gamepad, if available.
    ///
    /// Returns the serial number of the gamepad, or NULL if it is not available.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the serial number, or NULL if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadSerial(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the Steam Input handle of an opened gamepad, if available.
    ///
    /// Returns an InputHandle_t for the gamepad that can be used with Steam Input
    /// API: <https://partner.steamgames.com/doc/api/ISteamInput>
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the gamepad handle, or 0 if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadSteamHandle(gamepad: *mut SDL_Gamepad) -> Uint64;
}

extern "C" {
    /// Get the connection state of a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    ///
    /// ### Return value
    /// Returns the connection state on success or
    ///   [`SDL_JOYSTICK_CONNECTION_INVALID`] on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadConnectionState(gamepad: *mut SDL_Gamepad) -> SDL_JoystickConnectionState;
}

extern "C" {
    /// Get the battery state of a gamepad.
    ///
    /// You should never take a battery status as absolute truth. Batteries
    /// (especially failing batteries) are delicate hardware, and the values
    /// reported here are best estimates based on what that hardware reports. It's
    /// not uncommon for older batteries to lose stored power much faster than it
    /// reports, or completely drain when reporting it has 20 percent left, etc.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object to query.
    /// - `percent`: a pointer filled in with the percentage of battery life
    ///   left, between 0 and 100, or NULL to ignore. This will be
    ///   filled in with -1 we can't determine a value or there is no
    ///   battery.
    ///
    /// ### Return value
    /// Returns the current battery state.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadPowerInfo(
        gamepad: *mut SDL_Gamepad,
        percent: *mut ::core::ffi::c_int,
    ) -> SDL_PowerState;
}

extern "C" {
    /// Check if a gamepad has been opened and is currently connected.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Return value
    /// Returns true if the gamepad has been opened and is currently connected, or
    ///   false if not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GamepadConnected(gamepad: *mut SDL_Gamepad) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the underlying joystick from a gamepad.
    ///
    /// This function will give you a [`SDL_Joystick`] object, which allows you to use
    /// the [`SDL_Joystick`] functions with a [`SDL_Gamepad`] object. This would be useful
    /// for getting a joystick's position at any given time, even if it hasn't
    /// moved (moving it would produce an event, which would have the axis' value).
    ///
    /// The pointer returned is owned by the [`SDL_Gamepad`]. You should not call
    /// [`SDL_CloseJoystick()`] on it, for example, since doing so will likely cause
    /// SDL to crash.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad object that you want to get a joystick from.
    ///
    /// ### Return value
    /// Returns an [`SDL_Joystick`] object, or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadJoystick(gamepad: *mut SDL_Gamepad) -> *mut SDL_Joystick;
}

extern "C" {
    /// Set the state of gamepad event processing.
    ///
    /// If gamepad events are disabled, you must call [`SDL_UpdateGamepads()`] yourself
    /// and check the state of the gamepad when you want gamepad information.
    ///
    /// ### Parameters
    /// - `enabled`: whether to process gamepad events or not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadEventsEnabled`]
    /// - [`SDL_UpdateGamepads`]
    pub fn SDL_SetGamepadEventsEnabled(enabled: ::core::primitive::bool);
}

extern "C" {
    /// Query the state of gamepad event processing.
    ///
    /// If gamepad events are disabled, you must call [`SDL_UpdateGamepads()`] yourself
    /// and check the state of the gamepad when you want gamepad information.
    ///
    /// ### Return value
    /// Returns true if gamepad events are being processed, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetGamepadEventsEnabled`]
    pub fn SDL_GamepadEventsEnabled() -> ::core::primitive::bool;
}

extern "C" {
    /// Get the SDL joystick layer bindings for a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `count`: a pointer filled in with the number of bindings returned.
    ///
    /// ### Return value
    /// Returns a NULL terminated array of pointers to bindings or NULL on
    ///   failure; call [`SDL_GetError()`] for more information. This is a
    ///   single allocation that should be freed with [`SDL_free()`] when it is
    ///   no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadBindings(
        gamepad: *mut SDL_Gamepad,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut SDL_GamepadBinding;
}

extern "C" {
    /// Manually pump gamepad updates if not using the loop.
    ///
    /// This function is called automatically by the event loop if events are
    /// enabled. Under such circumstances, it will not be necessary to call this
    /// function.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_UpdateGamepads();
}

extern "C" {
    /// Convert a string into [`SDL_GamepadType`] enum.
    ///
    /// This function is called internally to translate [`SDL_Gamepad`] mapping strings
    /// for the underlying joystick device into the consistent [`SDL_Gamepad`] mapping.
    /// You do not normally need to call this function unless you are parsing
    /// [`SDL_Gamepad`] mappings in your own code.
    ///
    /// ### Parameters
    /// - `str`: string representing a [`SDL_GamepadType`] type.
    ///
    /// ### Return value
    /// Returns the [`SDL_GamepadType`] enum corresponding to the input string, or
    ///   [`SDL_GAMEPAD_TYPE_UNKNOWN`] if no match was found.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadStringForType`]
    pub fn SDL_GetGamepadTypeFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadType;
}

extern "C" {
    /// Convert from an [`SDL_GamepadType`] enum to a string.
    ///
    /// ### Parameters
    /// - `type`: an enum value for a given [`SDL_GamepadType`].
    ///
    /// ### Return value
    /// Returns a string for the given type, or NULL if an invalid type is
    ///   specified. The string returned is of the format used by
    ///   [`SDL_Gamepad`] mapping strings.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadTypeFromString`]
    pub fn SDL_GetGamepadStringForType(r#type: SDL_GamepadType) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Convert a string into [`SDL_GamepadAxis`] enum.
    ///
    /// This function is called internally to translate [`SDL_Gamepad`] mapping strings
    /// for the underlying joystick device into the consistent [`SDL_Gamepad`] mapping.
    /// You do not normally need to call this function unless you are parsing
    /// [`SDL_Gamepad`] mappings in your own code.
    ///
    /// Note specially that "righttrigger" and "lefttrigger" map to
    /// [`SDL_GAMEPAD_AXIS_RIGHT_TRIGGER`] and [`SDL_GAMEPAD_AXIS_LEFT_TRIGGER`],
    /// respectively.
    ///
    /// ### Parameters
    /// - `str`: string representing a [`SDL_Gamepad`] axis.
    ///
    /// ### Return value
    /// Returns the [`SDL_GamepadAxis`] enum corresponding to the input string, or
    ///   [`SDL_GAMEPAD_AXIS_INVALID`] if no match was found.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadStringForAxis`]
    pub fn SDL_GetGamepadAxisFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadAxis;
}

extern "C" {
    /// Convert from an [`SDL_GamepadAxis`] enum to a string.
    ///
    /// ### Parameters
    /// - `axis`: an enum value for a given [`SDL_GamepadAxis`].
    ///
    /// ### Return value
    /// Returns a string for the given axis, or NULL if an invalid axis is
    ///   specified. The string returned is of the format used by
    ///   [`SDL_Gamepad`] mapping strings.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadAxisFromString`]
    pub fn SDL_GetGamepadStringForAxis(axis: SDL_GamepadAxis) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query whether a gamepad has a given axis.
    ///
    /// This merely reports whether the gamepad's mapping defined this axis, as
    /// that is all the information SDL has about the physical device.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `axis`: an axis enum value (an [`SDL_GamepadAxis`] value).
    ///
    /// ### Return value
    /// Returns true if the gamepad has this axis, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadHasButton`]
    /// - [`SDL_GetGamepadAxis`]
    pub fn SDL_GamepadHasAxis(
        gamepad: *mut SDL_Gamepad,
        axis: SDL_GamepadAxis,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the current state of an axis control on a gamepad.
    ///
    /// The axis indices start at index 0.
    ///
    /// For thumbsticks, the state is a value ranging from -32768 (up/left) to
    /// 32767 (down/right).
    ///
    /// Triggers range from 0 when released to 32767 when fully pressed, and never
    /// return a negative value. Note that this differs from the value reported by
    /// the lower-level [`SDL_GetJoystickAxis()`], which normally uses the full range.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `axis`: an axis index (one of the [`SDL_GamepadAxis`] values).
    ///
    /// ### Return value
    /// Returns axis state (including 0) on success or 0 (also) on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadHasAxis`]
    /// - [`SDL_GetGamepadButton`]
    pub fn SDL_GetGamepadAxis(gamepad: *mut SDL_Gamepad, axis: SDL_GamepadAxis) -> Sint16;
}

extern "C" {
    /// Convert a string into an [`SDL_GamepadButton`] enum.
    ///
    /// This function is called internally to translate [`SDL_Gamepad`] mapping strings
    /// for the underlying joystick device into the consistent [`SDL_Gamepad`] mapping.
    /// You do not normally need to call this function unless you are parsing
    /// [`SDL_Gamepad`] mappings in your own code.
    ///
    /// ### Parameters
    /// - `str`: string representing a [`SDL_Gamepad`] axis.
    ///
    /// ### Return value
    /// Returns the [`SDL_GamepadButton`] enum corresponding to the input string, or
    ///   [`SDL_GAMEPAD_BUTTON_INVALID`] if no match was found.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadStringForButton`]
    pub fn SDL_GetGamepadButtonFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadButton;
}

extern "C" {
    /// Convert from an [`SDL_GamepadButton`] enum to a string.
    ///
    /// ### Parameters
    /// - `button`: an enum value for a given [`SDL_GamepadButton`].
    ///
    /// ### Return value
    /// Returns a string for the given button, or NULL if an invalid button is
    ///   specified. The string returned is of the format used by
    ///   [`SDL_Gamepad`] mapping strings.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadButtonFromString`]
    pub fn SDL_GetGamepadStringForButton(button: SDL_GamepadButton) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query whether a gamepad has a given button.
    ///
    /// This merely reports whether the gamepad's mapping defined this button, as
    /// that is all the information SDL has about the physical device.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `button`: a button enum value (an [`SDL_GamepadButton`] value).
    ///
    /// ### Return value
    /// Returns true if the gamepad has this button, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadHasAxis`]
    pub fn SDL_GamepadHasButton(
        gamepad: *mut SDL_Gamepad,
        button: SDL_GamepadButton,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the current state of a button on a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `button`: a button index (one of the [`SDL_GamepadButton`] values).
    ///
    /// ### Return value
    /// Returns true if the button is pressed, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadHasButton`]
    /// - [`SDL_GetGamepadAxis`]
    pub fn SDL_GetGamepadButton(
        gamepad: *mut SDL_Gamepad,
        button: SDL_GamepadButton,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the label of a button on a gamepad.
    ///
    /// ### Parameters
    /// - `type`: the type of gamepad to check.
    /// - `button`: a button index (one of the [`SDL_GamepadButton`] values).
    ///
    /// ### Return value
    /// Returns the [`SDL_GamepadButtonLabel`] enum corresponding to the button label.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadButtonLabel`]
    pub fn SDL_GetGamepadButtonLabelForType(
        r#type: SDL_GamepadType,
        button: SDL_GamepadButton,
    ) -> SDL_GamepadButtonLabel;
}

extern "C" {
    /// Get the label of a button on a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `button`: a button index (one of the [`SDL_GamepadButton`] values).
    ///
    /// ### Return value
    /// Returns the [`SDL_GamepadButtonLabel`] enum corresponding to the button label.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadButtonLabelForType`]
    pub fn SDL_GetGamepadButtonLabel(
        gamepad: *mut SDL_Gamepad,
        button: SDL_GamepadButton,
    ) -> SDL_GamepadButtonLabel;
}

extern "C" {
    /// Get the number of touchpads on a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    ///
    /// ### Return value
    /// Returns number of touchpads.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumGamepadTouchpadFingers`]
    pub fn SDL_GetNumGamepadTouchpads(gamepad: *mut SDL_Gamepad) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the number of supported simultaneous fingers on a touchpad on a game
    /// gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `touchpad`: a touchpad.
    ///
    /// ### Return value
    /// Returns number of supported simultaneous fingers.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadTouchpadFinger`]
    /// - [`SDL_GetNumGamepadTouchpads`]
    pub fn SDL_GetNumGamepadTouchpadFingers(
        gamepad: *mut SDL_Gamepad,
        touchpad: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the current state of a finger on a touchpad on a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad.
    /// - `touchpad`: a touchpad.
    /// - `finger`: a finger.
    /// - `down`: a pointer filled with true if the finger is down, false
    ///   otherwise, may be NULL.
    /// - `x`: a pointer filled with the x position, normalized 0 to 1, with the
    ///   origin in the upper left, may be NULL.
    /// - `y`: a pointer filled with the y position, normalized 0 to 1, with the
    ///   origin in the upper left, may be NULL.
    /// - `pressure`: a pointer filled with pressure value, may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumGamepadTouchpadFingers`]
    pub fn SDL_GetGamepadTouchpadFinger(
        gamepad: *mut SDL_Gamepad,
        touchpad: ::core::ffi::c_int,
        finger: ::core::ffi::c_int,
        down: *mut ::core::primitive::bool,
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
        pressure: *mut ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether a gamepad has a particular sensor.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `type`: the type of sensor to query.
    ///
    /// ### Return value
    /// Returns true if the sensor exists, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadSensorData`]
    /// - [`SDL_GetGamepadSensorDataRate`]
    /// - [`SDL_SetGamepadSensorEnabled`]
    pub fn SDL_GamepadHasSensor(
        gamepad: *mut SDL_Gamepad,
        r#type: SDL_SensorType,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set whether data reporting for a gamepad sensor is enabled.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to update.
    /// - `type`: the type of sensor to enable/disable.
    /// - `enabled`: whether data reporting should be enabled.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GamepadHasSensor`]
    /// - [`SDL_GamepadSensorEnabled`]
    pub fn SDL_SetGamepadSensorEnabled(
        gamepad: *mut SDL_Gamepad,
        r#type: SDL_SensorType,
        enabled: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether sensor data reporting is enabled for a gamepad.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `type`: the type of sensor to query.
    ///
    /// ### Return value
    /// Returns true if the sensor is enabled, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetGamepadSensorEnabled`]
    pub fn SDL_GamepadSensorEnabled(
        gamepad: *mut SDL_Gamepad,
        r#type: SDL_SensorType,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the data rate (number of events per second) of a gamepad sensor.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `type`: the type of sensor to query.
    ///
    /// ### Return value
    /// Returns the data rate, or 0.0f if the data rate is not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadSensorDataRate(
        gamepad: *mut SDL_Gamepad,
        r#type: SDL_SensorType,
    ) -> ::core::ffi::c_float;
}

extern "C" {
    /// Get the current state of a gamepad sensor.
    ///
    /// The number of values and interpretation of the data is sensor dependent.
    /// See SDL_sensor.h for the details for each type of sensor.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `type`: the type of sensor to query.
    /// - `data`: a pointer filled with the current sensor state.
    /// - `num_values`: the number of values to write to data.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGamepadSensorData(
        gamepad: *mut SDL_Gamepad,
        r#type: SDL_SensorType,
        data: *mut ::core::ffi::c_float,
        num_values: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Start a rumble effect on a gamepad.
    ///
    /// Each call to this function cancels any previous rumble effect, and calling
    /// it with 0 intensity stops any rumbling.
    ///
    /// This function requires you to process SDL events or call
    /// [`SDL_UpdateJoysticks()`] to update rumble state.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to vibrate.
    /// - `low_frequency_rumble`: the intensity of the low frequency (left)
    ///   rumble motor, from 0 to 0xFFFF.
    /// - `high_frequency_rumble`: the intensity of the high frequency (right)
    ///   rumble motor, from 0 to 0xFFFF.
    /// - `duration_ms`: the duration of the rumble effect, in milliseconds.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_RumbleGamepad(
        gamepad: *mut SDL_Gamepad,
        low_frequency_rumble: Uint16,
        high_frequency_rumble: Uint16,
        duration_ms: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Start a rumble effect in the gamepad's triggers.
    ///
    /// Each call to this function cancels any previous trigger rumble effect, and
    /// calling it with 0 intensity stops any rumbling.
    ///
    /// Note that this is rumbling of the _triggers_ and not the gamepad as a
    /// whole. This is currently only supported on Xbox One gamepads. If you want
    /// the (more common) whole-gamepad rumble, use [`SDL_RumbleGamepad()`] instead.
    ///
    /// This function requires you to process SDL events or call
    /// [`SDL_UpdateJoysticks()`] to update rumble state.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to vibrate.
    /// - `left_rumble`: the intensity of the left trigger rumble motor, from 0
    ///   to 0xFFFF.
    /// - `right_rumble`: the intensity of the right trigger rumble motor, from 0
    ///   to 0xFFFF.
    /// - `duration_ms`: the duration of the rumble effect, in milliseconds.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_RumbleGamepad`]
    pub fn SDL_RumbleGamepadTriggers(
        gamepad: *mut SDL_Gamepad,
        left_rumble: Uint16,
        right_rumble: Uint16,
        duration_ms: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Update a gamepad's LED color.
    ///
    /// An example of a joystick LED is the light on the back of a PlayStation 4's
    /// DualShock 4 controller.
    ///
    /// For gamepads with a single color LED, the maximum of the RGB values will be
    /// used as the LED brightness.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to update.
    /// - `red`: the intensity of the red LED.
    /// - `green`: the intensity of the green LED.
    /// - `blue`: the intensity of the blue LED.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetGamepadLED(
        gamepad: *mut SDL_Gamepad,
        red: Uint8,
        green: Uint8,
        blue: Uint8,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Send a gamepad specific effect packet.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to affect.
    /// - `data`: the data to send to the gamepad.
    /// - `size`: the size of the data to send to the gamepad.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SendGamepadEffect(
        gamepad: *mut SDL_Gamepad,
        data: *const ::core::ffi::c_void,
        size: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Close a gamepad previously opened with [`SDL_OpenGamepad()`].
    ///
    /// ### Parameters
    /// - `gamepad`: a gamepad identifier previously returned by
    ///   [`SDL_OpenGamepad()`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenGamepad`]
    pub fn SDL_CloseGamepad(gamepad: *mut SDL_Gamepad);
}

extern "C" {
    /// Return the sfSymbolsName for a given button on a gamepad on Apple
    /// platforms.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `button`: a button on the gamepad.
    ///
    /// ### Return value
    /// Returns the sfSymbolsName or NULL if the name can't be found.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadAppleSFSymbolsNameForAxis`]
    pub fn SDL_GetGamepadAppleSFSymbolsNameForButton(
        gamepad: *mut SDL_Gamepad,
        button: SDL_GamepadButton,
    ) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Return the sfSymbolsName for a given axis on a gamepad on Apple platforms.
    ///
    /// ### Parameters
    /// - `gamepad`: the gamepad to query.
    /// - `axis`: an axis on the gamepad.
    ///
    /// ### Return value
    /// Returns the sfSymbolsName or NULL if the name can't be found.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGamepadAppleSFSymbolsNameForButton`]
    pub fn SDL_GetGamepadAppleSFSymbolsNameForAxis(
        gamepad: *mut SDL_Gamepad,
        axis: SDL_GamepadAxis,
    ) -> *const ::core::ffi::c_char;
}

/// The structure used to identify an SDL gamepad
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Gamepad {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
