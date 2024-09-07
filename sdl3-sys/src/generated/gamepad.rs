#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryGamepad
//!
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
//! In order to use these functions, SDL_Init() must have been called with the
//! SDL_INIT_GAMEPAD flag. This causes SDL to scan the system for gamepads, and
//! load appropriate drivers.
//!
//! If you would like to receive gamepad updates while the application is in
//! the background, you should set the following hint before calling
//! SDL_Init(): SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS

use super::stdinc::*;

use super::error::*;

use super::joystick::*;

use super::properties::*;

use super::iostream::*;

use super::sensor::*;

/// Standard gamepad types.
///
/// This type does not necessarily map to first-party controllers from
/// Microsoft/Sony/Nintendo; in many cases, third-party controllers can report
/// as these, either because they were designed for a specific console, or they
/// simply most closely match that console's controllers (does it have A/B/X/Y
/// buttons or X/O/Square/Triangle? Does it have a touchpad? etc).
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GAMEPAD_TYPE_UNKNOWN`], [`SDL_GAMEPAD_TYPE_STANDARD`], [`SDL_GAMEPAD_TYPE_XBOX360`], [`SDL_GAMEPAD_TYPE_XBOXONE`], [`SDL_GAMEPAD_TYPE_PS3`], [`SDL_GAMEPAD_TYPE_PS4`], [`SDL_GAMEPAD_TYPE_PS5`], [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO`], [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT`], [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT`], [`SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR`], [`SDL_GAMEPAD_TYPE_MAX`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadType(pub ::core::ffi::c_int);
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
    pub const MAX: Self = Self(11);
}
pub const SDL_GAMEPAD_TYPE_UNKNOWN: SDL_GamepadType = SDL_GamepadType::UNKNOWN;
pub const SDL_GAMEPAD_TYPE_STANDARD: SDL_GamepadType = SDL_GamepadType::STANDARD;
pub const SDL_GAMEPAD_TYPE_XBOX360: SDL_GamepadType = SDL_GamepadType::XBOX360;
pub const SDL_GAMEPAD_TYPE_XBOXONE: SDL_GamepadType = SDL_GamepadType::XBOXONE;
pub const SDL_GAMEPAD_TYPE_PS3: SDL_GamepadType = SDL_GamepadType::PS3;
pub const SDL_GAMEPAD_TYPE_PS4: SDL_GamepadType = SDL_GamepadType::PS4;
pub const SDL_GAMEPAD_TYPE_PS5: SDL_GamepadType = SDL_GamepadType::PS5;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO: SDL_GamepadType = SDL_GamepadType::NINTENDO_SWITCH_PRO;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT: SDL_GamepadType = SDL_GamepadType::NINTENDO_SWITCH_JOYCON_LEFT;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT: SDL_GamepadType = SDL_GamepadType::NINTENDO_SWITCH_JOYCON_RIGHT;
pub const SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR: SDL_GamepadType = SDL_GamepadType::NINTENDO_SWITCH_JOYCON_PAIR;
pub const SDL_GAMEPAD_TYPE_MAX: SDL_GamepadType = SDL_GamepadType::MAX;

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
/// SDL_GetGamepadButtonLabel()
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GAMEPAD_BUTTON_INVALID`], [`SDL_GAMEPAD_BUTTON_SOUTH`], [`SDL_GAMEPAD_BUTTON_EAST`], [`SDL_GAMEPAD_BUTTON_WEST`], [`SDL_GAMEPAD_BUTTON_NORTH`], [`SDL_GAMEPAD_BUTTON_BACK`], [`SDL_GAMEPAD_BUTTON_GUIDE`], [`SDL_GAMEPAD_BUTTON_START`], [`SDL_GAMEPAD_BUTTON_LEFT_STICK`], [`SDL_GAMEPAD_BUTTON_RIGHT_STICK`], [`SDL_GAMEPAD_BUTTON_LEFT_SHOULDER`], [`SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER`], [`SDL_GAMEPAD_BUTTON_DPAD_UP`], [`SDL_GAMEPAD_BUTTON_DPAD_DOWN`], [`SDL_GAMEPAD_BUTTON_DPAD_LEFT`], [`SDL_GAMEPAD_BUTTON_DPAD_RIGHT`], [`SDL_GAMEPAD_BUTTON_MISC1`], [`SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1`], [`SDL_GAMEPAD_BUTTON_LEFT_PADDLE1`], [`SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2`], [`SDL_GAMEPAD_BUTTON_LEFT_PADDLE2`], [`SDL_GAMEPAD_BUTTON_TOUCHPAD`], [`SDL_GAMEPAD_BUTTON_MISC2`], [`SDL_GAMEPAD_BUTTON_MISC3`], [`SDL_GAMEPAD_BUTTON_MISC4`], [`SDL_GAMEPAD_BUTTON_MISC5`], [`SDL_GAMEPAD_BUTTON_MISC6`], [`SDL_GAMEPAD_BUTTON_MAX`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadButton(pub ::core::ffi::c_int);
impl SDL_GamepadButton {
    pub const INVALID: Self = Self(-1_i32);
    pub const SOUTH: Self = Self(0_i32);
    pub const EAST: Self = Self(1_i32);
    pub const WEST: Self = Self(2_i32);
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
    pub const MISC1: Self = Self(15_i32);
    pub const RIGHT_PADDLE1: Self = Self(16_i32);
    pub const LEFT_PADDLE1: Self = Self(17_i32);
    pub const RIGHT_PADDLE2: Self = Self(18_i32);
    pub const LEFT_PADDLE2: Self = Self(19_i32);
    pub const TOUCHPAD: Self = Self(20_i32);
    pub const MISC2: Self = Self(21_i32);
    pub const MISC3: Self = Self(22_i32);
    pub const MISC4: Self = Self(23_i32);
    pub const MISC5: Self = Self(24_i32);
    pub const MISC6: Self = Self(25_i32);
    pub const MAX: Self = Self(26_i32);
}
pub const SDL_GAMEPAD_BUTTON_INVALID: SDL_GamepadButton = SDL_GamepadButton::INVALID;
pub const SDL_GAMEPAD_BUTTON_SOUTH: SDL_GamepadButton = SDL_GamepadButton::SOUTH;
pub const SDL_GAMEPAD_BUTTON_EAST: SDL_GamepadButton = SDL_GamepadButton::EAST;
pub const SDL_GAMEPAD_BUTTON_WEST: SDL_GamepadButton = SDL_GamepadButton::WEST;
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
pub const SDL_GAMEPAD_BUTTON_MISC1: SDL_GamepadButton = SDL_GamepadButton::MISC1;
pub const SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1: SDL_GamepadButton = SDL_GamepadButton::RIGHT_PADDLE1;
pub const SDL_GAMEPAD_BUTTON_LEFT_PADDLE1: SDL_GamepadButton = SDL_GamepadButton::LEFT_PADDLE1;
pub const SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2: SDL_GamepadButton = SDL_GamepadButton::RIGHT_PADDLE2;
pub const SDL_GAMEPAD_BUTTON_LEFT_PADDLE2: SDL_GamepadButton = SDL_GamepadButton::LEFT_PADDLE2;
pub const SDL_GAMEPAD_BUTTON_TOUCHPAD: SDL_GamepadButton = SDL_GamepadButton::TOUCHPAD;
pub const SDL_GAMEPAD_BUTTON_MISC2: SDL_GamepadButton = SDL_GamepadButton::MISC2;
pub const SDL_GAMEPAD_BUTTON_MISC3: SDL_GamepadButton = SDL_GamepadButton::MISC3;
pub const SDL_GAMEPAD_BUTTON_MISC4: SDL_GamepadButton = SDL_GamepadButton::MISC4;
pub const SDL_GAMEPAD_BUTTON_MISC5: SDL_GamepadButton = SDL_GamepadButton::MISC5;
pub const SDL_GAMEPAD_BUTTON_MISC6: SDL_GamepadButton = SDL_GamepadButton::MISC6;
pub const SDL_GAMEPAD_BUTTON_MAX: SDL_GamepadButton = SDL_GamepadButton::MAX;

/// The set of gamepad button labels
///
/// This isn't a complete set, just the face buttons to make it easy to show
/// button prompts.
///
/// For a complete set, you should look at the button and gamepad type and have
/// a set of symbols that work well with your art style.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN`], [`SDL_GAMEPAD_BUTTON_LABEL_A`], [`SDL_GAMEPAD_BUTTON_LABEL_B`], [`SDL_GAMEPAD_BUTTON_LABEL_X`], [`SDL_GAMEPAD_BUTTON_LABEL_Y`], [`SDL_GAMEPAD_BUTTON_LABEL_CROSS`], [`SDL_GAMEPAD_BUTTON_LABEL_CIRCLE`], [`SDL_GAMEPAD_BUTTON_LABEL_SQUARE`], [`SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadButtonLabel(pub ::core::ffi::c_int);
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
pub const SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::UNKNOWN;
pub const SDL_GAMEPAD_BUTTON_LABEL_A: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::A;
pub const SDL_GAMEPAD_BUTTON_LABEL_B: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::B;
pub const SDL_GAMEPAD_BUTTON_LABEL_X: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::X;
pub const SDL_GAMEPAD_BUTTON_LABEL_Y: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::Y;
pub const SDL_GAMEPAD_BUTTON_LABEL_CROSS: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::CROSS;
pub const SDL_GAMEPAD_BUTTON_LABEL_CIRCLE: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::CIRCLE;
pub const SDL_GAMEPAD_BUTTON_LABEL_SQUARE: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::SQUARE;
pub const SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE: SDL_GamepadButtonLabel = SDL_GamepadButtonLabel::TRIANGLE;

/// The list of axes available on a gamepad
///
/// Thumbstick axis values range from SDL_JOYSTICK_AXIS_MIN to
/// SDL_JOYSTICK_AXIS_MAX, and are centered within ~8000 of zero, though
/// advanced UI will allow users to set or autodetect the dead zone, which
/// varies between gamepads.
///
/// Trigger axis values range from 0 (released) to SDL_JOYSTICK_AXIS_MAX (fully
/// pressed) when reported by SDL_GetGamepadAxis(). Note that this is not the
/// same range that will be reported by the lower-level SDL_GetJoystickAxis().
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GAMEPAD_AXIS_INVALID`], [`SDL_GAMEPAD_AXIS_LEFTX`], [`SDL_GAMEPAD_AXIS_LEFTY`], [`SDL_GAMEPAD_AXIS_RIGHTX`], [`SDL_GAMEPAD_AXIS_RIGHTY`], [`SDL_GAMEPAD_AXIS_LEFT_TRIGGER`], [`SDL_GAMEPAD_AXIS_RIGHT_TRIGGER`], [`SDL_GAMEPAD_AXIS_MAX`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadAxis(pub ::core::ffi::c_int);
impl SDL_GamepadAxis {
    pub const INVALID: Self = Self(-1_i32);
    pub const LEFTX: Self = Self(0_i32);
    pub const LEFTY: Self = Self(1_i32);
    pub const RIGHTX: Self = Self(2_i32);
    pub const RIGHTY: Self = Self(3_i32);
    pub const LEFT_TRIGGER: Self = Self(4_i32);
    pub const RIGHT_TRIGGER: Self = Self(5_i32);
    pub const MAX: Self = Self(6_i32);
}
pub const SDL_GAMEPAD_AXIS_INVALID: SDL_GamepadAxis = SDL_GamepadAxis::INVALID;
pub const SDL_GAMEPAD_AXIS_LEFTX: SDL_GamepadAxis = SDL_GamepadAxis::LEFTX;
pub const SDL_GAMEPAD_AXIS_LEFTY: SDL_GamepadAxis = SDL_GamepadAxis::LEFTY;
pub const SDL_GAMEPAD_AXIS_RIGHTX: SDL_GamepadAxis = SDL_GamepadAxis::RIGHTX;
pub const SDL_GAMEPAD_AXIS_RIGHTY: SDL_GamepadAxis = SDL_GamepadAxis::RIGHTY;
pub const SDL_GAMEPAD_AXIS_LEFT_TRIGGER: SDL_GamepadAxis = SDL_GamepadAxis::LEFT_TRIGGER;
pub const SDL_GAMEPAD_AXIS_RIGHT_TRIGGER: SDL_GamepadAxis = SDL_GamepadAxis::RIGHT_TRIGGER;
pub const SDL_GAMEPAD_AXIS_MAX: SDL_GamepadAxis = SDL_GamepadAxis::MAX;

/// Types of gamepad control bindings.
///
/// A gamepad is a collection of bindings that map arbitrary joystick buttons,
/// axes and hat switches to specific positions on a generic console-style
/// gamepad. This enum is used as part of SDL_GamepadBinding to specify those
/// mappings.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GAMEPAD_BINDTYPE_NONE`], [`SDL_GAMEPAD_BINDTYPE_BUTTON`], [`SDL_GAMEPAD_BINDTYPE_AXIS`], [`SDL_GAMEPAD_BINDTYPE_HAT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadBindingType(pub ::core::ffi::c_int);
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
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetGamepadBindings
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
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

extern_sdlcall! {{
    /// Add support for gamepads that SDL is unaware of or change the binding of an
    /// existing gamepad.
    ///
    /// The mapping string has the format "GUID,name,mapping", where GUID is the
    /// string value from SDL_GUIDToString(), name is the human readable string for
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
    /// This string shows an example of a valid mapping for a gamepad:
    ///
    /// ```c
    /// "341a3608000000000000504944564944,Afterglow PS3 Controller,a:b1,b:b2,y:b3,x:b0,start:b9,guide:b12,back:b8,dpup:h0.1,dpleft:h0.8,dpdown:h0.4,dpright:h0.2,leftshoulder:b4,rightshoulder:b5,leftstick:b10,rightstick:b11,leftx:a0,lefty:a1,rightx:a2,righty:a3,lefttrigger:b6,righttrigger:b7"
    /// ```
    ///
    /// \param mapping the mapping string.
    /// \returns 1 if a new mapping is added, 0 if an existing mapping is updated,
    ///          -1 on failure; call SDL_GetError() for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadMapping
    /// \sa SDL_GetGamepadMappingForGUID
    pub fn SDL_AddGamepadMapping(mapping: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Load a set of gamepad mappings from an SDL_IOStream.
    ///
    /// You can call this function several times, if needed, to load different
    /// database files.
    ///
    /// If a new mapping is loaded for an already known gamepad GUID, the later
    /// version will overwrite the one currently loaded.
    ///
    /// Mappings not belonging to the current platform or with no platform field
    /// specified will be ignored (i.e. mappings for Linux will be ignored in
    /// Windows, etc).
    ///
    /// This function will load the text database entirely in memory before
    /// processing it, so take this into consideration if you are in a memory
    /// constrained environment.
    ///
    /// \param src the data stream for the mappings to be added.
    /// \param closeio if SDL_TRUE, calls SDL_CloseIO() on `src` before returning,
    ///                even in the case of an error.
    /// \returns the number of mappings added or -1 on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddGamepadMapping
    /// \sa SDL_AddGamepadMappingsFromFile
    /// \sa SDL_GetGamepadMapping
    /// \sa SDL_GetGamepadMappingForGUID
    pub fn SDL_AddGamepadMappingsFromIO(src: *mut SDL_IOStream, closeio: SDL_bool) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Load a set of gamepad mappings from a file.
    ///
    /// You can call this function several times, if needed, to load different
    /// database files.
    ///
    /// If a new mapping is loaded for an already known gamepad GUID, the later
    /// version will overwrite the one currently loaded.
    ///
    /// Mappings not belonging to the current platform or with no platform field
    /// specified will be ignored (i.e. mappings for Linux will be ignored in
    /// Windows, etc).
    ///
    /// \param file the mappings file to load.
    /// \returns the number of mappings added or -1 on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddGamepadMapping
    /// \sa SDL_AddGamepadMappingsFromIO
    /// \sa SDL_GetGamepadMapping
    /// \sa SDL_GetGamepadMappingForGUID
    pub fn SDL_AddGamepadMappingsFromFile(file: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Reinitialize the SDL mapping database to its initial state.
    ///
    /// This will generate gamepad events as needed if device mappings change.
    ///
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReloadGamepadMappings() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the current gamepad mappings.
    ///
    /// \param count a pointer filled in with the number of mappings returned, can
    ///              be NULL.
    /// \returns an array of the mapping strings, NULL-terminated, or NULL on
    ///          failure; call SDL_GetError() for more information. This is a
    ///          single allocation that should be freed with SDL_free() when it is
    ///          no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadMappings(count: *mut ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the gamepad mapping string for a given GUID.
    ///
    /// \param guid a structure containing the GUID for which a mapping is desired.
    /// \returns a mapping string or NULL on failure; call SDL_GetError() for more
    ///          information. This should be freed with SDL_free() when it is no
    ///          longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetJoystickGUIDForID
    /// \sa SDL_GetJoystickGUID
    pub fn SDL_GetGamepadMappingForGUID(guid: SDL_GUID) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the current mapping of a gamepad.
    ///
    /// Details about mappings are discussed with SDL_AddGamepadMapping().
    ///
    /// \param gamepad the gamepad you want to get the current mapping for.
    /// \returns a string that has the gamepad's mapping or NULL if no mapping is
    ///          available; call SDL_GetError() for more information. This should
    ///          be freed with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddGamepadMapping
    /// \sa SDL_GetGamepadMappingForID
    /// \sa SDL_GetGamepadMappingForGUID
    /// \sa SDL_SetGamepadMapping
    pub fn SDL_GetGamepadMapping(gamepad: *mut SDL_Gamepad) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Set the current mapping of a joystick or gamepad.
    ///
    /// Details about mappings are discussed with SDL_AddGamepadMapping().
    ///
    /// \param instance_id the joystick instance ID.
    /// \param mapping the mapping to use for this device, or NULL to clear the
    ///                mapping.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddGamepadMapping
    /// \sa SDL_GetGamepadMapping
    pub fn SDL_SetGamepadMapping(instance_id: SDL_JoystickID, mapping: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Return whether a gamepad is currently connected.
    ///
    /// \returns SDL_TRUE if a gamepad is connected, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepads
    pub fn SDL_HasGamepad() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get a list of currently connected gamepads.
    ///
    /// \param count a pointer filled in with the number of gamepads returned, may
    ///              be NULL.
    /// \returns a 0 terminated array of joystick instance IDs or NULL on failure;
    ///          call SDL_GetError() for more information. This should be freed
    ///          with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasGamepad
    /// \sa SDL_OpenGamepad
    pub fn SDL_GetGamepads(count: *mut ::core::ffi::c_int) -> *mut SDL_JoystickID;
}}

extern_sdlcall! {{
    /// Check if the given joystick is supported by the gamepad interface.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns SDL_TRUE if the given joystick is supported by the gamepad
    ///          interface, SDL_FALSE if it isn't or it's an invalid index.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetJoysticks
    /// \sa SDL_OpenGamepad
    pub fn SDL_IsGamepad(instance_id: SDL_JoystickID) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the implementation dependent name of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the name of the selected gamepad. If no name can be found, this
    ///          function returns NULL; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadName
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadNameForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the implementation dependent path of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the path of the selected gamepad. If no path can be found, this
    ///          function returns NULL; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadPath
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadPathForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the player index of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the player index of a gamepad, or -1 if it's not available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadPlayerIndex
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadPlayerIndexForID(instance_id: SDL_JoystickID) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the implementation-dependent GUID of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the GUID of the selected gamepad. If called on an invalid index,
    ///          this function returns a zero GUID.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GUIDToString
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadGUIDForID(instance_id: SDL_JoystickID) -> SDL_GUID;
}}

extern_sdlcall! {{
    /// Get the USB vendor ID of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the vendor ID isn't
    /// available this function returns 0.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the USB vendor ID of the selected gamepad. If called on an invalid
    ///          index, this function returns zero.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadVendor
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadVendorForID(instance_id: SDL_JoystickID) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the USB product ID of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the product ID isn't
    /// available this function returns 0.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the USB product ID of the selected gamepad. If called on an
    ///          invalid index, this function returns zero.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadProduct
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadProductForID(instance_id: SDL_JoystickID) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the product version of a gamepad, if available.
    ///
    /// This can be called before any gamepads are opened. If the product version
    /// isn't available this function returns 0.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the product version of the selected gamepad. If called on an
    ///          invalid index, this function returns zero.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadProductVersion
    /// \sa SDL_GetGamepads
    pub fn SDL_GetGamepadProductVersionForID(instance_id: SDL_JoystickID) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the type of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the gamepad type.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadType
    /// \sa SDL_GetGamepads
    /// \sa SDL_GetRealGamepadTypeForID
    pub fn SDL_GetGamepadTypeForID(instance_id: SDL_JoystickID) -> SDL_GamepadType;
}}

extern_sdlcall! {{
    /// Get the type of a gamepad, ignoring any mapping override.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the gamepad type.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadTypeForID
    /// \sa SDL_GetGamepads
    /// \sa SDL_GetRealGamepadType
    pub fn SDL_GetRealGamepadTypeForID(instance_id: SDL_JoystickID) -> SDL_GamepadType;
}}

extern_sdlcall! {{
    /// Get the mapping of a gamepad.
    ///
    /// This can be called before any gamepads are opened.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns the mapping string. Returns NULL if no mapping is available. This
    ///          should be freed with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepads
    /// \sa SDL_GetGamepadMapping
    pub fn SDL_GetGamepadMappingForID(instance_id: SDL_JoystickID) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Open a gamepad for use.
    ///
    /// \param instance_id the joystick instance ID.
    /// \returns a gamepad identifier or NULL if an error occurred; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CloseGamepad
    /// \sa SDL_IsGamepad
    pub fn SDL_OpenGamepad(instance_id: SDL_JoystickID) -> *mut SDL_Gamepad;
}}

extern_sdlcall! {{
    /// Get the SDL_Gamepad associated with a joystick instance ID, if it has been
    /// opened.
    ///
    /// \param instance_id the joystick instance ID of the gamepad.
    /// \returns an SDL_Gamepad on success or NULL on failure or if it hasn't been
    ///          opened yet; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadFromID(instance_id: SDL_JoystickID) -> *mut SDL_Gamepad;
}}

extern_sdlcall! {{
    /// Get the SDL_Gamepad associated with a player index.
    ///
    /// \param player_index the player index, which different from the instance ID.
    /// \returns the SDL_Gamepad associated with a player index.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadPlayerIndex
    /// \sa SDL_SetGamepadPlayerIndex
    pub fn SDL_GetGamepadFromPlayerIndex(player_index: ::core::ffi::c_int) -> *mut SDL_Gamepad;
}}

extern_sdlcall! {{
    /// Get the properties associated with an opened gamepad.
    ///
    /// These properties are shared with the underlying joystick object.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN`: true if this gamepad has an LED
    ///   that has adjustable brightness
    /// - `SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN`: true if this gamepad has an LED
    ///   that has adjustable color
    /// - `SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN`: true if this gamepad has a
    ///   player LED
    /// - `SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN`: true if this gamepad has
    ///   left/right rumble
    /// - `SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN`: true if this gamepad has
    ///   simple trigger rumble
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadProperties(gamepad: *mut SDL_Gamepad) -> SDL_PropertiesID;
}}

pub const SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.joystick.cap.mono_led\0") };

pub const SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.joystick.cap.rgb_led\0") };

pub const SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.joystick.cap.player_led\0") };

pub const SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.joystick.cap.rumble\0") };

pub const SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.joystick.cap.trigger_rumble\0") };

extern_sdlcall! {{
    /// Get the instance ID of an opened gamepad.
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    /// \returns the instance ID of the specified gamepad on success or 0 on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadID(gamepad: *mut SDL_Gamepad) -> SDL_JoystickID;
}}

extern_sdlcall! {{
    /// Get the implementation-dependent name for an opened gamepad.
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    /// \returns the implementation dependent name for the gamepad, or NULL if
    ///          there is no name or the identifier passed is invalid.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadNameForID
    pub fn SDL_GetGamepadName(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the implementation-dependent path for an opened gamepad.
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    /// \returns the implementation dependent path for the gamepad, or NULL if
    ///          there is no path or the identifier passed is invalid.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadPathForID
    pub fn SDL_GetGamepadPath(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the type of an opened gamepad.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the gamepad type, or SDL_GAMEPAD_TYPE_UNKNOWN if it's not
    ///          available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadTypeForID
    pub fn SDL_GetGamepadType(gamepad: *mut SDL_Gamepad) -> SDL_GamepadType;
}}

extern_sdlcall! {{
    /// Get the type of an opened gamepad, ignoring any mapping override.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the gamepad type, or SDL_GAMEPAD_TYPE_UNKNOWN if it's not
    ///          available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRealGamepadTypeForID
    pub fn SDL_GetRealGamepadType(gamepad: *mut SDL_Gamepad) -> SDL_GamepadType;
}}

extern_sdlcall! {{
    /// Get the player index of an opened gamepad.
    ///
    /// For XInput gamepads this returns the XInput user index.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the player index for gamepad, or -1 if it's not available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetGamepadPlayerIndex
    pub fn SDL_GetGamepadPlayerIndex(gamepad: *mut SDL_Gamepad) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Set the player index of an opened gamepad.
    ///
    /// \param gamepad the gamepad object to adjust.
    /// \param player_index player index to assign to this gamepad, or -1 to clear
    ///                     the player index and turn off player LEDs.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadPlayerIndex
    pub fn SDL_SetGamepadPlayerIndex(gamepad: *mut SDL_Gamepad, player_index: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the USB vendor ID of an opened gamepad, if available.
    ///
    /// If the vendor ID isn't available this function returns 0.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the USB vendor ID, or zero if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadVendorForID
    pub fn SDL_GetGamepadVendor(gamepad: *mut SDL_Gamepad) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the USB product ID of an opened gamepad, if available.
    ///
    /// If the product ID isn't available this function returns 0.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the USB product ID, or zero if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadProductForID
    pub fn SDL_GetGamepadProduct(gamepad: *mut SDL_Gamepad) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the product version of an opened gamepad, if available.
    ///
    /// If the product version isn't available this function returns 0.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the USB product version, or zero if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadProductVersionForID
    pub fn SDL_GetGamepadProductVersion(gamepad: *mut SDL_Gamepad) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the firmware version of an opened gamepad, if available.
    ///
    /// If the firmware version isn't available this function returns 0.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the gamepad firmware version, or zero if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadFirmwareVersion(gamepad: *mut SDL_Gamepad) -> Uint16;
}}

extern_sdlcall! {{
    /// Get the serial number of an opened gamepad, if available.
    ///
    /// Returns the serial number of the gamepad, or NULL if it is not available.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the serial number, or NULL if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadSerial(gamepad: *mut SDL_Gamepad) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the Steam Input handle of an opened gamepad, if available.
    ///
    /// Returns an InputHandle_t for the gamepad that can be used with Steam Input
    /// API: https://partner.steamgames.com/doc/api/ISteamInput
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the gamepad handle, or 0 if unavailable.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadSteamHandle(gamepad: *mut SDL_Gamepad) -> Uint64;
}}

extern_sdlcall! {{
    /// Get the connection state of a gamepad.
    ///
    /// \param gamepad the gamepad object to query.
    /// \returns the connection state on success or
    ///          `SDL_JOYSTICK_CONNECTION_INVALID` on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadConnectionState(gamepad: *mut SDL_Gamepad) -> SDL_JoystickConnectionState;
}}

extern_sdlcall! {{
    /// Get the battery state of a gamepad.
    ///
    /// You should never take a battery status as absolute truth. Batteries
    /// (especially failing batteries) are delicate hardware, and the values
    /// reported here are best estimates based on what that hardware reports. It's
    /// not uncommon for older batteries to lose stored power much faster than it
    /// reports, or completely drain when reporting it has 20 percent left, etc.
    ///
    /// \param gamepad the gamepad object to query.
    /// \param percent a pointer filled in with the percentage of battery life
    ///                left, between 0 and 100, or NULL to ignore. This will be
    ///                filled in with -1 we can't determine a value or there is no
    ///                battery.
    /// \returns the current battery state.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadPowerInfo(gamepad: *mut SDL_Gamepad, percent: *mut ::core::ffi::c_int) -> SDL_PowerState;
}}

extern_sdlcall! {{
    /// Check if a gamepad has been opened and is currently connected.
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    /// \returns SDL_TRUE if the gamepad has been opened and is currently
    ///          connected, or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GamepadConnected(gamepad: *mut SDL_Gamepad) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the underlying joystick from a gamepad.
    ///
    /// This function will give you a SDL_Joystick object, which allows you to use
    /// the SDL_Joystick functions with a SDL_Gamepad object. This would be useful
    /// for getting a joystick's position at any given time, even if it hasn't
    /// moved (moving it would produce an event, which would have the axis' value).
    ///
    /// The pointer returned is owned by the SDL_Gamepad. You should not call
    /// SDL_CloseJoystick() on it, for example, since doing so will likely cause
    /// SDL to crash.
    ///
    /// \param gamepad the gamepad object that you want to get a joystick from.
    /// \returns an SDL_Joystick object, or NULL on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadJoystick(gamepad: *mut SDL_Gamepad) -> *mut SDL_Joystick;
}}

extern_sdlcall! {{
    /// Set the state of gamepad event processing.
    ///
    /// If gamepad events are disabled, you must call SDL_UpdateGamepads() yourself
    /// and check the state of the gamepad when you want gamepad information.
    ///
    /// \param enabled whether to process gamepad events or not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadEventsEnabled
    /// \sa SDL_UpdateGamepads
    pub fn SDL_SetGamepadEventsEnabled(enabled: SDL_bool);
}}

extern_sdlcall! {{
    /// Query the state of gamepad event processing.
    ///
    /// If gamepad events are disabled, you must call SDL_UpdateGamepads() yourself
    /// and check the state of the gamepad when you want gamepad information.
    ///
    /// \returns SDL_TRUE if gamepad events are being processed, SDL_FALSE
    ///          otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetGamepadEventsEnabled
    pub fn SDL_GamepadEventsEnabled() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the SDL joystick layer bindings for a gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \param count a pointer filled in with the number of bindings returned.
    /// \returns a NULL terminated array of pointers to bindings or NULL on
    ///          failure; call SDL_GetError() for more information. This is a
    ///          single allocation that should be freed with SDL_free() when it is
    ///          no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadBindings(gamepad: *mut SDL_Gamepad, count: *mut ::core::ffi::c_int) -> *mut *mut SDL_GamepadBinding;
}}

extern_sdlcall! {{
    /// Manually pump gamepad updates if not using the loop.
    ///
    /// This function is called automatically by the event loop if events are
    /// enabled. Under such circumstances, it will not be necessary to call this
    /// function.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UpdateGamepads();
}}

extern_sdlcall! {{
    /// Convert a string into SDL_GamepadType enum.
    ///
    /// This function is called internally to translate SDL_Gamepad mapping strings
    /// for the underlying joystick device into the consistent SDL_Gamepad mapping.
    /// You do not normally need to call this function unless you are parsing
    /// SDL_Gamepad mappings in your own code.
    ///
    /// \param str string representing a SDL_GamepadType type.
    /// \returns the SDL_GamepadType enum corresponding to the input string, or
    ///          `SDL_GAMEPAD_TYPE_UNKNOWN` if no match was found.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadStringForType
    pub fn SDL_GetGamepadTypeFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadType;
}}

extern_sdlcall! {{
    /// Convert from an SDL_GamepadType enum to a string.
    ///
    /// \param type an enum value for a given SDL_GamepadType.
    /// \returns a string for the given type, or NULL if an invalid type is
    ///          specified. The string returned is of the format used by
    ///          SDL_Gamepad mapping strings.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadTypeFromString
    pub fn SDL_GetGamepadStringForType(r#type: SDL_GamepadType) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Convert a string into SDL_GamepadAxis enum.
    ///
    /// This function is called internally to translate SDL_Gamepad mapping strings
    /// for the underlying joystick device into the consistent SDL_Gamepad mapping.
    /// You do not normally need to call this function unless you are parsing
    /// SDL_Gamepad mappings in your own code.
    ///
    /// Note specially that "righttrigger" and "lefttrigger" map to
    /// `SDL_GAMEPAD_AXIS_RIGHT_TRIGGER` and `SDL_GAMEPAD_AXIS_LEFT_TRIGGER`,
    /// respectively.
    ///
    /// \param str string representing a SDL_Gamepad axis.
    /// \returns the SDL_GamepadAxis enum corresponding to the input string, or
    ///          `SDL_GAMEPAD_AXIS_INVALID` if no match was found.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadStringForAxis
    pub fn SDL_GetGamepadAxisFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadAxis;
}}

extern_sdlcall! {{
    /// Convert from an SDL_GamepadAxis enum to a string.
    ///
    /// \param axis an enum value for a given SDL_GamepadAxis.
    /// \returns a string for the given axis, or NULL if an invalid axis is
    ///          specified. The string returned is of the format used by
    ///          SDL_Gamepad mapping strings.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadAxisFromString
    pub fn SDL_GetGamepadStringForAxis(axis: SDL_GamepadAxis) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Query whether a gamepad has a given axis.
    ///
    /// This merely reports whether the gamepad's mapping defined this axis, as
    /// that is all the information SDL has about the physical device.
    ///
    /// \param gamepad a gamepad.
    /// \param axis an axis enum value (an SDL_GamepadAxis value).
    /// \returns SDL_TRUE if the gamepad has this axis, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadHasButton
    /// \sa SDL_GetGamepadAxis
    pub fn SDL_GamepadHasAxis(gamepad: *mut SDL_Gamepad, axis: SDL_GamepadAxis) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the current state of an axis control on a gamepad.
    ///
    /// The axis indices start at index 0.
    ///
    /// For thumbsticks, the state is a value ranging from -32768 (up/left) to
    /// 32767 (down/right).
    ///
    /// Triggers range from 0 when released to 32767 when fully pressed, and never
    /// return a negative value. Note that this differs from the value reported by
    /// the lower-level SDL_GetJoystickAxis(), which normally uses the full range.
    ///
    /// \param gamepad a gamepad.
    /// \param axis an axis index (one of the SDL_GamepadAxis values).
    /// \returns axis state (including 0) on success or 0 (also) on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadHasAxis
    /// \sa SDL_GetGamepadButton
    pub fn SDL_GetGamepadAxis(gamepad: *mut SDL_Gamepad, axis: SDL_GamepadAxis) -> Sint16;
}}

extern_sdlcall! {{
    /// Convert a string into an SDL_GamepadButton enum.
    ///
    /// This function is called internally to translate SDL_Gamepad mapping strings
    /// for the underlying joystick device into the consistent SDL_Gamepad mapping.
    /// You do not normally need to call this function unless you are parsing
    /// SDL_Gamepad mappings in your own code.
    ///
    /// \param str string representing a SDL_Gamepad axis.
    /// \returns the SDL_GamepadButton enum corresponding to the input string, or
    ///          `SDL_GAMEPAD_BUTTON_INVALID` if no match was found.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadStringForButton
    pub fn SDL_GetGamepadButtonFromString(str: *const ::core::ffi::c_char) -> SDL_GamepadButton;
}}

extern_sdlcall! {{
    /// Convert from an SDL_GamepadButton enum to a string.
    ///
    /// \param button an enum value for a given SDL_GamepadButton.
    /// \returns a string for the given button, or NULL if an invalid button is
    ///          specified. The string returned is of the format used by
    ///          SDL_Gamepad mapping strings.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadButtonFromString
    pub fn SDL_GetGamepadStringForButton(button: SDL_GamepadButton) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Query whether a gamepad has a given button.
    ///
    /// This merely reports whether the gamepad's mapping defined this button, as
    /// that is all the information SDL has about the physical device.
    ///
    /// \param gamepad a gamepad.
    /// \param button a button enum value (an SDL_GamepadButton value).
    /// \returns SDL_TRUE if the gamepad has this button, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadHasAxis
    pub fn SDL_GamepadHasButton(gamepad: *mut SDL_Gamepad, button: SDL_GamepadButton) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the current state of a button on a gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \param button a button index (one of the SDL_GamepadButton values).
    /// \returns 1 for pressed state or 0 for not pressed state or failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadHasButton
    /// \sa SDL_GetGamepadAxis
    pub fn SDL_GetGamepadButton(gamepad: *mut SDL_Gamepad, button: SDL_GamepadButton) -> Uint8;
}}

extern_sdlcall! {{
    /// Get the label of a button on a gamepad.
    ///
    /// \param type the type of gamepad to check.
    /// \param button a button index (one of the SDL_GamepadButton values).
    /// \returns the SDL_GamepadButtonLabel enum corresponding to the button label.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadButtonLabel
    pub fn SDL_GetGamepadButtonLabelForType(r#type: SDL_GamepadType, button: SDL_GamepadButton) -> SDL_GamepadButtonLabel;
}}

extern_sdlcall! {{
    /// Get the label of a button on a gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \param button a button index (one of the SDL_GamepadButton values).
    /// \returns the SDL_GamepadButtonLabel enum corresponding to the button label.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadButtonLabelForType
    pub fn SDL_GetGamepadButtonLabel(gamepad: *mut SDL_Gamepad, button: SDL_GamepadButton) -> SDL_GamepadButtonLabel;
}}

extern_sdlcall! {{
    /// Get the number of touchpads on a gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \returns number of touchpads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetNumGamepadTouchpadFingers
    pub fn SDL_GetNumGamepadTouchpads(gamepad: *mut SDL_Gamepad) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the number of supported simultaneous fingers on a touchpad on a game
    /// gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \param touchpad a touchpad.
    /// \returns number of supported simultaneous fingers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadTouchpadFinger
    /// \sa SDL_GetNumGamepadTouchpads
    pub fn SDL_GetNumGamepadTouchpadFingers(gamepad: *mut SDL_Gamepad, touchpad: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the current state of a finger on a touchpad on a gamepad.
    ///
    /// \param gamepad a gamepad.
    /// \param touchpad a touchpad.
    /// \param finger a finger.
    /// \param state filled with state.
    /// \param x filled with x position, normalized 0 to 1, with the origin in the
    ///          upper left.
    /// \param y filled with y position, normalized 0 to 1, with the origin in the
    ///          upper left.
    /// \param pressure filled with pressure value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetNumGamepadTouchpadFingers
    pub fn SDL_GetGamepadTouchpadFinger(gamepad: *mut SDL_Gamepad, touchpad: ::core::ffi::c_int, finger: ::core::ffi::c_int, state: *mut Uint8, x: *mut ::core::ffi::c_float, y: *mut ::core::ffi::c_float, pressure: *mut ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Return whether a gamepad has a particular sensor.
    ///
    /// \param gamepad the gamepad to query.
    /// \param type the type of sensor to query.
    /// \returns SDL_TRUE if the sensor exists, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadSensorData
    /// \sa SDL_GetGamepadSensorDataRate
    /// \sa SDL_SetGamepadSensorEnabled
    pub fn SDL_GamepadHasSensor(gamepad: *mut SDL_Gamepad, r#type: SDL_SensorType) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set whether data reporting for a gamepad sensor is enabled.
    ///
    /// \param gamepad the gamepad to update.
    /// \param type the type of sensor to enable/disable.
    /// \param enabled whether data reporting should be enabled.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GamepadHasSensor
    /// \sa SDL_GamepadSensorEnabled
    pub fn SDL_SetGamepadSensorEnabled(gamepad: *mut SDL_Gamepad, r#type: SDL_SensorType, enabled: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Query whether sensor data reporting is enabled for a gamepad.
    ///
    /// \param gamepad the gamepad to query.
    /// \param type the type of sensor to query.
    /// \returns SDL_TRUE if the sensor is enabled, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetGamepadSensorEnabled
    pub fn SDL_GamepadSensorEnabled(gamepad: *mut SDL_Gamepad, r#type: SDL_SensorType) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the data rate (number of events per second) of a gamepad sensor.
    ///
    /// \param gamepad the gamepad to query.
    /// \param type the type of sensor to query.
    /// \returns the data rate, or 0.0f if the data rate is not available.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadSensorDataRate(gamepad: *mut SDL_Gamepad, r#type: SDL_SensorType) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Get the current state of a gamepad sensor.
    ///
    /// The number of values and interpretation of the data is sensor dependent.
    /// See SDL_sensor.h for the details for each type of sensor.
    ///
    /// \param gamepad the gamepad to query.
    /// \param type the type of sensor to query.
    /// \param data a pointer filled with the current sensor state.
    /// \param num_values the number of values to write to data.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGamepadSensorData(gamepad: *mut SDL_Gamepad, r#type: SDL_SensorType, data: *mut ::core::ffi::c_float, num_values: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Start a rumble effect on a gamepad.
    ///
    /// Each call to this function cancels any previous rumble effect, and calling
    /// it with 0 intensity stops any rumbling.
    ///
    /// This function requires you to process SDL events or call
    /// SDL_UpdateJoysticks() to update rumble state.
    ///
    /// \param gamepad the gamepad to vibrate.
    /// \param low_frequency_rumble the intensity of the low frequency (left)
    ///                             rumble motor, from 0 to 0xFFFF.
    /// \param high_frequency_rumble the intensity of the high frequency (right)
    ///                              rumble motor, from 0 to 0xFFFF.
    /// \param duration_ms the duration of the rumble effect, in milliseconds.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RumbleGamepad(gamepad: *mut SDL_Gamepad, low_frequency_rumble: Uint16, high_frequency_rumble: Uint16, duration_ms: Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Start a rumble effect in the gamepad's triggers.
    ///
    /// Each call to this function cancels any previous trigger rumble effect, and
    /// calling it with 0 intensity stops any rumbling.
    ///
    /// Note that this is rumbling of the _triggers_ and not the gamepad as a
    /// whole. This is currently only supported on Xbox One gamepads. If you want
    /// the (more common) whole-gamepad rumble, use SDL_RumbleGamepad() instead.
    ///
    /// This function requires you to process SDL events or call
    /// SDL_UpdateJoysticks() to update rumble state.
    ///
    /// \param gamepad the gamepad to vibrate.
    /// \param left_rumble the intensity of the left trigger rumble motor, from 0
    ///                    to 0xFFFF.
    /// \param right_rumble the intensity of the right trigger rumble motor, from 0
    ///                     to 0xFFFF.
    /// \param duration_ms the duration of the rumble effect, in milliseconds.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RumbleGamepad
    pub fn SDL_RumbleGamepadTriggers(gamepad: *mut SDL_Gamepad, left_rumble: Uint16, right_rumble: Uint16, duration_ms: Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Update a gamepad's LED color.
    ///
    /// An example of a joystick LED is the light on the back of a PlayStation 4's
    /// DualShock 4 controller.
    ///
    /// For gamepads with a single color LED, the maximum of the RGB values will be
    /// used as the LED brightness.
    ///
    /// \param gamepad the gamepad to update.
    /// \param red the intensity of the red LED.
    /// \param green the intensity of the green LED.
    /// \param blue the intensity of the blue LED.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGamepadLED(gamepad: *mut SDL_Gamepad, red: Uint8, green: Uint8, blue: Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Send a gamepad specific effect packet.
    ///
    /// \param gamepad the gamepad to affect.
    /// \param data the data to send to the gamepad.
    /// \param size the size of the data to send to the gamepad.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SendGamepadEffect(gamepad: *mut SDL_Gamepad, data: *const ::core::ffi::c_void, size: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Close a gamepad previously opened with SDL_OpenGamepad().
    ///
    /// \param gamepad a gamepad identifier previously returned by
    ///                SDL_OpenGamepad().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_OpenGamepad
    pub fn SDL_CloseGamepad(gamepad: *mut SDL_Gamepad);
}}

extern_sdlcall! {{
    /// Return the sfSymbolsName for a given button on a gamepad on Apple
    /// platforms.
    ///
    /// \param gamepad the gamepad to query.
    /// \param button a button on the gamepad.
    /// \returns the sfSymbolsName or NULL if the name can't be found.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadAppleSFSymbolsNameForAxis
    pub fn SDL_GetGamepadAppleSFSymbolsNameForButton(gamepad: *mut SDL_Gamepad, button: SDL_GamepadButton) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Return the sfSymbolsName for a given axis on a gamepad on Apple platforms.
    ///
    /// \param gamepad the gamepad to query.
    /// \param axis an axis on the gamepad.
    /// \returns the sfSymbolsName or NULL if the name can't be found.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGamepadAppleSFSymbolsNameForButton
    pub fn SDL_GetGamepadAppleSFSymbolsNameForAxis(gamepad: *mut SDL_Gamepad, axis: SDL_GamepadAxis) -> *const ::core::ffi::c_char;
}}

/// The structure used to identify an SDL gamepad
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Gamepad { _opaque: [::core::primitive::u8; 0] }

