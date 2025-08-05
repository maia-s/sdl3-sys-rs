//! Metadata for items in the `crate::power` module

use super::*;

pub const METADATA_SDL_PowerState: Group = Group {
    module: "power",
    kind: GroupKind::Enum,
    name: "SDL_PowerState",
    short_name: "PowerState",
    doc: Some(
        "The basic state for the system's power supply.\n\nThese are results returned by [`SDL_GetPowerInfo()`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_POWERSTATE_ERROR",
            short_name: "ERROR",
            doc: Some("error determining power status\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_POWERSTATE_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("cannot determine power status\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_POWERSTATE_ON_BATTERY",
            short_name: "ON_BATTERY",
            doc: Some("Not plugged in, running on the battery\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_POWERSTATE_NO_BATTERY",
            short_name: "NO_BATTERY",
            doc: Some("Plugged in, no battery available\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_POWERSTATE_CHARGING",
            short_name: "CHARGING",
            doc: Some("Plugged in, charging battery\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_POWERSTATE_CHARGED",
            short_name: "CHARGED",
            doc: Some("Plugged in, battery charged\n"),
            available_since: None,
        },
    ],
};
