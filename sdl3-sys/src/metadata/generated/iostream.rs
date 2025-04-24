//! Metadata for items in the `crate::iostream` module

use super::*;

pub static METADATA_SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER",
    short_name: "IOSTREAM_WINDOWS_HANDLE",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_STDIO_FILE_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_STDIO_FILE_POINTER",
    short_name: "IOSTREAM_STDIO_FILE",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_STDIO_FILE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER",
    short_name: "IOSTREAM_FILE_DESCRIPTOR",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER",
    short_name: "IOSTREAM_ANDROID_AASSET",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_MEMORY_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_MEMORY_POINTER",
    short_name: "IOSTREAM_MEMORY",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_MEMORY_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER",
    short_name: "IOSTREAM_MEMORY_SIZE",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER",
    short_name: "IOSTREAM_DYNAMIC_MEMORY",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER: Property = Property {
    module: "iostream",
    name: "SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER",
    short_name: "IOSTREAM_DYNAMIC_CHUNKSIZE",
    value: unsafe { CStr::from_ptr(crate::iostream::SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_IOStatus: Group = Group {
    module: "iostream",
    kind: GroupKind::Enum,
    name: "SDL_IOStatus",
    short_name: "IOStatus",
    doc: Some("[`SDL_IOStream`] status, set by a read or write operation.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
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
pub static METADATA_SDL_IOWhence: Group = Group {
    module: "iostream",
    kind: GroupKind::Enum,
    name: "SDL_IOWhence",
    short_name: "IOWhence",
    doc: Some("Possible `whence` values for [`SDL_IOStream`] seeking.\n\nThese map to the same \"whence\" concept that `fseek` or `lseek` use in the\nstandard C runtime.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
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
