//! Any GUI application has to deal with the mouse, and SDL provides functions
//! to manage mouse input and the displayed cursor.
//!
//! Most interactions with the mouse will come through the event subsystem.
//! Moving a mouse generates an [`SDL_EVENT_MOUSE_MOTION`] event, pushing a button
//! generates [`SDL_EVENT_MOUSE_BUTTON_DOWN`], etc, but one can also query the
//! current state of the mouse at any time with [`SDL_GetMouseState()`].
//!
//! For certain games, it's useful to disassociate the mouse cursor from mouse
//! input. An FPS, for example, would not want the player's motion to stop as
//! the mouse hits the edge of the window. For these scenarios, use
//! [`SDL_SetWindowRelativeMouseMode()`], which hides the cursor, grabs mouse input
//! to the window, and reads mouse input no matter how far it moves.
//!
//! Games that want the system to track the mouse but want to draw their own
//! cursor can use [`SDL_HideCursor()`] and [`SDL_ShowCursor()`]. It might be more
//! efficient to let the system manage the cursor, if possible, using
//! [`SDL_SetCursor()`] with a custom image made through [`SDL_CreateColorCursor()`],
//! or perhaps just a specific system cursor from [`SDL_CreateSystemCursor()`].
//!
//! SDL can, on many platforms, differentiate between multiple connected mice,
//! allowing for interesting input scenarios and multiplayer games. They can be
//! enumerated with [`SDL_GetMice()`], and SDL will send [`SDL_EVENT_MOUSE_ADDED`] and
//! [`SDL_EVENT_MOUSE_REMOVED`] events as they are connected and unplugged.
//!
//! Since many apps only care about basic mouse input, SDL offers a virtual
//! mouse device for touch and pen input, which often can make a desktop
//! application work on a touchscreen phone without any code changes. Apps that
//! care about touch/pen separately from mouse input should filter out events
//! with a `which` field of SDL_TOUCH_MOUSEID/SDL_PEN_MOUSEID.

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
/// ## Availability
/// This datatype is available since SDL 3.2.0.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MouseID(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_MouseID {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_MouseID> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_MouseID) -> bool {
        self == &other.0
    }
}

impl From<SDL_MouseID> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_MouseID) -> Self {
        value.0
    }
}

#[cfg(feature = "display-impls")]
impl ::core::fmt::Display for SDL_MouseID {
    #[inline(always)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <Uint32 as ::core::fmt::Display>::fmt(&self.0, f)
    }
}

impl SDL_MouseID {
    /// Initialize a `SDL_MouseID` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: Uint32) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> Uint32 {
        self.0
    }
}

impl SDL_MouseID {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> Uint32 {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_MouseID {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::mouse::METADATA_SDL_MouseID;
}

/// Cursor types for [`SDL_CreateSystemCursor()`].
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
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
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_SystemCursor(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_SystemCursor {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_SystemCursor> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_SystemCursor) -> bool {
        self == &other.0
    }
}

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
    pub const DEFAULT: Self = Self((0 as ::core::ffi::c_int));
    /// Text selection. Usually an I-beam.
    pub const TEXT: Self = Self((1 as ::core::ffi::c_int));
    /// Wait. Usually an hourglass or watch or spinning ball.
    pub const WAIT: Self = Self((2 as ::core::ffi::c_int));
    /// Crosshair.
    pub const CROSSHAIR: Self = Self((3 as ::core::ffi::c_int));
    /// Program is busy but still interactive. Usually it's WAIT with an arrow.
    pub const PROGRESS: Self = Self((4 as ::core::ffi::c_int));
    /// Double arrow pointing northwest and southeast.
    pub const NWSE_RESIZE: Self = Self((5 as ::core::ffi::c_int));
    /// Double arrow pointing northeast and southwest.
    pub const NESW_RESIZE: Self = Self((6 as ::core::ffi::c_int));
    /// Double arrow pointing west and east.
    pub const EW_RESIZE: Self = Self((7 as ::core::ffi::c_int));
    /// Double arrow pointing north and south.
    pub const NS_RESIZE: Self = Self((8 as ::core::ffi::c_int));
    /// Four pointed arrow pointing north, south, east, and west.
    pub const MOVE: Self = Self((9 as ::core::ffi::c_int));
    /// Not permitted. Usually a slashed circle or crossbones.
    pub const NOT_ALLOWED: Self = Self((10 as ::core::ffi::c_int));
    /// Pointer that indicates a link. Usually a pointing hand.
    pub const POINTER: Self = Self((11 as ::core::ffi::c_int));
    /// Window resize top-left. This may be a single arrow or a double arrow like NWSE_RESIZE.
    pub const NW_RESIZE: Self = Self((12 as ::core::ffi::c_int));
    /// Window resize top. May be NS_RESIZE.
    pub const N_RESIZE: Self = Self((13 as ::core::ffi::c_int));
    /// Window resize top-right. May be NESW_RESIZE.
    pub const NE_RESIZE: Self = Self((14 as ::core::ffi::c_int));
    /// Window resize right. May be EW_RESIZE.
    pub const E_RESIZE: Self = Self((15 as ::core::ffi::c_int));
    /// Window resize bottom-right. May be NWSE_RESIZE.
    pub const SE_RESIZE: Self = Self((16 as ::core::ffi::c_int));
    /// Window resize bottom. May be NS_RESIZE.
    pub const S_RESIZE: Self = Self((17 as ::core::ffi::c_int));
    /// Window resize bottom-left. May be NESW_RESIZE.
    pub const SW_RESIZE: Self = Self((18 as ::core::ffi::c_int));
    /// Window resize left. May be EW_RESIZE.
    pub const W_RESIZE: Self = Self((19 as ::core::ffi::c_int));
    pub const COUNT: Self = Self((20 as ::core::ffi::c_int));
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

impl SDL_SystemCursor {
    /// Initialize a `SDL_SystemCursor` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> ::core::ffi::c_int {
        self.0
    }
}

impl SDL_SystemCursor {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_SystemCursor {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::mouse::METADATA_SDL_SystemCursor;
}

/// Scroll direction types for the Scroll event
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NORMAL`](SDL_MouseWheelDirection::NORMAL) | [`SDL_MOUSEWHEEL_NORMAL`] | The scroll direction is normal |
/// | [`FLIPPED`](SDL_MouseWheelDirection::FLIPPED) | [`SDL_MOUSEWHEEL_FLIPPED`] | The scroll direction is flipped / natural |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MouseWheelDirection(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_MouseWheelDirection {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_MouseWheelDirection> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_MouseWheelDirection) -> bool {
        self == &other.0
    }
}

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
    pub const NORMAL: Self = Self((0 as ::core::ffi::c_int));
    /// The scroll direction is flipped / natural
    pub const FLIPPED: Self = Self((1 as ::core::ffi::c_int));
}

/// The scroll direction is normal
pub const SDL_MOUSEWHEEL_NORMAL: SDL_MouseWheelDirection = SDL_MouseWheelDirection::NORMAL;
/// The scroll direction is flipped / natural
pub const SDL_MOUSEWHEEL_FLIPPED: SDL_MouseWheelDirection = SDL_MouseWheelDirection::FLIPPED;

impl SDL_MouseWheelDirection {
    /// Initialize a `SDL_MouseWheelDirection` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> ::core::ffi::c_int {
        self.0
    }
}

impl SDL_MouseWheelDirection {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_MouseWheelDirection {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::mouse::METADATA_SDL_MouseWheelDirection;
}

/// Animated cursor frame info.
///
/// ## Availability
/// This struct is available since SDL 3.4.0.
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_CursorFrameInfo {
    /// The surface data for this frame
    pub surface: *mut SDL_Surface,
    /// The frame duration in milliseconds (a duration of 0 is infinite)
    pub duration: Uint32,
}

impl ::core::default::Default for SDL_CursorFrameInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

pub const SDL_BUTTON_LEFT: ::core::primitive::i32 = 1;

pub const SDL_BUTTON_MIDDLE: ::core::primitive::i32 = 2;

pub const SDL_BUTTON_RIGHT: ::core::primitive::i32 = 3;

pub const SDL_BUTTON_X1: ::core::primitive::i32 = 4;

pub const SDL_BUTTON_X2: ::core::primitive::i32 = 5;

#[inline(always)]
pub const fn SDL_BUTTON_MASK(X: ::core::primitive::i32) -> SDL_MouseButtonFlags {
    SDL_MouseButtonFlags(((1_u32 << (X - 1_i32)) as Uint32))
}

/// A callback used to transform mouse motion delta from raw values.
///
/// This is called during SDL's handling of platform mouse events to scale the
/// values of the resulting motion delta.
///
/// ## Parameters
/// - `userdata`: what was passed as `userdata` to
///   [`SDL_SetRelativeMouseTransform()`].
/// - `timestamp`: the associated time at which this mouse motion event was
///   received.
/// - `window`: the associated window to which this mouse motion event was
///   addressed.
/// - `mouseID`: the associated mouse from which this mouse motion event was
///   emitted.
/// - `x`: pointer to a variable that will be treated as the resulting x-axis
///   motion.
/// - `y`: pointer to a variable that will be treated as the resulting y-axis
///   motion.
///
/// ## Thread safety
/// This callback is called by SDL's internal mouse input
///   processing procedure, which may be a thread separate from the
///   main event loop that is run at realtime priority. Stalling
///   this thread with too much work in the callback can therefore
///   potentially freeze the entire system. Care should be taken
///   with proper synchronization practices when adding other side
///   effects beyond mutation of the x and y values.
///
/// ## Availability
/// This datatype is available since SDL 3.4.0.
///
/// ## See also
/// - [`SDL_SetRelativeMouseTransform`]
pub type SDL_MouseMotionTransformCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        timestamp: Uint64,
        window: *mut SDL_Window,
        mouseID: SDL_MouseID,
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ),
>;

unsafe extern "C" {
    /// Return whether a mouse is currently connected.
    ///
    /// ## Return value
    /// Returns true if a mouse is connected, false otherwise.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetMice`]
    pub fn SDL_HasMouse() -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Get a list of currently connected mice.
    ///
    /// Note that this will include any device or virtual driver that includes
    /// mouse functionality, including some game controllers, KVM switches, etc.
    /// You should wait for input from a device before you consider it actively in
    /// use.
    ///
    /// ## Parameters
    /// - `count`: a pointer filled in with the number of mice returned, may be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a 0 terminated array of mouse instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetMouseNameForID`]
    /// - [`SDL_HasMouse`]
    pub fn SDL_GetMice(count: *mut ::core::ffi::c_int) -> *mut SDL_MouseID;
}

unsafe extern "C" {
    /// Get the name of a mouse.
    ///
    /// This function returns "" if the mouse doesn't have a name.
    ///
    /// ## Parameters
    /// - `instance_id`: the mouse instance ID.
    ///
    /// ## Return value
    /// Returns the name of the selected mouse, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetMice`]
    pub fn SDL_GetMouseNameForID(instance_id: SDL_MouseID) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get the window which currently has mouse focus.
    ///
    /// ## Return value
    /// Returns the window with mouse focus.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetMouseFocus() -> *mut SDL_Window;
}

unsafe extern "C" {
    /// Query SDL's cache for the synchronous mouse button state and the
    /// window-relative SDL-cursor position.
    ///
    /// This function returns the cached synchronous state as SDL understands it
    /// from the last pump of the event queue.
    ///
    /// To query the platform for immediate asynchronous state, use
    /// [`SDL_GetGlobalMouseState`].
    ///
    /// Passing non-NULL pointers to `x` or `y` will write the destination with
    /// respective x or y coordinates relative to the focused window.
    ///
    /// In Relative Mode, the SDL-cursor's position usually contradicts the
    /// platform-cursor's position as manually calculated from
    /// [`SDL_GetGlobalMouseState()`] and [`SDL_GetWindowPosition`].
    ///
    /// ## Parameters
    /// - `x`: a pointer to receive the SDL-cursor's x-position from the focused
    ///   window's top left corner, can be NULL if unused.
    /// - `y`: a pointer to receive the SDL-cursor's y-position from the focused
    ///   window's top left corner, can be NULL if unused.
    ///
    /// ## Return value
    /// Returns a 32-bit bitmask of the button state that can be bitwise-compared
    ///   against the SDL_BUTTON_MASK(X) macro.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetGlobalMouseState`]
    /// - [`SDL_GetRelativeMouseState`]
    pub fn SDL_GetMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

unsafe extern "C" {
    /// Query the platform for the asynchronous mouse button state and the
    /// desktop-relative platform-cursor position.
    ///
    /// This function immediately queries the platform for the most recent
    /// asynchronous state, more costly than retrieving SDL's cached state in
    /// [`SDL_GetMouseState()`].
    ///
    /// Passing non-NULL pointers to `x` or `y` will write the destination with
    /// respective x or y coordinates relative to the desktop.
    ///
    /// In Relative Mode, the platform-cursor's position usually contradicts the
    /// SDL-cursor's position as manually calculated from [`SDL_GetMouseState()`] and
    /// [`SDL_GetWindowPosition`].
    ///
    /// This function can be useful if you need to track the mouse outside of a
    /// specific window and [`SDL_CaptureMouse()`] doesn't fit your needs. For example,
    /// it could be useful if you need to track the mouse while dragging a window,
    /// where coordinates relative to a window might not be in sync at all times.
    ///
    /// ## Parameters
    /// - `x`: a pointer to receive the platform-cursor's x-position from the
    ///   desktop's top left corner, can be NULL if unused.
    /// - `y`: a pointer to receive the platform-cursor's y-position from the
    ///   desktop's top left corner, can be NULL if unused.
    ///
    /// ## Return value
    /// Returns a 32-bit bitmask of the button state that can be bitwise-compared
    ///   against the SDL_BUTTON_MASK(X) macro.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CaptureMouse`]
    /// - [`SDL_GetMouseState`]
    /// - [`SDL_GetGlobalMouseState`]
    pub fn SDL_GetGlobalMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

unsafe extern "C" {
    /// Query SDL's cache for the synchronous mouse button state and accumulated
    /// mouse delta since last call.
    ///
    /// This function returns the cached synchronous state as SDL understands it
    /// from the last pump of the event queue.
    ///
    /// To query the platform for immediate asynchronous state, use
    /// [`SDL_GetGlobalMouseState`].
    ///
    /// Passing non-NULL pointers to `x` or `y` will write the destination with
    /// respective x or y deltas accumulated since the last call to this function
    /// (or since event initialization).
    ///
    /// This function is useful for reducing overhead by processing relative mouse
    /// inputs in one go per-frame instead of individually per-event, at the
    /// expense of losing the order between events within the frame (e.g. quickly
    /// pressing and releasing a button within the same frame).
    ///
    /// ## Parameters
    /// - `x`: a pointer to receive the x mouse delta accumulated since last
    ///   call, can be NULL if unused.
    /// - `y`: a pointer to receive the y mouse delta accumulated since last
    ///   call, can be NULL if unused.
    ///
    /// ## Return value
    /// Returns a 32-bit bitmask of the button state that can be bitwise-compared
    ///   against the SDL_BUTTON_MASK(X) macro.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetMouseState`]
    /// - [`SDL_GetGlobalMouseState`]
    pub fn SDL_GetRelativeMouseState(
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_MouseButtonFlags;
}

unsafe extern "C" {
    /// Move the mouse cursor to the given position within the window.
    ///
    /// This function generates a mouse motion event if relative mode is not
    /// enabled. If relative mode is enabled, you can force mouse events for the
    /// warp by setting the [`SDL_HINT_MOUSE_RELATIVE_WARP_MOTION`] hint.
    ///
    /// Note that this function will appear to succeed, but not actually move the
    /// mouse when used over Microsoft Remote Desktop.
    ///
    /// ## Parameters
    /// - `window`: the window to move the mouse into, or NULL for the current
    ///   mouse focus.
    /// - `x`: the x coordinate within the window.
    /// - `y`: the y coordinate within the window.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WarpMouseGlobal`]
    pub fn SDL_WarpMouseInWindow(
        window: *mut SDL_Window,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    );
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: the x coordinate.
    /// - `y`: the y coordinate.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WarpMouseInWindow`]
    pub fn SDL_WarpMouseGlobal(
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Set a user-defined function by which to transform relative mouse inputs.
    ///
    /// This overrides the relative system scale and relative speed scale hints.
    /// Should be called prior to enabling relative mouse mode, fails otherwise.
    ///
    /// ## Parameters
    /// - `callback`: a callback used to transform relative mouse motion, or NULL
    ///   for default behavior.
    /// - `userdata`: a pointer that will be passed to `callback`.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.4.0.
    pub fn SDL_SetRelativeMouseTransform(
        callback: SDL_MouseMotionTransformCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Set relative mouse mode for a window.
    ///
    /// While the window has focus and relative mouse mode is enabled, the cursor
    /// is hidden, the mouse position is constrained to the window, and SDL will
    /// report continuous relative mouse motion even if the mouse is at the edge of
    /// the window.
    ///
    /// If you'd like to keep the mouse position fixed while in relative mode you
    /// can use [`SDL_SetWindowMouseRect()`]. If you'd like the cursor to be at a
    /// specific location when relative mode ends, you should use
    /// [`SDL_WarpMouseInWindow()`] before disabling relative mode.
    ///
    /// This function will flush any pending mouse motion for this window.
    ///
    /// ## Parameters
    /// - `window`: the window to change.
    /// - `enabled`: true to enable relative mode, false to disable.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetWindowRelativeMouseMode`]
    pub fn SDL_SetWindowRelativeMouseMode(
        window: *mut SDL_Window,
        enabled: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Query whether relative mouse mode is enabled for a window.
    ///
    /// ## Parameters
    /// - `window`: the window to query.
    ///
    /// ## Return value
    /// Returns true if relative mode is enabled for a window or false otherwise.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetWindowRelativeMouseMode`]
    pub fn SDL_GetWindowRelativeMouseMode(window: *mut SDL_Window) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `enabled`: true to enable capturing, false to disable.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetGlobalMouseState`]
    pub fn SDL_CaptureMouse(enabled: ::core::primitive::bool) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `data`: the color value for each pixel of the cursor.
    /// - `mask`: the mask value for each pixel of the cursor.
    /// - `w`: the width of the cursor.
    /// - `h`: the height of the cursor.
    /// - `hot_x`: the x-axis offset from the left of the cursor image to the
    ///   mouse x position, in the range of 0 to `w` - 1.
    /// - `hot_y`: the y-axis offset from the top of the cursor image to the
    ///   mouse y position, in the range of 0 to `h` - 1.
    ///
    /// ## Return value
    /// Returns a new cursor with the specified parameters on success or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateAnimatedCursor`]
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

unsafe extern "C" {
    /// Create a color cursor.
    ///
    /// If this function is passed a surface with alternate representations added
    /// with [`SDL_AddSurfaceAlternateImage()`], the surface will be interpreted as the
    /// content to be used for 100% display scale, and the alternate
    /// representations will be used for high DPI situations if
    /// [`SDL_HINT_MOUSE_DPI_SCALE_CURSORS`] is enabled. For example, if the original
    /// surface is 32x32, then on a 2x macOS display or 200% display scale on
    /// Windows, a 64x64 version of the image will be used, if available. If a
    /// matching version of the image isn't available, the closest larger size
    /// image will be downscaled to the appropriate size and be used instead, if
    /// available. Otherwise, the closest smaller image will be upscaled and be
    /// used instead.
    ///
    /// ## Parameters
    /// - `surface`: an [`SDL_Surface`] structure representing the cursor image.
    /// - `hot_x`: the x position of the cursor hot spot.
    /// - `hot_y`: the y position of the cursor hot spot.
    ///
    /// ## Return value
    /// Returns the new cursor on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AddSurfaceAlternateImage`]
    /// - [`SDL_CreateAnimatedCursor`]
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

unsafe extern "C" {
    /// Create an animated color cursor.
    ///
    /// Animated cursors are composed of a sequential array of frames, specified as
    /// surfaces and durations in an array of [`SDL_CursorFrameInfo`] structs. The hot
    /// spot coordinates are universal to all frames, and all frames must have the
    /// same dimensions.
    ///
    /// Frame durations are specified in milliseconds. A duration of 0 implies an
    /// infinite frame time, and the animation will stop on that frame. To create a
    /// one-shot animation, set the duration of the last frame in the sequence to
    /// 0.
    ///
    /// If this function is passed surfaces with alternate representations added
    /// with [`SDL_AddSurfaceAlternateImage()`], the surfaces will be interpreted as
    /// the content to be used for 100% display scale, and the alternate
    /// representations will be used for high DPI situations. For example, if the
    /// original surfaces are 32x32, then on a 2x macOS display or 200% display
    /// scale on Windows, a 64x64 version of the image will be used, if available.
    /// If a matching version of the image isn't available, the closest larger size
    /// image will be downscaled to the appropriate size and be used instead, if
    /// available. Otherwise, the closest smaller image will be upscaled and be
    /// used instead.
    ///
    /// If the underlying platform does not support animated cursors, this function
    /// will fall back to creating a static color cursor using the first frame in
    /// the sequence.
    ///
    /// ## Parameters
    /// - `frames`: an array of cursor images composing the animation.
    /// - `frame_count`: the number of frames in the sequence.
    /// - `hot_x`: the x position of the cursor hot spot.
    /// - `hot_y`: the y position of the cursor hot spot.
    ///
    /// ## Return value
    /// Returns the new cursor on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.4.0.
    ///
    /// ## See also
    /// - [`SDL_AddSurfaceAlternateImage`]
    /// - [`SDL_CreateCursor`]
    /// - [`SDL_CreateColorCursor`]
    /// - [`SDL_CreateSystemCursor`]
    /// - [`SDL_DestroyCursor`]
    /// - [`SDL_SetCursor`]
    pub fn SDL_CreateAnimatedCursor(
        frames: *mut SDL_CursorFrameInfo,
        frame_count: ::core::ffi::c_int,
        hot_x: ::core::ffi::c_int,
        hot_y: ::core::ffi::c_int,
    ) -> *mut SDL_Cursor;
}

unsafe extern "C" {
    /// Create a system cursor.
    ///
    /// ## Parameters
    /// - `id`: an [`SDL_SystemCursor`] enum value.
    ///
    /// ## Return value
    /// Returns a cursor on success or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_DestroyCursor`]
    pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;
}

unsafe extern "C" {
    /// Set the active cursor.
    ///
    /// This function sets the currently active cursor to the specified one. If the
    /// cursor is currently visible, the change will be immediately represented on
    /// the display. SDL_SetCursor(NULL) can be used to force cursor redraw, if
    /// this is desired for any reason.
    ///
    /// ## Parameters
    /// - `cursor`: a cursor to make active.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCursor`]
    pub fn SDL_SetCursor(cursor: *mut SDL_Cursor) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Get the active cursor.
    ///
    /// This function returns a pointer to the current cursor which is owned by the
    /// library. It is not necessary to free the cursor with [`SDL_DestroyCursor()`].
    ///
    /// ## Return value
    /// Returns the active cursor or NULL if there is no mouse.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetCursor`]
    pub fn SDL_GetCursor() -> *mut SDL_Cursor;
}

unsafe extern "C" {
    /// Get the default cursor.
    ///
    /// You do not have to call [`SDL_DestroyCursor()`] on the return value, but it is
    /// safe to do so.
    ///
    /// ## Return value
    /// Returns the default cursor on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;
}

unsafe extern "C" {
    /// Free a previously-created cursor.
    ///
    /// Use this function to free cursor resources created with [`SDL_CreateCursor()`],
    /// [`SDL_CreateColorCursor()`] or [`SDL_CreateSystemCursor()`].
    ///
    /// ## Parameters
    /// - `cursor`: the cursor to free.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateAnimatedCursor`]
    /// - [`SDL_CreateColorCursor`]
    /// - [`SDL_CreateCursor`]
    /// - [`SDL_CreateSystemCursor`]
    pub fn SDL_DestroyCursor(cursor: *mut SDL_Cursor);
}

unsafe extern "C" {
    /// Show the cursor.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CursorVisible`]
    /// - [`SDL_HideCursor`]
    pub fn SDL_ShowCursor() -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Hide the cursor.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CursorVisible`]
    /// - [`SDL_ShowCursor`]
    pub fn SDL_HideCursor() -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Return whether the cursor is currently being shown.
    ///
    /// ## Return value
    /// Returns `true` if the cursor is being shown, or `false` if the cursor is
    ///   hidden.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_HideCursor`]
    /// - [`SDL_ShowCursor`]
    pub fn SDL_CursorVisible() -> ::core::primitive::bool;
}

/// The structure used to identify an SDL cursor.
///
/// This is opaque data.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_Cursor {
    _opaque: [::core::primitive::u8; 0],
}

/// A bitmask of pressed mouse buttons, as reported by [`SDL_GetMouseState`], etc.
///
/// - Button 1: Left mouse button
/// - Button 2: Middle mouse button
/// - Button 3: Right mouse button
/// - Button 4: Side mouse button 1
/// - Button 5: Side mouse button 2
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetMouseState`]
/// - [`SDL_GetGlobalMouseState`]
/// - [`SDL_GetRelativeMouseState`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`LMASK`](SDL_MouseButtonFlags::LMASK) | [`SDL_BUTTON_LMASK`] | |
/// | [`MMASK`](SDL_MouseButtonFlags::MMASK) | [`SDL_BUTTON_MMASK`] | |
/// | [`RMASK`](SDL_MouseButtonFlags::RMASK) | [`SDL_BUTTON_RMASK`] | |
/// | [`X1MASK`](SDL_MouseButtonFlags::X1MASK) | [`SDL_BUTTON_X1MASK`] | |
/// | [`X2MASK`](SDL_MouseButtonFlags::X2MASK) | [`SDL_BUTTON_X2MASK`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_MouseButtonFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_MouseButtonFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_MouseButtonFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_MouseButtonFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_MouseButtonFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_MouseButtonFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MouseButtonFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_MouseButtonFlags(")?;
        let all_bits = all_bits | Self::LMASK.0;
        if (Self::LMASK != 0 || self.0 == 0) && *self & Self::LMASK == Self::LMASK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "LMASK")?;
        }
        let all_bits = all_bits | Self::MMASK.0;
        if (Self::MMASK != 0 || self.0 == 0) && *self & Self::MMASK == Self::MMASK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "MMASK")?;
        }
        let all_bits = all_bits | Self::RMASK.0;
        if (Self::RMASK != 0 || self.0 == 0) && *self & Self::RMASK == Self::RMASK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "RMASK")?;
        }
        let all_bits = all_bits | Self::X1MASK.0;
        if (Self::X1MASK != 0 || self.0 == 0) && *self & Self::X1MASK == Self::X1MASK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "X1MASK")?;
        }
        let all_bits = all_bits | Self::X2MASK.0;
        if (Self::X2MASK != 0 || self.0 == 0) && *self & Self::X2MASK == Self::X2MASK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "X2MASK")?;
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

impl ::core::ops::BitAnd for SDL_MouseButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_MouseButtonFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_MouseButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_MouseButtonFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_MouseButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_MouseButtonFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_MouseButtonFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_MouseButtonFlags {
    pub const LMASK: Self = SDL_BUTTON_MASK(SDL_BUTTON_LEFT);
    pub const MMASK: Self = SDL_BUTTON_MASK(SDL_BUTTON_MIDDLE);
    pub const RMASK: Self = SDL_BUTTON_MASK(SDL_BUTTON_RIGHT);
    pub const X1MASK: Self = SDL_BUTTON_MASK(SDL_BUTTON_X1);
    pub const X2MASK: Self = SDL_BUTTON_MASK(SDL_BUTTON_X2);
}

pub const SDL_BUTTON_LMASK: SDL_MouseButtonFlags = SDL_MouseButtonFlags::LMASK;
pub const SDL_BUTTON_MMASK: SDL_MouseButtonFlags = SDL_MouseButtonFlags::MMASK;
pub const SDL_BUTTON_RMASK: SDL_MouseButtonFlags = SDL_MouseButtonFlags::RMASK;
pub const SDL_BUTTON_X1MASK: SDL_MouseButtonFlags = SDL_MouseButtonFlags::X1MASK;
pub const SDL_BUTTON_X2MASK: SDL_MouseButtonFlags = SDL_MouseButtonFlags::X2MASK;

impl SDL_MouseButtonFlags {
    /// Initialize a `SDL_MouseButtonFlags` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: Uint32) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> Uint32 {
        self.0
    }
}

impl SDL_MouseButtonFlags {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> Uint32 {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_MouseButtonFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::mouse::METADATA_SDL_MouseButtonFlags;
}

#[cfg(doc)]
use crate::everything::*;
