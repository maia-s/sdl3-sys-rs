//! Metadata for items in the `crate::camera` module

use super::*;

pub const METADATA_SDL_CameraID: Group = Group {
    module: "camera",
    kind: GroupKind::Id,
    name: "SDL_CameraID",
    short_name: "CameraID",
    doc: Some(
        "This is a unique ID for a camera device for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the device is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetCameras`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_CameraPosition: Group = Group {
    module: "camera",
    kind: GroupKind::Enum,
    name: "SDL_CameraPosition",
    short_name: "CameraPosition",
    doc: Some(
        "The position of camera in relation to system device.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetCameraPosition`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_CAMERA_POSITION_UNKNOWN",
            short_name: "UNKNOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAMERA_POSITION_FRONT_FACING",
            short_name: "FRONT_FACING",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAMERA_POSITION_BACK_FACING",
            short_name: "BACK_FACING",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_CameraPermissionState: Group = Group {
    module: "camera",
    kind: GroupKind::Enum,
    name: "SDL_CameraPermissionState",
    short_name: "CameraPermissionState",
    doc: Some(
        "The current state of a request for camera access.\n\n## Availability\nThis enum is available since SDL 3.4.0.\n\n## See also\n- [`SDL_GetCameraPermissionState`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
    values: &[
        GroupValue {
            name: "SDL_CAMERA_PERMISSION_STATE_DENIED",
            short_name: "DENIED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAMERA_PERMISSION_STATE_PENDING",
            short_name: "PENDING",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_CAMERA_PERMISSION_STATE_APPROVED",
            short_name: "APPROVED",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_CameraSpec: Struct = Struct {
    module: "camera",
    kind: StructKind::Struct,
    name: "SDL_CameraSpec",
    doc: Some(
        "The details of an output format for a camera device.\n\nCameras often support multiple formats; each one will be encapsulated in\nthis struct.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetCameraSupportedFormats`]\n- [`SDL_GetCameraFormat`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "format",
            doc: Some("Frame format\n"),
            available_since: None,
            ty: "SDL_PixelFormat",
        },
        Field {
            name: "colorspace",
            doc: Some("Frame colorspace\n"),
            available_since: None,
            ty: "SDL_Colorspace",
        },
        Field {
            name: "width",
            doc: Some("Frame width\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "height",
            doc: Some("Frame height\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "framerate_numerator",
            doc: Some(
                "Frame rate numerator ((num / denom) == FPS, (denom / num) == duration in seconds)\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "framerate_denominator",
            doc: Some(
                "Frame rate denominator ((num / denom) == FPS, (denom / num) == duration in seconds)\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
