#![doc = include_str!("../README.md")]

use std::path::PathBuf;

/// Get path of the SDL source code
pub fn source_path() -> PathBuf {
    PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "SDL"])
}
