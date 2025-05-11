#![no_std]
#![cfg_attr(all(feature = "nightly", doc), feature(doc_auto_cfg))] // https://github.com/rust-lang/rust/issues/43781
#![cfg_attr(all(feature = "nightly", doc), feature(doc_cfg))] // https://github.com/rust-lang/rust/issues/43781
#![doc = include_str!("../README.md")]

mod generated;

pub use generated::image;

#[doc(hidden)]
pub use generated::everything;

#[cfg(feature = "metadata")]
pub mod metadata;
