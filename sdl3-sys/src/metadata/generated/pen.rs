//! Metadata for items in the `crate::pen` module

use super::*;

pub const METADATA_SDL_PenID: Group = Group {
    module: "pen",
    kind: GroupKind::Id,
    name: "SDL_PenID",
    short_name: "PenID",
    doc: Some(
        "SDL pen instance IDs.\n\nZero is used to signify an invalid/null device.\n\nThese show up in pen events when SDL sees input from them. They remain\nconsistent as long as SDL can recognize a tool to be the same pen; but if a\npen physically leaves the area and returns, it might get a new ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_PenInputFlags: Group = Group {
    module: "pen",
    kind: GroupKind::Flags,
    name: "SDL_PenInputFlags",
    short_name: "PenInputFlags",
    doc: Some(
        "Pen input flags, as reported by various pen events' `pen_state` field.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_PEN_INPUT_DOWN",
            short_name: "DOWN",
            doc: Some("pen is pressed down\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_BUTTON_1",
            short_name: "BUTTON_1",
            doc: Some("button 1 is pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_BUTTON_2",
            short_name: "BUTTON_2",
            doc: Some("button 2 is pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_BUTTON_3",
            short_name: "BUTTON_3",
            doc: Some("button 3 is pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_BUTTON_4",
            short_name: "BUTTON_4",
            doc: Some("button 4 is pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_BUTTON_5",
            short_name: "BUTTON_5",
            doc: Some("button 5 is pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_INPUT_ERASER_TIP",
            short_name: "ERASER_TIP",
            doc: Some("eraser tip is used\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_PenAxis: Group = Group {
    module: "pen",
    kind: GroupKind::Enum,
    name: "SDL_PenAxis",
    short_name: "PenAxis",
    doc: Some(
        "Pen axis indices.\n\nThese are the valid values for the `axis` field in [`SDL_PenAxisEvent`]. All\naxes are either normalised to 0..1 or report a (positive or negative) angle\nin degrees, with 0.0 representing the centre. Not all pens/backends support\nall axes: unsupported axes are always zero.\n\nTo convert angles for tilt and rotation into vector representation, use\n[`SDL_sinf`] on the XTILT, YTILT, or ROTATION component, for example:\n\n`SDL_sinf(xtilt * SDL_PI_F / 180.0)`.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_PEN_AXIS_PRESSURE",
            short_name: "PRESSURE",
            doc: Some("Pen pressure.  Unidirectional: 0 to 1.0\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_XTILT",
            short_name: "XTILT",
            doc: Some(
                "Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_YTILT",
            short_name: "YTILT",
            doc: Some("Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_DISTANCE",
            short_name: "DISTANCE",
            doc: Some("Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_ROTATION",
            short_name: "ROTATION",
            doc: Some(
                "Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is facing down).\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_SLIDER",
            short_name: "SLIDER",
            doc: Some(
                "Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_TANGENTIAL_PRESSURE",
            short_name: "TANGENTIAL_PRESSURE",
            doc: Some("Pressure from squeezing the pen (\"barrel pressure\").\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_AXIS_COUNT",
            short_name: "COUNT",
            doc: Some(
                "Total known pen axis types in this version of SDL. This number may grow in future releases!\n",
            ),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_PenDeviceType: Group = Group {
    module: "pen",
    kind: GroupKind::Enum,
    name: "SDL_PenDeviceType",
    short_name: "PenDeviceType",
    doc: Some(
        "An enum that describes the type of a pen device.\n\nA \"direct\" device is a pen that touches a graphic display (like an Apple\nPencil on an iPad's screen). \"Indirect\" devices touch an external tablet\nsurface that is connected to the machine but is not a display (like a\nlower-end Wacom tablet connected over USB).\n\nApps may use this information to decide if they should draw a cursor; if\nthe pen is touching the screen directly, a cursor doesn't make sense and\ncan be in the way, but becomes necessary for indirect devices to know where\non the display they are interacting.\n\n## Availability\nThis enum is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
    values: &[
        GroupValue {
            name: "SDL_PEN_DEVICE_TYPE_INVALID",
            short_name: "INVALID",
            doc: Some("Not a valid pen device.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_DEVICE_TYPE_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("Don't know specifics of this pen.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_DEVICE_TYPE_DIRECT",
            short_name: "DIRECT",
            doc: Some("Pen touches display.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEN_DEVICE_TYPE_INDIRECT",
            short_name: "INDIRECT",
            doc: Some("Pen touches something that isn't the display.\n"),
            available_since: None,
        },
    ],
};
