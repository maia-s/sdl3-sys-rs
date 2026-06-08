#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL3_sound source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL_sound");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL_sound");

/// Revision
pub const REVISION: &str = "SDL_sound-3.2.0-v3.2.0";

/// Version part of the revision
pub const VERSION: &str = "3.2.0";

/// Tag part of the revision
pub const REVISION_TAG: &str = "v3.2.0";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "0";

/// Hash part of the revision
pub const REVISION_HASH: &str = "g49b3fad";
