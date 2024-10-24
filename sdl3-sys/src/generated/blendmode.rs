//! Blend modes decide how two colors will mix together. There are both
//! standard modes for basic needs and a means to create custom modes,
//! dictating what sort of math to do what on what color components.

use super::stdinc::*;

/// A set of blend modes used in drawing operations.
///
/// These predefined blend modes are supported everywhere.
///
/// Additional values may be obtained from [`SDL_ComposeCustomBlendMode`].
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ComposeCustomBlendMode`]
pub type SDL_BlendMode = Uint32;

/// no blending: dstRGBA = srcRGBA
pub const SDL_BLENDMODE_NONE: ::core::primitive::u32 = 0_u32;

/// alpha blending: dstRGB = (srcRGB * srcA) + (dstRGB * (1-srcA)), dstA = srcA + (dstA * (1-srcA))
pub const SDL_BLENDMODE_BLEND: ::core::primitive::u32 = 1_u32;

/// pre-multiplied alpha blending: dstRGBA = srcRGBA + (dstRGBA * (1-srcA))
pub const SDL_BLENDMODE_BLEND_PREMULTIPLIED: ::core::primitive::u32 = 16_u32;

/// additive blending: dstRGB = (srcRGB * srcA) + dstRGB, dstA = dstA
pub const SDL_BLENDMODE_ADD: ::core::primitive::u32 = 2_u32;

/// pre-multiplied additive blending: dstRGB = srcRGB + dstRGB, dstA = dstA
pub const SDL_BLENDMODE_ADD_PREMULTIPLIED: ::core::primitive::u32 = 32_u32;

/// color modulate: dstRGB = srcRGB * dstRGB, dstA = dstA
pub const SDL_BLENDMODE_MOD: ::core::primitive::u32 = 4_u32;

/// color multiply: dstRGB = (srcRGB * dstRGB) + (dstRGB * (1-srcA)), dstA = dstA
pub const SDL_BLENDMODE_MUL: ::core::primitive::u32 = 8_u32;

pub const SDL_BLENDMODE_INVALID: ::core::primitive::u32 = 2147483647_u32;

/// The blend operation used when combining source and destination pixel
/// components.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`ADD`](SDL_BlendOperation::ADD) | [`SDL_BLENDOPERATION_ADD`] | dst + src: supported by all renderers |
/// | [`SUBTRACT`](SDL_BlendOperation::SUBTRACT) | [`SDL_BLENDOPERATION_SUBTRACT`] | src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan |
/// | [`REV_SUBTRACT`](SDL_BlendOperation::REV_SUBTRACT) | [`SDL_BLENDOPERATION_REV_SUBTRACT`] | dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan |
/// | [`MINIMUM`](SDL_BlendOperation::MINIMUM) | [`SDL_BLENDOPERATION_MINIMUM`] | min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan |
/// | [`MAXIMUM`](SDL_BlendOperation::MAXIMUM) | [`SDL_BLENDOPERATION_MAXIMUM`] | max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_BlendOperation(pub ::core::ffi::c_int);
impl From<SDL_BlendOperation> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_BlendOperation) -> Self {
        value.0
    }
}
impl SDL_BlendOperation {
    /// dst + src: supported by all renderers
    pub const ADD: Self = Self(0x1);
    /// src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const SUBTRACT: Self = Self(0x2);
    /// dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const REV_SUBTRACT: Self = Self(0x3);
    /// min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const MINIMUM: Self = Self(0x4);
    /// max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const MAXIMUM: Self = Self(0x5);
}
/// dst + src: supported by all renderers
pub const SDL_BLENDOPERATION_ADD: SDL_BlendOperation = SDL_BlendOperation::ADD;
/// src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_SUBTRACT: SDL_BlendOperation = SDL_BlendOperation::SUBTRACT;
/// dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_REV_SUBTRACT: SDL_BlendOperation = SDL_BlendOperation::REV_SUBTRACT;
/// min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_MINIMUM: SDL_BlendOperation = SDL_BlendOperation::MINIMUM;
/// max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_MAXIMUM: SDL_BlendOperation = SDL_BlendOperation::MAXIMUM;

/// The normalized factor used to multiply pixel components.
///
/// The blend factors are multiplied with the pixels from a drawing operation
/// (src) and the pixels from the render target (dst) before the blend
/// operation. The comma-separated factors listed above are always applied in
/// the component order red, green, blue, and alpha.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`ZERO`](SDL_BlendFactor::ZERO) | [`SDL_BLENDFACTOR_ZERO`] | 0, 0, 0, 0 |
/// | [`ONE`](SDL_BlendFactor::ONE) | [`SDL_BLENDFACTOR_ONE`] | 1, 1, 1, 1 |
/// | [`SRC_COLOR`](SDL_BlendFactor::SRC_COLOR) | [`SDL_BLENDFACTOR_SRC_COLOR`] | srcR, srcG, srcB, srcA |
/// | [`ONE_MINUS_SRC_COLOR`](SDL_BlendFactor::ONE_MINUS_SRC_COLOR) | [`SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR`] | 1-srcR, 1-srcG, 1-srcB, 1-srcA |
/// | [`SRC_ALPHA`](SDL_BlendFactor::SRC_ALPHA) | [`SDL_BLENDFACTOR_SRC_ALPHA`] | srcA, srcA, srcA, srcA |
/// | [`ONE_MINUS_SRC_ALPHA`](SDL_BlendFactor::ONE_MINUS_SRC_ALPHA) | [`SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA`] | 1-srcA, 1-srcA, 1-srcA, 1-srcA |
/// | [`DST_COLOR`](SDL_BlendFactor::DST_COLOR) | [`SDL_BLENDFACTOR_DST_COLOR`] | dstR, dstG, dstB, dstA |
/// | [`ONE_MINUS_DST_COLOR`](SDL_BlendFactor::ONE_MINUS_DST_COLOR) | [`SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR`] | 1-dstR, 1-dstG, 1-dstB, 1-dstA |
/// | [`DST_ALPHA`](SDL_BlendFactor::DST_ALPHA) | [`SDL_BLENDFACTOR_DST_ALPHA`] | dstA, dstA, dstA, dstA |
/// | [`ONE_MINUS_DST_ALPHA`](SDL_BlendFactor::ONE_MINUS_DST_ALPHA) | [`SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA`] | 1-dstA, 1-dstA, 1-dstA, 1-dstA |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_BlendFactor(pub ::core::ffi::c_int);
impl From<SDL_BlendFactor> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_BlendFactor) -> Self {
        value.0
    }
}
impl SDL_BlendFactor {
    /// 0, 0, 0, 0
    pub const ZERO: Self = Self(0x1);
    /// 1, 1, 1, 1
    pub const ONE: Self = Self(0x2);
    /// srcR, srcG, srcB, srcA
    pub const SRC_COLOR: Self = Self(0x3);
    /// 1-srcR, 1-srcG, 1-srcB, 1-srcA
    pub const ONE_MINUS_SRC_COLOR: Self = Self(0x4);
    /// srcA, srcA, srcA, srcA
    pub const SRC_ALPHA: Self = Self(0x5);
    /// 1-srcA, 1-srcA, 1-srcA, 1-srcA
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(0x6);
    /// dstR, dstG, dstB, dstA
    pub const DST_COLOR: Self = Self(0x7);
    /// 1-dstR, 1-dstG, 1-dstB, 1-dstA
    pub const ONE_MINUS_DST_COLOR: Self = Self(0x8);
    /// dstA, dstA, dstA, dstA
    pub const DST_ALPHA: Self = Self(0x9);
    /// 1-dstA, 1-dstA, 1-dstA, 1-dstA
    pub const ONE_MINUS_DST_ALPHA: Self = Self(0xa);
}
/// 0, 0, 0, 0
pub const SDL_BLENDFACTOR_ZERO: SDL_BlendFactor = SDL_BlendFactor::ZERO;
/// 1, 1, 1, 1
pub const SDL_BLENDFACTOR_ONE: SDL_BlendFactor = SDL_BlendFactor::ONE;
/// srcR, srcG, srcB, srcA
pub const SDL_BLENDFACTOR_SRC_COLOR: SDL_BlendFactor = SDL_BlendFactor::SRC_COLOR;
/// 1-srcR, 1-srcG, 1-srcB, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR: SDL_BlendFactor =
    SDL_BlendFactor::ONE_MINUS_SRC_COLOR;
/// srcA, srcA, srcA, srcA
pub const SDL_BLENDFACTOR_SRC_ALPHA: SDL_BlendFactor = SDL_BlendFactor::SRC_ALPHA;
/// 1-srcA, 1-srcA, 1-srcA, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: SDL_BlendFactor =
    SDL_BlendFactor::ONE_MINUS_SRC_ALPHA;
/// dstR, dstG, dstB, dstA
pub const SDL_BLENDFACTOR_DST_COLOR: SDL_BlendFactor = SDL_BlendFactor::DST_COLOR;
/// 1-dstR, 1-dstG, 1-dstB, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR: SDL_BlendFactor =
    SDL_BlendFactor::ONE_MINUS_DST_COLOR;
/// dstA, dstA, dstA, dstA
pub const SDL_BLENDFACTOR_DST_ALPHA: SDL_BlendFactor = SDL_BlendFactor::DST_ALPHA;
/// 1-dstA, 1-dstA, 1-dstA, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA: SDL_BlendFactor =
    SDL_BlendFactor::ONE_MINUS_DST_ALPHA;

extern "C" {
    /// Compose a custom blend mode for renderers.
    ///
    /// The functions [`SDL_SetRenderDrawBlendMode`] and [`SDL_SetTextureBlendMode`] accept
    /// the [`SDL_BlendMode`] returned by this function if the renderer supports it.
    ///
    /// A blend mode controls how the pixels from a drawing operation (source) get
    /// combined with the pixels from the render target (destination). First, the
    /// components of the source and destination pixels get multiplied with their
    /// blend factors. Then, the blend operation takes the two products and
    /// calculates the result that will get stored in the render target.
    ///
    /// Expressed in pseudocode, it would look like this:
    ///
    /// ```c
    /// dstRGB = colorOperation(srcRGB * srcColorFactor, dstRGB * dstColorFactor);
    /// dstA = alphaOperation(srcA * srcAlphaFactor, dstA * dstAlphaFactor);
    /// ```
    ///
    /// Where the functions `colorOperation(src, dst)` and `alphaOperation(src,
    /// dst)` can return one of the following:
    ///
    /// - `src + dst`
    /// - `src - dst`
    /// - `dst - src`
    /// - `min(src, dst)`
    /// - `max(src, dst)`
    ///
    /// The red, green, and blue components are always multiplied with the first,
    /// second, and third components of the [`SDL_BlendFactor`], respectively. The
    /// fourth component is not used.
    ///
    /// The alpha component is always multiplied with the fourth component of the
    /// [`SDL_BlendFactor`]. The other components are not used in the alpha
    /// calculation.
    ///
    /// Support for these blend modes varies for each renderer. To check if a
    /// specific [`SDL_BlendMode`] is supported, create a renderer and pass it to
    /// either [`SDL_SetRenderDrawBlendMode`] or [`SDL_SetTextureBlendMode`]. They will
    /// return with an error if the blend mode is not supported.
    ///
    /// This list describes the support of custom blend modes for each renderer.
    /// All renderers support the four blend modes listed in the [`SDL_BlendMode`]
    /// enumeration.
    ///
    /// - **direct3d**: Supports all operations with all factors. However, some
    ///   factors produce unexpected results with `SDL_BLENDOPERATION_MINIMUM` and
    ///   `SDL_BLENDOPERATION_MAXIMUM`.
    /// - **direct3d11**: Same as Direct3D 9.
    /// - **opengl**: Supports the `SDL_BLENDOPERATION_ADD` operation with all
    ///   factors. OpenGL versions 1.1, 1.2, and 1.3 do not work correctly here.
    /// - **opengles2**: Supports the `SDL_BLENDOPERATION_ADD`,
    ///   `SDL_BLENDOPERATION_SUBTRACT`, `SDL_BLENDOPERATION_REV_SUBTRACT`
    ///   operations with all factors.
    /// - **psp**: No custom blend mode support.
    /// - **software**: No custom blend mode support.
    ///
    /// Some renderers do not provide an alpha component for the default render
    /// target. The `SDL_BLENDFACTOR_DST_ALPHA` and
    /// `SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA` factors do not have an effect in this
    /// case.
    ///
    /// ### Arguments
    /// - `srcColorFactor`: the [`SDL_BlendFactor`] applied to the red, green, and
    ///   blue components of the source pixels.
    /// - `dstColorFactor`: the [`SDL_BlendFactor`] applied to the red, green, and
    ///   blue components of the destination pixels.
    /// - `colorOperation`: the [`SDL_BlendOperation`] used to combine the red,
    ///   green, and blue components of the source and
    ///   destination pixels.
    /// - `srcAlphaFactor`: the [`SDL_BlendFactor`] applied to the alpha component of
    ///   the source pixels.
    /// - `dstAlphaFactor`: the [`SDL_BlendFactor`] applied to the alpha component of
    ///   the destination pixels.
    /// - `alphaOperation`: the [`SDL_BlendOperation`] used to combine the alpha
    ///   component of the source and destination pixels.
    /// ### Return value
    /// Returns an [`SDL_BlendMode`] that represents the chosen factors and
    ///   operations.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetRenderDrawBlendMode`]
    /// - [`SDL_GetRenderDrawBlendMode`]
    /// - [`SDL_SetTextureBlendMode`]
    /// - [`SDL_GetTextureBlendMode`]
    pub fn SDL_ComposeCustomBlendMode(
        srcColorFactor: SDL_BlendFactor,
        dstColorFactor: SDL_BlendFactor,
        colorOperation: SDL_BlendOperation,
        srcAlphaFactor: SDL_BlendFactor,
        dstAlphaFactor: SDL_BlendFactor,
        alphaOperation: SDL_BlendOperation,
    ) -> SDL_BlendMode;
}

#[cfg(doc)]
use crate::everything::*;
