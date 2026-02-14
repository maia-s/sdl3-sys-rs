//! Metadata for items in the `crate::iostream` module

use super::*;

pub const METADATA_SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER",
    short_name: "IOSTREAM_WINDOWS_HANDLE_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_STDIO_FILE_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_STDIO_FILE_POINTER",
    short_name: "IOSTREAM_STDIO_FILE_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_STDIO_FILE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER",
    short_name: "IOSTREAM_FILE_DESCRIPTOR_NUMBER",
    value: crate::iostream::SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER",
    short_name: "IOSTREAM_ANDROID_AASSET_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_MEMORY_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_MEMORY_POINTER",
    short_name: "IOSTREAM_MEMORY_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_MEMORY_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER",
    short_name: "IOSTREAM_MEMORY_SIZE_NUMBER",
    value: crate::iostream::SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_MEMORY_FREE_FUNC_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_MEMORY_FREE_FUNC_POINTER",
    short_name: "IOSTREAM_MEMORY_FREE_FUNC_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_MEMORY_FREE_FUNC_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER",
    short_name: "IOSTREAM_DYNAMIC_MEMORY_POINTER",
    value: crate::iostream::SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER",
    short_name: "IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER",
    value: crate::iostream::SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_IOStatus: Group = Group {
    module: "iostream",
    kind: GroupKind::Enum,
    name: "SDL_IOStatus",
    short_name: "IOStatus",
    doc: Some(
        "[`SDL_IOStream`] status, set by a read or write operation.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_IO_STATUS_READY",
            short_name: "READY",
            doc: Some("Everything is ready (no errors and not EOF).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_STATUS_ERROR",
            short_name: "ERROR",
            doc: Some("Read or write I/O error\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_STATUS_EOF",
            short_name: "EOF",
            doc: Some("End of file\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_STATUS_NOT_READY",
            short_name: "NOT_READY",
            doc: Some("Non blocking I/O, not ready\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_STATUS_READONLY",
            short_name: "READONLY",
            doc: Some("Tried to write a read-only buffer\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_STATUS_WRITEONLY",
            short_name: "WRITEONLY",
            doc: Some("Tried to read a write-only buffer\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_IOWhence: Group = Group {
    module: "iostream",
    kind: GroupKind::Enum,
    name: "SDL_IOWhence",
    short_name: "IOWhence",
    doc: Some(
        "Possible `whence` values for [`SDL_IOStream`] seeking.\n\nThese map to the same \"whence\" concept that `fseek` or `lseek` use in the\nstandard C runtime.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_IO_SEEK_SET",
            short_name: "SET",
            doc: Some("Seek from the beginning of data\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_SEEK_CUR",
            short_name: "CUR",
            doc: Some("Seek relative to current read point\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_IO_SEEK_END",
            short_name: "END",
            doc: Some("Seek relative to the end of data\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_IOStreamInterface: Struct = Struct {
    module: "iostream",
    kind: StructKind::Struct,
    name: "SDL_IOStreamInterface",
    doc: Some(
        "The function pointers that drive an [`SDL_IOStream`].\n\nApplications can provide this struct to [`SDL_OpenIO()`] to create their own\nimplementation of [`SDL_IOStream`]. This is not necessarily required, as SDL\nalready offers several common types of I/O streams, via functions like\n[`SDL_IOFromFile()`] and [`SDL_IOFromMem()`].\n\nThis structure should be initialized using [`SDL_INIT_INTERFACE()`]\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_INIT_INTERFACE`]\n\n## Notes for `sdl3-sys`\nThis interface struct can be initialized with [`SDL_IOStreamInterface::new()`] or `Default::default()`.\n",
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
            name: "size",
            doc: Some(
                "Return the number of bytes in this [`SDL_IOStream`]\n\n\\return the total size of the data stream, or -1 on error.\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void) -> Sint64>",
        },
        Field {
            name: "seek",
            doc: Some(
                "Seek to `offset` relative to `whence`, one of stdio's whence values:\n[`SDL_IO_SEEK_SET`], [`SDL_IO_SEEK_CUR`], [`SDL_IO_SEEK_END`]\n\n\\return the final offset in the data stream, or -1 on error.\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, offset: Sint64, whence: SDL_IOWhence) -> Sint64>",
        },
        Field {
            name: "read",
            doc: Some(
                "Read up to `size` bytes from the data stream to the area pointed\nat by `ptr`. `size` will always be > 0.\n\nOn an incomplete read, you should set `*status` to a value from the\n[`SDL_IOStatus`] enum. You do not have to explicitly set this on\na complete, successful read.\n\n\\return the number of bytes read\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, ptr: *mut ::core::ffi::c_void, size: ::core::primitive::usize, status: *mut SDL_IOStatus) -> ::core::primitive::usize>",
        },
        Field {
            name: "write",
            doc: Some(
                "Write exactly `size` bytes from the area pointed at by `ptr`\nto data stream. `size` will always be > 0.\n\nOn an incomplete write, you should set `*status` to a value from the\n[`SDL_IOStatus`] enum. You do not have to explicitly set this on\na complete, successful write.\n\n\\return the number of bytes written\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, ptr: *const ::core::ffi::c_void, size: ::core::primitive::usize, status: *mut SDL_IOStatus) -> ::core::primitive::usize>",
        },
        Field {
            name: "flush",
            doc: Some(
                "If the stream is buffering, make sure the data is written out.\n\nOn failure, you should set `*status` to a value from the\n[`SDL_IOStatus`] enum. You do not have to explicitly set this on\na successful flush.\n\n\\return true if successful or false on write error when flushing data.\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void, status: *mut SDL_IOStatus) -> ::core::primitive::bool>",
        },
        Field {
            name: "close",
            doc: Some(
                "Close and free any allocated resources.\n\nThis does not guarantee file writes will sync to physical media; they\ncan be in the system's file cache, waiting to go to disk.\n\nThe [`SDL_IOStream`] is still destroyed even if this fails, so clean up anything\neven if flushing buffers, etc, returns an error.\n\n\\return true if successful or false on write error when flushing data.\n",
            ),
            available_since: None,
            ty: "::core::option::Option<unsafe extern \"C\" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool>",
        },
    ],
};
