//! Metadata for SDL types and constants

use crate::properties::SDL_PropertyType;
use core::ffi::{c_int, CStr};

/// Metadata for hint constants
#[derive(Clone, Copy)]
pub struct Hint {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: &'static CStr,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
}

/// Metadata for property constants
#[derive(Clone, Copy)]
pub struct Property {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: &'static CStr,
    pub ty: SDL_PropertyType,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
}

/// Access metadata for typed groups of constants (c enums, flags, etc)
pub trait HasGroupMetadata: 'static + Sized {
    /// Metadata for this group
    const GROUP_METADATA: &'static Group;
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum GroupKind {
    Enum,
    Flags,
    Id,
    Lock,
}

/// Metadata for typed groups of constants (c enums, flags, etc)
#[derive(Clone, Copy)]
pub struct Group {
    pub kind: GroupKind,
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub doc: Option<&'static str>,
    pub values: &'static [GroupValue],
    pub available_since: Option<c_int>,
}

/// Metadata for a single value in a group of constants
#[derive(Clone, Copy)]
pub struct GroupValue {
    pub name: &'static str,
    pub short_name: &'static str,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
}

#[cfg(feature = "metadata")]
mod generated;

#[cfg(feature = "metadata")]
#[allow(unused_imports)] // false positive
pub use generated::*;
