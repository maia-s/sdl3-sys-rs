//! Metadata for items in the `crate::blendmode` module

use super::*;

pub const METADATA_SDL_BlendMode: Group = Group {
    module: "blendmode",
    kind: GroupKind::Flags,
    name: "SDL_BlendMode",
    short_name: "BlendMode",
    doc: Some(
        "A set of blend modes used in drawing operations.\n\nThese predefined blend modes are supported everywhere.\n\nAdditional values may be obtained from [`SDL_ComposeCustomBlendMode`].\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_ComposeCustomBlendMode`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_BLENDMODE_NONE",
            short_name: "NONE",
            doc: Some("no blending: dstRGBA = srcRGBA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_BLEND",
            short_name: "BLEND",
            doc: Some(
                "alpha blending: dstRGB = (srcRGB * srcA) + (dstRGB * (1-srcA)), dstA = srcA + (dstA * (1-srcA))\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_BLEND_PREMULTIPLIED",
            short_name: "BLEND_PREMULTIPLIED",
            doc: Some("pre-multiplied alpha blending: dstRGBA = srcRGBA + (dstRGBA * (1-srcA))\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_ADD",
            short_name: "ADD",
            doc: Some("additive blending: dstRGB = (srcRGB * srcA) + dstRGB, dstA = dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_ADD_PREMULTIPLIED",
            short_name: "ADD_PREMULTIPLIED",
            doc: Some("pre-multiplied additive blending: dstRGB = srcRGB + dstRGB, dstA = dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_MOD",
            short_name: "MOD",
            doc: Some("color modulate: dstRGB = srcRGB * dstRGB, dstA = dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_MUL",
            short_name: "MUL",
            doc: Some(
                "color multiply: dstRGB = (srcRGB * dstRGB) + (dstRGB * (1-srcA)), dstA = dstA\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDMODE_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_BlendOperation: Group = Group {
    module: "blendmode",
    kind: GroupKind::Enum,
    name: "SDL_BlendOperation",
    short_name: "BlendOperation",
    doc: Some(
        "The blend operation used when combining source and destination pixel\ncomponents.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_BLENDOPERATION_ADD",
            short_name: "ADD",
            doc: Some("dst + src: supported by all renderers\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDOPERATION_SUBTRACT",
            short_name: "SUBTRACT",
            doc: Some("src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDOPERATION_REV_SUBTRACT",
            short_name: "REV_SUBTRACT",
            doc: Some("dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDOPERATION_MINIMUM",
            short_name: "MINIMUM",
            doc: Some("min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDOPERATION_MAXIMUM",
            short_name: "MAXIMUM",
            doc: Some("max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_BlendFactor: Group = Group {
    module: "blendmode",
    kind: GroupKind::Enum,
    name: "SDL_BlendFactor",
    short_name: "BlendFactor",
    doc: Some(
        "The normalized factor used to multiply pixel components.\n\nThe blend factors are multiplied with the pixels from a drawing operation\n(src) and the pixels from the render target (dst) before the blend\noperation. The comma-separated factors listed above are always applied in\nthe component order red, green, blue, and alpha.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_BLENDFACTOR_ZERO",
            short_name: "ZERO",
            doc: Some("0, 0, 0, 0\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_ONE",
            short_name: "ONE",
            doc: Some("1, 1, 1, 1\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_SRC_COLOR",
            short_name: "SRC_COLOR",
            doc: Some("srcR, srcG, srcB, srcA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR",
            short_name: "ONE_MINUS_SRC_COLOR",
            doc: Some("1-srcR, 1-srcG, 1-srcB, 1-srcA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_SRC_ALPHA",
            short_name: "SRC_ALPHA",
            doc: Some("srcA, srcA, srcA, srcA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA",
            short_name: "ONE_MINUS_SRC_ALPHA",
            doc: Some("1-srcA, 1-srcA, 1-srcA, 1-srcA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_DST_COLOR",
            short_name: "DST_COLOR",
            doc: Some("dstR, dstG, dstB, dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR",
            short_name: "ONE_MINUS_DST_COLOR",
            doc: Some("1-dstR, 1-dstG, 1-dstB, 1-dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_DST_ALPHA",
            short_name: "DST_ALPHA",
            doc: Some("dstA, dstA, dstA, dstA\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA",
            short_name: "ONE_MINUS_DST_ALPHA",
            doc: Some("1-dstA, 1-dstA, 1-dstA, 1-dstA\n"),
            available_since: None,
        },
    ],
};
