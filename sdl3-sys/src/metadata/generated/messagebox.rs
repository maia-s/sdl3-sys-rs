//! Metadata for items in the `crate::messagebox` module

use super::*;

pub static METADATA_SDL_MessageBoxFlags: Group = Group {
    module: "messagebox",
    kind: GroupKind::Flags,
    name: "SDL_MessageBoxFlags",
    short_name: "MessageBoxFlags",
    doc: Some("Message box flags.\n\nIf supported will display warning icon, etc.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_MESSAGEBOX_ERROR",
            short_name: "ERROR",
            doc: Some("error dialog\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_WARNING",
            short_name: "WARNING",
            doc: Some("warning dialog\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_INFORMATION",
            short_name: "INFORMATION",
            doc: Some("informational dialog\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT",
            short_name: "BUTTONS_LEFT_TO_RIGHT",
            doc: Some("buttons placed left to right\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT",
            short_name: "BUTTONS_RIGHT_TO_LEFT",
            doc: Some("buttons placed right to left\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_MessageBoxButtonFlags: Group = Group {
    module: "messagebox",
    kind: GroupKind::Flags,
    name: "SDL_MessageBoxButtonFlags",
    short_name: "MessageBoxButtonFlags",
    doc: Some("[`SDL_MessageBoxButtonData`] flags.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT",
            short_name: "RETURNKEY_DEFAULT",
            doc: Some("Marks the default button when return is hit\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT",
            short_name: "ESCAPEKEY_DEFAULT",
            doc: Some("Marks the default button when escape is hit\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_MessageBoxColorType: Group = Group {
    module: "messagebox",
    kind: GroupKind::Enum,
    name: "SDL_MessageBoxColorType",
    short_name: "MessageBoxColorType",
    doc: Some(
        "An enumeration of indices inside the colors array of\n[`SDL_MessageBoxColorScheme`].\n",
    ),
    available_since: None,
    values: &[
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_BACKGROUND",
            short_name: "BACKGROUND",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_TEXT",
            short_name: "TEXT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_BUTTON_BORDER",
            short_name: "BUTTON_BORDER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND",
            short_name: "BUTTON_BACKGROUND",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED",
            short_name: "BUTTON_SELECTED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_MESSAGEBOX_COLOR_COUNT",
            short_name: "COUNT",
            doc: Some("Size of the colors array of [`SDL_MessageBoxColorScheme`].\n"),
            available_since: None,
        },
    ],
};
