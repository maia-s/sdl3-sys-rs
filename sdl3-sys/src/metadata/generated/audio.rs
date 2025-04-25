//! Metadata for items in the `crate::audio` module

use super::*;

pub const METADATA_SDL_AudioFormat: Group = Group {
    module: "audio",
    kind: GroupKind::Enum,
    name: "SDL_AudioFormat",
    short_name: "AudioFormat",
    doc: Some("Audio format.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_AUDIO_BITSIZE`]\n- [`SDL_AUDIO_BYTESIZE`]\n- [`SDL_AUDIO_ISINT`]\n- [`SDL_AUDIO_ISFLOAT`]\n- [`SDL_AUDIO_ISBIGENDIAN`]\n- [`SDL_AUDIO_ISLITTLEENDIAN`]\n- [`SDL_AUDIO_ISSIGNED`]\n- [`SDL_AUDIO_ISUNSIGNED`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_AUDIO_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("Unspecified audio format\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_U8",
            short_name: "U8",
            doc: Some("Unsigned 8-bit samples\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S8",
            short_name: "S8",
            doc: Some("Signed 8-bit samples\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S16LE",
            short_name: "S16LE",
            doc: Some("Signed 16-bit samples\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S16BE",
            short_name: "S16BE",
            doc: Some("As above, but big-endian byte order\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S32LE",
            short_name: "S32LE",
            doc: Some("32-bit integer samples\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S32BE",
            short_name: "S32BE",
            doc: Some("As above, but big-endian byte order\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_F32LE",
            short_name: "F32LE",
            doc: Some("32-bit floating point samples\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_F32BE",
            short_name: "F32BE",
            doc: Some("As above, but big-endian byte order\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S16",
            short_name: "S16",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S32",
            short_name: "S32",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_F32",
            short_name: "F32",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S16",
            short_name: "S16",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_S32",
            short_name: "S32",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_AUDIO_F32",
            short_name: "F32",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_AudioDeviceID: Group = Group {
    module: "audio",
    kind: GroupKind::Id,
    name: "SDL_AudioDeviceID",
    short_name: "AudioDeviceID",
    doc: Some("SDL Audio Device instance IDs.\n\nZero is used to signify an invalid/null device.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK",
            short_name: "DEFAULT_PLAYBACK",
            doc: Some("A value used to request a default playback audio device.\n\nSeveral functions that require an [`SDL_AudioDeviceID`] will accept this value\nto signify the app just wants the system to choose a default device instead\nof the app providing a specific one.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n"),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
        GroupValue {
            name: "SDL_AUDIO_DEVICE_DEFAULT_RECORDING",
            short_name: "DEFAULT_RECORDING",
            doc: Some("A value used to request a default recording audio device.\n\nSeveral functions that require an [`SDL_AudioDeviceID`] will accept this value\nto signify the app just wants the system to choose a default device instead\nof the app providing a specific one.\n\n## Availability\nThis macro is available since SDL 3.2.0.\n"),
            available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
        },
    ],
};
