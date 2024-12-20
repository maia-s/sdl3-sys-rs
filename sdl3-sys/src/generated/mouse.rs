//! SDL mouse handling.

use super::stdinc::*;

use super::error::*;

use super::surface::*;

use super::video::*;

/// This is a unique ID for a mouse for the time it is connected to the system,
/// and is never reused for the lifetime of the application.
///
/// If the mouse is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_MouseID = Uint32;

/// Cursor types for [`SDL_CreateSystemCursor()`].
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`DEFAULT`](SDL_SystemCursor::DEFAULT) | [`SDL_SYSTEM_CURSOR_DEFAULT`] | Default cursor. Usually an arrow. |
/// | [`TEXT`](SDL_SystemCursor::TEXT) | [`SDL_SYSTEM_CURSOR_TEXT`] | Text selection. Usually an I-beam. |
/// | [`WAIT`](SDL_SystemCursor::WAIT) | [`SDL_SYSTEM_CURSOR_WAIT`] | Wait. Usually an hourglass or watch or spinning ball. |
/// | [`CROSSHAIR`](SDL_SystemCursor::CROSSHAIR) | [`SDL_SYSTEM_CURSOR_CROSSHAIR`] | Crosshair. |
/// | [`PROGRESS`](SDL_SystemCursor::PROGRESS) | [`SDL_SYSTEM_CURSOR_PROGRESS`] | Program is busy but still interactive. Usually it's WAIT with an arrow. |
/// | [`NWSE_RESIZE`](SDL_SystemCursor::NWSE_RESIZE) | [`SDL_SYSTEM_CURSOR_NWSE_RESIZE`] | Double arrow pointing northwest and southeast. |
/// | [`NESW_RESIZE`](SDL_SystemCursor::NESW_RESIZE) | [`SDL_SYSTEM_CURSOR_NESW_RESIZE`] | Double arrow pointing northeast and southwest. |
/// | [`EW_RESIZE`](SDL_SystemCursor::EW_RESIZE) | [`SDL_SYSTEM_CURSOR_EW_RESIZE`] | Double arrow pointing west and east. |
/// | [`NS_RESIZE`](SDL_SystemCursor::NS_RESIZE) | [`SDL_SYSTEM_CURSOR_NS_RESIZE`] | Double arrow pointing north and south. |
/// | [`MOVE`](SDL_SystemCursor::MOVE) | [`SDL_SYSTEM_CURSOR_MOVE`] | Four pointed arrow pointing north, south, east, and west. |
/// | [`NOT_ALLOWED`](SDL_SystemCursor::NOT_ALLOWED) | [`SDL_SYSTEM_CURSOR_NOT_ALLOWED`] | Not permitted. Usually a slashed circle or crossbones. |
/// | [`POINTER`](SDL_SystemCursor::POINTER) | [`SDL_SYSTEM_CURSOR_POINTER`] | Pointer that indicates a link. Usually a pointing hand. |
/// | [`NW_RESIZE`](SDL_SystemCursor::NW_RESIZE) | [`SDL_SYSTEM_CURSOR_NW_RESIZE`] | Window resize top-left. This may be a single arrow or a double arrow like NWSE_RESIZE. |
/// | [`N_RESIZE`](SDL_SystemCursor::N_RESIZE) | [`SDL_SYSTEM_CURSOR_N_RESIZE`] | Window resize top. May be NS_RESIZE. |
/// | [`NE_RESIZE`](SDL_SystemCursor::NE_RESIZE) | [`SDL_SYSTEM_CURSOR_NE_RESIZE`] | Window resize top-right. May be NESW_RESIZE. |
/// | [`E_RESIZE`](SDL_SystemCursor::E_RESIZE) | [`SDL_SYSTEM_CURSOR_E_RESIZE`] | Window resize right. May be EW_RESIZE. |
/// | [`SE_RESIZE`](SDL_SystemCursor::SE_RESIZE) | [`SDL_SYSTEM_CURSOR_SE_RESIZE`] | Window resize bottom-right. May be NWSE_RESIZE. |
/// | [`S_RESIZE`](SDL_SystemCursor::S_RESIZE) | [`SDL_SYSTEM_CURSOR_S_RESIZE`] | Window resize bottom. May be NS_RESIZE. |
/// | [`SW_RESIZE`](SDL_SystemCursor::SW_RESIZE) | [`SDL_SYSTEM_CURSOR_SW_RESIZE`] | Window resize bottom-left. May be NESW_RESIZE. |
/// | [`W_RESIZE`](SDL_SystemCursor::W_RESIZE) | [`SDL_SYSTEM_CURSOR_W_RESIZE`] | Window resize left. May be EW_RESIZE. |
/// | [`COUNT`](SDL_SystemCursor::COUNT) | [`SDL_SYSTEM_CURSOR_COUNT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_SystemCursor(pub ::core::ffi::c_int);

impl From<SDL_SystemCursor> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_SystemCursor) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_SystemCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::DEFAULT => "SDL_SYSTEM_CURSOR_DEFAULT",
            Self::TEXT => "SDL_SYSTEM_CURSOR_TEXT",
            Self::WAIT => "SDL_SYSTEM_CURSOR_WAIT",
            Self::CROSSHAIR => "SDL_SYSTEM_CURSOR_CROSSHAIR",
            Self::PROGRESS => "SDL_SYSTEM_CURSOR_PROGRESS",
            Self::NWSE_RESIZE => "SDL_SYSTEM_CURSOR_NWSE_RESIZE",
            Self::NESW_RESIZE => "SDL_SYSTEM_CURSOR_NESW_RESIZE",
            Self::EW_RESIZE => "SDL_SYSTEM_CURSOR_EW_RESIZE",
            Self::NS_RESIZE => "SDL_SYSTEM_CURSOR_NS_RESIZE",
            Self::MOVE => "SDL_SYSTEM_CURSOR_MOVE",
            Self::NOT_ALLOWED => "SDL_SYSTEM_CURSOR_NOT_ALLOWED",
            Self::POINTER => "SDL_SYSTEM_CURSOR_POINTER",
            Self::NW_RESIZE => "SDL_SYSTEM_CURSOR_NW_RESIZE",
            Self::N_RESIZE => "SDL_SYSTEM_CURSOR_N_RESIZE",
            Self::NE_RESIZE => "SDL_SYSTEM_CURSOR_NE_RESIZE",
            Self::E_RESIZE => "SDL_SYSTEM_CURSOR_E_RESIZE",
            Self::SE_RESIZE => "SDL_SYSTEM_CURSOR_SE_RESIZE",
            Self::S_RESIZE => "SDL_SYSTEM_CURSOR_S_RESIZE",
            Self::SW_RESIZE => "SDL_SYSTEM_CURSOR_SW_RESIZE",
            Self::W_RESIZE => "SDL_SYSTEM_CURSOR_W_RESIZE",
            Self::COUNT => "SDL_SYSTEM_CURSOR_COUNT",

            _ => return write!(f, "SDL_SystemCursor({})", self.0),
        })
    }
}

impl SDL_SystemCursor {
    /// Default cursor. Usually an arrow.
    pub const DEFAULT: Self = Self(0);
    /// Text selection. Usually an I-beam.
    pub const TEXT: Self = Self(1);
    /// Wait. Usually an hourglass or watch or spinning ball.
    pub const WAIT: Self = Self(2);
    /// Crosshair.
    pub const CROSSHAIR: Self = Self(3);
    /// Program is busy but still interactive. Usually it's WAIT with an arrow.
    pub const PROGRESS: Self = Self(4);
    /// Double arrow pointing northwest and southeast.
    pub const NWSE_RESIZE: Self = Self(5);
    /// Double arrow pointing northeast and southwest.
    pub const NESW_RESIZE: Self = Self(6);
    /// Double arrow pointing west and east.
    pub const EW_RESIZE: Self = Self(7);
    /// Double arrow pointing north and south.
    pub const NS_RESIZE: Self = Self(8);
    /// Four pointed arrow pointing north, south, east, and west.
    pub const MOVE: Self = Self(9);
    /// Not permitted. Usually a slashed circle or crossbones.
    pub const NOT_ALLOWED: Self = Self(10);
    /// Pointer that indicates a link. Usually a pointing hand.
    pub const POINTER: Self = Self(11);
    /// Window resize top-left. This may be a single arrow or a double arrow like NWSE_RESIZE.
    pub const NW_RESIZE: Self = Self(12);
    /// Window resize top. May be NS_RESIZE.
    pub const N_RESIZE: Self = Self(13);
    /// Window resize top-right. May be NESW_RESIZE.
    pub const NE_RESIZE: Self = Self(14);
    /// Window resize right. May be EW_RESIZE.
    pub const E_RESIZE: Self = Self(15);
    /// Window resize bottom-right. May be NWSE_RESIZE.
    pub const SE_RESIZE: Self = Self(16);
    /// Window resize bottom. May be NS_RESIZE.
    pub const S_RESIZE: Self = Self(17);
    /// Window resize bottom-left. May be NESW_RESIZE.
    pub const SW_RESIZE: Self = Self(18);
    /// Window resize left. May be EW_RESIZE.
    pub const W_RESIZE: Self = Self(19);
    pub const COUNT: Self = Self(20);
}

/// Default cursor. Usually an arrow.
pub const SDL_SYSTEM_CURSOR_DEFAULT: SDL_SystemCursor = SDL_SystemCursor::DEFAULT;
/// Text selection. Usually an I-beam.
pub const SDL_SYSTEM_CURSOR_TEXT: SDL_SystemCursor = SDL_SystemCursor::TEXT;
/// Wait. Usually an hourglass or watch or spinning ball.
pub const SDL_SYSTEM_CURSOR_WAIT: SDL_SystemCursor = SDL_SystemCursor::WAIT;
/// Crosshair.
pub const SDL_SYSTEM_CURSOR_CROSSHAIR: SDL_SystemCursor = SDL_SystemCursor::CROSSHAIR;
/// Program is busy but still interactive. Usually it's WAIT with an arrow.
pub const SDL_SYSTEM_CURSOR_PROGRESS: SDL_SystemCursor = SDL_SystemCursor::PROGRESS;
/// Double arrow pointing northwest and southeast.
pub const SDL_SYSTEM_CURSOR_NWSE_RESIZE: SDL_SystemCursor = SDL_SystemCursor::NWSE_RESIZE;
/// Double arrow pointing northeast and southwest.
pub const SDL_SYSTEM_CURSOR_NESW_RESIZE: SDL_SystemCursor = SDL_SystemCursor::NESW_RESIZE;
/// Double arrow pointing west and east.
pub const SDL_SYSTEM_CURSOR_EW_RESIZE: SDL_SystemCursor = SDL_SystemCursor::EW_RESIZE;
/// Double arrow pointing north and south.
pub const SDL_SYSTEM_CURSOR_NS_RESIZE: SDL_SystemCursor = SDL_SystemCursor::NS_RESIZE;
/// Four pointed arrow pointing north, south, east, and west.
pub const SDL_SYSTEM_CURSOR_MOVE: SDL_SystemCursor = SDL_SystemCursor::MOVE;
/// Not permitted. Usually a slashed circle or crossbones.
pub const SDL_SYSTEM_CURSOR_NOT_ALLOWED: SDL_SystemCursor = SDL_SystemCursor::NOT_ALLOWED;
/// Pointer that indicates a link. Usually a pointing hand.
pub const SDL_SYSTEM_CURSOR_POINTER: SDL_SystemCursor = SDL_SystemCursor::POINTER;
/// Window resize top-left. This may be a single arrow or a double arrow like NWSE_RESIZE.
pub const SDL_SYSTEM_CURSOR_NW_RESIZE: SDL_SystemCursor = SDL_SystemCursor::NW_RESIZE;
/// Window resize top. May be NS_RESIZE.
pub const SDL_SYSTEM_CURSOR_N_RESIZE: SDL_SystemCursor = SDL_SystemCursor::N_RESIZE;
/// Window resize top-right. May be NESW_RESIZE.
pub const SDL_SYSTEM_CURSOR_NE_RESIZE: SDL_SystemCursor = SDL_SystemCursor::NE_RESIZE;
/// Window resize right. May be EW_RESIZE.
pub const SDL_SYSTEM_CURSOR_E_RESIZE: SDL_SystemCursor = SDL_SystemCursor::E_RESIZE;
/// Window resize bottom-right. May be NWSE_RESIZE.
pub const SDL_SYSTEM_CURSOR_SE_RESIZE: SDL_SystemCursor = SDL_SystemCursor::SE_RESIZE;
/// Window resize bottom. May be NS_RESIZE.
pub const SDL_SYSTEM_CURSOR_S_RESIZE: SDL_SystemCursor = SDL_SystemCursor::S_RESIZE;
/// Window resize bottom-left. May be NESW_RESIZE.
pub const SDL_SYSTEM_CURSOR_SW_RESIZE: SDL_SystemCursor = SDL_SystemCursor::SW_RESIZE;
/// Window resize left. May be EW_RESIZE.
pub const SDL_SYSTEM_CURSOR_W_RESIZE: SDL_SystemCursor = SDL_SystemCursor::W_RESIZE;
pub const SDL_SYSTEM_CURSOR_COUNT: SDL_SystemCursor = SDL_SystemCursor::COUNT;

/// Scroll direction types for the Scroll event
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NORMAL`](SDL_MouseWheelDirection::NORMAL) | [`SDL_MOUSEWHEEL_NORMAL`] | The scroll direction is normal |
/// | [`FLIPPED`](SDL_MouseWheelDirection::FLIPPED) | [`SDL_MOUSEWHEEL_FLIPPED`] | The scroll direction is flipped / natural |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MouseWheelDirection(pub ::core::ffi::c_int);

impl From<SDL_MouseWheelDirection> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_MouseWheelDirection) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MouseWheelDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NORMAL => "SDL_MOUSEWHEEL_NORMAL",
            Self::FLIPPED => "SDL_MOUSEWHEEL_FLIPPED",

            _ => return write!(f, "SDL_MouseWheelDirection({})", self.0),
        })
    }
}

impl SDL_MouseWheelDirection {
    /// The scroll direction is normal
    pub const NORMAL: Self = Self(0);
    /// The scroll direction is flipped / natural
    pub const FLIPPED: Self = Self(1);
}

/// The scroll direction is normal
pub const SDL_MOUSEWHEEL_NORMAL: SDL_MouseWheelDirection = SDL_MouseWheelDirection::NORMAL;
/// The scroll direction is flipped / natural
pub const SDL_MOUSEWHEEL_FLIPPED: SDL_MouseWheelDirection = SDL_MouseWheelDirection::FLIPPED;

/// A bitmask of pressed mouse buttons, as reported by [`SDL_GetMouseState`], etc.
///
/// - Button 1: Left mouse button
/// - Button 2: Middle mouse button
/// - Button 3: Right mouse button
/// - Button 4: Side mouse button 1
/// - Button 5: Side mouse button 2
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetMouseState`]
/// - [`SDL_GetGlobalMouseState`]
/// - [`SDL_GetRelativeMouseState`]
pub type SDL_MouseButtonFlags = Uint32;

pub const SDL_BUTTON_LEFT: ::core::primitive::i32 = 1;

pub const SDL_BUTTON_MIDDLE: ::core::primitive::i32 = 2;

pub const SDL_BUTTON_RIGHT: ::core::primitive::i32 = 3;

pub const SDL_BUTTON_X1: ::core::primitive::i32 = 4;

pub const SDL_BUTTON_X2: ::core::primitive::i32 = 5;

#[inline(always)]
pub const fn SDL_BUTTON_MASK(X: ::core::primitive::i32) -> SDL_MouseButtonFlags {
    ((1_u32 << (X - 1_i32)) as SDL_MouseButtonFlags)
}

pub const SDL_BUTTON_LMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK(SDL_BUTTON_LEFT);

pub const SDL_BUTTON_MMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK(SDL_BUTTON_MIDDLE);

pub const SDL_BUTTON_RMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK(SDL_BUTTON_RIGHT);

pub const SDL_BUTTON_X1MASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK(SDL_BUTTON_X1);

pub const SDL_BUTTON_X2MASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK(SDL_BUTTON_X2);

extern "C" {
    /// Return whether a mouse is currently connected.
    ///
    /// ### Return value
    /// Returns true if a mouse is connected, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetMice`]
    pub fn SDL_HasMouse() -> ::core::primitive::bool;
}

extern "C" {
    /// Get a list of currently connected mice.
    ///
    /// Note that this will include any device or virtual driver that includes
    /// mouse functionality, including some game controllers, KVM switches, etc.
    /// You should wait for input from a device before you consider it actively in
    /// use.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of mice returned, may be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns a 0 terminated array of mouse instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetMouseNameForID`]
    /// - [`SDL_HasMouse`]
    pub fn SDL_GetMice(count: *mut ::core::ffi::c_int) -> *mut SDL_MouseID;
}

extern "C" {
    /// Get the name of a mouse.
    ///
    /// This function returns "" if the mouse doesn't have a name.
    ///
    /// ### Parameters
    /// - `instance_id`: the mouse instance ID.
    ///
    /// ### Return value
    /// Returns the name of the selected mouse, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetMice`]
    pub fn SDL_GetMouseNameForID(instance_id: SDL_MouseID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the window which currently has mouse focus.
    ///
    /// ### Return value
    /// Returns the window with mouse focus.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetMouseFocus() -> *mut SDL_Window;
}

extern "C" {
    /// Retrieve the current state of the mouse.
    ///
    /// The current button state is returned as a button bitmask, which can be
    /// tested using the SDL_BUTTON_MASK(X) macro (where `X` is generally 1 for the
    /// left, 2 for middle, 3 for the right button), and `x` and `y` are set to the
    /// mouse cursor position relative to the focus window. You can pass NULL for
    /// either `x` or `y`.
    ///
    /// ### Parameters
    /// - `x`: the x coordinate of the mouse cursor position relative to the
    ///   focus window.
    /// - `y`: the y coordinate of the mouse cursor position relative to the
    ///   focus window.
    ///
    /// ### Return value
    /// Returns a 32-bit button bitmask of the current button state.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGlobalMouseState`]
    /// - [`SDL_GetRelativeMouseState`]
    pub fn SDL_GetMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

extern "C" {
    /// Get the current state of the mouse in relation to the desktop.
    ///
    /// This works similarly to [`SDL_GetMouseState()`], but the coordinates will be
    /// reported relative to the top-left of the desktop. This can be useful if you
    /// need to track the mouse outside of a specific window and [`SDL_CaptureMouse()`]
    /// doesn't fit your needs. For example, it could be useful if you need to
    /// track the mouse while dragging a window, where coordinates relative to a
    /// window might not be in sync at all times.
    ///
    /// Note: [`SDL_GetMouseState()`] returns the mouse position as SDL understands it
    /// from the last pump of the event queue. This function, however, queries the
    /// OS for the current mouse position, and as such, might be a slightly less
    /// efficient function. Unless you know what you're doing and have a good
    /// reason to use this function, you probably want [`SDL_GetMouseState()`] instead.
    ///
    /// ### Parameters
    /// - `x`: filled in with the current X coord relative to the desktop; can be
    ///   NULL.
    /// - `y`: filled in with the current Y coord relative to the desktop; can be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns the current button state as a bitmask which can be tested using
    ///   the SDL_BUTTON_MASK(X) macros.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CaptureMouse`]
    /// - [`SDL_GetMouseState`]
    pub fn SDL_GetGlobalMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

extern "C" {
    /// Retrieve the relative state of the mouse.
    ///
    /// The current button state is returned as a button bitmask, which can be
    /// tested using the `SDL_BUTTON_MASK(X)` macros (where `X` is generally 1 for
    /// the left, 2 for middle, 3 for the right button), and `x` and `y` are set to
    /// the mouse deltas since the last call to [`SDL_GetRelativeMouseState()`] or
    /// since event initialization. You can pass NULL for either `x` or `y`.
    ///
    /// ### Parameters
    /// - `x`: a pointer filled with the last recorded x coordinate of the mouse.
    /// - `y`: a pointer filled with the last recorded y coordinate of the mouse.
    ///
    /// ### Return value
    /// Returns a 32-bit button bitmask of the relative button state.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetMouseState`]
    pub fn SDL_GetRelativeMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

extern "C" {
    /// Move the mouse cursor to the given position within the window.
    ///
    /// This function generates a mouse motion event if relative mode is not
    /// enabled. If relative mode is enabled, you can force mouse events for the
    /// warp by setting the [`SDL_HINT_MOUSE_RELATIVE_WARP_MOTION`] hint.
    ///
    /// Note that this function will appear to succeed, but not actually move the
    /// mouse when used over Microsoft Remote Desktop.
    ///
    /// ### Parameters
    /// - `window`: the window to move the mouse into, or NULL for the current
    ///   mouse focus.
    /// - `x`: the x coordinate within the window.
    /// - `y`: the y coordinate within the window.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_WarpMouseGlobal`]
    pub fn SDL_WarpMouseInWindow(
        window: *mut SDL_Window,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    );
}

extern "C" {
    /// Move the mouse to the given position in global screen space.
    ///
    /// This function generates a mouse motion event.
    ///
    /// A failure of this function usually means that it is unsupported by a
    /// platform.
    ///
    /// Note that this function will appear to succeed, but not actually move the
    /// mouse when used over Microsoft Remote Desktop.
    ///
    /// ### Parameters
    /// - `x`: the x coordinate.
    /// - `y`: the y coordinate.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_WarpMouseInWindow`]
    pub fn SDL_WarpMouseGlobal(
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set relative mouse mode for a window.
    ///
    /// While the window has focus and relative mouse mode is enabled, the cursor
    /// is hidden, the mouse position is constrained to the window, and SDL will
    /// report continuous relative mouse motion even if the mouse is at the edge of
    /// the window.
    ///
    /// This function will flush any pending mouse motion for this window.
    ///
    /// ### Parameters
    /// - `window`: the window to change.
    /// - `enabled`: true to enable relative mode, false to disable.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetWindowRelativeMouseMode`]
    pub fn SDL_SetWindowRelativeMouseMode(
        window: *mut SDL_Window,
        enabled: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether relative mouse mode is enabled for a window.
    ///
    /// ### Parameters
    /// - `window`: the window to query.
    ///
    /// ### Return value
    /// Returns true if relative mode is enabled for a window or false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetWindowRelativeMouseMode`]
    pub fn SDL_GetWindowRelativeMouseMode(window: *mut SDL_Window) -> ::core::primitive::bool;
}

extern "C" {
    /// Capture the mouse and to track input outside an SDL window.
    ///
    /// Capturing enables your app to obtain mouse events globally, instead of just
    /// within your window. Not all video targets support this function. When
    /// capturing is enabled, the current window will get all mouse events, but
    /// unlike relative mode, no change is made to the cursor and it is not
    /// restrained to your window.
    ///
    /// This function may also deny mouse input to other windows--both those in
    /// your application and others on the system--so you should use this function
    /// sparingly, and in small bursts. For example, you might want to track the
    /// mouse while the user is dragging something, until the user releases a mouse
    /// button. It is not recommended that you capture the mouse for long periods
    /// of time, such as the entire time your app is running. For that, you should
    /// probably use [`SDL_SetWindowRelativeMouseMode()`] or [`SDL_SetWindowMouseGrab()`],
    /// depending on your goals.
    ///
    /// While captured, mouse events still report coordinates relative to the
    /// current (foreground) window, but those coordinates may be outside the
    /// bounds of the window (including negative values). Capturing is only allowed
    /// for the foreground window. If the window loses focus while capturing, the
    /// capture will be disabled automatically.
    ///
    /// While capturing is enabled, the current window will have the
    /// [`SDL_WINDOW_MOUSE_CAPTURE`] flag set.
    ///
    /// Please note that SDL will attempt to "auto capture" the mouse while the
    /// user is pressing a button; this is to try and make mouse behavior more
    /// consistent between platforms, and deal with the common case of a user
    /// dragging the mouse outside of the window. This means that if you are
    /// calling [`SDL_CaptureMouse()`] only to deal with this situation, you do not
    /// have to (although it is safe to do so). If this causes problems for your
    /// app, you can disable auto capture by setting the
    /// [`SDL_HINT_MOUSE_AUTO_CAPTURE`] hint to zero.
    ///
    /// ### Parameters
    /// - `enabled`: true to enable capturing, false to disable.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetGlobalMouseState`]
    pub fn SDL_CaptureMouse(enabled: ::core::primitive::bool) -> ::core::primitive::bool;
}

extern "C" {
    /// Create a cursor using the specified bitmap data and mask (in MSB format).
    ///
    /// `mask` has to be in MSB (Most Significant Bit) format.
    ///
    /// The cursor width (`w`) must be a multiple of 8 bits.
    ///
    /// The cursor is created in black and white according to the following:
    ///
    /// - data=0, mask=1: white
    /// - data=1, mask=1: black
    /// - data=0, mask=0: transparent
    /// - data=1, mask=0: inverted color if possible, black if not.
    ///
    /// Cursors created with this function must be freed with [`SDL_DestroyCursor()`].
    ///
    /// If you want to have a color cursor, or create your cursor from an
    /// [`SDL_Surface`], you should use [`SDL_CreateColorCursor()`]. Alternately, you can
    /// hide the cursor and draw your own as part of your game's rendering, but it
    /// will be bound to the framerate.
    ///
    /// Also, [`SDL_CreateSystemCursor()`] is available, which provides several
    /// readily-available system cursors to pick from.
    ///
    /// ### Parameters
    /// - `data`: the color value for each pixel of the cursor.
    /// - `mask`: the mask value for each pixel of the cursor.
    /// - `w`: the width of the cursor.
    /// - `h`: the height of the cursor.
    /// - `hot_x`: the x-axis offset from the left of the cursor image to the
    ///   mouse x position, in the range of 0 to `w` - 1.
    /// - `hot_y`: the y-axis offset from the top of the cursor image to the
    ///   mouse y position, in the range of 0 to `h` - 1.
    ///
    /// ### Return value
    /// Returns a new cursor with the specified parameters on success or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateColorCursor`]
    /// - [`SDL_CreateSystemCursor`]
    /// - [`SDL_DestroyCursor`]
    /// - [`SDL_SetCursor`]
    pub fn SDL_CreateCursor(
        data: *const Uint8,
        mask: *const Uint8,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        hot_x: ::core::ffi::c_int,
        hot_y: ::core::ffi::c_int,
    ) -> *mut SDL_Cursor;
}

extern "C" {
    /// Create a color cursor.
    ///
    /// If this function is passed a surface with alternate representations, the
    /// surface will be interpreted as the content to be used for 100% display
    /// scale, and the alternate representations will be used for high DPI
    /// situations. For example, if the original surface is 32x32, then on a 2x
    /// macOS display or 200% display scale on Windows, a 64x64 version of the
    /// image will be used, if available. If a matching version of the image isn't
    /// available, the closest larger size image will be downscaled to the
    /// appropriate size and be used instead, if available. Otherwise, the closest
    /// smaller image will be upscaled and be used instead.
    ///
    /// ### Parameters
    /// - `surface`: an [`SDL_Surface`] structure representing the cursor image.
    /// - `hot_x`: the x position of the cursor hot spot.
    /// - `hot_y`: the y position of the cursor hot spot.
    ///
    /// ### Return value
    /// Returns the new cursor on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateCursor`]
    /// - [`SDL_CreateSystemCursor`]
    /// - [`SDL_DestroyCursor`]
    /// - [`SDL_SetCursor`]
    pub fn SDL_CreateColorCursor(
        surface: *mut SDL_Surface,
        hot_x: ::core::ffi::c_int,
        hot_y: ::core::ffi::c_int,
    ) -> *mut SDL_Cursor;
}

extern "C" {
    /// Create a system cursor.
    ///
    /// ### Parameters
    /// - `id`: an [`SDL_SystemCursor`] enum value.
    ///
    /// ### Return value
    /// Returns a cursor on success or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroyCursor`]
    pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;
}

extern "C" {
    /// Set the active cursor.
    ///
    /// This function sets the currently active cursor to the specified one. If the
    /// cursor is currently visible, the change will be immediately represented on
    /// the display. SDL_SetCursor(NULL) can be used to force cursor redraw, if
    /// this is desired for any reason.
    ///
    /// ### Parameters
    /// - `cursor`: a cursor to make active.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCursor`]
    pub fn SDL_SetCursor(cursor: *mut SDL_Cursor) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the active cursor.
    ///
    /// This function returns a pointer to the current cursor which is owned by the
    /// library. It is not necessary to free the cursor with [`SDL_DestroyCursor()`].
    ///
    /// ### Return value
    /// Returns the active cursor or NULL if there is no mouse.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetCursor`]
    pub fn SDL_GetCursor() -> *mut SDL_Cursor;
}

extern "C" {
    /// Get the default cursor.
    ///
    /// You do not have to call [`SDL_DestroyCursor()`] on the return value, but it is
    /// safe to do so.
    ///
    /// ### Return value
    /// Returns the default cursor on success or NULL on failuree; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;
}

extern "C" {
    /// Free a previously-created cursor.
    ///
    /// Use this function to free cursor resources created with [`SDL_CreateCursor()`],
    /// [`SDL_CreateColorCursor()`] or [`SDL_CreateSystemCursor()`].
    ///
    /// ### Parameters
    /// - `cursor`: the cursor to free.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateColorCursor`]
    /// - [`SDL_CreateCursor`]
    /// - [`SDL_CreateSystemCursor`]
    pub fn SDL_DestroyCursor(cursor: *mut SDL_Cursor);
}

extern "C" {
    /// Show the cursor.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CursorVisible`]
    /// - [`SDL_HideCursor`]
    pub fn SDL_ShowCursor() -> ::core::primitive::bool;
}

extern "C" {
    /// Hide the cursor.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CursorVisible`]
    /// - [`SDL_ShowCursor`]
    pub fn SDL_HideCursor() -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether the cursor is currently being shown.
    ///
    /// ### Return value
    /// Returns `true` if the cursor is being shown, or `false` if the cursor is
    ///   hidden.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HideCursor`]
    /// - [`SDL_ShowCursor`]
    pub fn SDL_CursorVisible() -> ::core::primitive::bool;
}

/// The structure used to identify an SDL cursor.
///
/// This is opaque data.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Cursor {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
