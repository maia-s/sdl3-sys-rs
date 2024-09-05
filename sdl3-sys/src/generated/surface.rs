#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategorySurface
//!
//! SDL_Surface definition and management functions.

use super::stdinc::*;

use super::error::*;

use super::blendmode::*;

use super::pixels::*;

use super::properties::*;

use super::rect::*;

use super::iostream::*;

/// The flags on an SDL_Surface.
///
/// These are generally considered read-only.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_SurfaceFlags = Uint32;

/// Surface uses preallocated pixel memory
pub const SDL_SURFACE_PREALLOCATED: ::core::primitive::u32 = 1_u32;

/// Surface needs to be locked to access pixels
pub const SDL_SURFACE_LOCK_NEEDED: ::core::primitive::u32 = 2_u32;

/// Surface is currently locked
pub const SDL_SURFACE_LOCKED: ::core::primitive::u32 = 4_u32;

/// Surface uses pixel memory allocated with SDL_aligned_alloc()
pub const SDL_SURFACE_SIMD_ALIGNED: ::core::primitive::u32 = 8_u32;

/// The scaling mode.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_SCALEMODE_NEAREST`], [`SDL_SCALEMODE_LINEAR`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_ScaleMode(pub ::core::ffi::c_int);
impl SDL_ScaleMode {
    /// nearest pixel sampling
    pub const NEAREST: Self = Self(0);
    /// linear filtering
    pub const LINEAR: Self = Self(1);
}
/// nearest pixel sampling
pub const SDL_SCALEMODE_NEAREST: SDL_ScaleMode = SDL_ScaleMode::NEAREST;
/// linear filtering
pub const SDL_SCALEMODE_LINEAR: SDL_ScaleMode = SDL_ScaleMode::LINEAR;

/// The flip mode.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_FLIP_NONE`], [`SDL_FLIP_HORIZONTAL`], [`SDL_FLIP_VERTICAL`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FlipMode(pub ::core::ffi::c_int);
impl SDL_FlipMode {
    /// Do not flip
    pub const NONE: Self = Self(0);
    /// flip horizontally
    pub const HORIZONTAL: Self = Self(1);
    /// flip vertically
    pub const VERTICAL: Self = Self(2);
}
/// Do not flip
pub const SDL_FLIP_NONE: SDL_FlipMode = SDL_FlipMode::NONE;
/// flip horizontally
pub const SDL_FLIP_HORIZONTAL: SDL_FlipMode = SDL_FlipMode::HORIZONTAL;
/// flip vertically
pub const SDL_FLIP_VERTICAL: SDL_FlipMode = SDL_FlipMode::VERTICAL;

/// A collection of pixels used in software blitting.
///
/// Pixels are arranged in memory in rows, with the top row first. Each row
/// occupies an amount of memory given by the pitch (sometimes known as the row
/// stride in non-SDL APIs).
///
/// Within each row, pixels are arranged from left to right until the width is
/// reached. Each pixel occupies a number of bits appropriate for its format,
/// with most formats representing each pixel as one or more whole bytes (in
/// some indexed formats, instead multiple pixels are packed into each byte),
/// and a byte order given by the format. After encoding all pixels, any
/// remaining bytes to reach the pitch are used as padding to reach a desired
/// alignment, and have undefined contents.
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Surface {
    /// Read-only
    pub flags: SDL_SurfaceFlags,
    /// Read-only
    pub format: SDL_PixelFormat,
    /// Read-only
    pub w: ::core::ffi::c_int,
    /// Read-only
    pub h: ::core::ffi::c_int,
    /// Read-only
    pub pitch: ::core::ffi::c_int,
    /// Read-only pointer, writable pixels if non-NULL
    pub pixels: *mut ::core::ffi::c_void,
    /// Application reference count, used when freeing surface
    pub refcount: ::core::ffi::c_int,
    /// Private
    pub internal: *mut SDL_SurfaceData,
}

extern_sdlcall! {{
    /// Allocate a new surface with a specific pixel format.
    ///
    /// The pixels of the new surface are initialized to zero.
    ///
    /// \param width the width of the surface.
    /// \param height the height of the surface.
    /// \param format the SDL_PixelFormat for the new surface's pixel format.
    /// \returns the new SDL_Surface structure that is created or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateSurfaceFrom
    /// \sa SDL_DestroySurface
    pub fn SDL_CreateSurface(width: ::core::ffi::c_int, height: ::core::ffi::c_int, format: SDL_PixelFormat) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Allocate a new surface with a specific pixel format and existing pixel
    /// data.
    ///
    /// No copy is made of the pixel data. Pixel data is not managed automatically;
    /// you must free the surface before you free the pixel data.
    ///
    /// Pitch is the offset in bytes from one row of pixels to the next, e.g.
    /// `width*4` for `SDL_PIXELFORMAT_RGBA8888`.
    ///
    /// You may pass NULL for pixels and 0 for pitch to create a surface that you
    /// will fill in with valid values later.
    ///
    /// \param width the width of the surface.
    /// \param height the height of the surface.
    /// \param format the SDL_PixelFormat for the new surface's pixel format.
    /// \param pixels a pointer to existing pixel data.
    /// \param pitch the number of bytes between each row, including padding.
    /// \returns the new SDL_Surface structure that is created or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateSurface
    /// \sa SDL_DestroySurface
    pub fn SDL_CreateSurfaceFrom(width: ::core::ffi::c_int, height: ::core::ffi::c_int, format: SDL_PixelFormat, pixels: *mut ::core::ffi::c_void, pitch: ::core::ffi::c_int) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Free a surface.
    ///
    /// It is safe to pass NULL to this function.
    ///
    /// \param surface the SDL_Surface to free.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateStackSurface
    /// \sa SDL_CreateSurface
    /// \sa SDL_CreateSurfaceFrom
    pub fn SDL_DestroySurface(surface: *mut SDL_Surface);
}}

extern_sdlcall! {{
    /// Get the properties associated with a surface.
    ///
    /// The following properties are understood by SDL:
    ///
    /// - `SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT`: for HDR10 and floating point
    ///   surfaces, this defines the value of 100% diffuse white, with higher
    ///   values being displayed in the High Dynamic Range headroom. This defaults
    ///   to 203 for HDR10 surfaces and 1.0 for floating point surfaces.
    /// - `SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT`: for HDR10 and floating point
    ///   surfaces, this defines the maximum dynamic range used by the content, in
    ///   terms of the SDR white point. This defaults to 0.0, which disables tone
    ///   mapping.
    /// - `SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING`: the tone mapping operator
    ///   used when compressing from a surface with high dynamic range to another
    ///   with lower dynamic range. Currently this supports "chrome", which uses
    ///   the same tone mapping that Chrome uses for HDR content, the form "*=N",
    ///   where N is a floating point scale factor applied in linear space, and
    ///   "none", which disables tone mapping. This defaults to "chrome".
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSurfaceProperties(surface: *mut SDL_Surface) -> SDL_PropertiesID;
}}

pub const SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.surface.SDR_white_point\0") };

pub const SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.surface.HDR_headroom\0") };

pub const SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.surface.tonemap\0") };

extern_sdlcall! {{
    /// Set the colorspace used by a surface.
    ///
    /// Setting the colorspace doesn't change the pixels, only how they are
    /// interpreted in color operations.
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param colorspace an SDL_ColorSpace value describing the surface
    ///                   colorspace.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceColorspace
    pub fn SDL_SetSurfaceColorspace(surface: *mut SDL_Surface, colorspace: SDL_Colorspace) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the colorspace used by a surface.
    ///
    /// The colorspace defaults to SDL_COLORSPACE_SRGB_LINEAR for floating point
    /// formats, SDL_COLORSPACE_HDR10 for 10-bit formats, SDL_COLORSPACE_SRGB for
    /// other RGB surfaces and SDL_COLORSPACE_BT709_FULL for YUV textures.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns the colorspace used by the surface, or SDL_COLORSPACE_UNKNOWN if
    ///          the surface is NULL.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceColorspace
    pub fn SDL_GetSurfaceColorspace(surface: *mut SDL_Surface) -> SDL_Colorspace;
}}

extern_sdlcall! {{
    /// Create a palette and associate it with a surface.
    ///
    /// This function creates a palette compatible with the provided surface. The
    /// palette is then returned for you to modify, and the surface will
    /// automatically use the new palette in future operations. You do not need to
    /// destroy the returned palette, it will be freed when the reference count
    /// reaches 0, usually when the surface is destroyed.
    ///
    /// Bitmap surfaces (with format SDL_PIXELFORMAT_INDEX1LSB or
    /// SDL_PIXELFORMAT_INDEX1MSB) will have the palette initialized with 0 as
    /// white and 1 as black. Other surfaces will get a palette initialized with
    /// white in every entry.
    ///
    /// If this function is called for a surface that already has a palette, a new
    /// palette will be created to replace it.
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \returns a new SDL_Palette structure on success or NULL on failure (e.g. if
    ///          the surface didn't have an index format); call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetPaletteColors
    pub fn SDL_CreateSurfacePalette(surface: *mut SDL_Surface) -> *mut SDL_Palette;
}}

extern_sdlcall! {{
    /// Set the palette used by a surface.
    ///
    /// A single palette can be shared with many surfaces.
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param palette the SDL_Palette structure to use.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreatePalette
    /// \sa SDL_GetSurfacePalette
    pub fn SDL_SetSurfacePalette(surface: *mut SDL_Surface, palette: *mut SDL_Palette) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the palette used by a surface.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns a pointer to the palette used by the surface, or NULL if there is
    ///          no palette used.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfacePalette
    pub fn SDL_GetSurfacePalette(surface: *mut SDL_Surface) -> *mut SDL_Palette;
}}

extern_sdlcall! {{
    /// Add an alternate version of a surface.
    ///
    /// This function adds an alternate version of this surface, usually used for
    /// content with high DPI representations like cursors or icons. The size,
    /// format, and content do not need to match the original surface, and these
    /// alternate versions will not be updated when the original surface changes.
    ///
    /// This function adds a reference to the alternate version, so you should call
    /// SDL_DestroySurface() on the image after this call.
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param image a pointer to an alternate SDL_Surface to associate with this
    ///              surface.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RemoveSurfaceAlternateImages
    /// \sa SDL_GetSurfaceImages
    /// \sa SDL_SurfaceHasAlternateImages
    pub fn SDL_AddSurfaceAlternateImage(surface: *mut SDL_Surface, image: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Return whether a surface has alternate versions available.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns SDL_TRUE if alternate versions are available or SDL_TRUE
    ///          otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddSurfaceAlternateImage
    /// \sa SDL_RemoveSurfaceAlternateImages
    /// \sa SDL_GetSurfaceImages
    pub fn SDL_SurfaceHasAlternateImages(surface: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get an array including all versions of a surface.
    ///
    /// This returns all versions of a surface, with the surface being queried as
    /// the first element in the returned array.
    ///
    /// Freeing the array of surfaces does not affect the surfaces in the array.
    /// They are still referenced by the surface being queried and will be cleaned
    /// up normally.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \param count a pointer filled in with the number of surface pointers
    ///              returned, may be NULL.
    /// \returns a NULL terminated array of SDL_Surface pointers or NULL on
    ///          failure; call SDL_GetError() for more information. This should be
    ///          freed with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddSurfaceAlternateImage
    /// \sa SDL_RemoveSurfaceAlternateImages
    /// \sa SDL_SurfaceHasAlternateImages
    pub fn SDL_GetSurfaceImages(surface: *mut SDL_Surface, count: *mut ::core::ffi::c_int) -> *mut *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Remove all alternate versions of a surface.
    ///
    /// This function removes a reference from all the alternative versions,
    /// destroying them if this is the last reference to them.
    ///
    /// \param surface the SDL_Surface structure to update.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AddSurfaceAlternateImage
    /// \sa SDL_GetSurfaceImages
    /// \sa SDL_SurfaceHasAlternateImages
    pub fn SDL_RemoveSurfaceAlternateImages(surface: *mut SDL_Surface);
}}

extern_sdlcall! {{
    /// Set up a surface for directly accessing the pixels.
    ///
    /// Between calls to SDL_LockSurface() / SDL_UnlockSurface(), you can write to
    /// and read from `surface->pixels`, using the pixel format stored in
    /// `surface->format`. Once you are done accessing the surface, you should use
    /// SDL_UnlockSurface() to release it.
    ///
    /// Not all surfaces require locking. If `SDL_MUSTLOCK(surface)` evaluates to
    /// 0, then you can read and write to the surface at any time, and the pixel
    /// format of the surface will not change.
    ///
    /// \param surface the SDL_Surface structure to be locked.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MUSTLOCK
    /// \sa SDL_UnlockSurface
    pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Release a surface after directly accessing the pixels.
    ///
    /// \param surface the SDL_Surface structure to be unlocked.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockSurface
    pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);
}}

extern_sdlcall! {{
    /// Load a BMP image from a seekable SDL data stream.
    ///
    /// The new surface should be freed with SDL_DestroySurface(). Not doing so
    /// will result in a memory leak.
    ///
    /// \param src the data stream for the surface.
    /// \param closeio if SDL_TRUE, calls SDL_CloseIO() on `src` before returning,
    ///                even in the case of an error.
    /// \returns a pointer to a new SDL_Surface structure or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroySurface
    /// \sa SDL_LoadBMP
    /// \sa SDL_SaveBMP_IO
    pub fn SDL_LoadBMP_IO(src: *mut SDL_IOStream, closeio: SDL_bool) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Load a BMP image from a file.
    ///
    /// The new surface should be freed with SDL_DestroySurface(). Not doing so
    /// will result in a memory leak.
    ///
    /// \param file the BMP file to load.
    /// \returns a pointer to a new SDL_Surface structure or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroySurface
    /// \sa SDL_LoadBMP_IO
    /// \sa SDL_SaveBMP
    pub fn SDL_LoadBMP(file: *const ::core::ffi::c_char) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Save a surface to a seekable SDL data stream in BMP format.
    ///
    /// Surfaces with a 24-bit, 32-bit and paletted 8-bit format get saved in the
    /// BMP directly. Other RGB formats with 8-bit or higher get converted to a
    /// 24-bit surface or, if they have an alpha mask or a colorkey, to a 32-bit
    /// surface before they are saved. YUV and paletted 1-bit and 4-bit formats are
    /// not supported.
    ///
    /// \param surface the SDL_Surface structure containing the image to be saved.
    /// \param dst a data stream to save to.
    /// \param closeio if SDL_TRUE, calls SDL_CloseIO() on `dst` before returning,
    ///                even in the case of an error.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LoadBMP_IO
    /// \sa SDL_SaveBMP
    pub fn SDL_SaveBMP_IO(surface: *mut SDL_Surface, dst: *mut SDL_IOStream, closeio: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Save a surface to a file.
    ///
    /// Surfaces with a 24-bit, 32-bit and paletted 8-bit format get saved in the
    /// BMP directly. Other RGB formats with 8-bit or higher get converted to a
    /// 24-bit surface or, if they have an alpha mask or a colorkey, to a 32-bit
    /// surface before they are saved. YUV and paletted 1-bit and 4-bit formats are
    /// not supported.
    ///
    /// \param surface the SDL_Surface structure containing the image to be saved.
    /// \param file a file to save to.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LoadBMP
    /// \sa SDL_SaveBMP_IO
    pub fn SDL_SaveBMP(surface: *mut SDL_Surface, file: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the RLE acceleration hint for a surface.
    ///
    /// If RLE is enabled, color key and alpha blending blits are much faster, but
    /// the surface must be locked before directly accessing the pixels.
    ///
    /// \param surface the SDL_Surface structure to optimize.
    /// \param enabled SDL_TRUE to enable RLE acceleration, SDL_FALSE to disable
    ///                it.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    /// \sa SDL_LockSurface
    /// \sa SDL_UnlockSurface
    pub fn SDL_SetSurfaceRLE(surface: *mut SDL_Surface, enabled: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Returns whether the surface is RLE enabled.
    ///
    /// It is safe to pass a NULL `surface` here; it will return SDL_FALSE.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns SDL_TRUE if the surface is RLE enabled, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceRLE
    pub fn SDL_SurfaceHasRLE(surface: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the color key (transparent pixel) in a surface.
    ///
    /// The color key defines a pixel value that will be treated as transparent in
    /// a blit. For example, one can use this to specify that cyan pixels should be
    /// considered transparent, and therefore not rendered.
    ///
    /// It is a pixel of the format used by the surface, as generated by
    /// SDL_MapRGB().
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param enabled SDL_TRUE to enable color key, SDL_FALSE to disable color
    ///                key.
    /// \param key the transparent pixel.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceColorKey
    /// \sa SDL_SetSurfaceRLE
    /// \sa SDL_SurfaceHasColorKey
    pub fn SDL_SetSurfaceColorKey(surface: *mut SDL_Surface, enabled: SDL_bool, key: Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Returns whether the surface has a color key.
    ///
    /// It is safe to pass a NULL `surface` here; it will return SDL_FALSE.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \returns SDL_TRUE if the surface has a color key, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceColorKey
    /// \sa SDL_GetSurfaceColorKey
    pub fn SDL_SurfaceHasColorKey(surface: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the color key (transparent pixel) for a surface.
    ///
    /// The color key is a pixel of the format used by the surface, as generated by
    /// SDL_MapRGB().
    ///
    /// If the surface doesn't have color key enabled this function returns -1.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \param key a pointer filled in with the transparent pixel.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceColorKey
    /// \sa SDL_SurfaceHasColorKey
    pub fn SDL_GetSurfaceColorKey(surface: *mut SDL_Surface, key: *mut Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set an additional color value multiplied into blit operations.
    ///
    /// When this surface is blitted, during the blit operation each source color
    /// channel is modulated by the appropriate color value according to the
    /// following formula:
    ///
    /// `srcC = srcC * (color / 255)`
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param r the red color value multiplied into blit operations.
    /// \param g the green color value multiplied into blit operations.
    /// \param b the blue color value multiplied into blit operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceColorMod
    /// \sa SDL_SetSurfaceAlphaMod
    pub fn SDL_SetSurfaceColorMod(surface: *mut SDL_Surface, r: Uint8, g: Uint8, b: Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the additional color value multiplied into blit operations.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \param r a pointer filled in with the current red color value.
    /// \param g a pointer filled in with the current green color value.
    /// \param b a pointer filled in with the current blue color value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceAlphaMod
    /// \sa SDL_SetSurfaceColorMod
    pub fn SDL_GetSurfaceColorMod(surface: *mut SDL_Surface, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set an additional alpha value used in blit operations.
    ///
    /// When this surface is blitted, during the blit operation the source alpha
    /// value is modulated by this alpha value according to the following formula:
    ///
    /// `srcA = srcA * (alpha / 255)`
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param alpha the alpha value multiplied into blit operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceAlphaMod
    /// \sa SDL_SetSurfaceColorMod
    pub fn SDL_SetSurfaceAlphaMod(surface: *mut SDL_Surface, alpha: Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the additional alpha value used in blit operations.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \param alpha a pointer filled in with the current alpha value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceColorMod
    /// \sa SDL_SetSurfaceAlphaMod
    pub fn SDL_GetSurfaceAlphaMod(surface: *mut SDL_Surface, alpha: *mut Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the blend mode used for blit operations.
    ///
    /// To copy a surface to another surface (or texture) without blending with the
    /// existing data, the blendmode of the SOURCE surface should be set to
    /// `SDL_BLENDMODE_NONE`.
    ///
    /// \param surface the SDL_Surface structure to update.
    /// \param blendMode the SDL_BlendMode to use for blit blending.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceBlendMode
    pub fn SDL_SetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: SDL_BlendMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the blend mode used for blit operations.
    ///
    /// \param surface the SDL_Surface structure to query.
    /// \param blendMode a pointer filled in with the current SDL_BlendMode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceBlendMode
    pub fn SDL_GetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: *mut SDL_BlendMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the clipping rectangle for a surface.
    ///
    /// When `surface` is the destination of a blit, only the area within the clip
    /// rectangle is drawn into.
    ///
    /// Note that blits are automatically clipped to the edges of the source and
    /// destination surfaces.
    ///
    /// \param surface the SDL_Surface structure to be clipped.
    /// \param rect the SDL_Rect structure representing the clipping rectangle, or
    ///             NULL to disable clipping.
    /// \returns SDL_TRUE if the rectangle intersects the surface, otherwise
    ///          SDL_FALSE and blits will be completely clipped.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetSurfaceClipRect
    pub fn SDL_SetSurfaceClipRect(surface: *mut SDL_Surface, rect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the clipping rectangle for a surface.
    ///
    /// When `surface` is the destination of a blit, only the area within the clip
    /// rectangle is drawn into.
    ///
    /// \param surface the SDL_Surface structure representing the surface to be
    ///                clipped.
    /// \param rect an SDL_Rect structure filled in with the clipping rectangle for
    ///             the surface.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetSurfaceClipRect
    pub fn SDL_GetSurfaceClipRect(surface: *mut SDL_Surface, rect: *mut SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Flip a surface vertically or horizontally.
    ///
    /// \param surface the surface to flip.
    /// \param flip the direction to flip.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_FlipSurface(surface: *mut SDL_Surface, flip: SDL_FlipMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Creates a new surface identical to the existing surface.
    ///
    /// If the original surface has alternate images, the new surface will have a
    /// reference to them as well.
    ///
    /// The returned surface should be freed with SDL_DestroySurface().
    ///
    /// \param surface the surface to duplicate.
    /// \returns a copy of the surface or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroySurface
    pub fn SDL_DuplicateSurface(surface: *mut SDL_Surface) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Creates a new surface identical to the existing surface, scaled to the
    /// desired size.
    ///
    /// The returned surface should be freed with SDL_DestroySurface().
    ///
    /// \param surface the surface to duplicate and scale.
    /// \param width the width of the new surface.
    /// \param height the height of the new surface.
    /// \param scaleMode the SDL_ScaleMode to be used.
    /// \returns a copy of the surface or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroySurface
    pub fn SDL_ScaleSurface(surface: *mut SDL_Surface, width: ::core::ffi::c_int, height: ::core::ffi::c_int, scaleMode: SDL_ScaleMode) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Copy an existing surface to a new surface of the specified format.
    ///
    /// This function is used to optimize images for faster *repeat* blitting. This
    /// is accomplished by converting the original and storing the result as a new
    /// surface. The new, optimized surface can then be used as the source for
    /// future blits, making them faster.
    ///
    /// If you are converting to an indexed surface and want to map colors to a
    /// palette, you can use SDL_ConvertSurfaceAndColorspace() instead.
    ///
    /// If the original surface has alternate images, the new surface will have a
    /// reference to them as well.
    ///
    /// \param surface the existing SDL_Surface structure to convert.
    /// \param format the new pixel format.
    /// \returns the new SDL_Surface structure that is created or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ConvertSurfaceAndColorspace
    /// \sa SDL_DestroySurface
    pub fn SDL_ConvertSurface(surface: *mut SDL_Surface, format: SDL_PixelFormat) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Copy an existing surface to a new surface of the specified format and
    /// colorspace.
    ///
    /// This function converts an existing surface to a new format and colorspace
    /// and returns the new surface. This will perform any pixel format and
    /// colorspace conversion needed.
    ///
    /// If the original surface has alternate images, the new surface will have a
    /// reference to them as well.
    ///
    /// \param surface the existing SDL_Surface structure to convert.
    /// \param format the new pixel format.
    /// \param palette an optional palette to use for indexed formats, may be NULL.
    /// \param colorspace the new colorspace.
    /// \param props an SDL_PropertiesID with additional color properties, or 0.
    /// \returns the new SDL_Surface structure that is created or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ConvertSurface
    /// \sa SDL_ConvertSurface
    /// \sa SDL_DestroySurface
    pub fn SDL_ConvertSurfaceAndColorspace(surface: *mut SDL_Surface, format: SDL_PixelFormat, palette: *mut SDL_Palette, colorspace: SDL_Colorspace, props: SDL_PropertiesID) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Copy a block of pixels of one format to another format.
    ///
    /// \param width the width of the block to copy, in pixels.
    /// \param height the height of the block to copy, in pixels.
    /// \param src_format an SDL_PixelFormat value of the `src` pixels format.
    /// \param src a pointer to the source pixels.
    /// \param src_pitch the pitch of the source pixels, in bytes.
    /// \param dst_format an SDL_PixelFormat value of the `dst` pixels format.
    /// \param dst a pointer to be filled in with new pixel data.
    /// \param dst_pitch the pitch of the destination pixels, in bytes.
    /// \returns SDL_FALSE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ConvertPixelsAndColorspace
    pub fn SDL_ConvertPixels(width: ::core::ffi::c_int, height: ::core::ffi::c_int, src_format: SDL_PixelFormat, src: *const ::core::ffi::c_void, src_pitch: ::core::ffi::c_int, dst_format: SDL_PixelFormat, dst: *mut ::core::ffi::c_void, dst_pitch: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Copy a block of pixels of one format and colorspace to another format and
    /// colorspace.
    ///
    /// \param width the width of the block to copy, in pixels.
    /// \param height the height of the block to copy, in pixels.
    /// \param src_format an SDL_PixelFormat value of the `src` pixels format.
    /// \param src_colorspace an SDL_ColorSpace value describing the colorspace of
    ///                       the `src` pixels.
    /// \param src_properties an SDL_PropertiesID with additional source color
    ///                       properties, or 0.
    /// \param src a pointer to the source pixels.
    /// \param src_pitch the pitch of the source pixels, in bytes.
    /// \param dst_format an SDL_PixelFormat value of the `dst` pixels format.
    /// \param dst_colorspace an SDL_ColorSpace value describing the colorspace of
    ///                       the `dst` pixels.
    /// \param dst_properties an SDL_PropertiesID with additional destination color
    ///                       properties, or 0.
    /// \param dst a pointer to be filled in with new pixel data.
    /// \param dst_pitch the pitch of the destination pixels, in bytes.
    /// \returns SDL_FALSE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ConvertPixels
    pub fn SDL_ConvertPixelsAndColorspace(width: ::core::ffi::c_int, height: ::core::ffi::c_int, src_format: SDL_PixelFormat, src_colorspace: SDL_Colorspace, src_properties: SDL_PropertiesID, src: *const ::core::ffi::c_void, src_pitch: ::core::ffi::c_int, dst_format: SDL_PixelFormat, dst_colorspace: SDL_Colorspace, dst_properties: SDL_PropertiesID, dst: *mut ::core::ffi::c_void, dst_pitch: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Premultiply the alpha on a block of pixels.
    ///
    /// This is safe to use with src == dst, but not for other overlapping areas.
    ///
    /// \param width the width of the block to convert, in pixels.
    /// \param height the height of the block to convert, in pixels.
    /// \param src_format an SDL_PixelFormat value of the `src` pixels format.
    /// \param src a pointer to the source pixels.
    /// \param src_pitch the pitch of the source pixels, in bytes.
    /// \param dst_format an SDL_PixelFormat value of the `dst` pixels format.
    /// \param dst a pointer to be filled in with premultiplied pixel data.
    /// \param dst_pitch the pitch of the destination pixels, in bytes.
    /// \param linear SDL_TRUE to convert from sRGB to linear space for the alpha
    ///               multiplication, SDL_FALSE to do multiplication in sRGB space.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_PremultiplyAlpha(width: ::core::ffi::c_int, height: ::core::ffi::c_int, src_format: SDL_PixelFormat, src: *const ::core::ffi::c_void, src_pitch: ::core::ffi::c_int, dst_format: SDL_PixelFormat, dst: *mut ::core::ffi::c_void, dst_pitch: ::core::ffi::c_int, linear: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Premultiply the alpha in a surface.
    ///
    /// This is safe to use with src == dst, but not for other overlapping areas.
    ///
    /// \param surface the surface to modify.
    /// \param linear SDL_TRUE to convert from sRGB to linear space for the alpha
    ///               multiplication, SDL_FALSE to do multiplication in sRGB space.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_PremultiplySurfaceAlpha(surface: *mut SDL_Surface, linear: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Clear a surface with a specific color, with floating point precision.
    ///
    /// This function handles all surface formats, and ignores any clip rectangle.
    ///
    /// If the surface is YUV, the color is assumed to be in the sRGB colorspace,
    /// otherwise the color is assumed to be in the colorspace of the suface.
    ///
    /// \param surface the SDL_Surface to clear.
    /// \param r the red component of the pixel, normally in the range 0-1.
    /// \param g the green component of the pixel, normally in the range 0-1.
    /// \param b the blue component of the pixel, normally in the range 0-1.
    /// \param a the alpha component of the pixel, normally in the range 0-1.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ClearSurface(surface: *mut SDL_Surface, r: ::core::ffi::c_float, g: ::core::ffi::c_float, b: ::core::ffi::c_float, a: ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a fast fill of a rectangle with a specific color.
    ///
    /// `color` should be a pixel of the format used by the surface, and can be
    /// generated by SDL_MapRGB() or SDL_MapRGBA(). If the color value contains an
    /// alpha component then the destination is simply filled with that alpha
    /// information, no blending takes place.
    ///
    /// If there is a clip rectangle set on the destination (set via
    /// SDL_SetSurfaceClipRect()), then this function will fill based on the
    /// intersection of the clip rectangle and `rect`.
    ///
    /// \param dst the SDL_Surface structure that is the drawing target.
    /// \param rect the SDL_Rect structure representing the rectangle to fill, or
    ///             NULL to fill the entire surface.
    /// \param color the color to fill with.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_FillSurfaceRects
    pub fn SDL_FillSurfaceRect(dst: *mut SDL_Surface, rect: *const SDL_Rect, color: Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a fast fill of a set of rectangles with a specific color.
    ///
    /// `color` should be a pixel of the format used by the surface, and can be
    /// generated by SDL_MapRGB() or SDL_MapRGBA(). If the color value contains an
    /// alpha component then the destination is simply filled with that alpha
    /// information, no blending takes place.
    ///
    /// If there is a clip rectangle set on the destination (set via
    /// SDL_SetSurfaceClipRect()), then this function will fill based on the
    /// intersection of the clip rectangle and `rect`.
    ///
    /// \param dst the SDL_Surface structure that is the drawing target.
    /// \param rects an array of SDL_Rects representing the rectangles to fill.
    /// \param count the number of rectangles in the array.
    /// \param color the color to fill with.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_FillSurfaceRect
    pub fn SDL_FillSurfaceRects(dst: *mut SDL_Surface, rects: *const SDL_Rect, count: ::core::ffi::c_int, color: Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Performs a fast blit from the source surface to the destination surface.
    ///
    /// This assumes that the source and destination rectangles are the same size.
    /// If either `srcrect` or `dstrect` are NULL, the entire surface (`src` or
    /// `dst`) is copied. The final blit rectangles are saved in `srcrect` and
    /// `dstrect` after all clipping is performed.
    ///
    /// The blit function should not be called on a locked surface.
    ///
    /// The blit semantics for surfaces with and without blending and colorkey are
    /// defined as follows:
    ///
    /// ```
    ///    RGBA->RGB:
    ///      Source surface blend mode set to SDL_BLENDMODE_BLEND:
    ///       alpha-blend (using the source alpha-channel and per-surface alpha)
    ///       SDL_SRCCOLORKEY ignored.
    ///     Source surface blend mode set to SDL_BLENDMODE_NONE:
    ///       copy RGB.
    ///       if SDL_SRCCOLORKEY set, only copy the pixels that do not match the
    ///       RGB values of the source color key, ignoring alpha in the
    ///       comparison.
    ///
    ///   RGB->RGBA:
    ///     Source surface blend mode set to SDL_BLENDMODE_BLEND:
    ///       alpha-blend (using the source per-surface alpha)
    ///     Source surface blend mode set to SDL_BLENDMODE_NONE:
    ///       copy RGB, set destination alpha to source per-surface alpha value.
    ///     both:
    ///       if SDL_SRCCOLORKEY set, only copy the pixels that do not match the
    ///       source color key.
    ///
    ///   RGBA->RGBA:
    ///     Source surface blend mode set to SDL_BLENDMODE_BLEND:
    ///       alpha-blend (using the source alpha-channel and per-surface alpha)
    ///       SDL_SRCCOLORKEY ignored.
    ///     Source surface blend mode set to SDL_BLENDMODE_NONE:
    ///       copy all of RGBA to the destination.
    ///       if SDL_SRCCOLORKEY set, only copy the pixels that do not match the
    ///       RGB values of the source color key, ignoring alpha in the
    ///       comparison.
    ///
    ///   RGB->RGB:
    ///     Source surface blend mode set to SDL_BLENDMODE_BLEND:
    ///       alpha-blend (using the source per-surface alpha)
    ///     Source surface blend mode set to SDL_BLENDMODE_NONE:
    ///       copy RGB.
    ///     both:
    ///       if SDL_SRCCOLORKEY set, only copy the pixels that do not match the
    ///       source color key.
    /// ```
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, or NULL to copy the entire surface.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the x and y position in
    ///                the destination surface, or NULL for (0,0). The width and
    ///                height are ignored, and are copied from `srcrect`. If you
    ///                want a specific width and height, you should use
    ///                SDL_BlitSurfaceScaled().
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurfaceScaled
    pub fn SDL_BlitSurface(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform low-level surface blitting only.
    ///
    /// This is a semi-private blit function and it performs low-level surface
    /// blitting, assuming the input rectangles have already been clipped.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, may not be NULL.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, may not be NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    pub fn SDL_BlitSurfaceUnchecked(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a scaled blit to a destination surface, which may be of a different
    /// format.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, or NULL to copy the entire surface.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, or NULL to fill the entire
    ///                destination surface.
    /// \param scaleMode the SDL_ScaleMode to be used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    pub fn SDL_BlitSurfaceScaled(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *const SDL_Rect, scaleMode: SDL_ScaleMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform low-level surface scaled blitting only.
    ///
    /// This is a semi-private function and it performs low-level surface blitting,
    /// assuming the input rectangles have already been clipped.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, may not be NULL.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, may not be NULL.
    /// \param scaleMode the SDL_ScaleMode to be used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurfaceScaled
    pub fn SDL_BlitSurfaceUncheckedScaled(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *const SDL_Rect, scaleMode: SDL_ScaleMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a tiled blit to a destination surface, which may be of a different
    /// format.
    ///
    /// The pixels in `srcrect` will be repeated as many times as needed to
    /// completely fill `dstrect`.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, or NULL to copy the entire surface.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, or NULL to fill the entire surface.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    pub fn SDL_BlitSurfaceTiled(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a scaled and tiled blit to a destination surface, which may be of a
    /// different format.
    ///
    /// The pixels in `srcrect` will be scaled and repeated as many times as needed
    /// to completely fill `dstrect`.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be
    ///                copied, or NULL to copy the entire surface.
    /// \param scale the scale used to transform srcrect into the destination
    ///              rectangle, e.g. a 32x32 texture with a scale of 2 would fill
    ///              64x64 tiles.
    /// \param scaleMode scale algorithm to be used.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, or NULL to fill the entire surface.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    pub fn SDL_BlitSurfaceTiledWithScale(src: *mut SDL_Surface, srcrect: *const SDL_Rect, scale: ::core::ffi::c_float, scaleMode: SDL_ScaleMode, dst: *mut SDL_Surface, dstrect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Perform a scaled blit using the 9-grid algorithm to a destination surface,
    /// which may be of a different format.
    ///
    /// The pixels in the source surface are split into a 3x3 grid, using the
    /// different corner sizes for each corner, and the sides and center making up
    /// the remaining pixels. The corners are then scaled using `scale` and fit
    /// into the corners of the destination rectangle. The sides and center are
    /// then stretched into place to cover the remaining destination rectangle.
    ///
    /// \param src the SDL_Surface structure to be copied from.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be used
    ///                for the 9-grid, or NULL to use the entire surface.
    /// \param left_width the width, in pixels, of the left corners in `srcrect`.
    /// \param right_width the width, in pixels, of the right corners in `srcrect`.
    /// \param top_height the height, in pixels, of the top corners in `srcrect`.
    /// \param bottom_height the height, in pixels, of the bottom corners in
    ///                      `srcrect`.
    /// \param scale the scale used to transform the corner of `srcrect` into the
    ///              corner of `dstrect`, or 0.0f for an unscaled blit.
    /// \param scaleMode scale algorithm to be used.
    /// \param dst the SDL_Surface structure that is the blit target.
    /// \param dstrect the SDL_Rect structure representing the target rectangle in
    ///                the destination surface, or NULL to fill the entire surface.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety The same destination surface should not be used from two
    ///               threads at once. It is safe to use the same source surface
    ///               from multiple threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BlitSurface
    pub fn SDL_BlitSurface9Grid(src: *mut SDL_Surface, srcrect: *const SDL_Rect, left_width: ::core::ffi::c_int, right_width: ::core::ffi::c_int, top_height: ::core::ffi::c_int, bottom_height: ::core::ffi::c_int, scale: ::core::ffi::c_float, scaleMode: SDL_ScaleMode, dst: *mut SDL_Surface, dstrect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Map an RGB triple to an opaque pixel value for a surface.
    ///
    /// This function maps the RGB color value to the specified pixel format and
    /// returns the pixel value best approximating the given RGB color value for
    /// the given pixel format.
    ///
    /// If the surface has a palette, the index of the closest matching color in
    /// the palette will be returned.
    ///
    /// If the surface pixel format has an alpha component it will be returned as
    /// all 1 bits (fully opaque).
    ///
    /// If the pixel format bpp (color depth) is less than 32-bpp then the unused
    /// upper bits of the return value can safely be ignored (e.g., with a 16-bpp
    /// format the return value can be assigned to a Uint16, and similarly a Uint8
    /// for an 8-bpp format).
    ///
    /// \param surface the surface to use for the pixel format and palette.
    /// \param r the red component of the pixel in the range 0-255.
    /// \param g the green component of the pixel in the range 0-255.
    /// \param b the blue component of the pixel in the range 0-255.
    /// \returns a pixel value.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MapSurfaceRGBA
    pub fn SDL_MapSurfaceRGB(surface: *mut SDL_Surface, r: Uint8, g: Uint8, b: Uint8) -> Uint32;
}}

extern_sdlcall! {{
    /// Map an RGBA quadruple to a pixel value for a surface.
    ///
    /// This function maps the RGBA color value to the specified pixel format and
    /// returns the pixel value best approximating the given RGBA color value for
    /// the given pixel format.
    ///
    /// If the surface pixel format has no alpha component the alpha value will be
    /// ignored (as it will be in formats with a palette).
    ///
    /// If the surface has a palette, the index of the closest matching color in
    /// the palette will be returned.
    ///
    /// If the pixel format bpp (color depth) is less than 32-bpp then the unused
    /// upper bits of the return value can safely be ignored (e.g., with a 16-bpp
    /// format the return value can be assigned to a Uint16, and similarly a Uint8
    /// for an 8-bpp format).
    ///
    /// \param surface the surface to use for the pixel format and palette.
    /// \param r the red component of the pixel in the range 0-255.
    /// \param g the green component of the pixel in the range 0-255.
    /// \param b the blue component of the pixel in the range 0-255.
    /// \param a the alpha component of the pixel in the range 0-255.
    /// \returns a pixel value.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MapSurfaceRGB
    pub fn SDL_MapSurfaceRGBA(surface: *mut SDL_Surface, r: Uint8, g: Uint8, b: Uint8, a: Uint8) -> Uint32;
}}

extern_sdlcall! {{
    /// Retrieves a single pixel from a surface.
    ///
    /// This function prioritizes correctness over speed: it is suitable for unit
    /// tests, but is not intended for use in a game engine.
    ///
    /// Like SDL_GetRGBA, this uses the entire 0..255 range when converting color
    /// components from pixel formats with less than 8 bits per RGB component.
    ///
    /// \param surface the surface to read.
    /// \param x the horizontal coordinate, 0 <= x < width.
    /// \param y the vertical coordinate, 0 <= y < height.
    /// \param r a pointer filled in with the red channel, 0-255, or NULL to ignore
    ///          this channel.
    /// \param g a pointer filled in with the green channel, 0-255, or NULL to
    ///          ignore this channel.
    /// \param b a pointer filled in with the blue channel, 0-255, or NULL to
    ///          ignore this channel.
    /// \param a a pointer filled in with the alpha channel, 0-255, or NULL to
    ///          ignore this channel.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReadSurfacePixel(surface: *mut SDL_Surface, x: ::core::ffi::c_int, y: ::core::ffi::c_int, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8, a: *mut Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Retrieves a single pixel from a surface.
    ///
    /// This function prioritizes correctness over speed: it is suitable for unit
    /// tests, but is not intended for use in a game engine.
    ///
    /// \param surface the surface to read.
    /// \param x the horizontal coordinate, 0 <= x < width.
    /// \param y the vertical coordinate, 0 <= y < height.
    /// \param r a pointer filled in with the red channel, normally in the range
    ///          0-1, or NULL to ignore this channel.
    /// \param g a pointer filled in with the green channel, normally in the range
    ///          0-1, or NULL to ignore this channel.
    /// \param b a pointer filled in with the blue channel, normally in the range
    ///          0-1, or NULL to ignore this channel.
    /// \param a a pointer filled in with the alpha channel, normally in the range
    ///          0-1, or NULL to ignore this channel.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReadSurfacePixelFloat(surface: *mut SDL_Surface, x: ::core::ffi::c_int, y: ::core::ffi::c_int, r: *mut ::core::ffi::c_float, g: *mut ::core::ffi::c_float, b: *mut ::core::ffi::c_float, a: *mut ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Writes a single pixel to a surface.
    ///
    /// This function prioritizes correctness over speed: it is suitable for unit
    /// tests, but is not intended for use in a game engine.
    ///
    /// Like SDL_MapRGBA, this uses the entire 0..255 range when converting color
    /// components from pixel formats with less than 8 bits per RGB component.
    ///
    /// \param surface the surface to write.
    /// \param x the horizontal coordinate, 0 <= x < width.
    /// \param y the vertical coordinate, 0 <= y < height.
    /// \param r the red channel value, 0-255.
    /// \param g the green channel value, 0-255.
    /// \param b the blue channel value, 0-255.
    /// \param a the alpha channel value, 0-255.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_WriteSurfacePixel(surface: *mut SDL_Surface, x: ::core::ffi::c_int, y: ::core::ffi::c_int, r: Uint8, g: Uint8, b: Uint8, a: Uint8) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Writes a single pixel to a surface.
    ///
    /// This function prioritizes correctness over speed: it is suitable for unit
    /// tests, but is not intended for use in a game engine.
    ///
    /// \param surface the surface to write.
    /// \param x the horizontal coordinate, 0 <= x < width.
    /// \param y the vertical coordinate, 0 <= y < height.
    /// \param r the red channel value, normally in the range 0-1.
    /// \param g the green channel value, normally in the range 0-1.
    /// \param b the blue channel value, normally in the range 0-1.
    /// \param a the alpha channel value, normally in the range 0-1.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_WriteSurfacePixelFloat(surface: *mut SDL_Surface, x: ::core::ffi::c_int, y: ::core::ffi::c_int, r: ::core::ffi::c_float, g: ::core::ffi::c_float, b: ::core::ffi::c_float, a: ::core::ffi::c_float) -> SDL_bool;
}}

#[repr(C)]
#[non_exhaustive]
pub struct SDL_SurfaceData { _opaque: [::core::primitive::u8; 0] }

