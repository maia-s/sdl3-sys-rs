//! Metadata for items in the `crate::time` module

use super::*;

pub const METADATA_SDL_DateFormat: Group = Group {
    module: "time",
    kind: GroupKind::Enum,
    name: "SDL_DateFormat",
    short_name: "DateFormat",
    doc: Some(
        "The preferred date format of the current system locale.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetDateTimeLocalePreferences`]\n",
    ),
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
    doc: Some(
        "The preferred time format of the current system locale.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetDateTimeLocalePreferences`]\n",
    ),
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
pub const METADATA_SDL_DateTime: Struct = Struct {
    module: "time",
    kind: StructKind::Struct,
    name: "SDL_DateTime",
    doc: Some(
        "A structure holding a calendar date and time broken down into its\ncomponents.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "year",
            doc: Some("Year\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "month",
            doc: Some("Month \\[01-12\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "day",
            doc: Some("Day of the month \\[01-31\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "hour",
            doc: Some("Hour \\[0-23\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "minute",
            doc: Some("Minute \\[0-59\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "second",
            doc: Some("Seconds \\[0-60\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "nanosecond",
            doc: Some("Nanoseconds \\[0-999999999\\]\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "day_of_week",
            doc: Some("Day of the week \\[0-6\\] (0 being Sunday)\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "utc_offset",
            doc: Some("Seconds east of UTC\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
