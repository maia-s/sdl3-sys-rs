//! SDL joystick support.
//!
//! This is the lower-level joystick handling. If you want the simpler option,
//! where what buttons does what is well-defined, you should use the gamepad
//! API instead.
//!
//! The term "instance_id" is the current instantiation of a joystick device in
//! the system, if the joystick is removed and then re-inserted then it will
//! get a new instance_id, instance_id's are monotonically increasing
//! identifiers of a joystick plugged in.
//!
//! The term "player_index" is the number assigned to a player on a specific
//! controller. For XInput controllers this returns the XInput user index. Many
//! joysticks will not be able to supply this information.
//!
//! [`SDL_GUID`] is used as a stable 128-bit identifier for a joystick device that
//! does not change over time. It identifies class of the device (a X360 wired
//! controller for example). This identifier is platform dependent.
//!
//! In order to use these functions, [`SDL_Init()`] must have been called with the
//! [`SDL_INIT_JOYSTICK`] flag. This causes SDL to scan the system for joysticks,
//! and load appropriate drivers.
//!
//! If you would like to receive joystick updates while the application is in
//! the background, you should set the following hint before calling
//! [`SDL_Init()`]: [`SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS`]

use super::stdinc::*;

use super::error::*;

use super::guid::*;

use super::mutex::*;

use super::power::*;

use super::properties::*;

use super::sensor::*;

/// This is a unique ID for a joystick for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// If the joystick is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_JoystickID = Uint32;

/// An enum of some common joystick types.
///
/// In some cases, SDL can identify a low-level joystick as being a certain
/// type of device, and will report it through [`SDL_GetJoystickType`] (or
/// [`SDL_GetJoystickTypeForID`]).
///
/// This is by no means a complete list of everything that can be plugged into
/// a computer.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_JoystickType::UNKNOWN) | [`SDL_JOYSTICK_TYPE_UNKNOWN`] | |
/// | [`GAMEPAD`](SDL_JoystickType::GAMEPAD) | [`SDL_JOYSTICK_TYPE_GAMEPAD`] | |
/// | [`WHEEL`](SDL_JoystickType::WHEEL) | [`SDL_JOYSTICK_TYPE_WHEEL`] | |
/// | [`ARCADE_STICK`](SDL_JoystickType::ARCADE_STICK) | [`SDL_JOYSTICK_TYPE_ARCADE_STICK`] | |
/// | [`FLIGHT_STICK`](SDL_JoystickType::FLIGHT_STICK) | [`SDL_JOYSTICK_TYPE_FLIGHT_STICK`] | |
/// | [`DANCE_PAD`](SDL_JoystickType::DANCE_PAD) | [`SDL_JOYSTICK_TYPE_DANCE_PAD`] | |
/// | [`GUITAR`](SDL_JoystickType::GUITAR) | [`SDL_JOYSTICK_TYPE_GUITAR`] | |
/// | [`DRUM_KIT`](SDL_JoystickType::DRUM_KIT) | [`SDL_JOYSTICK_TYPE_DRUM_KIT`] | |
/// | [`ARCADE_PAD`](SDL_JoystickType::ARCADE_PAD) | [`SDL_JOYSTICK_TYPE_ARCADE_PAD`] | |
/// | [`THROTTLE`](SDL_JoystickType::THROTTLE) | [`SDL_JOYSTICK_TYPE_THROTTLE`] | |
/// | [`COUNT`](SDL_JoystickType::COUNT) | [`SDL_JOYSTICK_TYPE_COUNT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_JoystickType(pub ::core::ffi::c_int);

impl From<SDL_JoystickType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_JoystickType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_JoystickType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_JOYSTICK_TYPE_UNKNOWN",
            Self::GAMEPAD => "SDL_JOYSTICK_TYPE_GAMEPAD",
            Self::WHEEL => "SDL_JOYSTICK_TYPE_WHEEL",
            Self::ARCADE_STICK => "SDL_JOYSTICK_TYPE_ARCADE_STICK",
            Self::FLIGHT_STICK => "SDL_JOYSTICK_TYPE_FLIGHT_STICK",
            Self::DANCE_PAD => "SDL_JOYSTICK_TYPE_DANCE_PAD",
            Self::GUITAR => "SDL_JOYSTICK_TYPE_GUITAR",
            Self::DRUM_KIT => "SDL_JOYSTICK_TYPE_DRUM_KIT",
            Self::ARCADE_PAD => "SDL_JOYSTICK_TYPE_ARCADE_PAD",
            Self::THROTTLE => "SDL_JOYSTICK_TYPE_THROTTLE",
            Self::COUNT => "SDL_JOYSTICK_TYPE_COUNT",

            _ => return write!(f, "SDL_JoystickType({})", self.0),
        })
    }
}

impl SDL_JoystickType {
    pub const UNKNOWN: Self = Self(0);
    pub const GAMEPAD: Self = Self(1);
    pub const WHEEL: Self = Self(2);
    pub const ARCADE_STICK: Self = Self(3);
    pub const FLIGHT_STICK: Self = Self(4);
    pub const DANCE_PAD: Self = Self(5);
    pub const GUITAR: Self = Self(6);
    pub const DRUM_KIT: Self = Self(7);
    pub const ARCADE_PAD: Self = Self(8);
    pub const THROTTLE: Self = Self(9);
    pub const COUNT: Self = Self(10);
}

pub const SDL_JOYSTICK_TYPE_UNKNOWN: SDL_JoystickType = SDL_JoystickType::UNKNOWN;
pub const SDL_JOYSTICK_TYPE_GAMEPAD: SDL_JoystickType = SDL_JoystickType::GAMEPAD;
pub const SDL_JOYSTICK_TYPE_WHEEL: SDL_JoystickType = SDL_JoystickType::WHEEL;
pub const SDL_JOYSTICK_TYPE_ARCADE_STICK: SDL_JoystickType = SDL_JoystickType::ARCADE_STICK;
pub const SDL_JOYSTICK_TYPE_FLIGHT_STICK: SDL_JoystickType = SDL_JoystickType::FLIGHT_STICK;
pub const SDL_JOYSTICK_TYPE_DANCE_PAD: SDL_JoystickType = SDL_JoystickType::DANCE_PAD;
pub const SDL_JOYSTICK_TYPE_GUITAR: SDL_JoystickType = SDL_JoystickType::GUITAR;
pub const SDL_JOYSTICK_TYPE_DRUM_KIT: SDL_JoystickType = SDL_JoystickType::DRUM_KIT;
pub const SDL_JOYSTICK_TYPE_ARCADE_PAD: SDL_JoystickType = SDL_JoystickType::ARCADE_PAD;
pub const SDL_JOYSTICK_TYPE_THROTTLE: SDL_JoystickType = SDL_JoystickType::THROTTLE;
pub const SDL_JOYSTICK_TYPE_COUNT: SDL_JoystickType = SDL_JoystickType::COUNT;

/// Possible connection states for a joystick device.
///
/// This is used by [`SDL_GetJoystickConnectionState`] to report how a device is
/// connected to the system.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_JoystickConnectionState::INVALID) | [`SDL_JOYSTICK_CONNECTION_INVALID`] | |
/// | [`UNKNOWN`](SDL_JoystickConnectionState::UNKNOWN) | [`SDL_JOYSTICK_CONNECTION_UNKNOWN`] | |
/// | [`WIRED`](SDL_JoystickConnectionState::WIRED) | [`SDL_JOYSTICK_CONNECTION_WIRED`] | |
/// | [`WIRELESS`](SDL_JoystickConnectionState::WIRELESS) | [`SDL_JOYSTICK_CONNECTION_WIRELESS`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_JoystickConnectionState(pub ::core::ffi::c_int);

impl From<SDL_JoystickConnectionState> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_JoystickConnectionState) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_JoystickConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_JOYSTICK_CONNECTION_INVALID",
            Self::UNKNOWN => "SDL_JOYSTICK_CONNECTION_UNKNOWN",
            Self::WIRED => "SDL_JOYSTICK_CONNECTION_WIRED",
            Self::WIRELESS => "SDL_JOYSTICK_CONNECTION_WIRELESS",

            _ => return write!(f, "SDL_JoystickConnectionState({})", self.0),
        })
    }
}

impl SDL_JoystickConnectionState {
    pub const INVALID: Self = Self(-1_i32);
    pub const UNKNOWN: Self = Self(0_i32);
    pub const WIRED: Self = Self(1_i32);
    pub const WIRELESS: Self = Self(2_i32);
}

pub const SDL_JOYSTICK_CONNECTION_INVALID: SDL_JoystickConnectionState =
    SDL_JoystickConnectionState::INVALID;
pub const SDL_JOYSTICK_CONNECTION_UNKNOWN: SDL_JoystickConnectionState =
    SDL_JoystickConnectionState::UNKNOWN;
pub const SDL_JOYSTICK_CONNECTION_WIRED: SDL_JoystickConnectionState =
    SDL_JoystickConnectionState::WIRED;
pub const SDL_JOYSTICK_CONNECTION_WIRELESS: SDL_JoystickConnectionState =
    SDL_JoystickConnectionState::WIRELESS;

/// The largest value an SDL_Joystick's axis can report.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_JOYSTICK_AXIS_MIN`]
pub const SDL_JOYSTICK_AXIS_MAX: Sint16 = (32767 as Sint16);

/// The smallest value an SDL_Joystick's axis can report.
///
/// This is a negative number!
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_JOYSTICK_AXIS_MAX`]
pub const SDL_JOYSTICK_AXIS_MIN: Sint16 = (-32768_i32 as Sint16);

extern "C" {
    /// Locking for atomic access to the joystick API.
    ///
    /// The SDL joystick functions are thread-safe, however you can lock the
    /// joysticks while processing to guarantee that the joystick list won't change
    /// and joystick and gamepad events will not be delivered.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_LockJoysticks();
}

extern "C" {
    /// Unlocking for atomic access to the joystick API.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_UnlockJoysticks();
}

extern "C" {
    /// Return whether a joystick is currently connected.
    ///
    /// ### Return value
    /// Returns true if a joystick is connected, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_HasJoystick() -> ::core::primitive::bool;
}

extern "C" {
    /// Get a list of currently connected joysticks.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of joysticks returned, may
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
    /// - [`SDL_HasJoystick`]
    /// - [`SDL_OpenJoystick`]
    pub fn SDL_GetJoysticks(count: *mut ::core::ffi::c_int) -> *mut SDL_JoystickID;
}

extern "C" {
    /// Get the implementation dependent name of a joystick.
    ///
    /// This can be called before any joysticks are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the name of the selected joystick. If no name can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickName`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickNameForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the implementation dependent path of a joystick.
    ///
    /// This can be called before any joysticks are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the path of the selected joystick. If no path can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickPath`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickPathForID(instance_id: SDL_JoystickID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the player index of a joystick.
    ///
    /// This can be called before any joysticks are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the player index of a joystick, or -1 if it's not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickPlayerIndex`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickPlayerIndexForID(instance_id: SDL_JoystickID) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the implementation-dependent GUID of a joystick.
    ///
    /// This can be called before any joysticks are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the GUID of the selected joystick. If called with an invalid
    ///   instance_id, this function returns a zero GUID.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickGUID`]
    /// - [`SDL_GUIDToString`]
    pub fn SDL_GetJoystickGUIDForID(instance_id: SDL_JoystickID) -> SDL_GUID;
}

extern "C" {
    /// Get the USB vendor ID of a joystick, if available.
    ///
    /// This can be called before any joysticks are opened. If the vendor ID isn't
    /// available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the USB vendor ID of the selected joystick. If called with an
    ///   invalid instance_id, this function returns 0.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickVendor`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickVendorForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the USB product ID of a joystick, if available.
    ///
    /// This can be called before any joysticks are opened. If the product ID isn't
    /// available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the USB product ID of the selected joystick. If called with an
    ///   invalid instance_id, this function returns 0.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickProduct`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickProductForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the product version of a joystick, if available.
    ///
    /// This can be called before any joysticks are opened. If the product version
    /// isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the product version of the selected joystick. If called with an
    ///   invalid instance_id, this function returns 0.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickProductVersion`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickProductVersionForID(instance_id: SDL_JoystickID) -> Uint16;
}

extern "C" {
    /// Get the type of a joystick, if available.
    ///
    /// This can be called before any joysticks are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns the [`SDL_JoystickType`] of the selected joystick. If called with an
    ///   invalid instance_id, this function returns
    ///   [`SDL_JOYSTICK_TYPE_UNKNOWN`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickType`]
    /// - [`SDL_GetJoysticks`]
    pub fn SDL_GetJoystickTypeForID(instance_id: SDL_JoystickID) -> SDL_JoystickType;
}

extern "C" {
    /// Open a joystick for use.
    ///
    /// The joystick subsystem must be initialized before a joystick can be opened
    /// for use.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns a joystick identifier or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseJoystick`]
    pub fn SDL_OpenJoystick(instance_id: SDL_JoystickID) -> *mut SDL_Joystick;
}

extern "C" {
    /// Get the [`SDL_Joystick`] associated with an instance ID, if it has been opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the instance ID to get the [`SDL_Joystick`] for.
    ///
    /// ### Return value
    /// Returns an [`SDL_Joystick`] on success or NULL on failure or if it hasn't been
    ///   opened yet; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickFromID(instance_id: SDL_JoystickID) -> *mut SDL_Joystick;
}

extern "C" {
    /// Get the [`SDL_Joystick`] associated with a player index.
    ///
    /// ### Parameters
    /// - `player_index`: the player index to get the [`SDL_Joystick`] for.
    ///
    /// ### Return value
    /// Returns an [`SDL_Joystick`] on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickPlayerIndex`]
    /// - [`SDL_SetJoystickPlayerIndex`]
    pub fn SDL_GetJoystickFromPlayerIndex(player_index: ::core::ffi::c_int) -> *mut SDL_Joystick;
}

/// The structure that describes a virtual joystick touchpad.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_VirtualJoystickDesc`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_VirtualJoystickTouchpadDesc {
    /// the number of simultaneous fingers on this touchpad
    pub nfingers: Uint16,
    pub padding: [Uint16; 3],
}

/// The structure that describes a virtual joystick sensor.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_VirtualJoystickDesc`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_VirtualJoystickSensorDesc {
    /// the type of this sensor
    pub r#type: SDL_SensorType,
    /// the update frequency of this sensor, may be 0.0f
    pub rate: ::core::ffi::c_float,
}

/// The structure that describes a virtual joystick.
///
/// This structure should be initialized using [`SDL_INIT_INTERFACE()`]. All
/// elements of this structure are optional.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_AttachVirtualJoystick`]
/// - [`SDL_INIT_INTERFACE`]
/// - [`SDL_VirtualJoystickSensorDesc`]
/// - [`SDL_VirtualJoystickTouchpadDesc`]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_VirtualJoystickDesc {
    /// the version of this interface
    pub version: Uint32,
    /// [`SDL_JoystickType`]
    pub r#type: Uint16,
    /// unused
    pub padding: Uint16,
    /// the USB vendor ID of this joystick
    pub vendor_id: Uint16,
    /// the USB product ID of this joystick
    pub product_id: Uint16,
    /// the number of axes on this joystick
    pub naxes: Uint16,
    /// the number of buttons on this joystick
    pub nbuttons: Uint16,
    /// the number of balls on this joystick
    pub nballs: Uint16,
    /// the number of hats on this joystick
    pub nhats: Uint16,
    /// the number of touchpads on this joystick, requires `touchpads` to point at valid descriptions
    pub ntouchpads: Uint16,
    /// the number of sensors on this joystick, requires `sensors` to point at valid descriptions
    pub nsensors: Uint16,
    /// unused
    pub padding2: [Uint16; 2],
    /// A mask of which buttons are valid for this controller
    /// e.g. (1 << [`SDL_GAMEPAD_BUTTON_SOUTH`])
    pub button_mask: Uint32,
    /// A mask of which axes are valid for this controller
    /// e.g. (1 << [`SDL_GAMEPAD_AXIS_LEFTX`])
    pub axis_mask: Uint32,
    /// the name of the joystick
    pub name: *const ::core::ffi::c_char,
    /// A pointer to an array of touchpad descriptions, required if `ntouchpads` is > 0
    pub touchpads: *const SDL_VirtualJoystickTouchpadDesc,
    /// A pointer to an array of sensor descriptions, required if `nsensors` is > 0
    pub sensors: *const SDL_VirtualJoystickSensorDesc,
    /// User data pointer passed to callbacks
    pub userdata: *mut ::core::ffi::c_void,
    /// Called when the joystick state should be updated
    pub Update: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void)>,
    /// Called when the player index is set
    pub SetPlayerIndex: ::core::option::Option<
        extern "C" fn(userdata: *mut ::core::ffi::c_void, player_index: ::core::ffi::c_int),
    >,
    /// Implements [`SDL_RumbleJoystick()`]
    pub Rumble: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            low_frequency_rumble: Uint16,
            high_frequency_rumble: Uint16,
        ) -> ::core::primitive::bool,
    >,
    /// Implements [`SDL_RumbleJoystickTriggers()`]
    pub RumbleTriggers: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            left_rumble: Uint16,
            right_rumble: Uint16,
        ) -> ::core::primitive::bool,
    >,
    /// Implements [`SDL_SetJoystickLED()`]
    pub SetLED: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            red: Uint8,
            green: Uint8,
            blue: Uint8,
        ) -> ::core::primitive::bool,
    >,
    /// Implements [`SDL_SendJoystickEffect()`]
    pub SendEffect: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            data: *const ::core::ffi::c_void,
            size: ::core::ffi::c_int,
        ) -> ::core::primitive::bool,
    >,
    /// Implements [`SDL_SetGamepadSensorEnabled()`]
    pub SetSensorsEnabled: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            enabled: ::core::primitive::bool,
        ) -> ::core::primitive::bool,
    >,
    /// Cleans up the userdata when the joystick is detached
    pub Cleanup: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void)>,
}

impl SDL_VirtualJoystickDesc {
    /// Create a new `SDL_VirtualJoystickDesc` initialized with `SDL_INIT_INTERFACE`
    #[inline]
    pub const fn new() -> Self {
        ::core::assert!(::core::mem::size_of::<Self>() <= ::core::primitive::u32::MAX as usize);
        let mut this = unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        this.version = ::core::mem::size_of::<Self>() as ::core::primitive::u32;
        this
    }
}

impl ::core::default::Default for SDL_VirtualJoystickDesc {
    /// Create a new `SDL_VirtualJoystickDesc` initialized with `SDL_INIT_INTERFACE`
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const _: () = ::core::assert!(
    (((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 4_usize)
        && (::core::mem::size_of::<SDL_VirtualJoystickDesc>() == 84_usize))
        || ((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8_usize)
            && (::core::mem::size_of::<SDL_VirtualJoystickDesc>() == 136_usize)))
);

extern "C" {
    /// Attach a new virtual joystick.
    ///
    /// ### Parameters
    /// - `desc`: joystick description, initialized using [`SDL_INIT_INTERFACE()`].
    ///
    /// ### Return value
    /// Returns the joystick instance ID, or 0 on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DetachVirtualJoystick`]
    pub fn SDL_AttachVirtualJoystick(desc: *const SDL_VirtualJoystickDesc) -> SDL_JoystickID;
}

extern "C" {
    /// Detach a virtual joystick.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID, previously returned from
    ///   [`SDL_AttachVirtualJoystick()`].
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AttachVirtualJoystick`]
    pub fn SDL_DetachVirtualJoystick(instance_id: SDL_JoystickID) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether or not a joystick is virtual.
    ///
    /// ### Parameters
    /// - `instance_id`: the joystick instance ID.
    ///
    /// ### Return value
    /// Returns true if the joystick is virtual, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_IsJoystickVirtual(instance_id: SDL_JoystickID) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the state of an axis on an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// Note that when sending trigger axes, you should scale the value to the full
    /// range of Sint16. For example, a trigger at rest would have the value of
    /// [`SDL_JOYSTICK_AXIS_MIN`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `axis`: the index of the axis on the virtual joystick to update.
    /// - `value`: the new value for the specified axis.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetJoystickVirtualAxis(
        joystick: *mut SDL_Joystick,
        axis: ::core::ffi::c_int,
        value: Sint16,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Generate ball motion on an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `ball`: the index of the ball on the virtual joystick to update.
    /// - `xrel`: the relative motion on the X axis.
    /// - `yrel`: the relative motion on the Y axis.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetJoystickVirtualBall(
        joystick: *mut SDL_Joystick,
        ball: ::core::ffi::c_int,
        xrel: Sint16,
        yrel: Sint16,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the state of a button on an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `button`: the index of the button on the virtual joystick to update.
    /// - `down`: true if the button is pressed, false otherwise.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetJoystickVirtualButton(
        joystick: *mut SDL_Joystick,
        button: ::core::ffi::c_int,
        down: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the state of a hat on an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `hat`: the index of the hat on the virtual joystick to update.
    /// - `value`: the new value for the specified hat.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetJoystickVirtualHat(
        joystick: *mut SDL_Joystick,
        hat: ::core::ffi::c_int,
        value: Uint8,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set touchpad finger state on an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `touchpad`: the index of the touchpad on the virtual joystick to
    ///   update.
    /// - `finger`: the index of the finger on the touchpad to set.
    /// - `down`: true if the finger is pressed, false if the finger is released.
    /// - `x`: the x coordinate of the finger on the touchpad, normalized 0 to 1,
    ///   with the origin in the upper left.
    /// - `y`: the y coordinate of the finger on the touchpad, normalized 0 to 1,
    ///   with the origin in the upper left.
    /// - `pressure`: the pressure of the finger.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetJoystickVirtualTouchpad(
        joystick: *mut SDL_Joystick,
        touchpad: ::core::ffi::c_int,
        finger: ::core::ffi::c_int,
        down: ::core::primitive::bool,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
        pressure: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Send a sensor update for an opened virtual joystick.
    ///
    /// Please note that values set here will not be applied until the next call to
    /// [`SDL_UpdateJoysticks`], which can either be called directly, or can be called
    /// indirectly through various other SDL APIs, including, but not limited to
    /// the following: [`SDL_PollEvent`], [`SDL_PumpEvents`], [`SDL_WaitEventTimeout`],
    /// [`SDL_WaitEvent`].
    ///
    /// ### Parameters
    /// - `joystick`: the virtual joystick on which to set state.
    /// - `type`: the type of the sensor on the virtual joystick to update.
    /// - `sensor_timestamp`: a 64-bit timestamp in nanoseconds associated with
    ///   the sensor reading.
    /// - `data`: the data associated with the sensor reading.
    /// - `num_values`: the number of values pointed to by `data`.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SendJoystickVirtualSensorData(
        joystick: *mut SDL_Joystick,
        r#type: SDL_SensorType,
        sensor_timestamp: Uint64,
        data: *const ::core::ffi::c_float,
        num_values: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the properties associated with a joystick.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - [`SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN`]: true if this joystick has an
    ///   LED that has adjustable brightness
    /// - [`SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN`]: true if this joystick has an LED
    ///   that has adjustable color
    /// - [`SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN`]: true if this joystick has a
    ///   player LED
    /// - [`SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN`]: true if this joystick has
    ///   left/right rumble
    /// - [`SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN`]: true if this joystick has
    ///   simple trigger rumble
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickProperties(joystick: *mut SDL_Joystick) -> SDL_PropertiesID;
}

pub const SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.joystick.cap.mono_led".as_ptr();

pub const SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.joystick.cap.rgb_led".as_ptr();

pub const SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.joystick.cap.player_led".as_ptr();

pub const SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.joystick.cap.rumble".as_ptr();

pub const SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.joystick.cap.trigger_rumble".as_ptr();

extern "C" {
    /// Get the implementation dependent name of a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the name of the selected joystick. If no name can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickNameForID`]
    pub fn SDL_GetJoystickName(joystick: *mut SDL_Joystick) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the implementation dependent path of a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the path of the selected joystick. If no path can be found, this
    ///   function returns NULL; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickPathForID`]
    pub fn SDL_GetJoystickPath(joystick: *mut SDL_Joystick) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the player index of an opened joystick.
    ///
    /// For XInput controllers this returns the XInput user index. Many joysticks
    /// will not be able to supply this information.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the player index, or -1 if it's not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetJoystickPlayerIndex`]
    pub fn SDL_GetJoystickPlayerIndex(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set the player index of an opened joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    /// - `player_index`: player index to assign to this joystick, or -1 to clear
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
    /// - [`SDL_GetJoystickPlayerIndex`]
    pub fn SDL_SetJoystickPlayerIndex(
        joystick: *mut SDL_Joystick,
        player_index: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the implementation-dependent GUID for the joystick.
    ///
    /// This function requires an open joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the GUID of the given joystick. If called on an invalid index,
    ///   this function returns a zero GUID; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickGUIDForID`]
    /// - [`SDL_GUIDToString`]
    pub fn SDL_GetJoystickGUID(joystick: *mut SDL_Joystick) -> SDL_GUID;
}

extern "C" {
    /// Get the USB vendor ID of an opened joystick, if available.
    ///
    /// If the vendor ID isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the USB vendor ID of the selected joystick, or 0 if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickVendorForID`]
    pub fn SDL_GetJoystickVendor(joystick: *mut SDL_Joystick) -> Uint16;
}

extern "C" {
    /// Get the USB product ID of an opened joystick, if available.
    ///
    /// If the product ID isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the USB product ID of the selected joystick, or 0 if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickProductForID`]
    pub fn SDL_GetJoystickProduct(joystick: *mut SDL_Joystick) -> Uint16;
}

extern "C" {
    /// Get the product version of an opened joystick, if available.
    ///
    /// If the product version isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the product version of the selected joystick, or 0 if unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickProductVersionForID`]
    pub fn SDL_GetJoystickProductVersion(joystick: *mut SDL_Joystick) -> Uint16;
}

extern "C" {
    /// Get the firmware version of an opened joystick, if available.
    ///
    /// If the firmware version isn't available this function returns 0.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the firmware version of the selected joystick, or 0 if
    ///   unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickFirmwareVersion(joystick: *mut SDL_Joystick) -> Uint16;
}

extern "C" {
    /// Get the serial number of an opened joystick, if available.
    ///
    /// Returns the serial number of the joystick, or NULL if it is not available.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the serial number of the selected joystick, or NULL if
    ///   unavailable.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickSerial(joystick: *mut SDL_Joystick) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the type of an opened joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] obtained from [`SDL_OpenJoystick()`].
    ///
    /// ### Return value
    /// Returns the [`SDL_JoystickType`] of the selected joystick.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickTypeForID`]
    pub fn SDL_GetJoystickType(joystick: *mut SDL_Joystick) -> SDL_JoystickType;
}

extern "C" {
    /// Get the device information encoded in a [`SDL_GUID`] structure.
    ///
    /// ### Parameters
    /// - `guid`: the [`SDL_GUID`] you wish to get info about.
    /// - `vendor`: a pointer filled in with the device VID, or 0 if not
    ///   available.
    /// - `product`: a pointer filled in with the device PID, or 0 if not
    ///   available.
    /// - `version`: a pointer filled in with the device version, or 0 if not
    ///   available.
    /// - `crc16`: a pointer filled in with a CRC used to distinguish different
    ///   products with the same VID/PID, or 0 if not available.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickGUIDForID`]
    pub fn SDL_GetJoystickGUIDInfo(
        guid: SDL_GUID,
        vendor: *mut Uint16,
        product: *mut Uint16,
        version: *mut Uint16,
        crc16: *mut Uint16,
    );
}

extern "C" {
    /// Get the status of a specified joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to query.
    ///
    /// ### Return value
    /// Returns true if the joystick has been opened, false if it has not; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_JoystickConnected(joystick: *mut SDL_Joystick) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the instance ID of an opened joystick.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    ///
    /// ### Return value
    /// Returns the instance ID of the specified joystick on success or 0 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickID(joystick: *mut SDL_Joystick) -> SDL_JoystickID;
}

extern "C" {
    /// Get the number of general axis controls on a joystick.
    ///
    /// Often, the directional pad on a game controller will either look like 4
    /// separate buttons or a POV hat, and not axes, but all of this is up to the
    /// device and platform.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    ///
    /// ### Return value
    /// Returns the number of axis controls/number of axes on success or -1 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickAxis`]
    /// - [`SDL_GetNumJoystickBalls`]
    /// - [`SDL_GetNumJoystickButtons`]
    /// - [`SDL_GetNumJoystickHats`]
    pub fn SDL_GetNumJoystickAxes(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the number of trackballs on a joystick.
    ///
    /// Joystick trackballs have only relative motion events associated with them
    /// and their state cannot be polled.
    ///
    /// Most joysticks do not have trackballs.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    ///
    /// ### Return value
    /// Returns the number of trackballs on success or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickBall`]
    /// - [`SDL_GetNumJoystickAxes`]
    /// - [`SDL_GetNumJoystickButtons`]
    /// - [`SDL_GetNumJoystickHats`]
    pub fn SDL_GetNumJoystickBalls(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the number of POV hats on a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    ///
    /// ### Return value
    /// Returns the number of POV hats on success or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickHat`]
    /// - [`SDL_GetNumJoystickAxes`]
    /// - [`SDL_GetNumJoystickBalls`]
    /// - [`SDL_GetNumJoystickButtons`]
    pub fn SDL_GetNumJoystickHats(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the number of buttons on a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    ///
    /// ### Return value
    /// Returns the number of buttons on success or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetJoystickButton`]
    /// - [`SDL_GetNumJoystickAxes`]
    /// - [`SDL_GetNumJoystickBalls`]
    /// - [`SDL_GetNumJoystickHats`]
    pub fn SDL_GetNumJoystickButtons(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set the state of joystick event processing.
    ///
    /// If joystick events are disabled, you must call [`SDL_UpdateJoysticks()`]
    /// yourself and check the state of the joystick when you want joystick
    /// information.
    ///
    /// ### Parameters
    /// - `enabled`: whether to process joystick events or not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_JoystickEventsEnabled`]
    /// - [`SDL_UpdateJoysticks`]
    pub fn SDL_SetJoystickEventsEnabled(enabled: ::core::primitive::bool);
}

extern "C" {
    /// Query the state of joystick event processing.
    ///
    /// If joystick events are disabled, you must call [`SDL_UpdateJoysticks()`]
    /// yourself and check the state of the joystick when you want joystick
    /// information.
    ///
    /// ### Return value
    /// Returns true if joystick events are being processed, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetJoystickEventsEnabled`]
    pub fn SDL_JoystickEventsEnabled() -> ::core::primitive::bool;
}

extern "C" {
    /// Update the current state of the open joysticks.
    ///
    /// This is called automatically by the event loop if any joystick events are
    /// enabled.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_UpdateJoysticks();
}

extern "C" {
    /// Get the current state of an axis control on a joystick.
    ///
    /// SDL makes no promises about what part of the joystick any given axis refers
    /// to. Your game should have some sort of configuration UI to let users
    /// specify what each axis should be bound to. Alternately, SDL's higher-level
    /// Game Controller API makes a great effort to apply order to this lower-level
    /// interface, so you know that a specific axis is the "left thumb stick," etc.
    ///
    /// The value returned by [`SDL_GetJoystickAxis()`] is a signed integer (-32768 to
    /// 32767) representing the current position of the axis. It may be necessary
    /// to impose certain tolerances on these values to account for jitter.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    /// - `axis`: the axis to query; the axis indices start at index 0.
    ///
    /// ### Return value
    /// Returns a 16-bit signed integer representing the current position of the
    ///   axis or 0 on failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumJoystickAxes`]
    pub fn SDL_GetJoystickAxis(joystick: *mut SDL_Joystick, axis: ::core::ffi::c_int) -> Sint16;
}

extern "C" {
    /// Get the initial state of an axis control on a joystick.
    ///
    /// The state is a value ranging from -32768 to 32767.
    ///
    /// The axis indices start at index 0.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    /// - `axis`: the axis to query; the axis indices start at index 0.
    /// - `state`: upon return, the initial value is supplied here.
    ///
    /// ### Return value
    /// Returns true if this axis has any initial value, or false if not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickAxisInitialState(
        joystick: *mut SDL_Joystick,
        axis: ::core::ffi::c_int,
        state: *mut Sint16,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the ball axis change since the last poll.
    ///
    /// Trackballs can only return relative motion since the last call to
    /// [`SDL_GetJoystickBall()`], these motion deltas are placed into `dx` and `dy`.
    ///
    /// Most joysticks do not have trackballs.
    ///
    /// ### Parameters
    /// - `joystick`: the [`SDL_Joystick`] to query.
    /// - `ball`: the ball index to query; ball indices start at index 0.
    /// - `dx`: stores the difference in the x axis position since the last poll.
    /// - `dy`: stores the difference in the y axis position since the last poll.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumJoystickBalls`]
    pub fn SDL_GetJoystickBall(
        joystick: *mut SDL_Joystick,
        ball: ::core::ffi::c_int,
        dx: *mut ::core::ffi::c_int,
        dy: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the current state of a POV hat on a joystick.
    ///
    /// The returned value will be one of the `SDL_HAT_*` values.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    /// - `hat`: the hat index to get the state from; indices start at index 0.
    ///
    /// ### Return value
    /// Returns the current hat position.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumJoystickHats`]
    pub fn SDL_GetJoystickHat(joystick: *mut SDL_Joystick, hat: ::core::ffi::c_int) -> Uint8;
}

pub const SDL_HAT_CENTERED: Uint8 = (0x00 as Uint8);

pub const SDL_HAT_UP: Uint8 = (0x01 as Uint8);

pub const SDL_HAT_RIGHT: Uint8 = (0x02 as Uint8);

pub const SDL_HAT_DOWN: Uint8 = (0x04 as Uint8);

pub const SDL_HAT_LEFT: Uint8 = (0x08 as Uint8);

pub const SDL_HAT_RIGHTUP: Uint8 = (SDL_HAT_RIGHT | SDL_HAT_UP);

pub const SDL_HAT_RIGHTDOWN: Uint8 = (SDL_HAT_RIGHT | SDL_HAT_DOWN);

pub const SDL_HAT_LEFTUP: Uint8 = (SDL_HAT_LEFT | SDL_HAT_UP);

pub const SDL_HAT_LEFTDOWN: Uint8 = (SDL_HAT_LEFT | SDL_HAT_DOWN);

extern "C" {
    /// Get the current state of a button on a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: an [`SDL_Joystick`] structure containing joystick information.
    /// - `button`: the button index to get the state from; indices start at
    ///   index 0.
    ///
    /// ### Return value
    /// Returns true if the button is pressed, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumJoystickButtons`]
    pub fn SDL_GetJoystickButton(
        joystick: *mut SDL_Joystick,
        button: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Start a rumble effect.
    ///
    /// Each call to this function cancels any previous rumble effect, and calling
    /// it with 0 intensity stops any rumbling.
    ///
    /// This function requires you to process SDL events or call
    /// [`SDL_UpdateJoysticks()`] to update rumble state.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to vibrate.
    /// - `low_frequency_rumble`: the intensity of the low frequency (left)
    ///   rumble motor, from 0 to 0xFFFF.
    /// - `high_frequency_rumble`: the intensity of the high frequency (right)
    ///   rumble motor, from 0 to 0xFFFF.
    /// - `duration_ms`: the duration of the rumble effect, in milliseconds.
    ///
    /// ### Return value
    /// Returns true, or false if rumble isn't supported on this joystick.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_RumbleJoystick(
        joystick: *mut SDL_Joystick,
        low_frequency_rumble: Uint16,
        high_frequency_rumble: Uint16,
        duration_ms: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Start a rumble effect in the joystick's triggers.
    ///
    /// Each call to this function cancels any previous trigger rumble effect, and
    /// calling it with 0 intensity stops any rumbling.
    ///
    /// Note that this is rumbling of the _triggers_ and not the game controller as
    /// a whole. This is currently only supported on Xbox One controllers. If you
    /// want the (more common) whole-controller rumble, use [`SDL_RumbleJoystick()`]
    /// instead.
    ///
    /// This function requires you to process SDL events or call
    /// [`SDL_UpdateJoysticks()`] to update rumble state.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to vibrate.
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
    /// - [`SDL_RumbleJoystick`]
    pub fn SDL_RumbleJoystickTriggers(
        joystick: *mut SDL_Joystick,
        left_rumble: Uint16,
        right_rumble: Uint16,
        duration_ms: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Update a joystick's LED color.
    ///
    /// An example of a joystick LED is the light on the back of a PlayStation 4's
    /// DualShock 4 controller.
    ///
    /// For joysticks with a single color LED, the maximum of the RGB values will
    /// be used as the LED brightness.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to update.
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
    pub fn SDL_SetJoystickLED(
        joystick: *mut SDL_Joystick,
        red: Uint8,
        green: Uint8,
        blue: Uint8,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Send a joystick specific effect packet.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to affect.
    /// - `data`: the data to send to the joystick.
    /// - `size`: the size of the data to send to the joystick.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SendJoystickEffect(
        joystick: *mut SDL_Joystick,
        data: *const ::core::ffi::c_void,
        size: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Close a joystick previously opened with [`SDL_OpenJoystick()`].
    ///
    /// ### Parameters
    /// - `joystick`: the joystick device to close.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenJoystick`]
    pub fn SDL_CloseJoystick(joystick: *mut SDL_Joystick);
}

extern "C" {
    /// Get the connection state of a joystick.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to query.
    ///
    /// ### Return value
    /// Returns the connection state on success or
    ///   [`SDL_JOYSTICK_CONNECTION_INVALID`] on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickConnectionState(
        joystick: *mut SDL_Joystick,
    ) -> SDL_JoystickConnectionState;
}

extern "C" {
    /// Get the battery state of a joystick.
    ///
    /// You should never take a battery status as absolute truth. Batteries
    /// (especially failing batteries) are delicate hardware, and the values
    /// reported here are best estimates based on what that hardware reports. It's
    /// not uncommon for older batteries to lose stored power much faster than it
    /// reports, or completely drain when reporting it has 20 percent left, etc.
    ///
    /// ### Parameters
    /// - `joystick`: the joystick to query.
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
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetJoystickPowerInfo(
        joystick: *mut SDL_Joystick,
        percent: *mut ::core::ffi::c_int,
    ) -> SDL_PowerState;
}

/// The joystick structure used to identify an SDL joystick.
///
/// This is opaque data.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Joystick {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
