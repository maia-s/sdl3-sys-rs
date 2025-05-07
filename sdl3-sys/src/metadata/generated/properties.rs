//! Metadata for items in the `crate::properties` module

use super::*;

pub const METADATA_SDL_PROP_NAME_STRING: Property = Property {
    module: "properties",
    name: "SDL_PROP_NAME_STRING",
    short_name: "NAME_STRING",
    value: crate::properties::SDL_PROP_NAME_STRING,
    ty: PropertyType::STRING,
    doc: Some(
        "A generic property for naming things.\n\nThis property is intended to be added to any [`SDL_PropertiesID`] that needs a\ngeneric name associated with the property set. It is not guaranteed that\nany property set will include this key, but it is convenient to have a\nstandard key that any piece of code could reasonably agree to use.\n\nFor example, the properties associated with an [`SDL_Texture`] might have a\nname string of \"player sprites\", or an [`SDL_AudioStream`] might have\n\"background music\", etc. This might also be useful for an [`SDL_IOStream`] to\nlist the path to its asset.\n\nThere is no format for the value set with this key; it is expected to be\nhuman-readable and informational in nature, possibly for logging or\ndebugging purposes.\n\nSDL does not currently set this property on any objects it creates, but\nthis may change in later versions; it is currently expected that apps and\nexternal libraries will take advantage of it, when appropriate.\n\n## Availability\nThis macro is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_PropertiesID: Group = Group {
    module: "properties",
    kind: GroupKind::Id,
    name: "SDL_PropertiesID",
    short_name: "PropertiesID",
    doc: Some(
        "An ID that represents a properties set.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
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
