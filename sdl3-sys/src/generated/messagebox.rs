//! Message box support routines.

use super::stdinc::*;

use super::error::*;

use super::video::*;

/// [`SDL_MessageBox`] flags.
///
/// If supported will display warning icon, etc.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`SDL_MESSAGEBOX_ERROR`] | error dialog |
/// | [`SDL_MESSAGEBOX_WARNING`] | warning dialog |
/// | [`SDL_MESSAGEBOX_INFORMATION`] | informational dialog |
/// | [`SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT`] | buttons placed left to right |
/// | [`SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT`] | buttons placed right to left |
pub type SDL_MessageBoxFlags = Uint32;

/// error dialog
pub const SDL_MESSAGEBOX_ERROR: SDL_MessageBoxFlags = (0x00000010 as SDL_MessageBoxFlags);

/// warning dialog
pub const SDL_MESSAGEBOX_WARNING: SDL_MessageBoxFlags = (0x00000020 as SDL_MessageBoxFlags);

/// informational dialog
pub const SDL_MESSAGEBOX_INFORMATION: SDL_MessageBoxFlags = (0x00000040 as SDL_MessageBoxFlags);

/// buttons placed left to right
pub const SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT: SDL_MessageBoxFlags =
    (0x00000080 as SDL_MessageBoxFlags);

/// buttons placed right to left
pub const SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT: SDL_MessageBoxFlags =
    (0x00000100 as SDL_MessageBoxFlags);

/// [`SDL_MessageBoxButtonData`] flags.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT`] | Marks the default button when return is hit |
/// | [`SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT`] | Marks the default button when escape is hit |
pub type SDL_MessageBoxButtonFlags = Uint32;

/// Marks the default button when return is hit
pub const SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT: SDL_MessageBoxButtonFlags =
    (0x00000001 as SDL_MessageBoxButtonFlags);

/// Marks the default button when escape is hit
pub const SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT: SDL_MessageBoxButtonFlags =
    (0x00000002 as SDL_MessageBoxButtonFlags);

/// Individual button data.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
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
/// This struct is available since SDL 3.1.3.
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
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`BACKGROUND`](SDL_MessageBoxColorType::BACKGROUND) | [`SDL_MESSAGEBOX_COLOR_BACKGROUND`] | |
/// | [`TEXT`](SDL_MessageBoxColorType::TEXT) | [`SDL_MESSAGEBOX_COLOR_TEXT`] | |
/// | [`BUTTON_BORDER`](SDL_MessageBoxColorType::BUTTON_BORDER) | [`SDL_MESSAGEBOX_COLOR_BUTTON_BORDER`] | |
/// | [`BUTTON_BACKGROUND`](SDL_MessageBoxColorType::BUTTON_BACKGROUND) | [`SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND`] | |
/// | [`BUTTON_SELECTED`](SDL_MessageBoxColorType::BUTTON_SELECTED) | [`SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED`] | |
/// | [`COUNT`](SDL_MessageBoxColorType::COUNT) | [`SDL_MESSAGEBOX_COLOR_COUNT`] | Size of the colors array of [`SDL_MessageBoxColorScheme`]. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MessageBoxColorType(pub ::core::ffi::c_int);

impl From<SDL_MessageBoxColorType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_MessageBoxColorType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MessageBoxColorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::BACKGROUND => "SDL_MESSAGEBOX_COLOR_BACKGROUND",
            Self::TEXT => "SDL_MESSAGEBOX_COLOR_TEXT",
            Self::BUTTON_BORDER => "SDL_MESSAGEBOX_COLOR_BUTTON_BORDER",
            Self::BUTTON_BACKGROUND => "SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND",
            Self::BUTTON_SELECTED => "SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED",
            Self::COUNT => "SDL_MESSAGEBOX_COLOR_COUNT",

            _ => return write!(f, "SDL_MessageBoxColorType({})", self.0),
        })
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
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxColorScheme {
    pub colors: [SDL_MessageBoxColor; SDL_MESSAGEBOX_COLOR_COUNT.0 as ::core::primitive::usize],
}

/// MessageBox structure containing title, text, window, etc.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
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
    /// ### Parameters
    /// - `messageboxdata`: the [`SDL_MessageBoxData`] structure with title, text and
    ///   other options.
    /// - `buttonid`: the pointer to which user id of hit button should be
    ///   copied.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
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
    /// - [`SDL_MESSAGEBOX_ERROR`]: error dialog
    /// - [`SDL_MESSAGEBOX_WARNING`]: warning dialog
    /// - [`SDL_MESSAGEBOX_INFORMATION`]: informational dialog
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
    /// ### Parameters
    /// - `flags`: an [`SDL_MessageBoxFlags`] value.
    /// - `title`: UTF-8 title text.
    /// - `message`: UTF-8 message text.
    /// - `window`: the parent window, or NULL for no parent.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
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
