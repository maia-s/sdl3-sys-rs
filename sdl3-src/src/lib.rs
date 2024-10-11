#![no_std]
#![doc = include_str!("../README.md")]

/// Location of the SDL 3 source code
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SDL");
