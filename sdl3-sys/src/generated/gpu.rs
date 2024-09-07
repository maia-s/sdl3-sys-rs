#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryGPU
//!
//! Include file for SDL GPU API functions

use super::stdinc::*;

use super::pixels::*;

use super::properties::*;

use super::rect::*;

use super::surface::*;

use super::video::*;

/// Specifies the primitive topology of a graphics pipeline.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_PRIMITIVETYPE_POINTLIST`], [`SDL_GPU_PRIMITIVETYPE_LINELIST`], [`SDL_GPU_PRIMITIVETYPE_LINESTRIP`], [`SDL_GPU_PRIMITIVETYPE_TRIANGLELIST`], [`SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUPrimitiveType(pub ::core::ffi::c_int);
impl SDL_GPUPrimitiveType {
    /// A series of separate points.
    pub const POINTLIST: Self = Self(0);
    /// A series of separate lines.
    pub const LINELIST: Self = Self(1);
    /// A series of connected lines.
    pub const LINESTRIP: Self = Self(2);
    /// A series of separate triangles.
    pub const TRIANGLELIST: Self = Self(3);
    /// A series of connected triangles.
    pub const TRIANGLESTRIP: Self = Self(4);
}
/// A series of separate points.
pub const SDL_GPU_PRIMITIVETYPE_POINTLIST: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::POINTLIST;
/// A series of separate lines.
pub const SDL_GPU_PRIMITIVETYPE_LINELIST: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::LINELIST;
/// A series of connected lines.
pub const SDL_GPU_PRIMITIVETYPE_LINESTRIP: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::LINESTRIP;
/// A series of separate triangles.
pub const SDL_GPU_PRIMITIVETYPE_TRIANGLELIST: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::TRIANGLELIST;
/// A series of connected triangles.
pub const SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::TRIANGLESTRIP;

/// Specifies how the contents of a texture attached to a render pass are
/// treated at the beginning of the render pass.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_BeginGPURenderPass
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_LOADOP_LOAD`], [`SDL_GPU_LOADOP_CLEAR`], [`SDL_GPU_LOADOP_DONT_CARE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPULoadOp(pub ::core::ffi::c_int);
impl SDL_GPULoadOp {
    /// The previous contents of the texture will be preserved.
    pub const LOAD: Self = Self(0);
    /// The contents of the texture will be cleared to a color.
    pub const CLEAR: Self = Self(1);
    /// The previous contents of the texture need not be preserved. The contents will be undefined.
    pub const DONT_CARE: Self = Self(2);
}
/// The previous contents of the texture will be preserved.
pub const SDL_GPU_LOADOP_LOAD: SDL_GPULoadOp = SDL_GPULoadOp::LOAD;
/// The contents of the texture will be cleared to a color.
pub const SDL_GPU_LOADOP_CLEAR: SDL_GPULoadOp = SDL_GPULoadOp::CLEAR;
/// The previous contents of the texture need not be preserved. The contents will be undefined.
pub const SDL_GPU_LOADOP_DONT_CARE: SDL_GPULoadOp = SDL_GPULoadOp::DONT_CARE;

/// Specifies how the contents of a texture attached to a render pass are
/// treated at the end of the render pass.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_BeginGPURenderPass
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_STOREOP_STORE`], [`SDL_GPU_STOREOP_DONT_CARE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStoreOp(pub ::core::ffi::c_int);
impl SDL_GPUStoreOp {
    /// The contents generated during the render pass will be written to memory.
    pub const STORE: Self = Self(0);
    /// The contents generated during the render pass are not needed and may be discarded. The contents will be undefined.
    pub const DONT_CARE: Self = Self(1);
}
/// The contents generated during the render pass will be written to memory.
pub const SDL_GPU_STOREOP_STORE: SDL_GPUStoreOp = SDL_GPUStoreOp::STORE;
/// The contents generated during the render pass are not needed and may be discarded. The contents will be undefined.
pub const SDL_GPU_STOREOP_DONT_CARE: SDL_GPUStoreOp = SDL_GPUStoreOp::DONT_CARE;

/// Specifies the size of elements in an index buffer.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_INDEXELEMENTSIZE_16BIT`], [`SDL_GPU_INDEXELEMENTSIZE_32BIT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndexElementSize(pub ::core::ffi::c_int);
impl SDL_GPUIndexElementSize {
}
/// The index elements are 16-bit.
pub const SDL_GPU_INDEXELEMENTSIZE_16BIT: SDL_GPUIndexElementSize = SDL_GPUIndexElementSize(0);
/// The index elements are 32-bit.
pub const SDL_GPU_INDEXELEMENTSIZE_32BIT: SDL_GPUIndexElementSize = SDL_GPUIndexElementSize(1);

/// Specifies the pixel format of a texture.
///
/// Texture format support varies depending on driver, hardware, and usage
/// flags. In general, you should use SDL_GPUTextureSupportsFormat to query if
/// a format is supported before using it. However, there are a few guaranteed
/// formats.
///
/// For SAMPLER usage, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - B8G8R8A8_UNORM
/// - R8_UNORM
/// - R8_SNORM
/// - R8G8_UNORM
/// - R8G8_SNORM
/// - R8G8B8A8_SNORM
/// - R16_FLOAT
/// - R16G16_FLOAT
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R11G11B10_UFLOAT
/// - R8G8B8A8_UNORM_SRGB
/// - B8G8R8A8_UNORM_SRGB
/// - D16_UNORM
///
/// For COLOR_TARGET usage, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - B8G8R8A8_UNORM
/// - R8_UNORM
/// - R16_FLOAT
/// - R16G16_FLOAT
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R8_UINT
/// - R8G8_UINT
/// - R8G8B8A8_UINT
/// - R16_UINT
/// - R16G16_UINT
/// - R16G16B16A16_UINT
/// - R8_INT
/// - R8G8_INT
/// - R8G8B8A8_INT
/// - R16_INT
/// - R16G16_INT
/// - R16G16B16A16_INT
/// - R8G8B8A8_UNORM_SRGB
/// - B8G8R8A8_UNORM_SRGB
///
/// For STORAGE usages, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - R8G8B8A8_SNORM
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R8G8B8A8_UINT
/// - R16G16B16A16_UINT
/// - R8G8B8A8_INT
/// - R16G16B16A16_INT
///
/// For DEPTH_STENCIL_TARGET usage, the following formats are universally
/// supported:
///
/// - D16_UNORM
/// - Either (but not necessarily both!) D24_UNORM or D32_SFLOAT
/// - Either (but not necessarily both!) D24_UNORM_S8_UINT or
///   D32_SFLOAT_S8_UINT
///
/// Unless D16_UNORM is sufficient for your purposes, always check which of
/// D24/D32 is supported before creating a depth-stencil texture!
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTexture
/// \sa SDL_GPUTextureSupportsFormat
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_TEXTUREFORMAT_INVALID`], [`SDL_GPU_TEXTUREFORMAT_A8_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R8_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R8G8_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R16_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R16G16_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM`], [`SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM`], [`SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM`], [`SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM`], [`SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM`], [`SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM`], [`SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT`], [`SDL_GPU_TEXTUREFORMAT_R8_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R8G8_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R16_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R16G16_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM`], [`SDL_GPU_TEXTUREFORMAT_R16_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R32_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT`], [`SDL_GPU_TEXTUREFORMAT_R8_UINT`], [`SDL_GPU_TEXTUREFORMAT_R8G8_UINT`], [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT`], [`SDL_GPU_TEXTUREFORMAT_R16_UINT`], [`SDL_GPU_TEXTUREFORMAT_R16G16_UINT`], [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT`], [`SDL_GPU_TEXTUREFORMAT_R8_INT`], [`SDL_GPU_TEXTUREFORMAT_R8G8_INT`], [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT`], [`SDL_GPU_TEXTUREFORMAT_R16_INT`], [`SDL_GPU_TEXTUREFORMAT_R16G16_INT`], [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT`], [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB`], [`SDL_GPU_TEXTUREFORMAT_D16_UNORM`], [`SDL_GPU_TEXTUREFORMAT_D24_UNORM`], [`SDL_GPU_TEXTUREFORMAT_D32_FLOAT`], [`SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT`], [`SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureFormat(pub ::core::ffi::c_int);
impl SDL_GPUTextureFormat {
    pub const INVALID: Self = Self(-1_i32);
    pub const A8_UNORM: Self = Self(0_i32);
    pub const R8_UNORM: Self = Self(1_i32);
    pub const R8G8_UNORM: Self = Self(2_i32);
    pub const R8G8B8A8_UNORM: Self = Self(3_i32);
    pub const R16_UNORM: Self = Self(4_i32);
    pub const R16G16_UNORM: Self = Self(5_i32);
    pub const R16G16B16A16_UNORM: Self = Self(6_i32);
    pub const R10G10B10A2_UNORM: Self = Self(7_i32);
    pub const B5G6R5_UNORM: Self = Self(8_i32);
    pub const B5G5R5A1_UNORM: Self = Self(9_i32);
    pub const B4G4R4A4_UNORM: Self = Self(10_i32);
    pub const B8G8R8A8_UNORM: Self = Self(11_i32);
    pub const BC1_RGBA_UNORM: Self = Self(12_i32);
    pub const BC2_RGBA_UNORM: Self = Self(13_i32);
    pub const BC3_RGBA_UNORM: Self = Self(14_i32);
    pub const BC4_R_UNORM: Self = Self(15_i32);
    pub const BC5_RG_UNORM: Self = Self(16_i32);
    pub const BC7_RGBA_UNORM: Self = Self(17_i32);
    pub const BC6H_RGB_FLOAT: Self = Self(18_i32);
    pub const BC6H_RGB_UFLOAT: Self = Self(19_i32);
    pub const R8_SNORM: Self = Self(20_i32);
    pub const R8G8_SNORM: Self = Self(21_i32);
    pub const R8G8B8A8_SNORM: Self = Self(22_i32);
    pub const R16_SNORM: Self = Self(23_i32);
    pub const R16G16_SNORM: Self = Self(24_i32);
    pub const R16G16B16A16_SNORM: Self = Self(25_i32);
    pub const R16_FLOAT: Self = Self(26_i32);
    pub const R16G16_FLOAT: Self = Self(27_i32);
    pub const R16G16B16A16_FLOAT: Self = Self(28_i32);
    pub const R32_FLOAT: Self = Self(29_i32);
    pub const R32G32_FLOAT: Self = Self(30_i32);
    pub const R32G32B32A32_FLOAT: Self = Self(31_i32);
    pub const R11G11B10_UFLOAT: Self = Self(32_i32);
    pub const R8_UINT: Self = Self(33_i32);
    pub const R8G8_UINT: Self = Self(34_i32);
    pub const R8G8B8A8_UINT: Self = Self(35_i32);
    pub const R16_UINT: Self = Self(36_i32);
    pub const R16G16_UINT: Self = Self(37_i32);
    pub const R16G16B16A16_UINT: Self = Self(38_i32);
    pub const R8_INT: Self = Self(39_i32);
    pub const R8G8_INT: Self = Self(40_i32);
    pub const R8G8B8A8_INT: Self = Self(41_i32);
    pub const R16_INT: Self = Self(42_i32);
    pub const R16G16_INT: Self = Self(43_i32);
    pub const R16G16B16A16_INT: Self = Self(44_i32);
    pub const R8G8B8A8_UNORM_SRGB: Self = Self(45_i32);
    pub const B8G8R8A8_UNORM_SRGB: Self = Self(46_i32);
    pub const BC1_RGBA_UNORM_SRGB: Self = Self(47_i32);
    pub const BC2_RGBA_UNORM_SRGB: Self = Self(48_i32);
    pub const BC3_RGBA_UNORM_SRGB: Self = Self(49_i32);
    pub const BC7_RGBA_UNORM_SRGB: Self = Self(50_i32);
    pub const D16_UNORM: Self = Self(51_i32);
    pub const D24_UNORM: Self = Self(52_i32);
    pub const D32_FLOAT: Self = Self(53_i32);
    pub const D24_UNORM_S8_UINT: Self = Self(54_i32);
    pub const D32_FLOAT_S8_UINT: Self = Self(55_i32);
}
pub const SDL_GPU_TEXTUREFORMAT_INVALID: SDL_GPUTextureFormat = SDL_GPUTextureFormat::INVALID;
pub const SDL_GPU_TEXTUREFORMAT_A8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8B8A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16B16A16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R10G10B10A2_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::B5G6R5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::B5G5R5A1_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::B4G4R4A4_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::B8G8R8A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC1_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC2_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC3_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC4_R_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC5_RG_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC7_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC6H_RGB_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC6H_RGB_UFLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R8_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8B8A8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16B16A16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16B16A16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32G32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32G32B32A32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R11G11B10_UFLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8B8A8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16B16A16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8B8A8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16B16A16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8B8A8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::B8G8R8A8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC1_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC2_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC3_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB: SDL_GPUTextureFormat = SDL_GPUTextureFormat::BC7_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_D16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_D24_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D24_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_D32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D24_UNORM_S8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D32_FLOAT_S8_UINT;

/// Specifies how a texture is intended to be used by the client.
///
/// A texture must have at least one usage flag. Note that some usage flag
/// combinations are invalid.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTexture
pub type SDL_GPUTextureUsageFlags = Uint32;

/// Texture supports sampling.
pub const SDL_GPU_TEXTUREUSAGE_SAMPLER: ::core::primitive::u32 = 1_u32;

/// Texture is a color render target.
pub const SDL_GPU_TEXTUREUSAGE_COLOR_TARGET: ::core::primitive::u32 = 2_u32;

/// Texture is a depth stencil target.
pub const SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET: ::core::primitive::u32 = 4_u32;

/// Texture supports storage reads in graphics stages.
pub const SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ: ::core::primitive::u32 = 8_u32;

/// Texture supports storage reads in the compute stage.
pub const SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ: ::core::primitive::u32 = 16_u32;

/// Texture supports storage writes in the compute stage.
pub const SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_WRITE: ::core::primitive::u32 = 32_u32;

/// Specifies the type of a texture.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTexture
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_TEXTURETYPE_2D`], [`SDL_GPU_TEXTURETYPE_2D_ARRAY`], [`SDL_GPU_TEXTURETYPE_3D`], [`SDL_GPU_TEXTURETYPE_CUBE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureType(pub ::core::ffi::c_int);
impl SDL_GPUTextureType {
    /// The texture is a cube image.
    pub const CUBE: Self = Self(3);
}
/// The texture is a 2-dimensional image.
pub const SDL_GPU_TEXTURETYPE_2D: SDL_GPUTextureType = SDL_GPUTextureType(0);
/// The texture is a 2-dimensional array image.
pub const SDL_GPU_TEXTURETYPE_2D_ARRAY: SDL_GPUTextureType = SDL_GPUTextureType(1);
/// The texture is a 3-dimensional image.
pub const SDL_GPU_TEXTURETYPE_3D: SDL_GPUTextureType = SDL_GPUTextureType(2);
/// The texture is a cube image.
pub const SDL_GPU_TEXTURETYPE_CUBE: SDL_GPUTextureType = SDL_GPUTextureType::CUBE;

/// Specifies the sample count of a texture.
///
/// Used in multisampling. Note that this value only applies when the texture
/// is used as a render target.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTexture
/// \sa SDL_GPUTextureSupportsSampleCount
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_SAMPLECOUNT_1`], [`SDL_GPU_SAMPLECOUNT_2`], [`SDL_GPU_SAMPLECOUNT_4`], [`SDL_GPU_SAMPLECOUNT_8`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSampleCount(pub ::core::ffi::c_int);
impl SDL_GPUSampleCount {
}
/// No multisampling.
pub const SDL_GPU_SAMPLECOUNT_1: SDL_GPUSampleCount = SDL_GPUSampleCount(0);
/// MSAA 2x
pub const SDL_GPU_SAMPLECOUNT_2: SDL_GPUSampleCount = SDL_GPUSampleCount(1);
/// MSAA 4x
pub const SDL_GPU_SAMPLECOUNT_4: SDL_GPUSampleCount = SDL_GPUSampleCount(2);
/// MSAA 8x
pub const SDL_GPU_SAMPLECOUNT_8: SDL_GPUSampleCount = SDL_GPUSampleCount(3);

/// Specifies the face of a cube map.
///
/// Can be passed in as the layer field in texture-related structs.
///
/// \since This enum is available since SDL 3.0.0
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_CUBEMAPFACE_POSITIVEX`], [`SDL_GPU_CUBEMAPFACE_NEGATIVEX`], [`SDL_GPU_CUBEMAPFACE_POSITIVEY`], [`SDL_GPU_CUBEMAPFACE_NEGATIVEY`], [`SDL_GPU_CUBEMAPFACE_POSITIVEZ`], [`SDL_GPU_CUBEMAPFACE_NEGATIVEZ`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUCubeMapFace(pub ::core::ffi::c_int);
impl SDL_GPUCubeMapFace {
    pub const POSITIVEX: Self = Self(0);
    pub const NEGATIVEX: Self = Self(1);
    pub const POSITIVEY: Self = Self(2);
    pub const NEGATIVEY: Self = Self(3);
    pub const POSITIVEZ: Self = Self(4);
    pub const NEGATIVEZ: Self = Self(5);
}
pub const SDL_GPU_CUBEMAPFACE_POSITIVEX: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEX;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEX: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEX;
pub const SDL_GPU_CUBEMAPFACE_POSITIVEY: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEY;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEY: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEY;
pub const SDL_GPU_CUBEMAPFACE_POSITIVEZ: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEZ;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEZ: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEZ;

/// Specifies how a buffer is intended to be used by the client.
///
/// A buffer must have at least one usage flag. Note that some usage flag
/// combinations are invalid.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUBuffer
pub type SDL_GPUBufferUsageFlags = Uint32;

/// Buffer is a vertex buffer.
pub const SDL_GPU_BUFFERUSAGE_VERTEX: ::core::primitive::u32 = 1_u32;

/// Buffer is an index buffer.
pub const SDL_GPU_BUFFERUSAGE_INDEX: ::core::primitive::u32 = 2_u32;

/// Buffer is an indirect buffer.
pub const SDL_GPU_BUFFERUSAGE_INDIRECT: ::core::primitive::u32 = 4_u32;

/// Buffer supports storage reads in graphics stages.
pub const SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ: ::core::primitive::u32 = 8_u32;

/// Buffer supports storage reads in the compute stage.
pub const SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_READ: ::core::primitive::u32 = 16_u32;

/// Buffer supports storage writes in the compute stage.
pub const SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_WRITE: ::core::primitive::u32 = 32_u32;

/// Specifies how a transfer buffer is intended to be used by the client.
///
/// Note that mapping and copying FROM an upload transfer buffer or TO a
/// download transfer buffer is undefined behavior.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTransferBuffer
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD`], [`SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTransferBufferUsage(pub ::core::ffi::c_int);
impl SDL_GPUTransferBufferUsage {
    pub const UPLOAD: Self = Self(0);
    pub const DOWNLOAD: Self = Self(1);
}
pub const SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD: SDL_GPUTransferBufferUsage = SDL_GPUTransferBufferUsage::UPLOAD;
pub const SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD: SDL_GPUTransferBufferUsage = SDL_GPUTransferBufferUsage::DOWNLOAD;

/// Specifies which stage a shader program corresponds to.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUShader
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_SHADERSTAGE_VERTEX`], [`SDL_GPU_SHADERSTAGE_FRAGMENT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUShaderStage(pub ::core::ffi::c_int);
impl SDL_GPUShaderStage {
    pub const VERTEX: Self = Self(0);
    pub const FRAGMENT: Self = Self(1);
}
pub const SDL_GPU_SHADERSTAGE_VERTEX: SDL_GPUShaderStage = SDL_GPUShaderStage::VERTEX;
pub const SDL_GPU_SHADERSTAGE_FRAGMENT: SDL_GPUShaderStage = SDL_GPUShaderStage::FRAGMENT;

/// Specifies the format of shader code.
///
/// Each format corresponds to a specific backend that accepts it.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUShader
pub type SDL_GPUShaderFormat = Uint32;

/// Shaders for NDA'd platforms.
pub const SDL_GPU_SHADERFORMAT_PRIVATE: ::core::primitive::u32 = 1_u32;

/// SPIR-V shaders for Vulkan.
pub const SDL_GPU_SHADERFORMAT_SPIRV: ::core::primitive::u32 = 2_u32;

/// DXBC SM5_0 shaders for D3D11.
pub const SDL_GPU_SHADERFORMAT_DXBC: ::core::primitive::u32 = 4_u32;

/// DXIL shaders for D3D12.
pub const SDL_GPU_SHADERFORMAT_DXIL: ::core::primitive::u32 = 8_u32;

/// MSL shaders for Metal.
pub const SDL_GPU_SHADERFORMAT_MSL: ::core::primitive::u32 = 16_u32;

/// Precompiled metallib shaders for Metal.
pub const SDL_GPU_SHADERFORMAT_METALLIB: ::core::primitive::u32 = 32_u32;

/// Specifies the format of a vertex attribute.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_VERTEXELEMENTFORMAT_INT`], [`SDL_GPU_VERTEXELEMENTFORMAT_INT2`], [`SDL_GPU_VERTEXELEMENTFORMAT_INT3`], [`SDL_GPU_VERTEXELEMENTFORMAT_INT4`], [`SDL_GPU_VERTEXELEMENTFORMAT_UINT`], [`SDL_GPU_VERTEXELEMENTFORMAT_UINT2`], [`SDL_GPU_VERTEXELEMENTFORMAT_UINT3`], [`SDL_GPU_VERTEXELEMENTFORMAT_UINT4`], [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT`], [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2`], [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3`], [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4`], [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE2`], [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE4`], [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2`], [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4`], [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT2`], [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT4`], [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT2`], [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT4`], [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM`], [`SDL_GPU_VERTEXELEMENTFORMAT_HALF2`], [`SDL_GPU_VERTEXELEMENTFORMAT_HALF4`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexElementFormat(pub ::core::ffi::c_int);
impl SDL_GPUVertexElementFormat {
    pub const INT: Self = Self(0);
    pub const INT2: Self = Self(1);
    pub const INT3: Self = Self(2);
    pub const INT4: Self = Self(3);
    pub const UINT: Self = Self(4);
    pub const UINT2: Self = Self(5);
    pub const UINT3: Self = Self(6);
    pub const UINT4: Self = Self(7);
    pub const FLOAT: Self = Self(8);
    pub const FLOAT2: Self = Self(9);
    pub const FLOAT3: Self = Self(10);
    pub const FLOAT4: Self = Self(11);
    pub const BYTE2: Self = Self(12);
    pub const BYTE4: Self = Self(13);
    pub const UBYTE2: Self = Self(14);
    pub const UBYTE4: Self = Self(15);
    pub const BYTE2_NORM: Self = Self(16);
    pub const BYTE4_NORM: Self = Self(17);
    pub const UBYTE2_NORM: Self = Self(18);
    pub const UBYTE4_NORM: Self = Self(19);
    pub const SHORT2: Self = Self(20);
    pub const SHORT4: Self = Self(21);
    pub const USHORT2: Self = Self(22);
    pub const USHORT4: Self = Self(23);
    pub const SHORT2_NORM: Self = Self(24);
    pub const SHORT4_NORM: Self = Self(25);
    pub const USHORT2_NORM: Self = Self(26);
    pub const USHORT4_NORM: Self = Self(27);
    pub const HALF2: Self = Self(28);
    pub const HALF4: Self = Self(29);
}
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::INT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::INT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT3: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::INT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::INT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UINT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UINT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT3: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UINT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UINT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::FLOAT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::FLOAT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::FLOAT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::FLOAT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::BYTE2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::BYTE4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UBYTE2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UBYTE4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::BYTE2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::BYTE4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UBYTE2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::UBYTE4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::SHORT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::SHORT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::USHORT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::USHORT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::SHORT2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::SHORT4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::USHORT2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::USHORT4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_HALF2: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::HALF2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_HALF4: SDL_GPUVertexElementFormat = SDL_GPUVertexElementFormat::HALF4;

/// Specifies the rate at which vertex attributes are pulled from buffers.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_VERTEXINPUTRATE_VERTEX`], [`SDL_GPU_VERTEXINPUTRATE_INSTANCE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexInputRate(pub ::core::ffi::c_int);
impl SDL_GPUVertexInputRate {
    /// Attribute addressing is a function of the vertex index.
    pub const VERTEX: Self = Self(0);
    /// Attribute addressing is a function of the instance index.
    pub const INSTANCE: Self = Self(1);
}
/// Attribute addressing is a function of the vertex index.
pub const SDL_GPU_VERTEXINPUTRATE_VERTEX: SDL_GPUVertexInputRate = SDL_GPUVertexInputRate::VERTEX;
/// Attribute addressing is a function of the instance index.
pub const SDL_GPU_VERTEXINPUTRATE_INSTANCE: SDL_GPUVertexInputRate = SDL_GPUVertexInputRate::INSTANCE;

/// Specifies the fill mode of the graphics pipeline.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_FILLMODE_FILL`], [`SDL_GPU_FILLMODE_LINE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUFillMode(pub ::core::ffi::c_int);
impl SDL_GPUFillMode {
    /// Polygons will be rendered via rasterization.
    pub const FILL: Self = Self(0);
    /// Polygon edges will be drawn as line segments.
    pub const LINE: Self = Self(1);
}
/// Polygons will be rendered via rasterization.
pub const SDL_GPU_FILLMODE_FILL: SDL_GPUFillMode = SDL_GPUFillMode::FILL;
/// Polygon edges will be drawn as line segments.
pub const SDL_GPU_FILLMODE_LINE: SDL_GPUFillMode = SDL_GPUFillMode::LINE;

/// Specifies the facing direction in which triangle faces will be culled.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_CULLMODE_NONE`], [`SDL_GPU_CULLMODE_FRONT`], [`SDL_GPU_CULLMODE_BACK`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUCullMode(pub ::core::ffi::c_int);
impl SDL_GPUCullMode {
    /// No triangles are culled.
    pub const NONE: Self = Self(0);
    /// Front-facing triangles are culled.
    pub const FRONT: Self = Self(1);
    /// Back-facing triangles are culled.
    pub const BACK: Self = Self(2);
}
/// No triangles are culled.
pub const SDL_GPU_CULLMODE_NONE: SDL_GPUCullMode = SDL_GPUCullMode::NONE;
/// Front-facing triangles are culled.
pub const SDL_GPU_CULLMODE_FRONT: SDL_GPUCullMode = SDL_GPUCullMode::FRONT;
/// Back-facing triangles are culled.
pub const SDL_GPU_CULLMODE_BACK: SDL_GPUCullMode = SDL_GPUCullMode::BACK;

/// Specifies the vertex winding that will cause a triangle to be determined to
/// be front-facing.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_FRONTFACE_COUNTER_CLOCKWISE`], [`SDL_GPU_FRONTFACE_CLOCKWISE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUFrontFace(pub ::core::ffi::c_int);
impl SDL_GPUFrontFace {
    /// A triangle with counter-clockwise vertex winding will be considered front-facing.
    pub const OUNTER_CLOCKWISE: Self = Self(0);
    /// A triangle with clockwise vertex winding will be considered front-facing.
    pub const LOCKWISE: Self = Self(1);
}
/// A triangle with counter-clockwise vertex winding will be considered front-facing.
pub const SDL_GPU_FRONTFACE_COUNTER_CLOCKWISE: SDL_GPUFrontFace = SDL_GPUFrontFace::OUNTER_CLOCKWISE;
/// A triangle with clockwise vertex winding will be considered front-facing.
pub const SDL_GPU_FRONTFACE_CLOCKWISE: SDL_GPUFrontFace = SDL_GPUFrontFace::LOCKWISE;

/// Specifies a comparison operator for depth, stencil and sampler operations.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_COMPAREOP_NEVER`], [`SDL_GPU_COMPAREOP_LESS`], [`SDL_GPU_COMPAREOP_EQUAL`], [`SDL_GPU_COMPAREOP_LESS_OR_EQUAL`], [`SDL_GPU_COMPAREOP_GREATER`], [`SDL_GPU_COMPAREOP_NOT_EQUAL`], [`SDL_GPU_COMPAREOP_GREATER_OR_EQUAL`], [`SDL_GPU_COMPAREOP_ALWAYS`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUCompareOp(pub ::core::ffi::c_int);
impl SDL_GPUCompareOp {
    /// The comparison always evaluates false.
    pub const NEVER: Self = Self(0);
    /// The comparison evaluates reference < test.
    pub const LESS: Self = Self(1);
    /// The comparison evaluates reference == test.
    pub const EQUAL: Self = Self(2);
    /// The comparison evaluates reference <= test.
    pub const LESS_OR_EQUAL: Self = Self(3);
    /// The comparison evaluates reference > test.
    pub const GREATER: Self = Self(4);
    /// The comparison evaluates reference != test.
    pub const NOT_EQUAL: Self = Self(5);
    /// The comparison evalutes reference >= test.
    pub const GREATER_OR_EQUAL: Self = Self(6);
    /// The comparison always evaluates true.
    pub const ALWAYS: Self = Self(7);
}
/// The comparison always evaluates false.
pub const SDL_GPU_COMPAREOP_NEVER: SDL_GPUCompareOp = SDL_GPUCompareOp::NEVER;
/// The comparison evaluates reference < test.
pub const SDL_GPU_COMPAREOP_LESS: SDL_GPUCompareOp = SDL_GPUCompareOp::LESS;
/// The comparison evaluates reference == test.
pub const SDL_GPU_COMPAREOP_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::EQUAL;
/// The comparison evaluates reference <= test.
pub const SDL_GPU_COMPAREOP_LESS_OR_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::LESS_OR_EQUAL;
/// The comparison evaluates reference > test.
pub const SDL_GPU_COMPAREOP_GREATER: SDL_GPUCompareOp = SDL_GPUCompareOp::GREATER;
/// The comparison evaluates reference != test.
pub const SDL_GPU_COMPAREOP_NOT_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::NOT_EQUAL;
/// The comparison evalutes reference >= test.
pub const SDL_GPU_COMPAREOP_GREATER_OR_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::GREATER_OR_EQUAL;
/// The comparison always evaluates true.
pub const SDL_GPU_COMPAREOP_ALWAYS: SDL_GPUCompareOp = SDL_GPUCompareOp::ALWAYS;

/// Specifies what happens to a stored stencil value if stencil tests fail or
/// pass.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_STENCILOP_KEEP`], [`SDL_GPU_STENCILOP_ZERO`], [`SDL_GPU_STENCILOP_REPLACE`], [`SDL_GPU_STENCILOP_INCREMENT_AND_CLAMP`], [`SDL_GPU_STENCILOP_DECREMENT_AND_CLAMP`], [`SDL_GPU_STENCILOP_INVERT`], [`SDL_GPU_STENCILOP_INCREMENT_AND_WRAP`], [`SDL_GPU_STENCILOP_DECREMENT_AND_WRAP`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStencilOp(pub ::core::ffi::c_int);
impl SDL_GPUStencilOp {
    /// Keeps the current value.
    pub const KEEP: Self = Self(0);
    /// Sets the value to 0.
    pub const ZERO: Self = Self(1);
    /// Sets the value to reference.
    pub const REPLACE: Self = Self(2);
    /// Increments the current value and clamps to the maximum value.
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    /// Decrements the current value and clamps to 0.
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    /// Bitwise-inverts the current value.
    pub const INVERT: Self = Self(5);
    /// Increments the current value and wraps back to 0.
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    /// Decrements the current value and wraps to the maximum value.
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
/// Keeps the current value.
pub const SDL_GPU_STENCILOP_KEEP: SDL_GPUStencilOp = SDL_GPUStencilOp::KEEP;
/// Sets the value to 0.
pub const SDL_GPU_STENCILOP_ZERO: SDL_GPUStencilOp = SDL_GPUStencilOp::ZERO;
/// Sets the value to reference.
pub const SDL_GPU_STENCILOP_REPLACE: SDL_GPUStencilOp = SDL_GPUStencilOp::REPLACE;
/// Increments the current value and clamps to the maximum value.
pub const SDL_GPU_STENCILOP_INCREMENT_AND_CLAMP: SDL_GPUStencilOp = SDL_GPUStencilOp::INCREMENT_AND_CLAMP;
/// Decrements the current value and clamps to 0.
pub const SDL_GPU_STENCILOP_DECREMENT_AND_CLAMP: SDL_GPUStencilOp = SDL_GPUStencilOp::DECREMENT_AND_CLAMP;
/// Bitwise-inverts the current value.
pub const SDL_GPU_STENCILOP_INVERT: SDL_GPUStencilOp = SDL_GPUStencilOp::INVERT;
/// Increments the current value and wraps back to 0.
pub const SDL_GPU_STENCILOP_INCREMENT_AND_WRAP: SDL_GPUStencilOp = SDL_GPUStencilOp::INCREMENT_AND_WRAP;
/// Decrements the current value and wraps to the maximum value.
pub const SDL_GPU_STENCILOP_DECREMENT_AND_WRAP: SDL_GPUStencilOp = SDL_GPUStencilOp::DECREMENT_AND_WRAP;

/// Specifies the operator to be used when pixels in a render target are
/// blended with existing pixels in the texture.
///
/// The source color is the value written by the fragment shader. The
/// destination color is the value currently existing in the texture.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_BLENDOP_ADD`], [`SDL_GPU_BLENDOP_SUBTRACT`], [`SDL_GPU_BLENDOP_REVERSE_SUBTRACT`], [`SDL_GPU_BLENDOP_MIN`], [`SDL_GPU_BLENDOP_MAX`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBlendOp(pub ::core::ffi::c_int);
impl SDL_GPUBlendOp {
    /// (source * source_factor) + (destination * destination_factor)
    pub const ADD: Self = Self(0);
    /// (source * source_factor) - (destination * destination_factor)
    pub const SUBTRACT: Self = Self(1);
    /// (destination * destination_factor) - (source * source_factor)
    pub const REVERSE_SUBTRACT: Self = Self(2);
    /// min(source, destination)
    pub const MIN: Self = Self(3);
    /// max(source, destination)
    pub const MAX: Self = Self(4);
}
/// (source * source_factor) + (destination * destination_factor)
pub const SDL_GPU_BLENDOP_ADD: SDL_GPUBlendOp = SDL_GPUBlendOp::ADD;
/// (source * source_factor) - (destination * destination_factor)
pub const SDL_GPU_BLENDOP_SUBTRACT: SDL_GPUBlendOp = SDL_GPUBlendOp::SUBTRACT;
/// (destination * destination_factor) - (source * source_factor)
pub const SDL_GPU_BLENDOP_REVERSE_SUBTRACT: SDL_GPUBlendOp = SDL_GPUBlendOp::REVERSE_SUBTRACT;
/// min(source, destination)
pub const SDL_GPU_BLENDOP_MIN: SDL_GPUBlendOp = SDL_GPUBlendOp::MIN;
/// max(source, destination)
pub const SDL_GPU_BLENDOP_MAX: SDL_GPUBlendOp = SDL_GPUBlendOp::MAX;

/// Specifies a blending factor to be used when pixels in a render target are
/// blended with existing pixels in the texture.
///
/// The source color is the value written by the fragment shader. The
/// destination color is the value currently existing in the texture.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_BLENDFACTOR_ZERO`], [`SDL_GPU_BLENDFACTOR_ONE`], [`SDL_GPU_BLENDFACTOR_SRC_COLOR`], [`SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR`], [`SDL_GPU_BLENDFACTOR_DST_COLOR`], [`SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR`], [`SDL_GPU_BLENDFACTOR_SRC_ALPHA`], [`SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA`], [`SDL_GPU_BLENDFACTOR_DST_ALPHA`], [`SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA`], [`SDL_GPU_BLENDFACTOR_CONSTANT_COLOR`], [`SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR`], [`SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBlendFactor(pub ::core::ffi::c_int);
impl SDL_GPUBlendFactor {
    /// 0
    pub const ZERO: Self = Self(0);
    /// 1
    pub const ONE: Self = Self(1);
    /// source color
    pub const SRC_COLOR: Self = Self(2);
    /// 1 - source color
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    /// destination color
    pub const DST_COLOR: Self = Self(4);
    /// 1 - destination color
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    /// source alpha
    pub const SRC_ALPHA: Self = Self(6);
    /// 1 - source alpha
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    /// destination alpha
    pub const DST_ALPHA: Self = Self(8);
    /// 1 - destination alpha
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    /// blend constant
    pub const CONSTANT_COLOR: Self = Self(10);
    /// 1 - blend constant
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    /// min(source alpha, 1 - destination alpha)
    pub const SRC_ALPHA_SATURATE: Self = Self(12);
}
/// 0
pub const SDL_GPU_BLENDFACTOR_ZERO: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ZERO;
/// 1
pub const SDL_GPU_BLENDFACTOR_ONE: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE;
/// source color
pub const SDL_GPU_BLENDFACTOR_SRC_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::SRC_COLOR;
/// 1 - source color
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE_MINUS_SRC_COLOR;
/// destination color
pub const SDL_GPU_BLENDFACTOR_DST_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::DST_COLOR;
/// 1 - destination color
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE_MINUS_DST_COLOR;
/// source alpha
pub const SDL_GPU_BLENDFACTOR_SRC_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::SRC_ALPHA;
/// 1 - source alpha
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA;
/// destination alpha
pub const SDL_GPU_BLENDFACTOR_DST_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::DST_ALPHA;
/// 1 - destination alpha
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE_MINUS_DST_ALPHA;
/// blend constant
pub const SDL_GPU_BLENDFACTOR_CONSTANT_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::CONSTANT_COLOR;
/// 1 - blend constant
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE_MINUS_CONSTANT_COLOR;
/// min(source alpha, 1 - destination alpha)
pub const SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE: SDL_GPUBlendFactor = SDL_GPUBlendFactor::SRC_ALPHA_SATURATE;

/// Specifies which color components are written in a graphics pipeline.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
pub type SDL_GPUColorComponentFlags = Uint8;

/// the red component
pub const SDL_GPU_COLORCOMPONENT_R: ::core::primitive::u32 = 1_u32;

/// the green component
pub const SDL_GPU_COLORCOMPONENT_G: ::core::primitive::u32 = 2_u32;

/// the blue component
pub const SDL_GPU_COLORCOMPONENT_B: ::core::primitive::u32 = 4_u32;

/// the alpha component
pub const SDL_GPU_COLORCOMPONENT_A: ::core::primitive::u32 = 8_u32;

/// Specifies a filter operation used by a sampler.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUSampler
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_FILTER_NEAREST`], [`SDL_GPU_FILTER_LINEAR`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUFilter(pub ::core::ffi::c_int);
impl SDL_GPUFilter {
    /// Point filtering.
    pub const NEAREST: Self = Self(0);
    /// Linear filtering.
    pub const LINEAR: Self = Self(1);
}
/// Point filtering.
pub const SDL_GPU_FILTER_NEAREST: SDL_GPUFilter = SDL_GPUFilter::NEAREST;
/// Linear filtering.
pub const SDL_GPU_FILTER_LINEAR: SDL_GPUFilter = SDL_GPUFilter::LINEAR;

/// Specifies a mipmap mode used by a sampler.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUSampler
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_SAMPLERMIPMAPMODE_NEAREST`], [`SDL_GPU_SAMPLERMIPMAPMODE_LINEAR`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSamplerMipmapMode(pub ::core::ffi::c_int);
impl SDL_GPUSamplerMipmapMode {
    /// Point filtering.
    pub const NEAREST: Self = Self(0);
    /// Linear filtering.
    pub const LINEAR: Self = Self(1);
}
/// Point filtering.
pub const SDL_GPU_SAMPLERMIPMAPMODE_NEAREST: SDL_GPUSamplerMipmapMode = SDL_GPUSamplerMipmapMode::NEAREST;
/// Linear filtering.
pub const SDL_GPU_SAMPLERMIPMAPMODE_LINEAR: SDL_GPUSamplerMipmapMode = SDL_GPUSamplerMipmapMode::LINEAR;

/// Specifies behavior of texture sampling when the coordinates exceed the 0-1
/// range.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUSampler
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_SAMPLERADDRESSMODE_REPEAT`], [`SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT`], [`SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSamplerAddressMode(pub ::core::ffi::c_int);
impl SDL_GPUSamplerAddressMode {
    /// Specifies that the coordinates will wrap around.
    pub const REPEAT: Self = Self(0);
    /// Specifies that the coordinates will wrap around mirrored.
    pub const MIRRORED_REPEAT: Self = Self(1);
    /// Specifies that the coordinates will clamp to the 0-1 range.
    pub const CLAMP_TO_EDGE: Self = Self(2);
}
/// Specifies that the coordinates will wrap around.
pub const SDL_GPU_SAMPLERADDRESSMODE_REPEAT: SDL_GPUSamplerAddressMode = SDL_GPUSamplerAddressMode::REPEAT;
/// Specifies that the coordinates will wrap around mirrored.
pub const SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT: SDL_GPUSamplerAddressMode = SDL_GPUSamplerAddressMode::MIRRORED_REPEAT;
/// Specifies that the coordinates will clamp to the 0-1 range.
pub const SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE: SDL_GPUSamplerAddressMode = SDL_GPUSamplerAddressMode::CLAMP_TO_EDGE;

/// Specifies the timing that will be used to present swapchain textures to the
/// OS.
///
/// Note that this value affects the behavior of
/// SDL_AcquireGPUSwapchainTexture. VSYNC mode will always be supported.
/// IMMEDIATE and MAILBOX modes may not be supported on certain systems.
///
/// It is recommended to query SDL_WindowSupportsGPUPresentMode after claiming
/// the window if you wish to change the present mode to IMMEDIATE or MAILBOX.
///
/// - VSYNC: Waits for vblank before presenting. No tearing is possible. If
///   there is a pending image to present, the new image is enqueued for
///   presentation. Disallows tearing at the cost of visual latency. When using
///   this present mode, AcquireSwapchainTexture will block if too many frames
///   are in flight.
/// - IMMEDIATE: Immediately presents. Lowest latency option, but tearing may
///   occur. When using this mode, AcquireSwapchainTexture will return NULL if
///   too many frames are in flight.
/// - MAILBOX: Waits for vblank before presenting. No tearing is possible. If
///   there is a pending image to present, the pending image is replaced by the
///   new image. Similar to VSYNC, but with reduced visual latency. When using
///   this mode, AcquireSwapchainTexture will return NULL if too many frames
///   are in flight.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_SetGPUSwapchainParameters
/// \sa SDL_WindowSupportsGPUPresentMode
/// \sa SDL_AcquireGPUSwapchainTexture
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_PRESENTMODE_VSYNC`], [`SDL_GPU_PRESENTMODE_IMMEDIATE`], [`SDL_GPU_PRESENTMODE_MAILBOX`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUPresentMode(pub ::core::ffi::c_int);
impl SDL_GPUPresentMode {
    pub const VSYNC: Self = Self(0);
    pub const IMMEDIATE: Self = Self(1);
    pub const MAILBOX: Self = Self(2);
}
pub const SDL_GPU_PRESENTMODE_VSYNC: SDL_GPUPresentMode = SDL_GPUPresentMode::VSYNC;
pub const SDL_GPU_PRESENTMODE_IMMEDIATE: SDL_GPUPresentMode = SDL_GPUPresentMode::IMMEDIATE;
pub const SDL_GPU_PRESENTMODE_MAILBOX: SDL_GPUPresentMode = SDL_GPUPresentMode::MAILBOX;

/// Specifies the texture format and colorspace of the swapchain textures.
///
/// SDR will always be supported. Other compositions may not be supported on
/// certain systems.
///
/// It is recommended to query SDL_WindowSupportsGPUSwapchainComposition after
/// claiming the window if you wish to change the swapchain composition from
/// SDR.
///
/// - SDR: B8G8R8A8 or R8G8B8A8 swapchain. Pixel values are in nonlinear sRGB
///   encoding.
/// - SDR_LINEAR: B8G8R8A8_SRGB or R8G8B8A8_SRGB swapchain. Pixel values are in
///   nonlinear sRGB encoding.
/// - HDR_EXTENDED_LINEAR: R16G16B16A16_SFLOAT swapchain. Pixel values are in
///   extended linear encoding.
/// - HDR10_ST2048: A2R10G10B10 or A2B10G10R10 swapchain. Pixel values are in
///   PQ ST2048 encoding.
///
/// \since This enum is available since SDL 3.0.0
///
/// \sa SDL_SetGPUSwapchainParameters
/// \sa SDL_WindowSupportsGPUSwapchainComposition
/// \sa SDL_AcquireGPUSwapchainTexture
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR`], [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR_LINEAR`], [`SDL_GPU_SWAPCHAINCOMPOSITION_HDR_EXTENDED_LINEAR`], [`SDL_GPU_SWAPCHAINCOMPOSITION_HDR10_ST2048`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSwapchainComposition(pub ::core::ffi::c_int);
impl SDL_GPUSwapchainComposition {
    pub const SDR: Self = Self(0);
    pub const SDR_LINEAR: Self = Self(1);
    pub const HDR_EXTENDED_LINEAR: Self = Self(2);
    pub const HDR10_ST2048: Self = Self(3);
}
pub const SDL_GPU_SWAPCHAINCOMPOSITION_SDR: SDL_GPUSwapchainComposition = SDL_GPUSwapchainComposition::SDR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_SDR_LINEAR: SDL_GPUSwapchainComposition = SDL_GPUSwapchainComposition::SDR_LINEAR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_HDR_EXTENDED_LINEAR: SDL_GPUSwapchainComposition = SDL_GPUSwapchainComposition::HDR_EXTENDED_LINEAR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_HDR10_ST2048: SDL_GPUSwapchainComposition = SDL_GPUSwapchainComposition::HDR10_ST2048;

/// Specifies a backend API supported by SDL_GPU.
///
/// Only one of these will be in use at a time.
///
/// \since This enum is available since SDL 3.0.0
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GPU_DRIVER_INVALID`], [`SDL_GPU_DRIVER_PRIVATE`], [`SDL_GPU_DRIVER_VULKAN`], [`SDL_GPU_DRIVER_D3D11`], [`SDL_GPU_DRIVER_D3D12`], [`SDL_GPU_DRIVER_METAL`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDriver(pub ::core::ffi::c_int);
impl SDL_GPUDriver {
    pub const INVALID: Self = Self(-1_i32);
    pub const PRIVATE: Self = Self(0_i32);
    pub const VULKAN: Self = Self(1_i32);
    pub const D3D11: Self = Self(2_i32);
    pub const D3D12: Self = Self(3_i32);
    pub const METAL: Self = Self(4_i32);
}
pub const SDL_GPU_DRIVER_INVALID: SDL_GPUDriver = SDL_GPUDriver::INVALID;
pub const SDL_GPU_DRIVER_PRIVATE: SDL_GPUDriver = SDL_GPUDriver::PRIVATE;
pub const SDL_GPU_DRIVER_VULKAN: SDL_GPUDriver = SDL_GPUDriver::VULKAN;
pub const SDL_GPU_DRIVER_D3D11: SDL_GPUDriver = SDL_GPUDriver::D3D11;
pub const SDL_GPU_DRIVER_D3D12: SDL_GPUDriver = SDL_GPUDriver::D3D12;
pub const SDL_GPU_DRIVER_METAL: SDL_GPUDriver = SDL_GPUDriver::METAL;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDepthStencilValue {
    pub depth: ::core::ffi::c_float,
    pub stencil: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUViewport {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
    pub h: ::core::ffi::c_float,
    pub min_depth: ::core::ffi::c_float,
    pub max_depth: ::core::ffi::c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureTransferInfo {
    pub transfer_buffer: *mut SDL_GPUTransferBuffer,
    pub offset: Uint32,
    pub pixels_per_row: Uint32,
    pub rows_per_layer: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTransferBufferLocation {
    pub transfer_buffer: *mut SDL_GPUTransferBuffer,
    pub offset: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureLocation {
    pub texture: *mut SDL_GPUTexture,
    pub mip_level: Uint32,
    pub layer: Uint32,
    pub x: Uint32,
    pub y: Uint32,
    pub z: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureRegion {
    pub texture: *mut SDL_GPUTexture,
    pub mip_level: Uint32,
    pub layer: Uint32,
    pub x: Uint32,
    pub y: Uint32,
    pub z: Uint32,
    pub w: Uint32,
    pub h: Uint32,
    pub d: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBlitRegion {
    pub texture: *mut SDL_GPUTexture,
    pub mip_level: Uint32,
    pub layer_or_depth_plane: Uint32,
    pub x: Uint32,
    pub y: Uint32,
    pub w: Uint32,
    pub h: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferLocation {
    pub buffer: *mut SDL_GPUBuffer,
    pub offset: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferRegion {
    pub buffer: *mut SDL_GPUBuffer,
    pub offset: Uint32,
    pub size: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndirectDrawCommand {
    pub num_vertices: Uint32,
    pub num_instances: Uint32,
    pub first_vertex: Uint32,
    pub first_instance: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndexedIndirectDrawCommand {
    pub num_indices: Uint32,
    pub num_instances: Uint32,
    pub first_index: Uint32,
    pub vertex_offset: Sint32,
    pub first_instance: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndirectDispatchCommand {
    pub groupcount_x: Uint32,
    pub groupcount_y: Uint32,
    pub groupcount_z: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSamplerCreateInfo {
    pub min_filter: SDL_GPUFilter,
    pub mag_filter: SDL_GPUFilter,
    pub mipmap_mode: SDL_GPUSamplerMipmapMode,
    pub address_mode_u: SDL_GPUSamplerAddressMode,
    pub address_mode_v: SDL_GPUSamplerAddressMode,
    pub address_mode_w: SDL_GPUSamplerAddressMode,
    pub mip_lod_bias: ::core::ffi::c_float,
    pub max_anisotropy: ::core::ffi::c_float,
    pub enable_anisotropy: SDL_bool,
    pub enable_compare: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub compare_op: SDL_GPUCompareOp,
    pub min_lod: ::core::ffi::c_float,
    pub max_lod: ::core::ffi::c_float,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexBinding {
    pub binding: Uint32,
    pub pitch: Uint32,
    pub input_rate: SDL_GPUVertexInputRate,
    pub instance_step_rate: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexAttribute {
    pub location: Uint32,
    pub binding: Uint32,
    pub format: SDL_GPUVertexElementFormat,
    pub offset: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexInputState {
    pub vertex_bindings: *const SDL_GPUVertexBinding,
    pub num_vertex_bindings: Uint32,
    pub vertex_attributes: *const SDL_GPUVertexAttribute,
    pub num_vertex_attributes: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStencilOpState {
    pub fail_op: SDL_GPUStencilOp,
    pub pass_op: SDL_GPUStencilOp,
    pub depth_fail_op: SDL_GPUStencilOp,
    pub compare_op: SDL_GPUCompareOp,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetBlendState {
    pub enable_blend: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub src_color_blendfactor: SDL_GPUBlendFactor,
    pub dst_color_blendfactor: SDL_GPUBlendFactor,
    pub color_blend_op: SDL_GPUBlendOp,
    pub src_alpha_blendfactor: SDL_GPUBlendFactor,
    pub dst_alpha_blendfactor: SDL_GPUBlendFactor,
    pub alpha_blend_op: SDL_GPUBlendOp,
    pub color_write_mask: SDL_GPUColorComponentFlags,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUShaderCreateInfo {
    pub code_size: ::core::primitive::usize,
    pub code: *const Uint8,
    pub entrypoint: *const ::core::ffi::c_char,
    pub format: SDL_GPUShaderFormat,
    pub stage: SDL_GPUShaderStage,
    pub num_samplers: Uint32,
    pub num_storage_textures: Uint32,
    pub num_storage_buffers: Uint32,
    pub num_uniform_buffers: Uint32,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureCreateInfo {
    pub r#type: SDL_GPUTextureType,
    pub format: SDL_GPUTextureFormat,
    pub usage: SDL_GPUTextureUsageFlags,
    pub width: Uint32,
    pub height: Uint32,
    pub layer_count_or_depth: Uint32,
    pub num_levels: Uint32,
    pub sample_count: SDL_GPUSampleCount,
    pub props: SDL_PropertiesID,
}

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_R_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.r\0") };

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_G_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.g\0") };

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_B_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.b\0") };

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_A_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.a\0") };

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_DEPTH_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.depth\0") };

pub const SDL_PROP_GPU_CREATETEXTURE_D3D12_CLEAR_STENCIL_UINT8: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.createtexture.d3d12.clear.stencil\0") };

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferCreateInfo {
    pub usage: SDL_GPUBufferUsageFlags,
    pub size: Uint32,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTransferBufferCreateInfo {
    pub usage: SDL_GPUTransferBufferUsage,
    pub size: Uint32,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPURasterizerState {
    pub fill_mode: SDL_GPUFillMode,
    pub cull_mode: SDL_GPUCullMode,
    pub front_face: SDL_GPUFrontFace,
    pub enable_depth_bias: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub depth_bias_constant_factor: ::core::ffi::c_float,
    pub depth_bias_clamp: ::core::ffi::c_float,
    pub depth_bias_slope_factor: ::core::ffi::c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUMultisampleState {
    pub sample_count: SDL_GPUSampleCount,
    pub sample_mask: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDepthStencilState {
    pub enable_depth_test: SDL_bool,
    pub enable_depth_write: SDL_bool,
    pub enable_stencil_test: SDL_bool,
    pub padding1: Uint8,
    pub compare_op: SDL_GPUCompareOp,
    pub back_stencil_state: SDL_GPUStencilOpState,
    pub front_stencil_state: SDL_GPUStencilOpState,
    pub compare_mask: Uint8,
    pub write_mask: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetDescription {
    pub format: SDL_GPUTextureFormat,
    pub blend_state: SDL_GPUColorTargetBlendState,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GpuGraphicsPipelineTargetInfo {
    pub color_target_descriptions: *const SDL_GPUColorTargetDescription,
    pub num_color_targets: Uint32,
    pub has_depth_stencil_target: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub depth_stencil_format: SDL_GPUTextureFormat,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUGraphicsPipelineCreateInfo {
    pub vertex_shader: *mut SDL_GPUShader,
    pub fragment_shader: *mut SDL_GPUShader,
    pub vertex_input_state: SDL_GPUVertexInputState,
    pub primitive_type: SDL_GPUPrimitiveType,
    pub rasterizer_state: SDL_GPURasterizerState,
    pub multisample_state: SDL_GPUMultisampleState,
    pub depth_stencil_state: SDL_GPUDepthStencilState,
    pub target_info: SDL_GpuGraphicsPipelineTargetInfo,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUComputePipelineCreateInfo {
    pub code_size: ::core::primitive::usize,
    pub code: *const Uint8,
    pub entrypoint: *const ::core::ffi::c_char,
    pub format: SDL_GPUShaderFormat,
    pub num_readonly_storage_textures: Uint32,
    pub num_readonly_storage_buffers: Uint32,
    pub num_writeonly_storage_textures: Uint32,
    pub num_writeonly_storage_buffers: Uint32,
    pub num_uniform_buffers: Uint32,
    pub threadcount_x: Uint32,
    pub threadcount_y: Uint32,
    pub threadcount_z: Uint32,
    pub props: SDL_PropertiesID,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetInfo {
    pub texture: *mut SDL_GPUTexture,
    pub mip_level: Uint32,
    pub layer_or_depth_plane: Uint32,
    pub clear_color: SDL_FColor,
    pub load_op: SDL_GPULoadOp,
    pub store_op: SDL_GPUStoreOp,
    pub cycle: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDepthStencilTargetInfo {
    pub texture: *mut SDL_GPUTexture,
    pub clear_value: SDL_GPUDepthStencilValue,
    pub load_op: SDL_GPULoadOp,
    pub store_op: SDL_GPUStoreOp,
    pub stencil_load_op: SDL_GPULoadOp,
    pub stencil_store_op: SDL_GPUStoreOp,
    pub cycle: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferBinding {
    pub buffer: *mut SDL_GPUBuffer,
    pub offset: Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureSamplerBinding {
    pub texture: *mut SDL_GPUTexture,
    pub sampler: *mut SDL_GPUSampler,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStorageBufferWriteOnlyBinding {
    pub buffer: *mut SDL_GPUBuffer,
    pub cycle: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStorageTextureWriteOnlyBinding {
    pub texture: *mut SDL_GPUTexture,
    pub mip_level: Uint32,
    pub layer: Uint32,
    pub cycle: SDL_bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

extern_sdlcall! {{
    /// Creates a GPU context.
    ///
    /// \param format_flags a bitflag indicating which shader formats the app is
    ///                     able to provide.
    /// \param debug_mode enable debug mode properties and validations.
    /// \param name the preferred GPU driver, or NULL to let SDL pick the optimal
    ///             driver.
    /// \returns a GPU context on success or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGPUDriver
    /// \sa SDL_DestroyGPUDevice
    pub fn SDL_CreateGPUDevice(format_flags: SDL_GPUShaderFormat, debug_mode: SDL_bool, name: *const ::core::ffi::c_char) -> *mut SDL_GPUDevice;
}}

extern_sdlcall! {{
    /// Creates a GPU context.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOL`: enable debug mode properties
    ///   and validations, defaults to SDL_TRUE.
    /// - `SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOL`: enable to prefer energy
    ///   efficiency over maximum GPU performance, defaults to SDL_FALSE.
    /// - `SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING`: the name of the GPU driver to
    ///   use, if a specific one is desired.
    ///
    /// These are the current shader format properties:
    ///
    /// - `SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOL`: The app is able to
    ///   provide shaders for an NDA platform.
    /// - `SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOL`: The app is able to
    ///   provide SPIR-V shaders if applicable.
    /// - SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOL`: The app is able to provide
    ///   DXBC shaders if applicable
    ///   `SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOL`: The app is able to
    ///   provide DXIL shaders if applicable.
    /// - `SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOL`: The app is able to provide
    ///   MSL shaders if applicable.
    /// - `SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOL`: The app is able to
    ///   provide Metal shader libraries if applicable.
    ///
    /// With the D3D12 renderer:
    ///
    /// - `SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING`: the prefix to
    ///   use for all vertex semantics, default is "TEXCOORD".
    ///
    /// \param props the properties to use.
    /// \returns a GPU context on success or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetGPUDriver
    /// \sa SDL_DestroyGPUDevice
    pub fn SDL_CreateGPUDeviceWithProperties(props: SDL_PropertiesID) -> *mut SDL_GPUDevice;
}}

pub const SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.debugmode\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.preferlowpower\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.name\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.private\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.spirv\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.dxbc\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.dxil\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.msl\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOL: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.shaders.metallib\0") };

pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.gpu.device.create.d3d12.semantic\0") };

extern_sdlcall! {{
    /// Destroys a GPU context previously returned by SDL_CreateGPUDevice.
    ///
    /// \param device a GPU Context to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateGPUDevice
    pub fn SDL_DestroyGPUDevice(device: *mut SDL_GPUDevice);
}}

extern_sdlcall! {{
    /// Returns the backend used to create this GPU context.
    ///
    /// \param device a GPU context to query.
    /// \returns an SDL_GPUDriver value, or SDL_GPU_DRIVER_INVALID on error.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGPUDriver(device: *mut SDL_GPUDevice) -> SDL_GPUDriver;
}}

extern_sdlcall! {{
    /// Creates a pipeline object to be used in a compute workflow.
    ///
    /// Shader resource bindings must be authored to follow a particular order
    /// depending on the shader format.
    ///
    /// For SPIR-V shaders, use the following resource sets:
    ///
    /// - 0: Read-only storage textures, followed by read-only storage buffers
    /// - 1: Write-only storage textures, followed by write-only storage buffers
    /// - 2: Uniform buffers
    ///
    /// For DXBC Shader Model 5_0 shaders, use the following register order:
    ///
    /// - t registers: Read-only storage textures, followed by read-only storage
    ///   buffers
    /// - u registers: Write-only storage textures, followed by write-only storage
    ///   buffers
    /// - b registers: Uniform buffers
    ///
    /// For DXIL shaders, use the following register order:
    ///
    /// - (t[n], space0): Read-only storage textures, followed by read-only storage
    ///   buffers
    /// - (u[n], space1): Write-only storage textures, followed by write-only
    ///   storage buffers
    /// - (b[n], space2): Uniform buffers
    ///
    /// For MSL/metallib, use the following order:
    ///
    /// - [[buffer]]: Uniform buffers, followed by write-only storage buffers,
    ///   followed by write-only storage buffers
    /// - [[texture]]: Read-only storage textures, followed by write-only storage
    ///   textures
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the compute pipeline to
    ///                   create.
    /// \returns a compute pipeline object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BindGPUComputePipeline
    /// \sa SDL_ReleaseGPUComputePipeline
    pub fn SDL_CreateGPUComputePipeline(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUComputePipelineCreateInfo) -> *mut SDL_GPUComputePipeline;
}}

extern_sdlcall! {{
    /// Creates a pipeline object to be used in a graphics workflow.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the graphics pipeline to
    ///                   create.
    /// \returns a graphics pipeline object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateGPUShader
    /// \sa SDL_BindGPUGraphicsPipeline
    /// \sa SDL_ReleaseGPUGraphicsPipeline
    pub fn SDL_CreateGPUGraphicsPipeline(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUGraphicsPipelineCreateInfo) -> *mut SDL_GPUGraphicsPipeline;
}}

extern_sdlcall! {{
    /// Creates a sampler object to be used when binding textures in a graphics
    /// workflow.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the sampler to create.
    /// \returns a sampler object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BindGPUVertexSamplers
    /// \sa SDL_BindGPUFragmentSamplers
    /// \sa SDL_ReleaseSampler
    pub fn SDL_CreateGPUSampler(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUSamplerCreateInfo) -> *mut SDL_GPUSampler;
}}

extern_sdlcall! {{
    /// Creates a shader to be used when creating a graphics pipeline.
    ///
    /// Shader resource bindings must be authored to follow a particular order
    /// depending on the shader format.
    ///
    /// For SPIR-V shaders, use the following resource sets:
    ///
    /// For vertex shaders:
    ///
    /// - 0: Sampled textures, followed by storage textures, followed by storage
    ///   buffers
    /// - 1: Uniform buffers
    ///
    /// For fragment shaders:
    ///
    /// - 2: Sampled textures, followed by storage textures, followed by storage
    ///   buffers
    /// - 3: Uniform buffers
    ///
    /// For DXBC Shader Model 5_0 shaders, use the following register order:
    ///
    /// - t registers: Sampled textures, followed by storage textures, followed by
    ///   storage buffers
    /// - s registers: Samplers with indices corresponding to the sampled textures
    /// - b registers: Uniform buffers
    ///
    /// For DXIL shaders, use the following register order:
    ///
    /// For vertex shaders:
    ///
    /// - (t[n], space0): Sampled textures, followed by storage textures, followed
    ///   by storage buffers
    /// - (s[n], space0): Samplers with indices corresponding to the sampled
    ///   textures
    /// - (b[n], space1): Uniform buffers
    ///
    /// For pixel shaders:
    ///
    /// - (t[n], space2): Sampled textures, followed by storage textures, followed
    ///   by storage buffers
    /// - (s[n], space2): Samplers with indices corresponding to the sampled
    ///   textures
    /// - (b[n], space3): Uniform buffers
    ///
    /// For MSL/metallib, use the following order:
    ///
    /// - [[texture]]: Sampled textures, followed by storage textures
    /// - [[sampler]]: Samplers with indices corresponding to the sampled textures
    /// - [[buffer]]: Uniform buffers, followed by storage buffers. Vertex buffer 0
    ///   is bound at [[buffer(30)]], vertex buffer 1 at [[buffer(29)]], and so on.
    ///   Rather than manually authoring vertex buffer indices, use the
    ///   [[stage_in]] attribute which will automatically use the vertex input
    ///   information from the SDL_GPUPipeline.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the shader to create.
    /// \returns a shader object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateGPUGraphicsPipeline
    /// \sa SDL_ReleaseGPUShader
    pub fn SDL_CreateGPUShader(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUShaderCreateInfo) -> *mut SDL_GPUShader;
}}

extern_sdlcall! {{
    /// Creates a texture object to be used in graphics or compute workflows.
    ///
    /// The contents of this texture are undefined until data is written to the
    /// texture.
    ///
    /// Note that certain combinations of usage flags are invalid. For example, a
    /// texture cannot have both the SAMPLER and GRAPHICS_STORAGE_READ flags.
    ///
    /// If you request a sample count higher than the hardware supports, the
    /// implementation will automatically fall back to the highest available sample
    /// count.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the texture to create.
    /// \returns a texture object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_UploadToGPUTexture
    /// \sa SDL_DownloadFromGPUTexture
    /// \sa SDL_BindGPUVertexSamplers
    /// \sa SDL_BindGPUVertexStorageTextures
    /// \sa SDL_BindGPUFragmentSamplers
    /// \sa SDL_BindGPUFragmentStorageTextures
    /// \sa SDL_BindGPUComputeStorageTextures
    /// \sa SDL_BlitGPUTexture
    /// \sa SDL_ReleaseGPUTexture
    /// \sa SDL_GPUTextureSupportsFormat
    pub fn SDL_CreateGPUTexture(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUTextureCreateInfo) -> *mut SDL_GPUTexture;
}}

extern_sdlcall! {{
    /// Creates a buffer object to be used in graphics or compute workflows.
    ///
    /// The contents of this buffer are undefined until data is written to the
    /// buffer.
    ///
    /// Note that certain combinations of usage flags are invalid. For example, a
    /// buffer cannot have both the VERTEX and INDEX flags.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the buffer to create.
    /// \returns a buffer object on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetGPUBufferName
    /// \sa SDL_UploadToGPUBuffer
    /// \sa SDL_DownloadFromGPUBuffer
    /// \sa SDL_CopyGPUBufferToBuffer
    /// \sa SDL_BindGPUVertexBuffers
    /// \sa SDL_BindGPUIndexBuffer
    /// \sa SDL_BindGPUVertexStorageBuffers
    /// \sa SDL_BindGPUFragmentStorageBuffers
    /// \sa SDL_DrawGPUPrimitivesIndirect
    /// \sa SDL_DrawGPUIndexedPrimitivesIndirect
    /// \sa SDL_BindGPUComputeStorageBuffers
    /// \sa SDL_DispatchGPUComputeIndirect
    /// \sa SDL_ReleaseGPUBuffer
    pub fn SDL_CreateGPUBuffer(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUBufferCreateInfo) -> *mut SDL_GPUBuffer;
}}

extern_sdlcall! {{
    /// Creates a transfer buffer to be used when uploading to or downloading from
    /// graphics resources.
    ///
    /// \param device a GPU Context.
    /// \param createinfo a struct describing the state of the transfer buffer to
    ///                   create.
    /// \returns a transfer buffer on success, or NULL on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_UploadToGPUBuffer
    /// \sa SDL_DownloadFromGPUBuffer
    /// \sa SDL_UploadToGPUTexture
    /// \sa SDL_DownloadFromGPUTexture
    /// \sa SDL_ReleaseGPUTransferBuffer
    pub fn SDL_CreateGPUTransferBuffer(device: *mut SDL_GPUDevice, createinfo: *const SDL_GPUTransferBufferCreateInfo) -> *mut SDL_GPUTransferBuffer;
}}

extern_sdlcall! {{
    /// Sets an arbitrary string constant to label a buffer.
    ///
    /// Useful for debugging.
    ///
    /// \param device a GPU Context.
    /// \param buffer a buffer to attach the name to.
    /// \param text a UTF-8 string constant to mark as the name of the buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGPUBufferName(device: *mut SDL_GPUDevice, buffer: *mut SDL_GPUBuffer, text: *const ::core::ffi::c_char);
}}

extern_sdlcall! {{
    /// Sets an arbitrary string constant to label a texture.
    ///
    /// Useful for debugging.
    ///
    /// \param device a GPU Context.
    /// \param texture a texture to attach the name to.
    /// \param text a UTF-8 string constant to mark as the name of the texture.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGPUTextureName(device: *mut SDL_GPUDevice, texture: *mut SDL_GPUTexture, text: *const ::core::ffi::c_char);
}}

extern_sdlcall! {{
    /// Inserts an arbitrary string label into the command buffer callstream.
    ///
    /// Useful for debugging.
    ///
    /// \param command_buffer a command buffer.
    /// \param text a UTF-8 string constant to insert as the label.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_InsertGPUDebugLabel(command_buffer: *mut SDL_GPUCommandBuffer, text: *const ::core::ffi::c_char);
}}

extern_sdlcall! {{
    /// Begins a debug group with an arbitary name.
    ///
    /// Used for denoting groups of calls when viewing the command buffer
    /// callstream in a graphics debugging tool.
    ///
    /// Each call to SDL_PushGPUDebugGroup must have a corresponding call to
    /// SDL_PopGPUDebugGroup.
    ///
    /// On some backends (e.g. Metal), pushing a debug group during a
    /// render/blit/compute pass will create a group that is scoped to the native
    /// pass rather than the command buffer. For best results, if you push a debug
    /// group during a pass, always pop it in the same pass.
    ///
    /// \param command_buffer a command buffer.
    /// \param name a UTF-8 string constant that names the group.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_PopGPUDebugGroup
    pub fn SDL_PushGPUDebugGroup(command_buffer: *mut SDL_GPUCommandBuffer, name: *const ::core::ffi::c_char);
}}

extern_sdlcall! {{
    /// Ends the most-recently pushed debug group.
    ///
    /// \param command_buffer a command buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_PushGPUDebugGroup
    pub fn SDL_PopGPUDebugGroup(command_buffer: *mut SDL_GPUCommandBuffer);
}}

extern_sdlcall! {{
    /// Frees the given texture as soon as it is safe to do so.
    ///
    /// You must not reference the texture after calling this function.
    ///
    /// \param device a GPU context.
    /// \param texture a texture to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUTexture(device: *mut SDL_GPUDevice, texture: *mut SDL_GPUTexture);
}}

extern_sdlcall! {{
    /// Frees the given sampler as soon as it is safe to do so.
    ///
    /// You must not reference the texture after calling this function.
    ///
    /// \param device a GPU context.
    /// \param sampler a sampler to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUSampler(device: *mut SDL_GPUDevice, sampler: *mut SDL_GPUSampler);
}}

extern_sdlcall! {{
    /// Frees the given buffer as soon as it is safe to do so.
    ///
    /// You must not reference the buffer after calling this function.
    ///
    /// \param device a GPU context.
    /// \param buffer a buffer to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUBuffer(device: *mut SDL_GPUDevice, buffer: *mut SDL_GPUBuffer);
}}

extern_sdlcall! {{
    /// Frees the given transfer buffer as soon as it is safe to do so.
    ///
    /// You must not reference the transfer buffer after calling this function.
    ///
    /// \param device a GPU context.
    /// \param transfer_buffer a transfer buffer to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUTransferBuffer(device: *mut SDL_GPUDevice, transfer_buffer: *mut SDL_GPUTransferBuffer);
}}

extern_sdlcall! {{
    /// Frees the given compute pipeline as soon as it is safe to do so.
    ///
    /// You must not reference the compute pipeline after calling this function.
    ///
    /// \param device a GPU context.
    /// \param compute_pipeline a compute pipeline to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUComputePipeline(device: *mut SDL_GPUDevice, compute_pipeline: *mut SDL_GPUComputePipeline);
}}

extern_sdlcall! {{
    /// Frees the given shader as soon as it is safe to do so.
    ///
    /// You must not reference the shader after calling this function.
    ///
    /// \param device a GPU context.
    /// \param shader a shader to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUShader(device: *mut SDL_GPUDevice, shader: *mut SDL_GPUShader);
}}

extern_sdlcall! {{
    /// Frees the given graphics pipeline as soon as it is safe to do so.
    ///
    /// You must not reference the graphics pipeline after calling this function.
    ///
    /// \param device a GPU context.
    /// \param graphics_pipeline a graphics pipeline to be destroyed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReleaseGPUGraphicsPipeline(device: *mut SDL_GPUDevice, graphics_pipeline: *mut SDL_GPUGraphicsPipeline);
}}

extern_sdlcall! {{
    /// Acquire a command buffer.
    ///
    /// This command buffer is managed by the implementation and should not be
    /// freed by the user. The command buffer may only be used on the thread it was
    /// acquired on. The command buffer should be submitted on the thread it was
    /// acquired on.
    ///
    /// \param device a GPU context.
    /// \returns a command buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SubmitGPUCommandBuffer
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    pub fn SDL_AcquireGPUCommandBuffer(device: *mut SDL_GPUDevice) -> *mut SDL_GPUCommandBuffer;
}}

extern_sdlcall! {{
    /// Pushes data to a vertex uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// \param command_buffer a command buffer.
    /// \param slot_index the vertex uniform slot to push data to.
    /// \param data client data to write.
    /// \param length the length of the data to write.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_PushGPUVertexUniformData(command_buffer: *mut SDL_GPUCommandBuffer, slot_index: Uint32, data: *const ::core::ffi::c_void, length: Uint32);
}}

extern_sdlcall! {{
    /// Pushes data to a fragment uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// \param command_buffer a command buffer.
    /// \param slot_index the fragment uniform slot to push data to.
    /// \param data client data to write.
    /// \param length the length of the data to write.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_PushGPUFragmentUniformData(command_buffer: *mut SDL_GPUCommandBuffer, slot_index: Uint32, data: *const ::core::ffi::c_void, length: Uint32);
}}

extern_sdlcall! {{
    /// Pushes data to a uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// \param command_buffer a command buffer.
    /// \param slot_index the uniform slot to push data to.
    /// \param data client data to write.
    /// \param length the length of the data to write.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_PushGPUComputeUniformData(command_buffer: *mut SDL_GPUCommandBuffer, slot_index: Uint32, data: *const ::core::ffi::c_void, length: Uint32);
}}

extern_sdlcall! {{
    /// Begins a render pass on a command buffer.
    ///
    /// A render pass consists of a set of texture subresources (or depth slices in
    /// the 3D texture case) which will be rendered to during the render pass,
    /// along with corresponding clear values and load/store operations. All
    /// operations related to graphics pipelines must take place inside of a render
    /// pass. A default viewport and scissor state are automatically set when this
    /// is called. You cannot begin another render pass, or begin a compute pass or
    /// copy pass until you have ended the render pass.
    ///
    /// \param command_buffer a command buffer.
    /// \param color_target_infos an array of texture subresources with
    ///                           corresponding clear values and load/store ops.
    /// \param num_color_targets the number of color targets in the
    ///                          color_target_infos array.
    /// \param depth_stencil_target_info a texture subresource with corresponding
    ///                                  clear value and load/store ops, may be
    ///                                  NULL.
    /// \returns a render pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_EndGPURenderPass
    pub fn SDL_BeginGPURenderPass(command_buffer: *mut SDL_GPUCommandBuffer, color_target_infos: *const SDL_GPUColorTargetInfo, num_color_targets: Uint32, depth_stencil_target_info: *const SDL_GPUDepthStencilTargetInfo) -> *mut SDL_GPURenderPass;
}}

extern_sdlcall! {{
    /// Binds a graphics pipeline on a render pass to be used in rendering.
    ///
    /// A graphics pipeline must be bound before making any draw calls.
    ///
    /// \param render_pass a render pass handle.
    /// \param graphics_pipeline the graphics pipeline to bind.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUGraphicsPipeline(render_pass: *mut SDL_GPURenderPass, graphics_pipeline: *mut SDL_GPUGraphicsPipeline);
}}

extern_sdlcall! {{
    /// Sets the current viewport state on a command buffer.
    ///
    /// \param render_pass a render pass handle.
    /// \param viewport the viewport to set.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGPUViewport(render_pass: *mut SDL_GPURenderPass, viewport: *const SDL_GPUViewport);
}}

extern_sdlcall! {{
    /// Sets the current scissor state on a command buffer.
    ///
    /// \param render_pass a render pass handle.
    /// \param scissor the scissor area to set.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGPUScissor(render_pass: *mut SDL_GPURenderPass, scissor: *const SDL_Rect);
}}

extern_sdlcall! {{
    /// Sets the current blend constants on a command buffer.
    ///
    /// \param render_pass a render pass handle.
    /// \param blend_constants the blend constant color.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GPU_BLENDFACTOR_CONSTANT_COLOR
    /// \sa SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR
    pub fn SDL_SetGPUBlendConstants(render_pass: *mut SDL_GPURenderPass, blend_constants: SDL_FColor);
}}

extern_sdlcall! {{
    /// Sets the current stencil reference value on a command buffer.
    ///
    /// \param render_pass a render pass handle.
    /// \param reference the stencil reference value to set.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetGPUStencilReference(render_pass: *mut SDL_GPURenderPass, reference: Uint8);
}}

extern_sdlcall! {{
    /// Binds vertex buffers on a command buffer for use with subsequent draw
    /// calls.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_binding the starting bind point for the vertex buffers.
    /// \param bindings an array of SDL_GPUBufferBinding structs containing vertex
    ///                 buffers and offset values.
    /// \param num_bindings the number of bindings in the bindings array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUVertexBuffers(render_pass: *mut SDL_GPURenderPass, first_binding: Uint32, bindings: *const SDL_GPUBufferBinding, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds an index buffer on a command buffer for use with subsequent draw
    /// calls.
    ///
    /// \param render_pass a render pass handle.
    /// \param binding a pointer to a struct containing an index buffer and offset.
    /// \param index_element_size whether the index values in the buffer are 16- or
    ///                           32-bit.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUIndexBuffer(render_pass: *mut SDL_GPURenderPass, binding: *const SDL_GPUBufferBinding, index_element_size: SDL_GPUIndexElementSize);
}}

extern_sdlcall! {{
    /// Binds texture-sampler pairs for use on the vertex shader.
    ///
    /// The textures must have been created with SDL_GPU_TEXTUREUSAGE_SAMPLER.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the vertex sampler slot to begin binding from.
    /// \param texture_sampler_bindings an array of texture-sampler binding
    ///                                 structs.
    /// \param num_bindings the number of texture-sampler pairs to bind from the
    ///                     array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUVertexSamplers(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, texture_sampler_bindings: *const SDL_GPUTextureSamplerBinding, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds storage textures for use on the vertex shader.
    ///
    /// These textures must have been created with
    /// SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the vertex storage texture slot to begin binding from.
    /// \param storage_textures an array of storage textures.
    /// \param num_bindings the number of storage texture to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUVertexStorageTextures(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, storage_textures: *const *mut SDL_GPUTexture, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds storage buffers for use on the vertex shader.
    ///
    /// These buffers must have been created with
    /// SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the vertex storage buffer slot to begin binding from.
    /// \param storage_buffers an array of buffers.
    /// \param num_bindings the number of buffers to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUVertexStorageBuffers(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, storage_buffers: *const *mut SDL_GPUBuffer, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds texture-sampler pairs for use on the fragment shader.
    ///
    /// The textures must have been created with SDL_GPU_TEXTUREUSAGE_SAMPLER.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the fragment sampler slot to begin binding from.
    /// \param texture_sampler_bindings an array of texture-sampler binding
    ///                                 structs.
    /// \param num_bindings the number of texture-sampler pairs to bind from the
    ///                     array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUFragmentSamplers(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, texture_sampler_bindings: *const SDL_GPUTextureSamplerBinding, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds storage textures for use on the fragment shader.
    ///
    /// These textures must have been created with
    /// SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the fragment storage texture slot to begin binding from.
    /// \param storage_textures an array of storage textures.
    /// \param num_bindings the number of storage textures to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUFragmentStorageTextures(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, storage_textures: *const *mut SDL_GPUTexture, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds storage buffers for use on the fragment shader.
    ///
    /// These buffers must have been created with
    /// SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ.
    ///
    /// \param render_pass a render pass handle.
    /// \param first_slot the fragment storage buffer slot to begin binding from.
    /// \param storage_buffers an array of storage buffers.
    /// \param num_bindings the number of storage buffers to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUFragmentStorageBuffers(render_pass: *mut SDL_GPURenderPass, first_slot: Uint32, storage_buffers: *const *mut SDL_GPUBuffer, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Draws data using bound graphics state with an index buffer and instancing
    /// enabled.
    ///
    /// You must not call this function before binding a graphics pipeline.
    ///
    /// Note that the `first_vertex` and `first_instance` parameters are NOT
    /// compatible with built-in vertex/instance ID variables in shaders (for
    /// example, SV_VertexID). If your shader depends on these variables, the
    /// correlating draw call parameter MUST be 0.
    ///
    /// \param render_pass a render pass handle.
    /// \param num_indices the number of vertices to draw per instance.
    /// \param num_instances the number of instances to draw.
    /// \param first_index the starting index within the index buffer.
    /// \param vertex_offset value added to vertex index before indexing into the
    ///                      vertex buffer.
    /// \param first_instance the ID of the first instance to draw.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DrawGPUIndexedPrimitives(render_pass: *mut SDL_GPURenderPass, num_indices: Uint32, num_instances: Uint32, first_index: Uint32, vertex_offset: Sint32, first_instance: Uint32);
}}

extern_sdlcall! {{
    /// Draws data using bound graphics state.
    ///
    /// You must not call this function before binding a graphics pipeline.
    ///
    /// Note that the `first_vertex` and `first_instance` parameters are NOT
    /// compatible with built-in vertex/instance ID variables in shaders (for
    /// example, SV_VertexID). If your shader depends on these variables, the
    /// correlating draw call parameter MUST be 0.
    ///
    /// \param render_pass a render pass handle.
    /// \param num_vertices the number of vertices to draw.
    /// \param num_instances the number of instances that will be drawn.
    /// \param first_vertex the index of the first vertex to draw.
    /// \param first_instance the ID of the first instance to draw.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DrawGPUPrimitives(render_pass: *mut SDL_GPURenderPass, num_vertices: Uint32, num_instances: Uint32, first_vertex: Uint32, first_instance: Uint32);
}}

extern_sdlcall! {{
    /// Draws data using bound graphics state and with draw parameters set from a
    /// buffer.
    ///
    /// The buffer layout should match the layout of SDL_GPUIndirectDrawCommand.
    /// You must not call this function before binding a graphics pipeline.
    ///
    /// \param render_pass a render pass handle.
    /// \param buffer a buffer containing draw parameters.
    /// \param offset the offset to start reading from the draw buffer.
    /// \param draw_count the number of draw parameter sets that should be read
    ///                   from the draw buffer.
    /// \param pitch the byte pitch between sets of draw parameters.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DrawGPUPrimitivesIndirect(render_pass: *mut SDL_GPURenderPass, buffer: *mut SDL_GPUBuffer, offset: Uint32, draw_count: Uint32, pitch: Uint32);
}}

extern_sdlcall! {{
    /// Draws data using bound graphics state with an index buffer enabled and with
    /// draw parameters set from a buffer.
    ///
    /// The buffer layout should match the layout of
    /// SDL_GPUIndexedIndirectDrawCommand. You must not call this function before
    /// binding a graphics pipeline.
    ///
    /// \param render_pass a render pass handle.
    /// \param buffer a buffer containing draw parameters.
    /// \param offset the offset to start reading from the draw buffer.
    /// \param draw_count the number of draw parameter sets that should be read
    ///                   from the draw buffer.
    /// \param pitch the byte pitch between sets of draw parameters.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DrawGPUIndexedPrimitivesIndirect(render_pass: *mut SDL_GPURenderPass, buffer: *mut SDL_GPUBuffer, offset: Uint32, draw_count: Uint32, pitch: Uint32);
}}

extern_sdlcall! {{
    /// Ends the given render pass.
    ///
    /// All bound graphics state on the render pass command buffer is unset. The
    /// render pass handle is now invalid.
    ///
    /// \param render_pass a render pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EndGPURenderPass(render_pass: *mut SDL_GPURenderPass);
}}

extern_sdlcall! {{
    /// Begins a compute pass on a command buffer.
    ///
    /// A compute pass is defined by a set of texture subresources and buffers that
    /// will be written to by compute pipelines. These textures and buffers must
    /// have been created with the COMPUTE_STORAGE_WRITE bit. All operations
    /// related to compute pipelines must take place inside of a compute pass. You
    /// must not begin another compute pass, or a render pass or copy pass before
    /// ending the compute pass.
    ///
    /// A VERY IMPORTANT NOTE Textures and buffers bound as write-only MUST NOT be
    /// read from during the compute pass. Doing so will result in undefined
    /// behavior. If your compute work requires reading the output from a previous
    /// dispatch, you MUST end the current compute pass and begin a new one before
    /// you can safely access the data.
    ///
    /// \param command_buffer a command buffer.
    /// \param storage_texture_bindings an array of writeable storage texture
    ///                                 binding structs.
    /// \param num_storage_texture_bindings the number of storage textures to bind
    ///                                     from the array.
    /// \param storage_buffer_bindings an array of writeable storage buffer binding
    ///                                structs.
    /// \param num_storage_buffer_bindings the number of storage buffers to bind
    ///                                    from the array.
    /// \returns a compute pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_EndGPUComputePass
    pub fn SDL_BeginGPUComputePass(command_buffer: *mut SDL_GPUCommandBuffer, storage_texture_bindings: *const SDL_GPUStorageTextureWriteOnlyBinding, num_storage_texture_bindings: Uint32, storage_buffer_bindings: *const SDL_GPUStorageBufferWriteOnlyBinding, num_storage_buffer_bindings: Uint32) -> *mut SDL_GPUComputePass;
}}

extern_sdlcall! {{
    /// Binds a compute pipeline on a command buffer for use in compute dispatch.
    ///
    /// \param compute_pass a compute pass handle.
    /// \param compute_pipeline a compute pipeline to bind.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUComputePipeline(compute_pass: *mut SDL_GPUComputePass, compute_pipeline: *mut SDL_GPUComputePipeline);
}}

extern_sdlcall! {{
    /// Binds storage textures as readonly for use on the compute pipeline.
    ///
    /// These textures must have been created with
    /// SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ.
    ///
    /// \param compute_pass a compute pass handle.
    /// \param first_slot the compute storage texture slot to begin binding from.
    /// \param storage_textures an array of storage textures.
    /// \param num_bindings the number of storage textures to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUComputeStorageTextures(compute_pass: *mut SDL_GPUComputePass, first_slot: Uint32, storage_textures: *const *mut SDL_GPUTexture, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Binds storage buffers as readonly for use on the compute pipeline.
    ///
    /// These buffers must have been created with
    /// SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_READ.
    ///
    /// \param compute_pass a compute pass handle.
    /// \param first_slot the compute storage buffer slot to begin binding from.
    /// \param storage_buffers an array of storage buffer binding structs.
    /// \param num_bindings the number of storage buffers to bind from the array.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BindGPUComputeStorageBuffers(compute_pass: *mut SDL_GPUComputePass, first_slot: Uint32, storage_buffers: *const *mut SDL_GPUBuffer, num_bindings: Uint32);
}}

extern_sdlcall! {{
    /// Dispatches compute work.
    ///
    /// You must not call this function before binding a compute pipeline.
    ///
    /// A VERY IMPORTANT NOTE If you dispatch multiple times in a compute pass, and
    /// the dispatches write to the same resource region as each other, there is no
    /// guarantee of which order the writes will occur. If the write order matters,
    /// you MUST end the compute pass and begin another one.
    ///
    /// \param compute_pass a compute pass handle.
    /// \param groupcount_x number of local workgroups to dispatch in the X
    ///                     dimension.
    /// \param groupcount_y number of local workgroups to dispatch in the Y
    ///                     dimension.
    /// \param groupcount_z number of local workgroups to dispatch in the Z
    ///                     dimension.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DispatchGPUCompute(compute_pass: *mut SDL_GPUComputePass, groupcount_x: Uint32, groupcount_y: Uint32, groupcount_z: Uint32);
}}

extern_sdlcall! {{
    /// Dispatches compute work with parameters set from a buffer.
    ///
    /// The buffer layout should match the layout of
    /// SDL_GPUIndirectDispatchCommand. You must not call this function before
    /// binding a compute pipeline.
    ///
    /// A VERY IMPORTANT NOTE If you dispatch multiple times in a compute pass, and
    /// the dispatches write to the same resource region as each other, there is no
    /// guarantee of which order the writes will occur. If the write order matters,
    /// you MUST end the compute pass and begin another one.
    ///
    /// \param compute_pass a compute pass handle.
    /// \param buffer a buffer containing dispatch parameters.
    /// \param offset the offset to start reading from the dispatch buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DispatchGPUComputeIndirect(compute_pass: *mut SDL_GPUComputePass, buffer: *mut SDL_GPUBuffer, offset: Uint32);
}}

extern_sdlcall! {{
    /// Ends the current compute pass.
    ///
    /// All bound compute state on the command buffer is unset. The compute pass
    /// handle is now invalid.
    ///
    /// \param compute_pass a compute pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EndGPUComputePass(compute_pass: *mut SDL_GPUComputePass);
}}

extern_sdlcall! {{
    /// Maps a transfer buffer into application address space.
    ///
    /// You must unmap the transfer buffer before encoding upload commands.
    ///
    /// \param device a GPU context.
    /// \param transfer_buffer a transfer buffer.
    /// \param cycle if SDL_TRUE, cycles the transfer buffer if it is bound.
    /// \returns the address of the mapped transfer buffer memory.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_MapGPUTransferBuffer(device: *mut SDL_GPUDevice, transfer_buffer: *mut SDL_GPUTransferBuffer, cycle: SDL_bool) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    /// Unmaps a previously mapped transfer buffer.
    ///
    /// \param device a GPU context.
    /// \param transfer_buffer a previously mapped transfer buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UnmapGPUTransferBuffer(device: *mut SDL_GPUDevice, transfer_buffer: *mut SDL_GPUTransferBuffer);
}}

extern_sdlcall! {{
    /// Begins a copy pass on a command buffer.
    ///
    /// All operations related to copying to or from buffers or textures take place
    /// inside a copy pass. You must not begin another copy pass, or a render pass
    /// or compute pass before ending the copy pass.
    ///
    /// \param command_buffer a command buffer.
    /// \returns a copy pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BeginGPUCopyPass(command_buffer: *mut SDL_GPUCommandBuffer) -> *mut SDL_GPUCopyPass;
}}

extern_sdlcall! {{
    /// Uploads data from a transfer buffer to a texture.
    ///
    /// The upload occurs on the GPU timeline. You may assume that the upload has
    /// finished in subsequent commands.
    ///
    /// You must align the data in the transfer buffer to a multiple of the texel
    /// size of the texture format.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source the source transfer buffer with image layout information.
    /// \param destination the destination texture region.
    /// \param cycle if SDL_TRUE, cycles the texture if the texture is bound,
    ///              otherwise overwrites the data.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UploadToGPUTexture(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUTextureTransferInfo, destination: *const SDL_GPUTextureRegion, cycle: SDL_bool);
}}

extern_sdlcall! {{
    /// Uploads data from a transfer buffer to a buffer.
    ///
    /// The upload occurs on the GPU timeline. You may assume that the upload has
    /// finished in subsequent commands.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source the source transfer buffer with offset.
    /// \param destination the destination buffer with offset and size.
    /// \param cycle if SDL_TRUE, cycles the buffer if it is bound, otherwise
    ///              overwrites the data.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UploadToGPUBuffer(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUTransferBufferLocation, destination: *const SDL_GPUBufferRegion, cycle: SDL_bool);
}}

extern_sdlcall! {{
    /// Performs a texture-to-texture copy.
    ///
    /// This copy occurs on the GPU timeline. You may assume the copy has finished
    /// in subsequent commands.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source a source texture region.
    /// \param destination a destination texture region.
    /// \param w the width of the region to copy.
    /// \param h the height of the region to copy.
    /// \param d the depth of the region to copy.
    /// \param cycle if SDL_TRUE, cycles the destination texture if the destination
    ///              texture is bound, otherwise overwrites the data.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CopyGPUTextureToTexture(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUTextureLocation, destination: *const SDL_GPUTextureLocation, w: Uint32, h: Uint32, d: Uint32, cycle: SDL_bool);
}}

extern_sdlcall! {{
    /// Performs a buffer-to-buffer copy.
    ///
    /// This copy occurs on the GPU timeline. You may assume the copy has finished
    /// in subsequent commands.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source the buffer and offset to copy from.
    /// \param destination the buffer and offset to copy to.
    /// \param size the length of the buffer to copy.
    /// \param cycle if SDL_TRUE, cycles the destination buffer if it is bound,
    ///              otherwise overwrites the data.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CopyGPUBufferToBuffer(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUBufferLocation, destination: *const SDL_GPUBufferLocation, size: Uint32, cycle: SDL_bool);
}}

extern_sdlcall! {{
    /// Copies data from a texture to a transfer buffer on the GPU timeline.
    ///
    /// This data is not guaranteed to be copied until the command buffer fence is
    /// signaled.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source the source texture region.
    /// \param destination the destination transfer buffer with image layout
    ///                    information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DownloadFromGPUTexture(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUTextureRegion, destination: *const SDL_GPUTextureTransferInfo);
}}

extern_sdlcall! {{
    /// Copies data from a buffer to a transfer buffer on the GPU timeline.
    ///
    /// This data is not guaranteed to be copied until the command buffer fence is
    /// signaled.
    ///
    /// \param copy_pass a copy pass handle.
    /// \param source the source buffer with offset and size.
    /// \param destination the destination transfer buffer with offset.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_DownloadFromGPUBuffer(copy_pass: *mut SDL_GPUCopyPass, source: *const SDL_GPUBufferRegion, destination: *const SDL_GPUTransferBufferLocation);
}}

extern_sdlcall! {{
    /// Ends the current copy pass.
    ///
    /// \param copy_pass a copy pass handle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EndGPUCopyPass(copy_pass: *mut SDL_GPUCopyPass);
}}

extern_sdlcall! {{
    /// Generates mipmaps for the given texture.
    ///
    /// This function must not be called inside of any pass.
    ///
    /// \param command_buffer a command_buffer.
    /// \param texture a texture with more than 1 mip level.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GenerateMipmapsForGPUTexture(command_buffer: *mut SDL_GPUCommandBuffer, texture: *mut SDL_GPUTexture);
}}

extern_sdlcall! {{
    /// Blits from a source texture region to a destination texture region.
    ///
    /// This function must not be called inside of any pass.
    ///
    /// \param command_buffer a command buffer.
    /// \param source the texture region to copy from.
    /// \param destination the texture region to copy to.
    /// \param flip_mode the flip mode for the source texture region.
    /// \param filter the filter mode that will be used when blitting.
    /// \param cycle if SDL_TRUE, cycles the destination texture if the destination
    ///              texture is bound, otherwise overwrites the data.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_BlitGPUTexture(command_buffer: *mut SDL_GPUCommandBuffer, source: *const SDL_GPUBlitRegion, destination: *const SDL_GPUBlitRegion, flip_mode: SDL_FlipMode, filter: SDL_GPUFilter, cycle: SDL_bool);
}}

extern_sdlcall! {{
    /// Determines whether a swapchain composition is supported by the window.
    ///
    /// The window must be claimed before calling this function.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window.
    /// \param swapchain_composition the swapchain composition to check.
    /// \returns SDL_TRUE if supported, SDL_FALSE if unsupported (or on error).
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ClaimWindowForGPUDevice
    pub fn SDL_WindowSupportsGPUSwapchainComposition(device: *mut SDL_GPUDevice, window: *mut SDL_Window, swapchain_composition: SDL_GPUSwapchainComposition) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Determines whether a presentation mode is supported by the window.
    ///
    /// The window must be claimed before calling this function.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window.
    /// \param present_mode the presentation mode to check.
    /// \returns SDL_TRUE if supported, SDL_FALSE if unsupported (or on error).
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ClaimWindowForGPUDevice
    pub fn SDL_WindowSupportsGPUPresentMode(device: *mut SDL_GPUDevice, window: *mut SDL_Window, present_mode: SDL_GPUPresentMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Claims a window, creating a swapchain structure for it.
    ///
    /// This must be called before SDL_AcquireGPUSwapchainTexture is called using
    /// the window.
    ///
    /// The swapchain will be created with SDL_GPU_SWAPCHAINCOMPOSITION_SDR and
    /// SDL_GPU_PRESENTMODE_VSYNC. If you want to have different swapchain
    /// parameters, you must call SDL_SetGPUSwapchainParameters after claiming the
    /// window.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window.
    /// \returns SDL_TRUE on success, otherwise SDL_FALSE.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AcquireGPUSwapchainTexture
    /// \sa SDL_ReleaseWindowFromGPUDevice
    /// \sa SDL_WindowSupportsGPUPresentMode
    /// \sa SDL_WindowSupportsGPUSwapchainComposition
    pub fn SDL_ClaimWindowForGPUDevice(device: *mut SDL_GPUDevice, window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Unclaims a window, destroying its swapchain structure.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window that has been claimed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ClaimWindowForGPUDevice
    pub fn SDL_ReleaseWindowFromGPUDevice(device: *mut SDL_GPUDevice, window: *mut SDL_Window);
}}

extern_sdlcall! {{
    /// Changes the swapchain parameters for the given claimed window.
    ///
    /// This function will fail if the requested present mode or swapchain
    /// composition are unsupported by the device. Check if the parameters are
    /// supported via SDL_WindowSupportsGPUPresentMode /
    /// SDL_WindowSupportsGPUSwapchainComposition prior to calling this function.
    ///
    /// SDL_GPU_PRESENTMODE_VSYNC and SDL_GPU_SWAPCHAINCOMPOSITION_SDR are always
    /// supported.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window that has been claimed.
    /// \param swapchain_composition the desired composition of the swapchain.
    /// \param present_mode the desired present mode for the swapchain.
    /// \returns SDL_TRUE if successful, SDL_FALSE on error.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_WindowSupportsGPUPresentMode
    /// \sa SDL_WindowSupportsGPUSwapchainComposition
    pub fn SDL_SetGPUSwapchainParameters(device: *mut SDL_GPUDevice, window: *mut SDL_Window, swapchain_composition: SDL_GPUSwapchainComposition, present_mode: SDL_GPUPresentMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Obtains the texture format of the swapchain for the given window.
    ///
    /// \param device a GPU context.
    /// \param window an SDL_Window that has been claimed.
    /// \returns the texture format of the swapchain.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetGPUSwapchainTextureFormat(device: *mut SDL_GPUDevice, window: *mut SDL_Window) -> SDL_GPUTextureFormat;
}}

extern_sdlcall! {{
    /// Acquire a texture to use in presentation.
    ///
    /// When a swapchain texture is acquired on a command buffer, it will
    /// automatically be submitted for presentation when the command buffer is
    /// submitted. The swapchain texture should only be referenced by the command
    /// buffer used to acquire it. May return NULL under certain conditions. This
    /// is not necessarily an error. This texture is managed by the implementation
    /// and must not be freed by the user. You MUST NOT call this function from any
    /// thread other than the one that created the window.
    ///
    /// \param command_buffer a command buffer.
    /// \param window a window that has been claimed.
    /// \param w a pointer filled in with the swapchain width.
    /// \param h a pointer filled in with the swapchain height.
    /// \returns a swapchain texture.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ClaimWindowForGPUDevice
    /// \sa SDL_SubmitGPUCommandBuffer
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    pub fn SDL_AcquireGPUSwapchainTexture(command_buffer: *mut SDL_GPUCommandBuffer, window: *mut SDL_Window, w: *mut Uint32, h: *mut Uint32) -> *mut SDL_GPUTexture;
}}

extern_sdlcall! {{
    /// Submits a command buffer so its commands can be processed on the GPU.
    ///
    /// It is invalid to use the command buffer after this is called.
    ///
    /// This must be called from the thread the command buffer was acquired on.
    ///
    /// All commands in the submission are guaranteed to begin executing before any
    /// command in a subsequent submission begins executing.
    ///
    /// \param command_buffer a command buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AcquireGPUCommandBuffer
    /// \sa SDL_AcquireGPUSwapchainTexture
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    pub fn SDL_SubmitGPUCommandBuffer(command_buffer: *mut SDL_GPUCommandBuffer);
}}

extern_sdlcall! {{
    /// Submits a command buffer so its commands can be processed on the GPU, and
    /// acquires a fence associated with the command buffer.
    ///
    /// You must release this fence when it is no longer needed or it will cause a
    /// leak. It is invalid to use the command buffer after this is called.
    ///
    /// This must be called from the thread the command buffer was acquired on.
    ///
    /// All commands in the submission are guaranteed to begin executing before any
    /// command in a subsequent submission begins executing.
    ///
    /// \param command_buffer a command buffer.
    /// \returns a fence associated with the command buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AcquireGPUCommandBuffer
    /// \sa SDL_AcquireGPUSwapchainTexture
    /// \sa SDL_SubmitGPUCommandBuffer
    /// \sa SDL_ReleaseGPUFence
    pub fn SDL_SubmitGPUCommandBufferAndAcquireFence(command_buffer: *mut SDL_GPUCommandBuffer) -> *mut SDL_GPUFence;
}}

extern_sdlcall! {{
    /// Blocks the thread until the GPU is completely idle.
    ///
    /// \param device a GPU context.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_WaitForGPUFences
    pub fn SDL_WaitForGPUIdle(device: *mut SDL_GPUDevice);
}}

extern_sdlcall! {{
    /// Blocks the thread until the given fences are signaled.
    ///
    /// \param device a GPU context.
    /// \param wait_all if 0, wait for any fence to be signaled, if 1, wait for all
    ///                 fences to be signaled.
    /// \param fences an array of fences to wait on.
    /// \param num_fences the number of fences in the fences array.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    /// \sa SDL_WaitForGPUIdle
    pub fn SDL_WaitForGPUFences(device: *mut SDL_GPUDevice, wait_all: SDL_bool, fences: *const *mut SDL_GPUFence, num_fences: Uint32);
}}

extern_sdlcall! {{
    /// Checks the status of a fence.
    ///
    /// \param device a GPU context.
    /// \param fence a fence.
    /// \returns SDL_TRUE if the fence is signaled, SDL_FALSE if it is not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    pub fn SDL_QueryGPUFence(device: *mut SDL_GPUDevice, fence: *mut SDL_GPUFence) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Releases a fence obtained from SDL_SubmitGPUCommandBufferAndAcquireFence.
    ///
    /// \param device a GPU context.
    /// \param fence a fence.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
    pub fn SDL_ReleaseGPUFence(device: *mut SDL_GPUDevice, fence: *mut SDL_GPUFence);
}}

extern_sdlcall! {{
    /// Obtains the texel block size for a texture format.
    ///
    /// \param format the texture format you want to know the texel size of.
    /// \returns the texel block size of the texture format.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_UploadToGPUTexture
    pub fn SDL_GPUTextureFormatTexelBlockSize(format: SDL_GPUTextureFormat) -> Uint32;
}}

extern_sdlcall! {{
    /// Determines whether a texture format is supported for a given type and
    /// usage.
    ///
    /// \param device a GPU context.
    /// \param format the texture format to check.
    /// \param type the type of texture (2D, 3D, Cube).
    /// \param usage a bitmask of all usage scenarios to check.
    /// \returns whether the texture format is supported for this type and usage.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GPUTextureSupportsFormat(device: *mut SDL_GPUDevice, format: SDL_GPUTextureFormat, r#type: SDL_GPUTextureType, usage: SDL_GPUTextureUsageFlags) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Determines if a sample count for a texture format is supported.
    ///
    /// \param device a GPU context.
    /// \param format the texture format to check.
    /// \param sample_count the sample count to check.
    /// \returns a hardware-specific version of min(preferred, possible).
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GPUTextureSupportsSampleCount(device: *mut SDL_GPUDevice, format: SDL_GPUTextureFormat, sample_count: SDL_GPUSampleCount) -> SDL_bool;
}}

#[cfg(any(/* always disabled: SDL_PLATFORM_GDK */))]
emit! {
    extern_sdlcall! {{
        /// Call this to suspend GPU operation on Xbox when you receive the
        /// SDL_EVENT_DID_ENTER_BACKGROUND event.
        ///
        /// Do NOT call any SDL_GPU functions after calling this function! This must
        /// also be called before calling SDL_GDKSuspendComplete.
        ///
        /// \param device a GPU context.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_AddEventWatch
        pub fn SDL_GDKSuspendGPU(device: *mut SDL_GPUDevice);
    }}

    extern_sdlcall! {{
        /// Call this to resume GPU operation on Xbox when you receive the
        /// SDL_EVENT_WILL_ENTER_FOREGROUND event.
        ///
        /// When resuming, this function MUST be called before calling any other
        /// SDL_GPU functions.
        ///
        /// \param device a GPU context.
        ///
        /// \since This function is available since SDL 3.0.0.
        ///
        /// \sa SDL_AddEventWatch
        pub fn SDL_GDKResumeGPU(device: *mut SDL_GPUDevice);
    }}

}

/// An opaque handle representing a buffer.
///
/// Used for vertices, indices, indirect draw commands, and general compute
/// data.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUBuffer
/// \sa SDL_SetGPUBufferName
/// \sa SDL_UploadToGPUBuffer
/// \sa SDL_DownloadFromGPUBuffer
/// \sa SDL_CopyGPUBufferToBuffer
/// \sa SDL_BindGPUVertexBuffers
/// \sa SDL_BindGPUIndexBuffer
/// \sa SDL_BindGPUVertexStorageBuffers
/// \sa SDL_BindGPUFragmentStorageBuffers
/// \sa SDL_DrawGPUPrimitivesIndirect
/// \sa SDL_DrawGPUIndexedPrimitivesIndirect
/// \sa SDL_BindGPUComputeStorageBuffers
/// \sa SDL_DispatchGPUComputeIndirect
/// \sa SDL_ReleaseGPUBuffer
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUBuffer { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a command buffer.
///
/// Most state is managed via command buffers. When setting state using a
/// command buffer, that state is local to the command buffer.
///
/// Commands only begin execution on the GPU once SDL_SubmitGPUCommandBuffer is
/// called. Once the command buffer is submitted, it is no longer valid to use
/// it.
///
/// Command buffers are executed in submission order. If you submit command
/// buffer A and then command buffer B all commands in A will begin executing
/// before any command in B begins executing.
///
/// In multi-threading scenarios, you should acquire and submit a command
/// buffer on the same thread. As long as you satisfy this requirement, all
/// functionality related to command buffers is thread-safe.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_AcquireGPUCommandBuffer
/// \sa SDL_SubmitGPUCommandBuffer
/// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUCommandBuffer { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a compute pass.
///
/// This handle is transient and should not be held or referenced after
/// SDL_EndGPUComputePass is called.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_BeginGPUComputePass
/// \sa SDL_EndGPUComputePass
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUComputePass { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a compute pipeline.
///
/// Used during compute passes.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUComputePipeline
/// \sa SDL_BindGPUComputePipeline
/// \sa SDL_ReleaseGPUComputePipeline
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUComputePipeline { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a copy pass.
///
/// This handle is transient and should not be held or referenced after
/// SDL_EndGPUCopyPass is called.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_BeginGPUCopyPass
/// \sa SDL_EndGPUCopyPass
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUCopyPass { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing the SDL_GPU context.
///
/// \since This struct is available since SDL 3.0.0
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUDevice { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a fence.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_SubmitGPUCommandBufferAndAcquireFence
/// \sa SDL_QueryGPUFence
/// \sa SDL_WaitForGPUFences
/// \sa SDL_ReleaseGPUFence
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUFence { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a graphics pipeline.
///
/// Used during render passes.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUGraphicsPipeline
/// \sa SDL_BindGPUGraphicsPipeline
/// \sa SDL_ReleaseGPUGraphicsPipeline
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUGraphicsPipeline { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a render pass.
///
/// This handle is transient and should not be held or referenced after
/// SDL_EndGPURenderPass is called.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_BeginGPURenderPass
/// \sa SDL_EndGPURenderPass
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPURenderPass { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a sampler.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUSampler
/// \sa SDL_BindGPUVertexSamplers
/// \sa SDL_BindGPUFragmentSamplers
/// \sa SDL_ReleaseGPUSampler
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUSampler { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a compiled shader object.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUShader
/// \sa SDL_CreateGPUGraphicsPipeline
/// \sa SDL_ReleaseGPUShader
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUShader { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a texture.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTexture
/// \sa SDL_SetGPUTextureName
/// \sa SDL_UploadToGPUTexture
/// \sa SDL_DownloadFromGPUTexture
/// \sa SDL_CopyGPUTextureToTexture
/// \sa SDL_BindGPUVertexSamplers
/// \sa SDL_BindGPUVertexStorageTextures
/// \sa SDL_BindGPUFragmentSamplers
/// \sa SDL_BindGPUFragmentStorageTextures
/// \sa SDL_BindGPUComputeStorageTextures
/// \sa SDL_GenerateMipmapsForGPUTexture
/// \sa SDL_BlitGPUTexture
/// \sa SDL_ReleaseGPUTexture
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUTexture { _opaque: [::core::primitive::u8; 0] }

/// An opaque handle representing a transfer buffer.
///
/// Used for transferring data to and from the device.
///
/// \since This struct is available since SDL 3.0.0
///
/// \sa SDL_CreateGPUTransferBuffer
/// \sa SDL_MapGPUTransferBuffer
/// \sa SDL_UnmapGPUTransferBuffer
/// \sa SDL_UploadToGPUBuffer
/// \sa SDL_UploadToGPUTexture
/// \sa SDL_DownloadFromGPUBuffer
/// \sa SDL_DownloadFromGPUTexture
/// \sa SDL_ReleaseGPUTransferBuffer
#[repr(C)]
#[non_exhaustive]
pub struct SDL_GPUTransferBuffer { _opaque: [::core::primitive::u8; 0] }

