//! Metadata for items in the `crate::mutex` module

use super::*;

pub const METADATA_SDL_InitStatus: Group = Group {
    module: "mutex",
    kind: GroupKind::Enum,
    name: "SDL_InitStatus",
    short_name: "InitStatus",
    doc: Some(
        "The current status of an [`SDL_InitState`] structure.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_INIT_STATUS_UNINITIALIZED",
            short_name: "UNINITIALIZED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_INITIALIZING",
            short_name: "INITIALIZING",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_INITIALIZED",
            short_name: "INITIALIZED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_UNINITIALIZING",
            short_name: "UNINITIALIZING",
            doc: None,
            available_since: None,
        },
    ],
};
