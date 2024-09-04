#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryPixels
//!
//! Pixel management.

use super::stdinc::*;

use super::error::*;

pub const SDL_ALPHA_OPAQUE: ::core::primitive::i32 = 255;

pub const SDL_ALPHA_OPAQUE_FLOAT: ::core::primitive::f32 = 1;

pub const SDL_ALPHA_TRANSPARENT: ::core::primitive::i32 = 0;

pub const SDL_ALPHA_TRANSPARENT_FLOAT: ::core::primitive::f32 = 0;

/// Pixel type.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PIXELTYPE_UNKNOWN`], [`SDL_PIXELTYPE_INDEX1`], [`SDL_PIXELTYPE_INDEX4`], [`SDL_PIXELTYPE_INDEX8`], [`SDL_PIXELTYPE_PACKED8`], [`SDL_PIXELTYPE_PACKED16`], [`SDL_PIXELTYPE_PACKED32`], [`SDL_PIXELTYPE_ARRAYU8`], [`SDL_PIXELTYPE_ARRAYU16`], [`SDL_PIXELTYPE_ARRAYU32`], [`SDL_PIXELTYPE_ARRAYF16`], [`SDL_PIXELTYPE_ARRAYF32`], [`SDL_PIXELTYPE_INDEX2`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PixelType(pub ::core::ffi::c_int);
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
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_BITMAPORDER_NONE`], [`SDL_BITMAPORDER_4321`], [`SDL_BITMAPORDER_1234`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_BitmapOrder(pub ::core::ffi::c_int);
impl SDL_BitmapOrder {
    pub const NONE: Self = Self(0);
}
pub const SDL_BITMAPORDER_NONE: SDL_BitmapOrder = SDL_BitmapOrder::NONE;
pub const SDL_BITMAPORDER_4321: SDL_BitmapOrder = SDL_BitmapOrder(1);
pub const SDL_BITMAPORDER_1234: SDL_BitmapOrder = SDL_BitmapOrder(2);

/// Packed component order, high bit -> low bit.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PACKEDORDER_NONE`], [`SDL_PACKEDORDER_XRGB`], [`SDL_PACKEDORDER_RGBX`], [`SDL_PACKEDORDER_ARGB`], [`SDL_PACKEDORDER_RGBA`], [`SDL_PACKEDORDER_XBGR`], [`SDL_PACKEDORDER_BGRX`], [`SDL_PACKEDORDER_ABGR`], [`SDL_PACKEDORDER_BGRA`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PackedOrder(pub ::core::ffi::c_int);
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
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_ARRAYORDER_NONE`], [`SDL_ARRAYORDER_RGB`], [`SDL_ARRAYORDER_RGBA`], [`SDL_ARRAYORDER_ARGB`], [`SDL_ARRAYORDER_BGR`], [`SDL_ARRAYORDER_BGRA`], [`SDL_ARRAYORDER_ABGR`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ArrayOrder(pub ::core::ffi::c_int);
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
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PACKEDLAYOUT_NONE`], [`SDL_PACKEDLAYOUT_332`], [`SDL_PACKEDLAYOUT_4444`], [`SDL_PACKEDLAYOUT_1555`], [`SDL_PACKEDLAYOUT_5551`], [`SDL_PACKEDLAYOUT_565`], [`SDL_PACKEDLAYOUT_8888`], [`SDL_PACKEDLAYOUT_2101010`], [`SDL_PACKEDLAYOUT_1010102`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PackedLayout(pub ::core::ffi::c_int);
impl SDL_PackedLayout {
    pub const NONE: Self = Self(0);
}
pub const SDL_PACKEDLAYOUT_NONE: SDL_PackedLayout = SDL_PackedLayout::NONE;
pub const SDL_PACKEDLAYOUT_332: SDL_PackedLayout = SDL_PackedLayout(1);
pub const SDL_PACKEDLAYOUT_4444: SDL_PackedLayout = SDL_PackedLayout(2);
pub const SDL_PACKEDLAYOUT_1555: SDL_PackedLayout = SDL_PackedLayout(3);
pub const SDL_PACKEDLAYOUT_5551: SDL_PackedLayout = SDL_PackedLayout(4);
pub const SDL_PACKEDLAYOUT_565: SDL_PackedLayout = SDL_PackedLayout(5);
pub const SDL_PACKEDLAYOUT_8888: SDL_PackedLayout = SDL_PackedLayout(6);
pub const SDL_PACKEDLAYOUT_2101010: SDL_PackedLayout = SDL_PackedLayout(7);
pub const SDL_PACKEDLAYOUT_1010102: SDL_PackedLayout = SDL_PackedLayout(8);

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
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PIXELFORMAT_UNKNOWN`], [`SDL_PIXELFORMAT_INDEX1LSB`], [`SDL_PIXELFORMAT_INDEX1MSB`], [`SDL_PIXELFORMAT_INDEX2LSB`], [`SDL_PIXELFORMAT_INDEX2MSB`], [`SDL_PIXELFORMAT_INDEX4LSB`], [`SDL_PIXELFORMAT_INDEX4MSB`], [`SDL_PIXELFORMAT_INDEX8`], [`SDL_PIXELFORMAT_RGB332`], [`SDL_PIXELFORMAT_XRGB4444`], [`SDL_PIXELFORMAT_XBGR4444`], [`SDL_PIXELFORMAT_XRGB1555`], [`SDL_PIXELFORMAT_XBGR1555`], [`SDL_PIXELFORMAT_ARGB4444`], [`SDL_PIXELFORMAT_RGBA4444`], [`SDL_PIXELFORMAT_ABGR4444`], [`SDL_PIXELFORMAT_BGRA4444`], [`SDL_PIXELFORMAT_ARGB1555`], [`SDL_PIXELFORMAT_RGBA5551`], [`SDL_PIXELFORMAT_ABGR1555`], [`SDL_PIXELFORMAT_BGRA5551`], [`SDL_PIXELFORMAT_RGB565`], [`SDL_PIXELFORMAT_BGR565`], [`SDL_PIXELFORMAT_RGB24`], [`SDL_PIXELFORMAT_BGR24`], [`SDL_PIXELFORMAT_XRGB8888`], [`SDL_PIXELFORMAT_RGBX8888`], [`SDL_PIXELFORMAT_XBGR8888`], [`SDL_PIXELFORMAT_BGRX8888`], [`SDL_PIXELFORMAT_ARGB8888`], [`SDL_PIXELFORMAT_RGBA8888`], [`SDL_PIXELFORMAT_ABGR8888`], [`SDL_PIXELFORMAT_BGRA8888`], [`SDL_PIXELFORMAT_XRGB2101010`], [`SDL_PIXELFORMAT_XBGR2101010`], [`SDL_PIXELFORMAT_ARGB2101010`], [`SDL_PIXELFORMAT_ABGR2101010`], [`SDL_PIXELFORMAT_RGB48`], [`SDL_PIXELFORMAT_BGR48`], [`SDL_PIXELFORMAT_RGBA64`], [`SDL_PIXELFORMAT_ARGB64`], [`SDL_PIXELFORMAT_BGRA64`], [`SDL_PIXELFORMAT_ABGR64`], [`SDL_PIXELFORMAT_RGB48_FLOAT`], [`SDL_PIXELFORMAT_BGR48_FLOAT`], [`SDL_PIXELFORMAT_RGBA64_FLOAT`], [`SDL_PIXELFORMAT_ARGB64_FLOAT`], [`SDL_PIXELFORMAT_BGRA64_FLOAT`], [`SDL_PIXELFORMAT_ABGR64_FLOAT`], [`SDL_PIXELFORMAT_RGB96_FLOAT`], [`SDL_PIXELFORMAT_BGR96_FLOAT`], [`SDL_PIXELFORMAT_RGBA128_FLOAT`], [`SDL_PIXELFORMAT_ARGB128_FLOAT`], [`SDL_PIXELFORMAT_BGRA128_FLOAT`], [`SDL_PIXELFORMAT_ABGR128_FLOAT`], [`SDL_PIXELFORMAT_YV12`], [`SDL_PIXELFORMAT_IYUV`], [`SDL_PIXELFORMAT_YUY2`], [`SDL_PIXELFORMAT_UYVY`], [`SDL_PIXELFORMAT_YVYU`], [`SDL_PIXELFORMAT_NV12`], [`SDL_PIXELFORMAT_NV21`], [`SDL_PIXELFORMAT_P010`], [`SDL_PIXELFORMAT_EXTERNAL_OES`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PixelFormat(pub ::core::ffi::c_int);
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
emit! {
}

#[cfg(not(target_endian = "big"))]
emit! {
}

/// Colorspace color type.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_COLOR_TYPE_UNKNOWN`], [`SDL_COLOR_TYPE_RGB`], [`SDL_COLOR_TYPE_YCBCR`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorType(pub ::core::ffi::c_int);
impl SDL_ColorType {
    pub const UNKNOWN: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const YCBCR: Self = Self(2);
}
pub const SDL_COLOR_TYPE_UNKNOWN: SDL_ColorType = SDL_ColorType::UNKNOWN;
pub const SDL_COLOR_TYPE_RGB: SDL_ColorType = SDL_ColorType::RGB;
pub const SDL_COLOR_TYPE_YCBCR: SDL_ColorType = SDL_ColorType::YCBCR;

/// Colorspace color range, as described by
/// https://www.itu.int/rec/R-REC-BT.2100-2-201807-I/en
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_COLOR_RANGE_UNKNOWN`], [`SDL_COLOR_RANGE_LIMITED`], [`SDL_COLOR_RANGE_FULL`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorRange(pub ::core::ffi::c_int);
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
/// https://www.itu.int/rec/T-REC-H.273-201612-S/en
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_COLOR_PRIMARIES_UNKNOWN`], [`SDL_COLOR_PRIMARIES_BT709`], [`SDL_COLOR_PRIMARIES_UNSPECIFIED`], [`SDL_COLOR_PRIMARIES_BT470M`], [`SDL_COLOR_PRIMARIES_BT470BG`], [`SDL_COLOR_PRIMARIES_BT601`], [`SDL_COLOR_PRIMARIES_SMPTE240`], [`SDL_COLOR_PRIMARIES_GENERIC_FILM`], [`SDL_COLOR_PRIMARIES_BT2020`], [`SDL_COLOR_PRIMARIES_XYZ`], [`SDL_COLOR_PRIMARIES_SMPTE431`], [`SDL_COLOR_PRIMARIES_SMPTE432`], [`SDL_COLOR_PRIMARIES_EBU3213`], [`SDL_COLOR_PRIMARIES_CUSTOM`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ColorPrimaries(pub ::core::ffi::c_int);
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
    /// SMPTE 240M, functionally the same as SDL_COLOR_PRIMARIES_BT601
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
/// SMPTE 240M, functionally the same as SDL_COLOR_PRIMARIES_BT601
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
/// These are as described by https://www.itu.int/rec/T-REC-H.273-201612-S/en
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_TRANSFER_CHARACTERISTICS_UNKNOWN`], [`SDL_TRANSFER_CHARACTERISTICS_BT709`], [`SDL_TRANSFER_CHARACTERISTICS_UNSPECIFIED`], [`SDL_TRANSFER_CHARACTERISTICS_GAMMA22`], [`SDL_TRANSFER_CHARACTERISTICS_GAMMA28`], [`SDL_TRANSFER_CHARACTERISTICS_BT601`], [`SDL_TRANSFER_CHARACTERISTICS_SMPTE240`], [`SDL_TRANSFER_CHARACTERISTICS_LINEAR`], [`SDL_TRANSFER_CHARACTERISTICS_LOG100`], [`SDL_TRANSFER_CHARACTERISTICS_LOG100_SQRT10`], [`SDL_TRANSFER_CHARACTERISTICS_IEC61966`], [`SDL_TRANSFER_CHARACTERISTICS_BT1361`], [`SDL_TRANSFER_CHARACTERISTICS_SRGB`], [`SDL_TRANSFER_CHARACTERISTICS_BT2020_10BIT`], [`SDL_TRANSFER_CHARACTERISTICS_BT2020_12BIT`], [`SDL_TRANSFER_CHARACTERISTICS_PQ`], [`SDL_TRANSFER_CHARACTERISTICS_SMPTE428`], [`SDL_TRANSFER_CHARACTERISTICS_HLG`], [`SDL_TRANSFER_CHARACTERISTICS_CUSTOM`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_TransferCharacteristics(pub ::core::ffi::c_int);
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
pub const SDL_TRANSFER_CHARACTERISTICS_UNKNOWN: SDL_TransferCharacteristics = SDL_TransferCharacteristics::UNKNOWN;
/// Rec. ITU-R BT.709-6 / ITU-R BT1361
pub const SDL_TRANSFER_CHARACTERISTICS_BT709: SDL_TransferCharacteristics = SDL_TransferCharacteristics::BT709;
pub const SDL_TRANSFER_CHARACTERISTICS_UNSPECIFIED: SDL_TransferCharacteristics = SDL_TransferCharacteristics::UNSPECIFIED;
/// ITU-R BT.470-6 System M / ITU-R BT1700 625 PAL & SECAM
pub const SDL_TRANSFER_CHARACTERISTICS_GAMMA22: SDL_TransferCharacteristics = SDL_TransferCharacteristics::GAMMA22;
/// ITU-R BT.470-6 System B, G
pub const SDL_TRANSFER_CHARACTERISTICS_GAMMA28: SDL_TransferCharacteristics = SDL_TransferCharacteristics::GAMMA28;
/// SMPTE ST 170M / ITU-R BT.601-7 525 or 625
pub const SDL_TRANSFER_CHARACTERISTICS_BT601: SDL_TransferCharacteristics = SDL_TransferCharacteristics::BT601;
/// SMPTE ST 240M
pub const SDL_TRANSFER_CHARACTERISTICS_SMPTE240: SDL_TransferCharacteristics = SDL_TransferCharacteristics::SMPTE240;
pub const SDL_TRANSFER_CHARACTERISTICS_LINEAR: SDL_TransferCharacteristics = SDL_TransferCharacteristics::LINEAR;
pub const SDL_TRANSFER_CHARACTERISTICS_LOG100: SDL_TransferCharacteristics = SDL_TransferCharacteristics::LOG100;
pub const SDL_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: SDL_TransferCharacteristics = SDL_TransferCharacteristics::LOG100_SQRT10;
/// IEC 61966-2-4
pub const SDL_TRANSFER_CHARACTERISTICS_IEC61966: SDL_TransferCharacteristics = SDL_TransferCharacteristics::IEC61966;
/// ITU-R BT1361 Extended Colour Gamut
pub const SDL_TRANSFER_CHARACTERISTICS_BT1361: SDL_TransferCharacteristics = SDL_TransferCharacteristics::BT1361;
/// IEC 61966-2-1 (sRGB or sYCC)
pub const SDL_TRANSFER_CHARACTERISTICS_SRGB: SDL_TransferCharacteristics = SDL_TransferCharacteristics::SRGB;
/// ITU-R BT2020 for 10-bit system
pub const SDL_TRANSFER_CHARACTERISTICS_BT2020_10BIT: SDL_TransferCharacteristics = SDL_TransferCharacteristics::BT2020_10BIT;
/// ITU-R BT2020 for 12-bit system
pub const SDL_TRANSFER_CHARACTERISTICS_BT2020_12BIT: SDL_TransferCharacteristics = SDL_TransferCharacteristics::BT2020_12BIT;
/// SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems
pub const SDL_TRANSFER_CHARACTERISTICS_PQ: SDL_TransferCharacteristics = SDL_TransferCharacteristics::PQ;
/// SMPTE ST 428-1
pub const SDL_TRANSFER_CHARACTERISTICS_SMPTE428: SDL_TransferCharacteristics = SDL_TransferCharacteristics::SMPTE428;
/// ARIB STD-B67, known as "hybrid log-gamma" (HLG)
pub const SDL_TRANSFER_CHARACTERISTICS_HLG: SDL_TransferCharacteristics = SDL_TransferCharacteristics::HLG;
pub const SDL_TRANSFER_CHARACTERISTICS_CUSTOM: SDL_TransferCharacteristics = SDL_TransferCharacteristics::CUSTOM;

/// Colorspace matrix coefficients.
///
/// These are as described by https://www.itu.int/rec/T-REC-H.273-201612-S/en
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_MATRIX_COEFFICIENTS_IDENTITY`], [`SDL_MATRIX_COEFFICIENTS_BT709`], [`SDL_MATRIX_COEFFICIENTS_UNSPECIFIED`], [`SDL_MATRIX_COEFFICIENTS_FCC`], [`SDL_MATRIX_COEFFICIENTS_BT470BG`], [`SDL_MATRIX_COEFFICIENTS_BT601`], [`SDL_MATRIX_COEFFICIENTS_SMPTE240`], [`SDL_MATRIX_COEFFICIENTS_YCGCO`], [`SDL_MATRIX_COEFFICIENTS_BT2020_NCL`], [`SDL_MATRIX_COEFFICIENTS_BT2020_CL`], [`SDL_MATRIX_COEFFICIENTS_SMPTE2085`], [`SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL`], [`SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL`], [`SDL_MATRIX_COEFFICIENTS_ICTCP`], [`SDL_MATRIX_COEFFICIENTS_CUSTOM`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_MatrixCoefficients(pub ::core::ffi::c_int);
impl SDL_MatrixCoefficients {
    pub const IDENTITY: Self = Self(0);
    /// ITU-R BT.709-6
    pub const BT709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    /// US FCC Title 47
    pub const FCC: Self = Self(4);
    /// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625, functionally the same as SDL_MATRIX_COEFFICIENTS_BT601
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
pub const SDL_MATRIX_COEFFICIENTS_IDENTITY: SDL_MatrixCoefficients = SDL_MatrixCoefficients::IDENTITY;
/// ITU-R BT.709-6
pub const SDL_MATRIX_COEFFICIENTS_BT709: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT709;
pub const SDL_MATRIX_COEFFICIENTS_UNSPECIFIED: SDL_MatrixCoefficients = SDL_MatrixCoefficients::UNSPECIFIED;
/// US FCC Title 47
pub const SDL_MATRIX_COEFFICIENTS_FCC: SDL_MatrixCoefficients = SDL_MatrixCoefficients::FCC;
/// ITU-R BT.470-6 System B, G / ITU-R BT.601-7 625, functionally the same as SDL_MATRIX_COEFFICIENTS_BT601
pub const SDL_MATRIX_COEFFICIENTS_BT470BG: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT470BG;
/// ITU-R BT.601-7 525
pub const SDL_MATRIX_COEFFICIENTS_BT601: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT601;
/// SMPTE 240M
pub const SDL_MATRIX_COEFFICIENTS_SMPTE240: SDL_MatrixCoefficients = SDL_MatrixCoefficients::SMPTE240;
pub const SDL_MATRIX_COEFFICIENTS_YCGCO: SDL_MatrixCoefficients = SDL_MatrixCoefficients::YCGCO;
/// ITU-R BT.2020-2 non-constant luminance
pub const SDL_MATRIX_COEFFICIENTS_BT2020_NCL: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT2020_NCL;
/// ITU-R BT.2020-2 constant luminance
pub const SDL_MATRIX_COEFFICIENTS_BT2020_CL: SDL_MatrixCoefficients = SDL_MatrixCoefficients::BT2020_CL;
/// SMPTE ST 2085
pub const SDL_MATRIX_COEFFICIENTS_SMPTE2085: SDL_MatrixCoefficients = SDL_MatrixCoefficients::SMPTE2085;
pub const SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: SDL_MatrixCoefficients = SDL_MatrixCoefficients::CHROMA_DERIVED_NCL;
pub const SDL_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: SDL_MatrixCoefficients = SDL_MatrixCoefficients::CHROMA_DERIVED_CL;
/// ITU-R BT.2100-0 ICTCP
pub const SDL_MATRIX_COEFFICIENTS_ICTCP: SDL_MatrixCoefficients = SDL_MatrixCoefficients::ICTCP;
pub const SDL_MATRIX_COEFFICIENTS_CUSTOM: SDL_MatrixCoefficients = SDL_MatrixCoefficients::CUSTOM;

/// Colorspace chroma sample location.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_CHROMA_LOCATION_NONE`], [`SDL_CHROMA_LOCATION_LEFT`], [`SDL_CHROMA_LOCATION_CENTER`], [`SDL_CHROMA_LOCATION_TOPLEFT`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ChromaLocation(pub ::core::ffi::c_int);
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
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_ColorPrimaries
/// \sa SDL_ColorRange
/// \sa SDL_ColorType
/// \sa SDL_MatrixCoefficients
/// \sa SDL_TransferCharacteristics
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_COLORSPACE_UNKNOWN`], [`SDL_COLORSPACE_SRGB`], [`SDL_COLORSPACE_SRGB_LINEAR`], [`SDL_COLORSPACE_HDR10`], [`SDL_COLORSPACE_JPEG`], [`SDL_COLORSPACE_BT601_LIMITED`], [`SDL_COLORSPACE_BT601_FULL`], [`SDL_COLORSPACE_BT709_LIMITED`], [`SDL_COLORSPACE_BT709_FULL`], [`SDL_COLORSPACE_BT2020_LIMITED`], [`SDL_COLORSPACE_BT2020_FULL`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Colorspace(pub ::core::ffi::c_int);
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

/// A structure that represents a color as RGBA components.
///
/// The bits of this structure can be directly reinterpreted as an
/// integer-packed color which uses the SDL_PIXELFORMAT_RGBA32 format
/// (SDL_PIXELFORMAT_ABGR8888 on little-endian systems and
/// SDL_PIXELFORMAT_RGBA8888 on big-endian systems).
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}

/// The bits of this structure can be directly reinterpreted as a float-packed
/// color which uses the SDL_PIXELFORMAT_RGBA128_FLOAT format
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SDL_FColor {
    pub r: ::core::ffi::c_float,
    pub g: ::core::ffi::c_float,
    pub b: ::core::ffi::c_float,
    pub a: ::core::ffi::c_float,
}

/// A set of indexed colors representing a palette.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_SetPaletteColors
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

extern_sdlcall! {{
    /// Get the human readable name of a pixel format.
    ///
    /// \param format the pixel format to query.
    /// \returns the human readable name of the specified pixel format or
    ///          "SDL_PIXELFORMAT_UNKNOWN" if the format isn't recognized.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetPixelFormatName(format: SDL_PixelFormat) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Convert one of the enumerated pixel formats to a bpp value and RGBA masks.
    ///
    /// \param format one of the SDL_PixelFormat values.
    /// \param bpp a bits per pixel value; usually 15, 16, or 32.
    /// \param Rmask a pointer filled in with the red mask for the format.
    /// \param Gmask a pointer filled in with the green mask for the format.
    /// \param Bmask a pointer filled in with the blue mask for the format.
    /// \param Amask a pointer filled in with the alpha mask for the format.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPixelFormatForMasks
    pub fn SDL_GetMasksForPixelFormat(format: SDL_PixelFormat, bpp: *mut ::core::ffi::c_int, Rmask: *mut Uint32, Gmask: *mut Uint32, Bmask: *mut Uint32, Amask: *mut Uint32) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Convert a bpp value and RGBA masks to an enumerated pixel format.
    ///
    /// This will return `SDL_PIXELFORMAT_UNKNOWN` if the conversion wasn't
    /// possible.
    ///
    /// \param bpp a bits per pixel value; usually 15, 16, or 32.
    /// \param Rmask the red mask for the format.
    /// \param Gmask the green mask for the format.
    /// \param Bmask the blue mask for the format.
    /// \param Amask the alpha mask for the format.
    /// \returns the SDL_PixelFormat value corresponding to the format masks, or
    ///          SDL_PIXELFORMAT_UNKNOWN if there isn't a match.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetMasksForPixelFormat
    pub fn SDL_GetPixelFormatForMasks(bpp: ::core::ffi::c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32) -> SDL_PixelFormat;
}}

extern_sdlcall! {{
    /// Create an SDL_PixelFormatDetails structure corresponding to a pixel format.
    ///
    /// Returned structure may come from a shared global cache (i.e. not newly
    /// allocated), and hence should not be modified, especially the palette. Weird
    /// errors such as `Blit combination not supported` may occur.
    ///
    /// \param format one of the SDL_PixelFormat values.
    /// \returns a pointer to a SDL_PixelFormatDetails structure or NULL on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetPixelFormatDetails(format: SDL_PixelFormat) -> *const SDL_PixelFormatDetails;
}}

extern_sdlcall! {{
    /// Create a palette structure with the specified number of color entries.
    ///
    /// The palette entries are initialized to white.
    ///
    /// \param ncolors represents the number of color entries in the color palette.
    /// \returns a new SDL_Palette structure on success or NULL on failure (e.g. if
    ///          there wasn't enough memory); call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroyPalette
    /// \sa SDL_SetPaletteColors
    /// \sa SDL_SetSurfacePalette
    pub fn SDL_CreatePalette(ncolors: ::core::ffi::c_int) -> *mut SDL_Palette;
}}

extern_sdlcall! {{
    /// Set a range of colors in a palette.
    ///
    /// \param palette the SDL_Palette structure to modify.
    /// \param colors an array of SDL_Color structures to copy into the palette.
    /// \param firstcolor the index of the first palette entry to modify.
    /// \param ncolors the number of entries to modify.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified or destroyed in another thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetPaletteColors(palette: *mut SDL_Palette, colors: *const SDL_Color, firstcolor: ::core::ffi::c_int, ncolors: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Free a palette created with SDL_CreatePalette().
    ///
    /// \param palette the SDL_Palette structure to be freed.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified or destroyed in another thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreatePalette
    pub fn SDL_DestroyPalette(palette: *mut SDL_Palette);
}}

extern_sdlcall! {{
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
    /// \param format a pointer to SDL_PixelFormatDetails describing the pixel
    ///               format.
    /// \param palette an optional palette for indexed formats, may be NULL.
    /// \param r the red component of the pixel in the range 0-255.
    /// \param g the green component of the pixel in the range 0-255.
    /// \param b the blue component of the pixel in the range 0-255.
    /// \returns a pixel value.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPixelFormatDetails
    /// \sa SDL_GetRGB
    /// \sa SDL_MapRGBA
    /// \sa SDL_MapSurfaceRGB
    pub fn SDL_MapRGB(format: *const SDL_PixelFormatDetails, palette: *const SDL_Palette, r: Uint8, g: Uint8, b: Uint8) -> Uint32;
}}

extern_sdlcall! {{
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
    /// \param format a pointer to SDL_PixelFormatDetails describing the pixel
    ///               format.
    /// \param palette an optional palette for indexed formats, may be NULL.
    /// \param r the red component of the pixel in the range 0-255.
    /// \param g the green component of the pixel in the range 0-255.
    /// \param b the blue component of the pixel in the range 0-255.
    /// \param a the alpha component of the pixel in the range 0-255.
    /// \returns a pixel value.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPixelFormatDetails
    /// \sa SDL_GetRGBA
    /// \sa SDL_MapRGB
    /// \sa SDL_MapSurfaceRGBA
    pub fn SDL_MapRGBA(format: *const SDL_PixelFormatDetails, palette: *const SDL_Palette, r: Uint8, g: Uint8, b: Uint8, a: Uint8) -> Uint32;
}}

extern_sdlcall! {{
    /// Get RGB values from a pixel in the specified format.
    ///
    /// This function uses the entire 8-bit [0..255] range when converting color
    /// components from pixel formats with less than 8-bits per RGB component
    /// (e.g., a completely white pixel in 16-bit RGB565 format would return [0xff,
    /// 0xff, 0xff] not [0xf8, 0xfc, 0xf8]).
    ///
    /// \param pixel a pixel value.
    /// \param format a pointer to SDL_PixelFormatDetails describing the pixel
    ///               format.
    /// \param palette an optional palette for indexed formats, may be NULL.
    /// \param r a pointer filled in with the red component, may be NULL.
    /// \param g a pointer filled in with the green component, may be NULL.
    /// \param b a pointer filled in with the blue component, may be NULL.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPixelFormatDetails
    /// \sa SDL_GetRGBA
    /// \sa SDL_MapRGB
    /// \sa SDL_MapRGBA
    pub fn SDL_GetRGB(pixel: Uint32, format: *const SDL_PixelFormatDetails, palette: *const SDL_Palette, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8);
}}

extern_sdlcall! {{
    /// Get RGBA values from a pixel in the specified format.
    ///
    /// This function uses the entire 8-bit [0..255] range when converting color
    /// components from pixel formats with less than 8-bits per RGB component
    /// (e.g., a completely white pixel in 16-bit RGB565 format would return [0xff,
    /// 0xff, 0xff] not [0xf8, 0xfc, 0xf8]).
    ///
    /// If the surface has no alpha component, the alpha will be returned as 0xff
    /// (100% opaque).
    ///
    /// \param pixel a pixel value.
    /// \param format a pointer to SDL_PixelFormatDetails describing the pixel
    ///               format.
    /// \param palette an optional palette for indexed formats, may be NULL.
    /// \param r a pointer filled in with the red component, may be NULL.
    /// \param g a pointer filled in with the green component, may be NULL.
    /// \param b a pointer filled in with the blue component, may be NULL.
    /// \param a a pointer filled in with the alpha component, may be NULL.
    ///
    /// \threadsafety It is safe to call this function from any thread, as long as
    ///               the palette is not modified.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPixelFormatDetails
    /// \sa SDL_GetRGB
    /// \sa SDL_MapRGB
    /// \sa SDL_MapRGBA
    pub fn SDL_GetRGBA(pixel: Uint32, format: *const SDL_PixelFormatDetails, palette: *const SDL_Palette, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8, a: *mut Uint8);
}}

