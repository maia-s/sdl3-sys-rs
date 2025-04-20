use crate::properties::SDL_PropertyType;
use core::ffi::CStr;

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
pub use generated_metadata::*;
