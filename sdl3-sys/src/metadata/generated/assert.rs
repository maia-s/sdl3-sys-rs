//! Metadata for items in the `crate::assert` module

use super::*;

pub const METADATA_SDL_AssertState: Group = Group {
    module: "assert",
    kind: GroupKind::Enum,
    name: "SDL_AssertState",
    short_name: "AssertState",
    doc: Some("Possible outcomes from a triggered assertion.\n\nWhen an enabled assertion triggers, it may call the assertion handler\n(possibly one provided by the app via [`SDL_SetAssertionHandler`]), which will\nreturn one of these values, possibly after asking the user.\n\nThen SDL will respond based on this outcome (loop around to retry the\ncondition, try to break in a debugger, kill the program, or ignore the\nproblem).\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ASSERTION_RETRY",
            short_name: "RETRY",
            doc: Some("Retry the assert immediately.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASSERTION_BREAK",
            short_name: "BREAK",
            doc: Some("Make the debugger trigger a breakpoint.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASSERTION_ABORT",
            short_name: "ABORT",
            doc: Some("Terminate the program.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASSERTION_IGNORE",
            short_name: "IGNORE",
            doc: Some("Ignore the assert.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASSERTION_ALWAYS_IGNORE",
            short_name: "ALWAYS_IGNORE",
            doc: Some("Ignore the assert from now on.\n"),
            available_since: None,
        },
    ],
};
