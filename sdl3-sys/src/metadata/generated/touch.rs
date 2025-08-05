//! Metadata for items in the `crate::touch` module

use super::*;

pub const METADATA_SDL_TouchID: Group = Group {
    module: "touch",
    kind: GroupKind::Id,
    name: "SDL_TouchID",
    short_name: "TouchID",
    doc: Some(
        "A unique ID for a touch device.\n\nThis ID is valid for the time the device is connected to the system, and is\nnever reused for the lifetime of the application.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_FingerID: Group = Group {
    module: "touch",
    kind: GroupKind::Id,
    name: "SDL_FingerID",
    short_name: "FingerID",
    doc: Some(
        "A unique ID for a single finger on a touch device.\n\nThis ID is valid for the time the finger (stylus, etc) is touching and will\nbe unique for all fingers currently in contact, so this ID tracks the\nlifetime of a single continuous touch. This value may represent an index, a\npointer, or some other unique ID, depending on the platform.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_TouchDeviceType: Group = Group {
    module: "touch",
    kind: GroupKind::Enum,
    name: "SDL_TouchDeviceType",
    short_name: "TouchDeviceType",
    doc: Some(
        "An enum that describes the type of a touch device.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_TOUCH_DEVICE_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_TOUCH_DEVICE_DIRECT",
            short_name: "DIRECT",
            doc: Some("touch screen with window-relative coordinates\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE",
            short_name: "INDIRECT_ABSOLUTE",
            doc: Some("trackpad with absolute device coordinates\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TOUCH_DEVICE_INDIRECT_RELATIVE",
            short_name: "INDIRECT_RELATIVE",
            doc: Some("trackpad with screen cursor-relative coordinates\n"),
            available_since: None,
        },
    ],
};
