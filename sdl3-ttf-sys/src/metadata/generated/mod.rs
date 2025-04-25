#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{Group, GroupKind, GroupValue, Hint, Property, PropertyType},
    version::SDL_VERSIONNUM,
};

pub mod textengine;
pub mod ttf;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::textengine::*;
    #[doc(no_inline)]
    pub use super::ttf::*;
}

/// Metadata for hint constants in this crate
pub static HINTS: &[&Hint] = &[];

/// Metadata for property constants in this crate
pub static PROPERTIES: &[&Property] = &[];

/// Metadata for groups in this crate
pub static GROUPS: &[&Group] = &[
    &textengine::METADATA_TTF_DrawCommand,
    &ttf::METADATA_TTF_FontStyleFlags,
    &ttf::METADATA_TTF_HintingFlags,
    &ttf::METADATA_TTF_HorizontalAlignment,
    &ttf::METADATA_TTF_Direction,
    &ttf::METADATA_TTF_ImageType,
    &ttf::METADATA_TTF_GPUTextEngineWinding,
    &ttf::METADATA_TTF_SubStringFlags,
];
