//! Metadata for items in the `crate::surface` module

use super::*;

pub const METADATA_SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT",
    short_name: "SURFACE_SDR_WHITE_POINT_FLOAT",
    value: crate::surface::SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT",
    short_name: "SURFACE_HDR_HEADROOM_FLOAT",
    value: crate::surface::SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING",
    short_name: "SURFACE_TONEMAP_OPERATOR_STRING",
    value: crate::surface::SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_SURFACE_HOTSPOT_X_NUMBER: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_HOTSPOT_X_NUMBER",
    short_name: "SURFACE_HOTSPOT_X_NUMBER",
    value: crate::surface::SDL_PROP_SURFACE_HOTSPOT_X_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_SURFACE_HOTSPOT_Y_NUMBER: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_HOTSPOT_Y_NUMBER",
    short_name: "SURFACE_HOTSPOT_Y_NUMBER",
    value: crate::surface::SDL_PROP_SURFACE_HOTSPOT_Y_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_SURFACE_ROTATION_NUMBER: Property = Property {
    module: "surface",
    name: "SDL_PROP_SURFACE_ROTATION_NUMBER",
    short_name: "SURFACE_ROTATION_NUMBER",
    value: crate::surface::SDL_PROP_SURFACE_ROTATION_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_SurfaceFlags: Group = Group {
    module: "surface",
    kind: GroupKind::Flags,
    name: "SDL_SurfaceFlags",
    short_name: "SurfaceFlags",
    doc: Some(
        "The flags on an [`SDL_Surface`].\n\nThese are generally considered read-only.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SURFACE_PREALLOCATED",
            short_name: "PREALLOCATED",
            doc: Some("Surface uses preallocated pixel memory\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SURFACE_LOCK_NEEDED",
            short_name: "LOCK_NEEDED",
            doc: Some("Surface needs to be locked to access pixels\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SURFACE_LOCKED",
            short_name: "LOCKED",
            doc: Some("Surface is currently locked\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SURFACE_SIMD_ALIGNED",
            short_name: "SIMD_ALIGNED",
            doc: Some("Surface uses pixel memory allocated with [`SDL_aligned_alloc()`]\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_ScaleMode: Group = Group {
    module: "surface",
    kind: GroupKind::Enum,
    name: "SDL_ScaleMode",
    short_name: "ScaleMode",
    doc: Some("The scaling mode.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SCALEMODE_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_SCALEMODE_NEAREST",
            short_name: "NEAREST",
            doc: Some("nearest pixel sampling\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SCALEMODE_LINEAR",
            short_name: "LINEAR",
            doc: Some("linear filtering\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SCALEMODE_PIXELART",
            short_name: "PIXELART",
            doc: Some(
                "nearest pixel sampling with improved scaling for pixel art, available since SDL 3.4.0\n",
            ),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_FlipMode: Group = Group {
    module: "surface",
    kind: GroupKind::Flags,
    name: "SDL_FlipMode",
    short_name: "FlipMode",
    doc: Some("The flip mode.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_FLIP_NONE",
            short_name: "NONE",
            doc: Some("Do not flip\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FLIP_HORIZONTAL",
            short_name: "HORIZONTAL",
            doc: Some("flip horizontally\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FLIP_VERTICAL",
            short_name: "VERTICAL",
            doc: Some("flip vertically\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FLIP_HORIZONTAL_AND_VERTICAL",
            short_name: "HORIZONTAL_AND_VERTICAL",
            doc: Some("flip horizontally and vertically (not a diagonal flip)\n"),
            available_since: None,
        },
    ],
};
