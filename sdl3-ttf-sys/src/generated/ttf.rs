//! Header file for SDL_ttf library
//!
//! This library is a wrapper around the excellent FreeType 2.0 library,
//! available at: <https://www.freetype.org/>

use sdl3_sys::everything::*;

/// * Printable format: "%d.%d.%d", MAJOR, MINOR, MICRO
pub const SDL_TTF_MAJOR_VERSION: ::core::primitive::i32 = 3;

pub const SDL_TTF_MINOR_VERSION: ::core::primitive::i32 = 2;

pub const SDL_TTF_MICRO_VERSION: ::core::primitive::i32 = 0;

/// * This is the version number macro for the current SDL_ttf version.
pub const SDL_TTF_VERSION: ::core::primitive::i32 = SDL_VERSIONNUM(
    SDL_TTF_MAJOR_VERSION,
    SDL_TTF_MINOR_VERSION,
    SDL_TTF_MICRO_VERSION,
);

/// * This macro will evaluate to true if compiled with SDL_ttf at least X.Y.Z.
#[inline(always)]
pub const fn SDL_TTF_VERSION_ATLEAST(
    X: ::core::primitive::i32,
    Y: ::core::primitive::i32,
    Z: ::core::primitive::i32,
) -> ::core::primitive::bool {
    (((SDL_TTF_MAJOR_VERSION >= X)
        && ((SDL_TTF_MAJOR_VERSION > X) || (SDL_TTF_MINOR_VERSION >= Y)))
        && (((SDL_TTF_MAJOR_VERSION > X) || (SDL_TTF_MINOR_VERSION > Y))
            || (SDL_TTF_MICRO_VERSION >= Z)))
}

extern "C" {
    /// This function gets the version of the dynamically linked SDL_ttf library.
    ///
    /// ### Return value
    /// Returns SDL_ttf version.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_Version() -> ::core::ffi::c_int;
}

extern "C" {
    /// Query the version of the FreeType library in use.
    ///
    /// [`TTF_Init()`] should be called before calling this function.
    ///
    /// ### Parameters
    /// - `major`: to be filled in with the major version number. Can be NULL.
    /// - `minor`: to be filled in with the minor version number. Can be NULL.
    /// - `patch`: to be filled in with the param version number. Can be NULL.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_Init`]
    pub fn TTF_GetFreeTypeVersion(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        patch: *mut ::core::ffi::c_int,
    );
}

extern "C" {
    /// Query the version of the HarfBuzz library in use.
    ///
    /// If HarfBuzz is not available, the version reported is 0.0.0.
    ///
    /// ### Parameters
    /// - `major`: to be filled in with the major version number. Can be NULL.
    /// - `minor`: to be filled in with the minor version number. Can be NULL.
    /// - `patch`: to be filled in with the param version number. Can be NULL.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetHarfBuzzVersion(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        patch: *mut ::core::ffi::c_int,
    );
}

extern "C" {
    /// Initialize SDL_ttf.
    ///
    /// You must successfully call this function before it is safe to call any
    /// other function in this library.
    ///
    /// It is safe to call this more than once, and each successful [`TTF_Init()`] call
    /// should be paired with a matching [`TTF_Quit()`] call.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_Quit`]
    pub fn TTF_Init() -> ::core::primitive::bool;
}

extern "C" {
    /// Create a font from a file, using a specified point size.
    ///
    /// Some .fon fonts will have several sizes embedded in the file, so the point
    /// size becomes the index of choosing which size. If the value is too high,
    /// the last indexed size will be the default.
    ///
    /// When done with the returned [`TTF_Font`], use [`TTF_CloseFont()`] to dispose of it.
    ///
    /// ### Parameters
    /// - `file`: path to font file.
    /// - `ptsize`: point size to use for the newly-opened font.
    ///
    /// ### Return value
    /// Returns a valid [`TTF_Font`], or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CloseFont`]
    pub fn TTF_OpenFont(
        file: *const ::core::ffi::c_char,
        ptsize: ::core::ffi::c_float,
    ) -> *mut TTF_Font;
}

extern "C" {
    /// Create a font from an [`SDL_IOStream`], using a specified point size.
    ///
    /// Some .fon fonts will have several sizes embedded in the file, so the point
    /// size becomes the index of choosing which size. If the value is too high,
    /// the last indexed size will be the default.
    ///
    /// If `closeio` is true, `src` will be automatically closed once the font is
    /// closed. Otherwise you should close `src` yourself after closing the font.
    ///
    /// When done with the returned [`TTF_Font`], use [`TTF_CloseFont()`] to dispose of it.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to provide a font file's data.
    /// - `closeio`: true to close `src` when the font is closed, false to leave
    ///   it open.
    /// - `ptsize`: point size to use for the newly-opened font.
    ///
    /// ### Return value
    /// Returns a valid [`TTF_Font`], or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CloseFont`]
    pub fn TTF_OpenFontIO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        ptsize: ::core::ffi::c_float,
    ) -> *mut TTF_Font;
}

extern "C" {
    /// Create a font with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`TTF_PROP_FONT_CREATE_FILENAME_STRING`]\: the font file to open, if an
    ///   [`SDL_IOStream`] isn't being used. This is required if
    ///   [`TTF_PROP_FONT_CREATE_IOSTREAM_POINTER`] and
    ///   [`TTF_PROP_FONT_CREATE_EXISTING_FONT`] aren't set.
    /// - [`TTF_PROP_FONT_CREATE_IOSTREAM_POINTER`]\: an [`SDL_IOStream`] containing the
    ///   font to be opened. This should not be closed until the font is closed.
    ///   This is required if [`TTF_PROP_FONT_CREATE_FILENAME_STRING`] and
    ///   [`TTF_PROP_FONT_CREATE_EXISTING_FONT`] aren't set.
    /// - [`TTF_PROP_FONT_CREATE_IOSTREAM_OFFSET_NUMBER`]\: the offset in the iostream
    ///   for the beginning of the font, defaults to 0.
    /// - [`TTF_PROP_FONT_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN`]\: true if closing the
    ///   font should also close the associated [`SDL_IOStream`].
    /// - [`TTF_PROP_FONT_CREATE_SIZE_FLOAT`]\: the point size of the font. Some .fon
    ///   fonts will have several sizes embedded in the file, so the point size
    ///   becomes the index of choosing which size. If the value is too high, the
    ///   last indexed size will be the default.
    /// - [`TTF_PROP_FONT_CREATE_FACE_NUMBER`]\: the face index of the font, if the
    ///   font contains multiple font faces.
    /// - [`TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER`]\: the horizontal DPI to use
    ///   for font rendering, defaults to
    ///   [`TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER`] if set, or 72 otherwise.
    /// - [`TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER`]\: the vertical DPI to use for
    ///   font rendering, defaults to [`TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER`]
    ///   if set, or 72 otherwise.
    /// - [`TTF_PROP_FONT_CREATE_EXISTING_FONT`]\: an optional [`TTF_Font`] that, if set,
    ///   will be used as the font data source and the initial size and style of
    ///   the new font.
    ///
    /// ### Parameters
    /// - `props`: the properties to use.
    ///
    /// ### Return value
    /// Returns a valid [`TTF_Font`], or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CloseFont`]
    pub fn TTF_OpenFontWithProperties(props: SDL_PropertiesID) -> *mut TTF_Font;
}

pub const TTF_PROP_FONT_CREATE_FILENAME_STRING: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.filename".as_ptr();

pub const TTF_PROP_FONT_CREATE_IOSTREAM_POINTER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.iostream".as_ptr();

pub const TTF_PROP_FONT_CREATE_IOSTREAM_OFFSET_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.iostream.offset".as_ptr();

pub const TTF_PROP_FONT_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.iostream.autoclose".as_ptr();

pub const TTF_PROP_FONT_CREATE_SIZE_FLOAT: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.size".as_ptr();

pub const TTF_PROP_FONT_CREATE_FACE_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.face".as_ptr();

pub const TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.hdpi".as_ptr();

pub const TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.vdpi".as_ptr();

pub const TTF_PROP_FONT_CREATE_EXISTING_FONT: *const ::core::ffi::c_char =
    c"SDL_ttf.font.create.existing_font".as_ptr();

extern "C" {
    /// Create a copy of an existing font.
    ///
    /// The copy will be distinct from the original, but will share the font file
    /// and have the same size and style as the original.
    ///
    /// When done with the returned [`TTF_Font`], use [`TTF_CloseFont()`] to dispose of it.
    ///
    /// ### Parameters
    /// - `existing_font`: the font to copy.
    ///
    /// ### Return value
    /// Returns a valid [`TTF_Font`], or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   original font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CloseFont`]
    pub fn TTF_CopyFont(existing_font: *mut TTF_Font) -> *mut TTF_Font;
}

extern "C" {
    /// Get the properties associated with a font.
    ///
    /// The following read-write properties are provided by SDL:
    ///
    /// - [`TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER`]\: The FT_Stroker_LineCap value
    ///   used when setting the font outline, defaults to
    ///   `FT_STROKER_LINECAP_ROUND`.
    /// - [`TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER`]\: The FT_Stroker_LineJoin value
    ///   used when setting the font outline, defaults to
    ///   `FT_STROKER_LINEJOIN_ROUND`.
    /// - [`TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER`]\: The FT_Fixed miter limit used
    ///   when setting the font outline, defaults to 0.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontProperties(font: *mut TTF_Font) -> SDL_PropertiesID;
}

pub const TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.outline.line_cap".as_ptr();

pub const TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.outline.line_join".as_ptr();

pub const TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER: *const ::core::ffi::c_char =
    c"SDL_ttf.font.outline.miter_limit".as_ptr();

extern "C" {
    /// Get the font generation.
    ///
    /// The generation is incremented each time font properties change that require
    /// rebuilding glyphs, such as style, size, etc.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font generation or 0 on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontGeneration(font: *mut TTF_Font) -> Uint32;
}

extern "C" {
    /// Add a fallback font.
    ///
    /// Add a font that will be used for glyphs that are not in the current font.
    /// The fallback font should have the same size and style as the current font.
    ///
    /// If there are multiple fallback fonts, they are used in the order added.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    /// - `fallback`: the font to add as a fallback.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created
    ///   both fonts.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_ClearFallbackFonts`]
    /// - [`TTF_RemoveFallbackFont`]
    pub fn TTF_AddFallbackFont(
        font: *mut TTF_Font,
        fallback: *mut TTF_Font,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove a fallback font.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    /// - `fallback`: the font to remove as a fallback.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created
    ///   both fonts.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_AddFallbackFont`]
    /// - [`TTF_ClearFallbackFonts`]
    pub fn TTF_RemoveFallbackFont(font: *mut TTF_Font, fallback: *mut TTF_Font);
}

extern "C" {
    /// Remove all fallback fonts.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_AddFallbackFont`]
    /// - [`TTF_RemoveFallbackFont`]
    pub fn TTF_ClearFallbackFonts(font: *mut TTF_Font);
}

extern "C" {
    /// Set a font's size dynamically.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// ### Parameters
    /// - `font`: the font to resize.
    /// - `ptsize`: the new point size.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontSize`]
    pub fn TTF_SetFontSize(
        font: *mut TTF_Font,
        ptsize: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set font size dynamically with target resolutions, in dots per inch.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// ### Parameters
    /// - `font`: the font to resize.
    /// - `ptsize`: the new point size.
    /// - `hdpi`: the target horizontal DPI.
    /// - `vdpi`: the target vertical DPI.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontSize`]
    /// - [`TTF_GetFontSizeDPI`]
    pub fn TTF_SetFontSizeDPI(
        font: *mut TTF_Font,
        ptsize: ::core::ffi::c_float,
        hdpi: ::core::ffi::c_int,
        vdpi: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the size of a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the size of the font, or 0.0f on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontSize`]
    /// - [`TTF_SetFontSizeDPI`]
    pub fn TTF_GetFontSize(font: *mut TTF_Font) -> ::core::ffi::c_float;
}

extern "C" {
    /// Get font target resolutions, in dots per inch.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `hdpi`: a pointer filled in with the target horizontal DPI.
    /// - `vdpi`: a pointer filled in with the target vertical DPI.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontSizeDPI`]
    pub fn TTF_GetFontDPI(
        font: *mut TTF_Font,
        hdpi: *mut ::core::ffi::c_int,
        vdpi: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

/// Font style flags for [`TTF_Font`]
///
/// These are the flags which can be used to set the style of a font in
/// SDL_ttf. A combination of these flags can be used with functions that set
/// or query font style, such as [`TTF_SetFontStyle`] or [`TTF_GetFontStyle`].
///
/// ### Availability
/// This function is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_SetFontStyle`]
/// - [`TTF_GetFontStyle`]
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`TTF_STYLE_NORMAL`] | No special style |
/// | [`TTF_STYLE_BOLD`] | Bold style |
/// | [`TTF_STYLE_ITALIC`] | Italic style |
/// | [`TTF_STYLE_UNDERLINE`] | Underlined text |
/// | [`TTF_STYLE_STRIKETHROUGH`] | Strikethrough text |
pub type TTF_FontStyleFlags = Uint32;

/// No special style
pub const TTF_STYLE_NORMAL: TTF_FontStyleFlags = (0x00 as TTF_FontStyleFlags);

/// Bold style
pub const TTF_STYLE_BOLD: TTF_FontStyleFlags = (0x01 as TTF_FontStyleFlags);

/// Italic style
pub const TTF_STYLE_ITALIC: TTF_FontStyleFlags = (0x02 as TTF_FontStyleFlags);

/// Underlined text
pub const TTF_STYLE_UNDERLINE: TTF_FontStyleFlags = (0x04 as TTF_FontStyleFlags);

/// Strikethrough text
pub const TTF_STYLE_STRIKETHROUGH: TTF_FontStyleFlags = (0x08 as TTF_FontStyleFlags);

extern "C" {
    /// Set a font's current style.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// The font styles are a set of bit flags, OR'd together:
    ///
    /// - [`TTF_STYLE_NORMAL`] (is zero)
    /// - [`TTF_STYLE_BOLD`]
    /// - [`TTF_STYLE_ITALIC`]
    /// - [`TTF_STYLE_UNDERLINE`]
    /// - [`TTF_STYLE_STRIKETHROUGH`]
    ///
    /// ### Parameters
    /// - `font`: the font to set a new style on.
    /// - `style`: the new style values to set, OR'd together.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontStyle`]
    pub fn TTF_SetFontStyle(font: *mut TTF_Font, style: TTF_FontStyleFlags);
}

extern "C" {
    /// Query a font's current style.
    ///
    /// The font styles are a set of bit flags, OR'd together:
    ///
    /// - [`TTF_STYLE_NORMAL`] (is zero)
    /// - [`TTF_STYLE_BOLD`]
    /// - [`TTF_STYLE_ITALIC`]
    /// - [`TTF_STYLE_UNDERLINE`]
    /// - [`TTF_STYLE_STRIKETHROUGH`]
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the current font style, as a set of bit flags.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontStyle`]
    pub fn TTF_GetFontStyle(font: *const TTF_Font) -> TTF_FontStyleFlags;
}

extern "C" {
    /// Set a font's current outline.
    ///
    /// This uses the font properties [`TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER`],
    /// [`TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER`], and
    /// [`TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER`] when setting the font outline.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// ### Parameters
    /// - `font`: the font to set a new outline on.
    /// - `outline`: positive outline value, 0 to default.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontOutline`]
    pub fn TTF_SetFontOutline(
        font: *mut TTF_Font,
        outline: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Query a font's current outline.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's current outline value.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontOutline`]
    pub fn TTF_GetFontOutline(font: *const TTF_Font) -> ::core::ffi::c_int;
}

/// Hinting flags for TTF (TrueType Fonts)
///
/// This enum specifies the level of hinting to be applied to the font
/// rendering. The hinting level determines how much the font's outlines are
/// adjusted for better alignment on the pixel grid.
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_SetFontHinting`]
/// - [`TTF_GetFontHinting`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NORMAL`](TTF_HintingFlags::NORMAL) | [`TTF_HINTING_NORMAL`] | Normal hinting applies standard grid-fitting. |
/// | [`LIGHT`](TTF_HintingFlags::LIGHT) | [`TTF_HINTING_LIGHT`] | Light hinting applies subtle adjustments to improve rendering. |
/// | [`MONO`](TTF_HintingFlags::MONO) | [`TTF_HINTING_MONO`] | Monochrome hinting adjusts the font for better rendering at lower resolutions. |
/// | [`NONE`](TTF_HintingFlags::NONE) | [`TTF_HINTING_NONE`] | No hinting, the font is rendered without any grid-fitting. |
/// | [`LIGHT_SUBPIXEL`](TTF_HintingFlags::LIGHT_SUBPIXEL) | [`TTF_HINTING_LIGHT_SUBPIXEL`] | Light hinting with subpixel rendering for more precise font edges. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_HintingFlags(pub ::core::ffi::c_int);

impl From<TTF_HintingFlags> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: TTF_HintingFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_HintingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NORMAL => "TTF_HINTING_NORMAL",
            Self::LIGHT => "TTF_HINTING_LIGHT",
            Self::MONO => "TTF_HINTING_MONO",
            Self::NONE => "TTF_HINTING_NONE",
            Self::LIGHT_SUBPIXEL => "TTF_HINTING_LIGHT_SUBPIXEL",

            _ => return write!(f, "TTF_HintingFlags({})", self.0),
        })
    }
}

impl TTF_HintingFlags {
    /// Normal hinting applies standard grid-fitting.
    pub const NORMAL: Self = Self(0);
    /// Light hinting applies subtle adjustments to improve rendering.
    pub const LIGHT: Self = Self(1);
    /// Monochrome hinting adjusts the font for better rendering at lower resolutions.
    pub const MONO: Self = Self(2);
    /// No hinting, the font is rendered without any grid-fitting.
    pub const NONE: Self = Self(3);
    /// Light hinting with subpixel rendering for more precise font edges.
    pub const LIGHT_SUBPIXEL: Self = Self(4);
}

/// Normal hinting applies standard grid-fitting.
pub const TTF_HINTING_NORMAL: TTF_HintingFlags = TTF_HintingFlags::NORMAL;
/// Light hinting applies subtle adjustments to improve rendering.
pub const TTF_HINTING_LIGHT: TTF_HintingFlags = TTF_HintingFlags::LIGHT;
/// Monochrome hinting adjusts the font for better rendering at lower resolutions.
pub const TTF_HINTING_MONO: TTF_HintingFlags = TTF_HintingFlags::MONO;
/// No hinting, the font is rendered without any grid-fitting.
pub const TTF_HINTING_NONE: TTF_HintingFlags = TTF_HintingFlags::NONE;
/// Light hinting with subpixel rendering for more precise font edges.
pub const TTF_HINTING_LIGHT_SUBPIXEL: TTF_HintingFlags = TTF_HintingFlags::LIGHT_SUBPIXEL;

extern "C" {
    /// Set a font's current hinter setting.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// The hinter setting is a single value:
    ///
    /// - [`TTF_HINTING_NORMAL`]
    /// - [`TTF_HINTING_LIGHT`]
    /// - [`TTF_HINTING_MONO`]
    /// - [`TTF_HINTING_NONE`]
    /// - [`TTF_HINTING_LIGHT_SUBPIXEL`] (available in SDL_ttf 3.0.0 and later)
    ///
    /// ### Parameters
    /// - `font`: the font to set a new hinter setting on.
    /// - `hinting`: the new hinter setting.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontHinting`]
    pub fn TTF_SetFontHinting(font: *mut TTF_Font, hinting: TTF_HintingFlags);
}

extern "C" {
    /// Query the number of faces of a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the number of FreeType font faces.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetNumFontFaces(font: *const TTF_Font) -> ::core::ffi::c_int;
}

extern "C" {
    /// Query a font's current FreeType hinter setting.
    ///
    /// The hinter setting is a single value:
    ///
    /// - [`TTF_HINTING_NORMAL`]
    /// - [`TTF_HINTING_LIGHT`]
    /// - [`TTF_HINTING_MONO`]
    /// - [`TTF_HINTING_NONE`]
    /// - [`TTF_HINTING_LIGHT_SUBPIXEL`] (available in SDL_ttf 3.0.0 and later)
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's current hinter value.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontHinting`]
    pub fn TTF_GetFontHinting(font: *const TTF_Font) -> TTF_HintingFlags;
}

extern "C" {
    /// Enable Signed Distance Field rendering for a font.
    ///
    /// SDF is a technique that helps fonts look sharp even when scaling and
    /// rotating, and requires special shader support for display.
    ///
    /// This works with Blended APIs, and generates the raw signed distance values
    /// in the alpha channel of the resulting texture.
    ///
    /// This updates any [`TTF_Text`] objects using this font, and clears
    /// already-generated glyphs, if any, from the cache.
    ///
    /// ### Parameters
    /// - `font`: the font to set SDF support on.
    /// - `enabled`: true to enable SDF, false to disable.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontSDF`]
    pub fn TTF_SetFontSDF(
        font: *mut TTF_Font,
        enabled: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether Signed Distance Field rendering is enabled for a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns true if enabled, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontSDF`]
    pub fn TTF_GetFontSDF(font: *const TTF_Font) -> ::core::primitive::bool;
}

/// The horizontal alignment used when rendering wrapped text.
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](TTF_HorizontalAlignment::INVALID) | [`TTF_HORIZONTAL_ALIGN_INVALID`] | |
/// | [`LEFT`](TTF_HorizontalAlignment::LEFT) | [`TTF_HORIZONTAL_ALIGN_LEFT`] | |
/// | [`CENTER`](TTF_HorizontalAlignment::CENTER) | [`TTF_HORIZONTAL_ALIGN_CENTER`] | |
/// | [`RIGHT`](TTF_HorizontalAlignment::RIGHT) | [`TTF_HORIZONTAL_ALIGN_RIGHT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_HorizontalAlignment(pub ::core::ffi::c_int);

impl From<TTF_HorizontalAlignment> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: TTF_HorizontalAlignment) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_HorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "TTF_HORIZONTAL_ALIGN_INVALID",
            Self::LEFT => "TTF_HORIZONTAL_ALIGN_LEFT",
            Self::CENTER => "TTF_HORIZONTAL_ALIGN_CENTER",
            Self::RIGHT => "TTF_HORIZONTAL_ALIGN_RIGHT",

            _ => return write!(f, "TTF_HorizontalAlignment({})", self.0),
        })
    }
}

impl TTF_HorizontalAlignment {
    pub const INVALID: Self = Self(-1_i32);
    pub const LEFT: Self = Self(0_i32);
    pub const CENTER: Self = Self(1_i32);
    pub const RIGHT: Self = Self(2_i32);
}

pub const TTF_HORIZONTAL_ALIGN_INVALID: TTF_HorizontalAlignment = TTF_HorizontalAlignment::INVALID;
pub const TTF_HORIZONTAL_ALIGN_LEFT: TTF_HorizontalAlignment = TTF_HorizontalAlignment::LEFT;
pub const TTF_HORIZONTAL_ALIGN_CENTER: TTF_HorizontalAlignment = TTF_HorizontalAlignment::CENTER;
pub const TTF_HORIZONTAL_ALIGN_RIGHT: TTF_HorizontalAlignment = TTF_HorizontalAlignment::RIGHT;

extern "C" {
    /// Set a font's current wrap alignment option.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to set a new wrap alignment option on.
    /// - `align`: the new wrap alignment option.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontWrapAlignment`]
    pub fn TTF_SetFontWrapAlignment(font: *mut TTF_Font, align: TTF_HorizontalAlignment);
}

extern "C" {
    /// Query a font's current wrap alignment option.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's current wrap alignment option.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontWrapAlignment`]
    pub fn TTF_GetFontWrapAlignment(font: *const TTF_Font) -> TTF_HorizontalAlignment;
}

extern "C" {
    /// Query the total height of a font.
    ///
    /// This is usually equal to point size.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's height.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontHeight(font: *const TTF_Font) -> ::core::ffi::c_int;
}

extern "C" {
    /// Query the offset from the baseline to the top of a font.
    ///
    /// This is a positive value, relative to the baseline.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's ascent.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontAscent(font: *const TTF_Font) -> ::core::ffi::c_int;
}

extern "C" {
    /// Query the offset from the baseline to the bottom of a font.
    ///
    /// This is a negative value, relative to the baseline.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's descent.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontDescent(font: *const TTF_Font) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set the spacing between lines of text for a font.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    /// - `lineskip`: the new line spacing for the font.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontLineSkip`]
    pub fn TTF_SetFontLineSkip(font: *mut TTF_Font, lineskip: ::core::ffi::c_int);
}

extern "C" {
    /// Query the spacing between lines of text for a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's recommended spacing.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontLineSkip`]
    pub fn TTF_GetFontLineSkip(font: *const TTF_Font) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set if kerning is enabled for a font.
    ///
    /// Newly-opened fonts default to allowing kerning. This is generally a good
    /// policy unless you have a strong reason to disable it, as it tends to
    /// produce better rendering (with kerning disabled, some fonts might render
    /// the word `kerning` as something that looks like `keming` for example).
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to set kerning on.
    /// - `enabled`: true to enable kerning, false to disable.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetFontKerning`]
    pub fn TTF_SetFontKerning(font: *mut TTF_Font, enabled: ::core::primitive::bool);
}

extern "C" {
    /// Query whether or not kerning is enabled for a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns true if kerning is enabled, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontKerning`]
    pub fn TTF_GetFontKerning(font: *const TTF_Font) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether a font is fixed-width.
    ///
    /// A "fixed-width" font means all glyphs are the same width across; a
    /// lowercase 'i' will be the same size across as a capital 'W', for example.
    /// This is common for terminals and text editors, and other apps that treat
    /// text as a grid. Most other things (WYSIWYG word processors, web pages, etc)
    /// are more likely to not be fixed-width in most cases.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns true if the font is fixed-width, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_FontIsFixedWidth(font: *const TTF_Font) -> ::core::primitive::bool;
}

extern "C" {
    /// Query whether a font is scalable or not.
    ///
    /// Scalability lets us distinguish between outline and bitmap fonts.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns true if the font is scalable, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetFontSDF`]
    pub fn TTF_FontIsScalable(font: *const TTF_Font) -> ::core::primitive::bool;
}

extern "C" {
    /// Query a font's family name.
    ///
    /// This string is dictated by the contents of the font file.
    ///
    /// Note that the returned string is to internal storage, and should not be
    /// modified or free'd by the caller. The string becomes invalid, with the rest
    /// of the font, when `font` is handed to [`TTF_CloseFont()`].
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's family name.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontFamilyName(font: *const TTF_Font) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query a font's style name.
    ///
    /// This string is dictated by the contents of the font file.
    ///
    /// Note that the returned string is to internal storage, and should not be
    /// modified or free'd by the caller. The string becomes invalid, with the rest
    /// of the font, when `font` is handed to [`TTF_CloseFont()`].
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the font's style name.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontStyleName(font: *const TTF_Font) -> *const ::core::ffi::c_char;
}

/// Direction flags
///
/// The values here are chosen to match
/// [hb_direction_t](https://harfbuzz.github.io/harfbuzz-hb-common.html#hb-direction-t)
/// .
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_SetFontDirection`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](TTF_Direction::INVALID) | [`TTF_DIRECTION_INVALID`] | |
/// | [`LTR`](TTF_Direction::LTR) | [`TTF_DIRECTION_LTR`] | Left to Right |
/// | [`RTL`](TTF_Direction::RTL) | [`TTF_DIRECTION_RTL`] | Right to Left |
/// | [`TTB`](TTF_Direction::TTB) | [`TTF_DIRECTION_TTB`] | Top to Bottom |
/// | [`BTT`](TTF_Direction::BTT) | [`TTF_DIRECTION_BTT`] | Bottom to Top |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_Direction(pub ::core::primitive::u32);

impl From<TTF_Direction> for ::core::primitive::u32 {
    #[inline(always)]
    fn from(value: TTF_Direction) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_Direction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "TTF_DIRECTION_INVALID",
            Self::LTR => "TTF_DIRECTION_LTR",
            Self::RTL => "TTF_DIRECTION_RTL",
            Self::TTB => "TTF_DIRECTION_TTB",
            Self::BTT => "TTF_DIRECTION_BTT",

            _ => return write!(f, "TTF_Direction({})", self.0),
        })
    }
}

impl TTF_Direction {
    pub const INVALID: Self = Self(0);
    /// Left to Right
    pub const LTR: Self = Self(4);
    /// Right to Left
    pub const RTL: Self = Self(5);
    /// Top to Bottom
    pub const TTB: Self = Self(6);
    /// Bottom to Top
    pub const BTT: Self = Self(7);
}

pub const TTF_DIRECTION_INVALID: TTF_Direction = TTF_Direction::INVALID;
/// Left to Right
pub const TTF_DIRECTION_LTR: TTF_Direction = TTF_Direction::LTR;
/// Right to Left
pub const TTF_DIRECTION_RTL: TTF_Direction = TTF_Direction::RTL;
/// Top to Bottom
pub const TTF_DIRECTION_TTB: TTF_Direction = TTF_Direction::TTB;
/// Bottom to Top
pub const TTF_DIRECTION_BTT: TTF_Direction = TTF_Direction::BTT;

extern "C" {
    /// Set the direction to be used for text shaping by a font.
    ///
    /// This function only supports left-to-right text shaping if SDL_ttf was not
    /// built with HarfBuzz support.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    /// - `direction`: the new direction for text to flow.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_SetFontDirection(
        font: *mut TTF_Font,
        direction: TTF_Direction,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the direction to be used for text shaping by a font.
    ///
    /// This defaults to [`TTF_DIRECTION_INVALID`] if it hasn't been set.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns the direction to be used for text shaping.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetFontDirection(font: *mut TTF_Font) -> TTF_Direction;
}

extern "C" {
    /// Convert from a 4 character string to a 32-bit tag.
    ///
    /// ### Parameters
    /// - `string`: the 4 character string to convert.
    ///
    /// ### Return value
    /// Returns the 32-bit representation of the string.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TagToString`]
    pub fn TTF_StringToTag(string: *const ::core::ffi::c_char) -> Uint32;
}

extern "C" {
    /// Convert from a 32-bit tag to a 4 character string.
    ///
    /// ### Parameters
    /// - `tag`: the 32-bit tag to convert.
    /// - `string`: a pointer filled in with the 4 character representation of
    ///   the tag.
    /// - `size`: the size of the buffer pointed at by string, should be at least
    ///   4.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TagToString`]
    pub fn TTF_TagToString(
        tag: Uint32,
        string: *mut ::core::ffi::c_char,
        size: ::core::primitive::usize,
    );
}

extern "C" {
    /// Set the script to be used for text shaping by a font.
    ///
    /// This returns false if SDL_ttf isn't built with HarfBuzz support.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to modify.
    /// - `script`: an
    ///   [ISO 15924 code](https://unicode.org/iso15924/iso15924-codes.html)
    ///   .
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_StringToTag`]
    pub fn TTF_SetFontScript(font: *mut TTF_Font, script: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the script used for text shaping a font.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    ///
    /// ### Return value
    /// Returns an
    ///   [ISO 15924 code](https://unicode.org/iso15924/iso15924-codes.html)
    ///   or 0 if a script hasn't been set.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TagToString`]
    pub fn TTF_GetFontScript(font: *mut TTF_Font) -> Uint32;
}

extern "C" {
    /// Get the script used by a 32-bit codepoint.
    ///
    /// ### Parameters
    /// - `ch`: the character code to check.
    ///
    /// ### Return value
    /// Returns an
    ///   [ISO 15924 code](https://unicode.org/iso15924/iso15924-codes.html)
    ///   on success, or 0 on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function is thread-safe.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TagToString`]
    pub fn TTF_GetGlyphScript(ch: Uint32) -> Uint32;
}

extern "C" {
    /// Set language to be used for text shaping by a font.
    ///
    /// If SDL_ttf was not built with HarfBuzz support, this function returns
    /// false.
    ///
    /// This updates any [`TTF_Text`] objects using this font.
    ///
    /// ### Parameters
    /// - `font`: the font to specify a language for.
    /// - `language_bcp47`: a null-terminated string containing the desired
    ///   language's BCP47 code. Or null to reset the value.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_SetFontLanguage(
        font: *mut TTF_Font,
        language_bcp47: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Check whether a glyph is provided by the font for a UNICODE codepoint.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `ch`: the codepoint to check.
    ///
    /// ### Return value
    /// Returns true if font provides a glyph for this character, false if not.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_FontHasGlyph(font: *mut TTF_Font, ch: Uint32) -> ::core::primitive::bool;
}

/// The type of data in a glyph image
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](TTF_ImageType::INVALID) | [`TTF_IMAGE_INVALID`] | |
/// | [`ALPHA`](TTF_ImageType::ALPHA) | [`TTF_IMAGE_ALPHA`] | The color channels are white |
/// | [`COLOR`](TTF_ImageType::COLOR) | [`TTF_IMAGE_COLOR`] | The color channels have image data |
/// | [`SDF`](TTF_ImageType::SDF) | [`TTF_IMAGE_SDF`] | The alpha channel has signed distance field information |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_ImageType(pub ::core::ffi::c_int);

impl From<TTF_ImageType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: TTF_ImageType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_ImageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "TTF_IMAGE_INVALID",
            Self::ALPHA => "TTF_IMAGE_ALPHA",
            Self::COLOR => "TTF_IMAGE_COLOR",
            Self::SDF => "TTF_IMAGE_SDF",

            _ => return write!(f, "TTF_ImageType({})", self.0),
        })
    }
}

impl TTF_ImageType {
    pub const INVALID: Self = Self(0);
    /// The color channels are white
    pub const ALPHA: Self = Self(1);
    /// The color channels have image data
    pub const COLOR: Self = Self(2);
    /// The alpha channel has signed distance field information
    pub const SDF: Self = Self(3);
}

pub const TTF_IMAGE_INVALID: TTF_ImageType = TTF_ImageType::INVALID;
/// The color channels are white
pub const TTF_IMAGE_ALPHA: TTF_ImageType = TTF_ImageType::ALPHA;
/// The color channels have image data
pub const TTF_IMAGE_COLOR: TTF_ImageType = TTF_ImageType::COLOR;
/// The alpha channel has signed distance field information
pub const TTF_IMAGE_SDF: TTF_ImageType = TTF_ImageType::SDF;

extern "C" {
    /// Get the pixel image for a UNICODE codepoint.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `ch`: the codepoint to check.
    /// - `image_type`: a pointer filled in with the glyph image type, may be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns an [`SDL_Surface`] containing the glyph, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetGlyphImage(
        font: *mut TTF_Font,
        ch: Uint32,
        image_type: *mut TTF_ImageType,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Get the pixel image for a character index.
    ///
    /// This is useful for text engine implementations, which can call this with
    /// the `glyph_index` in a [`TTF_CopyOperation`]
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `glyph_index`: the index of the glyph to return.
    /// - `image_type`: a pointer filled in with the glyph image type, may be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns an [`SDL_Surface`] containing the glyph, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetGlyphImageForIndex(
        font: *mut TTF_Font,
        glyph_index: Uint32,
        image_type: *mut TTF_ImageType,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Query the metrics (dimensions) of a font's glyph for a UNICODE codepoint.
    ///
    /// To understand what these metrics mean, here is a useful link:
    ///
    /// <https://freetype.sourceforge.net/freetype2/docs/tutorial/step2.html>
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `ch`: the codepoint to check.
    /// - `minx`: a pointer filled in with the minimum x coordinate of the glyph
    ///   from the left edge of its bounding box. This value may be
    ///   negative.
    /// - `maxx`: a pointer filled in with the maximum x coordinate of the glyph
    ///   from the left edge of its bounding box.
    /// - `miny`: a pointer filled in with the minimum y coordinate of the glyph
    ///   from the bottom edge of its bounding box. This value may be
    ///   negative.
    /// - `maxy`: a pointer filled in with the maximum y coordinate of the glyph
    ///   from the bottom edge of its bounding box.
    /// - `advance`: a pointer filled in with the distance to the next glyph from
    ///   the left edge of this glyph's bounding box.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetGlyphMetrics(
        font: *mut TTF_Font,
        ch: Uint32,
        minx: *mut ::core::ffi::c_int,
        maxx: *mut ::core::ffi::c_int,
        miny: *mut ::core::ffi::c_int,
        maxy: *mut ::core::ffi::c_int,
        advance: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Query the kerning size between the glyphs of two UNICODE codepoints.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `previous_ch`: the previous codepoint.
    /// - `ch`: the current codepoint.
    /// - `kerning`: a pointer filled in with the kerning size between the two
    ///   glyphs, in pixels, may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetGlyphKerning(
        font: *mut TTF_Font,
        previous_ch: Uint32,
        ch: Uint32,
        kerning: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the dimensions of a rendered string of UTF-8 text.
    ///
    /// This will report the width and height, in pixels, of the space that the
    /// specified string will take to fully render.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `text`: text to calculate, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `w`: will be filled with width, in pixels, on return.
    /// - `h`: will be filled with height, in pixels, on return.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetStringSize(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the dimensions of a rendered string of UTF-8 text.
    ///
    /// This will report the width and height, in pixels, of the space that the
    /// specified string will take to fully render.
    ///
    /// Text is wrapped to multiple lines on line endings and on word boundaries if
    /// it extends beyond `wrap_width` in pixels.
    ///
    /// If wrap_width is 0, this function will only wrap on newline characters.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `text`: text to calculate, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `wrap_width`: the maximum width or 0 to wrap on newline characters.
    /// - `w`: will be filled with width, in pixels, on return.
    /// - `h`: will be filled with height, in pixels, on return.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetStringSizeWrapped(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        wrap_width: ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate how much of a UTF-8 string will fit in a given width.
    ///
    /// This reports the number of characters that can be rendered before reaching
    /// `max_width`.
    ///
    /// This does not need to render the string to do this calculation.
    ///
    /// ### Parameters
    /// - `font`: the font to query.
    /// - `text`: text to calculate, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `max_width`: maximum width, in pixels, available for the string, or 0
    ///   for unbounded width.
    /// - `measured_width`: a pointer filled in with the width, in pixels, of the
    ///   string that will fit, may be NULL.
    /// - `measured_length`: a pointer filled in with the length, in bytes, of
    ///   the string that will fit, may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_MeasureString(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        max_width: ::core::ffi::c_int,
        measured_width: *mut ::core::ffi::c_int,
        measured_length: *mut ::core::primitive::usize,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Render UTF-8 text at fast quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the colorkey, giving a transparent background. The 1 pixel
    /// will be set to the text color.
    ///
    /// This will not word-wrap the string; you'll get a surface with a single line
    /// of text, as long as the string requires. You can use
    /// [`TTF_RenderText_Solid_Wrapped()`] instead if you need to wrap the output to
    /// multiple lines.
    ///
    /// This will not wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Shaded`],
    /// [`TTF_RenderText_Blended`], and [`TTF_RenderText_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended`]
    /// - [`TTF_RenderText_LCD`]
    /// - [`TTF_RenderText_Shaded`]
    /// - [`TTF_RenderText_Solid`]
    /// - [`TTF_RenderText_Solid_Wrapped`]
    pub fn TTF_RenderText_Solid(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render word-wrapped UTF-8 text at fast quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the colorkey, giving a transparent background. The 1 pixel
    /// will be set to the text color.
    ///
    /// Text is wrapped to multiple lines on line endings and on word boundaries if
    /// it extends beyond `wrapLength` in pixels.
    ///
    /// If wrapLength is 0, this function will only wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Shaded_Wrapped`],
    /// [`TTF_RenderText_Blended_Wrapped`], and [`TTF_RenderText_LCD_Wrapped`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `wrapLength`: the maximum width of the text surface or 0 to wrap on
    ///   newline characters.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended_Wrapped`]
    /// - [`TTF_RenderText_LCD_Wrapped`]
    /// - [`TTF_RenderText_Shaded_Wrapped`]
    /// - [`TTF_RenderText_Solid`]
    pub fn TTF_RenderText_Solid_Wrapped(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        wrapLength: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render a single 32-bit glyph at fast quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the colorkey, giving a transparent background. The 1 pixel
    /// will be set to the text color.
    ///
    /// The glyph is rendered without any padding or centering in the X direction,
    /// and aligned normally in the Y direction.
    ///
    /// You can render at other quality levels with [`TTF_RenderGlyph_Shaded`],
    /// [`TTF_RenderGlyph_Blended`], and [`TTF_RenderGlyph_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `ch`: the character to render.
    /// - `fg`: the foreground color for the text.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderGlyph_Blended`]
    /// - [`TTF_RenderGlyph_LCD`]
    /// - [`TTF_RenderGlyph_Shaded`]
    pub fn TTF_RenderGlyph_Solid(
        font: *mut TTF_Font,
        ch: Uint32,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render UTF-8 text at high quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the specified background color, while other pixels have
    /// varying degrees of the foreground color. This function returns the new
    /// surface, or NULL if there was an error.
    ///
    /// This will not word-wrap the string; you'll get a surface with a single line
    /// of text, as long as the string requires. You can use
    /// [`TTF_RenderText_Shaded_Wrapped()`] instead if you need to wrap the output to
    /// multiple lines.
    ///
    /// This will not wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid`],
    /// [`TTF_RenderText_Blended`], and [`TTF_RenderText_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended`]
    /// - [`TTF_RenderText_LCD`]
    /// - [`TTF_RenderText_Shaded_Wrapped`]
    /// - [`TTF_RenderText_Solid`]
    pub fn TTF_RenderText_Shaded(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render word-wrapped UTF-8 text at high quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the specified background color, while other pixels have
    /// varying degrees of the foreground color. This function returns the new
    /// surface, or NULL if there was an error.
    ///
    /// Text is wrapped to multiple lines on line endings and on word boundaries if
    /// it extends beyond `wrap_width` in pixels.
    ///
    /// If wrap_width is 0, this function will only wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid_Wrapped`],
    /// [`TTF_RenderText_Blended_Wrapped`], and [`TTF_RenderText_LCD_Wrapped`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    /// - `wrap_width`: the maximum width of the text surface or 0 to wrap on
    ///   newline characters.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended_Wrapped`]
    /// - [`TTF_RenderText_LCD_Wrapped`]
    /// - [`TTF_RenderText_Shaded`]
    /// - [`TTF_RenderText_Solid_Wrapped`]
    pub fn TTF_RenderText_Shaded_Wrapped(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        bg: SDL_Color,
        wrap_width: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render a single UNICODE codepoint at high quality to a new 8-bit surface.
    ///
    /// This function will allocate a new 8-bit, palettized surface. The surface's
    /// 0 pixel will be the specified background color, while other pixels have
    /// varying degrees of the foreground color. This function returns the new
    /// surface, or NULL if there was an error.
    ///
    /// The glyph is rendered without any padding or centering in the X direction,
    /// and aligned normally in the Y direction.
    ///
    /// You can render at other quality levels with [`TTF_RenderGlyph_Solid`],
    /// [`TTF_RenderGlyph_Blended`], and [`TTF_RenderGlyph_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `ch`: the codepoint to render.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    ///
    /// ### Return value
    /// Returns a new 8-bit, palettized surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderGlyph_Blended`]
    /// - [`TTF_RenderGlyph_LCD`]
    /// - [`TTF_RenderGlyph_Solid`]
    pub fn TTF_RenderGlyph_Shaded(
        font: *mut TTF_Font,
        ch: Uint32,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render UTF-8 text at high quality to a new ARGB surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, using alpha
    /// blending to dither the font with the given color. This function returns the
    /// new surface, or NULL if there was an error.
    ///
    /// This will not word-wrap the string; you'll get a surface with a single line
    /// of text, as long as the string requires. You can use
    /// [`TTF_RenderText_Blended_Wrapped()`] instead if you need to wrap the output to
    /// multiple lines.
    ///
    /// This will not wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid`],
    /// [`TTF_RenderText_Shaded`], and [`TTF_RenderText_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended_Wrapped`]
    /// - [`TTF_RenderText_LCD`]
    /// - [`TTF_RenderText_Shaded`]
    /// - [`TTF_RenderText_Solid`]
    pub fn TTF_RenderText_Blended(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render word-wrapped UTF-8 text at high quality to a new ARGB surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, using alpha
    /// blending to dither the font with the given color. This function returns the
    /// new surface, or NULL if there was an error.
    ///
    /// Text is wrapped to multiple lines on line endings and on word boundaries if
    /// it extends beyond `wrap_width` in pixels.
    ///
    /// If wrap_width is 0, this function will only wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid_Wrapped`],
    /// [`TTF_RenderText_Shaded_Wrapped`], and [`TTF_RenderText_LCD_Wrapped`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `wrap_width`: the maximum width of the text surface or 0 to wrap on
    ///   newline characters.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended`]
    /// - [`TTF_RenderText_LCD_Wrapped`]
    /// - [`TTF_RenderText_Shaded_Wrapped`]
    /// - [`TTF_RenderText_Solid_Wrapped`]
    pub fn TTF_RenderText_Blended_Wrapped(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        wrap_width: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render a single UNICODE codepoint at high quality to a new ARGB surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, using alpha
    /// blending to dither the font with the given color. This function returns the
    /// new surface, or NULL if there was an error.
    ///
    /// The glyph is rendered without any padding or centering in the X direction,
    /// and aligned normally in the Y direction.
    ///
    /// You can render at other quality levels with [`TTF_RenderGlyph_Solid`],
    /// [`TTF_RenderGlyph_Shaded`], and [`TTF_RenderGlyph_LCD`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `ch`: the codepoint to render.
    /// - `fg`: the foreground color for the text.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderGlyph_LCD`]
    /// - [`TTF_RenderGlyph_Shaded`]
    /// - [`TTF_RenderGlyph_Solid`]
    pub fn TTF_RenderGlyph_Blended(
        font: *mut TTF_Font,
        ch: Uint32,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render UTF-8 text at LCD subpixel quality to a new ARGB surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, and render
    /// alpha-blended text using FreeType's LCD subpixel rendering. This function
    /// returns the new surface, or NULL if there was an error.
    ///
    /// This will not word-wrap the string; you'll get a surface with a single line
    /// of text, as long as the string requires. You can use
    /// [`TTF_RenderText_LCD_Wrapped()`] instead if you need to wrap the output to
    /// multiple lines.
    ///
    /// This will not wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid`],
    /// [`TTF_RenderText_Shaded`], and [`TTF_RenderText_Blended`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended`]
    /// - [`TTF_RenderText_LCD_Wrapped`]
    /// - [`TTF_RenderText_Shaded`]
    /// - [`TTF_RenderText_Solid`]
    pub fn TTF_RenderText_LCD(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render word-wrapped UTF-8 text at LCD subpixel quality to a new ARGB
    /// surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, and render
    /// alpha-blended text using FreeType's LCD subpixel rendering. This function
    /// returns the new surface, or NULL if there was an error.
    ///
    /// Text is wrapped to multiple lines on line endings and on word boundaries if
    /// it extends beyond `wrap_width` in pixels.
    ///
    /// If wrap_width is 0, this function will only wrap on newline characters.
    ///
    /// You can render at other quality levels with [`TTF_RenderText_Solid_Wrapped`],
    /// [`TTF_RenderText_Shaded_Wrapped`], and [`TTF_RenderText_Blended_Wrapped`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `text`: text to render, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    /// - `wrap_width`: the maximum width of the text surface or 0 to wrap on
    ///   newline characters.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderText_Blended_Wrapped`]
    /// - [`TTF_RenderText_LCD`]
    /// - [`TTF_RenderText_Shaded_Wrapped`]
    /// - [`TTF_RenderText_Solid_Wrapped`]
    pub fn TTF_RenderText_LCD_Wrapped(
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
        fg: SDL_Color,
        bg: SDL_Color,
        wrap_width: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Render a single UNICODE codepoint at LCD subpixel quality to a new ARGB
    /// surface.
    ///
    /// This function will allocate a new 32-bit, ARGB surface, and render
    /// alpha-blended text using FreeType's LCD subpixel rendering. This function
    /// returns the new surface, or NULL if there was an error.
    ///
    /// The glyph is rendered without any padding or centering in the X direction,
    /// and aligned normally in the Y direction.
    ///
    /// You can render at other quality levels with [`TTF_RenderGlyph_Solid`],
    /// [`TTF_RenderGlyph_Shaded`], and [`TTF_RenderGlyph_Blended`].
    ///
    /// ### Parameters
    /// - `font`: the font to render with.
    /// - `ch`: the codepoint to render.
    /// - `fg`: the foreground color for the text.
    /// - `bg`: the background color for the text.
    ///
    /// ### Return value
    /// Returns a new 32-bit, ARGB surface, or NULL if there was an error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_RenderGlyph_Blended`]
    /// - [`TTF_RenderGlyph_Shaded`]
    /// - [`TTF_RenderGlyph_Solid`]
    pub fn TTF_RenderGlyph_LCD(
        font: *mut TTF_Font,
        ch: Uint32,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}

pub use super::textengine::TTF_TextEngine;

/// Text created with [`TTF_CreateText()`]
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_CreateText`]
/// - [`TTF_GetTextProperties`]
/// - [`TTF_DestroyText`]
#[repr(C)]
// #[non_exhaustive] // temporarily disabled bc of https://github.com/rust-lang/rust/issues/132699
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct TTF_Text {
    /// A copy of the UTF-8 string that this text object represents, useful for layout, debugging and retrieving substring text. This is updated when the text object is modified and will be freed automatically when the object is destroyed.
    pub text: *mut ::core::ffi::c_char,
    /// The number of lines in the text, 0 if it's empty
    pub num_lines: ::core::ffi::c_int,
    /// Application reference count, used when freeing surface
    pub refcount: ::core::ffi::c_int,
    /// Private
    pub internal: *mut TTF_TextData,
}

extern "C" {
    /// Create a text engine for drawing text on SDL surfaces.
    ///
    /// ### Return value
    /// Returns a [`TTF_TextEngine`] object or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_DestroySurfaceTextEngine`]
    /// - [`TTF_DrawSurfaceText`]
    pub fn TTF_CreateSurfaceTextEngine() -> *mut TTF_TextEngine;
}

extern "C" {
    /// Draw text to an SDL surface.
    ///
    /// `text` must have been created using a [`TTF_TextEngine`] from
    /// [`TTF_CreateSurfaceTextEngine()`].
    ///
    /// ### Parameters
    /// - `text`: the text to draw.
    /// - `x`: the x coordinate in pixels, positive from the left edge towards
    ///   the right.
    /// - `y`: the y coordinate in pixels, positive from the top edge towards the
    ///   bottom.
    /// - `surface`: the surface to draw on.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateSurfaceTextEngine`]
    /// - [`TTF_CreateText`]
    pub fn TTF_DrawSurfaceText(
        text: *mut TTF_Text,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        surface: *mut SDL_Surface,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a text engine created for drawing text on SDL surfaces.
    ///
    /// All text created by this engine should be destroyed before calling this
    /// function.
    ///
    /// ### Parameters
    /// - `engine`: a [`TTF_TextEngine`] object created with
    ///   [`TTF_CreateSurfaceTextEngine()`].
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateSurfaceTextEngine`]
    pub fn TTF_DestroySurfaceTextEngine(engine: *mut TTF_TextEngine);
}

extern "C" {
    /// Create a text engine for drawing text on an SDL renderer.
    ///
    /// ### Parameters
    /// - `renderer`: the renderer to use for creating textures and drawing text.
    ///
    /// ### Return value
    /// Returns a [`TTF_TextEngine`] object or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   renderer.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_DestroyRendererTextEngine`]
    /// - [`TTF_DrawRendererText`]
    /// - [`TTF_CreateRendererTextEngineWithProperties`]
    pub fn TTF_CreateRendererTextEngine(renderer: *mut SDL_Renderer) -> *mut TTF_TextEngine;
}

extern "C" {
    /// Create a text engine for drawing text on an SDL renderer, with the
    /// specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER`]\: the renderer to use for
    ///   creating textures and drawing text
    /// - [`TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE`]\: the size of the
    ///   texture atlas
    ///
    /// ### Parameters
    /// - `props`: the properties to use.
    ///
    /// ### Return value
    /// Returns a [`TTF_TextEngine`] object or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   renderer.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateRendererTextEngine`]
    /// - [`TTF_DestroyRendererTextEngine`]
    /// - [`TTF_DrawRendererText`]
    pub fn TTF_CreateRendererTextEngineWithProperties(
        props: SDL_PropertiesID,
    ) -> *mut TTF_TextEngine;
}

pub const TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER: *const ::core::ffi::c_char =
    c"SDL_ttf.renderer_text_engine.create.renderer".as_ptr();

pub const TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE: *const ::core::ffi::c_char =
    c"SDL_ttf.renderer_text_engine.create.atlas_texture_size".as_ptr();

extern "C" {
    /// Draw text to an SDL renderer.
    ///
    /// `text` must have been created using a [`TTF_TextEngine`] from
    /// [`TTF_CreateRendererTextEngine()`], and will draw using the renderer passed to
    /// that function.
    ///
    /// ### Parameters
    /// - `text`: the text to draw.
    /// - `x`: the x coordinate in pixels, positive from the left edge towards
    ///   the right.
    /// - `y`: the y coordinate in pixels, positive from the top edge towards the
    ///   bottom.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateRendererTextEngine`]
    /// - [`TTF_CreateText`]
    pub fn TTF_DrawRendererText(
        text: *mut TTF_Text,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a text engine created for drawing text on an SDL renderer.
    ///
    /// All text created by this engine should be destroyed before calling this
    /// function.
    ///
    /// ### Parameters
    /// - `engine`: a [`TTF_TextEngine`] object created with
    ///   [`TTF_CreateRendererTextEngine()`].
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateRendererTextEngine`]
    pub fn TTF_DestroyRendererTextEngine(engine: *mut TTF_TextEngine);
}

extern "C" {
    /// Create a text engine for drawing text with the SDL GPU API.
    ///
    /// ### Parameters
    /// - `device`: the [`SDL_GPUDevice`] to use for creating textures and drawing
    ///   text.
    ///
    /// ### Return value
    /// Returns a [`TTF_TextEngine`] object or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   device.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateGPUTextEngineWithProperties`]
    /// - [`TTF_DestroyGPUTextEngine`]
    /// - [`TTF_GetGPUTextDrawData`]
    pub fn TTF_CreateGPUTextEngine(device: *mut SDL_GPUDevice) -> *mut TTF_TextEngine;
}

extern "C" {
    /// Create a text engine for drawing text with the SDL GPU API, with the
    /// specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`TTF_PROP_GPU_TEXT_ENGINE_DEVICE`]\: the [`SDL_GPUDevice`] to use for creating
    ///   textures and drawing text.
    /// - [`TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE`]\: the size of the texture
    ///   atlas
    ///
    /// ### Parameters
    /// - `props`: the properties to use.
    ///
    /// ### Return value
    /// Returns a [`TTF_TextEngine`] object or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   device.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateGPUTextEngine`]
    /// - [`TTF_DestroyGPUTextEngine`]
    /// - [`TTF_GetGPUTextDrawData`]
    pub fn TTF_CreateGPUTextEngineWithProperties(props: SDL_PropertiesID) -> *mut TTF_TextEngine;
}

pub const TTF_PROP_GPU_TEXT_ENGINE_DEVICE: *const ::core::ffi::c_char =
    c"SDL_ttf.gpu_text_engine.create.device".as_ptr();

pub const TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE: *const ::core::ffi::c_char =
    c"SDL_ttf.gpu_text_engine.create.atlas_texture_size".as_ptr();

/// Draw sequence returned by [`TTF_GetGPUTextDrawData`]
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_GetGPUTextDrawData`]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct TTF_GPUAtlasDrawSequence {
    /// Texture atlas that stores the glyphs
    pub atlas_texture: *mut SDL_GPUTexture,
    /// An array of vertex positions
    pub xy: *mut SDL_FPoint,
    /// An array of normalized texture coordinates for each vertex
    pub uv: *mut SDL_FPoint,
    /// Number of vertices
    pub num_vertices: ::core::ffi::c_int,
    /// An array of indices into the 'vertices' arrays
    pub indices: *mut ::core::ffi::c_int,
    /// Number of indices
    pub num_indices: ::core::ffi::c_int,
    /// The image type of this draw sequence
    pub image_type: TTF_ImageType,
    /// The next sequence (will be NULL in case of the last sequence)
    pub next: *mut TTF_GPUAtlasDrawSequence,
}

impl ::core::default::Default for TTF_GPUAtlasDrawSequence {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

extern "C" {
    /// Get the geometry data needed for drawing the text.
    ///
    /// `text` must have been created using a [`TTF_TextEngine`] from
    /// [`TTF_CreateGPUTextEngine()`].
    ///
    /// The positive X-axis is taken towards the right and the positive Y-axis is
    /// taken upwards for both the vertex and the texture coordinates, i.e, it
    /// follows the same convention used by the SDL_GPU API. If you want to use a
    /// different coordinate system you will need to transform the vertices
    /// yourself.
    ///
    /// If the text looks blocky use linear filtering.
    ///
    /// ### Parameters
    /// - `text`: the text to draw.
    ///
    /// ### Return value
    /// Returns a NULL terminated linked list of [`TTF_GPUAtlasDrawSequence`] objects
    ///   or NULL if the passed text is empty or in case of failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateGPUTextEngine`]
    /// - [`TTF_CreateText`]
    pub fn TTF_GetGPUTextDrawData(text: *mut TTF_Text) -> *mut TTF_GPUAtlasDrawSequence;
}

extern "C" {
    /// Destroy a text engine created for drawing text with the SDL GPU API.
    ///
    /// All text created by this engine should be destroyed before calling this
    /// function.
    ///
    /// ### Parameters
    /// - `engine`: a [`TTF_TextEngine`] object created with
    ///   [`TTF_CreateGPUTextEngine()`].
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateGPUTextEngine`]
    pub fn TTF_DestroyGPUTextEngine(engine: *mut TTF_TextEngine);
}

/// The winding order of the vertices returned by [`TTF_GetGPUTextDrawData`]
///
/// ### Availability
/// This enum is available since SDL_ttf 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](TTF_GPUTextEngineWinding::INVALID) | [`TTF_GPU_TEXTENGINE_WINDING_INVALID`] | |
/// | [`CLOCKWISE`](TTF_GPUTextEngineWinding::CLOCKWISE) | [`TTF_GPU_TEXTENGINE_WINDING_CLOCKWISE`] | |
/// | [`COUNTER_CLOCKWISE`](TTF_GPUTextEngineWinding::COUNTER_CLOCKWISE) | [`TTF_GPU_TEXTENGINE_WINDING_COUNTER_CLOCKWISE`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TTF_GPUTextEngineWinding(pub ::core::ffi::c_int);

impl From<TTF_GPUTextEngineWinding> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: TTF_GPUTextEngineWinding) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for TTF_GPUTextEngineWinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "TTF_GPU_TEXTENGINE_WINDING_INVALID",
            Self::CLOCKWISE => "TTF_GPU_TEXTENGINE_WINDING_CLOCKWISE",
            Self::COUNTER_CLOCKWISE => "TTF_GPU_TEXTENGINE_WINDING_COUNTER_CLOCKWISE",

            _ => return write!(f, "TTF_GPUTextEngineWinding({})", self.0),
        })
    }
}

impl TTF_GPUTextEngineWinding {
    pub const INVALID: Self = Self(-1_i32);
    pub const CLOCKWISE: Self = Self(0_i32);
    pub const COUNTER_CLOCKWISE: Self = Self(1_i32);
}

pub const TTF_GPU_TEXTENGINE_WINDING_INVALID: TTF_GPUTextEngineWinding =
    TTF_GPUTextEngineWinding::INVALID;
pub const TTF_GPU_TEXTENGINE_WINDING_CLOCKWISE: TTF_GPUTextEngineWinding =
    TTF_GPUTextEngineWinding::CLOCKWISE;
pub const TTF_GPU_TEXTENGINE_WINDING_COUNTER_CLOCKWISE: TTF_GPUTextEngineWinding =
    TTF_GPUTextEngineWinding::COUNTER_CLOCKWISE;

extern "C" {
    /// Sets the winding order of the vertices returned by [`TTF_GetGPUTextDrawData`]
    /// for a particular GPU text engine.
    ///
    /// ### Parameters
    /// - `engine`: a [`TTF_TextEngine`] object created with
    ///   [`TTF_CreateGPUTextEngine()`].
    /// - `winding`: the new winding order option.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetGPUTextEngineWinding`]
    pub fn TTF_SetGPUTextEngineWinding(
        engine: *mut TTF_TextEngine,
        winding: TTF_GPUTextEngineWinding,
    );
}

extern "C" {
    /// Get the winding order of the vertices returned by [`TTF_GetGPUTextDrawData`]
    /// for a particular GPU text engine
    ///
    /// ### Parameters
    /// - `engine`: a [`TTF_TextEngine`] object created with
    ///   [`TTF_CreateGPUTextEngine()`].
    ///
    /// ### Return value
    /// Returns the winding order used by the GPU text engine or
    ///   [`TTF_GPU_TEXTENGINE_WINDING_INVALID`] in case of error.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetGPUTextEngineWinding`]
    pub fn TTF_GetGPUTextEngineWinding(engine: *const TTF_TextEngine) -> TTF_GPUTextEngineWinding;
}

extern "C" {
    /// Create a text object from UTF-8 text and a text engine.
    ///
    /// ### Parameters
    /// - `engine`: the text engine to use when creating the text object, may be
    ///   NULL.
    /// - `font`: the font to render with.
    /// - `text`: the text to use, in UTF-8 encoding.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    ///
    /// ### Return value
    /// Returns a [`TTF_Text`] object or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   font and text engine.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_DestroyText`]
    pub fn TTF_CreateText(
        engine: *mut TTF_TextEngine,
        font: *mut TTF_Font,
        text: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
    ) -> *mut TTF_Text;
}

extern "C" {
    /// Get the properties associated with a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextProperties(text: *mut TTF_Text) -> SDL_PropertiesID;
}

extern "C" {
    /// Set the text engine used by a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `engine`: the text engine to use for drawing.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextEngine`]
    pub fn TTF_SetTextEngine(
        text: *mut TTF_Text,
        engine: *mut TTF_TextEngine,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the text engine used by a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    ///
    /// ### Return value
    /// Returns the [`TTF_TextEngine`] used by the text on success or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetTextEngine`]
    pub fn TTF_GetTextEngine(text: *mut TTF_Text) -> *mut TTF_TextEngine;
}

extern "C" {
    /// Set the font used by a text object.
    ///
    /// When a text object has a font, any changes to the font will automatically
    /// regenerate the text. If you set the font to NULL, the text will continue to
    /// render but changes to the font will no longer affect the text.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `font`: the font to use, may be NULL.
    ///
    /// ### Return value
    /// Returns false if the text pointer is null; otherwise, true. call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextFont`]
    pub fn TTF_SetTextFont(text: *mut TTF_Text, font: *mut TTF_Font) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the font used by a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    ///
    /// ### Return value
    /// Returns the [`TTF_Font`] used by the text on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetTextFont`]
    pub fn TTF_GetTextFont(text: *mut TTF_Text) -> *mut TTF_Font;
}

extern "C" {
    /// Set the direction to be used for text shaping a text object.
    ///
    /// This function only supports left-to-right text shaping if SDL_ttf was not
    /// built with HarfBuzz support.
    ///
    /// ### Parameters
    /// - `text`: the text to modify.
    /// - `direction`: the new direction for text to flow.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_SetTextDirection(
        text: *mut TTF_Text,
        direction: TTF_Direction,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the direction to be used for text shaping a text object.
    ///
    /// This defaults to the direction of the font used by the text object.
    ///
    /// ### Parameters
    /// - `text`: the text to query.
    ///
    /// ### Return value
    /// Returns the direction to be used for text shaping.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextDirection(text: *mut TTF_Text) -> TTF_Direction;
}

extern "C" {
    /// Set the script to be used for text shaping a text object.
    ///
    /// This returns false if SDL_ttf isn't built with HarfBuzz support.
    ///
    /// ### Parameters
    /// - `text`: the text to modify.
    /// - `script`: an
    ///   [ISO 15924 code](https://unicode.org/iso15924/iso15924-codes.html)
    ///   .
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_StringToTag`]
    pub fn TTF_SetTextScript(text: *mut TTF_Text, script: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the script used for text shaping a text object.
    ///
    /// This defaults to the script of the font used by the text object.
    ///
    /// ### Parameters
    /// - `text`: the text to query.
    ///
    /// ### Return value
    /// Returns an
    ///   [ISO 15924 code](https://unicode.org/iso15924/iso15924-codes.html)
    ///   or 0 if a script hasn't been set on either the text object or the
    ///   font.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TagToString`]
    pub fn TTF_GetTextScript(text: *mut TTF_Text) -> Uint32;
}

extern "C" {
    /// Set the color of a text object.
    ///
    /// The default text color is white (255, 255, 255, 255).
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `r`: the red color value in the range of 0-255.
    /// - `g`: the green color value in the range of 0-255.
    /// - `b`: the blue color value in the range of 0-255.
    /// - `a`: the alpha value in the range of 0-255.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextColor`]
    /// - [`TTF_SetTextColorFloat`]
    pub fn TTF_SetTextColor(
        text: *mut TTF_Text,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the color of a text object.
    ///
    /// The default text color is white (1.0f, 1.0f, 1.0f, 1.0f).
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `r`: the red color value, normally in the range of 0-1.
    /// - `g`: the green color value, normally in the range of 0-1.
    /// - `b`: the blue color value, normally in the range of 0-1.
    /// - `a`: the alpha value in the range of 0-1.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextColorFloat`]
    /// - [`TTF_SetTextColor`]
    pub fn TTF_SetTextColorFloat(
        text: *mut TTF_Text,
        r: ::core::ffi::c_float,
        g: ::core::ffi::c_float,
        b: ::core::ffi::c_float,
        a: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the color of a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `r`: a pointer filled in with the red color value in the range of
    ///   0-255, may be NULL.
    /// - `g`: a pointer filled in with the green color value in the range of
    ///   0-255, may be NULL.
    /// - `b`: a pointer filled in with the blue color value in the range of
    ///   0-255, may be NULL.
    /// - `a`: a pointer filled in with the alpha value in the range of 0-255,
    ///   may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextColorFloat`]
    /// - [`TTF_SetTextColor`]
    pub fn TTF_GetTextColor(
        text: *mut TTF_Text,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
        a: *mut Uint8,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the color of a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `r`: a pointer filled in with the red color value, normally in the
    ///   range of 0-1, may be NULL.
    /// - `g`: a pointer filled in with the green color value, normally in the
    ///   range of 0-1, may be NULL.
    /// - `b`: a pointer filled in with the blue color value, normally in the
    ///   range of 0-1, may be NULL.
    /// - `a`: a pointer filled in with the alpha value in the range of 0-1, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextColor`]
    /// - [`TTF_SetTextColorFloat`]
    pub fn TTF_GetTextColorFloat(
        text: *mut TTF_Text,
        r: *mut ::core::ffi::c_float,
        g: *mut ::core::ffi::c_float,
        b: *mut ::core::ffi::c_float,
        a: *mut ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the position of a text object.
    ///
    /// This can be used to position multiple text objects within a single wrapping
    /// text area.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `x`: the x offset of the upper left corner of this text in pixels.
    /// - `y`: the y offset of the upper left corner of this text in pixels.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextPosition`]
    pub fn TTF_SetTextPosition(
        text: *mut TTF_Text,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the position of a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `x`: a pointer filled in with the x offset of the upper left corner of
    ///   this text in pixels, may be NULL.
    /// - `y`: a pointer filled in with the y offset of the upper left corner of
    ///   this text in pixels, may be NULL.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetTextPosition`]
    pub fn TTF_GetTextPosition(
        text: *mut TTF_Text,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set whether wrapping is enabled on a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `wrap_width`: the maximum width in pixels, 0 to wrap on newline
    ///   characters.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_GetTextWrapWidth`]
    pub fn TTF_SetTextWrapWidth(
        text: *mut TTF_Text,
        wrap_width: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get whether wrapping is enabled on a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `wrap_width`: a pointer filled in with the maximum width in pixels or 0
    ///   if the text is being wrapped on newline characters.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetTextWrapWidth`]
    pub fn TTF_GetTextWrapWidth(
        text: *mut TTF_Text,
        wrap_width: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set whether whitespace should be visible when wrapping a text object.
    ///
    /// If the whitespace is visible, it will take up space for purposes of
    /// alignment and wrapping. This is good for editing, but looks better when
    /// centered or aligned if whitespace around line wrapping is hidden. This
    /// defaults false.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `visible`: true to show whitespace when wrapping text, false to hide
    ///   it.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_TextWrapWhitespaceVisible`]
    pub fn TTF_SetTextWrapWhitespaceVisible(
        text: *mut TTF_Text,
        visible: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether whitespace is shown when wrapping a text object.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    ///
    /// ### Return value
    /// Returns true if whitespace is shown when wrapping text, or false
    ///   otherwise.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_SetTextWrapWhitespaceVisible`]
    pub fn TTF_TextWrapWhitespaceVisible(text: *mut TTF_Text) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the UTF-8 text used by a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `string`: the UTF-8 text to use, may be NULL.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_AppendTextString`]
    /// - [`TTF_DeleteTextString`]
    /// - [`TTF_InsertTextString`]
    pub fn TTF_SetTextString(
        text: *mut TTF_Text,
        string: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Insert UTF-8 text into a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `offset`: the offset, in bytes, from the beginning of the string if >=
    ///   0, the offset from the end of the string if < 0. Note that
    ///   this does not do UTF-8 validation, so you should only insert
    ///   at UTF-8 sequence boundaries.
    /// - `string`: the UTF-8 text to insert.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_AppendTextString`]
    /// - [`TTF_DeleteTextString`]
    /// - [`TTF_SetTextString`]
    pub fn TTF_InsertTextString(
        text: *mut TTF_Text,
        offset: ::core::ffi::c_int,
        string: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Append UTF-8 text to a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `string`: the UTF-8 text to insert.
    /// - `length`: the length of the text, in bytes, or 0 for null terminated
    ///   text.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_DeleteTextString`]
    /// - [`TTF_InsertTextString`]
    /// - [`TTF_SetTextString`]
    pub fn TTF_AppendTextString(
        text: *mut TTF_Text,
        string: *const ::core::ffi::c_char,
        length: ::core::primitive::usize,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Delete UTF-8 text from a text object.
    ///
    /// This function may cause the internal text representation to be rebuilt.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to modify.
    /// - `offset`: the offset, in bytes, from the beginning of the string if >=
    ///   0, the offset from the end of the string if < 0. Note that
    ///   this does not do UTF-8 validation, so you should only delete
    ///   at UTF-8 sequence boundaries.
    /// - `length`: the length of text to delete, in bytes, or -1 for the
    ///   remainder of the string.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_AppendTextString`]
    /// - [`TTF_InsertTextString`]
    /// - [`TTF_SetTextString`]
    pub fn TTF_DeleteTextString(
        text: *mut TTF_Text,
        offset: ::core::ffi::c_int,
        length: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the size of a text object.
    ///
    /// The size of the text may change when the font or font style and size
    /// change.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `w`: a pointer filled in with the width of the text, in pixels, may be
    ///   NULL.
    /// - `h`: a pointer filled in with the height of the text, in pixels, may be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextSize(
        text: *mut TTF_Text,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

/// Flags for [`TTF_SubString`]
///
/// ### Availability
/// This datatype is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_SubString`]
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`TTF_SUBSTRING_DIRECTION_MASK`] | The mask for the flow direction for this substring |
/// | [`TTF_SUBSTRING_TEXT_START`] | This substring contains the beginning of the text |
/// | [`TTF_SUBSTRING_LINE_START`] | This substring contains the beginning of line `line_index` |
/// | [`TTF_SUBSTRING_LINE_END`] | This substring contains the end of line `line_index` |
/// | [`TTF_SUBSTRING_TEXT_END`] | This substring contains the end of the text |
pub type TTF_SubStringFlags = Uint32;

/// The mask for the flow direction for this substring
pub const TTF_SUBSTRING_DIRECTION_MASK: TTF_SubStringFlags = (0x000000ff as TTF_SubStringFlags);

/// This substring contains the beginning of the text
pub const TTF_SUBSTRING_TEXT_START: TTF_SubStringFlags = (0x00000100 as TTF_SubStringFlags);

/// This substring contains the beginning of line `line_index`
pub const TTF_SUBSTRING_LINE_START: TTF_SubStringFlags = (0x00000200 as TTF_SubStringFlags);

/// This substring contains the end of line `line_index`
pub const TTF_SUBSTRING_LINE_END: TTF_SubStringFlags = (0x00000400 as TTF_SubStringFlags);

/// This substring contains the end of the text
pub const TTF_SUBSTRING_TEXT_END: TTF_SubStringFlags = (0x00000800 as TTF_SubStringFlags);

/// The representation of a substring within text.
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
///
/// ### See also
/// - [`TTF_GetNextTextSubString`]
/// - [`TTF_GetPreviousTextSubString`]
/// - [`TTF_GetTextSubString`]
/// - [`TTF_GetTextSubStringForLine`]
/// - [`TTF_GetTextSubStringForPoint`]
/// - [`TTF_GetTextSubStringsForRange`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[derive(Default)]
pub struct TTF_SubString {
    /// The flags for this substring
    pub flags: TTF_SubStringFlags,
    /// The byte offset from the beginning of the text
    pub offset: ::core::ffi::c_int,
    /// The byte length starting at the offset
    pub length: ::core::ffi::c_int,
    /// The index of the line that contains this substring
    pub line_index: ::core::ffi::c_int,
    /// The internal cluster index, used for quickly iterating
    pub cluster_index: ::core::ffi::c_int,
    /// The rectangle, relative to the top left of the text, containing the substring
    pub rect: SDL_Rect,
}

extern "C" {
    /// Get the substring of a text object that surrounds a text offset.
    ///
    /// If `offset` is less than 0, this will return a zero length substring at the
    /// beginning of the text with the [`TTF_SUBSTRING_TEXT_START`] flag set. If
    /// `offset` is greater than or equal to the length of the text string, this
    /// will return a zero length substring at the end of the text with the
    /// [`TTF_SUBSTRING_TEXT_END`] flag set.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `offset`: a byte offset into the text string.
    /// - `substring`: a pointer filled in with the substring containing the
    ///   offset.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextSubString(
        text: *mut TTF_Text,
        offset: ::core::ffi::c_int,
        substring: *mut TTF_SubString,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the substring of a text object that contains the given line.
    ///
    /// If `line` is less than 0, this will return a zero length substring at the
    /// beginning of the text with the [`TTF_SUBSTRING_TEXT_START`] flag set. If `line`
    /// is greater than or equal to `text->num_lines` this will return a zero
    /// length substring at the end of the text with the [`TTF_SUBSTRING_TEXT_END`]
    /// flag set.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `line`: a zero-based line index, in the range \[0 .. text->num_lines-1\].
    /// - `substring`: a pointer filled in with the substring containing the
    ///   offset.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextSubStringForLine(
        text: *mut TTF_Text,
        line: ::core::ffi::c_int,
        substring: *mut TTF_SubString,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the substrings of a text object that contain a range of text.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `offset`: a byte offset into the text string.
    /// - `length`: the length of the range being queried, in bytes, or -1 for
    ///   the remainder of the string.
    /// - `count`: a pointer filled in with the number of substrings returned,
    ///   may be NULL.
    ///
    /// ### Return value
    /// Returns a NULL terminated array of substring pointers or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This is a single
    ///   allocation that should be freed with [`SDL_free()`] when it is no
    ///   longer needed.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextSubStringsForRange(
        text: *mut TTF_Text,
        offset: ::core::ffi::c_int,
        length: ::core::ffi::c_int,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut TTF_SubString;
}

extern "C" {
    /// Get the portion of a text string that is closest to a point.
    ///
    /// This will return the closest substring of text to the given point.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `x`: the x coordinate relative to the left side of the text, may be
    ///   outside the bounds of the text area.
    /// - `y`: the y coordinate relative to the top side of the text, may be
    ///   outside the bounds of the text area.
    /// - `substring`: a pointer filled in with the closest substring of text to
    ///   the given point.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetTextSubStringForPoint(
        text: *mut TTF_Text,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        substring: *mut TTF_SubString,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the previous substring in a text object
    ///
    /// If called at the start of the text, this will return a zero length
    /// substring with the [`TTF_SUBSTRING_TEXT_START`] flag set.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `substring`: the [`TTF_SubString`] to query.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetPreviousTextSubString(
        text: *mut TTF_Text,
        substring: *const TTF_SubString,
        previous: *mut TTF_SubString,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the next substring in a text object
    ///
    /// If called at the end of the text, this will return a zero length substring
    /// with the [`TTF_SUBSTRING_TEXT_END`] flag set.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to query.
    /// - `substring`: the [`TTF_SubString`] to query.
    /// - `next`: a pointer filled in with the next substring.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_GetNextTextSubString(
        text: *mut TTF_Text,
        substring: *const TTF_SubString,
        next: *mut TTF_SubString,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Update the layout of a text object.
    ///
    /// This is automatically done when the layout is requested or the text is
    /// rendered, but you can call this if you need more control over the timing of
    /// when the layout and text engine representation are updated.
    ///
    /// ### Parameters
    /// - `text`: the [`TTF_Text`] to update.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_UpdateText(text: *mut TTF_Text) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a text object created by a text engine.
    ///
    /// ### Parameters
    /// - `text`: the text to destroy.
    ///
    /// ### Thread safety
    /// This function should be called on the thread that created the
    ///   text.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_CreateText`]
    pub fn TTF_DestroyText(text: *mut TTF_Text);
}

extern "C" {
    /// Dispose of a previously-created font.
    ///
    /// Call this when done with a font. This function will free any resources
    /// associated with it. It is safe to call this function on NULL, for example
    /// on the result of a failed call to [`TTF_OpenFont()`].
    ///
    /// The font is not valid after being passed to this function. String pointers
    /// from functions that return information on this font, such as
    /// [`TTF_GetFontFamilyName()`] and [`TTF_GetFontStyleName()`], are no longer valid
    /// after this call, as well.
    ///
    /// ### Parameters
    /// - `font`: the font to dispose of.
    ///
    /// ### Thread safety
    /// This function should not be called while any other thread is
    ///   using the font.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_OpenFont`]
    /// - [`TTF_OpenFontIO`]
    pub fn TTF_CloseFont(font: *mut TTF_Font);
}

extern "C" {
    /// Deinitialize SDL_ttf.
    ///
    /// You must call this when done with the library, to free internal resources.
    /// It is safe to call this when the library isn't initialized, as it will just
    /// return immediately.
    ///
    /// Once you have as many quit calls as you have had successful calls to
    /// [`TTF_Init`], the library will actually deinitialize.
    ///
    /// Please note that this does not automatically close any fonts that are still
    /// open at the time of deinitialization, and it is possibly not safe to close
    /// them afterwards, as parts of the library will no longer be initialized to
    /// deal with it. A well-written program should call [`TTF_CloseFont()`] on any
    /// open fonts before calling this function!
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    pub fn TTF_Quit();
}

extern "C" {
    /// Check if SDL_ttf is initialized.
    ///
    /// This reports the number of times the library has been initialized by a call
    /// to [`TTF_Init()`], without a paired deinitialization request from [`TTF_Quit()`].
    ///
    /// In short: if it's greater than zero, the library is currently initialized
    /// and ready to work. If zero, it is not initialized.
    ///
    /// Despite the return value being a signed integer, this function should not
    /// return a negative number.
    ///
    /// ### Return value
    /// Returns the current number of initialization calls, that need to
    ///   eventually be paired with this many calls to [`TTF_Quit()`].
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL_ttf 3.0.0.
    ///
    /// ### See also
    /// - [`TTF_Init`]
    /// - [`TTF_Quit`]
    pub fn TTF_WasInit() -> ::core::ffi::c_int;
}

/// The internal structure containing font information.
///
/// Opaque data!
#[repr(C)]
pub struct TTF_Font {
    _opaque: [::core::primitive::u8; 0],
}

/// Internal data for [`TTF_Text`]
///
/// ### Availability
/// This struct is available since SDL_ttf 3.0.0.
#[repr(C)]
pub struct TTF_TextData {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
