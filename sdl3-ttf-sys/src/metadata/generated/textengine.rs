//! Metadata for items in the `crate::textengine` module

use super::*;

pub const METADATA_TTF_DrawCommand: Group = Group {
    module: "textengine",
    kind: GroupKind::Enum,
    name: "TTF_DrawCommand",
    short_name: "DrawCommand",
    doc: Some(
        "A font atlas draw command.\n\n## Availability\nThis enum is available since SDL_ttf 3.0.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "TTF_DRAW_COMMAND_NOOP",
            short_name: "NOOP",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_DRAW_COMMAND_FILL",
            short_name: "FILL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "TTF_DRAW_COMMAND_COPY",
            short_name: "COPY",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_TTF_FillOperation: Struct = Struct {
    module: "textengine",
    kind: StructKind::Struct,
    name: "TTF_FillOperation",
    doc: Some(
        "A filled rectangle draw operation.\n\n## Availability\nThis struct is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_DrawOperation`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    fields: &[
        Field {
            name: "cmd",
            doc: Some("[`TTF_DRAW_COMMAND_FILL`]\n"),
            available_since: None,
            ty: "TTF_DrawCommand",
        },
        Field {
            name: "rect",
            doc: Some(
                "The rectangle to fill, in pixels. The x coordinate is relative to the left side of the text area, going right, and the y coordinate is relative to the top side of the text area, going down.\n",
            ),
            available_since: None,
            ty: "SDL_Rect",
        },
    ],
};
pub const METADATA_TTF_CopyOperation: Struct = Struct {
    module: "textengine",
    kind: StructKind::Struct,
    name: "TTF_CopyOperation",
    doc: Some(
        "A texture copy draw operation.\n\n## Availability\nThis struct is available since SDL_ttf 3.0.0.\n\n## See also\n- [`TTF_DrawOperation`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    fields: &[
        Field {
            name: "cmd",
            doc: Some("[`TTF_DRAW_COMMAND_COPY`]\n"),
            available_since: None,
            ty: "TTF_DrawCommand",
        },
        Field {
            name: "text_offset",
            doc: Some(
                "The offset in the text corresponding to this glyph.\nThere may be multiple glyphs with the same text offset\nand the next text offset might be several Unicode codepoints\nlater. In this case the glyphs and codepoints are grouped\ntogether and the group bounding box is the union of the dst\nrectangles for the corresponding glyphs.\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "glyph_font",
            doc: Some(
                "The font containing the glyph to be drawn, can be passed to [`TTF_GetGlyphImageForIndex()`]\n",
            ),
            available_since: None,
            ty: "*mut TTF_Font",
        },
        Field {
            name: "glyph_index",
            doc: Some(
                "The glyph index of the glyph to be drawn, can be passed to [`TTF_GetGlyphImageForIndex()`]\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "src",
            doc: Some("The area within the glyph to be drawn\n"),
            available_since: None,
            ty: "SDL_Rect",
        },
        Field {
            name: "dst",
            doc: Some(
                "The drawing coordinates of the glyph, in pixels. The x coordinate is relative to the left side of the text area, going right, and the y coordinate is relative to the top side of the text area, going down.\n",
            ),
            available_since: None,
            ty: "SDL_Rect",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
pub const METADATA_TTF_DrawOperation: Struct = Struct {
    module: "textengine",
    kind: StructKind::Union,
    name: "TTF_DrawOperation",
    doc: Some(
        "A text engine draw operation.\n\n## Availability\nThis struct is available since SDL_ttf 3.0.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    fields: &[
        Field {
            name: "cmd",
            doc: None,
            available_since: None,
            ty: "TTF_DrawCommand",
        },
        Field {
            name: "fill",
            doc: None,
            available_since: None,
            ty: "TTF_FillOperation",
        },
        Field {
            name: "copy",
            doc: None,
            available_since: None,
            ty: "TTF_CopyOperation",
        },
    ],
};
pub const METADATA_TTF_TextData: Struct = Struct {
    module: "textengine",
    kind: StructKind::Struct,
    name: "TTF_TextData",
    doc: None,
    available_since: None,
    fields: &[
        Field {
            name: "font",
            doc: Some("The font used by this text, read-only.\n"),
            available_since: None,
            ty: "*mut TTF_Font",
        },
        Field {
            name: "color",
            doc: Some("The color of the text, read-only.\n"),
            available_since: None,
            ty: "SDL_FColor",
        },
        Field {
            name: "needs_layout_update",
            doc: Some("True if the layout needs to be updated\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "layout",
            doc: Some("Cached layout information, read-only.\n"),
            available_since: None,
            ty: "*mut TTF_TextLayout",
        },
        Field {
            name: "x",
            doc: Some(
                "The x offset of the upper left corner of this text, in pixels, read-only.\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "y",
            doc: Some(
                "The y offset of the upper left corner of this text, in pixels, read-only.\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "w",
            doc: Some("The width of this text, in pixels, read-only.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "h",
            doc: Some("The height of this text, in pixels, read-only.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "num_ops",
            doc: Some("The number of drawing operations to render this text, read-only.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "ops",
            doc: Some("The drawing operations used to render this text, read-only.\n"),
            available_since: None,
            ty: "*mut TTF_DrawOperation",
        },
        Field {
            name: "num_clusters",
            doc: Some(
                "The number of substrings representing clusters of glyphs in the string, read-only\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "clusters",
            doc: Some("Substrings representing clusters of glyphs in the string, read-only\n"),
            available_since: None,
            ty: "*mut TTF_SubString",
        },
        Field {
            name: "props",
            doc: Some(
                "Custom properties associated with this text, read-only. This field is created as-needed using [`TTF_GetTextProperties()`] and the properties may be then set and read normally\n",
            ),
            available_since: None,
            ty: "SDL_PropertiesID",
        },
        Field {
            name: "needs_engine_update",
            doc: Some("True if the engine text needs to be updated\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "engine",
            doc: Some("The engine used to render this text, read-only.\n"),
            available_since: None,
            ty: "*mut TTF_TextEngine",
        },
        Field {
            name: "engine_text",
            doc: Some("The implementation-specific representation of this text\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
pub const METADATA_TTF_TextEngine: Struct = Struct {
    module: "textengine",
    kind: StructKind::Struct,
    name: "TTF_TextEngine",
    doc: Some(
        "A text engine interface.\n\nThis structure should be initialized using [`SDL_INIT_INTERFACE()`]\n\n## Availability\nThis struct is available since SDL_ttf 3.0.0.\n\n## See also\n- [`SDL_INIT_INTERFACE`]\n\n## Notes for `sdl3-sys`\nThis interface struct can be initialized with [`TTF_TextEngine::new()`] or `Default::default()`.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    fields: &[
        Field {
            name: "version",
            doc: Some("The version of this interface\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "userdata",
            doc: Some("User data pointer passed to callbacks\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "CreateText",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, text: *mut TTF_Text) -> ::core::primitive::bool>",
        },
        Field {
            name: "DestroyText",
            doc: Some("* Destroy a text representation.\n"),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, text: *mut TTF_Text)>",
        },
    ],
};
