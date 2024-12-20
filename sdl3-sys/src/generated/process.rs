//! Process control support.
//!
//! These functions provide a cross-platform way to spawn and manage OS-level
//! processes.
//!
//! You can create a new subprocess with [`SDL_CreateProcess()`] and optionally
//! read and write to it using [`SDL_ReadProcess()`] or [`SDL_GetProcessInput()`] and
//! [`SDL_GetProcessOutput()`]. If more advanced functionality like chaining input
//! between processes is necessary, you can use
//! [`SDL_CreateProcessWithProperties()`].
//!
//! You can get the status of a created process with [`SDL_WaitProcess()`], or
//! terminate the process with [`SDL_KillProcess()`].
//!
//! Don't forget to call [`SDL_DestroyProcess()`] to clean up, whether the process
//! process was killed, terminated on its own, or is still running!

use super::stdinc::*;

use super::error::*;

use super::iostream::*;

use super::properties::*;

extern "C" {
    /// Create a new process.
    ///
    /// The path to the executable is supplied in args\[0\]. args\[1..N\] are
    /// additional arguments passed on the command line of the new process, and the
    /// argument list should be terminated with a NULL, e.g.:
    ///
    /// ```c
    /// const char *args[] = { "myprogram", "argument", NULL };
    /// ```
    ///
    /// Setting pipe_stdio to true is equivalent to setting
    /// [`SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`] and
    /// [`SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`] to [`SDL_PROCESS_STDIO_APP`], and
    /// will allow the use of [`SDL_ReadProcess()`] or [`SDL_GetProcessInput()`] and
    /// [`SDL_GetProcessOutput()`].
    ///
    /// See [`SDL_CreateProcessWithProperties()`] for more details.
    ///
    /// ### Parameters
    /// - `args`: the path and arguments for the new process.
    /// - `pipe_stdio`: true to create pipes to the process's standard input and
    ///   from the process's standard output, false for the process
    ///   to have no input and inherit the application's standard
    ///   output.
    ///
    /// ### Return value
    /// Returns the newly created and running process, or NULL if the process
    ///   couldn't be created.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_GetProcessProperties`]
    /// - [`SDL_ReadProcess`]
    /// - [`SDL_GetProcessInput`]
    /// - [`SDL_GetProcessOutput`]
    /// - [`SDL_KillProcess`]
    /// - [`SDL_WaitProcess`]
    /// - [`SDL_DestroyProcess`]
    pub fn SDL_CreateProcess(
        args: *const *const ::core::ffi::c_char,
        pipe_stdio: ::core::primitive::bool,
    ) -> *mut SDL_Process;
}

/// Description of where standard I/O should be directed when creating a
/// process.
///
/// If a standard I/O stream is set to [`SDL_PROCESS_STDIO_INHERIT`], it will go to
/// the same place as the application's I/O stream. This is the default for
/// standard output and standard error.
///
/// If a standard I/O stream is set to [`SDL_PROCESS_STDIO_NULL`], it is connected
/// to `NUL:` on Windows and `/dev/null` on POSIX systems. This is the default
/// for standard input.
///
/// If a standard I/O stream is set to [`SDL_PROCESS_STDIO_APP`], it is connected
/// to a new [`SDL_IOStream`] that is available to the application. Standard input
/// will be available as [`SDL_PROP_PROCESS_STDIN_POINTER`] and allows
/// [`SDL_GetProcessInput()`], standard output will be available as
/// [`SDL_PROP_PROCESS_STDOUT_POINTER`] and allows [`SDL_ReadProcess()`] and
/// [`SDL_GetProcessOutput()`], and standard error will be available as
/// [`SDL_PROP_PROCESS_STDERR_POINTER`] in the properties for the created
/// process.
///
/// If a standard I/O stream is set to [`SDL_PROCESS_STDIO_REDIRECT`], it is
/// connected to an existing [`SDL_IOStream`] provided by the application. Standard
/// input is provided using [`SDL_PROP_PROCESS_CREATE_STDIN_POINTER`], standard
/// output is provided using [`SDL_PROP_PROCESS_CREATE_STDOUT_POINTER`], and
/// standard error is provided using [`SDL_PROP_PROCESS_CREATE_STDERR_POINTER`]
/// in the creation properties. These existing streams should be closed by the
/// application once the new process is created.
///
/// In order to use an [`SDL_IOStream`] with [`SDL_PROCESS_STDIO_REDIRECT`], it must
/// have [`SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER`] or
/// [`SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER`] set. This is true for streams
/// representing files and process I/O.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_CreateProcessWithProperties`]
/// - [`SDL_GetProcessProperties`]
/// - [`SDL_ReadProcess`]
/// - [`SDL_GetProcessInput`]
/// - [`SDL_GetProcessOutput`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INHERITED`](SDL_ProcessIO::INHERITED) | [`SDL_PROCESS_STDIO_INHERITED`] | The I/O stream is inherited from the application. |
/// | [`NULL`](SDL_ProcessIO::NULL) | [`SDL_PROCESS_STDIO_NULL`] | The I/O stream is ignored. |
/// | [`APP`](SDL_ProcessIO::APP) | [`SDL_PROCESS_STDIO_APP`] | The I/O stream is connected to a new [`SDL_IOStream`] that the application can read or write |
/// | [`REDIRECT`](SDL_ProcessIO::REDIRECT) | [`SDL_PROCESS_STDIO_REDIRECT`] | The I/O stream is redirected to an existing [`SDL_IOStream`]. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_ProcessIO(pub ::core::ffi::c_int);

impl From<SDL_ProcessIO> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_ProcessIO) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_ProcessIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INHERITED => "SDL_PROCESS_STDIO_INHERITED",
            Self::NULL => "SDL_PROCESS_STDIO_NULL",
            Self::APP => "SDL_PROCESS_STDIO_APP",
            Self::REDIRECT => "SDL_PROCESS_STDIO_REDIRECT",

            _ => return write!(f, "SDL_ProcessIO({})", self.0),
        })
    }
}

impl SDL_ProcessIO {
    /// The I/O stream is inherited from the application.
    pub const INHERITED: Self = Self(0);
    /// The I/O stream is ignored.
    pub const NULL: Self = Self(1);
    /// The I/O stream is connected to a new [`SDL_IOStream`] that the application can read or write
    pub const APP: Self = Self(2);
    /// The I/O stream is redirected to an existing [`SDL_IOStream`].
    pub const REDIRECT: Self = Self(3);
}

/// The I/O stream is inherited from the application.
pub const SDL_PROCESS_STDIO_INHERITED: SDL_ProcessIO = SDL_ProcessIO::INHERITED;
/// The I/O stream is ignored.
pub const SDL_PROCESS_STDIO_NULL: SDL_ProcessIO = SDL_ProcessIO::NULL;
/// The I/O stream is connected to a new [`SDL_IOStream`] that the application can read or write
pub const SDL_PROCESS_STDIO_APP: SDL_ProcessIO = SDL_ProcessIO::APP;
/// The I/O stream is redirected to an existing [`SDL_IOStream`].
pub const SDL_PROCESS_STDIO_REDIRECT: SDL_ProcessIO = SDL_ProcessIO::REDIRECT;

extern "C" {
    /// Create a new process with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_PROCESS_CREATE_ARGS_POINTER`]: an array of strings containing
    ///   the program to run, any arguments, and a NULL pointer, e.g. const char
    ///   *args\[\] = { "myprogram", "argument", NULL }. This is a required property.
    /// - [`SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER`]: an [`SDL_Environment`]
    ///   pointer. If this property is set, it will be the entire environment for
    ///   the process, otherwise the current environment is used.
    /// - [`SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`]: an [`SDL_ProcessIO`] value describing
    ///   where standard input for the process comes from, defaults to
    ///   [`SDL_PROCESS_STDIO_NULL`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDIN_POINTER`]: an [`SDL_IOStream`] pointer used for
    ///   standard input when [`SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`] is set to
    ///   [`SDL_PROCESS_STDIO_REDIRECT`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`]: an [`SDL_ProcessIO`] value
    ///   describing where standard output for the process goes go, defaults to
    ///   [`SDL_PROCESS_STDIO_INHERITED`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDOUT_POINTER`]: an [`SDL_IOStream`] pointer used
    ///   for standard output when [`SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`] is set
    ///   to [`SDL_PROCESS_STDIO_REDIRECT`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDERR_NUMBER`]: an [`SDL_ProcessIO`] value
    ///   describing where standard error for the process goes go, defaults to
    ///   [`SDL_PROCESS_STDIO_INHERITED`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDERR_POINTER`]: an [`SDL_IOStream`] pointer used
    ///   for standard error when [`SDL_PROP_PROCESS_CREATE_STDERR_NUMBER`] is set to
    ///   [`SDL_PROCESS_STDIO_REDIRECT`].
    /// - [`SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN`]: true if the error
    ///   output of the process should be redirected into the standard output of
    ///   the process. This property has no effect if
    ///   [`SDL_PROP_PROCESS_CREATE_STDERR_NUMBER`] is set.
    /// - [`SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN`]: true if the process should
    ///   run in the background. In this case the default input and output is
    ///   [`SDL_PROCESS_STDIO_NULL`] and the exitcode of the process is not
    ///   available, and will always be 0.
    ///
    /// On POSIX platforms, wait() and waitpid(-1, ...) should not be called, and
    /// SIGCHLD should not be ignored or handled because those would prevent SDL
    /// from properly tracking the lifetime of the underlying process. You should
    /// use [`SDL_WaitProcess()`] instead.
    ///
    /// ### Parameters
    /// - `props`: the properties to use.
    ///
    /// ### Return value
    /// Returns the newly created and running process, or NULL if the process
    ///   couldn't be created.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_GetProcessProperties`]
    /// - [`SDL_ReadProcess`]
    /// - [`SDL_GetProcessInput`]
    /// - [`SDL_GetProcessOutput`]
    /// - [`SDL_KillProcess`]
    /// - [`SDL_WaitProcess`]
    /// - [`SDL_DestroyProcess`]
    pub fn SDL_CreateProcessWithProperties(props: SDL_PropertiesID) -> *mut SDL_Process;
}

pub const SDL_PROP_PROCESS_CREATE_ARGS_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.create.args".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.create.environment".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDIN_NUMBER: *const ::core::ffi::c_char =
    c"SDL.process.create.stdin_option".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDIN_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.create.stdin_source".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER: *const ::core::ffi::c_char =
    c"SDL.process.create.stdout_option".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDOUT_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.create.stdout_source".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDERR_NUMBER: *const ::core::ffi::c_char =
    c"SDL.process.create.stderr_option".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDERR_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.create.stderr_source".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.process.create.stderr_to_stdout".as_ptr();

pub const SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.process.create.background".as_ptr();

extern "C" {
    /// Get the properties associated with a process.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - [`SDL_PROP_PROCESS_PID_NUMBER`]: the process ID of the process.
    /// - [`SDL_PROP_PROCESS_STDIN_POINTER`]: an [`SDL_IOStream`] that can be used to
    ///   write input to the process, if it was created with
    ///   [`SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`] set to [`SDL_PROCESS_STDIO_APP`].
    /// - [`SDL_PROP_PROCESS_STDOUT_POINTER`]: a non-blocking [`SDL_IOStream`] that can
    ///   be used to read output from the process, if it was created with
    ///   [`SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`] set to [`SDL_PROCESS_STDIO_APP`].
    /// - [`SDL_PROP_PROCESS_STDERR_POINTER`]: a non-blocking [`SDL_IOStream`] that can
    ///   be used to read error output from the process, if it was created with
    ///   [`SDL_PROP_PROCESS_CREATE_STDERR_NUMBER`] set to [`SDL_PROCESS_STDIO_APP`].
    /// - [`SDL_PROP_PROCESS_BACKGROUND_BOOLEAN`]: true if the process is running in
    ///   the background.
    ///
    /// ### Parameters
    /// - `process`: the process to query.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    pub fn SDL_GetProcessProperties(process: *mut SDL_Process) -> SDL_PropertiesID;
}

pub const SDL_PROP_PROCESS_PID_NUMBER: *const ::core::ffi::c_char = c"SDL.process.pid".as_ptr();

pub const SDL_PROP_PROCESS_STDIN_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.stdin".as_ptr();

pub const SDL_PROP_PROCESS_STDOUT_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.stdout".as_ptr();

pub const SDL_PROP_PROCESS_STDERR_POINTER: *const ::core::ffi::c_char =
    c"SDL.process.stderr".as_ptr();

pub const SDL_PROP_PROCESS_BACKGROUND_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.process.background".as_ptr();

extern "C" {
    /// Read all the output from a process.
    ///
    /// If a process was created with I/O enabled, you can use this function to
    /// read the output. This function blocks until the process is complete,
    /// capturing all output, and providing the process exit code.
    ///
    /// The data is allocated with a zero byte at the end (null terminated) for
    /// convenience. This extra byte is not included in the value reported via
    /// `datasize`.
    ///
    /// The data should be freed with [`SDL_free()`].
    ///
    /// ### Parameters
    /// - `process`: The process to read.
    /// - `datasize`: a pointer filled in with the number of bytes read, may be
    ///   NULL.
    /// - `exitcode`: a pointer filled in with the process exit code if the
    ///   process has exited, may be NULL.
    ///
    /// ### Return value
    /// Returns the data or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function is not thread safe.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_DestroyProcess`]
    pub fn SDL_ReadProcess(
        process: *mut SDL_Process,
        datasize: *mut ::core::primitive::usize,
        exitcode: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Get the [`SDL_IOStream`] associated with process standard input.
    ///
    /// The process must have been created with [`SDL_CreateProcess()`] and pipe_stdio
    /// set to true, or with [`SDL_CreateProcessWithProperties()`] and
    /// [`SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`] set to [`SDL_PROCESS_STDIO_APP`].
    ///
    /// Writing to this stream can return less data than expected if the process
    /// hasn't read its input. It may be blocked waiting for its output to be read,
    /// so if you may need to call [`SDL_GetOutputStream()`] and read the output in
    /// parallel with writing input.
    ///
    /// ### Parameters
    /// - `process`: The process to get the input stream for.
    ///
    /// ### Return value
    /// Returns the input stream or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_GetProcessOutput`]
    pub fn SDL_GetProcessInput(process: *mut SDL_Process) -> *mut SDL_IOStream;
}

extern "C" {
    /// Get the [`SDL_IOStream`] associated with process standard output.
    ///
    /// The process must have been created with [`SDL_CreateProcess()`] and pipe_stdio
    /// set to true, or with [`SDL_CreateProcessWithProperties()`] and
    /// [`SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`] set to [`SDL_PROCESS_STDIO_APP`].
    ///
    /// Reading from this stream can return 0 with [`SDL_GetIOStatus()`] returning
    /// [`SDL_IO_STATUS_NOT_READY`] if no output is available yet.
    ///
    /// ### Parameters
    /// - `process`: The process to get the output stream for.
    ///
    /// ### Return value
    /// Returns the output stream or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_GetProcessInput`]
    pub fn SDL_GetProcessOutput(process: *mut SDL_Process) -> *mut SDL_IOStream;
}

extern "C" {
    /// Stop a process.
    ///
    /// ### Parameters
    /// - `process`: The process to stop.
    /// - `force`: true to terminate the process immediately, false to try to
    ///   stop the process gracefully. In general you should try to stop
    ///   the process gracefully first as terminating a process may
    ///   leave it with half-written data or in some other unstable
    ///   state.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// This function is not thread safe.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_WaitProcess`]
    /// - [`SDL_DestroyProcess`]
    pub fn SDL_KillProcess(
        process: *mut SDL_Process,
        force: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait for a process to finish.
    ///
    /// This can be called multiple times to get the status of a process.
    ///
    /// The exit code will be the exit code of the process if it terminates
    /// normally, a negative signal if it terminated due to a signal, or -255
    /// otherwise. It will not be changed if the process is still running.
    ///
    /// ### Parameters
    /// - `process`: The process to wait for.
    /// - `block`: If true, block until the process finishes; otherwise, report
    ///   on the process' status.
    /// - `exitcode`: a pointer filled in with the process exit code if the
    ///   process has exited, may be NULL.
    ///
    /// ### Return value
    /// Returns true if the process exited, false otherwise.
    ///
    /// ### Thread safety
    /// This function is not thread safe.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_KillProcess`]
    /// - [`SDL_DestroyProcess`]
    pub fn SDL_WaitProcess(
        process: *mut SDL_Process,
        block: ::core::primitive::bool,
        exitcode: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a previously created process object.
    ///
    /// Note that this does not stop the process, just destroys the SDL object used
    /// to track it. If you want to stop the process you should use
    /// [`SDL_KillProcess()`].
    ///
    /// ### Parameters
    /// - `process`: The process object to destroy.
    ///
    /// ### Thread safety
    /// This function is not thread safe.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProcess`]
    /// - [`SDL_CreateProcessWithProperties`]
    /// - [`SDL_KillProcess`]
    pub fn SDL_DestroyProcess(process: *mut SDL_Process);
}

#[repr(C)]
pub struct SDL_Process {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
