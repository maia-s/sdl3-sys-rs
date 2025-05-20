#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{
        Field, Group, GroupKind, GroupValue, Hint, Property, PropertyType, Struct, StructKind,
    },
    version::SDL_VERSIONNUM,
};

pub mod net;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::net::*;
}

/// Metadata for hint constants in this crate
pub const HINTS: &[&Hint] = &[];

/// Metadata for property constants in this crate
pub const PROPERTIES: &[&Property] = &[
    &net::METADATA_NET_PROP_SERVER_REUSEADDR_BOOLEAN,
    &net::METADATA_NET_PROP_DATAGRAM_SOCKET_REUSEADDR_BOOLEAN,
    &net::METADATA_NET_PROP_DATAGRAM_SOCKET_ALLOW_BROADCAST_BOOLEAN,
];

/// Metadata for groups in this crate
pub const GROUPS: &[&Group] = &[&net::METADATA_NET_Status];

/// Metadata for structs and unions in this crate
pub const STRUCTS: &[&Struct] = &[&net::METADATA_NET_Datagram];
