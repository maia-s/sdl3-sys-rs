use sdl3_sys::everything::*;

use super::ttf::*;

/// A font atlas draw command.
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NOOP`](TTF_DrawCommand::NOOP) | [`TTF_DRAW_COMMAND_NOOP`] | |
/// | [`FILL`](TTF_DrawCommand::FILL) | [`TTF_DRAW_COMMAND_FILL`] | |
/// | [`COPY`](TTF_DrawCommand::COPY) | [`TTF_DRAW_COMMAND_COPY`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_DrawCommand(pub ::core::ffi::c_int);

impl From<TTF_DrawCommand> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: TTF_DrawCommand) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_DrawCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NOOP => "TTF_DRAW_COMMAND_NOOP",
            Self::FILL => "TTF_DRAW_COMMAND_FILL",
            Self::COPY => "TTF_DRAW_COMMAND_COPY",

            _ => return write!(f, "TTF_DrawCommand({})", self.0),
        })
    }
}

impl TTF_DrawCommand {
    pub const NOOP: Self = Self((0 as ::core::ffi::c_int));
    pub const FILL: Self = Self((1 as ::core::ffi::c_int));
    pub const COPY: Self = Self((2 as ::core::ffi::c_int));
}

pub const TTF_DRAW_COMMAND_NOOP: TTF_DrawCommand = TTF_DrawCommand::NOOP;
pub const TTF_DRAW_COMMAND_FILL: TTF_DrawCommand = TTF_DrawCommand::FILL;
pub const TTF_DRAW_COMMAND_COPY: TTF_DrawCommand = TTF_DrawCommand::COPY;

/// A filled rectangle draw operation.
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_DrawOperation`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct TTF_FillOperation {
    /// [`TTF_DRAW_COMMAND_FILL`]
    pub cmd: TTF_DrawCommand,
    /// The rectangle to fill, in pixels. The x coordinate is relative to the left side of the text area, going right, and the y coordinate is relative to the top side of the text area, going down.
    pub rect: SDL_Rect,
}

/// A texture copy draw operation.
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_DrawOperation`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct TTF_CopyOperation {
    /// [`TTF_DRAW_COMMAND_COPY`]
    pub cmd: TTF_DrawCommand,
    /// The offset in the text corresponding to this glyph.
    /// There may be multiple glyphs with the same text offset
    /// and the next text offset might be several Unicode codepoints
    /// later. In this case the glyphs and codepoints are grouped
    /// together and the group bounding box is the union of the dst
    /// rectangles for the corresponding glyphs.
    pub text_offset: ::core::ffi::c_int,
    /// The font containing the glyph to be drawn, can be passed to [`TTF_GetGlyphImageForIndex()`]
    pub glyph_font: *mut TTF_Font,
    /// The glyph index of the glyph to be drawn, can be passed to [`TTF_GetGlyphImageForIndex()`]
    pub glyph_index: Uint32,
    /// The area within the glyph to be drawn
    pub src: SDL_Rect,
    /// The drawing coordinates of the glyph, in pixels. The x coordinate is relative to the left side of the text area, going right, and the y coordinate is relative to the top side of the text area, going down.
    pub dst: SDL_Rect,
    pub reserved: *mut ::core::ffi::c_void,
}

impl ::core::default::Default for TTF_CopyOperation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A text engine draw operation.
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
pub union TTF_DrawOperation {
    pub cmd: TTF_DrawCommand,
    pub fill: TTF_FillOperation,
    pub copy: TTF_CopyOperation,
}

impl ::core::default::Default for TTF_DrawOperation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A text engine interface.
///
/// This structure should be initialized using [`SDL_INIT_INTERFACE()`]
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`SDL_INIT_INTERFACE`]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct TTF_TextEngine {
    /// The version of this interface
    pub version: Uint32,
    /// User data pointer passed to callbacks
    pub userdata: *mut ::core::ffi::c_void,
    pub CreateText: ::core::option::Option<
        unsafe extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            text: *mut TTF_Text,
        ) -> ::core::primitive::bool,
    >,
    /// * Destroy a text representation.
    pub DestroyText: ::core::option::Option<
        unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, text: *mut TTF_Text),
    >,
}

impl TTF_TextEngine {
    /// Create a new `TTF_TextEngine` initialized with `SDL_INIT_INTERFACE`
    #[inline]
    pub const fn new() -> Self {
        const { ::core::assert!(::core::mem::size_of::<Self>() <= ::core::primitive::u32::MAX as usize) };
        let mut this = unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        this.version = ::core::mem::size_of::<Self>() as ::core::primitive::u32;
        this
    }
}

impl ::core::default::Default for TTF_TextEngine {
    /// Create a new `TTF_TextEngine` initialized with `SDL_INIT_INTERFACE`
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const _: () = ::core::assert!(
    (((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 4_usize)
        && (::core::mem::size_of::<TTF_TextEngine>() == 16_usize))
        || ((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8_usize)
            && (::core::mem::size_of::<TTF_TextEngine>() == 32_usize)))
);

#[repr(C)]
pub struct TTF_TextLayout {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
