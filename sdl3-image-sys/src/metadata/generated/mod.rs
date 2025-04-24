#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{Group, GroupKind, GroupValue, Hint, Property},
    properties::SDL_PropertyType,
    version::SDL_VERSIONNUM,
};

pub mod image;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::image::*;
}

/// Metadata for hint constants in this crate
pub static HINTS: &[&Hint] = &[];

/// Metadata for property constants in this crate
pub static PROPERTIES: &[&Property] = &[];

/// Metadata for groups in this crate
pub static GROUPS: &[&Group] = &[];
