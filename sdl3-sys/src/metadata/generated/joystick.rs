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
        "An enum of some common joystick types.\n\nIn some cases, SDL can identify a low-level joystick as being a certain\ntype of device, and will report it through [`SDL_GetJoystickType`] (or\n[`SDL_GetJoystickTypeForID`]).\n\nThis is by no means a complete list of everything that can be plugged into\na computer.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
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
