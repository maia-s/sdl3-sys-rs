//! Metadata for items in the `crate::system` module

use super::*;

pub static METADATA_SDL_Sandbox: Group = Group {
    module: "system",
    kind: GroupKind::Enum,
    name: "SDL_Sandbox",
    short_name: "Sandbox",
    doc: Some("Application sandbox environment.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SANDBOX_NONE",
            short_name: "NONE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_SANDBOX_UNKNOWN_CONTAINER",
            short_name: "UNKNOWN_CONTAINER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_SANDBOX_FLATPAK",
            short_name: "FLATPAK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_SANDBOX_SNAP",
            short_name: "SNAP",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_SANDBOX_MACOS",
            short_name: "MACOS",
            doc: None,
            available_since: None,
        },
    ],
};
