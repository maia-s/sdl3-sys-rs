//! Metadata for items in the `crate::asyncio` module

use super::*;

pub static METADATA_SDL_AsyncIOTaskType: Group = Group {
    module: "asyncio",
    kind: GroupKind::Enum,
    name: "SDL_AsyncIOTaskType",
    short_name: "AsyncIOTaskType",
    doc: Some("Types of asynchronous I/O tasks.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
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
pub static METADATA_SDL_AsyncIOResult: Group = Group {
    module: "asyncio",
    kind: GroupKind::Enum,
    name: "SDL_AsyncIOResult",
    short_name: "AsyncIOResult",
    doc: Some("Possible outcomes of an asynchronous I/O task.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
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
