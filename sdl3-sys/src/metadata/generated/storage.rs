//! Metadata for items in the `crate::storage` module

use super::*;

pub const METADATA_SDL_StorageInterface: Struct = Struct {
    module: "storage",
    kind: StructKind::Struct,
    name: "SDL_StorageInterface",
    doc: Some(
        "Function interface for [`SDL_Storage`].\n\nApps that want to supply a custom implementation of [`SDL_Storage`] will fill\nin all the functions in this struct, and then pass it to [`SDL_OpenStorage`] to\ncreate a custom [`SDL_Storage`] object.\n\nIt is not usually necessary to do this; SDL provides standard\nimplementations for many things you might expect to do with an [`SDL_Storage`].\n\nThis structure should be initialized using [`SDL_INIT_INTERFACE()`]\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_INIT_INTERFACE`]\n\n## Notes for `sdl3-sys`\nThis interface struct can be initialized with [`SDL_StorageInterface::new()`] or `Default::default()`.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "version",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "close",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool>",
        },
        Field {
            name: "ready",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool>",
        },
        Field {
            name: "enumerate",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, callback: SDL_EnumerateDirectoryCallback, callback_userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool>",
        },
        Field {
            name: "info",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, info: *mut SDL_PathInfo) -> ::core::primitive::bool>",
        },
        Field {
            name: "read_file",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, destination: *mut ::core::ffi::c_void, length: Uint64) -> ::core::primitive::bool>",
        },
        Field {
            name: "write_file",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, source: *const ::core::ffi::c_void, length: Uint64) -> ::core::primitive::bool>",
        },
        Field {
            name: "mkdir",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char) -> ::core::primitive::bool>",
        },
        Field {
            name: "remove",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char) -> ::core::primitive::bool>",
        },
        Field {
            name: "rename",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> ::core::primitive::bool>",
        },
        Field {
            name: "copy",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> ::core::primitive::bool>",
        },
        Field {
            name: "space_remaining",
            doc: None,
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void) -> Uint64>",
        },
    ],
};
