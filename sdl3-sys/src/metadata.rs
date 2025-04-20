use crate::properties::SDL_PropertyType;
use core::ffi::CStr;

pub struct Hint {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: &'static CStr,
    pub doc: &'static str,
}

pub struct Property {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: &'static CStr,
    pub ty: SDL_PropertyType,
    pub doc: &'static str,
}

#[cfg(feature = "metadata")]
#[path = "generated_metadata.rs"]
mod generated_metadata;

#[cfg(feature = "metadata")]
#[allow(unused_imports)] // false positive
pub use generated_metadata::*;
