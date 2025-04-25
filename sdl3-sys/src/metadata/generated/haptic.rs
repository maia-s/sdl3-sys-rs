//! Metadata for items in the `crate::haptic` module

use super::*;

pub const METADATA_SDL_HapticID: Group = Group {
    module: "haptic",
    kind: GroupKind::Id,
    name: "SDL_HapticID",
    short_name: "HapticID",
    doc: Some("This is a unique ID for a haptic device for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the haptic device is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
