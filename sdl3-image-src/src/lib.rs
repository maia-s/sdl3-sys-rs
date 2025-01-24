#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL3_image source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL_image");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL_image");

/// Revision
pub const REVISION: &str = "SDL3_image-prerelease-3.1.1";

/// Version part of the revision
pub const VERSION: &str = "3.1.1";

/// Tag part of the revision
pub const REVISION_TAG: &str = "prerelease-3.1.1";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "prerelease";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "0";

/// Hash part of the revision
pub const REVISION_HASH: &str = "g4ff27afa";
