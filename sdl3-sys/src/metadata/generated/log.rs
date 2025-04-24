//! Metadata for items in the `crate::log` module

use super::*;

pub static METADATA_SDL_LogCategory: Group = Group {
    module: "log",
    kind: GroupKind::Enum,
    name: "SDL_LogCategory",
    short_name: "LogCategory",
    doc: Some("The predefined log categories\n\nBy default the application and gpu categories are enabled at the INFO\nlevel, the assert category is enabled at the WARN level, test is enabled at\nthe VERBOSE level and all other categories are enabled at the ERROR level.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_LOG_CATEGORY_APPLICATION",
            short_name: "APPLICATION",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_ERROR",
            short_name: "ERROR",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_ASSERT",
            short_name: "ASSERT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_SYSTEM",
            short_name: "SYSTEM",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_AUDIO",
            short_name: "AUDIO",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_VIDEO",
            short_name: "VIDEO",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RENDER",
            short_name: "RENDER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_INPUT",
            short_name: "INPUT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_TEST",
            short_name: "TEST",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_GPU",
            short_name: "GPU",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED2",
            short_name: "RESERVED2",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED3",
            short_name: "RESERVED3",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED4",
            short_name: "RESERVED4",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED5",
            short_name: "RESERVED5",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED6",
            short_name: "RESERVED6",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED7",
            short_name: "RESERVED7",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED8",
            short_name: "RESERVED8",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED9",
            short_name: "RESERVED9",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_RESERVED10",
            short_name: "RESERVED10",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_CATEGORY_CUSTOM",
            short_name: "CUSTOM",
            doc: None,
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_LogPriority: Group = Group {
    module: "log",
    kind: GroupKind::Enum,
    name: "SDL_LogPriority",
    short_name: "LogPriority",
    doc: Some("The predefined log priorities\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_LOG_PRIORITY_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_TRACE",
            short_name: "TRACE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_VERBOSE",
            short_name: "VERBOSE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_DEBUG",
            short_name: "DEBUG",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_INFO",
            short_name: "INFO",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_WARN",
            short_name: "WARN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_ERROR",
            short_name: "ERROR",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_CRITICAL",
            short_name: "CRITICAL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOG_PRIORITY_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
