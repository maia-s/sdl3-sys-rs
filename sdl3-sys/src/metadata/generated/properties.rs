//! Metadata for items in the `crate::properties` module

use super::*;

pub const METADATA_SDL_PropertiesID: Group = Group {
    module: "properties",
    kind: GroupKind::Id,
    name: "SDL_PropertiesID",
    short_name: "PropertiesID",
    doc: Some(
        "SDL properties ID\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_PropertyType: Group = Group {
    module: "properties",
    kind: GroupKind::Enum,
    name: "SDL_PropertyType",
    short_name: "PropertyType",
    doc: Some("SDL property type\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_PROPERTY_TYPE_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROPERTY_TYPE_POINTER",
            short_name: "POINTER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROPERTY_TYPE_STRING",
            short_name: "STRING",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROPERTY_TYPE_NUMBER",
            short_name: "NUMBER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROPERTY_TYPE_FLOAT",
            short_name: "FLOAT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROPERTY_TYPE_BOOLEAN",
            short_name: "BOOLEAN",
            doc: None,
            available_since: None,
        },
    ],
};
