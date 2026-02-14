#![allow(non_upper_case_globals, unused)]

use core::ffi::CStr;
use sdl3_sys::{
    metadata::{
        Field, Group, GroupKind, GroupValue, Hint, Property, PropertyType, Struct, StructKind,
    },
    version::SDL_VERSIONNUM,
};

pub mod textengine;
pub mod ttf;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::textengine::*;
    #[doc(no_inline)]
    pub use super::ttf::*;
}

/// Metadata for hint constants in this crate
pub const HINTS: &[&Hint] = &[];

/// Metadata for property constants in this crate
pub const PROPERTIES: &[&Property] = &[
    &ttf::METADATA_TTF_PROP_FONT_CREATE_FILENAME_STRING,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_IOSTREAM_POINTER,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_IOSTREAM_OFFSET_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_SIZE_FLOAT,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_FACE_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_CREATE_EXISTING_FONT,
    &ttf::METADATA_TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER,
    &ttf::METADATA_TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER,
    &ttf::METADATA_TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER,
    &ttf::METADATA_TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE,
    &ttf::METADATA_TTF_PROP_GPU_TEXT_ENGINE_DEVICE,
    &ttf::METADATA_TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE,
];

/// Metadata for groups in this crate
pub const GROUPS: &[&Group] = &[
    &textengine::METADATA_TTF_DrawCommand,
    &ttf::METADATA_TTF_FontStyleFlags,
    &ttf::METADATA_TTF_HintingFlags,
    &ttf::METADATA_TTF_HorizontalAlignment,
    &ttf::METADATA_TTF_Direction,
    &ttf::METADATA_TTF_ImageType,
    &ttf::METADATA_TTF_GPUTextEngineWinding,
    &ttf::METADATA_TTF_SubStringFlags,
];

/// Metadata for structs and unions in this crate
pub const STRUCTS: &[&Struct] = &[
    &textengine::METADATA_TTF_FillOperation,
    &textengine::METADATA_TTF_CopyOperation,
    &textengine::METADATA_TTF_DrawOperation,
    &textengine::METADATA_TTF_TextData,
    &textengine::METADATA_TTF_TextEngine,
    &ttf::METADATA_TTF_Text,
    &ttf::METADATA_TTF_GPUAtlasDrawSequence,
    &ttf::METADATA_TTF_SubString,
];
