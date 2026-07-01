#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL 3 source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL");

/// Revision
pub const REVISION: &str = "SDL-3.4.13-release-3.4.12-44-g01997ebeb";

/// Version part of the revision
pub const VERSION: &str = "3.4.13";

/// Tag part of the revision
pub const REVISION_TAG: &str = "release-3.4.12";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "release";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "44";

/// Hash part of the revision
pub const REVISION_HASH: &str = "g01997ebeb";
