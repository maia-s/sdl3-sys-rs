//! All SDL programs need to initialize the library before starting to work
//! with it.
//!
//! Almost everything can simply call [`SDL_Init()`] near startup, with a handful
//! of flags to specify subsystems to touch. These are here to make sure SDL
//! does not even attempt to touch low-level pieces of the operating system
//! that you don't intend to use. For example, you might be using SDL for video
//! and input but chose an external library for audio, and in this case you
//! would just need to leave off the [`SDL_INIT_AUDIO`] flag to make sure that
//! external library has complete control.
//!
//! Most apps, when terminating, should call [`SDL_Quit()`]. This will clean up
//! (nearly) everything that SDL might have allocated, and crucially, it'll
//! make sure that the display's resolution is back to what the user expects if
//! you had previously changed it for your game.
//!
//! SDL3 apps are strongly encouraged to call [`SDL_SetAppMetadata()`] at startup
//! to fill in details about the program. This is completely optional, but it
//! helps in small ways (we can provide an About dialog box for the macOS menu,
//! we can name the app in the system's audio mixer, etc). Those that want to
//! provide a _lot_ of information should look at the more-detailed
//! [`SDL_SetAppMetadataProperty()`].

use super::stdinc::*;

use super::error::*;

use super::events::*;

/// Initialization flags for [`SDL_Init`] and/or [`SDL_InitSubSystem`]
///
/// These are the flags which may be passed to [`SDL_Init()`]. You should specify
/// the subsystems which you will be using in your application.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_Init`]
/// - [`SDL_Quit`]
/// - [`SDL_InitSubSystem`]
/// - [`SDL_QuitSubSystem`]
/// - [`SDL_WasInit`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`AUDIO`](SDL_InitFlags::AUDIO) | [`SDL_INIT_AUDIO`] | [`SDL_INIT_AUDIO`] implies [`SDL_INIT_EVENTS`] |
/// | [`VIDEO`](SDL_InitFlags::VIDEO) | [`SDL_INIT_VIDEO`] | [`SDL_INIT_VIDEO`] implies [`SDL_INIT_EVENTS`], should be initialized on the main thread |
/// | [`JOYSTICK`](SDL_InitFlags::JOYSTICK) | [`SDL_INIT_JOYSTICK`] | [`SDL_INIT_JOYSTICK`] implies [`SDL_INIT_EVENTS`] |
/// | [`HAPTIC`](SDL_InitFlags::HAPTIC) | [`SDL_INIT_HAPTIC`] | |
/// | [`GAMEPAD`](SDL_InitFlags::GAMEPAD) | [`SDL_INIT_GAMEPAD`] | [`SDL_INIT_GAMEPAD`] implies [`SDL_INIT_JOYSTICK`] |
/// | [`EVENTS`](SDL_InitFlags::EVENTS) | [`SDL_INIT_EVENTS`] | |
/// | [`SENSOR`](SDL_InitFlags::SENSOR) | [`SDL_INIT_SENSOR`] | [`SDL_INIT_SENSOR`] implies [`SDL_INIT_EVENTS`] |
/// | [`CAMERA`](SDL_InitFlags::CAMERA) | [`SDL_INIT_CAMERA`] | [`SDL_INIT_CAMERA`] implies [`SDL_INIT_EVENTS`] |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_InitFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_InitFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_InitFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_InitFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_InitFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_InitFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_InitFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_InitFlags(")?;
        let all_bits = all_bits | Self::AUDIO.0;
        if (Self::AUDIO != 0 || self.0 == 0) && *self & Self::AUDIO == Self::AUDIO {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "AUDIO")?;
        }
        let all_bits = all_bits | Self::VIDEO.0;
        if (Self::VIDEO != 0 || self.0 == 0) && *self & Self::VIDEO == Self::VIDEO {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "VIDEO")?;
        }
        let all_bits = all_bits | Self::JOYSTICK.0;
        if (Self::JOYSTICK != 0 || self.0 == 0) && *self & Self::JOYSTICK == Self::JOYSTICK {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "JOYSTICK")?;
        }
        let all_bits = all_bits | Self::HAPTIC.0;
        if (Self::HAPTIC != 0 || self.0 == 0) && *self & Self::HAPTIC == Self::HAPTIC {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "HAPTIC")?;
        }
        let all_bits = all_bits | Self::GAMEPAD.0;
        if (Self::GAMEPAD != 0 || self.0 == 0) && *self & Self::GAMEPAD == Self::GAMEPAD {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "GAMEPAD")?;
        }
        let all_bits = all_bits | Self::EVENTS.0;
        if (Self::EVENTS != 0 || self.0 == 0) && *self & Self::EVENTS == Self::EVENTS {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "EVENTS")?;
        }
        let all_bits = all_bits | Self::SENSOR.0;
        if (Self::SENSOR != 0 || self.0 == 0) && *self & Self::SENSOR == Self::SENSOR {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "SENSOR")?;
        }
        let all_bits = all_bits | Self::CAMERA.0;
        if (Self::CAMERA != 0 || self.0 == 0) && *self & Self::CAMERA == Self::CAMERA {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "CAMERA")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_InitFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_InitFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_InitFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_InitFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_InitFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_InitFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_InitFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_InitFlags {
    /// [`SDL_INIT_AUDIO`] implies [`SDL_INIT_EVENTS`]
    pub const AUDIO: Self = Self((0x00000010 as Uint32));
    /// [`SDL_INIT_VIDEO`] implies [`SDL_INIT_EVENTS`], should be initialized on the main thread
    pub const VIDEO: Self = Self((0x00000020 as Uint32));
    /// [`SDL_INIT_JOYSTICK`] implies [`SDL_INIT_EVENTS`]
    pub const JOYSTICK: Self = Self((0x00000200 as Uint32));
    pub const HAPTIC: Self = Self((0x00001000 as Uint32));
    /// [`SDL_INIT_GAMEPAD`] implies [`SDL_INIT_JOYSTICK`]
    pub const GAMEPAD: Self = Self((0x00002000 as Uint32));
    pub const EVENTS: Self = Self((0x00004000 as Uint32));
    /// [`SDL_INIT_SENSOR`] implies [`SDL_INIT_EVENTS`]
    pub const SENSOR: Self = Self((0x00008000 as Uint32));
    /// [`SDL_INIT_CAMERA`] implies [`SDL_INIT_EVENTS`]
    pub const CAMERA: Self = Self((0x00010000 as Uint32));
}

/// [`SDL_INIT_AUDIO`] implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_AUDIO: SDL_InitFlags = SDL_InitFlags::AUDIO;
/// [`SDL_INIT_VIDEO`] implies [`SDL_INIT_EVENTS`], should be initialized on the main thread
pub const SDL_INIT_VIDEO: SDL_InitFlags = SDL_InitFlags::VIDEO;
/// [`SDL_INIT_JOYSTICK`] implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_JOYSTICK: SDL_InitFlags = SDL_InitFlags::JOYSTICK;
pub const SDL_INIT_HAPTIC: SDL_InitFlags = SDL_InitFlags::HAPTIC;
/// [`SDL_INIT_GAMEPAD`] implies [`SDL_INIT_JOYSTICK`]
pub const SDL_INIT_GAMEPAD: SDL_InitFlags = SDL_InitFlags::GAMEPAD;
pub const SDL_INIT_EVENTS: SDL_InitFlags = SDL_InitFlags::EVENTS;
/// [`SDL_INIT_SENSOR`] implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_SENSOR: SDL_InitFlags = SDL_InitFlags::SENSOR;
/// [`SDL_INIT_CAMERA`] implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_CAMERA: SDL_InitFlags = SDL_InitFlags::CAMERA;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_InitFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::init::METADATA_SDL_InitFlags;
}

/// Return values for optional main callbacks.
///
/// Returning [`SDL_APP_SUCCESS`] or [`SDL_APP_FAILURE`] from [`SDL_AppInit`],
/// [`SDL_AppEvent`], or [`SDL_AppIterate`] will terminate the program and report
/// success/failure to the operating system. What that means is
/// platform-dependent. On Unix, for example, on success, the process error
/// code will be zero, and on failure it will be 1. This interface doesn't
/// allow you to return specific exit codes, just whether there was an error
/// generally or not.
///
/// Returning [`SDL_APP_CONTINUE`] from these functions will let the app continue
/// to run.
///
/// See
/// [Main callbacks in SDL3](https://wiki.libsdl.org/SDL3/README/main-functions#main-callbacks-in-sdl3)
/// for complete details.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`CONTINUE`](SDL_AppResult::CONTINUE) | [`SDL_APP_CONTINUE`] | Value that requests that the app continue from the main callbacks. |
/// | [`SUCCESS`](SDL_AppResult::SUCCESS) | [`SDL_APP_SUCCESS`] | Value that requests termination with success from the main callbacks. |
/// | [`FAILURE`](SDL_AppResult::FAILURE) | [`SDL_APP_FAILURE`] | Value that requests termination with error from the main callbacks. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_AppResult(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_AppResult {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_AppResult> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_AppResult) -> bool {
        self == &other.0
    }
}

impl From<SDL_AppResult> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_AppResult) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_AppResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::CONTINUE => "SDL_APP_CONTINUE",
            Self::SUCCESS => "SDL_APP_SUCCESS",
            Self::FAILURE => "SDL_APP_FAILURE",

            _ => return write!(f, "SDL_AppResult({})", self.0),
        })
    }
}

impl SDL_AppResult {
    /// Value that requests that the app continue from the main callbacks.
    pub const CONTINUE: Self = Self((0 as ::core::ffi::c_int));
    /// Value that requests termination with success from the main callbacks.
    pub const SUCCESS: Self = Self((1 as ::core::ffi::c_int));
    /// Value that requests termination with error from the main callbacks.
    pub const FAILURE: Self = Self((2 as ::core::ffi::c_int));
}

/// Value that requests that the app continue from the main callbacks.
pub const SDL_APP_CONTINUE: SDL_AppResult = SDL_AppResult::CONTINUE;
/// Value that requests termination with success from the main callbacks.
pub const SDL_APP_SUCCESS: SDL_AppResult = SDL_AppResult::SUCCESS;
/// Value that requests termination with error from the main callbacks.
pub const SDL_APP_FAILURE: SDL_AppResult = SDL_AppResult::FAILURE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_AppResult {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::init::METADATA_SDL_AppResult;
}

/// Function pointer typedef for [`SDL_AppInit`].
///
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind
/// the scenes for apps using the optional main callbacks. Apps that want to
/// use this should just implement [`SDL_AppInit`] directly.
///
/// ## Parameters
/// - `appstate`: a place where the app can optionally store a pointer for
///   future use.
/// - `argc`: the standard ANSI C main's argc; number of elements in `argv`.
/// - `argv`: the standard ANSI C main's argv; array of command line
///   arguments.
///
/// ## Return value
/// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
pub type SDL_AppInit_func = ::core::option::Option<
    unsafe extern "C" fn(
        appstate: *mut *mut ::core::ffi::c_void,
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> SDL_AppResult,
>;

/// Function pointer typedef for [`SDL_AppIterate`].
///
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind
/// the scenes for apps using the optional main callbacks. Apps that want to
/// use this should just implement [`SDL_AppIterate`] directly.
///
/// ## Parameters
/// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
///
/// ## Return value
/// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
pub type SDL_AppIterate_func = ::core::option::Option<
    unsafe extern "C" fn(appstate: *mut ::core::ffi::c_void) -> SDL_AppResult,
>;

/// Function pointer typedef for [`SDL_AppEvent`].
///
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind
/// the scenes for apps using the optional main callbacks. Apps that want to
/// use this should just implement [`SDL_AppEvent`] directly.
///
/// ## Parameters
/// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
/// - `event`: the new event for the app to examine.
///
/// ## Return value
/// Returns [`SDL_APP_FAILURE`] to terminate with an error, [`SDL_APP_SUCCESS`] to
///   terminate with success, [`SDL_APP_CONTINUE`] to continue.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
pub type SDL_AppEvent_func = ::core::option::Option<
    unsafe extern "C" fn(
        appstate: *mut ::core::ffi::c_void,
        event: *mut SDL_Event,
    ) -> SDL_AppResult,
>;

/// Function pointer typedef for [`SDL_AppQuit`].
///
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind
/// the scenes for apps using the optional main callbacks. Apps that want to
/// use this should just implement [`SDL_AppEvent`] directly.
///
/// ## Parameters
/// - `appstate`: an optional pointer, provided by the app in [`SDL_AppInit`].
/// - `result`: the result code that terminated the app (success or failure).
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
pub type SDL_AppQuit_func = ::core::option::Option<
    unsafe extern "C" fn(appstate: *mut ::core::ffi::c_void, result: SDL_AppResult),
>;

extern "C" {
    /// Initialize the SDL library.
    ///
    /// [`SDL_Init()`] simply forwards to calling [`SDL_InitSubSystem()`]. Therefore, the
    /// two may be used interchangeably. Though for readability of your code
    /// [`SDL_InitSubSystem()`] might be preferred.
    ///
    /// The file I/O (for example: [`SDL_IOFromFile`]) and threading ([`SDL_CreateThread`])
    /// subsystems are initialized by default. Message boxes
    /// ([`SDL_ShowSimpleMessageBox`]) also attempt to work without initializing the
    /// video subsystem, in hopes of being useful in showing an error dialog when
    /// [`SDL_Init`] fails. You must specifically initialize other subsystems if you
    /// use them in your application.
    ///
    /// Logging (such as [`SDL_Log`]) works without initialization, too.
    ///
    /// `flags` may be any of the following OR'd together:
    ///
    /// - [`SDL_INIT_AUDIO`]\: audio subsystem; automatically initializes the events
    ///   subsystem
    /// - [`SDL_INIT_VIDEO`]\: video subsystem; automatically initializes the events
    ///   subsystem, should be initialized on the main thread.
    /// - [`SDL_INIT_JOYSTICK`]\: joystick subsystem; automatically initializes the
    ///   events subsystem
    /// - [`SDL_INIT_HAPTIC`]\: haptic (force feedback) subsystem
    /// - [`SDL_INIT_GAMEPAD`]\: gamepad subsystem; automatically initializes the
    ///   joystick subsystem
    /// - [`SDL_INIT_EVENTS`]\: events subsystem
    /// - [`SDL_INIT_SENSOR`]\: sensor subsystem; automatically initializes the events
    ///   subsystem
    /// - [`SDL_INIT_CAMERA`]\: camera subsystem; automatically initializes the events
    ///   subsystem
    ///
    /// Subsystem initialization is ref-counted, you must call [`SDL_QuitSubSystem()`]
    /// for each [`SDL_InitSubSystem()`] to correctly shutdown a subsystem manually (or
    /// call [`SDL_Quit()`] to force shutdown). If a subsystem is already loaded then
    /// this call will increase the ref-count and return.
    ///
    /// Consider reporting some basic metadata about your application before
    /// calling [`SDL_Init`], using either [`SDL_SetAppMetadata()`] or
    /// [`SDL_SetAppMetadataProperty()`].
    ///
    /// ## Parameters
    /// - `flags`: subsystem initialization flags.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetAppMetadata`]
    /// - [`SDL_SetAppMetadataProperty`]
    /// - [`SDL_InitSubSystem`]
    /// - [`SDL_Quit`]
    /// - [`SDL_SetMainReady`]
    /// - [`SDL_WasInit`]
    pub fn SDL_Init(flags: SDL_InitFlags) -> ::core::primitive::bool;
}

extern "C" {
    /// Compatibility function to initialize the SDL library.
    ///
    /// This function and [`SDL_Init()`] are interchangeable.
    ///
    /// ## Parameters
    /// - `flags`: any of the flags used by [`SDL_Init()`]; see [`SDL_Init`] for details.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Init`]
    /// - [`SDL_Quit`]
    /// - [`SDL_QuitSubSystem`]
    pub fn SDL_InitSubSystem(flags: SDL_InitFlags) -> ::core::primitive::bool;
}

extern "C" {
    /// Shut down specific SDL subsystems.
    ///
    /// You still need to call [`SDL_Quit()`] even if you close all open subsystems
    /// with [`SDL_QuitSubSystem()`].
    ///
    /// ## Parameters
    /// - `flags`: any of the flags used by [`SDL_Init()`]; see [`SDL_Init`] for details.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_InitSubSystem`]
    /// - [`SDL_Quit`]
    pub fn SDL_QuitSubSystem(flags: SDL_InitFlags);
}

extern "C" {
    /// Get a mask of the specified subsystems which are currently initialized.
    ///
    /// ## Parameters
    /// - `flags`: any of the flags used by [`SDL_Init()`]; see [`SDL_Init`] for details.
    ///
    /// ## Return value
    /// Returns a mask of all initialized subsystems if `flags` is 0, otherwise it
    ///   returns the initialization status of the specified subsystems.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Init`]
    /// - [`SDL_InitSubSystem`]
    pub fn SDL_WasInit(flags: SDL_InitFlags) -> SDL_InitFlags;
}

extern "C" {
    /// Clean up all initialized subsystems.
    ///
    /// You should call this function even if you have already shutdown each
    /// initialized subsystem with [`SDL_QuitSubSystem()`]. It is safe to call this
    /// function even in the case of errors in initialization.
    ///
    /// You can use this function with atexit() to ensure that it is run when your
    /// application is shutdown, but it is not wise to do this from a library or
    /// other dynamically loaded code.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Init`]
    /// - [`SDL_QuitSubSystem`]
    pub fn SDL_Quit();
}

extern "C" {
    /// Return whether this is the main thread.
    ///
    /// On Apple platforms, the main thread is the thread that runs your program's
    /// main() entry point. On other platforms, the main thread is the one that
    /// calls SDL_Init([`SDL_INIT_VIDEO`]), which should usually be the one that runs
    /// your program's main() entry point. If you are using the main callbacks,
    /// [`SDL_AppInit()`], [`SDL_AppIterate()`], and [`SDL_AppQuit()`] are all called on the
    /// main thread.
    ///
    /// ## Return value
    /// Returns true if this thread is the main thread, or false otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_RunOnMainThread`]
    pub fn SDL_IsMainThread() -> ::core::primitive::bool;
}

/// Callback run on the main thread.
///
/// ## Parameters
/// - `userdata`: an app-controlled pointer that is passed to the callback.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_RunOnMainThread`]
pub type SDL_MainThreadCallback =
    ::core::option::Option<unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void)>;

extern "C" {
    /// Call a function on the main thread during event processing.
    ///
    /// If this is called on the main thread, the callback is executed immediately.
    /// If this is called on another thread, this callback is queued for execution
    /// on the main thread during event processing.
    ///
    /// Be careful of deadlocks when using this functionality. You should not have
    /// the main thread wait for the current thread while this function is being
    /// called with `wait_complete` true.
    ///
    /// ## Parameters
    /// - `callback`: the callback to call on the main thread.
    /// - `userdata`: a pointer that is passed to `callback`.
    /// - `wait_complete`: true to wait for the callback to complete, false to
    ///   return immediately.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_IsMainThread`]
    pub fn SDL_RunOnMainThread(
        callback: SDL_MainThreadCallback,
        userdata: *mut ::core::ffi::c_void,
        wait_complete: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
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
    /// This function should be called as early as possible, before [`SDL_Init`].
    /// Multiple calls to this function are allowed, but various state might not
    /// change once it has been set up with a previous call to this function.
    ///
    /// Passing a NULL removes any previous metadata.
    ///
    /// This is a simplified interface for the most important information. You can
    /// supply significantly more detailed metadata with
    /// [`SDL_SetAppMetadataProperty()`].
    ///
    /// ## Parameters
    /// - `appname`: The name of the application ("My Game 2: Bad Guy's
    ///   Revenge!").
    /// - `appversion`: The version of the application ("1.0.0beta5" or a git
    ///   hash, or whatever makes sense).
    /// - `appidentifier`: A unique string in reverse-domain format that
    ///   identifies this app ("com.example.mygame2").
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetAppMetadataProperty`]
    pub fn SDL_SetAppMetadata(
        appname: *const ::core::ffi::c_char,
        appversion: *const ::core::ffi::c_char,
        appidentifier: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
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
    /// This function should be called as early as possible, before [`SDL_Init`].
    /// Multiple calls to this function are allowed, but various state might not
    /// change once it has been set up with a previous call to this function.
    ///
    /// Once set, this metadata can be read using [`SDL_GetAppMetadataProperty()`].
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_APP_METADATA_NAME_STRING`]\: The human-readable name of the
    ///   application, like "My Game 2: Bad Guy's Revenge!". This will show up
    ///   anywhere the OS shows the name of the application separately from window
    ///   titles, such as volume control applets, etc. This defaults to "SDL
    ///   Application".
    /// - [`SDL_PROP_APP_METADATA_VERSION_STRING`]\: The version of the app that is
    ///   running; there are no rules on format, so "1.0.3beta2" and "April 22nd,
    ///   2024" and a git hash are all valid options. This has no default.
    /// - [`SDL_PROP_APP_METADATA_IDENTIFIER_STRING`]\: A unique string that
    ///   identifies this app. This must be in reverse-domain format, like
    ///   "com.example.mygame2". This string is used by desktop compositors to
    ///   identify and group windows together, as well as match applications with
    ///   associated desktop settings and icons. If you plan to package your
    ///   application in a container such as Flatpak, the app ID should match the
    ///   name of your Flatpak container as well. This has no default.
    /// - [`SDL_PROP_APP_METADATA_CREATOR_STRING`]\: The human-readable name of the
    ///   creator/developer/maker of this app, like "MojoWorkshop, LLC"
    /// - [`SDL_PROP_APP_METADATA_COPYRIGHT_STRING`]\: The human-readable copyright
    ///   notice, like "Copyright (c) 2024 MojoWorkshop, LLC" or whatnot. Keep this
    ///   to one line, don't paste a copy of a whole software license in here. This
    ///   has no default.
    /// - [`SDL_PROP_APP_METADATA_URL_STRING`]\: A URL to the app on the web. Maybe a
    ///   product page, or a storefront, or even a GitHub repository, for user's
    ///   further information This has no default.
    /// - [`SDL_PROP_APP_METADATA_TYPE_STRING`]\: The type of application this is.
    ///   Currently this string can be "game" for a video game, "mediaplayer" for a
    ///   media player, or generically "application" if nothing else applies.
    ///   Future versions of SDL might add new types. This defaults to
    ///   "application".
    ///
    /// ## Parameters
    /// - `name`: the name of the metadata property to set.
    /// - `value`: the value of the property, or NULL to remove that property.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetAppMetadataProperty`]
    /// - [`SDL_SetAppMetadata`]
    pub fn SDL_SetAppMetadataProperty(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

pub const SDL_PROP_APP_METADATA_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.name".as_ptr();

pub const SDL_PROP_APP_METADATA_VERSION_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.version".as_ptr();

pub const SDL_PROP_APP_METADATA_IDENTIFIER_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.identifier".as_ptr();

pub const SDL_PROP_APP_METADATA_CREATOR_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.creator".as_ptr();

pub const SDL_PROP_APP_METADATA_COPYRIGHT_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.copyright".as_ptr();

pub const SDL_PROP_APP_METADATA_URL_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.url".as_ptr();

pub const SDL_PROP_APP_METADATA_TYPE_STRING: *const ::core::ffi::c_char =
    c"SDL.app.metadata.type".as_ptr();

extern "C" {
    /// Get metadata about your app.
    ///
    /// This returns metadata previously set using [`SDL_SetAppMetadata()`] or
    /// [`SDL_SetAppMetadataProperty()`]. See [`SDL_SetAppMetadataProperty()`] for the list
    /// of available properties and their meanings.
    ///
    /// ## Parameters
    /// - `name`: the name of the metadata property to get.
    ///
    /// ## Return value
    /// Returns the current value of the metadata property, or the default if it
    ///   is not set, NULL for properties with no default.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, although
    ///   the string returned is not protected and could potentially be
    ///   freed if you call [`SDL_SetAppMetadataProperty()`] to set that
    ///   property from another thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetAppMetadata`]
    /// - [`SDL_SetAppMetadataProperty`]
    pub fn SDL_GetAppMetadataProperty(
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}

#[cfg(doc)]
use crate::everything::*;
