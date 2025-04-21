#![no_std]
#![doc = include_str!("../README.md")]

mod generated;
pub use generated::*;

#[cfg(feature = "metadata")]
pub mod metadata;
