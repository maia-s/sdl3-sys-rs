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
