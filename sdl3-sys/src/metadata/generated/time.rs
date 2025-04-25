//! Metadata for items in the `crate::time` module

use super::*;

pub const METADATA_SDL_DateFormat: Group = Group {
    module: "time",
    kind: GroupKind::Enum,
    name: "SDL_DateFormat",
    short_name: "DateFormat",
    doc: Some("The preferred date format of the current system locale.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetDateTimeLocalePreferences`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_DATE_FORMAT_YYYYMMDD",
            short_name: "YYYYMMDD",
            doc: Some("Year/Month/Day\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_DATE_FORMAT_DDMMYYYY",
            short_name: "DDMMYYYY",
            doc: Some("Day/Month/Year\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_DATE_FORMAT_MMDDYYYY",
            short_name: "MMDDYYYY",
            doc: Some("Month/Day/Year\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_TimeFormat: Group = Group {
    module: "time",
    kind: GroupKind::Enum,
    name: "SDL_TimeFormat",
    short_name: "TimeFormat",
    doc: Some("The preferred time format of the current system locale.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetDateTimeLocalePreferences`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_TIME_FORMAT_24HR",
            short_name: "_24HR",
            doc: Some("24 hour time\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TIME_FORMAT_12HR",
            short_name: "_12HR",
            doc: Some("12 hour time\n"),
            available_since: None,
        },
    ],
};
