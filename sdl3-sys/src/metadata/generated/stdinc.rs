//! Metadata for items in the `crate::stdinc` module

use super::*;

pub const METADATA_SDL_alignment_test: Struct = Struct {
    module: "stdinc",
    kind: StructKind::Struct,
    name: "SDL_alignment_test",
    doc: None,
    available_since: None,
    fields: &[
        Field {
            name: "a",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "b",
            doc: None,
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
