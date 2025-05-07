#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL3_ttf source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL_ttf");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL_ttf");

/// Revision
pub const REVISION: &str = "SDL3_ttf-3.2.2";

/// Version part of the revision
pub const VERSION: &str = "3.2.2";

/// Tag part of the revision
pub const REVISION_TAG: &str = "release-3.2.2";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "release";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "0";

/// Hash part of the revision
pub const REVISION_HASH: &str = "ga1ce367";
