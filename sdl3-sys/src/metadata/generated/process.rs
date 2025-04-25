//! Metadata for items in the `crate::process` module

use super::*;

pub static METADATA_SDL_PROP_PROCESS_CREATE_ARGS_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_ARGS_POINTER",
    short_name: "PROCESS_CREATE_ARGS_POINTER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_ARGS_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER",
    short_name: "PROCESS_CREATE_ENVIRONMENT_POINTER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDIN_NUMBER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDIN_NUMBER",
    short_name: "PROCESS_CREATE_STDIN_NUMBER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDIN_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDIN_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDIN_POINTER",
    short_name: "PROCESS_CREATE_STDIN_POINTER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDIN_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER",
    short_name: "PROCESS_CREATE_STDOUT_NUMBER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDOUT_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDOUT_POINTER",
    short_name: "PROCESS_CREATE_STDOUT_POINTER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDOUT_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDERR_NUMBER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDERR_NUMBER",
    short_name: "PROCESS_CREATE_STDERR_NUMBER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDERR_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDERR_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDERR_POINTER",
    short_name: "PROCESS_CREATE_STDERR_POINTER",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDERR_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN",
    short_name: "PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN",
    value: crate::process::SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN",
    short_name: "PROCESS_CREATE_BACKGROUND_BOOLEAN",
    value: crate::process::SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_PID_NUMBER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_PID_NUMBER",
    short_name: "PROCESS_PID_NUMBER",
    value: crate::process::SDL_PROP_PROCESS_PID_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_STDIN_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_STDIN_POINTER",
    short_name: "PROCESS_STDIN_POINTER",
    value: crate::process::SDL_PROP_PROCESS_STDIN_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_STDOUT_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_STDOUT_POINTER",
    short_name: "PROCESS_STDOUT_POINTER",
    value: crate::process::SDL_PROP_PROCESS_STDOUT_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_STDERR_POINTER: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_STDERR_POINTER",
    short_name: "PROCESS_STDERR_POINTER",
    value: crate::process::SDL_PROP_PROCESS_STDERR_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_PROCESS_BACKGROUND_BOOLEAN: Property = Property {
    module: "process",
    name: "SDL_PROP_PROCESS_BACKGROUND_BOOLEAN",
    short_name: "PROCESS_BACKGROUND_BOOLEAN",
    value: crate::process::SDL_PROP_PROCESS_BACKGROUND_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_ProcessIO: Group = Group {
    module: "process",
    kind: GroupKind::Enum,
    name: "SDL_ProcessIO",
    short_name: "ProcessIO",
    doc: Some("Description of where standard I/O should be directed when creating a\nprocess.\n\nIf a standard I/O stream is set to [`SDL_PROCESS_STDIO_INHERITED`], it will go\nto the same place as the application's I/O stream. This is the default for\nstandard output and standard error.\n\nIf a standard I/O stream is set to [`SDL_PROCESS_STDIO_NULL`], it is connected\nto `NUL:` on Windows and `/dev/null` on POSIX systems. This is the default\nfor standard input.\n\nIf a standard I/O stream is set to [`SDL_PROCESS_STDIO_APP`], it is connected\nto a new [`SDL_IOStream`] that is available to the application. Standard input\nwill be available as [`SDL_PROP_PROCESS_STDIN_POINTER`] and allows\n[`SDL_GetProcessInput()`], standard output will be available as\n[`SDL_PROP_PROCESS_STDOUT_POINTER`] and allows [`SDL_ReadProcess()`] and\n[`SDL_GetProcessOutput()`], and standard error will be available as\n[`SDL_PROP_PROCESS_STDERR_POINTER`] in the properties for the created\nprocess.\n\nIf a standard I/O stream is set to [`SDL_PROCESS_STDIO_REDIRECT`], it is\nconnected to an existing [`SDL_IOStream`] provided by the application. Standard\ninput is provided using [`SDL_PROP_PROCESS_CREATE_STDIN_POINTER`], standard\noutput is provided using [`SDL_PROP_PROCESS_CREATE_STDOUT_POINTER`], and\nstandard error is provided using [`SDL_PROP_PROCESS_CREATE_STDERR_POINTER`]\nin the creation properties. These existing streams should be closed by the\napplication once the new process is created.\n\nIn order to use an [`SDL_IOStream`] with [`SDL_PROCESS_STDIO_REDIRECT`], it must\nhave [`SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER`] or\n[`SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER`] set. This is true for streams\nrepresenting files and process I/O.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_CreateProcessWithProperties`]\n- [`SDL_GetProcessProperties`]\n- [`SDL_ReadProcess`]\n- [`SDL_GetProcessInput`]\n- [`SDL_GetProcessOutput`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_PROCESS_STDIO_INHERITED",
            short_name: "INHERITED",
            doc: Some("The I/O stream is inherited from the application.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROCESS_STDIO_NULL",
            short_name: "NULL",
            doc: Some("The I/O stream is ignored.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROCESS_STDIO_APP",
            short_name: "APP",
            doc: Some("The I/O stream is connected to a new [`SDL_IOStream`] that the application can read or write\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PROCESS_STDIO_REDIRECT",
            short_name: "REDIRECT",
            doc: Some("The I/O stream is redirected to an existing [`SDL_IOStream`].\n"),
            available_since: None,
        },
    ],
};
