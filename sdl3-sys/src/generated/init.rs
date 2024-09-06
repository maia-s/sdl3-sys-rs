#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryInit
//!
//! SDL subsystem init and quit functions.

use super::stdinc::*;

use super::error::*;

use super::events::*;

/// Initialization flags for SDL_Init and/or SDL_InitSubSystem
///
/// These are the flags which may be passed to SDL_Init(). You should specify
/// the subsystems which you will be using in your application.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_Init
/// \sa SDL_Quit
/// \sa SDL_InitSubSystem
/// \sa SDL_QuitSubSystem
/// \sa SDL_WasInit
pub type SDL_InitFlags = Uint32;

pub const SDL_INIT_TIMER: ::core::primitive::u32 = 1_u32;

/// `SDL_INIT_AUDIO` implies `SDL_INIT_EVENTS`
pub const SDL_INIT_AUDIO: ::core::primitive::u32 = 16_u32;

/// `SDL_INIT_VIDEO` implies `SDL_INIT_EVENTS`
pub const SDL_INIT_VIDEO: ::core::primitive::u32 = 32_u32;

/// `SDL_INIT_JOYSTICK` implies `SDL_INIT_EVENTS`, should be initialized on the same thread as SDL_INIT_VIDEO on Windows if you don't set SDL_HINT_JOYSTICK_THREAD
pub const SDL_INIT_JOYSTICK: ::core::primitive::u32 = 512_u32;

pub const SDL_INIT_HAPTIC: ::core::primitive::u32 = 4096_u32;

/// `SDL_INIT_GAMEPAD` implies `SDL_INIT_JOYSTICK`
pub const SDL_INIT_GAMEPAD: ::core::primitive::u32 = 8192_u32;

pub const SDL_INIT_EVENTS: ::core::primitive::u32 = 16384_u32;

/// `SDL_INIT_SENSOR` implies `SDL_INIT_EVENTS`
pub const SDL_INIT_SENSOR: ::core::primitive::u32 = 32768_u32;

/// `SDL_INIT_CAMERA` implies `SDL_INIT_EVENTS`
pub const SDL_INIT_CAMERA: ::core::primitive::u32 = 65536_u32;

/// Return values for optional main callbacks.
///
/// Returning SDL_APP_SUCCESS or SDL_APP_FAILURE from SDL_AppInit,
/// SDL_AppEvent, or SDL_AppIterate will terminate the program and report
/// success/failure to the operating system. What that means is
/// platform-dependent. On Unix, for example, on success, the process error
/// code will be zero, and on failure it will be 1. This interface doesn't
/// allow you to return specific exit codes, just whether there was an error
/// generally or not.
///
/// Returning SDL_APP_CONTINUE from these functions will let the app continue
/// to run.
///
/// See
/// [Main callbacks in SDL3](https://wiki.libsdl.org/SDL3/README/main-functions#main-callbacks-in-sdl3)
/// for complete details.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_APP_CONTINUE`], [`SDL_APP_SUCCESS`], [`SDL_APP_FAILURE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AppResult(pub ::core::ffi::c_int);
impl SDL_AppResult {
    /// Value that requests that the app continue from the main callbacks.
    pub const CONTINUE: Self = Self(0);
    /// Value that requests termination with success from the main callbacks.
    pub const SUCCESS: Self = Self(1);
    /// Value that requests termination with error from the main callbacks.
    pub const FAILURE: Self = Self(2);
}
/// Value that requests that the app continue from the main callbacks.
pub const SDL_APP_CONTINUE: SDL_AppResult = SDL_AppResult::CONTINUE;
/// Value that requests termination with success from the main callbacks.
pub const SDL_APP_SUCCESS: SDL_AppResult = SDL_AppResult::SUCCESS;
/// Value that requests termination with error from the main callbacks.
pub const SDL_APP_FAILURE: SDL_AppResult = SDL_AppResult::FAILURE;

pub type SDL_AppInit_func = ::core::option::Option<extern_sdlcall!(fn(appstate: *mut *mut ::core::ffi::c_void, argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> SDL_AppResult)>;

pub type SDL_AppIterate_func = ::core::option::Option<extern_sdlcall!(fn(appstate: *mut ::core::ffi::c_void) -> SDL_AppResult)>;

pub type SDL_AppEvent_func = ::core::option::Option<extern_sdlcall!(fn(appstate: *mut ::core::ffi::c_void, event: *mut SDL_Event) -> SDL_AppResult)>;

pub type SDL_AppQuit_func = ::core::option::Option<extern_sdlcall!(fn(appstate: *mut ::core::ffi::c_void))>;

extern_sdlcall! {{
    /// Initialize the SDL library.
    ///
    /// SDL_Init() simply forwards to calling SDL_InitSubSystem(). Therefore, the
    /// two may be used interchangeably. Though for readability of your code
    /// SDL_InitSubSystem() might be preferred.
    ///
    /// The file I/O (for example: SDL_IOFromFile) and threading (SDL_CreateThread)
    /// subsystems are initialized by default. Message boxes
    /// (SDL_ShowSimpleMessageBox) also attempt to work without initializing the
    /// video subsystem, in hopes of being useful in showing an error dialog when
    /// SDL_Init fails. You must specifically initialize other subsystems if you
    /// use them in your application.
    ///
    /// Logging (such as SDL_Log) works without initialization, too.
    ///
    /// `flags` may be any of the following OR'd together:
    ///
    /// - `SDL_INIT_TIMER`: timer subsystem
    /// - `SDL_INIT_AUDIO`: audio subsystem; automatically initializes the events
    ///   subsystem
    /// - `SDL_INIT_VIDEO`: video subsystem; automatically initializes the events
    ///   subsystem
    /// - `SDL_INIT_JOYSTICK`: joystick subsystem; automatically initializes the
    ///   events subsystem
    /// - `SDL_INIT_HAPTIC`: haptic (force feedback) subsystem
    /// - `SDL_INIT_GAMEPAD`: gamepad subsystem; automatically initializes the
    ///   joystick subsystem
    /// - `SDL_INIT_EVENTS`: events subsystem
    /// - `SDL_INIT_SENSOR`: sensor subsystem; automatically initializes the events
    ///   subsystem
    /// - `SDL_INIT_CAMERA`: camera subsystem; automatically initializes the events
    ///   subsystem
    ///
    /// Subsystem initialization is ref-counted, you must call SDL_QuitSubSystem()
    /// for each SDL_InitSubSystem() to correctly shutdown a subsystem manually (or
    /// call SDL_Quit() to force shutdown). If a subsystem is already loaded then
    /// this call will increase the ref-count and return.
    ///
    /// Consider reporting some basic metadata about your application before
    /// calling SDL_Init, using either SDL_SetAppMetadata() or
    /// SDL_SetAppMetadataProperty().
    ///
    /// \param flags subsystem initialization flags.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetAppMetadata
    /// \sa SDL_SetAppMetadataProperty
    /// \sa SDL_InitSubSystem
    /// \sa SDL_Quit
    /// \sa SDL_SetMainReady
    /// \sa SDL_WasInit
    pub fn SDL_Init(flags: SDL_InitFlags) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Compatibility function to initialize the SDL library.
    ///
    /// This function and SDL_Init() are interchangeable.
    ///
    /// \param flags any of the flags used by SDL_Init(); see SDL_Init for details.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Init
    /// \sa SDL_Quit
    /// \sa SDL_QuitSubSystem
    pub fn SDL_InitSubSystem(flags: SDL_InitFlags) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Shut down specific SDL subsystems.
    ///
    /// You still need to call SDL_Quit() even if you close all open subsystems
    /// with SDL_QuitSubSystem().
    ///
    /// \param flags any of the flags used by SDL_Init(); see SDL_Init for details.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_InitSubSystem
    /// \sa SDL_Quit
    pub fn SDL_QuitSubSystem(flags: SDL_InitFlags);
}}

extern_sdlcall! {{
    /// Get a mask of the specified subsystems which are currently initialized.
    ///
    /// \param flags any of the flags used by SDL_Init(); see SDL_Init for details.
    /// \returns a mask of all initialized subsystems if `flags` is 0, otherwise it
    ///          returns the initialization status of the specified subsystems.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Init
    /// \sa SDL_InitSubSystem
    pub fn SDL_WasInit(flags: SDL_InitFlags) -> SDL_InitFlags;
}}

extern_sdlcall! {{
    /// Clean up all initialized subsystems.
    ///
    /// You should call this function even if you have already shutdown each
    /// initialized subsystem with SDL_QuitSubSystem(). It is safe to call this
    /// function even in the case of errors in initialization.
    ///
    /// You can use this function with atexit() to ensure that it is run when your
    /// application is shutdown, but it is not wise to do this from a library or
    /// other dynamically loaded code.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Init
    /// \sa SDL_QuitSubSystem
    pub fn SDL_Quit();
}}

extern_sdlcall! {{
    /// Specify basic metadata about your app.
    ///
    /// You can optionally provide metadata about your app to SDL. This is not
    /// required, but strongly encouraged.
    ///
    /// There are several locations where SDL can make use of metadata (an "About"
    /// box in the macOS menu bar, the name of the app can be shown on some audio
    /// mixers, etc). Any piece of metadata can be left as NULL, if a specific
    /// detail doesn't make sense for the app.
    ///
    /// This function should be called as early as possible, before SDL_Init.
    /// Multiple calls to this function are allowed, but various state might not
    /// change once it has been set up with a previous call to this function.
    ///
    /// Passing a NULL removes any previous metadata.
    ///
    /// This is a simplified interface for the most important information. You can
    /// supply significantly more detailed metadata with
    /// SDL_SetAppMetadataProperty().
    ///
    /// \param appname The name of the application ("My Game 2: Bad Guy's
    ///                Revenge!").
    /// \param appversion The version of the application ("1.0.0beta5" or a git
    ///                   hash, or whatever makes sense).
    /// \param appidentifier A unique string in reverse-domain format that
    ///                      identifies this app ("com.example.mygame2").
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetAppMetadataProperty
    pub fn SDL_SetAppMetadata(appname: *const ::core::ffi::c_char, appversion: *const ::core::ffi::c_char, appidentifier: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Specify metadata about your app through a set of properties.
    ///
    /// You can optionally provide metadata about your app to SDL. This is not
    /// required, but strongly encouraged.
    ///
    /// There are several locations where SDL can make use of metadata (an "About"
    /// box in the macOS menu bar, the name of the app can be shown on some audio
    /// mixers, etc). Any piece of metadata can be left out, if a specific detail
    /// doesn't make sense for the app.
    ///
    /// This function should be called as early as possible, before SDL_Init.
    /// Multiple calls to this function are allowed, but various state might not
    /// change once it has been set up with a previous call to this function.
    ///
    /// Once set, this metadata can be read using SDL_GetMetadataProperty().
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_APP_METADATA_NAME_STRING`: The human-readable name of the
    ///   application, like "My Game 2: Bad Guy's Revenge!". This will show up
    ///   anywhere the OS shows the name of the application separately from window
    ///   titles, such as volume control applets, etc. This defaults to "SDL
    ///   Application".
    /// - `SDL_PROP_APP_METADATA_VERSION_STRING`: The version of the app that is
    ///   running; there are no rules on format, so "1.0.3beta2" and "April 22nd,
    ///   2024" and a git hash are all valid options. This has no default.
    /// - `SDL_PROP_APP_METADATA_IDENTIFIER_STRING`: A unique string that
    ///   identifies this app. This must be in reverse-domain format, like
    ///   "com.example.mygame2". This string is used by desktop compositors to
    ///   identify and group windows together, as well as match applications with
    ///   associated desktop settings and icons. If you plan to package your
    ///   application in a container such as Flatpak, the app ID should match the
    ///   name of your Flatpak container as well. This has no default.
    /// - `SDL_PROP_APP_METADATA_CREATOR_STRING`: The human-readable name of the
    ///   creator/developer/maker of this app, like "MojoWorkshop, LLC"
    /// - `SDL_PROP_APP_METADATA_COPYRIGHT_STRING`: The human-readable copyright
    ///   notice, like "Copyright (c) 2024 MojoWorkshop, LLC" or whatnot. Keep this
    ///   to one line, don't paste a copy of a whole software license in here. This
    ///   has no default.
    /// - `SDL_PROP_APP_METADATA_URL_STRING`: A URL to the app on the web. Maybe a
    ///   product page, or a storefront, or even a GitHub repository, for user's
    ///   further information This has no default.
    /// - `SDL_PROP_APP_METADATA_TYPE_STRING`: The type of application this is.
    ///   Currently this string can be "game" for a video game, "mediaplayer" for a
    ///   media player, or generically "application" if nothing else applies.
    ///   Future versions of SDL might add new types. This defaults to
    ///   "application".
    ///
    /// \param name the name of the metadata property to set.
    /// \param value the value of the property, or NULL to remove that property.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAppMetadataProperty
    /// \sa SDL_SetAppMetadata
    pub fn SDL_SetAppMetadataProperty(name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char) -> SDL_bool;
}}

pub const SDL_PROP_APP_METADATA_NAME_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.name\0") };

pub const SDL_PROP_APP_METADATA_VERSION_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.version\0") };

pub const SDL_PROP_APP_METADATA_IDENTIFIER_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.identifier\0") };

pub const SDL_PROP_APP_METADATA_CREATOR_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.creator\0") };

pub const SDL_PROP_APP_METADATA_COPYRIGHT_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.copyright\0") };

pub const SDL_PROP_APP_METADATA_URL_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.url\0") };

pub const SDL_PROP_APP_METADATA_TYPE_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.app.metadata.type\0") };

extern_sdlcall! {{
    /// Get metadata about your app.
    ///
    /// This returns metadata previously set using SDL_SetAppMetadata() or
    /// SDL_SetAppMetadataProperty(). See SDL_SetAppMetadataProperty() for the list
    /// of available properties and their meanings.
    ///
    /// \param name the name of the metadata property to get.
    /// \returns the current value of the metadata property, or the default if it
    ///          is not set, NULL for properties with no default.
    ///
    /// \threadsafety It is safe to call this function from any thread, although
    ///               the string returned is not protected and could potentially be
    ///               freed if you call SDL_SetAppMetadataProperty() to set that
    ///               property from another thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetAppMetadata
    /// \sa SDL_SetAppMetadataProperty
    pub fn SDL_GetAppMetadataProperty(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}}
