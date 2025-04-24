//! Metadata for items in the `crate::ttf` module

use super::*;

pub static METADATA_TTF_FontStyleFlags: Group = Group {
    module: "ttf",
    kind: GroupKind::Flags,
    name: "TTF_FontStyleFlags",
    short_name: "FontStyleFlags",
    doc: Some("Font style flags for [`TTF_Font`]\n\nThese are the flags which can be used to set the style of a font in\nSDL_ttf. A combination of these flags can be used with functions that set\nor query font style, such as [`TTF_SetFontStyle`] or [`TTF_GetFontStyle`].\n\n## Availability\nThis datatype is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontStyle`]\n- [`TTF_GetFontStyle`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_STYLE_NORMAL",
            short_name: "NORMAL",
            doc: Some("No special style\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_STYLE_BOLD",
            short_name: "BOLD",
            doc: Some("Bold style\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_STYLE_ITALIC",
            short_name: "ITALIC",
            doc: Some("Italic style\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_STYLE_UNDERLINE",
            short_name: "UNDERLINE",
            doc: Some("Underlined text\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_STYLE_STRIKETHROUGH",
            short_name: "STRIKETHROUGH",
            doc: Some("Strikethrough text\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_HintingFlags: Group = Group {
    module: "ttf",
    kind: GroupKind::Enum,
    name: "TTF_HintingFlags",
    short_name: "HintingFlags",
    doc: Some("Hinting flags for TTF (TrueType Fonts)\n\nThis enum specifies the level of hinting to be applied to the font\nrendering. The hinting level determines how much the font's outlines are\nadjusted for better alignment on the pixel grid.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontHinting`]\n- [`TTF_GetFontHinting`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_HINTING_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_HINTING_NORMAL",
            short_name: "NORMAL",
            doc: Some("Normal hinting applies standard grid-fitting.\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_HINTING_LIGHT",
            short_name: "LIGHT",
            doc: Some("Light hinting applies subtle adjustments to improve rendering.\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_HINTING_MONO",
            short_name: "MONO",
            doc: Some("Monochrome hinting adjusts the font for better rendering at lower resolutions.\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_HINTING_NONE",
            short_name: "NONE",
            doc: Some("No hinting, the font is rendered without any grid-fitting.\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_HINTING_LIGHT_SUBPIXEL",
            short_name: "LIGHT_SUBPIXEL",
            doc: Some("Light hinting with subpixel rendering for more precise font edges.\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_HorizontalAlignment: Group = Group {
    module: "ttf",
    kind: GroupKind::Enum,
    name: "TTF_HorizontalAlignment",
    short_name: "HorizontalAlignment",
    doc: Some("The horizontal alignment used when rendering wrapped text.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_HORIZONTAL_ALIGN_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_HORIZONTAL_ALIGN_LEFT",
            short_name: "LEFT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_HORIZONTAL_ALIGN_CENTER",
            short_name: "CENTER",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_HORIZONTAL_ALIGN_RIGHT",
            short_name: "RIGHT",
            doc: None,
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_Direction: Group = Group {
    module: "ttf",
    kind: GroupKind::Enum,
    name: "TTF_Direction",
    short_name: "Direction",
    doc: Some("Direction flags\n\nThe values here are chosen to match\n[hb_direction_t](https://harfbuzz.github.io/harfbuzz-hb-common.html#hb-direction-t)\n.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontDirection`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_DIRECTION_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_DIRECTION_LTR",
            short_name: "LTR",
            doc: Some("Left to Right\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_DIRECTION_RTL",
            short_name: "RTL",
            doc: Some("Right to Left\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_DIRECTION_TTB",
            short_name: "TTB",
            doc: Some("Top to Bottom\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_DIRECTION_BTT",
            short_name: "BTT",
            doc: Some("Bottom to Top\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_ImageType: Group = Group {
    module: "ttf",
    kind: GroupKind::Enum,
    name: "TTF_ImageType",
    short_name: "ImageType",
    doc: Some("The type of data in a glyph image\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_IMAGE_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_IMAGE_ALPHA",
            short_name: "ALPHA",
            doc: Some("The color channels are white\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_IMAGE_COLOR",
            short_name: "COLOR",
            doc: Some("The color channels have image data\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_IMAGE_SDF",
            short_name: "SDF",
            doc: Some("The alpha channel has signed distance field information\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_GPUTextEngineWinding: Group = Group {
    module: "ttf",
    kind: GroupKind::Enum,
    name: "TTF_GPUTextEngineWinding",
    short_name: "GPUTextEngineWinding",
    doc: Some("The winding order of the vertices returned by [`TTF_GetGPUTextDrawData`]\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_GPU_TEXTENGINE_WINDING_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_GPU_TEXTENGINE_WINDING_CLOCKWISE",
            short_name: "CLOCKWISE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_GPU_TEXTENGINE_WINDING_COUNTER_CLOCKWISE",
            short_name: "COUNTER_CLOCKWISE",
            doc: None,
            available_since: None,
        },
    ],
};
pub static METADATA_TTF_SubStringFlags: Group = Group {
    module: "ttf",
    kind: GroupKind::Flags,
    name: "TTF_SubStringFlags",
    short_name: "SubStringFlags",
    doc: Some("Flags for [`TTF_SubString`]\n\n## Availability\nThis datatype is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SubString`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_SUBSTRING_DIRECTION_MASK",
            short_name: "DIRECTION_MASK",
            doc: Some("The mask for the flow direction for this substring\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_SUBSTRING_TEXT_START",
            short_name: "TEXT_START",
            doc: Some("This substring contains the beginning of the text\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_SUBSTRING_LINE_START",
            short_name: "LINE_START",
            doc: Some("This substring contains the beginning of line `line_index`\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_SUBSTRING_LINE_END",
            short_name: "LINE_END",
            doc: Some("This substring contains the end of line `line_index`\n"),
            available_since: None,
        },
        GroupValue {
            name: "TTF_SUBSTRING_TEXT_END",
            short_name: "TEXT_END",
            doc: Some("This substring contains the end of the text\n"),
            available_since: None,
        },
    ],
};
