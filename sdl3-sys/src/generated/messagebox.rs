//! Message box support routines.

use super::stdinc::*;

use super::error::*;

use super::video::*;

/// [`SDL_MessageBox`] flags.
///
/// If supported will display warning icon, etc.
///
/// ### Availability
/// This datatype is available since SDL 3.0.0.
pub type SDL_MessageBoxFlags = Uint32;

/// error dialog
pub const SDL_MESSAGEBOX_ERROR: ::core::primitive::u32 = 16_u32;

/// warning dialog
pub const SDL_MESSAGEBOX_WARNING: ::core::primitive::u32 = 32_u32;

/// informational dialog
pub const SDL_MESSAGEBOX_INFORMATION: ::core::primitive::u32 = 64_u32;

/// buttons placed left to right
pub const SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT: ::core::primitive::u32 = 128_u32;

/// buttons placed right to left
pub const SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT: ::core::primitive::u32 = 256_u32;

/// [`SDL_MessageBoxButtonData`] flags.
///
/// ### Availability
/// This datatype is available since SDL 3.0.0.
pub type SDL_MessageBoxButtonFlags = Uint32;

/// Marks the default button when return is hit
pub const SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT: ::core::primitive::u32 = 1_u32;

/// Marks the default button when escape is hit
pub const SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT: ::core::primitive::u32 = 2_u32;

/// Individual button data.
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxButtonData {
    pub flags: SDL_MessageBoxButtonFlags,
    /// User defined button id (value returned via [`SDL_ShowMessageBox`])
    pub buttonID: ::core::ffi::c_int,
    /// The UTF-8 button text
    pub text: *const ::core::ffi::c_char,
}

/// RGB value used in a message box color scheme
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxColor {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
}

/// An enumeration of indices inside the colors array of
/// [`SDL_MessageBoxColorScheme`].
///
/// ### `sdl3-sys` note
/// This is a `C` enum. Known values:
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`SDL_MessageBoxColorType::BACKGROUND`] | [`SDL_MESSAGEBOX_COLOR_BACKGROUND`] |  |
/// | [`SDL_MessageBoxColorType::TEXT`] | [`SDL_MESSAGEBOX_COLOR_TEXT`] |  |
/// | [`SDL_MessageBoxColorType::BUTTON_BORDER`] | [`SDL_MESSAGEBOX_COLOR_BUTTON_BORDER`] |  |
/// | [`SDL_MessageBoxColorType::BUTTON_BACKGROUND`] | [`SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND`] |  |
/// | [`SDL_MessageBoxColorType::BUTTON_SELECTED`] | [`SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED`] |  |
/// | [`SDL_MessageBoxColorType::COUNT`] | [`SDL_MESSAGEBOX_COLOR_COUNT`] | Size of the colors array of SDL_MessageBoxColorScheme. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxColorType(pub ::core::ffi::c_int);
impl From<SDL_MessageBoxColorType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_MessageBoxColorType) -> Self {
        value.0
    }
}
impl SDL_MessageBoxColorType {
    pub const BACKGROUND: Self = Self(0);
    pub const TEXT: Self = Self(1);
    pub const BUTTON_BORDER: Self = Self(2);
    pub const BUTTON_BACKGROUND: Self = Self(3);
    pub const BUTTON_SELECTED: Self = Self(4);
    /// Size of the colors array of [`SDL_MessageBoxColorScheme`].
    pub const COUNT: Self = Self(5);
}
pub const SDL_MESSAGEBOX_COLOR_BACKGROUND: SDL_MessageBoxColorType =
    SDL_MessageBoxColorType::BACKGROUND;
pub const SDL_MESSAGEBOX_COLOR_TEXT: SDL_MessageBoxColorType = SDL_MessageBoxColorType::TEXT;
pub const SDL_MESSAGEBOX_COLOR_BUTTON_BORDER: SDL_MessageBoxColorType =
    SDL_MessageBoxColorType::BUTTON_BORDER;
pub const SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND: SDL_MessageBoxColorType =
    SDL_MessageBoxColorType::BUTTON_BACKGROUND;
pub const SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED: SDL_MessageBoxColorType =
    SDL_MessageBoxColorType::BUTTON_SELECTED;
/// Size of the colors array of [`SDL_MessageBoxColorScheme`].
pub const SDL_MESSAGEBOX_COLOR_COUNT: SDL_MessageBoxColorType = SDL_MessageBoxColorType::COUNT;

/// A set of colors to use for message box dialogs
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxColorScheme {
    pub colors: [SDL_MessageBoxColor; SDL_MESSAGEBOX_COLOR_COUNT.0 as ::core::primitive::usize],
}

/// MessageBox structure containing title, text, window, etc.
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxData {
    pub flags: SDL_MessageBoxFlags,
    /// Parent window, can be NULL
    pub window: *mut SDL_Window,
    /// UTF-8 title
    pub title: *const ::core::ffi::c_char,
    /// UTF-8 message text
    pub message: *const ::core::ffi::c_char,
    pub numbuttons: ::core::ffi::c_int,
    pub buttons: *const SDL_MessageBoxButtonData,
    /// [`SDL_MessageBoxColorScheme`], can be NULL to use system settings
    pub colorScheme: *const SDL_MessageBoxColorScheme,
}

extern "C" {
    /// Create a modal message box.
    ///
    /// If your needs aren't complex, it might be easier to use
    /// [`SDL_ShowSimpleMessageBox`].
    ///
    /// This function should be called on the thread that created the parent
    /// window, or on the main thread if the messagebox has no parent. It will
    /// block execution of that thread until the user clicks a button or closes the
    /// messagebox.
    ///
    /// This function may be called at any time, even before [`SDL_Init()`]. This makes
    /// it useful for reporting errors like a failure to create a renderer or
    /// OpenGL context.
    ///
    /// On X11, SDL rolls its own dialog box with X11 primitives instead of a
    /// formal toolkit like GTK+ or Qt.
    ///
    /// Note that if [`SDL_Init()`] would fail because there isn't any available video
    /// target, this function is likely to fail for the same reasons. If this is a
    /// concern, check the return value from this function and fall back to writing
    /// to stderr if you can.
    ///
    /// ### Arguments
    /// - `messageboxdata`: the [`SDL_MessageBoxData`] structure with title, text and
    ///   other options.
    /// - `buttonid`: the pointer to which user id of hit button should be
    ///   copied.
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_ShowSimpleMessageBox`]
    pub fn SDL_ShowMessageBox(
        messageboxdata: *const SDL_MessageBoxData,
        buttonid: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Display a simple modal message box.
    ///
    /// If your needs aren't complex, this function is preferred over
    /// [`SDL_ShowMessageBox`].
    ///
    /// `flags` may be any of the following:
    ///
    /// - `SDL_MESSAGEBOX_ERROR`: error dialog
    /// - `SDL_MESSAGEBOX_WARNING`: warning dialog
    /// - `SDL_MESSAGEBOX_INFORMATION`: informational dialog
    ///
    /// This function should be called on the thread that created the parent
    /// window, or on the main thread if the messagebox has no parent. It will
    /// block execution of that thread until the user clicks a button or closes the
    /// messagebox.
    ///
    /// This function may be called at any time, even before [`SDL_Init()`]. This makes
    /// it useful for reporting errors like a failure to create a renderer or
    /// OpenGL context.
    ///
    /// On X11, SDL rolls its own dialog box with X11 primitives instead of a
    /// formal toolkit like GTK+ or Qt.
    ///
    /// Note that if [`SDL_Init()`] would fail because there isn't any available video
    /// target, this function is likely to fail for the same reasons. If this is a
    /// concern, check the return value from this function and fall back to writing
    /// to stderr if you can.
    ///
    /// ### Arguments
    /// - `flags`: an [`SDL_MessageBoxFlags`] value.
    /// - `title`: uTF-8 title text.
    /// - `message`: uTF-8 message text.
    /// - `window`: the parent window, or NULL for no parent.
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_ShowMessageBox`]
    pub fn SDL_ShowSimpleMessageBox(
        flags: SDL_MessageBoxFlags,
        title: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
        window: *mut SDL_Window,
    ) -> ::core::primitive::bool;
}

#[cfg(doc)]
use crate::everything::*;
