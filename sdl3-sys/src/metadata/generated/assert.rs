//! Metadata for items in the `crate::assert` module

use super::*;

pub const METADATA_SDL_AssertState: Group = Group {
    module: "assert",
    kind: GroupKind::Enum,
    name: "SDL_AssertState",
    short_name: "AssertState",
    doc: Some(
        "Possible outcomes from a triggered assertion.\n\nWhen an enabled assertion triggers, it may call the assertion handler\n(possibly one provided by the app via [`SDL_SetAssertionHandler`]), which will\nreturn one of these values, possibly after asking the user.\n\nThen SDL will respond based on this outcome (loop around to retry the\ncondition, try to break in a debugger, kill the program, or ignore the\nproblem).\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
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
pub const METADATA_SDL_AssertData: Struct = Struct {
    module: "assert",
    kind: StructKind::Struct,
    name: "SDL_AssertData",
    doc: Some(
        "Information about an assertion failure.\n\nThis structure is filled in with information about a triggered assertion,\nused by the assertion handler, then added to the assertion report. This is\nreturned as a linked list from [`SDL_GetAssertionReport()`].\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "always_ignore",
            doc: Some("true if app should always continue when assertion is triggered.\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "trigger_count",
            doc: Some("Number of times this assertion has been triggered.\n"),
            available_since: None,
            ty: "::core::ffi::c_uint",
        },
        Field {
            name: "condition",
            doc: Some("A string of this assert's test code.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "filename",
            doc: Some("The source file where this assert lives.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "linenum",
            doc: Some("The line in `filename` where this assert lives.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "function",
            doc: Some("The name of the function where this assert lives.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "next",
            doc: Some("next item in the linked list.\n"),
            available_since: None,
            ty: "*const SDL_AssertData",
        },
    ],
};
