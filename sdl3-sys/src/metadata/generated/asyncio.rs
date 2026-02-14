//! Metadata for items in the `crate::asyncio` module

use super::*;

pub const METADATA_SDL_AsyncIOTaskType: Group = Group {
    module: "asyncio",
    kind: GroupKind::Enum,
    name: "SDL_AsyncIOTaskType",
    short_name: "AsyncIOTaskType",
    doc: Some(
        "Types of asynchronous I/O tasks.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ASYNCIO_TASK_READ",
            short_name: "READ",
            doc: Some("A read operation.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASYNCIO_TASK_WRITE",
            short_name: "WRITE",
            doc: Some("A write operation.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASYNCIO_TASK_CLOSE",
            short_name: "CLOSE",
            doc: Some("A close operation.\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_AsyncIOResult: Group = Group {
    module: "asyncio",
    kind: GroupKind::Enum,
    name: "SDL_AsyncIOResult",
    short_name: "AsyncIOResult",
    doc: Some(
        "Possible outcomes of an asynchronous I/O task.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ASYNCIO_COMPLETE",
            short_name: "COMPLETE",
            doc: Some("request was completed without error\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASYNCIO_FAILURE",
            short_name: "FAILURE",
            doc: Some("request failed for some reason; check [`SDL_GetError()`]!\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ASYNCIO_CANCELED",
            short_name: "CANCELED",
            doc: Some("request was canceled before completing.\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_AsyncIOOutcome: Struct = Struct {
    module: "asyncio",
    kind: StructKind::Struct,
    name: "SDL_AsyncIOOutcome",
    doc: Some(
        "Information about a completed asynchronous I/O request.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "asyncio",
            doc: Some("what generated this task. This pointer will be invalid if it was closed!\n"),
            available_since: None,
            ty: "*mut SDL_AsyncIO",
        },
        Field {
            name: "r#type",
            doc: Some("What sort of task was this? Read, write, etc?\n"),
            available_since: None,
            ty: "SDL_AsyncIOTaskType",
        },
        Field {
            name: "result",
            doc: Some("the result of the work (success, failure, cancellation).\n"),
            available_since: None,
            ty: "SDL_AsyncIOResult",
        },
        Field {
            name: "buffer",
            doc: Some("buffer where data was read/written.\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "offset",
            doc: Some("offset in the [`SDL_AsyncIO`] where data was read/written.\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "bytes_requested",
            doc: Some("number of bytes the task was to read/write.\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "bytes_transferred",
            doc: Some("actual number of bytes that were read/written.\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "userdata",
            doc: Some("pointer provided by the app when starting the task\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
