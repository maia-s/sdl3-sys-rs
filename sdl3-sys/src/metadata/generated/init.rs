//! Metadata for items in the `crate::init` module

use super::*;

pub const METADATA_SDL_PROP_APP_METADATA_NAME_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_NAME_STRING",
    short_name: "APP_METADATA_NAME_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_NAME_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_VERSION_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_VERSION_STRING",
    short_name: "APP_METADATA_VERSION_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_VERSION_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_IDENTIFIER_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_IDENTIFIER_STRING",
    short_name: "APP_METADATA_IDENTIFIER_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_IDENTIFIER_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_CREATOR_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_CREATOR_STRING",
    short_name: "APP_METADATA_CREATOR_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_CREATOR_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_COPYRIGHT_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_COPYRIGHT_STRING",
    short_name: "APP_METADATA_COPYRIGHT_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_COPYRIGHT_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_URL_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_URL_STRING",
    short_name: "APP_METADATA_URL_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_URL_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_APP_METADATA_TYPE_STRING: Property = Property {
    module: "init",
    name: "SDL_PROP_APP_METADATA_TYPE_STRING",
    short_name: "APP_METADATA_TYPE_STRING",
    value: crate::init::SDL_PROP_APP_METADATA_TYPE_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_InitFlags: Group = Group {
    module: "init",
    kind: GroupKind::Flags,
    name: "SDL_InitFlags",
    short_name: "InitFlags",
    doc: Some(
        "Initialization flags for [`SDL_Init`] and/or [`SDL_InitSubSystem`]\n\nThese are the flags which may be passed to [`SDL_Init()`]. You should specify\nthe subsystems which you will be using in your application.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_Init`]\n- [`SDL_Quit`]\n- [`SDL_InitSubSystem`]\n- [`SDL_QuitSubSystem`]\n- [`SDL_WasInit`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_INIT_AUDIO",
            short_name: "AUDIO",
            doc: Some("[`SDL_INIT_AUDIO`] implies [`SDL_INIT_EVENTS`]\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_VIDEO",
            short_name: "VIDEO",
            doc: Some(
                "[`SDL_INIT_VIDEO`] implies [`SDL_INIT_EVENTS`], should be initialized on the main thread\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_JOYSTICK",
            short_name: "JOYSTICK",
            doc: Some("[`SDL_INIT_JOYSTICK`] implies [`SDL_INIT_EVENTS`]\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_HAPTIC",
            short_name: "HAPTIC",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_GAMEPAD",
            short_name: "GAMEPAD",
            doc: Some("[`SDL_INIT_GAMEPAD`] implies [`SDL_INIT_JOYSTICK`]\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_EVENTS",
            short_name: "EVENTS",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_SENSOR",
            short_name: "SENSOR",
            doc: Some("[`SDL_INIT_SENSOR`] implies [`SDL_INIT_EVENTS`]\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_CAMERA",
            short_name: "CAMERA",
            doc: Some("[`SDL_INIT_CAMERA`] implies [`SDL_INIT_EVENTS`]\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_AppResult: Group = Group {
    module: "init",
    kind: GroupKind::Enum,
    name: "SDL_AppResult",
    short_name: "AppResult",
    doc: Some(
        "Return values for optional main callbacks.\n\nReturning [`SDL_APP_SUCCESS`] or [`SDL_APP_FAILURE`] from [`SDL_AppInit`],\n[`SDL_AppEvent`], or [`SDL_AppIterate`] will terminate the program and report\nsuccess/failure to the operating system. What that means is\nplatform-dependent. On Unix, for example, on success, the process error\ncode will be zero, and on failure it will be 1. This interface doesn't\nallow you to return specific exit codes, just whether there was an error\ngenerally or not.\n\nReturning [`SDL_APP_CONTINUE`] from these functions will let the app continue\nto run.\n\nSee\n[Main callbacks in SDL3](https://wiki.libsdl.org/SDL3/README/main-functions#main-callbacks-in-sdl3)\nfor complete details.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_APP_CONTINUE",
            short_name: "CONTINUE",
            doc: Some("Value that requests that the app continue from the main callbacks.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_APP_SUCCESS",
            short_name: "SUCCESS",
            doc: Some("Value that requests termination with success from the main callbacks.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_APP_FAILURE",
            short_name: "FAILURE",
            doc: Some("Value that requests termination with error from the main callbacks.\n"),
            available_since: None,
        },
    ],
};
