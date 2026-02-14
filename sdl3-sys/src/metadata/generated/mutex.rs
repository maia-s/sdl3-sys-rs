//! Metadata for items in the `crate::mutex` module

use super::*;

pub const METADATA_SDL_InitStatus: Group = Group {
    module: "mutex",
    kind: GroupKind::Enum,
    name: "SDL_InitStatus",
    short_name: "InitStatus",
    doc: Some(
        "The current status of an [`SDL_InitState`] structure.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_INIT_STATUS_UNINITIALIZED",
            short_name: "UNINITIALIZED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_INITIALIZING",
            short_name: "INITIALIZING",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_INITIALIZED",
            short_name: "INITIALIZED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_INIT_STATUS_UNINITIALIZING",
            short_name: "UNINITIALIZING",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_InitState: Struct = Struct {
    module: "mutex",
    kind: StructKind::Struct,
    name: "SDL_InitState",
    doc: Some(
        "A structure used for thread-safe initialization and shutdown.\n\nHere is an example of using this:\n\n```c\nstatic SDL_InitState init;\n\nbool InitSystem(void)\n{\nif (!SDL_ShouldInit(&init)) {\n// The system is initialized\nreturn true;\n}\n\n// At this point, you should not leave this function without calling SDL_SetInitialized()\n\nbool initialized = DoInitTasks();\nSDL_SetInitialized(&init, initialized);\nreturn initialized;\n}\n\nbool UseSubsystem(void)\n{\nif (SDL_ShouldInit(&init)) {\n// Error, the subsystem isn't initialized\nSDL_SetInitialized(&init, false);\nreturn false;\n}\n\n// Do work using the initialized subsystem\n\nreturn true;\n}\n\nvoid QuitSystem(void)\n{\nif (!SDL_ShouldQuit(&init)) {\n// The system is not initialized\nreturn;\n}\n\n// At this point, you should not leave this function without calling SDL_SetInitialized()\n\nDoQuitTasks();\nSDL_SetInitialized(&init, false);\n}\n```\n\nNote that this doesn't protect any resources created during initialization,\nor guarantee that nobody is using those resources during cleanup. You\nshould use other mechanisms to protect those, if that's a concern for your\ncode.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "status",
            doc: None,
            available_since: None,
            ty: "SDL_AtomicInt",
        },
        Field {
            name: "thread",
            doc: None,
            available_since: None,
            ty: "SDL_ThreadID",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
