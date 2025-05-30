//! Metadata for items in the `crate::gamepad` module

use super::*;

pub const METADATA_SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN: Property = Property {
    module: "gamepad",
    name: "SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN",
    short_name: "GAMEPAD_CAP_MONO_LED_BOOLEAN",
    value: crate::gamepad::SDL_PROP_GAMEPAD_CAP_MONO_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN: Property = Property {
    module: "gamepad",
    name: "SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN",
    short_name: "GAMEPAD_CAP_RGB_LED_BOOLEAN",
    value: crate::gamepad::SDL_PROP_GAMEPAD_CAP_RGB_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN: Property = Property {
    module: "gamepad",
    name: "SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN",
    short_name: "GAMEPAD_CAP_PLAYER_LED_BOOLEAN",
    value: crate::gamepad::SDL_PROP_GAMEPAD_CAP_PLAYER_LED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN: Property = Property {
    module: "gamepad",
    name: "SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN",
    short_name: "GAMEPAD_CAP_RUMBLE_BOOLEAN",
    value: crate::gamepad::SDL_PROP_GAMEPAD_CAP_RUMBLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN: Property = Property {
    module: "gamepad",
    name: "SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN",
    short_name: "GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN",
    value: crate::gamepad::SDL_PROP_GAMEPAD_CAP_TRIGGER_RUMBLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_GamepadType: Group = Group {
    module: "gamepad",
    kind: GroupKind::Enum,
    name: "SDL_GamepadType",
    short_name: "GamepadType",
    doc: Some("Standard gamepad types.\n\nThis type does not necessarily map to first-party controllers from\nMicrosoft/Sony/Nintendo; in many cases, third-party controllers can report\nas these, either because they were designed for a specific console, or they\nsimply most closely match that console's controllers (does it have A/B/X/Y\nbuttons or X/O/Square/Triangle? Does it have a touchpad? etc).\n"),
    available_since: None,
    values: &[
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_UNKNOWN",
            short_name: "UNKNOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_STANDARD",
            short_name: "STANDARD",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_XBOX360",
            short_name: "XBOX360",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_XBOXONE",
            short_name: "XBOXONE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_PS3",
            short_name: "PS3",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_PS4",
            short_name: "PS4",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_PS5",
            short_name: "PS5",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_PRO",
            short_name: "NINTENDO_SWITCH_PRO",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_LEFT",
            short_name: "NINTENDO_SWITCH_JOYCON_LEFT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT",
            short_name: "NINTENDO_SWITCH_JOYCON_RIGHT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_NINTENDO_SWITCH_JOYCON_PAIR",
            short_name: "NINTENDO_SWITCH_JOYCON_PAIR",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_TYPE_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GamepadButton: Group = Group {
    module: "gamepad",
    kind: GroupKind::Enum,
    name: "SDL_GamepadButton",
    short_name: "GamepadButton",
    doc: Some("The list of buttons available on a gamepad\n\nFor controllers that use a diamond pattern for the face buttons, the\nsouth/east/west/north buttons below correspond to the locations in the\ndiamond pattern. For Xbox controllers, this would be A/B/X/Y, for Nintendo\nSwitch controllers, this would be B/A/Y/X, for PlayStation controllers this\nwould be Cross/Circle/Square/Triangle.\n\nFor controllers that don't use a diamond pattern for the face buttons, the\nsouth/east/west/north buttons indicate the buttons labeled A, B, C, D, or\n1, 2, 3, 4, or for controllers that aren't labeled, they are the primary,\nsecondary, etc. buttons.\n\nThe activate action is often the south button and the cancel action is\noften the east button, but in some regions this is reversed, so your game\nshould allow remapping actions based on user preferences.\n\nYou can query the labels for the face buttons using\n[`SDL_GetGamepadButtonLabel()`]\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_SOUTH",
            short_name: "SOUTH",
            doc: Some("Bottom face button (e.g. Xbox A button)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_EAST",
            short_name: "EAST",
            doc: Some("Right face button (e.g. Xbox B button)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_WEST",
            short_name: "WEST",
            doc: Some("Left face button (e.g. Xbox X button)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_NORTH",
            short_name: "NORTH",
            doc: Some("Top face button (e.g. Xbox Y button)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_BACK",
            short_name: "BACK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_GUIDE",
            short_name: "GUIDE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_START",
            short_name: "START",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LEFT_STICK",
            short_name: "LEFT_STICK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_RIGHT_STICK",
            short_name: "RIGHT_STICK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LEFT_SHOULDER",
            short_name: "LEFT_SHOULDER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER",
            short_name: "RIGHT_SHOULDER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_DPAD_UP",
            short_name: "DPAD_UP",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_DPAD_DOWN",
            short_name: "DPAD_DOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_DPAD_LEFT",
            short_name: "DPAD_LEFT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_DPAD_RIGHT",
            short_name: "DPAD_RIGHT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC1",
            short_name: "MISC1",
            doc: Some("Additional button (e.g. Xbox Series X share button, PS5 microphone button, Nintendo Switch Pro capture button, Amazon Luna microphone button, Google Stadia capture button)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_RIGHT_PADDLE1",
            short_name: "RIGHT_PADDLE1",
            doc: Some("Upper or primary paddle, under your right hand (e.g. Xbox Elite paddle P1)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LEFT_PADDLE1",
            short_name: "LEFT_PADDLE1",
            doc: Some("Upper or primary paddle, under your left hand (e.g. Xbox Elite paddle P3)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_RIGHT_PADDLE2",
            short_name: "RIGHT_PADDLE2",
            doc: Some("Lower or secondary paddle, under your right hand (e.g. Xbox Elite paddle P2)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LEFT_PADDLE2",
            short_name: "LEFT_PADDLE2",
            doc: Some("Lower or secondary paddle, under your left hand (e.g. Xbox Elite paddle P4)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_TOUCHPAD",
            short_name: "TOUCHPAD",
            doc: Some("PS4/PS5 touchpad button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC2",
            short_name: "MISC2",
            doc: Some("Additional button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC3",
            short_name: "MISC3",
            doc: Some("Additional button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC4",
            short_name: "MISC4",
            doc: Some("Additional button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC5",
            short_name: "MISC5",
            doc: Some("Additional button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_MISC6",
            short_name: "MISC6",
            doc: Some("Additional button\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GamepadButtonLabel: Group = Group {
    module: "gamepad",
    kind: GroupKind::Enum,
    name: "SDL_GamepadButtonLabel",
    short_name: "GamepadButtonLabel",
    doc: Some("The set of gamepad button labels\n\nThis isn't a complete set, just the face buttons to make it easy to show\nbutton prompts.\n\nFor a complete set, you should look at the button and gamepad type and have\na set of symbols that work well with your art style.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_UNKNOWN",
            short_name: "UNKNOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_A",
            short_name: "A",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_B",
            short_name: "B",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_X",
            short_name: "X",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_Y",
            short_name: "Y",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_CROSS",
            short_name: "CROSS",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_CIRCLE",
            short_name: "CIRCLE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_SQUARE",
            short_name: "SQUARE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BUTTON_LABEL_TRIANGLE",
            short_name: "TRIANGLE",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GamepadAxis: Group = Group {
    module: "gamepad",
    kind: GroupKind::Enum,
    name: "SDL_GamepadAxis",
    short_name: "GamepadAxis",
    doc: Some("The list of axes available on a gamepad\n\nThumbstick axis values range from [`SDL_JOYSTICK_AXIS_MIN`] to\n[`SDL_JOYSTICK_AXIS_MAX`], and are centered within ~8000 of zero, though\nadvanced UI will allow users to set or autodetect the dead zone, which\nvaries between gamepads.\n\nTrigger axis values range from 0 (released) to [`SDL_JOYSTICK_AXIS_MAX`] (fully\npressed) when reported by [`SDL_GetGamepadAxis()`]. Note that this is not the\nsame range that will be reported by the lower-level [`SDL_GetJoystickAxis()`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_LEFTX",
            short_name: "LEFTX",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_LEFTY",
            short_name: "LEFTY",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_RIGHTX",
            short_name: "RIGHTX",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_RIGHTY",
            short_name: "RIGHTY",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_LEFT_TRIGGER",
            short_name: "LEFT_TRIGGER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_RIGHT_TRIGGER",
            short_name: "RIGHT_TRIGGER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_AXIS_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GamepadBindingType: Group = Group {
    module: "gamepad",
    kind: GroupKind::Enum,
    name: "SDL_GamepadBindingType",
    short_name: "GamepadBindingType",
    doc: Some("Types of gamepad control bindings.\n\nA gamepad is a collection of bindings that map arbitrary joystick buttons,\naxes and hat switches to specific positions on a generic console-style\ngamepad. This enum is used as part of [`SDL_GamepadBinding`] to specify those\nmappings.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GAMEPAD_BINDTYPE_NONE",
            short_name: "NONE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BINDTYPE_BUTTON",
            short_name: "BUTTON",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BINDTYPE_AXIS",
            short_name: "AXIS",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GAMEPAD_BINDTYPE_HAT",
            short_name: "HAT",
            doc: None,
            available_since: None,
        },
    ],
};
