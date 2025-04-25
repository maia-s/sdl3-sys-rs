//! Metadata for items in the `crate::keyboard` module

use super::*;

pub static METADATA_SDL_PROP_TEXTINPUT_TYPE_NUMBER: Property = Property {
    module: "keyboard",
    name: "SDL_PROP_TEXTINPUT_TYPE_NUMBER",
    short_name: "TEXTINPUT_TYPE_NUMBER",
    value: crate::keyboard::SDL_PROP_TEXTINPUT_TYPE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER: Property = Property {
    module: "keyboard",
    name: "SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER",
    short_name: "TEXTINPUT_CAPITALIZATION_NUMBER",
    value: crate::keyboard::SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN: Property = Property {
    module: "keyboard",
    name: "SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN",
    short_name: "TEXTINPUT_AUTOCORRECT_BOOLEAN",
    value: crate::keyboard::SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN: Property = Property {
    module: "keyboard",
    name: "SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN",
    short_name: "TEXTINPUT_MULTILINE_BOOLEAN",
    value: crate::keyboard::SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER: Property = Property {
    module: "keyboard",
    name: "SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER",
    short_name: "TEXTINPUT_ANDROID_INPUTTYPE_NUMBER",
    value: crate::keyboard::SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_KeyboardID: Group = Group {
    module: "keyboard",
    kind: GroupKind::Id,
    name: "SDL_KeyboardID",
    short_name: "KeyboardID",
    doc: Some("This is a unique ID for a keyboard for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the keyboard is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub static METADATA_SDL_TextInputType: Group = Group {
    module: "keyboard",
    kind: GroupKind::Enum,
    name: "SDL_TextInputType",
    short_name: "TextInputType",
    doc: Some("Text input type.\n\nThese are the valid values for [`SDL_PROP_TEXTINPUT_TYPE_NUMBER`]. Not every\nvalue is valid on every platform, but where a value isn't supported, a\nreasonable fallback will be used.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_StartTextInputWithProperties`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT",
            short_name: "TEXT",
            doc: Some("The input is text\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT_NAME",
            short_name: "TEXT_NAME",
            doc: Some("The input is a person's name\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT_EMAIL",
            short_name: "TEXT_EMAIL",
            doc: Some("The input is an e-mail address\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT_USERNAME",
            short_name: "TEXT_USERNAME",
            doc: Some("The input is a username\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN",
            short_name: "TEXT_PASSWORD_HIDDEN",
            doc: Some("The input is a secure password that is hidden\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE",
            short_name: "TEXT_PASSWORD_VISIBLE",
            doc: Some("The input is a secure password that is visible\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_NUMBER",
            short_name: "NUMBER",
            doc: Some("The input is a number\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN",
            short_name: "NUMBER_PASSWORD_HIDDEN",
            doc: Some("The input is a secure PIN that is hidden\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE",
            short_name: "NUMBER_PASSWORD_VISIBLE",
            doc: Some("The input is a secure PIN that is visible\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_Capitalization: Group = Group {
    module: "keyboard",
    kind: GroupKind::Enum,
    name: "SDL_Capitalization",
    short_name: "Capitalization",
    doc: Some("Auto capitalization type.\n\nThese are the valid values for [`SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER`].\nNot every value is valid on every platform, but where a value isn't\nsupported, a reasonable fallback will be used.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_StartTextInputWithProperties`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_CAPITALIZE_NONE",
            short_name: "NONE",
            doc: Some("No auto-capitalization will be done\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAPITALIZE_SENTENCES",
            short_name: "SENTENCES",
            doc: Some("The first letter of sentences will be capitalized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAPITALIZE_WORDS",
            short_name: "WORDS",
            doc: Some("The first letter of words will be capitalized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAPITALIZE_LETTERS",
            short_name: "LETTERS",
            doc: Some("All letters will be capitalized\n"),
            available_since: None,
        },
    ],
};
