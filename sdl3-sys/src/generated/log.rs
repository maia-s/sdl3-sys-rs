//! Simple log messages with priorities and categories. A message's
//! [`SDL_LogPriority`] signifies how important the message is. A message's
//! [`SDL_LogCategory`] signifies from what domain it belongs to. Every category
//! has a minimum priority specified: when a message belongs to that category,
//! it will only be sent out if it has that minimum priority or higher.
//!
//! SDL's own logs are sent below the default priority threshold, so they are
//! quiet by default.
//!
//! You can change the log verbosity programmatically using
//! [`SDL_SetLogPriority()`] or with SDL_SetHint([`SDL_HINT_LOGGING`], ...), or with
//! the "SDL_LOGGING" environment variable. This variable is a comma separated
//! set of category=level tokens that define the default logging levels for SDL
//! applications.
//!
//! The category can be a numeric category, one of "app", "error", "assert",
//! "system", "audio", "video", "render", "input", "test", or `*` for any
//! unspecified category.
//!
//! The level can be a numeric level, one of "verbose", "debug", "info",
//! "warn", "error", "critical", or "quiet" to disable that category.
//!
//! You can omit the category if you want to set the logging level for all
//! categories.
//!
//! If this hint isn't set, the default log levels are equivalent to:
//!
//! `app=info,assert=warn,test=verbose,*=error`
//!
//! Here's where the messages go on different platforms:
//!
//! - Windows: debug output stream
//! - Android: log output
//! - Others: standard error output (stderr)

use super::stdinc::*;

/// The predefined log categories
///
/// By default the application and gpu categories are enabled at the INFO
/// level, the assert category is enabled at the WARN level, test is enabled at
/// the VERBOSE level and all other categories are enabled at the ERROR level.
///
/// ### Availability
/// This enum is available since SDL 3.0.0.
///
/// ### `sdl3-sys` note
/// This is a `C` enum. Known values:
/// - [`SDL_LOG_CATEGORY_ERROR`]
/// - [`SDL_LOG_CATEGORY_ASSERT`]
/// - [`SDL_LOG_CATEGORY_SYSTEM`]
/// - [`SDL_LOG_CATEGORY_AUDIO`]
/// - [`SDL_LOG_CATEGORY_VIDEO`]
/// - [`SDL_LOG_CATEGORY_RENDER`]
/// - [`SDL_LOG_CATEGORY_INPUT`]
/// - [`SDL_LOG_CATEGORY_TEST`]
/// - [`SDL_LOG_CATEGORY_GPU`]
/// - [`SDL_LOG_CATEGORY_RESERVED2`]
/// - [`SDL_LOG_CATEGORY_RESERVED3`]
/// - [`SDL_LOG_CATEGORY_RESERVED4`]
/// - [`SDL_LOG_CATEGORY_RESERVED5`]
/// - [`SDL_LOG_CATEGORY_RESERVED6`]
/// - [`SDL_LOG_CATEGORY_RESERVED7`]
/// - [`SDL_LOG_CATEGORY_RESERVED8`]
/// - [`SDL_LOG_CATEGORY_RESERVED9`]
/// - [`SDL_LOG_CATEGORY_RESERVED10`]
/// - [`SDL_LOG_CATEGORY_CUSTOM`]

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_LogCategory(pub ::core::ffi::c_int);
impl From<SDL_LogCategory> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_LogCategory) -> Self {
        value.0
    }
}
impl SDL_LogCategory {
    pub const APPLICATION: Self = Self(0);
    pub const ERROR: Self = Self(1);
    pub const ASSERT: Self = Self(2);
    pub const SYSTEM: Self = Self(3);
    pub const AUDIO: Self = Self(4);
    pub const VIDEO: Self = Self(5);
    pub const RENDER: Self = Self(6);
    pub const INPUT: Self = Self(7);
    pub const TEST: Self = Self(8);
    pub const GPU: Self = Self(9);
    pub const RESERVED2: Self = Self(10);
    pub const RESERVED3: Self = Self(11);
    pub const RESERVED4: Self = Self(12);
    pub const RESERVED5: Self = Self(13);
    pub const RESERVED6: Self = Self(14);
    pub const RESERVED7: Self = Self(15);
    pub const RESERVED8: Self = Self(16);
    pub const RESERVED9: Self = Self(17);
    pub const RESERVED10: Self = Self(18);
    pub const CUSTOM: Self = Self(19);
}
pub const SDL_LOG_CATEGORY_APPLICATION: SDL_LogCategory = SDL_LogCategory::APPLICATION;
pub const SDL_LOG_CATEGORY_ERROR: SDL_LogCategory = SDL_LogCategory::ERROR;
pub const SDL_LOG_CATEGORY_ASSERT: SDL_LogCategory = SDL_LogCategory::ASSERT;
pub const SDL_LOG_CATEGORY_SYSTEM: SDL_LogCategory = SDL_LogCategory::SYSTEM;
pub const SDL_LOG_CATEGORY_AUDIO: SDL_LogCategory = SDL_LogCategory::AUDIO;
pub const SDL_LOG_CATEGORY_VIDEO: SDL_LogCategory = SDL_LogCategory::VIDEO;
pub const SDL_LOG_CATEGORY_RENDER: SDL_LogCategory = SDL_LogCategory::RENDER;
pub const SDL_LOG_CATEGORY_INPUT: SDL_LogCategory = SDL_LogCategory::INPUT;
pub const SDL_LOG_CATEGORY_TEST: SDL_LogCategory = SDL_LogCategory::TEST;
pub const SDL_LOG_CATEGORY_GPU: SDL_LogCategory = SDL_LogCategory::GPU;
pub const SDL_LOG_CATEGORY_RESERVED2: SDL_LogCategory = SDL_LogCategory::RESERVED2;
pub const SDL_LOG_CATEGORY_RESERVED3: SDL_LogCategory = SDL_LogCategory::RESERVED3;
pub const SDL_LOG_CATEGORY_RESERVED4: SDL_LogCategory = SDL_LogCategory::RESERVED4;
pub const SDL_LOG_CATEGORY_RESERVED5: SDL_LogCategory = SDL_LogCategory::RESERVED5;
pub const SDL_LOG_CATEGORY_RESERVED6: SDL_LogCategory = SDL_LogCategory::RESERVED6;
pub const SDL_LOG_CATEGORY_RESERVED7: SDL_LogCategory = SDL_LogCategory::RESERVED7;
pub const SDL_LOG_CATEGORY_RESERVED8: SDL_LogCategory = SDL_LogCategory::RESERVED8;
pub const SDL_LOG_CATEGORY_RESERVED9: SDL_LogCategory = SDL_LogCategory::RESERVED9;
pub const SDL_LOG_CATEGORY_RESERVED10: SDL_LogCategory = SDL_LogCategory::RESERVED10;
pub const SDL_LOG_CATEGORY_CUSTOM: SDL_LogCategory = SDL_LogCategory::CUSTOM;

/// The predefined log priorities
///
/// ### Availability
/// This enum is available since SDL 3.0.0.
///
/// ### `sdl3-sys` note
/// This is a `C` enum. Known values:
/// - [`SDL_LOG_PRIORITY_TRACE`]
/// - [`SDL_LOG_PRIORITY_VERBOSE`]
/// - [`SDL_LOG_PRIORITY_DEBUG`]
/// - [`SDL_LOG_PRIORITY_INFO`]
/// - [`SDL_LOG_PRIORITY_WARN`]
/// - [`SDL_LOG_PRIORITY_ERROR`]
/// - [`SDL_LOG_PRIORITY_CRITICAL`]
/// - [`SDL_LOG_PRIORITY_COUNT`]

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_LogPriority(pub ::core::ffi::c_int);
impl From<SDL_LogPriority> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_LogPriority) -> Self {
        value.0
    }
}
impl SDL_LogPriority {
    pub const INVALID: Self = Self(0);
    pub const TRACE: Self = Self(1);
    pub const VERBOSE: Self = Self(2);
    pub const DEBUG: Self = Self(3);
    pub const INFO: Self = Self(4);
    pub const WARN: Self = Self(5);
    pub const ERROR: Self = Self(6);
    pub const CRITICAL: Self = Self(7);
    pub const COUNT: Self = Self(8);
}
pub const SDL_LOG_PRIORITY_INVALID: SDL_LogPriority = SDL_LogPriority::INVALID;
pub const SDL_LOG_PRIORITY_TRACE: SDL_LogPriority = SDL_LogPriority::TRACE;
pub const SDL_LOG_PRIORITY_VERBOSE: SDL_LogPriority = SDL_LogPriority::VERBOSE;
pub const SDL_LOG_PRIORITY_DEBUG: SDL_LogPriority = SDL_LogPriority::DEBUG;
pub const SDL_LOG_PRIORITY_INFO: SDL_LogPriority = SDL_LogPriority::INFO;
pub const SDL_LOG_PRIORITY_WARN: SDL_LogPriority = SDL_LogPriority::WARN;
pub const SDL_LOG_PRIORITY_ERROR: SDL_LogPriority = SDL_LogPriority::ERROR;
pub const SDL_LOG_PRIORITY_CRITICAL: SDL_LogPriority = SDL_LogPriority::CRITICAL;
pub const SDL_LOG_PRIORITY_COUNT: SDL_LogPriority = SDL_LogPriority::COUNT;

extern "C" {
    /// Set the priority of all log categories.
    ///
    /// ### Arguments
    /// - `priority`: the [`SDL_LogPriority`] to assign.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_ResetLogPriorities`]
    /// - [`SDL_SetLogPriority`]
    pub fn SDL_SetLogPriorities(priority: SDL_LogPriority);
}

extern "C" {
    /// Set the priority of a particular log category.
    ///
    /// ### Arguments
    /// - `category`: the category to assign a priority to.
    /// - `priority`: the [`SDL_LogPriority`] to assign.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_GetLogPriority`]
    /// - [`SDL_ResetLogPriorities`]
    /// - [`SDL_SetLogPriorities`]
    pub fn SDL_SetLogPriority(category: ::core::ffi::c_int, priority: SDL_LogPriority);
}

extern "C" {
    /// Get the priority of a particular log category.
    ///
    /// ### Arguments
    /// - `category`: the category to query.
    /// ### Return value
    /// Returns the [`SDL_LogPriority`] for the requested category.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_SetLogPriority`]
    pub fn SDL_GetLogPriority(category: ::core::ffi::c_int) -> SDL_LogPriority;
}

extern "C" {
    /// Reset all priorities to default.
    ///
    /// This is called by [`SDL_Quit()`].
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_SetLogPriorities`]
    /// - [`SDL_SetLogPriority`]
    pub fn SDL_ResetLogPriorities();
}

extern "C" {
    /// Set the text prepended to log messages of a given priority.
    ///
    /// By default [`SDL_LOG_PRIORITY_INFO`] and below have no prefix, and
    /// [`SDL_LOG_PRIORITY_WARN`] and higher have a prefix showing their priority, e.g.
    /// "WARNING: ".
    ///
    /// ### Arguments
    /// - `priority`: the [`SDL_LogPriority`] to modify.
    /// - `prefix`: the prefix to use for that log priority, or NULL to use no
    ///   prefix.
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_SetLogPriorities`]
    /// - [`SDL_SetLogPriority`]
    pub fn SDL_SetLogPriorityPrefix(
        priority: SDL_LogPriority,
        prefix: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Log a message with [`SDL_LOG_CATEGORY_APPLICATION`] and [`SDL_LOG_PRIORITY_INFO`].
    ///
    /// ### Arguments
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the `fmt` string, if
    ///   any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_Log(fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_TRACE`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogTrace(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_VERBOSE`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogVerbose(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_DEBUG`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogDebug(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_INFO`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogInfo(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_WARN`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    pub fn SDL_LogWarn(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_ERROR`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogError(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with [`SDL_LOG_PRIORITY_CRITICAL`].
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogCritical(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    /// Log a message with the specified category and priority.
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `priority`: the priority of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `...`: additional parameters matching % tokens in the **fmt** string,
    ///   if any.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessageV`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogMessage(
        category: ::core::ffi::c_int,
        priority: SDL_LogPriority,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}

extern "C" {
    /// Log a message with the specified category and priority.
    ///
    /// ### Arguments
    /// - `category`: the category of the message.
    /// - `priority`: the priority of the message.
    /// - `fmt`: a printf() style message format string.
    /// - `ap`: a variable argument list.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_Log`]
    /// - [`SDL_LogCritical`]
    /// - [`SDL_LogDebug`]
    /// - [`SDL_LogError`]
    /// - [`SDL_LogInfo`]
    /// - [`SDL_LogMessage`]
    /// - [`SDL_LogTrace`]
    /// - [`SDL_LogVerbose`]
    /// - [`SDL_LogWarn`]
    pub fn SDL_LogMessageV(
        category: ::core::ffi::c_int,
        priority: SDL_LogPriority,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    );
}

/// The prototype for the log output callback function.
///
/// This function is called by SDL when there is new text to be logged. A mutex
/// is held so that this function is never called by more than one thread at
/// once.
///
/// ### Arguments
/// - `userdata`: what was passed as `userdata` to
///   [`SDL_SetLogOutputFunction()`].
/// - `category`: the category of the message.
/// - `priority`: the priority of the message.
/// - `message`: the message being output.
///
/// ### Availability
/// This datatype is available since SDL 3.0.0.
pub type SDL_LogOutputFunction = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        category: ::core::ffi::c_int,
        priority: SDL_LogPriority,
        message: *const ::core::ffi::c_char,
    ),
>;

extern "C" {
    /// Get the current log output function.
    ///
    /// ### Arguments
    /// - `callback`: an [`SDL_LogOutputFunction`] filled in with the current log
    ///   callback.
    /// - `userdata`: a pointer filled in with the pointer that is passed to
    ///   `callback`.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_SetLogOutputFunction`]
    pub fn SDL_GetLogOutputFunction(
        callback: *mut SDL_LogOutputFunction,
        userdata: *mut *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Replace the default log output function with one of your own.
    ///
    /// ### Arguments
    /// - `callback`: an [`SDL_LogOutputFunction`] to call instead of the default.
    /// - `userdata`: a pointer that is passed to `callback`.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.0.0.
    ///
    /// ### See also
    /// - [`SDL_GetLogOutputFunction`]
    pub fn SDL_SetLogOutputFunction(
        callback: SDL_LogOutputFunction,
        userdata: *mut ::core::ffi::c_void,
    );
}
