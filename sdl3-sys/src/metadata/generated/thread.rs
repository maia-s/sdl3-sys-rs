//! Metadata for items in the `crate::thread` module

use super::*;

pub static METADATA_SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER: Property = Property {
    module: "thread",
    name: "SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER",
    short_name: "THREAD_CREATE_ENTRY_FUNCTION_POINTER",
    value: crate::thread::SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_THREAD_CREATE_NAME_STRING: Property = Property {
    module: "thread",
    name: "SDL_PROP_THREAD_CREATE_NAME_STRING",
    short_name: "THREAD_CREATE_NAME_STRING",
    value: crate::thread::SDL_PROP_THREAD_CREATE_NAME_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_THREAD_CREATE_USERDATA_POINTER: Property = Property {
    module: "thread",
    name: "SDL_PROP_THREAD_CREATE_USERDATA_POINTER",
    short_name: "THREAD_CREATE_USERDATA_POINTER",
    value: crate::thread::SDL_PROP_THREAD_CREATE_USERDATA_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER: Property = Property {
    module: "thread",
    name: "SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER",
    short_name: "THREAD_CREATE_STACKSIZE_NUMBER",
    value: crate::thread::SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_ThreadID: Group = Group {
    module: "thread",
    kind: GroupKind::Id,
    name: "SDL_ThreadID",
    short_name: "ThreadID",
    doc: Some("A unique numeric ID that identifies a thread.\n\nThese are different from [`SDL_Thread`] objects, which are generally what an\napplication will operate on, but having a way to uniquely identify a thread\ncan be useful at times.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetThreadID`]\n- [`SDL_GetCurrentThreadID`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub static METADATA_SDL_TLSID: Group = Group {
    module: "thread",
    kind: GroupKind::Id,
    name: "SDL_TLSID",
    short_name: "TLSID",
    doc: Some("Thread local storage ID.\n\n0 is the invalid ID. An app can create these and then set data for these\nIDs that is unique to each thread.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetTLS`]\n- [`SDL_SetTLS`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub static METADATA_SDL_ThreadPriority: Group = Group {
    module: "thread",
    kind: GroupKind::Enum,
    name: "SDL_ThreadPriority",
    short_name: "ThreadPriority",
    doc: Some("The SDL thread priority.\n\nSDL will make system changes as necessary in order to apply the thread\npriority. Code which attempts to control thread state related to priority\nshould be aware that calling [`SDL_SetCurrentThreadPriority`] may alter such\nstate. [`SDL_HINT_THREAD_PRIORITY_POLICY`] can be used to control aspects of\nthis behavior.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_THREAD_PRIORITY_LOW",
            short_name: "LOW",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_PRIORITY_NORMAL",
            short_name: "NORMAL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_PRIORITY_HIGH",
            short_name: "HIGH",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_PRIORITY_TIME_CRITICAL",
            short_name: "TIME_CRITICAL",
            doc: None,
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_ThreadState: Group = Group {
    module: "thread",
    kind: GroupKind::Enum,
    name: "SDL_ThreadState",
    short_name: "ThreadState",
    doc: Some("The SDL thread state.\n\nThe current state of a thread can be checked by calling [`SDL_GetThreadState`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetThreadState`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_THREAD_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("The thread is not valid\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_ALIVE",
            short_name: "ALIVE",
            doc: Some("The thread is currently running\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_DETACHED",
            short_name: "DETACHED",
            doc: Some("The thread is detached and can't be waited on\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_THREAD_COMPLETE",
            short_name: "COMPLETE",
            doc: Some("The thread has finished and should be cleaned up with [`SDL_WaitThread()`]\n"),
            available_since: None,
        },
    ],
};
