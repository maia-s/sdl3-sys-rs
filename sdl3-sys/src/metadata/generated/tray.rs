//! Metadata for items in the `crate::tray` module

use super::*;

pub const METADATA_SDL_TrayEntryFlags: Group = Group {
    module: "tray",
    kind: GroupKind::Flags,
    name: "SDL_TrayEntryFlags",
    short_name: "TrayEntryFlags",
    doc: Some(
        "Flags that control the creation of system tray entries.\n\nSome of these flags are required; exactly one of them must be specified at\nthe time a tray entry is created. Other flags are optional; zero or more of\nthose can be OR'ed together with the required flag.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_InsertTrayEntryAt`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_TRAYENTRY_BUTTON",
            short_name: "BUTTON",
            doc: Some("Make the entry a simple button. Required.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TRAYENTRY_CHECKBOX",
            short_name: "CHECKBOX",
            doc: Some("Make the entry a checkbox. Required.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TRAYENTRY_SUBMENU",
            short_name: "SUBMENU",
            doc: Some("Prepare the entry to have a submenu. Required\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TRAYENTRY_DISABLED",
            short_name: "DISABLED",
            doc: Some("Make the entry disabled. Optional.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TRAYENTRY_CHECKED",
            short_name: "CHECKED",
            doc: Some("Make the entry checked. This is valid only for checkboxes. Optional.\n"),
            available_since: None,
        },
    ],
};
