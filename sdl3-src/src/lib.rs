#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL 3 source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL");

/// Revision
pub const REVISION: &str = "SDL-preview-3.1.3-290-g8bf1a8420";

/// Version part of the revision
pub const VERSION: &str = "3.1.3";

/// Tag part of the revision
pub const REVISION_TAG: &str = "preview-3.1.3";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "preview";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "290";

/// Hash part of the revision
pub const REVISION_HASH: &str = "g8bf1a8420";
