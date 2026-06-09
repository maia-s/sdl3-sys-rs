//! Metadata for items in the `crate::sound` module

use super::*;

pub const METADATA_Sound_SampleFlags: Group = Group {
    module: "sound",
    kind: GroupKind::Enum,
    name: "Sound_SampleFlags",
    short_name: "SampleFlags",
    doc: Some(
        "Flags that are used in a [`Sound_Sample`] to show various states.\n\n## Availability\nThis enum is available since SDL_sound 1.0.0.\n\n## See also\n- [`Sound_SampleNew`]\n- [`Sound_SampleNewFromFile`]\n- [`Sound_SampleDecode`]\n- [`Sound_SampleDecodeAll`]\n- [`Sound_SampleSeek`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(1, 0, 0)),
    values: &[
        GroupValue {
            name: "SOUND_SAMPLEFLAG_NONE",
            short_name: "NONE",
            doc: Some("No special attributes.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SOUND_SAMPLEFLAG_CANSEEK",
            short_name: "CANSEEK",
            doc: Some("Sample can seek to arbitrary points.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SOUND_SAMPLEFLAG_EOF",
            short_name: "EOF",
            doc: Some("End of input stream.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SOUND_SAMPLEFLAG_ERROR",
            short_name: "ERROR",
            doc: Some("Unrecoverable error.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SOUND_SAMPLEFLAG_EAGAIN",
            short_name: "EAGAIN",
            doc: Some("Function would block, or temp error.\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_Sound_DecoderInfo: Struct = Struct {
    module: "sound",
    kind: StructKind::Struct,
    name: "Sound_DecoderInfo",
    doc: Some(
        "[`Sound_DecoderInfo`] Information about available sound decoders.\n\nEach decoder sets up one of these structs, which can be retrieved via the\n[`Sound_AvailableDecoders()`] function. EVERY FIELD IN THIS IS READ-ONLY.\n\nThe extensions field is a NULL-terminated list of ASCIZ strings. You should\nread it like this:\n\n```c\nfor (const char **ext = info->extensions; *ext != NULL; ext++) {\nprintf(\"   File extension \\\"%s\\\"\\n\", *ext);\n}\n```\n\n## Availability\nThis struct is available since SDL_sound 1.0.0.\n\n## See also\n- [`Sound_AvailableDecoders`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(1, 0, 0)),
    fields: &[
        Field {
            name: "extensions",
            doc: Some("File extensions, list ends with NULL.\n"),
            available_since: None,
            ty: "*mut *const ::core::ffi::c_char",
        },
        Field {
            name: "description",
            doc: Some("Human readable description of decoder.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "author",
            doc: Some("\"Name Of Author <email@emailhost.dom>\"\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "url",
            doc: Some("URL specific to this decoder.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
    ],
};
pub const METADATA_Sound_Sample: Struct = Struct {
    module: "sound",
    kind: StructKind::Struct,
    name: "Sound_Sample",
    doc: Some(
        "Represents sound data in the process of being decoded.\n\nThe [`Sound_Sample`] structure is the heart of SDL_sound. This holds\ninformation about a source of sound data as it is being decoded. EVERY\nFIELD IN THIS IS READ-ONLY. Please use the API functions to change them.\n\n## Availability\nThis struct is available since SDL_sound 1.0.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(1, 0, 0)),
    fields: &[
        Field {
            name: "opaque",
            doc: Some("Internal use only. Don't touch.\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "decoder",
            doc: Some("Decoder used for this sample.\n"),
            available_since: None,
            ty: "*const Sound_DecoderInfo",
        },
        Field {
            name: "desired",
            doc: Some("Desired audio format for conversion.\n"),
            available_since: None,
            ty: "SDL_AudioSpec",
        },
        Field {
            name: "actual",
            doc: Some("Actual audio format of sample.\n"),
            available_since: None,
            ty: "SDL_AudioSpec",
        },
        Field {
            name: "buffer",
            doc: Some("Decoded sound data lands in here.\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "buffer_size",
            doc: Some("Current size of (buffer), in bytes (Uint8).\n"),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "flags",
            doc: Some("Flags relating to this sample.\n"),
            available_since: None,
            ty: "Sound_SampleFlags",
        },
    ],
};
