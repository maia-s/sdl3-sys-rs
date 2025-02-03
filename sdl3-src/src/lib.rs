#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL 3 source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL");

/// Revision
pub const REVISION: &str = "SDL3-release-3.2.2-41-g6cb3d37a2";

/// Version part of the revision
pub const VERSION: &str = "3.2.2";

/// Tag part of the revision
pub const REVISION_TAG: &str = "release-3.2.2";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "release";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "41";

/// Hash part of the revision
pub const REVISION_HASH: &str = "g6cb3d37a2";
