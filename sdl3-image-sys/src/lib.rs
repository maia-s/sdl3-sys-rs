#![no_std]
#![doc = include_str!("../README.md")]

mod generated;

pub use generated::image;

#[doc(hidden)]
pub use generated::everything;

#[cfg(feature = "metadata")]
pub mod metadata;
