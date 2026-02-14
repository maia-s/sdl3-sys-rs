//! Metadata for SDL types and constants

use core::ffi::{CStr, c_char, c_int};

pub use crate::properties::SDL_PropertyType as PropertyType;

/// Metadata for hint constants
#[derive(Clone, Copy)]
pub struct Hint {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: *const c_char,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
}

unsafe impl Send for Hint {}
unsafe impl Sync for Hint {}

impl Hint {
    #[inline(always)]
    pub const fn value_cstr(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.value) }
    }

    #[inline(always)]
    pub const fn value_str(&self) -> &'static str {
        match self.value_cstr().to_str() {
            Ok(str) => str,
            Err(_) => unreachable!(),
        }
    }
}

/// Metadata for property constants
#[derive(Clone, Copy)]
pub struct Property {
    pub module: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub value: *const c_char,
    pub ty: PropertyType,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
}

unsafe impl Send for Property {}
unsafe impl Sync for Property {}

impl Property {
    #[inline(always)]
    pub const fn value_cstr(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.value) }
    }

    #[inline(always)]
    pub const fn value_str(&self) -> &'static str {
        match self.value_cstr().to_str() {
            Ok(str) => str,
            Err(_) => unreachable!(),
        }
    }
}

/// Access metadata for typed groups of constants (c enums, flags, etc)
pub trait GroupMetadata: 'static + Sized {
    /// Metadata for this group
    const GROUP_METADATA: &'static Group;
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StructKind {
    Struct,
    Union,
}

/// Metadata for structs and unions
#[derive(Clone, Copy)]
pub struct Struct {
    pub kind: StructKind,
    pub module: &'static str,
    pub name: &'static str,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
    pub fields: &'static [Field],
}

/// Metadata for fields of structs and unions
#[derive(Clone, Copy)]
pub struct Field {
    pub name: &'static str,
    pub doc: Option<&'static str>,
    pub available_since: Option<c_int>,
    pub ty: &'static str,
}

#[cfg(feature = "metadata")]
mod generated;

#[cfg(feature = "metadata")]
#[allow(unused_imports)] // false positive
pub use generated::*;
