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
pub const METADATA_SDL_HapticDirection: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticDirection",
    doc: Some(
        "Structure that represents a haptic direction.\n\nThis is the direction where the force comes from, instead of the direction\nin which the force is exerted.\n\nDirections can be specified by:\n\n- [`SDL_HAPTIC_POLAR`] : Specified by polar coordinates.\n- [`SDL_HAPTIC_CARTESIAN`] : Specified by cartesian coordinates.\n- [`SDL_HAPTIC_SPHERICAL`] : Specified by spherical coordinates.\n\nCardinal directions of the haptic device are relative to the positioning of\nthe device. North is considered to be away from the user.\n\nThe following diagram represents the cardinal directions:\n\n```text\n.--.\n|__| .-------.\n|=.| |.-----.|\n|--| ||     ||\n|  | |'-----'|\n|__|~')_____('\n[ COMPUTER ]\n\n\nNorth (0,-1)\n^\n|\n|\n(-1,0)  West <----[ HAPTIC ]----> East (1,0)\n|\n|\nv\nSouth (0,1)\n\n\n[ USER ]\n\\|||/\n(o o)\n---ooO-(_)-Ooo---\n```\n\nIf type is [`SDL_HAPTIC_POLAR`], direction is encoded by hundredths of a degree\nstarting north and turning clockwise. [`SDL_HAPTIC_POLAR`] only uses the first\n`dir` parameter. The cardinal directions would be:\n\n- North: 0 (0 degrees)\n- East: 9000 (90 degrees)\n- South: 18000 (180 degrees)\n- West: 27000 (270 degrees)\n\nIf type is [`SDL_HAPTIC_CARTESIAN`], direction is encoded by three positions (X\naxis, Y axis and Z axis (with 3 axes)). [`SDL_HAPTIC_CARTESIAN`] uses the first\nthree `dir` parameters. The cardinal directions would be:\n\n- North: 0,-1, 0\n- East: 1, 0, 0\n- South: 0, 1, 0\n- West: -1, 0, 0\n\nThe Z axis represents the height of the effect if supported, otherwise it's\nunused. In cartesian encoding (1, 2) would be the same as (2, 4), you can\nuse any multiple you want, only the direction matters.\n\nIf type is [`SDL_HAPTIC_SPHERICAL`], direction is encoded by two rotations. The\nfirst two `dir` parameters are used. The `dir` parameters are as follows\n(all values are in hundredths of degrees):\n\n- Degrees from (1, 0) rotated towards (0, 1).\n- Degrees towards (0, 0, 1) (device needs at least 3 axes).\n\nExample of force coming from the south with all encodings (force coming\nfrom the south means the user will have to pull the stick to counteract):\n\n```c\nSDL_HapticDirection direction;\n\n// Cartesian directions\ndirection.type = SDL_HAPTIC_CARTESIAN; // Using cartesian direction encoding.\ndirection.dir[0] = 0; // X position\ndirection.dir[1] = 1; // Y position\n// Assuming the device has 2 axes, we don't need to specify third parameter.\n\n// Polar directions\ndirection.type = SDL_HAPTIC_POLAR; // We'll be using polar direction encoding.\ndirection.dir[0] = 18000; // Polar only uses first parameter\n\n// Spherical coordinates\ndirection.type = SDL_HAPTIC_SPHERICAL; // Spherical encoding\ndirection.dir[0] = 9000; // Since we only have two axes we don't need more parameters.\n```\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_POLAR`]\n- [`SDL_HAPTIC_CARTESIAN`]\n- [`SDL_HAPTIC_SPHERICAL`]\n- [`SDL_HAPTIC_STEERING_AXIS`]\n- [`SDL_HapticEffect`]\n- [`SDL_GetNumHapticAxes`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("The type of encoding.\n"),
            available_since: None,
            ty: "SDL_HapticDirectionType",
        },
        Field {
            name: "dir",
            doc: Some("The encoded direction.\n"),
            available_since: None,
            ty: "[Sint32; 3]",
        },
    ],
};
pub const METADATA_SDL_HapticConstant: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticConstant",
    doc: Some(
        "A structure containing a template for a Constant effect.\n\nThis struct is exclusively for the [`SDL_HAPTIC_CONSTANT`] effect.\n\nA constant effect applies a constant force in the specified direction to\nthe joystick.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_CONSTANT`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_HAPTIC_CONSTANT`]\n"),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "direction",
            doc: Some("Direction of the effect.\n"),
            available_since: None,
            ty: "SDL_HapticDirection",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "delay",
            doc: Some("Delay before starting the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "button",
            doc: Some("Button that triggers the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "interval",
            doc: Some("How soon it can be triggered again after button.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "level",
            doc: Some("Strength of the constant effect.\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "attack_length",
            doc: Some("Duration of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "attack_level",
            doc: Some("Level at the start of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_length",
            doc: Some("Duration of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_level",
            doc: Some("Level at the end of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_HapticPeriodic: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticPeriodic",
    doc: Some(
        "A structure containing a template for a Periodic effect.\n\nThe struct handles the following effects:\n\n- [`SDL_HAPTIC_SINE`]\n- [`SDL_HAPTIC_SQUARE`]\n- [`SDL_HAPTIC_TRIANGLE`]\n- [`SDL_HAPTIC_SAWTOOTHUP`]\n- [`SDL_HAPTIC_SAWTOOTHDOWN`]\n\nA periodic effect consists in a wave-shaped effect that repeats itself over\ntime. The type determines the shape of the wave and the parameters\ndetermine the dimensions of the wave.\n\nPhase is given by hundredth of a degree meaning that giving the phase a\nvalue of 9000 will displace it 25% of its period. Here are sample values:\n\n- 0: No phase displacement.\n- 9000: Displaced 25% of its period.\n- 18000: Displaced 50% of its period.\n- 27000: Displaced 75% of its period.\n- 36000: Displaced 100% of its period, same as 0, but 0 is preferred.\n\nExamples:\n\n```text\nSDL_HAPTIC_SINE\n__      __      __      __\n/  \\    /  \\    /  \\    /\n/    \\__/    \\__/    \\__/\n\nSDL_HAPTIC_SQUARE\n__    __    __    __    __\n|  |  |  |  |  |  |  |  |  |\n|  |__|  |__|  |__|  |__|  |\n\nSDL_HAPTIC_TRIANGLE\n/\\    /\\    /\\    /\\    /\\\n/  \\  /  \\  /  \\  /  \\  /\n/    \\/    \\/    \\/    \\/\n\nSDL_HAPTIC_SAWTOOTHUP\n/|  /|  /|  /|  /|  /|  /|\n/ | / | / | / | / | / | / |\n/  |/  |/  |/  |/  |/  |/  |\n\nSDL_HAPTIC_SAWTOOTHDOWN\n\\  |\\  |\\  |\\  |\\  |\\  |\\  |\n\\ | \\ | \\ | \\ | \\ | \\ | \\ |\n\\|  \\|  \\|  \\|  \\|  \\|  \\|\n```\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_SINE`]\n- [`SDL_HAPTIC_SQUARE`]\n- [`SDL_HAPTIC_TRIANGLE`]\n- [`SDL_HAPTIC_SAWTOOTHUP`]\n- [`SDL_HAPTIC_SAWTOOTHDOWN`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_HAPTIC_SINE`], [`SDL_HAPTIC_SQUARE`]\n[`SDL_HAPTIC_TRIANGLE`], [`SDL_HAPTIC_SAWTOOTHUP`] or\n[`SDL_HAPTIC_SAWTOOTHDOWN`]\n",
            ),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "direction",
            doc: Some("Direction of the effect.\n"),
            available_since: None,
            ty: "SDL_HapticDirection",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "delay",
            doc: Some("Delay before starting the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "button",
            doc: Some("Button that triggers the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "interval",
            doc: Some("How soon it can be triggered again after button.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "period",
            doc: Some("Period of the wave.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "magnitude",
            doc: Some("Peak value; if negative, equivalent to 180 degrees extra phase shift.\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "offset",
            doc: Some("Mean value of the wave.\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "phase",
            doc: Some("Positive phase shift given by hundredth of a degree.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "attack_length",
            doc: Some("Duration of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "attack_level",
            doc: Some("Level at the start of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_length",
            doc: Some("Duration of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_level",
            doc: Some("Level at the end of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_HapticCondition: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticCondition",
    doc: Some(
        "A structure containing a template for a Condition effect.\n\nThe struct handles the following effects:\n\n- [`SDL_HAPTIC_SPRING`]\\: Effect based on axes position.\n- [`SDL_HAPTIC_DAMPER`]\\: Effect based on axes velocity.\n- [`SDL_HAPTIC_INERTIA`]\\: Effect based on axes acceleration.\n- [`SDL_HAPTIC_FRICTION`]\\: Effect based on axes movement.\n\nDirection is handled by condition internals instead of a direction member.\nThe condition effect specific members have three parameters. The first\nrefers to the X axis, the second refers to the Y axis and the third refers\nto the Z axis. The right terms refer to the positive side of the axis and\nthe left terms refer to the negative side of the axis. Please refer to the\n[`SDL_HapticDirection`] diagram for which side is positive and which is\nnegative.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticDirection`]\n- [`SDL_HAPTIC_SPRING`]\n- [`SDL_HAPTIC_DAMPER`]\n- [`SDL_HAPTIC_INERTIA`]\n- [`SDL_HAPTIC_FRICTION`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_HAPTIC_SPRING`], [`SDL_HAPTIC_DAMPER`],\n[`SDL_HAPTIC_INERTIA`] or [`SDL_HAPTIC_FRICTION`]\n",
            ),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "direction",
            doc: Some("Direction of the effect.\n"),
            available_since: None,
            ty: "SDL_HapticDirection",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "delay",
            doc: Some("Delay before starting the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "button",
            doc: Some("Button that triggers the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "interval",
            doc: Some("How soon it can be triggered again after button.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "right_sat",
            doc: Some("Level when joystick is to the positive side; max 0xFFFF.\n"),
            available_since: None,
            ty: "[Uint16; 3]",
        },
        Field {
            name: "left_sat",
            doc: Some("Level when joystick is to the negative side; max 0xFFFF.\n"),
            available_since: None,
            ty: "[Uint16; 3]",
        },
        Field {
            name: "right_coeff",
            doc: Some("How fast to increase the force towards the positive side.\n"),
            available_since: None,
            ty: "[Sint16; 3]",
        },
        Field {
            name: "left_coeff",
            doc: Some("How fast to increase the force towards the negative side.\n"),
            available_since: None,
            ty: "[Sint16; 3]",
        },
        Field {
            name: "deadband",
            doc: Some("Size of the dead zone; max 0xFFFF: whole axis-range when 0-centered.\n"),
            available_since: None,
            ty: "[Uint16; 3]",
        },
        Field {
            name: "center",
            doc: Some("Position of the dead zone.\n"),
            available_since: None,
            ty: "[Sint16; 3]",
        },
    ],
};
pub const METADATA_SDL_HapticRamp: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticRamp",
    doc: Some(
        "A structure containing a template for a Ramp effect.\n\nThis struct is exclusively for the [`SDL_HAPTIC_RAMP`] effect.\n\nThe ramp effect starts at start strength and ends at end strength. It\naugments in linear fashion. If you use attack and fade with a ramp the\neffects get added to the ramp effect making the effect become quadratic\ninstead of linear.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_RAMP`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_HAPTIC_RAMP`]\n"),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "direction",
            doc: Some("Direction of the effect.\n"),
            available_since: None,
            ty: "SDL_HapticDirection",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "delay",
            doc: Some("Delay before starting the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "button",
            doc: Some("Button that triggers the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "interval",
            doc: Some("How soon it can be triggered again after button.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "start",
            doc: Some("Beginning strength level.\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "end",
            doc: Some("Ending strength level.\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "attack_length",
            doc: Some("Duration of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "attack_level",
            doc: Some("Level at the start of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_length",
            doc: Some("Duration of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_level",
            doc: Some("Level at the end of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_HapticLeftRight: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticLeftRight",
    doc: Some(
        "A structure containing a template for a Left/Right effect.\n\nThis struct is exclusively for the [`SDL_HAPTIC_LEFTRIGHT`] effect.\n\nThe Left/Right effect is used to explicitly control the large and small\nmotors, commonly found in modern game controllers. The small (right) motor\nis high frequency, and the large (left) motor is low frequency.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_LEFTRIGHT`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_HAPTIC_LEFTRIGHT`]\n"),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect in milliseconds.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "large_magnitude",
            doc: Some("Control of the large controller motor.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "small_magnitude",
            doc: Some("Control of the small controller motor.\n"),
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_HapticCustom: Struct = Struct {
    module: "haptic",
    kind: StructKind::Struct,
    name: "SDL_HapticCustom",
    doc: Some(
        "A structure containing a template for the [`SDL_HAPTIC_CUSTOM`] effect.\n\nThis struct is exclusively for the [`SDL_HAPTIC_CUSTOM`] effect.\n\nA custom force feedback effect is much like a periodic effect, where the\napplication can define its exact shape. You will have to allocate the data\nyourself. Data should consist of channels * samples Uint16 samples.\n\nIf channels is one, the effect is rotated using the defined direction.\nOtherwise it uses the samples in data for the different axes.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HAPTIC_CUSTOM`]\n- [`SDL_HapticEffect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_HAPTIC_CUSTOM`]\n"),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "direction",
            doc: Some("Direction of the effect.\n"),
            available_since: None,
            ty: "SDL_HapticDirection",
        },
        Field {
            name: "length",
            doc: Some("Duration of the effect.\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "delay",
            doc: Some("Delay before starting the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "button",
            doc: Some("Button that triggers the effect.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "interval",
            doc: Some("How soon it can be triggered again after button.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "channels",
            doc: Some("Axes to use, minimum of one.\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "period",
            doc: Some("Sample periods.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "samples",
            doc: Some("Amount of samples.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "data",
            doc: Some("Should contain channels*samples items.\n"),
            available_since: None,
            ty: "*mut Uint16",
        },
        Field {
            name: "attack_length",
            doc: Some("Duration of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "attack_level",
            doc: Some("Level at the start of the attack.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_length",
            doc: Some("Duration of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "fade_level",
            doc: Some("Level at the end of the fade.\n"),
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_HapticEffect: Struct = Struct {
    module: "haptic",
    kind: StructKind::Union,
    name: "SDL_HapticEffect",
    doc: Some(
        "The generic template for any haptic effect.\n\nAll values max at 32767 (0x7FFF). Signed values also can be negative. Time\nvalues unless specified otherwise are in milliseconds.\n\nYou can also pass [`SDL_HAPTIC_INFINITY`] to length instead of a 0-32767 value.\nNeither delay, interval, attack_length nor fade_length support\n[`SDL_HAPTIC_INFINITY`]. Fade will also not be used since effect never ends.\n\nAdditionally, the [`SDL_HAPTIC_RAMP`] effect does not support a duration of\n[`SDL_HAPTIC_INFINITY`].\n\nButton triggers may not be supported on all devices, it is advised to not\nuse them if possible. Buttons start at index 1 instead of index 0 like the\njoystick.\n\nIf both attack_length and fade_level are 0, the envelope is not used,\notherwise both values are used.\n\nCommon parts:\n\n```c\n// Replay - All effects have this\nUint32 length;        // Duration of effect (ms).\nUint16 delay;         // Delay before starting effect.\n\n// Trigger - All effects have this\nUint16 button;        // Button that triggers effect.\nUint16 interval;      // How soon before effect can be triggered again.\n\n// Envelope - All effects except condition effects have this\nUint16 attack_length; // Duration of the attack (ms).\nUint16 attack_level;  // Level at the start of the attack.\nUint16 fade_length;   // Duration of the fade out (ms).\nUint16 fade_level;    // Level at the end of the fade.\n```\n\nHere we have an example of a constant effect evolution in time:\n\n```text\nStrength\n^\n|\n|    effect level -->  _________________\n|                     /                 \\\n|                    /                   \\\n|                   /                     \\\n|                  /                       \\\n| attack_level --> |                        \\\n|                  |                        |  <---  fade_level\n|\n+--------------------------------------------------> Time\n[--]                 [---]\nattack_length        fade_length\n\n[------------------][-----------------------]\ndelay               length\n```\n\nNote either the attack_level or the fade_level may be above the actual\neffect level.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HapticConstant`]\n- [`SDL_HapticPeriodic`]\n- [`SDL_HapticCondition`]\n- [`SDL_HapticRamp`]\n- [`SDL_HapticLeftRight`]\n- [`SDL_HapticCustom`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("Effect type.\n"),
            available_since: None,
            ty: "SDL_HapticEffectType",
        },
        Field {
            name: "constant",
            doc: Some("Constant effect.\n"),
            available_since: None,
            ty: "SDL_HapticConstant",
        },
        Field {
            name: "periodic",
            doc: Some("Periodic effect.\n"),
            available_since: None,
            ty: "SDL_HapticPeriodic",
        },
        Field {
            name: "condition",
            doc: Some("Condition effect.\n"),
            available_since: None,
            ty: "SDL_HapticCondition",
        },
        Field {
            name: "ramp",
            doc: Some("Ramp effect.\n"),
            available_since: None,
            ty: "SDL_HapticRamp",
        },
        Field {
            name: "leftright",
            doc: Some("Left/Right effect.\n"),
            available_since: None,
            ty: "SDL_HapticLeftRight",
        },
        Field {
            name: "custom",
            doc: Some("Custom effect.\n"),
            available_since: None,
            ty: "SDL_HapticCustom",
        },
    ],
};
