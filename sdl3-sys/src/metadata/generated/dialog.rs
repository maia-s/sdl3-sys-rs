//! Metadata for items in the `crate::dialog` module

use super::*;

pub static METADATA_SDL_PROP_FILE_DIALOG_FILTERS_POINTER: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_FILTERS_POINTER",
    short_name: "FILE_DIALOG_FILTERS",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_FILTERS_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER",
    short_name: "FILE_DIALOG_NFILTERS",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_WINDOW_POINTER: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_WINDOW_POINTER",
    short_name: "FILE_DIALOG_WINDOW",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_WINDOW_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_LOCATION_STRING: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_LOCATION_STRING",
    short_name: "FILE_DIALOG_LOCATION",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_LOCATION_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_MANY_BOOLEAN: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_MANY_BOOLEAN",
    short_name: "FILE_DIALOG_MANY",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_MANY_BOOLEAN) },
    ty: SDL_PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_TITLE_STRING: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_TITLE_STRING",
    short_name: "FILE_DIALOG_TITLE",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_TITLE_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_ACCEPT_STRING: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_ACCEPT_STRING",
    short_name: "FILE_DIALOG_ACCEPT",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_ACCEPT_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_FILE_DIALOG_CANCEL_STRING: Property = Property {
    module: "dialog",
    name: "SDL_PROP_FILE_DIALOG_CANCEL_STRING",
    short_name: "FILE_DIALOG_CANCEL",
    value: unsafe { CStr::from_ptr(crate::dialog::SDL_PROP_FILE_DIALOG_CANCEL_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_FileDialogType: Group = Group {
    module: "dialog",
    kind: GroupKind::Enum,
    name: "SDL_FileDialogType",
    short_name: "FileDialogType",
    doc: Some("Various types of file dialogs.\n\nThis is used by [`SDL_ShowFileDialogWithProperties()`] to decide what kind of\ndialog to present to the user.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_ShowFileDialogWithProperties`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_FILEDIALOG_OPENFILE",
            short_name: "OPENFILE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_FILEDIALOG_SAVEFILE",
            short_name: "SAVEFILE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_FILEDIALOG_OPENFOLDER",
            short_name: "OPENFOLDER",
            doc: None,
            available_since: None,
        },
    ],
};
