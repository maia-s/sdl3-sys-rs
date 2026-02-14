//! Metadata for items in the `crate::rect` module

use super::*;

pub const METADATA_SDL_Point: Struct = Struct {
    module: "rect",
    kind: StructKind::Struct,
    name: "SDL_Point",
    doc: Some(
        "The structure that defines a point (using integers).\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetRectEnclosingPoints`]\n- [`SDL_PointInRect`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "x",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "y",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
pub const METADATA_SDL_FPoint: Struct = Struct {
    module: "rect",
    kind: StructKind::Struct,
    name: "SDL_FPoint",
    doc: Some(
        "The structure that defines a point (using floating point values).\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetRectEnclosingPointsFloat`]\n- [`SDL_PointInRectFloat`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "x",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_Rect: Struct = Struct {
    module: "rect",
    kind: StructKind::Struct,
    name: "SDL_Rect",
    doc: Some(
        "A rectangle, with the origin at the upper left (using integers).\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_RectEmpty`]\n- [`SDL_RectsEqual`]\n- [`SDL_HasRectIntersection`]\n- [`SDL_GetRectIntersection`]\n- [`SDL_GetRectAndLineIntersection`]\n- [`SDL_GetRectUnion`]\n- [`SDL_GetRectEnclosingPoints`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "x",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "y",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "w",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "h",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
pub const METADATA_SDL_FRect: Struct = Struct {
    module: "rect",
    kind: StructKind::Struct,
    name: "SDL_FRect",
    doc: Some(
        "A rectangle stored using floating point values.\n\nThe origin of the coordinate space is in the top-left, with increasing\nvalues moving down and right. The properties `x` and `y` represent the\ncoordinates of the top-left corner of the rectangle.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_RectEmptyFloat`]\n- [`SDL_RectsEqualFloat`]\n- [`SDL_RectsEqualEpsilon`]\n- [`SDL_HasRectIntersectionFloat`]\n- [`SDL_GetRectIntersectionFloat`]\n- [`SDL_GetRectAndLineIntersectionFloat`]\n- [`SDL_GetRectUnionFloat`]\n- [`SDL_GetRectEnclosingPointsFloat`]\n- [`SDL_PointInRectFloat`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "x",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "w",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "h",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
