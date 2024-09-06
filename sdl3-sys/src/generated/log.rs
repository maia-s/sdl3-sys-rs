#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryLog
//!
//! Simple log messages with priorities and categories. A message's
//! SDL_LogPriority signifies how important the message is. A message's
//! SDL_LogCategory signifies from what domain it belongs to. Every category
//! has a minimum priority specified: when a message belongs to that category,
//! it will only be sent out if it has that minimum priority or higher.
//!
//! SDL's own logs are sent below the default priority threshold, so they are
//! quiet by default.
//!
//! You can change the log verbosity programmatically using
//! SDL_SetLogPriority() or with SDL_SetHint(SDL_HINT_LOGGING, ...), or with
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
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_LOG_CATEGORY_APPLICATION`], [`SDL_LOG_CATEGORY_ERROR`], [`SDL_LOG_CATEGORY_ASSERT`], [`SDL_LOG_CATEGORY_SYSTEM`], [`SDL_LOG_CATEGORY_AUDIO`], [`SDL_LOG_CATEGORY_VIDEO`], [`SDL_LOG_CATEGORY_RENDER`], [`SDL_LOG_CATEGORY_GPU`], [`SDL_LOG_CATEGORY_INPUT`], [`SDL_LOG_CATEGORY_TEST`], [`SDL_LOG_CATEGORY_RESERVED1`], [`SDL_LOG_CATEGORY_RESERVED2`], [`SDL_LOG_CATEGORY_RESERVED3`], [`SDL_LOG_CATEGORY_RESERVED4`], [`SDL_LOG_CATEGORY_RESERVED5`], [`SDL_LOG_CATEGORY_RESERVED6`], [`SDL_LOG_CATEGORY_RESERVED7`], [`SDL_LOG_CATEGORY_RESERVED8`], [`SDL_LOG_CATEGORY_RESERVED9`], [`SDL_LOG_CATEGORY_RESERVED10`], [`SDL_LOG_CATEGORY_CUSTOM`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_LogCategory(pub ::core::ffi::c_int);
impl SDL_LogCategory {
    pub const APPLICATION: Self = Self(0);
    pub const ERROR: Self = Self(1);
    pub const ASSERT: Self = Self(2);
    pub const SYSTEM: Self = Self(3);
    pub const AUDIO: Self = Self(4);
    pub const VIDEO: Self = Self(5);
    pub const RENDER: Self = Self(6);
    pub const GPU: Self = Self(7);
    pub const INPUT: Self = Self(8);
    pub const TEST: Self = Self(9);
    pub const RESERVED1: Self = Self(10);
    pub const RESERVED2: Self = Self(11);
    pub const RESERVED3: Self = Self(12);
    pub const RESERVED4: Self = Self(13);
    pub const RESERVED5: Self = Self(14);
    pub const RESERVED6: Self = Self(15);
    pub const RESERVED7: Self = Self(16);
    pub const RESERVED8: Self = Self(17);
    pub const RESERVED9: Self = Self(18);
    pub const RESERVED10: Self = Self(19);
    pub const CUSTOM: Self = Self(20);
}
pub const SDL_LOG_CATEGORY_APPLICATION: SDL_LogCategory = SDL_LogCategory::APPLICATION;
pub const SDL_LOG_CATEGORY_ERROR: SDL_LogCategory = SDL_LogCategory::ERROR;
pub const SDL_LOG_CATEGORY_ASSERT: SDL_LogCategory = SDL_LogCategory::ASSERT;
pub const SDL_LOG_CATEGORY_SYSTEM: SDL_LogCategory = SDL_LogCategory::SYSTEM;
pub const SDL_LOG_CATEGORY_AUDIO: SDL_LogCategory = SDL_LogCategory::AUDIO;
pub const SDL_LOG_CATEGORY_VIDEO: SDL_LogCategory = SDL_LogCategory::VIDEO;
pub const SDL_LOG_CATEGORY_RENDER: SDL_LogCategory = SDL_LogCategory::RENDER;
pub const SDL_LOG_CATEGORY_GPU: SDL_LogCategory = SDL_LogCategory::GPU;
pub const SDL_LOG_CATEGORY_INPUT: SDL_LogCategory = SDL_LogCategory::INPUT;
pub const SDL_LOG_CATEGORY_TEST: SDL_LogCategory = SDL_LogCategory::TEST;
pub const SDL_LOG_CATEGORY_RESERVED1: SDL_LogCategory = SDL_LogCategory::RESERVED1;
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
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_LOG_PRIORITY_VERBOSE`], [`SDL_LOG_PRIORITY_DEBUG`], [`SDL_LOG_PRIORITY_INFO`], [`SDL_LOG_PRIORITY_WARN`], [`SDL_LOG_PRIORITY_ERROR`], [`SDL_LOG_PRIORITY_CRITICAL`], [`SDL_NUM_LOG_PRIORITIES`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_LogPriority(pub ::core::ffi::c_int);
impl SDL_LogPriority {
    pub const LOG_PRIORITY_VERBOSE: Self = Self(1);
    pub const LOG_PRIORITY_DEBUG: Self = Self(2);
    pub const LOG_PRIORITY_INFO: Self = Self(3);
    pub const LOG_PRIORITY_WARN: Self = Self(4);
    pub const LOG_PRIORITY_ERROR: Self = Self(5);
    pub const LOG_PRIORITY_CRITICAL: Self = Self(6);
    pub const NUM_LOG_PRIORITIES: Self = Self(7);
}
pub const SDL_LOG_PRIORITY_VERBOSE: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_VERBOSE;
pub const SDL_LOG_PRIORITY_DEBUG: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_DEBUG;
pub const SDL_LOG_PRIORITY_INFO: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_INFO;
pub const SDL_LOG_PRIORITY_WARN: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_WARN;
pub const SDL_LOG_PRIORITY_ERROR: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_ERROR;
pub const SDL_LOG_PRIORITY_CRITICAL: SDL_LogPriority = SDL_LogPriority::LOG_PRIORITY_CRITICAL;
pub const SDL_NUM_LOG_PRIORITIES: SDL_LogPriority = SDL_LogPriority::NUM_LOG_PRIORITIES;

extern_sdlcall! {{
    /// Set the priority of all log categories.
    ///
    /// \param priority the SDL_LogPriority to assign.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ResetLogPriorities
    /// \sa SDL_SetLogPriority
    pub fn SDL_SetLogPriorities(priority: SDL_LogPriority);
}}

extern_sdlcall! {{
    /// Set the priority of a particular log category.
    ///
    /// \param category the category to assign a priority to.
    /// \param priority the SDL_LogPriority to assign.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetLogPriority
    /// \sa SDL_ResetLogPriorities
    /// \sa SDL_SetLogPriorities
    pub fn SDL_SetLogPriority(category: ::core::ffi::c_int, priority: SDL_LogPriority);
}}

extern_sdlcall! {{
    /// Get the priority of a particular log category.
    ///
    /// \param category the category to query.
    /// \returns the SDL_LogPriority for the requested category.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetLogPriority
    pub fn SDL_GetLogPriority(category: ::core::ffi::c_int) -> SDL_LogPriority;
}}

extern_sdlcall! {{
    /// Reset all priorities to default.
    ///
    /// This is called by SDL_Quit().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetLogPriorities
    /// \sa SDL_SetLogPriority
    pub fn SDL_ResetLogPriorities();
}}

extern_sdlcall! {{
    /// Set the text prepended to log messages of a given priority.
    ///
    /// By default SDL_LOG_PRIORITY_INFO and below have no prefix, and
    /// SDL_LOG_PRIORITY_WARN and higher have a prefix showing their priority, e.g.
    /// "WARNING: ".
    ///
    /// \param priority the SDL_LogPriority to modify.
    /// \param prefix the prefix to use for that log priority, or NULL to use no
    ///               prefix.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetLogPriorities
    /// \sa SDL_SetLogPriority
    pub fn SDL_SetLogPriorityPrefix(priority: SDL_LogPriority, prefix: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_CATEGORY_APPLICATION and SDL_LOG_PRIORITY_INFO.
    ///
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the `fmt` string, if
    ///            any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_Log(fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_VERBOSE.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogWarn
    pub fn SDL_LogVerbose(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_DEBUG.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogDebug(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_INFO.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogInfo(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_WARN.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    pub fn SDL_LogWarn(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_ERROR.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogError(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with SDL_LOG_PRIORITY_CRITICAL.
    ///
    /// \param category the category of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogCritical(category: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with the specified category and priority.
    ///
    /// \param category the category of the message.
    /// \param priority the priority of the message.
    /// \param fmt a printf() style message format string.
    /// \param ... additional parameters matching % tokens in the **fmt** string,
    ///            if any.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessageV
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogMessage(category: ::core::ffi::c_int, priority: SDL_LogPriority, fmt: *const ::core::ffi::c_char, ...);
}}

extern_sdlcall! {{
    /// Log a message with the specified category and priority.
    ///
    /// \param category the category of the message.
    /// \param priority the priority of the message.
    /// \param fmt a printf() style message format string.
    /// \param ap a variable argument list.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Log
    /// \sa SDL_LogCritical
    /// \sa SDL_LogDebug
    /// \sa SDL_LogError
    /// \sa SDL_LogInfo
    /// \sa SDL_LogMessage
    /// \sa SDL_LogVerbose
    /// \sa SDL_LogWarn
    pub fn SDL_LogMessageV(category: ::core::ffi::c_int, priority: SDL_LogPriority, fmt: *const ::core::ffi::c_char, ap: crate::ffi::VaList);
}}

/// The prototype for the log output callback function.
///
/// This function is called by SDL when there is new text to be logged.
///
/// \param userdata what was passed as `userdata` to
///                 SDL_SetLogOutputFunction().
/// \param category the category of the message.
/// \param priority the priority of the message.
/// \param message the message being output.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_LogOutputFunction = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, category: ::core::ffi::c_int, priority: SDL_LogPriority, message: *const ::core::ffi::c_char))>;

extern_sdlcall! {{
    /// Get the current log output function.
    ///
    /// \param callback an SDL_LogOutputFunction filled in with the current log
    ///                 callback.
    /// \param userdata a pointer filled in with the pointer that is passed to
    ///                 `callback`.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetLogOutputFunction
    pub fn SDL_GetLogOutputFunction(callback: *mut SDL_LogOutputFunction, userdata: *mut *mut ::core::ffi::c_void);
}}

extern_sdlcall! {{
    /// Replace the default log output function with one of your own.
    ///
    /// \param callback an SDL_LogOutputFunction to call instead of the default.
    /// \param userdata a pointer that is passed to `callback`.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetLogOutputFunction
    pub fn SDL_SetLogOutputFunction(callback: SDL_LogOutputFunction, userdata: *mut ::core::ffi::c_void);
}}

