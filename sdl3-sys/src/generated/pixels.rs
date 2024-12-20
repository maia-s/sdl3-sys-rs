//! Pixel management.

use super::stdinc::*;

use super::error::*;

/// A fully opaque 8-bit alpha value.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ALPHA_TRANSPARENT`]
pub const SDL_ALPHA_OPAQUE: Uint8 = (255 as Uint8);

/// A fully opaque floating point alpha value.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ALPHA_TRANSPARENT_FLOAT`]
pub const SDL_ALPHA_OPAQUE_FLOAT: ::core::ffi::c_float = 1.0_f32;

/// A fully transparent 8-bit alpha value.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ALPHA_OPAQUE`]
pub const SDL_ALPHA_TRANSPARENT: Uint8 = (0 as Uint8);

/// A fully transparent floating point alpha value.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ALPHA_OPAQUE_FLOAT`]
pub const SDL_ALPHA_TRANSPARENT_FLOAT: ::core::ffi::c_float = 0.0_f32;

/// Pixel type.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_PixelType::UNKNOWN) | [`SDL_PIXELTYPE_UNKNOWN`] | |
/// | [`INDEX1`](SDL_PixelType::INDEX1) | [`SDL_PIXELTYPE_INDEX1`] | |
/// | [`INDEX4`](SDL_PixelType::INDEX4) | [`SDL_PIXELTYPE_INDEX4`] | |
/// | [`INDEX8`](SDL_PixelType::INDEX8) | [`SDL_PIXELTYPE_INDEX8`] | |
/// | [`PACKED8`](SDL_PixelType::PACKED8) | [`SDL_PIXELTYPE_PACKED8`] | |
/// | [`PACKED16`](SDL_PixelType::PACKED16) | [`SDL_PIXELTYPE_PACKED16`] | |
/// | [`PACKED32`](SDL_PixelType::PACKED32) | [`SDL_PIXELTYPE_PACKED32`] | |
/// | [`ARRAYU8`](SDL_PixelType::ARRAYU8) | [`SDL_PIXELTYPE_ARRAYU8`] | |
/// | [`ARRAYU16`](SDL_PixelType::ARRAYU16) | [`SDL_PIXELTYPE_ARRAYU16`] | |
/// | [`ARRAYU32`](SDL_PixelType::ARRAYU32) | [`SDL_PIXELTYPE_ARRAYU32`] | |
/// | [`ARRAYF16`](SDL_PixelType::ARRAYF16) | [`SDL_PIXELTYPE_ARRAYF16`] | |
/// | [`ARRAYF32`](SDL_PixelType::ARRAYF32) | [`SDL_PIXELTYPE_ARRAYF32`] | |
/// | [`INDEX2`](SDL_PixelType::INDEX2) | [`SDL_PIXELTYPE_INDEX2`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PixelType(pub ::core::ffi::c_int);

impl From<SDL_PixelType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PixelType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PixelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_PIXELTYPE_UNKNOWN",
            Self::INDEX1 => "SDL_PIXELTYPE_INDEX1",
            Self::INDEX4 => "SDL_PIXELTYPE_INDEX4",
            Self::INDEX8 => "SDL_PIXELTYPE_INDEX8",
            Self::PACKED8 => "SDL_PIXELTYPE_PACKED8",
            Self::PACKED16 => "SDL_PIXELTYPE_PACKED16",
            Self::PACKED32 => "SDL_PIXELTYPE_PACKED32",
            Self::ARRAYU8 => "SDL_PIXELTYPE_ARRAYU8",
            Self::ARRAYU16 => "SDL_PIXELTYPE_ARRAYU16",
            Self::ARRAYU32 => "SDL_PIXELTYPE_ARRAYU32",
            Self::ARRAYF16 => "SDL_PIXELTYPE_ARRAYF16",
            Self::ARRAYF32 => "SDL_PIXELTYPE_ARRAYF32",
            Self::INDEX2 => "SDL_PIXELTYPE_INDEX2",

            _ => return write!(f, "SDL_PixelType({})", self.0),
        })
    }
}

impl SDL_PixelType {
    pub const UNKNOWN: Self = Self(0);
    pub const INDEX1: Self = Self(1);
    pub const INDEX4: Self = Self(2);
    pub const INDEX8: Self = Self(3);
    pub const PACKED8: Self = Self(4);
    pub const PACKED16: Self = Self(5);
    pub const PACKED32: Self = Self(6);
    pub const ARRAYU8: Self = Self(7);
    pub const ARRAYU16: Self = Self(8);
    pub const ARRAYU32: Self = Self(9);
    pub const ARRAYF16: Self = Self(10);
    pub const ARRAYF32: Self = Self(11);
    pub const INDEX2: Self = Self(12);
}

pub const SDL_PIXELTYPE_UNKNOWN: SDL_PixelType = SDL_PixelType::UNKNOWN;
pub const SDL_PIXELTYPE_INDEX1: SDL_PixelType = SDL_PixelType::INDEX1;
pub const SDL_PIXELTYPE_INDEX4: SDL_PixelType = SDL_PixelType::INDEX4;
pub const SDL_PIXELTYPE_INDEX8: SDL_PixelType = SDL_PixelType::INDEX8;
pub const SDL_PIXELTYPE_PACKED8: SDL_PixelType = SDL_PixelType::PACKED8;
pub const SDL_PIXELTYPE_PACKED16: SDL_PixelType = SDL_PixelType::PACKED16;
pub const SDL_PIXELTYPE_PACKED32: SDL_PixelType = SDL_PixelType::PACKED32;
pub const SDL_PIXELTYPE_ARRAYU8: SDL_PixelType = SDL_PixelType::ARRAYU8;
pub const SDL_PIXELTYPE_ARRAYU16: SDL_PixelType = SDL_PixelType::ARRAYU16;
pub const SDL_PIXELTYPE_ARRAYU32: SDL_PixelType = SDL_PixelType::ARRAYU32;
pub const SDL_PIXELTYPE_ARRAYF16: SDL_PixelType = SDL_PixelType::ARRAYF16;
pub const SDL_PIXELTYPE_ARRAYF32: SDL_PixelType = SDL_PixelType::ARRAYF32;
pub const SDL_PIXELTYPE_INDEX2: SDL_PixelType = SDL_PixelType::INDEX2;

/// Bitmap pixel order, high bit -> low bit.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_BitmapOrder::NONE) | [`SDL_BITMAPORDER_NONE`] | |
/// | [`_4321`](SDL_BitmapOrder::_4321) | [`SDL_BITMAPORDER_4321`] | |
/// | [`_1234`](SDL_BitmapOrder::_1234) | [`SDL_BITMAPORDER_1234`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_BitmapOrder(pub ::core::ffi::c_int);

impl From<SDL_BitmapOrder> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_BitmapOrder) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_BitmapOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_BITMAPORDER_NONE",
            Self::_4321 => "SDL_BITMAPORDER_4321",
            Self::_1234 => "SDL_BITMAPORDER_1234",

            _ => return write!(f, "SDL_BitmapOrder({})", self.0),
        })
    }
}

impl SDL_BitmapOrder {
    pub const NONE: Self = Self(0);
    pub const _4321: Self = Self(1);
    pub const _1234: Self = Self(2);
}

pub const SDL_BITMAPORDER_NONE: SDL_BitmapOrder = SDL_BitmapOrder::NONE;
pub const SDL_BITMAPORDER_4321: SDL_BitmapOrder = SDL_BitmapOrder::_4321;
pub const SDL_BITMAPORDER_1234: SDL_BitmapOrder = SDL_BitmapOrder::_1234;

/// Packed component order, high bit -> low bit.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_PackedOrder::NONE) | [`SDL_PACKEDORDER_NONE`] | |
/// | [`XRGB`](SDL_PackedOrder::XRGB) | [`SDL_PACKEDORDER_XRGB`] | |
/// | [`RGBX`](SDL_PackedOrder::RGBX) | [`SDL_PACKEDORDER_RGBX`] | |
/// | [`ARGB`](SDL_PackedOrder::ARGB) | [`SDL_PACKEDORDER_ARGB`] | |
/// | [`RGBA`](SDL_PackedOrder::RGBA) | [`SDL_PACKEDORDER_RGBA`] | |
/// | [`XBGR`](SDL_PackedOrder::XBGR) | [`SDL_PACKEDORDER_XBGR`] | |
/// | [`BGRX`](SDL_PackedOrder::BGRX) | [`SDL_PACKEDORDER_BGRX`] | |
/// | [`ABGR`](SDL_PackedOrder::ABGR) | [`SDL_PACKEDORDER_ABGR`] | |
/// | [`BGRA`](SDL_PackedOrder::BGRA) | [`SDL_PACKEDORDER_BGRA`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PackedOrder(pub ::core::ffi::c_int);

impl From<SDL_PackedOrder> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PackedOrder) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PackedOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_PACKEDORDER_NONE",
            Self::XRGB => "SDL_PACKEDORDER_XRGB",
            Self::RGBX => "SDL_PACKEDORDER_RGBX",
            Self::ARGB => "SDL_PACKEDORDER_ARGB",
            Self::RGBA => "SDL_PACKEDORDER_RGBA",
            Self::XBGR => "SDL_PACKEDORDER_XBGR",
            Self::BGRX => "SDL_PACKEDORDER_BGRX",
            Self::ABGR => "SDL_PACKEDORDER_ABGR",
            Self::BGRA => "SDL_PACKEDORDER_BGRA",

            _ => return write!(f, "SDL_PackedOrder({})", self.0),
        })
    }
}

impl SDL_PackedOrder {
    pub const NONE: Self = Self(0);
    pub const XRGB: Self = Self(1);
    pub const RGBX: Self = Self(2);
    pub const ARGB: Self = Self(3);
    pub const RGBA: Self = Self(4);
    pub const XBGR: Self = Self(5);
    pub const BGRX: Self = Self(6);
    pub const ABGR: Self = Self(7);
    pub const BGRA: Self = Self(8);
}

pub const SDL_PACKEDORDER_NONE: SDL_PackedOrder = SDL_PackedOrder::NONE;
pub const SDL_PACKEDORDER_XRGB: SDL_PackedOrder = SDL_PackedOrder::XRGB;
pub const SDL_PACKEDORDER_RGBX: SDL_PackedOrder = SDL_PackedOrder::RGBX;
pub const SDL_PACKEDORDER_ARGB: SDL_PackedOrder = SDL_PackedOrder::ARGB;
pub const SDL_PACKEDORDER_RGBA: SDL_PackedOrder = SDL_PackedOrder::RGBA;
pub const SDL_PACKEDORDER_XBGR: SDL_PackedOrder = SDL_PackedOrder::XBGR;
pub const SDL_PACKEDORDER_BGRX: SDL_PackedOrder = SDL_PackedOrder::BGRX;
pub const SDL_PACKEDORDER_ABGR: SDL_PackedOrder = SDL_PackedOrder::ABGR;
pub const SDL_PACKEDORDER_BGRA: SDL_PackedOrder = SDL_PackedOrder::BGRA;

/// Array component order, low byte -> high byte.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_ArrayOrder::NONE) | [`SDL_ARRAYORDER_NONE`] | |
/// | [`RGB`](SDL_ArrayOrder::RGB) | [`SDL_ARRAYORDER_RGB`] | |
/// | [`RGBA`](SDL_ArrayOrder::RGBA) | [`SDL_ARRAYORDER_RGBA`] | |
/// | [`ARGB`](SDL_ArrayOrder::ARGB) | [`SDL_ARRAYORDER_ARGB`] | |
/// | [`BGR`](SDL_ArrayOrder::BGR) | [`SDL_ARRAYORDER_BGR`] | |
/// | [`BGRA`](SDL_ArrayOrder::BGRA) | [`SDL_ARRAYORDER_BGRA`] | |
/// | [`ABGR`](SDL_ArrayOrder::ABGR) | [`SDL_ARRAYORDER_ABGR`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ArrayOrder(pub ::core::ffi::c_int);

impl From<SDL_ArrayOrder> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_ArrayOrder) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ArrayOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_ARRAYORDER_NONE",
            Self::RGB => "SDL_ARRAYORDER_RGB",
            Self::RGBA => "SDL_ARRAYORDER_RGBA",
            Self::ARGB => "SDL_ARRAYORDER_ARGB",
            Self::BGR => "SDL_ARRAYORDER_BGR",
            Self::BGRA => "SDL_ARRAYORDER_BGRA",
            Self::ABGR => "SDL_ARRAYORDER_ABGR",

            _ => return write!(f, "SDL_ArrayOrder({})", self.0),
        })
    }
}

impl SDL_ArrayOrder {
    pub const NONE: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const RGBA: Self = Self(2);
    pub const ARGB: Self = Self(3);
    pub const BGR: Self = Self(4);
    pub const BGRA: Self = Self(5);
    pub const ABGR: Self = Self(6);
}

pub const SDL_ARRAYORDER_NONE: SDL_ArrayOrder = SDL_ArrayOrder::NONE;
pub const SDL_ARRAYORDER_RGB: SDL_ArrayOrder = SDL_ArrayOrder::RGB;
pub const SDL_ARRAYORDER_RGBA: SDL_ArrayOrder = SDL_ArrayOrder::RGBA;
pub const SDL_ARRAYORDER_ARGB: SDL_ArrayOrder = SDL_ArrayOrder::ARGB;
pub const SDL_ARRAYORDER_BGR: SDL_ArrayOrder = SDL_ArrayOrder::BGR;
pub const SDL_ARRAYORDER_BGRA: SDL_ArrayOrder = SDL_ArrayOrder::BGRA;
pub const SDL_ARRAYORDER_ABGR: SDL_ArrayOrder = SDL_ArrayOrder::ABGR;

/// Packed component layout.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_PackedLayout::NONE) | [`SDL_PACKEDLAYOUT_NONE`] | |
/// | [`_332`](SDL_PackedLayout::_332) | [`SDL_PACKEDLAYOUT_332`] | |
/// | [`_4444`](SDL_PackedLayout::_4444) | [`SDL_PACKEDLAYOUT_4444`] | |
/// | [`_1555`](SDL_PackedLayout::_1555) | [`SDL_PACKEDLAYOUT_1555`] | |
/// | [`_5551`](SDL_PackedLayout::_5551) | [`SDL_PACKEDLAYOUT_5551`] | |
/// | [`_565`](SDL_PackedLayout::_565) | [`SDL_PACKEDLAYOUT_565`] | |
/// | [`_8888`](SDL_PackedLayout::_8888) | [`SDL_PACKEDLAYOUT_8888`] | |
/// | [`_2101010`](SDL_PackedLayout::_2101010) | [`SDL_PACKEDLAYOUT_2101010`] | |
/// | [`_1010102`](SDL_PackedLayout::_1010102) | [`SDL_PACKEDLAYOUT_1010102`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PackedLayout(pub ::core::ffi::c_int);

impl From<SDL_PackedLayout> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PackedLayout) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PackedLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_PACKEDLAYOUT_NONE",
            Self::_332 => "SDL_PACKEDLAYOUT_332",
            Self::_4444 => "SDL_PACKEDLAYOUT_4444",
            Self::_1555 => "SDL_PACKEDLAYOUT_1555",
            Self::_5551 => "SDL_PACKEDLAYOUT_5551",
            Self::_565 => "SDL_PACKEDLAYOUT_565",
            Self::_8888 => "SDL_PACKEDLAYOUT_8888",
            Self::_2101010 => "SDL_PACKEDLAYOUT_2101010",
            Self::_1010102 => "SDL_PACKEDLAYOUT_1010102",

            _ => return write!(f, "SDL_PackedLayout({})", self.0),
        })
    }
}

impl SDL_PackedLayout {
    pub const NONE: Self = Self(0);
    pub const _332: Self = Self(1);
    pub const _4444: Self = Self(2);
    pub const _1555: Self = Self(3);
    pub const _5551: Self = Self(4);
    pub const _565: Self = Self(5);
    pub const _8888: Self = Self(6);
    pub const _2101010: Self = Self(7);
    pub const _1010102: Self = Self(8);
}

pub const SDL_PACKEDLAYOUT_NONE: SDL_PackedLayout = SDL_PackedLayout::NONE;
pub const SDL_PACKEDLAYOUT_332: SDL_PackedLayout = SDL_PackedLayout::_332;
pub const SDL_PACKEDLAYOUT_4444: SDL_PackedLayout = SDL_PackedLayout::_4444;
pub const SDL_PACKEDLAYOUT_1555: SDL_PackedLayout = SDL_PackedLayout::_1555;
pub const SDL_PACKEDLAYOUT_5551: SDL_PackedLayout = SDL_PackedLayout::_5551;
pub const SDL_PACKEDLAYOUT_565: SDL_PackedLayout = SDL_PackedLayout::_565;
pub const SDL_PACKEDLAYOUT_8888: SDL_PackedLayout = SDL_PackedLayout::_8888;
pub const SDL_PACKEDLAYOUT_2101010: SDL_PackedLayout = SDL_PackedLayout::_2101010;
pub const SDL_PACKEDLAYOUT_1010102: SDL_PackedLayout = SDL_PackedLayout::_1010102;

#[inline(always)]
pub const fn SDL_DEFINE_PIXELFOURCC(A: Uint8, B: Uint8, C: Uint8, D: Uint8) -> Uint32 {
    SDL_FOURCC(A, B, C, D)
}

/// Pixel format.
///
/// SDL's pixel formats have the following naming convention:
///
/// - Names with a list of components and a single bit count, such as RGB24 and
///   ABGR32, define a platform-independent encoding into bytes in the order
///   specified. For example, in RGB24 data, each pixel is encoded in 3 bytes
///   (red, green, blue) in that order, and in ABGR32 data, each pixel is
///   encoded in 4 bytes alpha, blue, green, red) in that order. Use these
///   names if the property of a format that is important to you is the order
///   of the bytes in memory or on disk.
/// - Names with a bit count per component, such as ARGB8888 and XRGB1555, are
///   "packed" into an appropriately-sized integer in the platform's native
///   endianness. For example, ARGB8888 is a sequence of 32-bit integers; in
///   each integer, the most significant bits are alpha, and the least
///   significant bits are blue. On a little-endian CPU such as x86, the least
///   significant bits of each integer are arranged first in memory, but on a
///   big-endian CPU such as s390x, the most significant bits are arranged
///   first. Use these names if the property of a format that is important to
///   you is the meaning of each bit position within a native-endianness
///   integer.
/// - In indexed formats such as INDEX4LSB, each pixel is represented by
///   encoding an index into the palette into the indicated number of bits,
///   with multiple pixels packed into each byte if appropriate. In LSB
///   formats, the first (leftmost) pixel is stored in the least-significant
///   bits of the byte; in MSB formats, it's stored in the most-significant
///   bits. INDEX8 does not need LSB/MSB variants, because each pixel exactly
///   fills one byte.
///
/// The 32-bit byte-array encodings such as RGBA32 are aliases for the
/// appropriate 8888 encoding for the current platform. For example, RGBA32 is
/// an alias for ABGR8888 on little-endian CPUs like x86, or an alias for
/// RGBA8888 on big-endian CPUs.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_PixelFormat::UNKNOWN) | [`SDL_PIXELFORMAT_UNKNOWN`] | |
/// | [`INDEX1LSB`](SDL_PixelFormat::INDEX1LSB) | [`SDL_PIXELFORMAT_INDEX1LSB`] | |
/// | [`INDEX1MSB`](SDL_PixelFormat::INDEX1MSB) | [`SDL_PIXELFORMAT_INDEX1MSB`] | |
/// | [`INDEX2LSB`](SDL_PixelFormat::INDEX2LSB) | [`SDL_PIXELFORMAT_INDEX2LSB`] | |
/// | [`INDEX2MSB`](SDL_PixelFormat::INDEX2MSB) | [`SDL_PIXELFORMAT_INDEX2MSB`] | |
/// | [`INDEX4LSB`](SDL_PixelFormat::INDEX4LSB) | [`SDL_PIXELFORMAT_INDEX4LSB`] | |
/// | [`INDEX4MSB`](SDL_PixelFormat::INDEX4MSB) | [`SDL_PIXELFORMAT_INDEX4MSB`] | |
/// | [`INDEX8`](SDL_PixelFormat::INDEX8) | [`SDL_PIXELFORMAT_INDEX8`] | |
/// | [`RGB332`](SDL_PixelFormat::RGB332) | [`SDL_PIXELFORMAT_RGB332`] | |
/// | [`XRGB4444`](SDL_PixelFormat::XRGB4444) | [`SDL_PIXELFORMAT_XRGB4444`] | |
/// | [`XBGR4444`](SDL_PixelFormat::XBGR4444) | [`SDL_PIXELFORMAT_XBGR4444`] | |
/// | [`XRGB1555`](SDL_PixelFormat::XRGB1555) | [`SDL_PIXELFORMAT_XRGB1555`] | |
/// | [`XBGR1555`](SDL_PixelFormat::XBGR1555) | [`SDL_PIXELFORMAT_XBGR1555`] | |
/// | [`ARGB4444`](SDL_PixelFormat::ARGB4444) | [`SDL_PIXELFORMAT_ARGB4444`] | |
/// | [`RGBA4444`](SDL_PixelFormat::RGBA4444) | [`SDL_PIXELFORMAT_RGBA4444`] | |
/// | [`ABGR4444`](SDL_PixelFormat::ABGR4444) | [`SDL_PIXELFORMAT_ABGR4444`] | |
/// | [`BGRA4444`](SDL_PixelFormat::BGRA4444) | [`SDL_PIXELFORMAT_BGRA4444`] | |
/// | [`ARGB1555`](SDL_PixelFormat::ARGB1555) | [`SDL_PIXELFORMAT_ARGB1555`] | |
/// | [`RGBA5551`](SDL_PixelFormat::RGBA5551) | [`SDL_PIXELFORMAT_RGBA5551`] | |
/// | [`ABGR1555`](SDL_PixelFormat::ABGR1555) | [`SDL_PIXELFORMAT_ABGR1555`] | |
/// | [`BGRA5551`](SDL_PixelFormat::BGRA5551) | [`SDL_PIXELFORMAT_BGRA5551`] | |
/// | [`RGB565`](SDL_PixelFormat::RGB565) | [`SDL_PIXELFORMAT_RGB565`] | |
/// | [`BGR565`](SDL_PixelFormat::BGR565) | [`SDL_PIXELFORMAT_BGR565`] | |
/// | [`RGB24`](SDL_PixelFormat::RGB24) | [`SDL_PIXELFORMAT_RGB24`] | |
/// | [`BGR24`](SDL_PixelFormat::BGR24) | [`SDL_PIXELFORMAT_BGR24`] | |
/// | [`XRGB8888`](SDL_PixelFormat::XRGB8888) | [`SDL_PIXELFORMAT_XRGB8888`] | |
/// | [`RGBX8888`](SDL_PixelFormat::RGBX8888) | [`SDL_PIXELFORMAT_RGBX8888`] | |
/// | [`XBGR8888`](SDL_PixelFormat::XBGR8888) | [`SDL_PIXELFORMAT_XBGR8888`] | |
/// | [`BGRX8888`](SDL_PixelFormat::BGRX8888) | [`SDL_PIXELFORMAT_BGRX8888`] | |
/// | [`ARGB8888`](SDL_PixelFormat::ARGB8888) | [`SDL_PIXELFORMAT_ARGB8888`] | |
/// | [`RGBA8888`](SDL_PixelFormat::RGBA8888) | [`SDL_PIXELFORMAT_RGBA8888`] | |
/// | [`ABGR8888`](SDL_PixelFormat::ABGR8888) | [`SDL_PIXELFORMAT_ABGR8888`] | |
/// | [`BGRA8888`](SDL_PixelFormat::BGRA8888) | [`SDL_PIXELFORMAT_BGRA8888`] | |
/// | [`XRGB2101010`](SDL_PixelFormat::XRGB2101010) | [`SDL_PIXELFORMAT_XRGB2101010`] | |
/// | [`XBGR2101010`](SDL_PixelFormat::XBGR2101010) | [`SDL_PIXELFORMAT_XBGR2101010`] | |
/// | [`ARGB2101010`](SDL_PixelFormat::ARGB2101010) | [`SDL_PIXELFORMAT_ARGB2101010`] | |
/// | [`ABGR2101010`](SDL_PixelFormat::ABGR2101010) | [`SDL_PIXELFORMAT_ABGR2101010`] | |
/// | [`RGB48`](SDL_PixelFormat::RGB48) | [`SDL_PIXELFORMAT_RGB48`] | |
/// | [`BGR48`](SDL_PixelFormat::BGR48) | [`SDL_PIXELFORMAT_BGR48`] | |
/// | [`RGBA64`](SDL_PixelFormat::RGBA64) | [`SDL_PIXELFORMAT_RGBA64`] | |
/// | [`ARGB64`](SDL_PixelFormat::ARGB64) | [`SDL_PIXELFORMAT_ARGB64`] | |
/// | [`BGRA64`](SDL_PixelFormat::BGRA64) | [`SDL_PIXELFORMAT_BGRA64`] | |
/// | [`ABGR64`](SDL_PixelFormat::ABGR64) | [`SDL_PIXELFORMAT_ABGR64`] | |
/// | [`RGB48_FLOAT`](SDL_PixelFormat::RGB48_FLOAT) | [`SDL_PIXELFORMAT_RGB48_FLOAT`] | |
/// | [`BGR48_FLOAT`](SDL_PixelFormat::BGR48_FLOAT) | [`SDL_PIXELFORMAT_BGR48_FLOAT`] | |
/// | [`RGBA64_FLOAT`](SDL_PixelFormat::RGBA64_FLOAT) | [`SDL_PIXELFORMAT_RGBA64_FLOAT`] | |
/// | [`ARGB64_FLOAT`](SDL_PixelFormat::ARGB64_FLOAT) | [`SDL_PIXELFORMAT_ARGB64_FLOAT`] | |
/// | [`BGRA64_FLOAT`](SDL_PixelFormat::BGRA64_FLOAT) | [`SDL_PIXELFORMAT_BGRA64_FLOAT`] | |
/// | [`ABGR64_FLOAT`](SDL_PixelFormat::ABGR64_FLOAT) | [`SDL_PIXELFORMAT_ABGR64_FLOAT`] | |
/// | [`RGB96_FLOAT`](SDL_PixelFormat::RGB96_FLOAT) | [`SDL_PIXELFORMAT_RGB96_FLOAT`] | |
/// | [`BGR96_FLOAT`](SDL_PixelFormat::BGR96_FLOAT) | [`SDL_PIXELFORMAT_BGR96_FLOAT`] | |
/// | [`RGBA128_FLOAT`](SDL_PixelFormat::RGBA128_FLOAT) | [`SDL_PIXELFORMAT_RGBA128_FLOAT`] | |
/// | [`ARGB128_FLOAT`](SDL_PixelFormat::ARGB128_FLOAT) | [`SDL_PIXELFORMAT_ARGB128_FLOAT`] | |
/// | [`BGRA128_FLOAT`](SDL_PixelFormat::BGRA128_FLOAT) | [`SDL_PIXELFORMAT_BGRA128_FLOAT`] | |
/// | [`ABGR128_FLOAT`](SDL_PixelFormat::ABGR128_FLOAT) | [`SDL_PIXELFORMAT_ABGR128_FLOAT`] | |
/// | [`YV12`](SDL_PixelFormat::YV12) | [`SDL_PIXELFORMAT_YV12`] | Planar mode: Y + V + U  (3 planes) |
/// | [`IYUV`](SDL_PixelFormat::IYUV) | [`SDL_PIXELFORMAT_IYUV`] | Planar mode: Y + U + V  (3 planes) |
/// | [`YUY2`](SDL_PixelFormat::YUY2) | [`SDL_PIXELFORMAT_YUY2`] | Packed mode: Y0+U0+Y1+V0 (1 plane) |
/// | [`UYVY`](SDL_PixelFormat::UYVY) | [`SDL_PIXELFORMAT_UYVY`] | Packed mode: U0+Y0+V0+Y1 (1 plane) |
/// | [`YVYU`](SDL_PixelFormat::YVYU) | [`SDL_PIXELFORMAT_YVYU`] | Packed mode: Y0+V0+Y1+U0 (1 plane) |
/// | [`NV12`](SDL_PixelFormat::NV12) | [`SDL_PIXELFORMAT_NV12`] | Planar mode: Y + U/V interleaved  (2 planes) |
/// | [`NV21`](SDL_PixelFormat::NV21) | [`SDL_PIXELFORMAT_NV21`] | Planar mode: Y + V/U interleaved  (2 planes) |
/// | [`P010`](SDL_PixelFormat::P010) | [`SDL_PIXELFORMAT_P010`] | Planar mode: Y + U/V interleaved  (2 planes) |
/// | [`EXTERNAL_OES`](SDL_PixelFormat::EXTERNAL_OES) | [`SDL_PIXELFORMAT_EXTERNAL_OES`] | Android video texture format |
/// | [`RGBA32`](SDL_PixelFormat::RGBA32) | [`SDL_PIXELFORMAT_RGBA32`] | (target dependent) |
/// | [`ARGB32`](SDL_PixelFormat::ARGB32) | [`SDL_PIXELFORMAT_ARGB32`] | (target dependent) |
/// | [`BGRA32`](SDL_PixelFormat::BGRA32) | [`SDL_PIXELFORMAT_BGRA32`] | (target dependent) |
/// | [`ABGR32`](SDL_PixelFormat::ABGR32) | [`SDL_PIXELFORMAT_ABGR32`] | (target dependent) |
/// | [`RGBX32`](SDL_PixelFormat::RGBX32) | [`SDL_PIXELFORMAT_RGBX32`] | (target dependent) |
/// | [`XRGB32`](SDL_PixelFormat::XRGB32) | [`SDL_PIXELFORMAT_XRGB32`] | (target dependent) |
/// | [`BGRX32`](SDL_PixelFormat::BGRX32) | [`SDL_PIXELFORMAT_BGRX32`] | (target dependent) |
/// | [`XBGR32`](SDL_PixelFormat::XBGR32) | [`SDL_PIXELFORMAT_XBGR32`] | (target dependent) |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PixelFormat(pub ::core::ffi::c_int);

impl From<SDL_PixelFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PixelFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_PIXELFORMAT_UNKNOWN",
            Self::INDEX1LSB => "SDL_PIXELFORMAT_INDEX1LSB",
            Self::INDEX1MSB => "SDL_PIXELFORMAT_INDEX1MSB",
            Self::INDEX2LSB => "SDL_PIXELFORMAT_INDEX2LSB",
            Self::INDEX2MSB => "SDL_PIXELFORMAT_INDEX2MSB",
            Self::INDEX4LSB => "SDL_PIXELFORMAT_INDEX4LSB",
            Self::INDEX4MSB => "SDL_PIXELFORMAT_INDEX4MSB",
            Self::INDEX8 => "SDL_PIXELFORMAT_INDEX8",
            Self::RGB332 => "SDL_PIXELFORMAT_RGB332",
            Self::XRGB4444 => "SDL_PIXELFORMAT_XRGB4444",
            Self::XBGR4444 => "SDL_PIXELFORMAT_XBGR4444",
            Self::XRGB1555 => "SDL_PIXELFORMAT_XRGB1555",
            Self::XBGR1555 => "SDL_PIXELFORMAT_XBGR1555",
            Self::ARGB4444 => "SDL_PIXELFORMAT_ARGB4444",
            Self::RGBA4444 => "SDL_PIXELFORMAT_RGBA4444",
            Self::ABGR4444 => "SDL_PIXELFORMAT_ABGR4444",
            Self::BGRA4444 => "SDL_PIXELFORMAT_BGRA4444",
            Self::ARGB1555 => "SDL_PIXELFORMAT_ARGB1555",
            Self::RGBA5551 => "SDL_PIXELFORMAT_RGBA5551",
            Self::ABGR1555 => "SDL_PIXELFORMAT_ABGR1555",
            Self::BGRA5551 => "SDL_PIXELFORMAT_BGRA5551",
            Self::RGB565 => "SDL_PIXELFORMAT_RGB565",
            Self::BGR565 => "SDL_PIXELFORMAT_BGR565",
            Self::RGB24 => "SDL_PIXELFORMAT_RGB24",
            Self::BGR24 => "SDL_PIXELFORMAT_BGR24",
            Self::XRGB8888 => "SDL_PIXELFORMAT_XRGB8888",
            Self::RGBX8888 => "SDL_PIXELFORMAT_RGBX8888",
            Self::XBGR8888 => "SDL_PIXELFORMAT_XBGR8888",
            Self::BGRX8888 => "SDL_PIXELFORMAT_BGRX8888",
            Self::ARGB8888 => "SDL_PIXELFORMAT_ARGB8888",
            Self::RGBA8888 => "SDL_PIXELFORMAT_RGBA8888",
            Self::ABGR8888 => "SDL_PIXELFORMAT_ABGR8888",
            Self::BGRA8888 => "SDL_PIXELFORMAT_BGRA8888",
            Self::XRGB2101010 => "SDL_PIXELFORMAT_XRGB2101010",
            Self::XBGR2101010 => "SDL_PIXELFORMAT_XBGR2101010",
            Self::ARGB2101010 => "SDL_PIXELFORMAT_ARGB2101010",
            Self::ABGR2101010 => "SDL_PIXELFORMAT_ABGR2101010",
            Self::RGB48 => "SDL_PIXELFORMAT_RGB48",
            Self::BGR48 => "SDL_PIXELFORMAT_BGR48",
            Self::RGBA64 => "SDL_PIXELFORMAT_RGBA64",
            Self::ARGB64 => "SDL_PIXELFORMAT_ARGB64",
            Self::BGRA64 => "SDL_PIXELFORMAT_BGRA64",
            Self::ABGR64 => "SDL_PIXELFORMAT_ABGR64",
            Self::RGB48_FLOAT => "SDL_PIXELFORMAT_RGB48_FLOAT",
            Self::BGR48_FLOAT => "SDL_PIXELFORMAT_BGR48_FLOAT",
            Self::RGBA64_FLOAT => "SDL_PIXELFORMAT_RGBA64_FLOAT",
            Self::ARGB64_FLOAT => "SDL_PIXELFORMAT_ARGB64_FLOAT",
            Self::BGRA64_FLOAT => "SDL_PIXELFORMAT_BGRA64_FLOAT",
            Self::ABGR64_FLOAT => "SDL_PIXELFORMAT_ABGR64_FLOAT",
            Self::RGB96_FLOAT => "SDL_PIXELFORMAT_RGB96_FLOAT",
            Self::BGR96_FLOAT => "SDL_PIXELFORMAT_BGR96_FLOAT",
            Self::RGBA128_FLOAT => "SDL_PIXELFORMAT_RGBA128_FLOAT",
            Self::ARGB128_FLOAT => "SDL_PIXELFORMAT_ARGB128_FLOAT",
            Self::BGRA128_FLOAT => "SDL_PIXELFORMAT_BGRA128_FLOAT",
            Self::ABGR128_FLOAT => "SDL_PIXELFORMAT_ABGR128_FLOAT",
            Self::YV12 => "SDL_PIXELFORMAT_YV12",
            Self::IYUV => "SDL_PIXELFORMAT_IYUV",
            Self::YUY2 => "SDL_PIXELFORMAT_YUY2",
            Self::UYVY => "SDL_PIXELFORMAT_UYVY",
            Self::YVYU => "SDL_PIXELFORMAT_YVYU",
            Self::NV12 => "SDL_PIXELFORMAT_NV12",
            Self::NV21 => "SDL_PIXELFORMAT_NV21",
            Self::P010 => "SDL_PIXELFORMAT_P010",
            Self::EXTERNAL_OES => "SDL_PIXELFORMAT_EXTERNAL_OES",
            Self::RGBA32 => "SDL_PIXELFORMAT_RGBA32",
            Self::ARGB32 => "SDL_PIXELFORMAT_ARGB32",
            Self::BGRA32 => "SDL_PIXELFORMAT_BGRA32",
            Self::ABGR32 => "SDL_PIXELFORMAT_ABGR32",
            Self::RGBX32 => "SDL_PIXELFORMAT_RGBX32",
            Self::XRGB32 => "SDL_PIXELFORMAT_XRGB32",
            Self::BGRX32 => "SDL_PIXELFORMAT_BGRX32",
            Self::XBGR32 => "SDL_PIXELFORMAT_XBGR32",
            Self::RGBA32 => "SDL_PIXELFORMAT_RGBA32",
            Self::ARGB32 => "SDL_PIXELFORMAT_ARGB32",
            Self::BGRA32 => "SDL_PIXELFORMAT_BGRA32",
            Self::ABGR32 => "SDL_PIXELFORMAT_ABGR32",
            Self::RGBX32 => "SDL_PIXELFORMAT_RGBX32",
            Self::XRGB32 => "SDL_PIXELFORMAT_XRGB32",
            Self::BGRX32 => "SDL_PIXELFORMAT_BGRX32",
            Self::XBGR32 => "SDL_PIXELFORMAT_XBGR32",

            _ => return write!(f, "SDL_PixelFormat({})", self.0),
        })
    }
}

impl SDL_PixelFormat {
    pub const UNKNOWN: Self = Self(0);
    pub const INDEX1LSB: Self = Self(0x11100100);
    pub const INDEX1MSB: Self = Self(0x11200100);
    pub const INDEX2LSB: Self = Self(0x1c100200);
    pub const INDEX2MSB: Self = Self(0x1c200200);
    pub const INDEX4LSB: Self = Self(0x12100400);
    pub const INDEX4MSB: Self = Self(0x12200400);
    pub const INDEX8: Self = Self(0x13000801);
    pub const RGB332: Self = Self(0x14110801);
    pub const XRGB4444: Self = Self(0x15120c02);
    pub const XBGR4444: Self = Self(0x15520c02);
    pub const XRGB1555: Self = Self(0x15130f02);
    pub const XBGR1555: Self = Self(0x15530f02);
    pub const ARGB4444: Self = Self(0x15321002);
    pub const RGBA4444: Self = Self(0x15421002);
    pub const ABGR4444: Self = Self(0x15721002);
    pub const BGRA4444: Self = Self(0x15821002);
    pub const ARGB1555: Self = Self(0x15331002);
    pub const RGBA5551: Self = Self(0x15441002);
    pub const ABGR1555: Self = Self(0x15731002);
    pub const BGRA5551: Self = Self(0x15841002);
    pub const RGB565: Self = Self(0x15151002);
    pub const BGR565: Self = Self(0x15551002);
    pub const RGB24: Self = Self(0x17101803);
    pub const BGR24: Self = Self(0x17401803);
    pub const XRGB8888: Self = Self(0x16161804);
    pub const RGBX8888: Self = Self(0x16261804);
    pub const XBGR8888: Self = Self(0x16561804);
    pub const BGRX8888: Self = Self(0x16661804);
    pub const ARGB8888: Self = Self(0x16362004);
    pub const RGBA8888: Self = Self(0x16462004);
    pub const ABGR8888: Self = Self(0x16762004);
    pub const BGRA8888: Self = Self(0x16862004);
    pub const XRGB2101010: Self = Self(0x16172004);
    pub const XBGR2101010: Self = Self(0x16572004);
    pub const ARGB2101010: Self = Self(0x16372004);
    pub const ABGR2101010: Self = Self(0x16772004);
    pub const RGB48: Self = Self(0x18103006);
    pub const BGR48: Self = Self(0x18403006);
    pub const RGBA64: Self = Self(0x18204008);
    pub const ARGB64: Self = Self(0x18304008);
    pub const BGRA64: Self = Self(0x18504008);
    pub const ABGR64: Self = Self(0x18604008);
    pub const RGB48_FLOAT: Self = Self(0x1a103006);
    pub const BGR48_FLOAT: Self = Self(0x1a403006);
    pub const RGBA64_FLOAT: Self = Self(0x1a204008);
    pub const ARGB64_FLOAT: Self = Self(0x1a304008);
    pub const BGRA64_FLOAT: Self = Self(0x1a504008);
    pub const ABGR64_FLOAT: Self = Self(0x1a604008);
    pub const RGB96_FLOAT: Self = Self(0x1b10600c);
    pub const BGR96_FLOAT: Self = Self(0x1b40600c);
    pub const RGBA128_FLOAT: Self = Self(0x1b208010);
    pub const ARGB128_FLOAT: Self = Self(0x1b308010);
    pub const BGRA128_FLOAT: Self = Self(0x1b508010);
    pub const ABGR128_FLOAT: Self = Self(0x1b608010);
    /// Planar mode: Y + V + U  (3 planes)
    pub const YV12: Self = Self(0x32315659);
    /// Planar mode: Y + U + V  (3 planes)
    pub const IYUV: Self = Self(0x56555949);
    /// Packed mode: Y0+U0+Y1+V0 (1 plane)
    pub const YUY2: Self = Self(0x32595559);
    /// Packed mode: U0+Y0+V0+Y1 (1 plane)
    pub const UYVY: Self = Self(0x59565955);
    /// Packed mode: Y0+V0+Y1+U0 (1 plane)
    pub const YVYU: Self = Self(0x55595659);
    /// Planar mode: Y + U/V interleaved  (2 planes)
    pub const NV12: Self = Self(0x3231564e);
    /// Planar mode: Y + V/U interleaved  (2 planes)
    pub const NV21: Self = Self(0x3132564e);
    /// Planar mode: Y + U/V interleaved  (2 planes)
    pub const P010: Self = Self(0x30313050);
    /// Android video texture format
    pub const EXTERNAL_OES: Self = Self(0x2053454f);
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const RGBA32: Self = SDL_PIXELFORMAT_RGBA8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const ARGB32: Self = SDL_PIXELFORMAT_ARGB8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const BGRA32: Self = SDL_PIXELFORMAT_BGRA8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const ABGR32: Self = SDL_PIXELFORMAT_ABGR8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const RGBX32: Self = SDL_PIXELFORMAT_RGBX8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const XRGB32: Self = SDL_PIXELFORMAT_XRGB8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const BGRX32: Self = SDL_PIXELFORMAT_BGRX8888;
    #[cfg(target_endian = "big")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const XBGR32: Self = SDL_PIXELFORMAT_XBGR8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const RGBA32: Self = SDL_PIXELFORMAT_ABGR8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const ARGB32: Self = SDL_PIXELFORMAT_BGRA8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const BGRA32: Self = SDL_PIXELFORMAT_ARGB8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const ABGR32: Self = SDL_PIXELFORMAT_RGBA8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const RGBX32: Self = SDL_PIXELFORMAT_XBGR8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const XRGB32: Self = SDL_PIXELFORMAT_BGRX8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const BGRX32: Self = SDL_PIXELFORMAT_XRGB8888;
    #[cfg(not(target_endian = "big"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub const XBGR32: Self = SDL_PIXELFORMAT_RGBX8888;
}

pub const SDL_PIXELFORMAT_UNKNOWN: SDL_PixelFormat = SDL_PixelFormat::UNKNOWN;
pub const SDL_PIXELFORMAT_INDEX1LSB: SDL_PixelFormat = SDL_PixelFormat::INDEX1LSB;
pub const SDL_PIXELFORMAT_INDEX1MSB: SDL_PixelFormat = SDL_PixelFormat::INDEX1MSB;
pub const SDL_PIXELFORMAT_INDEX2LSB: SDL_PixelFormat = SDL_PixelFormat::INDEX2LSB;
pub const SDL_PIXELFORMAT_INDEX2MSB: SDL_PixelFormat = SDL_PixelFormat::INDEX2MSB;
pub const SDL_PIXELFORMAT_INDEX4LSB: SDL_PixelFormat = SDL_PixelFormat::INDEX4LSB;
pub const SDL_PIXELFORMAT_INDEX4MSB: SDL_PixelFormat = SDL_PixelFormat::INDEX4MSB;
pub const SDL_PIXELFORMAT_INDEX8: SDL_PixelFormat = SDL_PixelFormat::INDEX8;
pub const SDL_PIXELFORMAT_RGB332: SDL_PixelFormat = SDL_PixelFormat::RGB332;
pub const SDL_PIXELFORMAT_XRGB4444: SDL_PixelFormat = SDL_PixelFormat::XRGB4444;
pub const SDL_PIXELFORMAT_XBGR4444: SDL_PixelFormat = SDL_PixelFormat::XBGR4444;
pub const SDL_PIXELFORMAT_XRGB1555: SDL_PixelFormat = SDL_PixelFormat::XRGB1555;
pub const SDL_PIXELFORMAT_XBGR1555: SDL_PixelFormat = SDL_PixelFormat::XBGR1555;
pub const SDL_PIXELFORMAT_ARGB4444: SDL_PixelFormat = SDL_PixelFormat::ARGB4444;
pub const SDL_PIXELFORMAT_RGBA4444: SDL_PixelFormat = SDL_PixelFormat::RGBA4444;
pub const SDL_PIXELFORMAT_ABGR4444: SDL_PixelFormat = SDL_PixelFormat::ABGR4444;
pub const SDL_PIXELFORMAT_BGRA4444: SDL_PixelFormat = SDL_PixelFormat::BGRA4444;
pub const SDL_PIXELFORMAT_ARGB1555: SDL_PixelFormat = SDL_PixelFormat::ARGB1555;
pub const SDL_PIXELFORMAT_RGBA5551: SDL_PixelFormat = SDL_PixelFormat::RGBA5551;
pub const SDL_PIXELFORMAT_ABGR1555: SDL_PixelFormat = SDL_PixelFormat::ABGR1555;
pub const SDL_PIXELFORMAT_BGRA5551: SDL_PixelFormat = SDL_PixelFormat::BGRA5551;
pub const SDL_PIXELFORMAT_RGB565: SDL_PixelFormat = SDL_PixelFormat::RGB565;
pub const SDL_PIXELFORMAT_BGR565: SDL_PixelFormat = SDL_PixelFormat::BGR565;
pub const SDL_PIXELFORMAT_RGB24: SDL_PixelFormat = SDL_PixelFormat::RGB24;
pub const SDL_PIXELFORMAT_BGR24: SDL_PixelFormat = SDL_PixelFormat::BGR24;
pub const SDL_PIXELFORMAT_XRGB8888: SDL_PixelFormat = SDL_PixelFormat::XRGB8888;
pub const SDL_PIXELFORMAT_RGBX8888: SDL_PixelFormat = SDL_PixelFormat::RGBX8888;
pub const SDL_PIXELFORMAT_XBGR8888: SDL_PixelFormat = SDL_PixelFormat::XBGR8888;
pub const SDL_PIXELFORMAT_BGRX8888: SDL_PixelFormat = SDL_PixelFormat::BGRX8888;
pub const SDL_PIXELFORMAT_ARGB8888: SDL_PixelFormat = SDL_PixelFormat::ARGB8888;
pub const SDL_PIXELFORMAT_RGBA8888: SDL_PixelFormat = SDL_PixelFormat::RGBA8888;
pub const SDL_PIXELFORMAT_ABGR8888: SDL_PixelFormat = SDL_PixelFormat::ABGR8888;
pub const SDL_PIXELFORMAT_BGRA8888: SDL_PixelFormat = SDL_PixelFormat::BGRA8888;
pub const SDL_PIXELFORMAT_XRGB2101010: SDL_PixelFormat = SDL_PixelFormat::XRGB2101010;
pub const SDL_PIXELFORMAT_XBGR2101010: SDL_PixelFormat = SDL_PixelFormat::XBGR2101010;
pub const SDL_PIXELFORMAT_ARGB2101010: SDL_PixelFormat = SDL_PixelFormat::ARGB2101010;
pub const SDL_PIXELFORMAT_ABGR2101010: SDL_PixelFormat = SDL_PixelFormat::ABGR2101010;
pub const SDL_PIXELFORMAT_RGB48: SDL_PixelFormat = SDL_PixelFormat::RGB48;
pub const SDL_PIXELFORMAT_BGR48: SDL_PixelFormat = SDL_PixelFormat::BGR48;
pub const SDL_PIXELFORMAT_RGBA64: SDL_PixelFormat = SDL_PixelFormat::RGBA64;
pub const SDL_PIXELFORMAT_ARGB64: SDL_PixelFormat = SDL_PixelFormat::ARGB64;
pub const SDL_PIXELFORMAT_BGRA64: SDL_PixelFormat = SDL_PixelFormat::BGRA64;
pub const SDL_PIXELFORMAT_ABGR64: SDL_PixelFormat = SDL_PixelFormat::ABGR64;
pub const SDL_PIXELFORMAT_RGB48_FLOAT: SDL_PixelFormat = SDL_PixelFormat::RGB48_FLOAT;
pub const SDL_PIXELFORMAT_BGR48_FLOAT: SDL_PixelFormat = SDL_PixelFormat::BGR48_FLOAT;
pub const SDL_PIXELFORMAT_RGBA64_FLOAT: SDL_PixelFormat = SDL_PixelFormat::RGBA64_FLOAT;
pub const SDL_PIXELFORMAT_ARGB64_FLOAT: SDL_PixelFormat = SDL_PixelFormat::ARGB64_FLOAT;
pub const SDL_PIXELFORMAT_BGRA64_FLOAT: SDL_PixelFormat = SDL_PixelFormat::BGRA64_FLOAT;
pub const SDL_PIXELFORMAT_ABGR64_FLOAT: SDL_PixelFormat = SDL_PixelFormat::ABGR64_FLOAT;
pub const SDL_PIXELFORMAT_RGB96_FLOAT: SDL_PixelFormat = SDL_PixelFormat::RGB96_FLOAT;
pub const SDL_PIXELFORMAT_BGR96_FLOAT: SDL_PixelFormat = SDL_PixelFormat::BGR96_FLOAT;
pub const SDL_PIXELFORMAT_RGBA128_FLOAT: SDL_PixelFormat = SDL_PixelFormat::RGBA128_FLOAT;
pub const SDL_PIXELFORMAT_ARGB128_FLOAT: SDL_PixelFormat = SDL_PixelFormat::ARGB128_FLOAT;
pub const SDL_PIXELFORMAT_BGRA128_FLOAT: SDL_PixelFormat = SDL_PixelFormat::BGRA128_FLOAT;
pub const SDL_PIXELFORMAT_ABGR128_FLOAT: SDL_PixelFormat = SDL_PixelFormat::ABGR128_FLOAT;
/// Planar mode: Y + V + U  (3 planes)
pub const SDL_PIXELFORMAT_YV12: SDL_PixelFormat = SDL_PixelFormat::YV12;
/// Planar mode: Y + U + V  (3 planes)
pub const SDL_PIXELFORMAT_IYUV: SDL_PixelFormat = SDL_PixelFormat::IYUV;
/// Packed mode: Y0+U0+Y1+V0 (1 plane)
pub const SDL_PIXELFORMAT_YUY2: SDL_PixelFormat = SDL_PixelFormat::YUY2;
/// Packed mode: U0+Y0+V0+Y1 (1 plane)
pub const SDL_PIXELFORMAT_UYVY: SDL_PixelFormat = SDL_PixelFormat::UYVY;
/// Packed mode: Y0+V0+Y1+U0 (1 plane)
pub const SDL_PIXELFORMAT_YVYU: SDL_PixelFormat = SDL_PixelFormat::YVYU;
/// Planar mode: Y + U/V interleaved  (2 planes)
pub const SDL_PIXELFORMAT_NV12: SDL_PixelFormat = SDL_PixelFormat::NV12;
/// Planar mode: Y + V/U interleaved  (2 planes)
pub const SDL_PIXELFORMAT_NV21: SDL_PixelFormat = SDL_PixelFormat::NV21;
/// Planar mode: Y + U/V interleaved  (2 planes)
pub const SDL_PIXELFORMAT_P010: SDL_PixelFormat = SDL_PixelFormat::P010;
/// Android video texture format
pub const SDL_PIXELFORMAT_EXTERNAL_OES: SDL_PixelFormat = SDL_PixelFormat::EXTERNAL_OES;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_RGBA32: SDL_PixelFormat = SDL_PixelFormat::RGBA32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_ARGB32: SDL_PixelFormat = SDL_PixelFormat::ARGB32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_BGRA32: SDL_PixelFormat = SDL_PixelFormat::BGRA32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_ABGR32: SDL_PixelFormat = SDL_PixelFormat::ABGR32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_RGBX32: SDL_PixelFormat = SDL_PixelFormat::RGBX32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_XRGB32: SDL_PixelFormat = SDL_PixelFormat::XRGB32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_BGRX32: SDL_PixelFormat = SDL_PixelFormat::BGRX32;
#[cfg(target_endian = "big")]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_XBGR32: SDL_PixelFormat = SDL_PixelFormat::XBGR32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_RGBA32: SDL_PixelFormat = SDL_PixelFormat::RGBA32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_ARGB32: SDL_PixelFormat = SDL_PixelFormat::ARGB32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_BGRA32: SDL_PixelFormat = SDL_PixelFormat::BGRA32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_ABGR32: SDL_PixelFormat = SDL_PixelFormat::ABGR32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_RGBX32: SDL_PixelFormat = SDL_PixelFormat::RGBX32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_XRGB32: SDL_PixelFormat = SDL_PixelFormat::XRGB32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_BGRX32: SDL_PixelFormat = SDL_PixelFormat::BGRX32;
#[cfg(not(target_endian = "big"))]
#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
pub const SDL_PIXELFORMAT_XBGR32: SDL_PixelFormat = SDL_PixelFormat::XBGR32;

#[inline(always)]
pub const fn SDL_DEFINE_PIXELFORMAT(
    r#type: SDL_PixelType,
    order: ::core::ffi::c_int,
    layout: SDL_PackedLayout,
    bits: ::core::primitive::u8,
    bytes: ::core::primitive::u8,
) -> SDL_PixelFormat {
    SDL_PixelFormat(
        (((((268435456_i32 | (r#type.0 << 24)) | (order << 20)) | (layout.0 << 16))
            | ((bits as ::core::ffi::c_int) << 8))
            | ((bytes as ::core::ffi::c_int) << 0)),
    )
}

#[inline(always)]
pub const fn SDL_PIXELFLAG(X: SDL_PixelFormat) -> ::core::ffi::c_int {
    ((X.0 >> 28) & 15_i32)
}

#[inline(always)]
pub const fn SDL_PIXELTYPE(X: SDL_PixelFormat) -> SDL_PixelType {
    SDL_PixelType(((X.0 >> 24) & 15_i32))
}

#[inline(always)]
pub const fn SDL_PIXELORDER(X: SDL_PixelFormat) -> ::core::ffi::c_int {
    ((X.0 >> 20) & 15_i32)
}

#[inline(always)]
pub const fn SDL_PIXELLAYOUT(X: SDL_PixelFormat) -> SDL_PackedLayout {
    SDL_PackedLayout(((X.0 >> 16) & 15_i32))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_FOURCC(format: SDL_PixelFormat) -> ::core::primitive::bool {
    ((format.0 != 0) && (SDL_PIXELFLAG(format) != 1_i32))
}

#[inline(always)]
pub const fn SDL_BITSPERPIXEL(X: SDL_PixelFormat) -> ::core::primitive::u8 {
    ((if SDL_ISPIXELFORMAT_FOURCC(X) {
        0_i32
    } else {
        ((X.0 >> 8) & 255_i32)
    }) as ::core::primitive::u8)
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_INDEXED(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (!(SDL_ISPIXELFORMAT_FOURCC(format))
        && ((((SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_INDEX1.0)
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_INDEX2.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_INDEX4.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_INDEX8.0)))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_PACKED(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (!(SDL_ISPIXELFORMAT_FOURCC(format))
        && (((SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_PACKED8.0)
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_PACKED16.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_PACKED32.0)))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_ARRAY(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (!(SDL_ISPIXELFORMAT_FOURCC(format))
        && (((((SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYU8.0)
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYU16.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYU32.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYF16.0))
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYF32.0)))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_FLOAT(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (!(SDL_ISPIXELFORMAT_FOURCC(format))
        && ((SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYF16.0)
            || (SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_ARRAYF32.0)))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_ALPHA(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (SDL_ISPIXELFORMAT_PACKED(format)
        && ((((SDL_PIXELORDER(format) == SDL_PACKEDORDER_ARGB.0)
            || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_RGBA.0))
            || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_ABGR.0))
            || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_BGRA.0)))
}

#[inline(always)]
pub const fn SDL_ISPIXELFORMAT_10BIT(format: SDL_PixelFormat) -> ::core::primitive::bool {
    (!(SDL_ISPIXELFORMAT_FOURCC(format))
        && ((SDL_PIXELTYPE(format).0 == SDL_PIXELTYPE_PACKED32.0)
            && (SDL_PIXELLAYOUT(format).0 == SDL_PACKEDLAYOUT_2101010.0)))
}

#[inline(always)]
pub const fn SDL_BYTESPERPIXEL(X: SDL_PixelFormat) -> ::core::primitive::u8 {
    ((if SDL_ISPIXELFORMAT_FOURCC(X) {
        if ((((X.0 == SDL_PIXELFORMAT_YUY2.0) || (X.0 == SDL_PIXELFORMAT_UYVY.0))
            || (X.0 == SDL_PIXELFORMAT_YVYU.0))
            || (X.0 == SDL_PIXELFORMAT_P010.0))
        {
            2
        } else {
            1
        }
    } else {
        ((X.0 >> 0) & 255_i32)
    }) as ::core::primitive::u8)
}

/// Colorspace color type.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_ColorType::UNKNOWN) | [`SDL_COLOR_TYPE_UNKNOWN`] | |
/// | [`RGB`](SDL_ColorType::RGB) | [`SDL_COLOR_TYPE_RGB`] | |
/// | [`YCBCR`](SDL_ColorType::YCBCR) | [`SDL_COLOR_TYPE_YCBCR`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorType(pub ::core::ffi::c_uint);

impl From<SDL_ColorType> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_ColorType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ColorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_COLOR_TYPE_UNKNOWN",
            Self::RGB => "SDL_COLOR_TYPE_RGB",
            Self::YCBCR => "SDL_COLOR_TYPE_YCBCR",

            _ => return write!(f, "SDL_ColorType({})", self.0),
        })
    }
}

impl SDL_ColorType {
    pub const UNKNOWN: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const YCBCR: Self = Self(2);
}

pub const SDL_COLOR_TYPE_UNKNOWN: SDL_ColorType = SDL_ColorType::UNKNOWN;
pub const SDL_COLOR_TYPE_RGB: SDL_ColorType = SDL_ColorType::RGB;
pub const SDL_COLOR_TYPE_YCBCR: SDL_ColorType = SDL_ColorType::YCBCR;

/// Colorspace color range, as described by
/// <https://www.itu.int/rec/R-REC-BT.2100-2-201807-I/en>
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_ColorRange::UNKNOWN) | [`SDL_COLOR_RANGE_UNKNOWN`] | |
/// | [`LIMITED`](SDL_ColorRange::LIMITED) | [`SDL_COLOR_RANGE_LIMITED`] | Narrow range, e.g. 16-235 for 8-bit RGB and luma, and 16-240 for 8-bit chroma |
/// | [`FULL`](SDL_ColorRange::FULL) | [`SDL_COLOR_RANGE_FULL`] | Full range, e.g. 0-255 for 8-bit RGB and luma, and 1-255 for 8-bit chroma |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorRange(pub ::core::ffi::c_uint);

impl From<SDL_ColorRange> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_ColorRange) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ColorRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_COLOR_RANGE_UNKNOWN",
            Self::LIMITED => "SDL_COLOR_RANGE_LIMITED",
            Self::FULL => "SDL_COLOR_RANGE_FULL",

            _ => return write!(f, "SDL_ColorRange({})", self.0),
        })
    }
}

impl SDL_ColorRange {
    pub const UNKNOWN: Self = Self(0);
    /// Narrow range, e.g. 16-235 for 8-bit RGB and luma, and 16-240 for 8-bit chroma
    pub const LIMITED: Self = Self(1);
    /// Full range, e.g. 0-255 for 8-bit RGB and luma, and 1-255 for 8-bit chroma
    pub const FULL: Self = Self(2);
}

pub const SDL_COLOR_RANGE_UNKNOWN: SDL_ColorRange = SDL_ColorRange::UNKNOWN;
/// Narrow range, e.g. 16-235 for 8-bit RGB and luma, and 16-240 for 8-bit chroma
pub const SDL_COLOR_RANGE_LIMITED: SDL_ColorRange = SDL_ColorRange::LIMITED;
/// Full range, e.g. 0-255 for 8-bit RGB and luma, and 1-255 for 8-bit chroma
pub const SDL_COLOR_RANGE_FULL: SDL_ColorRange = SDL_ColorRange::FULL;

/// Colorspace color primaries, as described by
/// <https://www.itu.int/rec/T-REC-H.273-201612-S/en>
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_ColorPrimaries::UNKNOWN) | [`SDL_COLOR_PRIMARIES_UNKNOWN`] | |
/// | [`BT709`](SDL_ColorPrimaries::BT709) | [`SDL_COLOR_PRIMARIES_BT709`] | ITU-R BT.709-6 |
/// | [`UNSPECIFIED`](SDL_ColorPrimaries::UNSPECIFIED) | [`SDL_COLOR_PRIMARIES_UNSPECIFIED`] | |
/// | [`BT470M`](SDL_ColorPrimaries::BT470M) | [`SDL_COLOR_PRIMARIES_BT470M`] | ITU-R BT.470-6 System M |
/// | [`BT470BG`](SDL_ColorPrimaries::BT470BG) | [`SDL_COLOR_PRIMARIES_BT470BG`] | ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625 |
/// | [`BT601`](SDL_ColorPrimaries::BT601) | [`SDL_COLOR_PRIMARIES_BT601`] | ITU-R BT.601-7 525, SMPTE 170M |
/// | [`SMPTE240`](SDL_ColorPrimaries::SMPTE240) | [`SDL_COLOR_PRIMARIES_SMPTE240`] | SMPTE 240M, functionally the same as [`SDL_COLOR_PRIMARIES_BT601`] |
/// | [`GENERIC_FILM`](SDL_ColorPrimaries::GENERIC_FILM) | [`SDL_COLOR_PRIMARIES_GENERIC_FILM`] | Generic film (color filters using Illuminant C) |
/// | [`BT2020`](SDL_ColorPrimaries::BT2020) | [`SDL_COLOR_PRIMARIES_BT2020`] | ITU-R BT.2020-2 / ITU-R BT.2100-0 |
/// | [`XYZ`](SDL_ColorPrimaries::XYZ) | [`SDL_COLOR_PRIMARIES_XYZ`] | SMPTE ST 428-1 |
/// | [`SMPTE431`](SDL_ColorPrimaries::SMPTE431) | [`SDL_COLOR_PRIMARIES_SMPTE431`] | SMPTE RP 431-2 |
/// | [`SMPTE432`](SDL_ColorPrimaries::SMPTE432) | [`SDL_COLOR_PRIMARIES_SMPTE432`] | SMPTE EG 432-1 / DCI P3 |
/// | [`EBU3213`](SDL_ColorPrimaries::EBU3213) | [`SDL_COLOR_PRIMARIES_EBU3213`] | EBU Tech. 3213-E |
/// | [`CUSTOM`](SDL_ColorPrimaries::CUSTOM) | [`SDL_COLOR_PRIMARIES_CUSTOM`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorPrimaries(pub ::core::ffi::c_uint);

impl From<SDL_ColorPrimaries> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_ColorPrimaries) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ColorPrimaries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_COLOR_PRIMARIES_UNKNOWN",
            Self::BT709 => "SDL_COLOR_PRIMARIES_BT709",
            Self::UNSPECIFIED => "SDL_COLOR_PRIMARIES_UNSPECIFIED",
            Self::BT470M => "SDL_COLOR_PRIMARIES_BT470M",
            Self::BT470BG => "SDL_COLOR_PRIMARIES_BT470BG",
            Self::BT601 => "SDL_COLOR_PRIMARIES_BT601",
            Self::SMPTE240 => "SDL_COLOR_PRIMARIES_SMPTE240",
            Self::GENERIC_FILM => "SDL_COLOR_PRIMARIES_GENERIC_FILM",
            Self::BT2020 => "SDL_COLOR_PRIMARIES_BT2020",
            Self::XYZ => "SDL_COLOR_PRIMARIES_XYZ",
            Self::SMPTE431 => "SDL_COLOR_PRIMARIES_SMPTE431",
            Self::SMPTE432 => "SDL_COLOR_PRIMARIES_SMPTE432",
            Self::EBU3213 => "SDL_COLOR_PRIMARIES_EBU3213",
            Self::CUSTOM => "SDL_COLOR_PRIMARIES_CUSTOM",

            _ => return write!(f, "SDL_ColorPrimaries({})", self.0),
        })
    }
}

impl SDL_ColorPrimaries {
    pub const UNKNOWN: Self = Self(0);
    /// ITU-R BT.709-6
    pub const BT709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    /// ITU-R BT.470-6 System M
    pub const BT470M: Self = Self(4);
    /// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625
    pub const BT470BG: Self = Self(5);
    /// ITU-R BT.601-7 525, SMPTE 170M
    pub const BT601: Self = Self(6);
    /// SMPTE 240M, functionally the same as [`SDL_COLOR_PRIMARIES_BT601`]
    pub const SMPTE240: Self = Self(7);
    /// Generic film (color filters using Illuminant C)
    pub const GENERIC_FILM: Self = Self(8);
    /// ITU-R BT.2020-2 / ITU-R BT.2100-0
    pub const BT2020: Self = Self(9);
    /// SMPTE ST 428-1
    pub const XYZ: Self = Self(10);
    /// SMPTE RP 431-2
    pub const SMPTE431: Self = Self(11);
    /// SMPTE EG 432-1 / DCI P3
    pub const SMPTE432: Self = Self(12);
    /// EBU Tech. 3213-E
    pub const EBU3213: Self = Self(22);
    pub const CUSTOM: Self = Self(31);
}

pub const SDL_COLOR_PRIMARIES_UNKNOWN: SDL_ColorPrimaries = SDL_ColorPrimaries::UNKNOWN;
/// ITU-R BT.709-6
pub const SDL_COLOR_PRIMARIES_BT709: SDL_ColorPrimaries = SDL_ColorPrimaries::BT709;
pub const SDL_COLOR_PRIMARIES_UNSPECIFIED: SDL_ColorPrimaries = SDL_ColorPrimaries::UNSPECIFIED;
/// ITU-R BT.470-6 System M
pub const SDL_COLOR_PRIMARIES_BT470M: SDL_ColorPrimaries = SDL_ColorPrimaries::BT470M;
/// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625
pub const SDL_COLOR_PRIMARIES_BT470BG: SDL_ColorPrimaries = SDL_ColorPrimaries::BT470BG;
/// ITU-R BT.601-7 525, SMPTE 170M
pub const SDL_COLOR_PRIMARIES_BT601: SDL_ColorPrimaries = SDL_ColorPrimaries::BT601;
/// SMPTE 240M, functionally the same as [`SDL_COLOR_PRIMARIES_BT601`]
pub const SDL_COLOR_PRIMARIES_SMPTE240: SDL_ColorPrimaries = SDL_ColorPrimaries::SMPTE240;
/// Generic film (color filters using Illuminant C)
pub const SDL_COLOR_PRIMARIES_GENERIC_FILM: SDL_ColorPrimaries = SDL_ColorPrimaries::GENERIC_FILM;
/// ITU-R BT.2020-2 / ITU-R BT.2100-0
pub const SDL_COLOR_PRIMARIES_BT2020: SDL_ColorPrimaries = SDL_ColorPrimaries::BT2020;
/// SMPTE ST 428-1
pub const SDL_COLOR_PRIMARIES_XYZ: SDL_ColorPrimaries = SDL_ColorPrimaries::XYZ;
/// SMPTE RP 431-2
pub const SDL_COLOR_PRIMARIES_SMPTE431: SDL_ColorPrimaries = SDL_ColorPrimaries::SMPTE431;
/// SMPTE EG 432-1 / DCI P3
pub const SDL_COLOR_PRIMARIES_SMPTE432: SDL_ColorPrimaries = SDL_ColorPrimaries::SMPTE432;
/// EBU Tech. 3213-E
pub const SDL_COLOR_PRIMARIES_EBU3213: SDL_ColorPrimaries = SDL_ColorPrimaries::EBU3213;
pub const SDL_COLOR_PRIMARIES_CUSTOM: SDL_ColorPrimaries = SDL_ColorPrimaries::CUSTOM;

/// Colorspace transfer characteristics.
///
/// These are as described by <https://www.itu.int/rec/T-REC-H.273-201612-S/en>
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_TransferCharacteristics::UNKNOWN) | [`SDL_TRANSFER_CHARACTERISTICS_UNKNOWN`] | |
/// | [`BT709`](SDL_TransferCharacteristics::BT709) | [`SDL_TRANSFER_CHARACTERISTICS_BT709`] | Rec. ITU-R BT.709-6 / ITU-R BT1361 |
/// | [`UNSPECIFIED`](SDL_TransferCharacteristics::UNSPECIFIED) | [`SDL_TRANSFER_CHARACTERISTICS_UNSPECIFIED`] | |
/// | [`GAMMA22`](SDL_TransferCharacteristics::GAMMA22) | [`SDL_TRANSFER_CHARACTERISTICS_GAMMA22`] | ITU-R BT.470-6 System M / ITU-R BT1700 625 PAL & SECAM |
/// | [`GAMMA28`](SDL_TransferCharacteristics::GAMMA28) | [`SDL_TRANSFER_CHARACTERISTICS_GAMMA28`] | ITU-R BT.470-6 System B, G |
/// | [`BT601`](SDL_TransferCharacteristics::BT601) | [`SDL_TRANSFER_CHARACTERISTICS_BT601`] | SMPTE ST 170M / ITU-R BT.601-7 525 or 625 |
/// | [`SMPTE240`](SDL_TransferCharacteristics::SMPTE240) | [`SDL_TRANSFER_CHARACTERISTICS_SMPTE240`] | SMPTE ST 240M |
/// | [`LINEAR`](SDL_TransferCharacteristics::LINEAR) | [`SDL_TRANSFER_CHARACTERISTICS_LINEAR`] | |
/// | [`LOG100`](SDL_TransferCharacteristics::LOG100) | [`SDL_TRANSFER_CHARACTERISTICS_LOG100`] | |
/// | [`LOG100_SQRT10`](SDL_TransferCharacteristics::LOG100_SQRT10) | [`SDL_TRANSFER_CHARACTERISTICS_LOG100_SQRT10`] | |
/// | [`IEC61966`](SDL_TransferCharacteristics::IEC61966) | [`SDL_TRANSFER_CHARACTERISTICS_IEC61966`] | IEC 61966-2-4 |
/// | [`BT1361`](SDL_TransferCharacteristics::BT1361) | [`SDL_TRANSFER_CHARACTERISTICS_BT1361`] | ITU-R BT1361 Extended Colour Gamut |
/// | [`SRGB`](SDL_TransferCharacteristics::SRGB) | [`SDL_TRANSFER_CHARACTERISTICS_SRGB`] | IEC 61966-2-1 (sRGB or sYCC) |
/// | [`BT2020_10BIT`](SDL_TransferCharacteristics::BT2020_10BIT) | [`SDL_TRANSFER_CHARACTERISTICS_BT2020_10BIT`] | ITU-R BT2020 for 10-bit system |
/// | [`BT2020_12BIT`](SDL_TransferCharacteristics::BT2020_12BIT) | [`SDL_TRANSFER_CHARACTERISTICS_BT2020_12BIT`] | ITU-R BT2020 for 12-bit system |
/// | [`PQ`](SDL_TransferCharacteristics::PQ) | [`SDL_TRANSFER_CHARACTERISTICS_PQ`] | SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems |
/// | [`SMPTE428`](SDL_TransferCharacteristics::SMPTE428) | [`SDL_TRANSFER_CHARACTERISTICS_SMPTE428`] | SMPTE ST 428-1 |
/// | [`HLG`](SDL_TransferCharacteristics::HLG) | [`SDL_TRANSFER_CHARACTERISTICS_HLG`] | ARIB STD-B67, known as "hybrid log-gamma" (HLG) |
/// | [`CUSTOM`](SDL_TransferCharacteristics::CUSTOM) | [`SDL_TRANSFER_CHARACTERISTICS_CUSTOM`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_TransferCharacteristics(pub ::core::ffi::c_uint);

impl From<SDL_TransferCharacteristics> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_TransferCharacteristics) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_TransferCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_TRANSFER_CHARACTERISTICS_UNKNOWN",
            Self::BT709 => "SDL_TRANSFER_CHARACTERISTICS_BT709",
            Self::UNSPECIFIED => "SDL_TRANSFER_CHARACTERISTICS_UNSPECIFIED",
            Self::GAMMA22 => "SDL_TRANSFER_CHARACTERISTICS_GAMMA22",
            Self::GAMMA28 => "SDL_TRANSFER_CHARACTERISTICS_GAMMA28",
            Self::BT601 => "SDL_TRANSFER_CHARACTERISTICS_BT601",
            Self::SMPTE240 => "SDL_TRANSFER_CHARACTERISTICS_SMPTE240",
            Self::LINEAR => "SDL_TRANSFER_CHARACTERISTICS_LINEAR",
            Self::LOG100 => "SDL_TRANSFER_CHARACTERISTICS_LOG100",
            Self::LOG100_SQRT10 => "SDL_TRANSFER_CHARACTERISTICS_LOG100_SQRT10",
            Self::IEC61966 => "SDL_TRANSFER_CHARACTERISTICS_IEC61966",
            Self::BT1361 => "SDL_TRANSFER_CHARACTERISTICS_BT1361",
            Self::SRGB => "SDL_TRANSFER_CHARACTERISTICS_SRGB",
            Self::BT2020_10BIT => "SDL_TRANSFER_CHARACTERISTICS_BT2020_10BIT",
            Self::BT2020_12BIT => "SDL_TRANSFER_CHARACTERISTICS_BT2020_12BIT",
            Self::PQ => "SDL_TRANSFER_CHARACTERISTICS_PQ",
            Self::SMPTE428 => "SDL_TRANSFER_CHARACTERISTICS_SMPTE428",
            Self::HLG => "SDL_TRANSFER_CHARACTERISTICS_HLG",
            Self::CUSTOM => "SDL_TRANSFER_CHARACTERISTICS_CUSTOM",

            _ => return write!(f, "SDL_TransferCharacteristics({})", self.0),
        })
    }
}

impl SDL_TransferCharacteristics {
    pub const UNKNOWN: Self = Self(0);
    /// Rec. ITU-R BT.709-6 / ITU-R BT1361
    pub const BT709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    /// ITU-R BT.470-6 System M / ITU-R BT1700 625 PAL & SECAM
    pub const GAMMA22: Self = Self(4);
    /// ITU-R BT.470-6 System B, G
    pub const GAMMA28: Self = Self(5);
    /// SMPTE ST 170M / ITU-R BT.601-7 525 or 625
    pub const BT601: Self = Self(6);
    /// SMPTE ST 240M
    pub const SMPTE240: Self = Self(7);
    pub const LINEAR: Self = Self(8);
    pub const LOG100: Self = Self(9);
    pub const LOG100_SQRT10: Self = Self(10);
    /// IEC 61966-2-4
    pub const IEC61966: Self = Self(11);
    /// ITU-R BT1361 Extended Colour Gamut
    pub const BT1361: Self = Self(12);
    /// IEC 61966-2-1 (sRGB or sYCC)
    pub const SRGB: Self = Self(13);
    /// ITU-R BT2020 for 10-bit system
    pub const BT2020_10BIT: Self = Self(14);
    /// ITU-R BT2020 for 12-bit system
    pub const BT2020_12BIT: Self = Self(15);
    /// SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems
    pub const PQ: Self = Self(16);
    /// SMPTE ST 428-1
    pub const SMPTE428: Self = Self(17);
    /// ARIB STD-B67, known as "hybrid log-gamma" (HLG)
    pub const HLG: Self = Self(18);
    pub const CUSTOM: Self = Self(31);
}

pub const SDL_TRANSFER_CHARACTERISTICS_UNKNOWN: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::UNKNOWN;
/// Rec. ITU-R BT.709-6 / ITU-R BT1361
pub const SDL_TRANSFER_CHARACTERISTICS_BT709: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::BT709;
pub const SDL_TRANSFER_CHARACTERISTICS_UNSPECIFIED: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::UNSPECIFIED;
/// ITU-R BT.470-6 System M / ITU-R BT1700 625 PAL & SECAM
pub const SDL_TRANSFER_CHARACTERISTICS_GAMMA22: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::GAMMA22;
/// ITU-R BT.470-6 System B, G
pub const SDL_TRANSFER_CHARACTERISTICS_GAMMA28: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::GAMMA28;
/// SMPTE ST 170M / ITU-R BT.601-7 525 or 625
pub const SDL_TRANSFER_CHARACTERISTICS_BT601: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::BT601;
/// SMPTE ST 240M
pub const SDL_TRANSFER_CHARACTERISTICS_SMPTE240: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::SMPTE240;
pub const SDL_TRANSFER_CHARACTERISTICS_LINEAR: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::LINEAR;
pub const SDL_TRANSFER_CHARACTERISTICS_LOG100: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::LOG100;
pub const SDL_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::LOG100_SQRT10;
/// IEC 61966-2-4
pub const SDL_TRANSFER_CHARACTERISTICS_IEC61966: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::IEC61966;
/// ITU-R BT1361 Extended Colour Gamut
pub const SDL_TRANSFER_CHARACTERISTICS_BT1361: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::BT1361;
/// IEC 61966-2-1 (sRGB or sYCC)
pub const SDL_TRANSFER_CHARACTERISTICS_SRGB: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::SRGB;
/// ITU-R BT2020 for 10-bit system
pub const SDL_TRANSFER_CHARACTERISTICS_BT2020_10BIT: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::BT2020_10BIT;
/// ITU-R BT2020 for 12-bit system
pub const SDL_TRANSFER_CHARACTERISTICS_BT2020_12BIT: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::BT2020_12BIT;
/// SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems
pub const SDL_TRANSFER_CHARACTERISTICS_PQ: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::PQ;
/// SMPTE ST 428-1
pub const SDL_TRANSFER_CHARACTERISTICS_SMPTE428: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::SMPTE428;
/// ARIB STD-B67, known as "hybrid log-gamma" (HLG)
pub const SDL_TRANSFER_CHARACTERISTICS_HLG: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::HLG;
pub const SDL_TRANSFER_CHARACTERISTICS_CUSTOM: SDL_TransferCharacteristics =
    SDL_TransferCharacteristics::CUSTOM;

/// Colorspace matrix coefficients.
///
/// These are as described by <https://www.itu.int/rec/T-REC-H.273-201612-S/en>
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`IDENTITY`](SDL_MatrixCoefficients::IDENTITY) | [`SDL_MATRIX_COEFFICIENTS_IDENTITY`] | |
/// | [`BT709`](SDL_MatrixCoefficients::BT709) | [`SDL_MATRIX_COEFFICIENTS_BT709`] | ITU-R BT.709-6 |
/// | [`UNSPECIFIED`](SDL_MatrixCoefficients::UNSPECIFIED) | [`SDL_MATRIX_COEFFICIENTS_UNSPECIFIED`] | |
/// | [`FCC`](SDL_MatrixCoefficients::FCC) | [`SDL_MATRIX_COEFFICIENTS_FCC`] | US FCC Title 47 |
/// | [`BT470BG`](SDL_MatrixCoefficients::BT470BG) | [`SDL_MATRIX_COEFFICIENTS_BT470BG`] | ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625, functionally the same as [`SDL_MATRIX_COEFFICIENTS_BT601`] |
/// | [`BT601`](SDL_MatrixCoefficients::BT601) | [`SDL_MATRIX_COEFFICIENTS_BT601`] | ITU-R BT.601-7 525 |
/// | [`SMPTE240`](SDL_MatrixCoefficients::SMPTE240) | [`SDL_MATRIX_COEFFICIENTS_SMPTE240`] | SMPTE 240M |
/// | [`YCGCO`](SDL_MatrixCoefficients::YCGCO) | [`SDL_MATRIX_COEFFICIENTS_YCGCO`] | |
/// | [`BT2020_NCL`](SDL_MatrixCoefficients::BT2020_NCL) | [`SDL_MATRIX_COEFFICIENTS_BT2020_NCL`] | ITU-R BT.2020-2 non-constant luminance |
/// | [`BT2020_CL`](SDL_MatrixCoefficients::BT2020_CL) | [`SDL_MATRIX_COEFFICIENTS_BT2020_CL`] | ITU-R BT.2020-2 constant luminance |
/// | [`SMPTE2085`](SDL_MatrixCoefficients::SMPTE2085) | [`SDL_MATRIX_COEFFICIENTS_SMPTE2085`] | SMPTE ST 2085 |
/// | [`CHROMA_DERIVED_NCL`](SDL_MatrixCoefficients::CHROMA_DERIVED_NCL) | [`SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL`] | |
/// | [`CHROMA_DERIVED_CL`](SDL_MatrixCoefficients::CHROMA_DERIVED_CL) | [`SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL`] | |
/// | [`ICTCP`](SDL_MatrixCoefficients::ICTCP) | [`SDL_MATRIX_COEFFICIENTS_ICTCP`] | ITU-R BT.2100-0 ICTCP |
/// | [`CUSTOM`](SDL_MatrixCoefficients::CUSTOM) | [`SDL_MATRIX_COEFFICIENTS_CUSTOM`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MatrixCoefficients(pub ::core::ffi::c_uint);

impl From<SDL_MatrixCoefficients> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_MatrixCoefficients) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_MatrixCoefficients {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::IDENTITY => "SDL_MATRIX_COEFFICIENTS_IDENTITY",
            Self::BT709 => "SDL_MATRIX_COEFFICIENTS_BT709",
            Self::UNSPECIFIED => "SDL_MATRIX_COEFFICIENTS_UNSPECIFIED",
            Self::FCC => "SDL_MATRIX_COEFFICIENTS_FCC",
            Self::BT470BG => "SDL_MATRIX_COEFFICIENTS_BT470BG",
            Self::BT601 => "SDL_MATRIX_COEFFICIENTS_BT601",
            Self::SMPTE240 => "SDL_MATRIX_COEFFICIENTS_SMPTE240",
            Self::YCGCO => "SDL_MATRIX_COEFFICIENTS_YCGCO",
            Self::BT2020_NCL => "SDL_MATRIX_COEFFICIENTS_BT2020_NCL",
            Self::BT2020_CL => "SDL_MATRIX_COEFFICIENTS_BT2020_CL",
            Self::SMPTE2085 => "SDL_MATRIX_COEFFICIENTS_SMPTE2085",
            Self::CHROMA_DERIVED_NCL => "SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL",
            Self::CHROMA_DERIVED_CL => "SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL",
            Self::ICTCP => "SDL_MATRIX_COEFFICIENTS_ICTCP",
            Self::CUSTOM => "SDL_MATRIX_COEFFICIENTS_CUSTOM",

            _ => return write!(f, "SDL_MatrixCoefficients({})", self.0),
        })
    }
}

impl SDL_MatrixCoefficients {
    pub const IDENTITY: Self = Self(0);
    /// ITU-R BT.709-6
    pub const BT709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    /// US FCC Title 47
    pub const FCC: Self = Self(4);
    /// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625, functionally the same as [`SDL_MATRIX_COEFFICIENTS_BT601`]
    pub const BT470BG: Self = Self(5);
    /// ITU-R BT.601-7 525
    pub const BT601: Self = Self(6);
    /// SMPTE 240M
    pub const SMPTE240: Self = Self(7);
    pub const YCGCO: Self = Self(8);
    /// ITU-R BT.2020-2 non-constant luminance
    pub const BT2020_NCL: Self = Self(9);
    /// ITU-R BT.2020-2 constant luminance
    pub const BT2020_CL: Self = Self(10);
    /// SMPTE ST 2085
    pub const SMPTE2085: Self = Self(11);
    pub const CHROMA_DERIVED_NCL: Self = Self(12);
    pub const CHROMA_DERIVED_CL: Self = Self(13);
    /// ITU-R BT.2100-0 ICTCP
    pub const ICTCP: Self = Self(14);
    pub const CUSTOM: Self = Self(31);
}

pub const SDL_MATRIX_COEFFICIENTS_IDENTITY: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::IDENTITY;
/// ITU-R BT.709-6
pub const SDL_MATRIX_COEFFICIENTS_BT709: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT709;
pub const SDL_MATRIX_COEFFICIENTS_UNSPECIFIED: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::UNSPECIFIED;
/// US FCC Title 47
pub const SDL_MATRIX_COEFFICIENTS_FCC: SDL_MatrixCoefficients = SDL_MatrixCoefficients::FCC;
/// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625, functionally the same as [`SDL_MATRIX_COEFFICIENTS_BT601`]
pub const SDL_MATRIX_COEFFICIENTS_BT470BG: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT470BG;
/// ITU-R BT.601-7 525
pub const SDL_MATRIX_COEFFICIENTS_BT601: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT601;
/// SMPTE 240M
pub const SDL_MATRIX_COEFFICIENTS_SMPTE240: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::SMPTE240;
pub const SDL_MATRIX_COEFFICIENTS_YCGCO: SDL_MatrixCoefficients = SDL_MatrixCoefficients::YCGCO;
/// ITU-R BT.2020-2 non-constant luminance
pub const SDL_MATRIX_COEFFICIENTS_BT2020_NCL: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::BT2020_NCL;
/// ITU-R BT.2020-2 constant luminance
pub const SDL_MATRIX_COEFFICIENTS_BT2020_CL: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::BT2020_CL;
/// SMPTE ST 2085
pub const SDL_MATRIX_COEFFICIENTS_SMPTE2085: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::SMPTE2085;
pub const SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::CHROMA_DERIVED_NCL;
pub const SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: SDL_MatrixCoefficients =
    SDL_MatrixCoefficients::CHROMA_DERIVED_CL;
/// ITU-R BT.2100-0 ICTCP
pub const SDL_MATRIX_COEFFICIENTS_ICTCP: SDL_MatrixCoefficients = SDL_MatrixCoefficients::ICTCP;
pub const SDL_MATRIX_COEFFICIENTS_CUSTOM: SDL_MatrixCoefficients = SDL_MatrixCoefficients::CUSTOM;

/// Colorspace chroma sample location.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_ChromaLocation::NONE) | [`SDL_CHROMA_LOCATION_NONE`] | RGB, no chroma sampling |
/// | [`LEFT`](SDL_ChromaLocation::LEFT) | [`SDL_CHROMA_LOCATION_LEFT`] | In MPEG-2, MPEG-4, and AVC, Cb and Cr are taken on midpoint of the left-edge of the 2x2 square. In other words, they have the same horizontal location as the top-left pixel, but is shifted one-half pixel down vertically. |
/// | [`CENTER`](SDL_ChromaLocation::CENTER) | [`SDL_CHROMA_LOCATION_CENTER`] | In JPEG/JFIF, H.261, and MPEG-1, Cb and Cr are taken at the center of the 2x2 square. In other words, they are offset one-half pixel to the right and one-half pixel down compared to the top-left pixel. |
/// | [`TOPLEFT`](SDL_ChromaLocation::TOPLEFT) | [`SDL_CHROMA_LOCATION_TOPLEFT`] | In HEVC for BT.2020 and BT.2100 content (in particular on Blu-rays), Cb and Cr are sampled at the same location as the group's top-left Y pixel ("co-sited", "co-located"). |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ChromaLocation(pub ::core::ffi::c_uint);

impl From<SDL_ChromaLocation> for ::core::ffi::c_uint {
    #[inline(always)]
    fn from(value: SDL_ChromaLocation) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ChromaLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_CHROMA_LOCATION_NONE",
            Self::LEFT => "SDL_CHROMA_LOCATION_LEFT",
            Self::CENTER => "SDL_CHROMA_LOCATION_CENTER",
            Self::TOPLEFT => "SDL_CHROMA_LOCATION_TOPLEFT",

            _ => return write!(f, "SDL_ChromaLocation({})", self.0),
        })
    }
}

impl SDL_ChromaLocation {
    /// RGB, no chroma sampling
    pub const NONE: Self = Self(0);
    /// In MPEG-2, MPEG-4, and AVC, Cb and Cr are taken on midpoint of the left-edge of the 2x2 square. In other words, they have the same horizontal location as the top-left pixel, but is shifted one-half pixel down vertically.
    pub const LEFT: Self = Self(1);
    /// In JPEG/JFIF, H.261, and MPEG-1, Cb and Cr are taken at the center of the 2x2 square. In other words, they are offset one-half pixel to the right and one-half pixel down compared to the top-left pixel.
    pub const CENTER: Self = Self(2);
    /// In HEVC for BT.2020 and BT.2100 content (in particular on Blu-rays), Cb and Cr are sampled at the same location as the group's top-left Y pixel ("co-sited", "co-located").
    pub const TOPLEFT: Self = Self(3);
}

/// RGB, no chroma sampling
pub const SDL_CHROMA_LOCATION_NONE: SDL_ChromaLocation = SDL_ChromaLocation::NONE;
/// In MPEG-2, MPEG-4, and AVC, Cb and Cr are taken on midpoint of the left-edge of the 2x2 square. In other words, they have the same horizontal location as the top-left pixel, but is shifted one-half pixel down vertically.
pub const SDL_CHROMA_LOCATION_LEFT: SDL_ChromaLocation = SDL_ChromaLocation::LEFT;
/// In JPEG/JFIF, H.261, and MPEG-1, Cb and Cr are taken at the center of the 2x2 square. In other words, they are offset one-half pixel to the right and one-half pixel down compared to the top-left pixel.
pub const SDL_CHROMA_LOCATION_CENTER: SDL_ChromaLocation = SDL_ChromaLocation::CENTER;
/// In HEVC for BT.2020 and BT.2100 content (in particular on Blu-rays), Cb and Cr are sampled at the same location as the group's top-left Y pixel ("co-sited", "co-located").
pub const SDL_CHROMA_LOCATION_TOPLEFT: SDL_ChromaLocation = SDL_ChromaLocation::TOPLEFT;

/// Colorspace definitions.
///
/// Since similar colorspaces may vary in their details (matrix, transfer
/// function, etc.), this is not an exhaustive list, but rather a
/// representative sample of the kinds of colorspaces supported in SDL.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ColorPrimaries`]
/// - [`SDL_ColorRange`]
/// - [`SDL_ColorType`]
/// - [`SDL_MatrixCoefficients`]
/// - [`SDL_TransferCharacteristics`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_Colorspace::UNKNOWN) | [`SDL_COLORSPACE_UNKNOWN`] | |
/// | [`SRGB`](SDL_Colorspace::SRGB) | [`SDL_COLORSPACE_SRGB`] | Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709 |
/// | [`SRGB_LINEAR`](SDL_Colorspace::SRGB_LINEAR) | [`SDL_COLORSPACE_SRGB_LINEAR`] | Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709 |
/// | [`HDR10`](SDL_Colorspace::HDR10) | [`SDL_COLORSPACE_HDR10`] | Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020 |
/// | [`JPEG`](SDL_Colorspace::JPEG) | [`SDL_COLORSPACE_JPEG`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601 |
/// | [`BT601_LIMITED`](SDL_Colorspace::BT601_LIMITED) | [`SDL_COLORSPACE_BT601_LIMITED`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 |
/// | [`BT601_FULL`](SDL_Colorspace::BT601_FULL) | [`SDL_COLORSPACE_BT601_FULL`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 |
/// | [`BT709_LIMITED`](SDL_Colorspace::BT709_LIMITED) | [`SDL_COLORSPACE_BT709_LIMITED`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 |
/// | [`BT709_FULL`](SDL_Colorspace::BT709_FULL) | [`SDL_COLORSPACE_BT709_FULL`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 |
/// | [`BT2020_LIMITED`](SDL_Colorspace::BT2020_LIMITED) | [`SDL_COLORSPACE_BT2020_LIMITED`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020 |
/// | [`BT2020_FULL`](SDL_Colorspace::BT2020_FULL) | [`SDL_COLORSPACE_BT2020_FULL`] | Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020 |
/// | [`RGB_DEFAULT`](SDL_Colorspace::RGB_DEFAULT) | [`SDL_COLORSPACE_RGB_DEFAULT`] | The default colorspace for RGB surfaces if no colorspace is specified |
/// | [`YUV_DEFAULT`](SDL_Colorspace::YUV_DEFAULT) | [`SDL_COLORSPACE_YUV_DEFAULT`] | The default colorspace for YUV surfaces if no colorspace is specified |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Colorspace(pub Uint32);

impl From<SDL_Colorspace> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_Colorspace) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_Colorspace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_COLORSPACE_UNKNOWN",
            Self::SRGB => "SDL_COLORSPACE_SRGB",
            Self::SRGB_LINEAR => "SDL_COLORSPACE_SRGB_LINEAR",
            Self::HDR10 => "SDL_COLORSPACE_HDR10",
            Self::JPEG => "SDL_COLORSPACE_JPEG",
            Self::BT601_LIMITED => "SDL_COLORSPACE_BT601_LIMITED",
            Self::BT601_FULL => "SDL_COLORSPACE_BT601_FULL",
            Self::BT709_LIMITED => "SDL_COLORSPACE_BT709_LIMITED",
            Self::BT709_FULL => "SDL_COLORSPACE_BT709_FULL",
            Self::BT2020_LIMITED => "SDL_COLORSPACE_BT2020_LIMITED",
            Self::BT2020_FULL => "SDL_COLORSPACE_BT2020_FULL",
            Self::RGB_DEFAULT => "SDL_COLORSPACE_RGB_DEFAULT",
            Self::YUV_DEFAULT => "SDL_COLORSPACE_YUV_DEFAULT",

            _ => return write!(f, "SDL_Colorspace({})", self.0),
        })
    }
}

impl SDL_Colorspace {
    pub const UNKNOWN: Self = Self(0);
    /// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709
    pub const SRGB: Self = Self(0x120005a0);
    /// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709
    pub const SRGB_LINEAR: Self = Self(0x12000500);
    /// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020
    pub const HDR10: Self = Self(0x12002600);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601
    pub const JPEG: Self = Self(0x220004c6);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601
    pub const BT601_LIMITED: Self = Self(0x211018c6);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601
    pub const BT601_FULL: Self = Self(0x221018c6);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709
    pub const BT709_LIMITED: Self = Self(0x21100421);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709
    pub const BT709_FULL: Self = Self(0x22100421);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020
    pub const BT2020_LIMITED: Self = Self(0x21102609);
    /// Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020
    pub const BT2020_FULL: Self = Self(0x22102609);
    /// The default colorspace for RGB surfaces if no colorspace is specified
    pub const RGB_DEFAULT: Self = SDL_COLORSPACE_SRGB;
    /// The default colorspace for YUV surfaces if no colorspace is specified
    pub const YUV_DEFAULT: Self = SDL_COLORSPACE_JPEG;
}

pub const SDL_COLORSPACE_UNKNOWN: SDL_Colorspace = SDL_Colorspace::UNKNOWN;
/// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709
pub const SDL_COLORSPACE_SRGB: SDL_Colorspace = SDL_Colorspace::SRGB;
/// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709
pub const SDL_COLORSPACE_SRGB_LINEAR: SDL_Colorspace = SDL_Colorspace::SRGB_LINEAR;
/// Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020
pub const SDL_COLORSPACE_HDR10: SDL_Colorspace = SDL_Colorspace::HDR10;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601
pub const SDL_COLORSPACE_JPEG: SDL_Colorspace = SDL_Colorspace::JPEG;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601
pub const SDL_COLORSPACE_BT601_LIMITED: SDL_Colorspace = SDL_Colorspace::BT601_LIMITED;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601
pub const SDL_COLORSPACE_BT601_FULL: SDL_Colorspace = SDL_Colorspace::BT601_FULL;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709
pub const SDL_COLORSPACE_BT709_LIMITED: SDL_Colorspace = SDL_Colorspace::BT709_LIMITED;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709
pub const SDL_COLORSPACE_BT709_FULL: SDL_Colorspace = SDL_Colorspace::BT709_FULL;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020
pub const SDL_COLORSPACE_BT2020_LIMITED: SDL_Colorspace = SDL_Colorspace::BT2020_LIMITED;
/// Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020
pub const SDL_COLORSPACE_BT2020_FULL: SDL_Colorspace = SDL_Colorspace::BT2020_FULL;
/// The default colorspace for RGB surfaces if no colorspace is specified
pub const SDL_COLORSPACE_RGB_DEFAULT: SDL_Colorspace = SDL_Colorspace::RGB_DEFAULT;
/// The default colorspace for YUV surfaces if no colorspace is specified
pub const SDL_COLORSPACE_YUV_DEFAULT: SDL_Colorspace = SDL_Colorspace::YUV_DEFAULT;

#[inline(always)]
pub const fn SDL_DEFINE_COLORSPACE(
    r#type: SDL_ColorType,
    range: SDL_ColorRange,
    primaries: SDL_ColorPrimaries,
    transfer: SDL_TransferCharacteristics,
    matrix: SDL_MatrixCoefficients,
    chroma: SDL_ChromaLocation,
) -> SDL_Colorspace {
    SDL_Colorspace(
        (((((((r#type.0 as Uint32) << 28) | ((range.0 as Uint32) << 24))
            | ((chroma.0 as Uint32) << 20))
            | ((primaries.0 as Uint32) << 10))
            | ((transfer.0 as Uint32) << 5))
            | ((matrix.0 as Uint32) << 0)),
    )
}

#[inline(always)]
pub const fn SDL_COLORSPACETYPE(X: SDL_Colorspace) -> SDL_ColorType {
    SDL_ColorType(((X.0 >> 28) & 15_u32))
}

#[inline(always)]
pub const fn SDL_COLORSPACERANGE(X: SDL_Colorspace) -> SDL_ColorRange {
    SDL_ColorRange(((X.0 >> 24) & 15_u32))
}

#[inline(always)]
pub const fn SDL_COLORSPACECHROMA(X: SDL_Colorspace) -> SDL_ChromaLocation {
    SDL_ChromaLocation(((X.0 >> 20) & 15_u32))
}

#[inline(always)]
pub const fn SDL_COLORSPACEPRIMARIES(X: SDL_Colorspace) -> SDL_ColorPrimaries {
    SDL_ColorPrimaries(((X.0 >> 10) & 31_u32))
}

#[inline(always)]
pub const fn SDL_COLORSPACETRANSFER(X: SDL_Colorspace) -> SDL_TransferCharacteristics {
    SDL_TransferCharacteristics(((X.0 >> 5) & 31_u32))
}

#[inline(always)]
pub const fn SDL_COLORSPACEMATRIX(X: SDL_Colorspace) -> SDL_MatrixCoefficients {
    SDL_MatrixCoefficients((X.0 & 31_u32))
}

#[inline(always)]
pub const fn SDL_ISCOLORSPACE_LIMITED_RANGE(X: SDL_Colorspace) -> ::core::primitive::bool {
    (SDL_COLORSPACERANGE(X).0 != SDL_COLOR_RANGE_FULL.0)
}

#[inline(always)]
pub const fn SDL_ISCOLORSPACE_FULL_RANGE(X: SDL_Colorspace) -> ::core::primitive::bool {
    (SDL_COLORSPACERANGE(X).0 == SDL_COLOR_RANGE_FULL.0)
}

#[inline(always)]
pub const fn SDL_ISCOLORSPACE_MATRIX_BT601(X: SDL_Colorspace) -> ::core::primitive::bool {
    ((SDL_COLORSPACEMATRIX(X).0 == SDL_MATRIX_COEFFICIENTS_BT601.0)
        || (SDL_COLORSPACEMATRIX(X).0 == SDL_MATRIX_COEFFICIENTS_BT470BG.0))
}

#[inline(always)]
pub const fn SDL_ISCOLORSPACE_MATRIX_BT709(X: SDL_Colorspace) -> ::core::primitive::bool {
    (SDL_COLORSPACEMATRIX(X).0 == SDL_MATRIX_COEFFICIENTS_BT709.0)
}

#[inline(always)]
pub const fn SDL_ISCOLORSPACE_MATRIX_BT2020_NCL(X: SDL_Colorspace) -> ::core::primitive::bool {
    (SDL_COLORSPACEMATRIX(X).0 == SDL_MATRIX_COEFFICIENTS_BT2020_NCL.0)
}

/// A structure that represents a color as RGBA components.
///
/// The bits of this structure can be directly reinterpreted as an
/// integer-packed color which uses the [`SDL_PIXELFORMAT_RGBA32`] format
/// ([`SDL_PIXELFORMAT_ABGR8888`] on little-endian systems and
/// [`SDL_PIXELFORMAT_RGBA8888`] on big-endian systems).
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}

/// The bits of this structure can be directly reinterpreted as a float-packed
/// color which uses the [`SDL_PIXELFORMAT_RGBA128_FLOAT`] format
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FColor {
    pub r: ::core::ffi::c_float,
    pub g: ::core::ffi::c_float,
    pub b: ::core::ffi::c_float,
    pub a: ::core::ffi::c_float,
}

/// A set of indexed colors representing a palette.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_SetPaletteColors`]
#[repr(C)]
// #[non_exhaustive] // temporarily disabled bc of https://github.com/rust-lang/rust/issues/132699
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Palette {
    /// number of elements in `colors`.
    pub ncolors: ::core::ffi::c_int,
    /// an array of colors, `ncolors` long.
    pub colors: *mut SDL_Color,
    /// internal use only, do not touch.
    pub version: Uint32,
    /// internal use only, do not touch.
    pub refcount: ::core::ffi::c_int,
}

/// Details about the format of a pixel.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PixelFormatDetails {
    pub format: SDL_PixelFormat,
    pub bits_per_pixel: Uint8,
    pub bytes_per_pixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rbits: Uint8,
    pub Gbits: Uint8,
    pub Bbits: Uint8,
    pub Abits: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
}

extern "C" {
    /// Get the human readable name of a pixel format.
    ///
    /// ### Parameters
    /// - `format`: the pixel format to query.
    ///
    /// ### Return value
    /// Returns the human readable name of the specified pixel format or
    ///   "SDL_PIXELFORMAT_UNKNOWN" if the format isn't recognized.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetPixelFormatName(format: SDL_PixelFormat) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Convert one of the enumerated pixel formats to a bpp value and RGBA masks.
    ///
    /// ### Parameters
    /// - `format`: one of the [`SDL_PixelFormat`] values.
    /// - `bpp`: a bits per pixel value; usually 15, 16, or 32.
    /// - `Rmask`: a pointer filled in with the red mask for the format.
    /// - `Gmask`: a pointer filled in with the green mask for the format.
    /// - `Bmask`: a pointer filled in with the blue mask for the format.
    /// - `Amask`: a pointer filled in with the alpha mask for the format.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPixelFormatForMasks`]
    pub fn SDL_GetMasksForPixelFormat(
        format: SDL_PixelFormat,
        bpp: *mut ::core::ffi::c_int,
        Rmask: *mut Uint32,
        Gmask: *mut Uint32,
        Bmask: *mut Uint32,
        Amask: *mut Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Convert a bpp value and RGBA masks to an enumerated pixel format.
    ///
    /// This will return [`SDL_PIXELFORMAT_UNKNOWN`] if the conversion wasn't
    /// possible.
    ///
    /// ### Parameters
    /// - `bpp`: a bits per pixel value; usually 15, 16, or 32.
    /// - `Rmask`: the red mask for the format.
    /// - `Gmask`: the green mask for the format.
    /// - `Bmask`: the blue mask for the format.
    /// - `Amask`: the alpha mask for the format.
    ///
    /// ### Return value
    /// Returns the [`SDL_PixelFormat`] value corresponding to the format masks, or
    ///   [`SDL_PIXELFORMAT_UNKNOWN`] if there isn't a match.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetMasksForPixelFormat`]
    pub fn SDL_GetPixelFormatForMasks(
        bpp: ::core::ffi::c_int,
        Rmask: Uint32,
        Gmask: Uint32,
        Bmask: Uint32,
        Amask: Uint32,
    ) -> SDL_PixelFormat;
}

extern "C" {
    /// Create an [`SDL_PixelFormatDetails`] structure corresponding to a pixel format.
    ///
    /// Returned structure may come from a shared global cache (i.e. not newly
    /// allocated), and hence should not be modified, especially the palette. Weird
    /// errors such as `Blit combination not supported` may occur.
    ///
    /// ### Parameters
    /// - `format`: one of the [`SDL_PixelFormat`] values.
    ///
    /// ### Return value
    /// Returns a pointer to a [`SDL_PixelFormatDetails`] structure or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetPixelFormatDetails(format: SDL_PixelFormat) -> *const SDL_PixelFormatDetails;
}

extern "C" {
    /// Create a palette structure with the specified number of color entries.
    ///
    /// The palette entries are initialized to white.
    ///
    /// ### Parameters
    /// - `ncolors`: represents the number of color entries in the color palette.
    ///
    /// ### Return value
    /// Returns a new [`SDL_Palette`] structure on success or NULL on failure (e.g. if
    ///   there wasn't enough memory); call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroyPalette`]
    /// - [`SDL_SetPaletteColors`]
    /// - [`SDL_SetSurfacePalette`]
    pub fn SDL_CreatePalette(ncolors: ::core::ffi::c_int) -> *mut SDL_Palette;
}

extern "C" {
    /// Set a range of colors in a palette.
    ///
    /// ### Parameters
    /// - `palette`: the [`SDL_Palette`] structure to modify.
    /// - `colors`: an array of [`SDL_Color`] structures to copy into the palette.
    /// - `firstcolor`: the index of the first palette entry to modify.
    /// - `ncolors`: the number of entries to modify.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified or destroyed in another thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_SetPaletteColors(
        palette: *mut SDL_Palette,
        colors: *const SDL_Color,
        firstcolor: ::core::ffi::c_int,
        ncolors: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Free a palette created with [`SDL_CreatePalette()`].
    ///
    /// ### Parameters
    /// - `palette`: the [`SDL_Palette`] structure to be freed.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified or destroyed in another thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreatePalette`]
    pub fn SDL_DestroyPalette(palette: *mut SDL_Palette);
}

extern "C" {
    /// Map an RGB triple to an opaque pixel value for a given pixel format.
    ///
    /// This function maps the RGB color value to the specified pixel format and
    /// returns the pixel value best approximating the given RGB color value for
    /// the given pixel format.
    ///
    /// If the format has a palette (8-bit) the index of the closest matching color
    /// in the palette will be returned.
    ///
    /// If the specified pixel format has an alpha component it will be returned as
    /// all 1 bits (fully opaque).
    ///
    /// If the pixel format bpp (color depth) is less than 32-bpp then the unused
    /// upper bits of the return value can safely be ignored (e.g., with a 16-bpp
    /// format the return value can be assigned to a Uint16, and similarly a Uint8
    /// for an 8-bpp format).
    ///
    /// ### Parameters
    /// - `format`: a pointer to [`SDL_PixelFormatDetails`] describing the pixel
    ///   format.
    /// - `palette`: an optional palette for indexed formats, may be NULL.
    /// - `r`: the red component of the pixel in the range 0-255.
    /// - `g`: the green component of the pixel in the range 0-255.
    /// - `b`: the blue component of the pixel in the range 0-255.
    ///
    /// ### Return value
    /// Returns a pixel value.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPixelFormatDetails`]
    /// - [`SDL_GetRGB`]
    /// - [`SDL_MapRGBA`]
    /// - [`SDL_MapSurfaceRGB`]
    pub fn SDL_MapRGB(
        format: *const SDL_PixelFormatDetails,
        palette: *const SDL_Palette,
        r: Uint8,
        g: Uint8,
        b: Uint8,
    ) -> Uint32;
}

extern "C" {
    /// Map an RGBA quadruple to a pixel value for a given pixel format.
    ///
    /// This function maps the RGBA color value to the specified pixel format and
    /// returns the pixel value best approximating the given RGBA color value for
    /// the given pixel format.
    ///
    /// If the specified pixel format has no alpha component the alpha value will
    /// be ignored (as it will be in formats with a palette).
    ///
    /// If the format has a palette (8-bit) the index of the closest matching color
    /// in the palette will be returned.
    ///
    /// If the pixel format bpp (color depth) is less than 32-bpp then the unused
    /// upper bits of the return value can safely be ignored (e.g., with a 16-bpp
    /// format the return value can be assigned to a Uint16, and similarly a Uint8
    /// for an 8-bpp format).
    ///
    /// ### Parameters
    /// - `format`: a pointer to [`SDL_PixelFormatDetails`] describing the pixel
    ///   format.
    /// - `palette`: an optional palette for indexed formats, may be NULL.
    /// - `r`: the red component of the pixel in the range 0-255.
    /// - `g`: the green component of the pixel in the range 0-255.
    /// - `b`: the blue component of the pixel in the range 0-255.
    /// - `a`: the alpha component of the pixel in the range 0-255.
    ///
    /// ### Return value
    /// Returns a pixel value.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPixelFormatDetails`]
    /// - [`SDL_GetRGBA`]
    /// - [`SDL_MapRGB`]
    /// - [`SDL_MapSurfaceRGBA`]
    pub fn SDL_MapRGBA(
        format: *const SDL_PixelFormatDetails,
        palette: *const SDL_Palette,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> Uint32;
}

extern "C" {
    /// Get RGB values from a pixel in the specified format.
    ///
    /// This function uses the entire 8-bit \[0..255\] range when converting color
    /// components from pixel formats with less than 8-bits per RGB component
    /// (e.g., a completely white pixel in 16-bit RGB565 format would return [0xff,
    /// 0xff, 0xff\] not \[0xf8, 0xfc, 0xf8\]).
    ///
    /// ### Parameters
    /// - `pixel`: a pixel value.
    /// - `format`: a pointer to [`SDL_PixelFormatDetails`] describing the pixel
    ///   format.
    /// - `palette`: an optional palette for indexed formats, may be NULL.
    /// - `r`: a pointer filled in with the red component, may be NULL.
    /// - `g`: a pointer filled in with the green component, may be NULL.
    /// - `b`: a pointer filled in with the blue component, may be NULL.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPixelFormatDetails`]
    /// - [`SDL_GetRGBA`]
    /// - [`SDL_MapRGB`]
    /// - [`SDL_MapRGBA`]
    pub fn SDL_GetRGB(
        pixel: Uint32,
        format: *const SDL_PixelFormatDetails,
        palette: *const SDL_Palette,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
    );
}

extern "C" {
    /// Get RGBA values from a pixel in the specified format.
    ///
    /// This function uses the entire 8-bit \[0..255\] range when converting color
    /// components from pixel formats with less than 8-bits per RGB component
    /// (e.g., a completely white pixel in 16-bit RGB565 format would return [0xff,
    /// 0xff, 0xff\] not \[0xf8, 0xfc, 0xf8\]).
    ///
    /// If the surface has no alpha component, the alpha will be returned as 0xff
    /// (100% opaque).
    ///
    /// ### Parameters
    /// - `pixel`: a pixel value.
    /// - `format`: a pointer to [`SDL_PixelFormatDetails`] describing the pixel
    ///   format.
    /// - `palette`: an optional palette for indexed formats, may be NULL.
    /// - `r`: a pointer filled in with the red component, may be NULL.
    /// - `g`: a pointer filled in with the green component, may be NULL.
    /// - `b`: a pointer filled in with the blue component, may be NULL.
    /// - `a`: a pointer filled in with the alpha component, may be NULL.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the palette is not modified.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPixelFormatDetails`]
    /// - [`SDL_GetRGB`]
    /// - [`SDL_MapRGB`]
    /// - [`SDL_MapRGBA`]
    pub fn SDL_GetRGBA(
        pixel: Uint32,
        format: *const SDL_PixelFormatDetails,
        palette: *const SDL_Palette,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
        a: *mut Uint8,
    );
}

#[cfg(doc)]
use crate::everything::*;
