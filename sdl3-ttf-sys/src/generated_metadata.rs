#![allow(non_upper_case_globals, unused)]

use sdl3_sys::{
    metadata::{Group, GroupKind, GroupValue, Hint, Property},
    properties::SDL_PropertyType,
};

/// Metadata for hint constants in this crate
pub static HINTS: &[Hint] = &[];

/// Metadata for property constants in this crate
pub static PROPERTIES: &[Property] = &[];

/// Metadata for groups in this crate
pub static GROUPS: &[Group] = &[
    Group {
        module: "textengine",
        kind: GroupKind::Enum,
        name: "TTF_DrawCommand",
        short_name: "DrawCommand",
        doc: "A font atlas draw command.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n",
        values: &[
            GroupValue {
                name: "TTF_DRAW_COMMAND_NOOP",
                short_name: "NOOP",
                doc: "",
            },
            GroupValue {
                name: "TTF_DRAW_COMMAND_FILL",
                short_name: "FILL",
                doc: "",
            },
            GroupValue {
                name: "TTF_DRAW_COMMAND_COPY",
                short_name: "COPY",
                doc: "",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Flags,
        name: "TTF_FontStyleFlags",
        short_name: "FontStyleFlags",
        doc: "Font style flags for [`TTF_Font`]\n\nThese are the flags which can be used to set the style of a font in\nSDL_ttf. A combination of these flags can be used with functions that set\nor query font style, such as [`TTF_SetFontStyle`] or [`TTF_GetFontStyle`].\n\n## Availability\nThis datatype is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontStyle`]\n- [`TTF_GetFontStyle`]\n",
        values: &[
            GroupValue {
                name: "TTF_STYLE_NORMAL",
                short_name: "NORMAL",
                doc: "No special style\n",
            },
            GroupValue {
                name: "TTF_STYLE_BOLD",
                short_name: "BOLD",
                doc: "Bold style\n",
            },
            GroupValue {
                name: "TTF_STYLE_ITALIC",
                short_name: "ITALIC",
                doc: "Italic style\n",
            },
            GroupValue {
                name: "TTF_STYLE_UNDERLINE",
                short_name: "UNDERLINE",
                doc: "Underlined text\n",
            },
            GroupValue {
                name: "TTF_STYLE_STRIKETHROUGH",
                short_name: "STRIKETHROUGH",
                doc: "Strikethrough text\n",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Enum,
        name: "TTF_HintingFlags",
        short_name: "HintingFlags",
        doc: "Hinting flags for TTF (TrueType Fonts)\n\nThis enum specifies the level of hinting to be applied to the font\nrendering. The hinting level determines how much the font's outlines are\nadjusted for better alignment on the pixel grid.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontHinting`]\n- [`TTF_GetFontHinting`]\n",
        values: &[
            GroupValue {
                name: "TTF_HINTING_INVALID",
                short_name: "INVALID",
                doc: "",
            },
            GroupValue {
                name: "TTF_HINTING_NORMAL",
                short_name: "NORMAL",
                doc: "Normal hinting applies standard grid-fitting.\n",
            },
            GroupValue {
                name: "TTF_HINTING_LIGHT",
                short_name: "LIGHT",
                doc: "Light hinting applies subtle adjustments to improve rendering.\n",
            },
            GroupValue {
                name: "TTF_HINTING_MONO",
                short_name: "MONO",
                doc: "Monochrome hinting adjusts the font for better rendering at lower resolutions.\n",
            },
            GroupValue {
                name: "TTF_HINTING_NONE",
                short_name: "NONE",
                doc: "No hinting, the font is rendered without any grid-fitting.\n",
            },
            GroupValue {
                name: "TTF_HINTING_LIGHT_SUBPIXEL",
                short_name: "LIGHT_SUBPIXEL",
                doc: "Light hinting with subpixel rendering for more precise font edges.\n",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Enum,
        name: "TTF_HorizontalAlignment",
        short_name: "HorizontalAlignment",
        doc: "The horizontal alignment used when rendering wrapped text.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n",
        values: &[
            GroupValue {
                name: "TTF_HORIZONTAL_ALIGN_INVALID",
                short_name: "INVALID",
                doc: "",
            },
            GroupValue {
                name: "TTF_HORIZONTAL_ALIGN_LEFT",
                short_name: "LEFT",
                doc: "",
            },
            GroupValue {
                name: "TTF_HORIZONTAL_ALIGN_CENTER",
                short_name: "CENTER",
                doc: "",
            },
            GroupValue {
                name: "TTF_HORIZONTAL_ALIGN_RIGHT",
                short_name: "RIGHT",
                doc: "",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Enum,
        name: "TTF_Direction",
        short_name: "Direction",
        doc: "Direction flags\n\nThe values here are chosen to match\n[hb_direction_t](https://harfbuzz.github.io/harfbuzz-hb-common.html#hb-direction-t)\n.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SetFontDirection`]\n",
        values: &[
            GroupValue {
                name: "TTF_DIRECTION_INVALID",
                short_name: "INVALID",
                doc: "",
            },
            GroupValue {
                name: "TTF_DIRECTION_LTR",
                short_name: "LTR",
                doc: "Left to Right\n",
            },
            GroupValue {
                name: "TTF_DIRECTION_RTL",
                short_name: "RTL",
                doc: "Right to Left\n",
            },
            GroupValue {
                name: "TTF_DIRECTION_TTB",
                short_name: "TTB",
                doc: "Top to Bottom\n",
            },
            GroupValue {
                name: "TTF_DIRECTION_BTT",
                short_name: "BTT",
                doc: "Bottom to Top\n",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Enum,
        name: "TTF_ImageType",
        short_name: "ImageType",
        doc: "The type of data in a glyph image\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n",
        values: &[
            GroupValue {
                name: "TTF_IMAGE_INVALID",
                short_name: "INVALID",
                doc: "",
            },
            GroupValue {
                name: "TTF_IMAGE_ALPHA",
                short_name: "ALPHA",
                doc: "The color channels are white\n",
            },
            GroupValue {
                name: "TTF_IMAGE_COLOR",
                short_name: "COLOR",
                doc: "The color channels have image data\n",
            },
            GroupValue {
                name: "TTF_IMAGE_SDF",
                short_name: "SDF",
                doc: "The alpha channel has signed distance field information\n",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Enum,
        name: "TTF_GPUTextEngineWinding",
        short_name: "GPUTextEngineWinding",
        doc: "The winding order of the vertices returned by [`TTF_GetGPUTextDrawData`]\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n",
        values: &[
            GroupValue {
                name: "TTF_GPU_TEXTENGINE_WINDING_INVALID",
                short_name: "INVALID",
                doc: "",
            },
            GroupValue {
                name: "TTF_GPU_TEXTENGINE_WINDING_CLOCKWISE",
                short_name: "CLOCKWISE",
                doc: "",
            },
            GroupValue {
                name: "TTF_GPU_TEXTENGINE_WINDING_COUNTER_CLOCKWISE",
                short_name: "COUNTER_CLOCKWISE",
                doc: "",
            },
        ],
    },
    Group {
        module: "ttf",
        kind: GroupKind::Flags,
        name: "TTF_SubStringFlags",
        short_name: "SubStringFlags",
        doc: "Flags for [`TTF_SubString`]\n\n## Availability\nThis datatype is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_SubString`]\n",
        values: &[
            GroupValue {
                name: "TTF_SUBSTRING_DIRECTION_MASK",
                short_name: "DIRECTION_MASK",
                doc: "The mask for the flow direction for this substring\n",
            },
            GroupValue {
                name: "TTF_SUBSTRING_TEXT_START",
                short_name: "TEXT_START",
                doc: "This substring contains the beginning of the text\n",
            },
            GroupValue {
                name: "TTF_SUBSTRING_LINE_START",
                short_name: "LINE_START",
                doc: "This substring contains the beginning of line `line_index`\n",
            },
            GroupValue {
                name: "TTF_SUBSTRING_LINE_END",
                short_name: "LINE_END",
                doc: "This substring contains the end of line `line_index`\n",
            },
            GroupValue {
                name: "TTF_SUBSTRING_TEXT_END",
                short_name: "TEXT_END",
                doc: "This substring contains the end of the text\n",
            },
        ],
    },
];

pub(crate) const GROUP_OFFSET_textengine: usize = 0;
pub(crate) const GROUP_OFFSET_ttf: usize = 1;
