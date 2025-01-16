#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL 3 source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL");

/// Revision
pub const REVISION: &str = "SDL3-prerelease-3.1.10";

/// Version part of the revision
pub const VERSION: &str = "3.1.10";

/// Tag part of the revision
pub const REVISION_TAG: &str = "prerelease-3.1.10";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "prerelease";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "0";

/// Hash part of the revision
pub const REVISION_HASH: &str = "gb96bb152c";
