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
    pub use super::assert::*;
    pub use super::atomic::*;
    pub use super::audio::*;
    pub use super::bits::*;
    pub use super::blendmode::*;
    pub use super::camera::*;
    pub use super::clipboard::*;
    pub use super::cpuinfo::*;
    pub use super::dialog::*;
    pub use super::error::*;
    pub use super::events::*;
    pub use super::filesystem::*;
    pub use super::gamepad::*;
    pub use super::gpu::*;
    pub use super::guid::*;
    pub use super::haptic::*;
    pub use super::hidapi::*;
    pub use super::hints::*;
    pub use super::init::*;
    pub use super::iostream::*;
    pub use super::joystick::*;
    pub use super::keyboard::*;
    pub use super::keycode::*;
    pub use super::loadso::*;
    pub use super::locale::*;
    pub use super::log::*;
    pub use super::messagebox::*;
    pub use super::metal::*;
    pub use super::misc::*;
    pub use super::mouse::*;
    pub use super::mutex::*;
    pub use super::pen::*;
    pub use super::pixels::*;
    pub use super::platform::*;
    pub use super::power::*;
    pub use super::process::*;
    pub use super::properties::*;
    pub use super::rect::*;
    pub use super::render::*;
    pub use super::revision::*;
    pub use super::scancode::*;
    pub use super::sensor::*;
    pub use super::stdinc::*;
    pub use super::storage::*;
    pub use super::surface::*;
    pub use super::system::*;
    pub use super::thread::*;
    pub use super::time::*;
    pub use super::timer::*;
    pub use super::touch::*;
    pub use super::version::*;
    pub use super::video::*;
    pub use super::vulkan::*;
}
