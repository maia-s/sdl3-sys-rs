//! SDL offers a simple message box API, which is useful for simple alerts,
//! such as informing the user when something fatal happens at startup without
//! the need to build a UI for it (or informing the user _before_ your UI is
//! ready).
//!
//! These message boxes are native system dialogs where possible.
//!
//! There is both a customizable function ([`SDL_ShowMessageBox()`]) that offers
//! lots of options for what to display and reports on what choice the user
//! made, and also a much-simplified version ([`SDL_ShowSimpleMessageBox()`]),
//! merely takes a text message and title, and waits until the user presses a
//! single "OK" UI button. Often, this is all that is necessary.

use super::stdinc::*;

use super::error::*;

use super::video::*;

/// Message box flags.
///
/// If supported will display warning icon, etc.
///
/// ### Availability
/// This datatype is available since SDL 3.2.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`ERROR`](SDL_MessageBoxFlags::ERROR) | [`SDL_MESSAGEBOX_ERROR`] | error dialog |
/// | [`WARNING`](SDL_MessageBoxFlags::WARNING) | [`SDL_MESSAGEBOX_WARNING`] | warning dialog |
/// | [`INFORMATION`](SDL_MessageBoxFlags::INFORMATION) | [`SDL_MESSAGEBOX_INFORMATION`] | informational dialog |
/// | [`BUTTONS_LEFT_TO_RIGHT`](SDL_MessageBoxFlags::BUTTONS_LEFT_TO_RIGHT) | [`SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT`] | buttons placed left to right |
/// | [`BUTTONS_RIGHT_TO_LEFT`](SDL_MessageBoxFlags::BUTTONS_RIGHT_TO_LEFT) | [`SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT`] | buttons placed right to left |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_MessageBoxFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_MessageBoxFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl From<SDL_MessageBoxFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_MessageBoxFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MessageBoxFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_MessageBoxFlags(")?;
        let all_bits = all_bits | Self::ERROR.0;
        if (Self::ERROR != 0 || self.0 == 0) && *self & Self::ERROR == Self::ERROR {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "ERROR")?;
        }
        let all_bits = all_bits | Self::WARNING.0;
        if (Self::WARNING != 0 || self.0 == 0) && *self & Self::WARNING == Self::WARNING {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "WARNING")?;
        }
        let all_bits = all_bits | Self::INFORMATION.0;
        if (Self::INFORMATION != 0 || self.0 == 0) && *self & Self::INFORMATION == Self::INFORMATION
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "INFORMATION")?;
        }
        let all_bits = all_bits | Self::BUTTONS_LEFT_TO_RIGHT.0;
        if (Self::BUTTONS_LEFT_TO_RIGHT != 0 || self.0 == 0)
            && *self & Self::BUTTONS_LEFT_TO_RIGHT == Self::BUTTONS_LEFT_TO_RIGHT
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTONS_LEFT_TO_RIGHT")?;
        }
        let all_bits = all_bits | Self::BUTTONS_RIGHT_TO_LEFT.0;
        if (Self::BUTTONS_RIGHT_TO_LEFT != 0 || self.0 == 0)
            && *self & Self::BUTTONS_RIGHT_TO_LEFT == Self::BUTTONS_RIGHT_TO_LEFT
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTONS_RIGHT_TO_LEFT")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_MessageBoxFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_MessageBoxFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_MessageBoxFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_MessageBoxFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_MessageBoxFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_MessageBoxFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_MessageBoxFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_MessageBoxFlags {
    /// error dialog
    pub const ERROR: Self = Self((0x00000010 as Uint32));
    /// warning dialog
    pub const WARNING: Self = Self((0x00000020 as Uint32));
    /// informational dialog
    pub const INFORMATION: Self = Self((0x00000040 as Uint32));
    /// buttons placed left to right
    pub const BUTTONS_LEFT_TO_RIGHT: Self = Self((0x00000080 as Uint32));
    /// buttons placed right to left
    pub const BUTTONS_RIGHT_TO_LEFT: Self = Self((0x00000100 as Uint32));
}

/// error dialog
pub const SDL_MESSAGEBOX_ERROR: SDL_MessageBoxFlags = SDL_MessageBoxFlags::ERROR;
/// warning dialog
pub const SDL_MESSAGEBOX_WARNING: SDL_MessageBoxFlags = SDL_MessageBoxFlags::WARNING;
/// informational dialog
pub const SDL_MESSAGEBOX_INFORMATION: SDL_MessageBoxFlags = SDL_MessageBoxFlags::INFORMATION;
/// buttons placed left to right
pub const SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT: SDL_MessageBoxFlags =
    SDL_MessageBoxFlags::BUTTONS_LEFT_TO_RIGHT;
/// buttons placed right to left
pub const SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT: SDL_MessageBoxFlags =
    SDL_MessageBoxFlags::BUTTONS_RIGHT_TO_LEFT;

/// [`SDL_MessageBoxButtonData`] flags.
///
/// ### Availability
/// This datatype is available since SDL 3.2.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`RETURNKEY_DEFAULT`](SDL_MessageBoxButtonFlags::RETURNKEY_DEFAULT) | [`SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT`] | Marks the default button when return is hit |
/// | [`ESCAPEKEY_DEFAULT`](SDL_MessageBoxButtonFlags::ESCAPEKEY_DEFAULT) | [`SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT`] | Marks the default button when escape is hit |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_MessageBoxButtonFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_MessageBoxButtonFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl From<SDL_MessageBoxButtonFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_MessageBoxButtonFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MessageBoxButtonFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_MessageBoxButtonFlags(")?;
        let all_bits = all_bits | Self::RETURNKEY_DEFAULT.0;
        if (Self::RETURNKEY_DEFAULT != 0 || self.0 == 0)
            && *self & Self::RETURNKEY_DEFAULT == Self::RETURNKEY_DEFAULT
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "RETURNKEY_DEFAULT")?;
        }
        let all_bits = all_bits | Self::ESCAPEKEY_DEFAULT.0;
        if (Self::ESCAPEKEY_DEFAULT != 0 || self.0 == 0)
            && *self & Self::ESCAPEKEY_DEFAULT == Self::ESCAPEKEY_DEFAULT
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "ESCAPEKEY_DEFAULT")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_MessageBoxButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_MessageBoxButtonFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_MessageBoxButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_MessageBoxButtonFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_MessageBoxButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_MessageBoxButtonFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_MessageBoxButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_MessageBoxButtonFlags {
    /// Marks the default button when return is hit
    pub const RETURNKEY_DEFAULT: Self = Self((0x00000001 as Uint32));
    /// Marks the default button when escape is hit
    pub const ESCAPEKEY_DEFAULT: Self = Self((0x00000002 as Uint32));
}

/// Marks the default button when return is hit
pub const SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT: SDL_MessageBoxButtonFlags =
    SDL_MessageBoxButtonFlags::RETURNKEY_DEFAULT;
/// Marks the default button when escape is hit
pub const SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT: SDL_MessageBoxButtonFlags =
    SDL_MessageBoxButtonFlags::ESCAPEKEY_DEFAULT;

/// Individual button data.
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
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

impl ::core::default::Default for SDL_MessageBoxButtonData {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// RGB value used in a message box color scheme
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MessageBoxColorType(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_MessageBoxColorType {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

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
    pub const BACKGROUND: Self = Self((0 as ::core::ffi::c_int));
    pub const TEXT: Self = Self((1 as ::core::ffi::c_int));
    pub const BUTTON_BORDER: Self = Self((2 as ::core::ffi::c_int));
    pub const BUTTON_BACKGROUND: Self = Self((3 as ::core::ffi::c_int));
    pub const BUTTON_SELECTED: Self = Self((4 as ::core::ffi::c_int));
    /// Size of the colors array of [`SDL_MessageBoxColorScheme`].
    pub const COUNT: Self = Self((5 as ::core::ffi::c_int));
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
/// This struct is available since SDL 3.2.0.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MessageBoxColorScheme {
    pub colors: [SDL_MessageBoxColor; SDL_MESSAGEBOX_COLOR_COUNT.0 as ::core::primitive::usize],
}

/// MessageBox structure containing title, text, window, etc.
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
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

impl ::core::default::Default for SDL_MessageBoxData {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
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
    /// This function is available since SDL 3.2.0.
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
    /// - [`SDL_MESSAGEBOX_ERROR`]\: error dialog
    /// - [`SDL_MESSAGEBOX_WARNING`]\: warning dialog
    /// - [`SDL_MESSAGEBOX_INFORMATION`]\: informational dialog
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
    /// This function is available since SDL 3.2.0.
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
