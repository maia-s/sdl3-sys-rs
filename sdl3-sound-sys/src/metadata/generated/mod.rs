#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{
        Field, Group, GroupKind, GroupValue, Hint, Property, PropertyType, Struct, StructKind,
    },
    version::SDL_VERSIONNUM,
};

pub mod sound;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::sound::*;
}

/// Metadata for hint constants in this crate
pub const HINTS: &[&Hint] = &[];

/// Metadata for property constants in this crate
pub const PROPERTIES: &[&Property] = &[];

/// Metadata for groups in this crate
pub const GROUPS: &[&Group] = &[&sound::METADATA_Sound_SampleFlags];

/// Metadata for structs and unions in this crate
pub const STRUCTS: &[&Struct] = &[
    &sound::METADATA_Sound_DecoderInfo,
    &sound::METADATA_Sound_Sample,
];
