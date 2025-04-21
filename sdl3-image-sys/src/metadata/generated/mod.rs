#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{Group, GroupKind, GroupValue, Hint, Property},
    properties::SDL_PropertyType,
    version::SDL_VERSIONNUM,
};

mod hints;
pub use hints::*;

mod properties;
pub use properties::*;

mod groups;
pub use groups::*;
