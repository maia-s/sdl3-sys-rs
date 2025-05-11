#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL3_mixer source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL_mixer");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\SDL_mixer");

/// Revision
pub const REVISION: &str = "SDL_mixer-3.1.2-prerelease";

/// Version part of the revision
pub const VERSION: &str = "3.1.2";

/// Tag part of the revision
pub const REVISION_TAG: &str = "prerelease-3.1.2";

/// Tag part of the revision without version
pub const REVISION_TAG_BASE: &str = "prerelease";

/// Offset from tag part of the revision
pub const REVISION_OFFSET: &str = "0";

/// Hash part of the revision
pub const REVISION_HASH: &str = "gda025888";
