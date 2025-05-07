//! Metadata for items in the `crate::haptic` module

use super::*;

pub const METADATA_SDL_HapticEffectType: Group = Group {
    module: "haptic",
    kind: GroupKind::Flags,
    name: "SDL_HapticEffectType",
    short_name: "HapticEffectType",
    doc: Some("* Type of haptic effect.\n"),
    available_since: None,
    values: &[
        GroupValue {
            name: "SDL_HAPTIC_CONSTANT",
            short_name: "CONSTANT",
            doc: Some(
                "Constant effect supported.\n\nConstant haptic effect.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticCondition`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SINE",
            short_name: "SINE",
            doc: Some(
                "Sine wave effect supported.\n\nPeriodic haptic effect that simulates sine waves.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticPeriodic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SQUARE",
            short_name: "SQUARE",
            doc: Some(
                "Square wave effect supported.\n\nPeriodic haptic effect that simulates square waves.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticPeriodic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_TRIANGLE",
            short_name: "TRIANGLE",
            doc: Some(
                "Triangle wave effect supported.\n\nPeriodic haptic effect that simulates triangular waves.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticPeriodic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SAWTOOTHUP",
            short_name: "SAWTOOTHUP",
            doc: Some(
                "Sawtoothup wave effect supported.\n\nPeriodic haptic effect that simulates saw tooth up waves.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticPeriodic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SAWTOOTHDOWN",
            short_name: "SAWTOOTHDOWN",
            doc: Some(
                "Sawtoothdown wave effect supported.\n\nPeriodic haptic effect that simulates saw tooth down waves.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticPeriodic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_RAMP",
            short_name: "RAMP",
            doc: Some(
                "Ramp effect supported.\n\nRamp haptic effect.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticRamp`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SPRING",
            short_name: "SPRING",
            doc: Some(
                "Spring effect supported - uses axes position.\n\nCondition haptic effect that simulates a spring. Effect is based on the\naxes position.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticCondition`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_DAMPER",
            short_name: "DAMPER",
            doc: Some(
                "Damper effect supported - uses axes velocity.\n\nCondition haptic effect that simulates dampening. Effect is based on the\naxes velocity.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticCondition`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_INERTIA",
            short_name: "INERTIA",
            doc: Some(
                "Inertia effect supported - uses axes acceleration.\n\nCondition haptic effect that simulates inertia. Effect is based on the axes\nacceleration.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticCondition`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_FRICTION",
            short_name: "FRICTION",
            doc: Some(
                "Friction effect supported - uses axes movement.\n\nCondition haptic effect that simulates friction. Effect is based on the\naxes movement.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticCondition`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_LEFTRIGHT",
            short_name: "LEFTRIGHT",
            doc: Some(
                "Left/Right effect supported.\n\nHaptic effect for direct control over high/low frequency motors.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticLeftRight`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_RESERVED1",
            short_name: "RESERVED1",
            doc: Some(
                "Reserved for future use.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_RESERVED2",
            short_name: "RESERVED2",
            doc: Some(
                "Reserved for future use.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_RESERVED3",
            short_name: "RESERVED3",
            doc: Some(
                "Reserved for future use.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_CUSTOM",
            short_name: "CUSTOM",
            doc: Some(
                "Custom effect is supported.\n\nUser defined custom haptic effect.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_GAIN",
            short_name: "GAIN",
            doc: Some(
                "Device can set global gain.\n\nDevice supports setting the global gain.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_SetHapticGain`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_AUTOCENTER",
            short_name: "AUTOCENTER",
            doc: Some(
                "Device can set autocenter.\n\nDevice supports setting autocenter.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_SetHapticAutocenter`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_STATUS",
            short_name: "STATUS",
            doc: Some(
                "Device can be queried for effect status.\n\nDevice supports querying effect status.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetHapticEffectStatus`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_PAUSE",
            short_name: "PAUSE",
            doc: Some(
                "Device can be paused.\n\nDevices supports being paused.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_PauseHaptic`]\n- [`SDL_ResumeHaptic`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
    ],
};
pub const METADATA_SDL_HapticDirectionType: Group = Group {
    module: "haptic",
    kind: GroupKind::Enum,
    name: "SDL_HapticDirectionType",
    short_name: "HapticDirectionType",
    doc: Some("* Type of coordinates used for haptic direction.\n"),
    available_since: None,
    values: &[
        GroupValue {
            name: "SDL_HAPTIC_POLAR",
            short_name: "POLAR",
            doc: Some(
                "Uses polar coordinates for the direction.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticDirection`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_CARTESIAN",
            short_name: "CARTESIAN",
            doc: Some(
                "Uses cartesian coordinates for the direction.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticDirection`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_SPHERICAL",
            short_name: "SPHERICAL",
            doc: Some(
                "Uses spherical coordinates for the direction.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticDirection`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_HAPTIC_STEERING_AXIS",
            short_name: "STEERING_AXIS",
            doc: Some(
                "Use this value to play an effect on the steering wheel axis.\n\nThis provides better compatibility across platforms and devices as SDL will\nguess the correct axis.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticDirection`]\n",
            ),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
    ],
};
pub const METADATA_SDL_HapticEffectID: Group = Group {
    module: "haptic",
    kind: GroupKind::Id,
    name: "SDL_HapticEffectID",
    short_name: "HapticEffectID",
    doc: Some(
        "ID for haptic effects.\n\nThis is -1 if the ID is invalid.\n\n## See also\n- [`SDL_CreateHapticEffect`]\n",
    ),
    available_since: None,
    values: &[],
};
pub const METADATA_SDL_HapticID: Group = Group {
    module: "haptic",
    kind: GroupKind::Id,
    name: "SDL_HapticID",
    short_name: "HapticID",
    doc: Some(
        "This is a unique ID for a haptic device for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the haptic device is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
