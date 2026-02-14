//! Metadata for items in the `crate::joystick` module

use super::*;

pub const METADATA_SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN: Property = Property {
    module: "joystick",
    name: "SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN",
    short_name: "JOYSTICK_CAP_MONO_LED_BOOLEAN",
    value: crate::joystick::SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN: Property = Property {
    module: "joystick",
    name: "SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN",
    short_name: "JOYSTICK_CAP_RGB_LED_BOOLEAN",
    value: crate::joystick::SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN: Property = Property {
    module: "joystick",
    name: "SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN",
    short_name: "JOYSTICK_CAP_PLAYER_LED_BOOLEAN",
    value: crate::joystick::SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN: Property = Property {
    module: "joystick",
    name: "SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN",
    short_name: "JOYSTICK_CAP_RUMBLE_BOOLEAN",
    value: crate::joystick::SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN: Property = Property {
    module: "joystick",
    name: "SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN",
    short_name: "JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN",
    value: crate::joystick::SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_JoystickID: Group = Group {
    module: "joystick",
    kind: GroupKind::Id,
    name: "SDL_JoystickID",
    short_name: "JoystickID",
    doc: Some(
        "This is a unique ID for a joystick for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the joystick is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_JoystickType: Group = Group {
    module: "joystick",
    kind: GroupKind::Enum,
    name: "SDL_JoystickType",
    short_name: "JoystickType",
    doc: Some(
        "An enum of some common joystick types.\n\nIn some cases, SDL can identify a low-level joystick as being a certain\ntype of device, and will report it through [`SDL_GetJoystickType`] (or\n[`SDL_GetJoystickTypeForID`]).\n\nThis is by no means a complete list of everything that can be plugged into\na computer.\n\nYou may refer to\n[XInput Controller Types](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-and-controller-subtypes)\ntable for a general understanding of each joystick type.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_UNKNOWN",
            short_name: "UNKNOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_GAMEPAD",
            short_name: "GAMEPAD",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_WHEEL",
            short_name: "WHEEL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_ARCADE_STICK",
            short_name: "ARCADE_STICK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_FLIGHT_STICK",
            short_name: "FLIGHT_STICK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_DANCE_PAD",
            short_name: "DANCE_PAD",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_GUITAR",
            short_name: "GUITAR",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_DRUM_KIT",
            short_name: "DRUM_KIT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_ARCADE_PAD",
            short_name: "ARCADE_PAD",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_THROTTLE",
            short_name: "THROTTLE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_TYPE_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_JoystickConnectionState: Group = Group {
    module: "joystick",
    kind: GroupKind::Enum,
    name: "SDL_JoystickConnectionState",
    short_name: "JoystickConnectionState",
    doc: Some(
        "Possible connection states for a joystick device.\n\nThis is used by [`SDL_GetJoystickConnectionState`] to report how a device is\nconnected to the system.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_JOYSTICK_CONNECTION_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_CONNECTION_UNKNOWN",
            short_name: "UNKNOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_CONNECTION_WIRED",
            short_name: "WIRED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_JOYSTICK_CONNECTION_WIRELESS",
            short_name: "WIRELESS",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_VirtualJoystickTouchpadDesc: Struct = Struct {
    module: "joystick",
    kind: StructKind::Struct,
    name: "SDL_VirtualJoystickTouchpadDesc",
    doc: Some(
        "The structure that describes a virtual joystick touchpad.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_VirtualJoystickDesc`]\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "nfingers",
            doc: Some("the number of simultaneous fingers on this touchpad\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "padding",
            doc: None,
            available_since: None,
            ty: "[Uint16; 3]",
        },
    ],
};
pub const METADATA_SDL_VirtualJoystickSensorDesc: Struct = Struct {
    module: "joystick",
    kind: StructKind::Struct,
    name: "SDL_VirtualJoystickSensorDesc",
    doc: Some(
        "The structure that describes a virtual joystick sensor.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_VirtualJoystickDesc`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("the type of this sensor\n"),
            available_since: None,
            ty: "SDL_SensorType",
        },
        Field {
            name: "rate",
            doc: Some("the update frequency of this sensor, may be 0.0f\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_VirtualJoystickDesc: Struct = Struct {
    module: "joystick",
    kind: StructKind::Struct,
    name: "SDL_VirtualJoystickDesc",
    doc: Some(
        "The structure that describes a virtual joystick.\n\nThis structure should be initialized using [`SDL_INIT_INTERFACE()`]. All\nelements of this structure are optional.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_AttachVirtualJoystick`]\n- [`SDL_INIT_INTERFACE`]\n- [`SDL_VirtualJoystickSensorDesc`]\n- [`SDL_VirtualJoystickTouchpadDesc`]\n\n## Notes for `sdl3-sys`\n- This interface struct can be initialized with [`SDL_VirtualJoystickDesc::new()`] or `Default::default()`.\n- This struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "version",
            doc: Some("the version of this interface\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "r#type",
            doc: Some("[`SDL_JoystickType`]\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "padding",
            doc: Some("unused\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "vendor_id",
            doc: Some("the USB vendor ID of this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "product_id",
            doc: Some("the USB product ID of this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "naxes",
            doc: Some("the number of axes on this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "nbuttons",
            doc: Some("the number of buttons on this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "nballs",
            doc: Some("the number of balls on this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "nhats",
            doc: Some("the number of hats on this joystick\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "ntouchpads",
            doc: Some(
                "the number of touchpads on this joystick, requires `touchpads` to point at valid descriptions\n",
            ),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "nsensors",
            doc: Some(
                "the number of sensors on this joystick, requires `sensors` to point at valid descriptions\n",
            ),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "padding2",
            doc: Some("unused\n"),
            available_since: None,
            ty: "[Uint16; 2]",
        },
        Field {
            name: "button_mask",
            doc: Some(
                "A mask of which buttons are valid for this controller\ne.g. (1 << [`SDL_GAMEPAD_BUTTON_SOUTH`])\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "axis_mask",
            doc: Some(
                "A mask of which axes are valid for this controller\ne.g. (1 << [`SDL_GAMEPAD_AXIS_LEFTX`])\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "name",
            doc: Some("the name of the joystick\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "touchpads",
            doc: Some(
                "A pointer to an array of touchpad descriptions, required if `ntouchpads` is > 0\n",
            ),
            available_since: None,
            ty: "*const SDL_VirtualJoystickTouchpadDesc",
        },
        Field {
            name: "sensors",
            doc: Some(
                "A pointer to an array of sensor descriptions, required if `nsensors` is > 0\n",
            ),
            available_since: None,
            ty: "*const SDL_VirtualJoystickSensorDesc",
        },
        Field {
            name: "userdata",
            doc: Some("User data pointer passed to callbacks\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "Update",
            doc: Some("Called when the joystick state should be updated\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void)>",
        },
        Field {
            name: "SetPlayerIndex",
            doc: Some("Called when the player index is set\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, player_index: ::core::ffi::c_int)>",
        },
        Field {
            name: "Rumble",
            doc: Some("Implements [`SDL_RumbleJoystick()`]\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, low_frequency_rumble: Uint16, high_frequency_rumble: Uint16) -> ::core::primitive::bool>",
        },
        Field {
            name: "RumbleTriggers",
            doc: Some("Implements [`SDL_RumbleJoystickTriggers()`]\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, left_rumble: Uint16, right_rumble: Uint16) -> ::core::primitive::bool>",
        },
        Field {
            name: "SetLED",
            doc: Some("Implements [`SDL_SetJoystickLED()`]\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, red: Uint8, green: Uint8, blue: Uint8) -> ::core::primitive::bool>",
        },
        Field {
            name: "SendEffect",
            doc: Some("Implements [`SDL_SendJoystickEffect()`]\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, size: ::core::ffi::c_int) -> ::core::primitive::bool>",
        },
        Field {
            name: "SetSensorsEnabled",
            doc: Some("Implements [`SDL_SetGamepadSensorEnabled()`]\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, enabled: ::core::primitive::bool) -> ::core::primitive::bool>",
        },
        Field {
            name: "Cleanup",
            doc: Some("Cleans up the userdata when the joystick is detached\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void)>",
        },
    ],
};
