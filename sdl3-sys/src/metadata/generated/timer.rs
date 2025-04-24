//! Metadata for items in the `crate::timer` module

use super::*;

pub static METADATA_SDL_TimerID: Group = Group {
    module: "timer",
    kind: GroupKind::Id,
    name: "SDL_TimerID",
    short_name: "TimerID",
    doc: Some("Definition of the timer ID type.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
