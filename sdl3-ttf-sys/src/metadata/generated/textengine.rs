//! Metadata for items in the `crate::textengine` module

use super::*;

pub const METADATA_TTF_DrawCommand: Group = Group {
    module: "textengine",
    kind: GroupKind::Enum,
    name: "TTF_DrawCommand",
    short_name: "DrawCommand",
    doc: Some("A font atlas draw command.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_DRAW_COMMAND_NOOP",
            short_name: "NOOP",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_DRAW_COMMAND_FILL",
            short_name: "FILL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_DRAW_COMMAND_COPY",
            short_name: "COPY",
            doc: None,
            available_since: None,
        },
    ],
};
