//! Metadata for items in the `crate::messagebox` module

use super::*;

pub const METADATA_SDL_MessageBoxFlags: Group = Group {
    module: "messagebox",
    kind: GroupKind::Flags,
    name: "SDL_MessageBoxFlags",
    short_name: "MessageBoxFlags",
    doc: Some(
        "Message box flags.\n\nIf supported will display warning icon, etc.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
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
pub const METADATA_SDL_MessageBoxButtonFlags: Group = Group {
    module: "messagebox",
    kind: GroupKind::Flags,
    name: "SDL_MessageBoxButtonFlags",
    short_name: "MessageBoxButtonFlags",
    doc: Some(
        "[`SDL_MessageBoxButtonData`] flags.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
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
pub const METADATA_SDL_MessageBoxColorType: Group = Group {
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
pub const METADATA_SDL_MessageBoxButtonData: Struct = Struct {
    module: "messagebox",
    kind: StructKind::Struct,
    name: "SDL_MessageBoxButtonData",
    doc: Some(
        "Individual button data.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "flags",
            doc: None,
            available_since: None,
            ty: "SDL_MessageBoxButtonFlags",
        },
        Field {
            name: "buttonID",
            doc: Some("User defined button id (value returned via [`SDL_ShowMessageBox`])\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "text",
            doc: Some("The UTF-8 button text\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
    ],
};
pub const METADATA_SDL_MessageBoxColor: Struct = Struct {
    module: "messagebox",
    kind: StructKind::Struct,
    name: "SDL_MessageBoxColor",
    doc: Some(
        "RGB value used in a message box color scheme\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "g",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "b",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_MessageBoxColorScheme: Struct = Struct {
    module: "messagebox",
    kind: StructKind::Struct,
    name: "SDL_MessageBoxColorScheme",
    doc: Some(
        "A set of colors to use for message box dialogs\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[Field {
        name: "colors",
        doc: None,
        available_since: None,
        ty: "[SDL_MessageBoxColor; SDL_MESSAGEBOX_COLOR_COUNT.0 as ::core::primitive::usize]",
    }],
};
pub const METADATA_SDL_MessageBoxData: Struct = Struct {
    module: "messagebox",
    kind: StructKind::Struct,
    name: "SDL_MessageBoxData",
    doc: Some(
        "MessageBox structure containing title, text, window, etc.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "flags",
            doc: None,
            available_since: None,
            ty: "SDL_MessageBoxFlags",
        },
        Field {
            name: "window",
            doc: Some("Parent window, can be NULL\n"),
            available_since: None,
            ty: "*mut SDL_Window",
        },
        Field {
            name: "title",
            doc: Some("UTF-8 title\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "message",
            doc: Some("UTF-8 message text\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "numbuttons",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "buttons",
            doc: None,
            available_since: None,
            ty: "*const SDL_MessageBoxButtonData",
        },
        Field {
            name: "colorScheme",
            doc: Some("[`SDL_MessageBoxColorScheme`], can be NULL to use system settings\n"),
            available_since: None,
            ty: "*const SDL_MessageBoxColorScheme",
        },
    ],
};
