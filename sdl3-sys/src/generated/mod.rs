#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_parens,
    unused_unsafe,
    unused_variables,
    clippy::approx_constant,
    clippy::double_parens,
    clippy::eq_op,
    clippy::identity_op,
    clippy::missing_safety_doc,
    clippy::needless_bool,
    clippy::needless_return,
    clippy::nonminimal_bool,
    clippy::too_long_first_doc_paragraph,
    clippy::unnecessary_cast
)]

pub mod assert;
pub mod atomic;
pub mod audio;
pub mod bits;
pub mod blendmode;
pub mod camera;
pub mod clipboard;
pub mod cpuinfo;
pub mod dialog;
pub mod error;
pub mod events;
pub mod filesystem;
pub mod gamepad;
pub mod gpu;
pub mod guid;
pub mod haptic;
pub mod hidapi;
pub mod hints;
pub mod init;
pub mod iostream;
pub mod joystick;
pub mod keyboard;
pub mod keycode;
pub mod loadso;
pub mod locale;
pub mod log;
pub mod main;
pub mod messagebox;
pub mod metal;
pub mod misc;
pub mod mouse;
pub mod mutex;
pub mod pen;
pub mod pixels;
pub mod platform;
pub mod power;
pub mod process;
pub mod properties;
pub mod rect;
pub mod render;
pub mod revision;
pub mod scancode;
pub mod sensor;
pub mod stdinc;
pub mod storage;
pub mod surface;
pub mod system;
pub mod thread;
pub mod time;
pub mod timer;
pub mod touch;
pub mod version;
pub mod video;
pub mod vulkan;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::assert::*;
    #[doc(no_inline)]
    pub use super::atomic::*;
    #[doc(no_inline)]
    pub use super::audio::*;
    #[doc(no_inline)]
    pub use super::bits::*;
    #[doc(no_inline)]
    pub use super::blendmode::*;
    #[doc(no_inline)]
    pub use super::camera::*;
    #[doc(no_inline)]
    pub use super::clipboard::*;
    #[doc(no_inline)]
    pub use super::cpuinfo::*;
    #[doc(no_inline)]
    pub use super::dialog::*;
    #[doc(no_inline)]
    pub use super::error::*;
    #[doc(no_inline)]
    pub use super::events::*;
    #[doc(no_inline)]
    pub use super::filesystem::*;
    #[doc(no_inline)]
    pub use super::gamepad::*;
    #[doc(no_inline)]
    pub use super::gpu::*;
    #[doc(no_inline)]
    pub use super::guid::*;
    #[doc(no_inline)]
    pub use super::haptic::*;
    #[doc(no_inline)]
    pub use super::hidapi::*;
    #[doc(no_inline)]
    pub use super::hints::*;
    #[doc(no_inline)]
    pub use super::init::*;
    #[doc(no_inline)]
    pub use super::iostream::*;
    #[doc(no_inline)]
    pub use super::joystick::*;
    #[doc(no_inline)]
    pub use super::keyboard::*;
    #[doc(no_inline)]
    pub use super::keycode::*;
    #[doc(no_inline)]
    pub use super::loadso::*;
    #[doc(no_inline)]
    pub use super::locale::*;
    #[doc(no_inline)]
    pub use super::log::*;
    #[doc(no_inline)]
    pub use super::main::*;
    #[doc(no_inline)]
    pub use super::messagebox::*;
    #[doc(no_inline)]
    pub use super::metal::*;
    #[doc(no_inline)]
    pub use super::misc::*;
    #[doc(no_inline)]
    pub use super::mouse::*;
    #[doc(no_inline)]
    pub use super::mutex::*;
    #[doc(no_inline)]
    pub use super::pen::*;
    #[doc(no_inline)]
    pub use super::pixels::*;
    #[doc(no_inline)]
    pub use super::platform::*;
    #[doc(no_inline)]
    pub use super::power::*;
    #[doc(no_inline)]
    pub use super::process::*;
    #[doc(no_inline)]
    pub use super::properties::*;
    #[doc(no_inline)]
    pub use super::rect::*;
    #[doc(no_inline)]
    pub use super::render::*;
    #[doc(no_inline)]
    pub use super::revision::*;
    #[doc(no_inline)]
    pub use super::scancode::*;
    #[doc(no_inline)]
    pub use super::sensor::*;
    #[doc(no_inline)]
    pub use super::stdinc::*;
    #[doc(no_inline)]
    pub use super::storage::*;
    #[doc(no_inline)]
    pub use super::surface::*;
    #[doc(no_inline)]
    pub use super::system::*;
    #[doc(no_inline)]
    pub use super::thread::*;
    #[doc(no_inline)]
    pub use super::time::*;
    #[doc(no_inline)]
    pub use super::timer::*;
    #[doc(no_inline)]
    pub use super::touch::*;
    #[doc(no_inline)]
    pub use super::version::*;
    #[doc(no_inline)]
    pub use super::video::*;
    #[doc(no_inline)]
    pub use super::vulkan::*;
}
