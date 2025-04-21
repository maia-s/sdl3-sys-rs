#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{Group, GroupKind, GroupValue, Hint, Property},
    properties::SDL_PropertyType,
    version::SDL_VERSIONNUM,
};

/// Metadata for hint constants in this crate
pub static HINTS: &[Hint] = &[];

/// Metadata for property constants in this crate
pub static PROPERTIES: &[Property] = &[];

/// Metadata for groups in this crate
pub static GROUPS: &[Group] = &[];

pub(crate) const GROUP_OFFSET_image: usize = 0;
