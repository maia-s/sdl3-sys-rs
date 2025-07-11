//! This file contains functions to set and get configuration hints, as well as
//! listing each of them alphabetically.
//!
//! The convention for naming hints is SDL_HINT_X, where "SDL_X" is the
//! environment variable that can be used to override the default.
//!
//! In general these hints are just that - they may or may not be supported or
//! applicable on any given platform, but they provide a way for an application
//! or user to give the library a hint as to how they would like the library to
//! work.

use super::error::*;

use super::stdinc::*;

/// Specify the behavior of Alt+Tab while the keyboard is grabbed.
///
/// By default, SDL emulates Alt+Tab functionality while the keyboard is
/// grabbed and your window is full-screen. This prevents the user from getting
/// stuck in your application if you've enabled keyboard grab.
///
/// The variable can be set to the following values:
///
/// - "0": SDL will not handle Alt+Tab. Your application is responsible for
///   handling Alt+Tab while the keyboard is grabbed.
/// - "1": SDL will minimize your window when Alt+Tab is pressed (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED: *const ::core::ffi::c_char =
    c"SDL_ALLOW_ALT_TAB_WHILE_GRABBED".as_ptr();

/// A variable to control whether the SDL activity is allowed to be re-created.
///
/// If this hint is true, the activity can be recreated on demand by the OS,
/// and Java static data and C++ static data remain with their current values.
/// If this hint is false, then SDL will call exit() when you return from your
/// main function and the application will be terminated and then started fresh
/// each time.
///
/// The variable can be set to the following values:
///
/// - "0": The application starts fresh at each launch. (default)
/// - "1": The application activity can be recreated by the OS.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY: *const ::core::ffi::c_char =
    c"SDL_ANDROID_ALLOW_RECREATE_ACTIVITY".as_ptr();

/// A variable to control whether the event loop will block itself when the app
/// is paused.
///
/// The variable can be set to the following values:
///
/// - "0": Non blocking.
/// - "1": Blocking. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ANDROID_BLOCK_ON_PAUSE: *const ::core::ffi::c_char =
    c"SDL_ANDROID_BLOCK_ON_PAUSE".as_ptr();

/// A variable to control whether low latency audio should be enabled.
///
/// Some devices have poor quality output when this is enabled, but this is
/// usually an improvement in audio latency.
///
/// The variable can be set to the following values:
///
/// - "0": Low latency audio is not enabled.
/// - "1": Low latency audio is enabled. (default)
///
/// This hint should be set before SDL audio is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ANDROID_LOW_LATENCY_AUDIO: *const ::core::ffi::c_char =
    c"SDL_ANDROID_LOW_LATENCY_AUDIO".as_ptr();

/// A variable to control whether we trap the Android back button to handle it
/// manually.
///
/// This is necessary for the right mouse button to work on some Android
/// devices, or to be able to trap the back button for use in your code
/// reliably. If this hint is true, the back button will show up as an
/// [`SDL_EVENT_KEY_DOWN`] / [`SDL_EVENT_KEY_UP`] pair with a keycode of
/// [`SDL_SCANCODE_AC_BACK`].
///
/// The variable can be set to the following values:
///
/// - "0": Back button will be handled as usual for system. (default)
/// - "1": Back button will be trapped, allowing you to handle the key press
///   manually. (This will also let right mouse click work on systems where the
///   right mouse button functions as back.)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ANDROID_TRAP_BACK_BUTTON: *const ::core::ffi::c_char =
    c"SDL_ANDROID_TRAP_BACK_BUTTON".as_ptr();

/// A variable setting the app ID string.
///
/// This string is used by desktop compositors to identify and group windows
/// together, as well as match applications with associated desktop settings
/// and icons.
///
/// This will override [`SDL_PROP_APP_METADATA_IDENTIFIER_STRING`], if set by the
/// application.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_APP_ID: *const ::core::ffi::c_char = c"SDL_APP_ID".as_ptr();

/// A variable setting the application name.
///
/// This hint lets you specify the application name sent to the OS when
/// required. For example, this will often appear in volume control applets for
/// audio streams, and in lists of applications which are inhibiting the
/// screensaver. You should use a string that describes your program ("My Game
/// 2: The Revenge")
///
/// This will override [`SDL_PROP_APP_METADATA_NAME_STRING`], if set by the
/// application.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_APP_NAME: *const ::core::ffi::c_char = c"SDL_APP_NAME".as_ptr();

/// A variable controlling whether controllers used with the Apple TV generate
/// UI events.
///
/// When UI events are generated by controller input, the app will be
/// backgrounded when the Apple TV remote's menu button is pressed, and when
/// the pause or B buttons on gamepads are pressed.
///
/// More information about properly making use of controllers for the Apple TV
/// can be found here:
/// <https://developer.apple.com/tvos/human-interface-guidelines/remote-and-controllers/>
///
/// The variable can be set to the following values:
///
/// - "0": Controller input does not generate UI events. (default)
/// - "1": Controller input generates UI events.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS: *const ::core::ffi::c_char =
    c"SDL_APPLE_TV_CONTROLLER_UI_EVENTS".as_ptr();

/// A variable controlling whether the Apple TV remote's joystick axes will
/// automatically match the rotation of the remote.
///
/// The variable can be set to the following values:
///
/// - "0": Remote orientation does not affect joystick axes. (default)
/// - "1": Joystick axes are based on the orientation of the remote.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION: *const ::core::ffi::c_char =
    c"SDL_APPLE_TV_REMOTE_ALLOW_ROTATION".as_ptr();

/// Specify the default ALSA audio device name.
///
/// This variable is a specific audio device to open when the "default" audio
/// device is used.
///
/// This hint will be ignored when opening the default playback device if
/// [`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`] is set, or when opening the
/// default recording device if [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`] is
/// set.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`]
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`]
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_ALSA_DEFAULT_DEVICE".as_ptr();

/// Specify the default ALSA audio playback device name.
///
/// This variable is a specific audio device to open for playback, when the
/// "default" audio device is used.
///
/// If this hint isn't set, SDL will check [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]
/// before choosing a reasonable default.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`]
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE".as_ptr();

/// Specify the default ALSA audio recording device name.
///
/// This variable is a specific audio device to open for recording, when the
/// "default" audio device is used.
///
/// If this hint isn't set, SDL will check [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]
/// before choosing a reasonable default.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`]
/// - [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE".as_ptr();

/// A variable controlling the audio category on iOS and macOS.
///
/// The variable can be set to the following values:
///
/// - "ambient": Use the AVAudioSessionCategoryAmbient audio category, will be
///   muted by the phone mute switch (default)
/// - "playback": Use the AVAudioSessionCategoryPlayback category.
///
/// For more information, see Apple's documentation:
/// <https://developer.apple.com/library/content/documentation/Audio/Conceptual/AudioSessionProgrammingGuide/AudioSessionCategoriesandModes/AudioSessionCategoriesandModes.html>
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_CATEGORY: *const ::core::ffi::c_char = c"SDL_AUDIO_CATEGORY".as_ptr();

/// A variable controlling the default audio channel count.
///
/// If the application doesn't specify the audio channel count when opening the
/// device, this hint can be used to specify a default channel count that will
/// be used. This defaults to "1" for recording and "2" for playback devices.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_CHANNELS: *const ::core::ffi::c_char = c"SDL_AUDIO_CHANNELS".as_ptr();

/// Specify an application icon name for an audio device.
///
/// Some audio backends (such as Pulseaudio and Pipewire) allow you to set an
/// XDG icon name for your application. Among other things, this icon might
/// show up in a system control panel that lets the user adjust the volume on
/// specific audio streams instead of using one giant master volume slider.
/// Note that this is unrelated to the icon used by the windowing system, which
/// may be set with [`SDL_SetWindowIcon`] (or via desktop file on Wayland).
///
/// Setting this to "" or leaving it unset will have SDL use a reasonable
/// default, "applications-games", which is likely to be installed. See
/// <https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html>
/// and
/// <https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html>
/// for the relevant XDG icon specs.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DEVICE_APP_ICON_NAME".as_ptr();

/// A variable controlling device buffer size.
///
/// This hint is an integer > 0, that represents the size of the device's
/// buffer in sample frames (stereo audio data in 16-bit format is 4 bytes per
/// sample frame, for example).
///
/// SDL3 generally decides this value on behalf of the app, but if for some
/// reason the app needs to dictate this (because they want either lower
/// latency or higher throughput AND ARE WILLING TO DEAL WITH what that might
/// require of the app), they can specify it.
///
/// SDL will try to accommodate this value, but there is no promise you'll get
/// the buffer size requested. Many platforms won't honor this request at all,
/// or might adjust it.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DEVICE_SAMPLE_FRAMES".as_ptr();

/// Specify an audio stream name for an audio device.
///
/// Some audio backends (such as PulseAudio) allow you to describe your audio
/// stream. Among other things, this description might show up in a system
/// control panel that lets the user adjust the volume on specific audio
/// streams instead of using one giant master volume slider.
///
/// This hints lets you transmit that information to the OS. The contents of
/// this hint are used while opening an audio device. You should use a string
/// that describes your what your program is playing ("audio stream" is
/// probably sufficient in many cases, but this could be useful for something
/// like "team chat" if you have a headset playing VoIP audio separately).
///
/// Setting this to "" or leaving it unset will have SDL use a reasonable
/// default: "audio stream" or something similar.
///
/// Note that while this talks about audio streams, this is an OS-level
/// concept, so it applies to a physical audio device in this case, and not an
/// [`SDL_AudioStream`], nor an SDL logical audio device.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DEVICE_STREAM_NAME: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DEVICE_STREAM_NAME".as_ptr();

/// Specify an application role for an audio device.
///
/// Some audio backends (such as Pipewire) allow you to describe the role of
/// your audio stream. Among other things, this description might show up in a
/// system control panel or software for displaying and manipulating media
/// playback/recording graphs.
///
/// This hints lets you transmit that information to the OS. The contents of
/// this hint are used while opening an audio device. You should use a string
/// that describes your what your program is playing (Game, Music, Movie,
/// etc...).
///
/// Setting this to "" or leaving it unset will have SDL use a reasonable
/// default: "Game" or something similar.
///
/// Note that while this talks about audio streams, this is an OS-level
/// concept, so it applies to a physical audio device in this case, and not an
/// [`SDL_AudioStream`], nor an SDL logical audio device.
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DEVICE_STREAM_ROLE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DEVICE_STREAM_ROLE".as_ptr();

/// Specify the input file when recording audio using the disk audio driver.
///
/// This defaults to "sdlaudio-in.raw"
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DISK_INPUT_FILE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DISK_INPUT_FILE".as_ptr();

/// Specify the output file when playing audio using the disk audio driver.
///
/// This defaults to "sdlaudio.raw"
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DISK_OUTPUT_FILE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DISK_OUTPUT_FILE".as_ptr();

/// A variable controlling the audio rate when using the disk audio driver.
///
/// The disk audio driver normally simulates real-time for the audio rate that
/// was specified, but you can use this variable to adjust this rate higher or
/// lower down to 0. The default value is "1.0".
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DISK_TIMESCALE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DISK_TIMESCALE".as_ptr();

/// A variable that specifies an audio backend to use.
///
/// By default, SDL will try all available audio backends in a reasonable order
/// until it finds one that can work, but this hint allows the app or user to
/// force a specific driver, such as "pipewire" if, say, you are on PulseAudio
/// but want to try talking to the lower level instead.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DRIVER: *const ::core::ffi::c_char = c"SDL_AUDIO_DRIVER".as_ptr();

/// A variable controlling the audio rate when using the dummy audio driver.
///
/// The dummy audio driver normally simulates real-time for the audio rate that
/// was specified, but you can use this variable to adjust this rate higher or
/// lower down to 0. The default value is "1.0".
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_DUMMY_TIMESCALE: *const ::core::ffi::c_char =
    c"SDL_AUDIO_DUMMY_TIMESCALE".as_ptr();

/// A variable controlling the default audio format.
///
/// If the application doesn't specify the audio format when opening the
/// device, this hint can be used to specify a default format that will be
/// used.
///
/// The variable can be set to the following values:
///
/// - "U8": Unsigned 8-bit audio
/// - "S8": Signed 8-bit audio
/// - "S16LE": Signed 16-bit little-endian audio
/// - "S16BE": Signed 16-bit big-endian audio
/// - "S16": Signed 16-bit native-endian audio (default)
/// - "S32LE": Signed 32-bit little-endian audio
/// - "S32BE": Signed 32-bit big-endian audio
/// - "S32": Signed 32-bit native-endian audio
/// - "F32LE": Floating point little-endian audio
/// - "F32BE": Floating point big-endian audio
/// - "F32": Floating point native-endian audio
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_FORMAT: *const ::core::ffi::c_char = c"SDL_AUDIO_FORMAT".as_ptr();

/// A variable controlling the default audio frequency.
///
/// If the application doesn't specify the audio frequency when opening the
/// device, this hint can be used to specify a default frequency that will be
/// used. This defaults to "44100".
///
/// This hint should be set before an audio device is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_FREQUENCY: *const ::core::ffi::c_char = c"SDL_AUDIO_FREQUENCY".as_ptr();

/// A variable that causes SDL to not ignore audio "monitors".
///
/// This is currently only used by the PulseAudio driver.
///
/// By default, SDL ignores audio devices that aren't associated with physical
/// hardware. Changing this hint to "1" will expose anything SDL sees that
/// appears to be an audio source or sink. This will add "devices" to the list
/// that the user probably doesn't want or need, but it can be useful in
/// scenarios where you want to hook up SDL to some sort of virtual device,
/// etc.
///
/// The variable can be set to the following values:
///
/// - "0": Audio monitor devices will be ignored. (default)
/// - "1": Audio monitor devices will show up in the device list.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUDIO_INCLUDE_MONITORS: *const ::core::ffi::c_char =
    c"SDL_AUDIO_INCLUDE_MONITORS".as_ptr();

/// A variable controlling whether SDL updates joystick state when getting
/// input events.
///
/// The variable can be set to the following values:
///
/// - "0": You'll call [`SDL_UpdateJoysticks()`] manually.
/// - "1": SDL will automatically call [`SDL_UpdateJoysticks()`]. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUTO_UPDATE_JOYSTICKS: *const ::core::ffi::c_char =
    c"SDL_AUTO_UPDATE_JOYSTICKS".as_ptr();

/// A variable controlling whether SDL updates sensor state when getting input
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": You'll call [`SDL_UpdateSensors()`] manually.
/// - "1": SDL will automatically call [`SDL_UpdateSensors()`]. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_AUTO_UPDATE_SENSORS: *const ::core::ffi::c_char =
    c"SDL_AUTO_UPDATE_SENSORS".as_ptr();

/// Prevent SDL from using version 4 of the bitmap header when saving BMPs.
///
/// The bitmap header version 4 is required for proper alpha channel support
/// and SDL will use it when required. Should this not be desired, this hint
/// can force the use of the 40 byte header version which is supported
/// everywhere.
///
/// The variable can be set to the following values:
///
/// - "0": Surfaces with a colorkey or an alpha channel are saved to a 32-bit
///   BMP file with an alpha mask. SDL will use the bitmap header version 4 and
///   set the alpha mask accordingly. (default)
/// - "1": Surfaces with a colorkey or an alpha channel are saved to a 32-bit
///   BMP file without an alpha mask. The alpha channel data will be in the
///   file, but applications are going to ignore it.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_BMP_SAVE_LEGACY_FORMAT: *const ::core::ffi::c_char =
    c"SDL_BMP_SAVE_LEGACY_FORMAT".as_ptr();

/// A variable that decides what camera backend to use.
///
/// By default, SDL will try all available camera backends in a reasonable
/// order until it finds one that can work, but this hint allows the app or
/// user to force a specific target, such as "directshow" if, say, you are on
/// Windows Media Foundations but want to try DirectShow instead.
///
/// The default value is unset, in which case SDL will try to figure out the
/// best camera backend on your behalf. This hint needs to be set before
/// [`SDL_Init()`] is called to be useful.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_CAMERA_DRIVER: *const ::core::ffi::c_char = c"SDL_CAMERA_DRIVER".as_ptr();

/// A variable that limits what CPU features are available.
///
/// By default, SDL marks all features the current CPU supports as available.
/// This hint allows to limit these to a subset.
///
/// When the hint is unset, or empty, SDL will enable all detected CPU
/// features.
///
/// The variable can be set to a comma separated list containing the following
/// items:
///
/// - "all"
/// - "altivec"
/// - "sse"
/// - "sse2"
/// - "sse3"
/// - "sse41"
/// - "sse42"
/// - "avx"
/// - "avx2"
/// - "avx512f"
/// - "arm-simd"
/// - "neon"
/// - "lsx"
/// - "lasx"
///
/// The items can be prefixed by '+'/'-' to add/remove features.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_CPU_FEATURE_MASK: *const ::core::ffi::c_char = c"SDL_CPU_FEATURE_MASK".as_ptr();

/// A variable controlling whether DirectInput should be used for controllers.
///
/// The variable can be set to the following values:
///
/// - "0": Disable DirectInput detection.
/// - "1": Enable DirectInput detection. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_DIRECTINPUT: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_DIRECTINPUT".as_ptr();

/// A variable that specifies a dialog backend to use.
///
/// By default, SDL will try all available dialog backends in a reasonable
/// order until it finds one that can work, but this hint allows the app or
/// user to force a specific target.
///
/// If the specified target does not exist or is not available, the
/// dialog-related function calls will fail.
///
/// This hint currently only applies to platforms using the generic "Unix"
/// dialog implementation, but may be extended to more platforms in the future.
/// Note that some Unix and Unix-like platforms have their own implementation,
/// such as macOS and Haiku.
///
/// The variable can be set to the following values:
///
/// - NULL: Select automatically (default, all platforms)
/// - "portal": Use XDG Portals through DBus (Unix only)
/// - "zenity": Use the Zenity program (Unix only)
///
/// More options may be added in the future.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_FILE_DIALOG_DRIVER: *const ::core::ffi::c_char =
    c"SDL_FILE_DIALOG_DRIVER".as_ptr();

/// Override for [`SDL_GetDisplayUsableBounds()`].
///
/// If set, this hint will override the expected results for
/// [`SDL_GetDisplayUsableBounds()`] for display index 0. Generally you don't want
/// to do this, but this allows an embedded system to request that some of the
/// screen be reserved for other uses when paired with a well-behaved
/// application.
///
/// The contents of this hint must be 4 comma-separated integers, the first is
/// the bounds x, then y, width and height, in that order.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_DISPLAY_USABLE_BOUNDS: *const ::core::ffi::c_char =
    c"SDL_DISPLAY_USABLE_BOUNDS".as_ptr();

/// Disable giving back control to the browser automatically when running with
/// asyncify.
///
/// With -s ASYNCIFY, SDL calls emscripten_sleep during operations such as
/// refreshing the screen or polling events.
///
/// This hint only applies to the emscripten platform.
///
/// The variable can be set to the following values:
///
/// - "0": Disable emscripten_sleep calls (if you give back browser control
///   manually or use asyncify for other purposes).
/// - "1": Enable emscripten_sleep calls. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EMSCRIPTEN_ASYNCIFY: *const ::core::ffi::c_char =
    c"SDL_EMSCRIPTEN_ASYNCIFY".as_ptr();

/// Specify the CSS selector used for the "default" window/canvas.
///
/// This hint only applies to the emscripten platform.
///
/// The default value is "#canvas"
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR: *const ::core::ffi::c_char =
    c"SDL_EMSCRIPTEN_CANVAS_SELECTOR".as_ptr();

/// Override the binding element for keyboard inputs for Emscripten builds.
///
/// This hint only applies to the emscripten platform.
///
/// The variable can be one of:
///
/// - "#window": the javascript window object (default)
/// - "#document": the javascript document object
/// - "#screen": the javascript window.screen object
/// - "#canvas": the WebGL canvas element
/// - "#none": Don't bind anything at all
/// - any other string without a leading # sign applies to the element on the
///   page with that ID.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT: *const ::core::ffi::c_char =
    c"SDL_EMSCRIPTEN_KEYBOARD_ELEMENT".as_ptr();

/// A variable that controls whether the on-screen keyboard should be shown
/// when text input is active.
///
/// The variable can be set to the following values:
///
/// - "auto": The on-screen keyboard will be shown if there is no physical
///   keyboard attached. (default)
/// - "0": Do not show the on-screen keyboard.
/// - "1": Show the on-screen keyboard, if available.
///
/// This hint must be set before [`SDL_StartTextInput()`] is called
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ENABLE_SCREEN_KEYBOARD: *const ::core::ffi::c_char =
    c"SDL_ENABLE_SCREEN_KEYBOARD".as_ptr();

/// A variable containing a list of evdev devices to use if udev is not
/// available.
///
/// The list of devices is in the form:
///
/// deviceclass:path\[,deviceclass:path\[,...\]\]
///
/// where device class is an integer representing the SDL_UDEV_deviceclass and
/// path is the full path to the event device.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EVDEV_DEVICES: *const ::core::ffi::c_char = c"SDL_EVDEV_DEVICES".as_ptr();

/// A variable controlling verbosity of the logging of SDL events pushed onto
/// the internal queue.
///
/// The variable can be set to the following values, from least to most
/// verbose:
///
/// - "0": Don't log any events. (default)
/// - "1": Log most events (other than the really spammy ones).
/// - "2": Include mouse and finger motion events.
///
/// This is generally meant to be used to debug SDL itself, but can be useful
/// for application developers that need better visibility into what is going
/// on in the event queue. Logged events are sent through [`SDL_Log()`], which
/// means by default they appear on stdout on most platforms or maybe
/// OutputDebugString() on Windows, and can be funneled by the app with
/// [`SDL_SetLogOutputFunction()`], etc.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EVENT_LOGGING: *const ::core::ffi::c_char = c"SDL_EVENT_LOGGING".as_ptr();

/// A variable controlling whether raising the window should be done more
/// forcefully.
///
/// The variable can be set to the following values:
///
/// - "0": Honor the OS policy for raising windows. (default)
/// - "1": Force the window to be raised, overriding any OS policy.
///
/// At present, this is only an issue under MS Windows, which makes it nearly
/// impossible to programmatically move a window to the foreground, for
/// "security" reasons. See <http://stackoverflow.com/a/34414846> for a
/// discussion.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_FORCE_RAISEWINDOW: *const ::core::ffi::c_char =
    c"SDL_FORCE_RAISEWINDOW".as_ptr();

/// A variable controlling how 3D acceleration is used to accelerate the SDL
/// screen surface.
///
/// SDL can try to accelerate the SDL screen surface by using streaming
/// textures with a 3D rendering engine. This variable controls whether and how
/// this is done.
///
/// The variable can be set to the following values:
///
/// - "0": Disable 3D acceleration
/// - "1": Enable 3D acceleration, using the default renderer. (default)
/// - "X": Enable 3D acceleration, using X where X is one of the valid
///   rendering drivers. (e.g. "direct3d", "opengl", etc.)
///
/// This hint should be set before calling [`SDL_GetWindowSurface()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_FRAMEBUFFER_ACCELERATION: *const ::core::ffi::c_char =
    c"SDL_FRAMEBUFFER_ACCELERATION".as_ptr();

/// A variable that lets you manually hint extra gamecontroller db entries.
///
/// The variable should be newline delimited rows of gamecontroller config
/// data, see SDL_gamepad.h
///
/// You can update mappings after SDL is initialized with
/// [`SDL_GetGamepadMappingForGUID()`] and [`SDL_AddGamepadMapping()`]
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLERCONFIG: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLERCONFIG".as_ptr();

/// A variable that lets you provide a file with extra gamecontroller db
/// entries.
///
/// The file should contain lines of gamecontroller config data, see
/// SDL_gamepad.h
///
/// You can update mappings after SDL is initialized with
/// [`SDL_GetGamepadMappingForGUID()`] and [`SDL_AddGamepadMapping()`]
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLERCONFIG_FILE: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLERCONFIG_FILE".as_ptr();

/// A variable that overrides the automatic controller type detection.
///
/// The variable should be comma separated entries, in the form: VID/PID=type
///
/// The VID and PID should be hexadecimal with exactly 4 digits, e.g. 0x00fd
///
/// This hint affects what low level protocol is used with the HIDAPI driver.
///
/// The variable can be set to the following values:
///
/// - "Xbox360"
/// - "XboxOne"
/// - "PS3"
/// - "PS4"
/// - "PS5"
/// - "SwitchPro"
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLERTYPE: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLERTYPE".as_ptr();

/// A variable containing a list of devices to skip when scanning for game
/// controllers.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// 0xAAAA/0xBBBB,0xCCCC/0xDDDD
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLER_IGNORE_DEVICES".as_ptr();

/// If set, all devices will be skipped when scanning for game controllers
/// except for the ones listed in this variable.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// 0xAAAA/0xBBBB,0xCCCC/0xDDDD
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT".as_ptr();

/// A variable that controls whether the device's built-in accelerometer and
/// gyro should be used as sensors for gamepads.
///
/// The variable can be set to the following values:
///
/// - "0": Sensor fusion is disabled
/// - "1": Sensor fusion is enabled for all controllers that lack sensors
///
/// Or the variable can be a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// 0xAAAA/0xBBBB,0xCCCC/0xDDDD
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint should be set before a gamepad is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GAMECONTROLLER_SENSOR_FUSION: *const ::core::ffi::c_char =
    c"SDL_GAMECONTROLLER_SENSOR_FUSION".as_ptr();

/// This variable sets the default text of the TextInput window on GDK
/// platforms.
///
/// This hint is available only if SDL_GDK_TEXTINPUT defined.
///
/// This hint should be set before calling [`SDL_StartTextInput()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT: *const ::core::ffi::c_char =
    c"SDL_GDK_TEXTINPUT_DEFAULT_TEXT".as_ptr();

/// This variable sets the description of the TextInput window on GDK
/// platforms.
///
/// This hint is available only if SDL_GDK_TEXTINPUT defined.
///
/// This hint should be set before calling [`SDL_StartTextInput()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GDK_TEXTINPUT_DESCRIPTION: *const ::core::ffi::c_char =
    c"SDL_GDK_TEXTINPUT_DESCRIPTION".as_ptr();

/// This variable sets the maximum input length of the TextInput window on GDK
/// platforms.
///
/// The value must be a stringified integer, for example "10" to allow for up
/// to 10 characters of text input.
///
/// This hint is available only if SDL_GDK_TEXTINPUT defined.
///
/// This hint should be set before calling [`SDL_StartTextInput()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH: *const ::core::ffi::c_char =
    c"SDL_GDK_TEXTINPUT_MAX_LENGTH".as_ptr();

/// This variable sets the input scope of the TextInput window on GDK
/// platforms.
///
/// Set this hint to change the XGameUiTextEntryInputScope value that will be
/// passed to the window creation function. The value must be a stringified
/// integer, for example "0" for XGameUiTextEntryInputScope::Default.
///
/// This hint is available only if SDL_GDK_TEXTINPUT defined.
///
/// This hint should be set before calling [`SDL_StartTextInput()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GDK_TEXTINPUT_SCOPE: *const ::core::ffi::c_char =
    c"SDL_GDK_TEXTINPUT_SCOPE".as_ptr();

/// This variable sets the title of the TextInput window on GDK platforms.
///
/// This hint is available only if SDL_GDK_TEXTINPUT defined.
///
/// This hint should be set before calling [`SDL_StartTextInput()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GDK_TEXTINPUT_TITLE: *const ::core::ffi::c_char =
    c"SDL_GDK_TEXTINPUT_TITLE".as_ptr();

/// A variable to control whether HIDAPI uses libusb for device access.
///
/// By default libusb will only be used for a few devices that require direct
/// USB access, and this can be controlled with
/// [`SDL_HINT_HIDAPI_LIBUSB_WHITELIST`].
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI will not use libusb for device access.
/// - "1": HIDAPI will use libusb for device access if available. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_HIDAPI_LIBUSB: *const ::core::ffi::c_char = c"SDL_HIDAPI_LIBUSB".as_ptr();

/// A variable to control whether HIDAPI uses libusb only for whitelisted
/// devices.
///
/// By default libusb will only be used for a few devices that require direct
/// USB access.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI will use libusb for all device access.
/// - "1": HIDAPI will use libusb only for whitelisted devices. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_HIDAPI_LIBUSB_WHITELIST: *const ::core::ffi::c_char =
    c"SDL_HIDAPI_LIBUSB_WHITELIST".as_ptr();

/// A variable to control whether HIDAPI uses udev for device detection.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI will poll for device changes.
/// - "1": HIDAPI will use udev for device detection. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_HIDAPI_UDEV: *const ::core::ffi::c_char = c"SDL_HIDAPI_UDEV".as_ptr();

/// A variable that specifies a GPU backend to use.
///
/// By default, SDL will try all available GPU backends in a reasonable order
/// until it finds one that can work, but this hint allows the app or user to
/// force a specific target, such as "direct3d12" if, say, your hardware
/// supports Vulkan but you want to try using D3D12 instead.
///
/// This hint should be set before any GPU functions are called.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_GPU_DRIVER: *const ::core::ffi::c_char = c"SDL_GPU_DRIVER".as_ptr();

/// A variable to control whether [`SDL_hid_enumerate()`] enumerates all HID
/// devices or only controllers.
///
/// The variable can be set to the following values:
///
/// - "0": [`SDL_hid_enumerate()`] will enumerate all HID devices.
/// - "1": [`SDL_hid_enumerate()`] will only enumerate controllers. (default)
///
/// By default SDL will only enumerate controllers, to reduce risk of hanging
/// or crashing on devices with bad drivers and avoiding macOS keyboard capture
/// permission prompts.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS: *const ::core::ffi::c_char =
    c"SDL_HIDAPI_ENUMERATE_ONLY_CONTROLLERS".as_ptr();

/// A variable containing a list of devices to ignore in [`SDL_hid_enumerate()`].
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// For example, to ignore the Shanwan DS3 controller and any Valve controller,
/// you might use the string "0x2563/0x0523,0x28de/0x0000"
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_HIDAPI_IGNORE_DEVICES: *const ::core::ffi::c_char =
    c"SDL_HIDAPI_IGNORE_DEVICES".as_ptr();

/// A variable describing what IME UI elements the application can display.
///
/// By default IME UI is handled using native components by the OS where
/// possible, however this can interfere with or not be visible when exclusive
/// fullscreen mode is used.
///
/// The variable can be set to a comma separated list containing the following
/// items:
///
/// - "none" or "0": The application can't render any IME elements, and native
///   UI should be used. (default)
/// - "composition": The application handles [`SDL_EVENT_TEXT_EDITING`] events and
///   can render the composition text.
/// - "candidates": The application handles [`SDL_EVENT_TEXT_EDITING_CANDIDATES`]
///   and can render the candidate list.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_IME_IMPLEMENTED_UI: *const ::core::ffi::c_char =
    c"SDL_IME_IMPLEMENTED_UI".as_ptr();

/// A variable controlling whether the home indicator bar on iPhone X should be
/// hidden.
///
/// The variable can be set to the following values:
///
/// - "0": The indicator bar is not hidden. (default for windowed applications)
/// - "1": The indicator bar is hidden and is shown when the screen is touched
///   (useful for movie playback applications).
/// - "2": The indicator bar is dim and the first swipe makes it visible and
///   the second swipe performs the "home" action. (default for fullscreen
///   applications)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_IOS_HIDE_HOME_INDICATOR: *const ::core::ffi::c_char =
    c"SDL_IOS_HIDE_HOME_INDICATOR".as_ptr();

/// A variable that lets you enable joystick (and gamecontroller) events even
/// when your app is in the background.
///
/// The variable can be set to the following values:
///
/// - "0": Disable joystick & gamecontroller input events when the application
///   is in the background. (default)
/// - "1": Enable joystick & gamecontroller input events when the application
///   is in the background.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ALLOW_BACKGROUND_EVENTS".as_ptr();

/// A variable containing a list of arcade stick style controllers.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ARCADESTICK_DEVICES".as_ptr();

/// A variable containing a list of devices that are not arcade stick style
/// controllers.
///
/// This will override [`SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES`] and the built in
/// device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED".as_ptr();

/// A variable containing a list of devices that should not be considered
/// joysticks.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_BLACKLIST_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_BLACKLIST_DEVICES".as_ptr();

/// A variable containing a list of devices that should be considered
/// joysticks.
///
/// This will override [`SDL_HINT_JOYSTICK_BLACKLIST_DEVICES`] and the built in
/// device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED".as_ptr();

/// A variable containing a comma separated list of devices to open as
/// joysticks.
///
/// This variable is currently only used by the Linux joystick driver.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_DEVICE: *const ::core::ffi::c_char = c"SDL_JOYSTICK_DEVICE".as_ptr();

/// A variable controlling whether enhanced reports should be used for
/// controllers when using the HIDAPI driver.
///
/// Enhanced reports allow rumble and effects on Bluetooth PlayStation
/// controllers and gyro on Nintendo Switch controllers, but break Windows
/// DirectInput for other applications that don't use SDL.
///
/// Once enhanced reports are enabled, they can't be disabled on PlayStation
/// controllers without power cycling the controller.
///
/// The variable can be set to the following values:
///
/// - "0": enhanced reports are not enabled.
/// - "1": enhanced reports are enabled. (default)
/// - "auto": enhanced features are advertised to the application, but SDL
///   doesn't change the controller report mode unless the application uses
///   them.
///
/// This hint can be enabled anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ENHANCED_REPORTS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ENHANCED_REPORTS".as_ptr();

/// A variable containing a list of flightstick style controllers.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of @file, in which case the named file
/// will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_FLIGHTSTICK_DEVICES".as_ptr();

/// A variable containing a list of devices that are not flightstick style
/// controllers.
///
/// This will override [`SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES`] and the built in
/// device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED".as_ptr();

/// A variable controlling whether GameInput should be used for controller
/// handling on Windows.
///
/// The variable can be set to the following values:
///
/// - "0": GameInput is not used.
/// - "1": GameInput is used.
///
/// The default is "1" on GDK platforms, and "0" otherwise.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_GAMEINPUT: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_GAMEINPUT".as_ptr();

/// A variable containing a list of devices known to have a GameCube form
/// factor.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_GAMECUBE_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_GAMECUBE_DEVICES".as_ptr();

/// A variable containing a list of devices known not to have a GameCube form
/// factor.
///
/// This will override [`SDL_HINT_JOYSTICK_GAMECUBE_DEVICES`] and the built in
/// device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED".as_ptr();

/// A variable controlling whether the HIDAPI joystick drivers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI drivers are not used.
/// - "1": HIDAPI drivers are used. (default)
///
/// This variable is the default for all drivers, but can be overridden by the
/// hints for specific drivers below.
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI: *const ::core::ffi::c_char = c"SDL_JOYSTICK_HIDAPI".as_ptr();

/// A variable controlling whether Nintendo Switch Joy-Con controllers will be
/// combined into a single Pro-like controller when using the HIDAPI driver.
///
/// The variable can be set to the following values:
///
/// - "0": Left and right Joy-Con controllers will not be combined and each
///   will be a mini-gamepad.
/// - "1": Left and right Joy-Con controllers will be combined into a single
///   controller. (default)
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_COMBINE_JOY_CONS".as_ptr();

/// A variable controlling whether the HIDAPI driver for Nintendo GameCube
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_GAMECUBE".as_ptr();

/// A variable controlling whether rumble is used to implement the GameCube
/// controller's 3 rumble modes, Stop(0), Rumble(1), and StopHard(2).
///
/// This is useful for applications that need full compatibility for things
/// like ADSR envelopes. - Stop is implemented by setting low_frequency_rumble
/// to 0 and high_frequency_rumble >0 - Rumble is both at any arbitrary value -
/// StopHard is implemented by setting both low_frequency_rumble and
/// high_frequency_rumble to 0
///
/// The variable can be set to the following values:
///
/// - "0": Normal rumble behavior is behavior is used. (default)
/// - "1": Proper GameCube controller rumble behavior is used.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE".as_ptr();

/// A variable controlling whether the HIDAPI driver for Nintendo Switch
/// Joy-Cons should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_JOY_CONS".as_ptr();

/// A variable controlling whether the Home button LED should be turned on when
/// a Nintendo Switch Joy-Con controller is opened.
///
/// The variable can be set to the following values:
///
/// - "0": home button LED is turned off
/// - "1": home button LED is turned on
///
/// By default the Home button LED state is not changed. This hint can also be
/// set to a floating point value between 0.0 and 1.0 which controls the
/// brightness of the Home button LED.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_JOYCON_HOME_LED".as_ptr();

/// A variable controlling whether the HIDAPI driver for Amazon Luna
/// controllers connected via Bluetooth should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_LUNA: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_LUNA".as_ptr();

/// A variable controlling whether the HIDAPI driver for Nintendo Online
/// classic controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_NINTENDO_CLASSIC".as_ptr();

/// A variable controlling whether the HIDAPI driver for PS3 controllers should
/// be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`] on macOS, and "0" on
/// other platforms.
///
/// For official Sony driver (sixaxis.sys) use
/// [`SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER`]. See
/// <https://github.com/ViGEm/DsHidMini> for an alternative driver on Windows.
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS3: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS3".as_ptr();

/// A variable controlling whether the Sony driver (sixaxis.sys) for PS3
/// controllers (Sixaxis/DualShock 3) should be used.
///
/// The variable can be set to the following values:
///
/// - "0": Sony driver (sixaxis.sys) is not used.
/// - "1": Sony driver (sixaxis.sys) is used.
///
/// The default value is 0.
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER".as_ptr();

/// A variable controlling whether the HIDAPI driver for PS4 controllers should
/// be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS4: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS4".as_ptr();

/// A variable controlling the update rate of the PS4 controller over Bluetooth
/// when using the HIDAPI driver.
///
/// This defaults to 4 ms, to match the behavior over USB, and to be more
/// friendly to other Bluetooth devices and older Bluetooth hardware on the
/// computer. It can be set to "1" (1000Hz), "2" (500Hz) and "4" (250Hz)
///
/// This hint can be set anytime, but only takes effect when extended input
/// reports are enabled.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL".as_ptr();

/// A variable controlling whether the HIDAPI driver for PS5 controllers should
/// be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS5: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS5".as_ptr();

/// A variable controlling whether the player LEDs should be lit to indicate
/// which player is associated with a PS5 controller.
///
/// The variable can be set to the following values:
///
/// - "0": player LEDs are not enabled.
/// - "1": player LEDs are enabled. (default)
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_PS5_PLAYER_LED".as_ptr();

/// A variable controlling whether the HIDAPI driver for NVIDIA SHIELD
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_SHIELD: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_SHIELD".as_ptr();

/// A variable controlling whether the HIDAPI driver for Google Stadia
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_STADIA: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_STADIA".as_ptr();

/// A variable controlling whether the HIDAPI driver for Bluetooth Steam
/// Controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used. (default)
/// - "1": HIDAPI driver is used for Steam Controllers, which requires
///   Bluetooth access and may prompt the user for permission on iOS and
///   Android.
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_STEAM".as_ptr();

/// A variable controlling whether the Steam button LED should be turned on
/// when a Steam controller is opened.
///
/// The variable can be set to the following values:
///
/// - "0": Steam button LED is turned off.
/// - "1": Steam button LED is turned on.
///
/// By default the Steam button LED state is not changed. This hint can also be
/// set to a floating point value between 0.0 and 1.0 which controls the
/// brightness of the Steam button LED.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_STEAM_HOME_LED".as_ptr();

/// A variable controlling whether the HIDAPI driver for the Steam Deck builtin
/// controller should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_STEAMDECK".as_ptr();

/// A variable controlling whether the HIDAPI driver for HORI licensed Steam
/// controllers should be used.
///
/// This variable can be set to the following values: "0" - HIDAPI driver is
/// not used "1" - HIDAPI driver is used
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_STEAM_HORI".as_ptr();

/// A variable controlling whether the HIDAPI driver for Nintendo Switch
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_SWITCH".as_ptr();

/// A variable controlling whether the Home button LED should be turned on when
/// a Nintendo Switch Pro controller is opened.
///
/// The variable can be set to the following values:
///
/// - "0": Home button LED is turned off.
/// - "1": Home button LED is turned on.
///
/// By default the Home button LED state is not changed. This hint can also be
/// set to a floating point value between 0.0 and 1.0 which controls the
/// brightness of the Home button LED.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_SWITCH_HOME_LED".as_ptr();

/// A variable controlling whether the player LEDs should be lit to indicate
/// which player is associated with a Nintendo Switch controller.
///
/// The variable can be set to the following values:
///
/// - "0": Player LEDs are not enabled.
/// - "1": Player LEDs are enabled. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED".as_ptr();

/// A variable controlling whether Nintendo Switch Joy-Con controllers will be
/// in vertical mode when using the HIDAPI driver.
///
/// The variable can be set to the following values:
///
/// - "0": Left and right Joy-Con controllers will not be in vertical mode.
///   (default)
/// - "1": Left and right Joy-Con controllers will be in vertical mode.
///
/// This hint should be set before opening a Joy-Con controller.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS".as_ptr();

/// A variable controlling whether the HIDAPI driver for Nintendo Wii and Wii U
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// This driver doesn't work with the dolphinbar, so the default is false for
/// now.
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_WII: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_WII".as_ptr();

/// A variable controlling whether the player LEDs should be lit to indicate
/// which player is associated with a Wii controller.
///
/// The variable can be set to the following values:
///
/// - "0": Player LEDs are not enabled.
/// - "1": Player LEDs are enabled. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_WII_PLAYER_LED".as_ptr();

/// A variable controlling whether the HIDAPI driver for XBox controllers
/// should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is "0" on Windows, otherwise the value of
/// [`SDL_HINT_JOYSTICK_HIDAPI`]
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX".as_ptr();

/// A variable controlling whether the HIDAPI driver for XBox 360 controllers
/// should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX`]
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX_360".as_ptr();

/// A variable controlling whether the player LEDs should be lit to indicate
/// which player is associated with an Xbox 360 controller.
///
/// The variable can be set to the following values:
///
/// - "0": Player LEDs are not enabled.
/// - "1": Player LEDs are enabled. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED".as_ptr();

/// A variable controlling whether the HIDAPI driver for XBox 360 wireless
/// controllers should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX_360`]
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX_360_WIRELESS".as_ptr();

/// A variable controlling whether the HIDAPI driver for XBox One controllers
/// should be used.
///
/// The variable can be set to the following values:
///
/// - "0": HIDAPI driver is not used.
/// - "1": HIDAPI driver is used.
///
/// The default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX`].
///
/// This hint should be set before initializing joysticks and gamepads.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX_ONE".as_ptr();

/// A variable controlling whether the Home button LED should be turned on when
/// an Xbox One controller is opened.
///
/// The variable can be set to the following values:
///
/// - "0": Home button LED is turned off.
/// - "1": Home button LED is turned on.
///
/// By default the Home button LED state is not changed. This hint can also be
/// set to a floating point value between 0.0 and 1.0 which controls the
/// brightness of the Home button LED. The default brightness is 0.4.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED".as_ptr();

/// A variable controlling whether IOKit should be used for controller
/// handling.
///
/// The variable can be set to the following values:
///
/// - "0": IOKit is not used.
/// - "1": IOKit is used. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_IOKIT: *const ::core::ffi::c_char = c"SDL_JOYSTICK_IOKIT".as_ptr();

/// A variable controlling whether to use the classic /dev/input/js* joystick
/// interface or the newer /dev/input/event* joystick interface on Linux.
///
/// The variable can be set to the following values:
///
/// - "0": Use /dev/input/event* (default)
/// - "1": Use /dev/input/js*
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_LINUX_CLASSIC: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_LINUX_CLASSIC".as_ptr();

/// A variable controlling whether joysticks on Linux adhere to their
/// HID-defined deadzones or return unfiltered values.
///
/// The variable can be set to the following values:
///
/// - "0": Return unfiltered joystick axis values. (default)
/// - "1": Return axis values with deadzones taken into account.
///
/// This hint should be set before a controller is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_LINUX_DEADZONES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_LINUX_DEADZONES".as_ptr();

/// A variable controlling whether joysticks on Linux will always treat 'hat'
/// axis inputs (ABS_HAT0X - ABS_HAT3Y) as 8-way digital hats without checking
/// whether they may be analog.
///
/// The variable can be set to the following values:
///
/// - "0": Only map hat axis inputs to digital hat outputs if the input axes
///   appear to actually be digital. (default)
/// - "1": Always handle the input axes numbered ABS_HAT0X to ABS_HAT3Y as
///   digital hats.
///
/// This hint should be set before a controller is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_LINUX_DIGITAL_HATS".as_ptr();

/// A variable controlling whether digital hats on Linux will apply deadzones
/// to their underlying input axes or use unfiltered values.
///
/// The variable can be set to the following values:
///
/// - "0": Return digital hat values based on unfiltered input axis values.
/// - "1": Return digital hat values with deadzones on the input axes taken
///   into account. (default)
///
/// This hint should be set before a controller is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_LINUX_HAT_DEADZONES".as_ptr();

/// A variable controlling whether GCController should be used for controller
/// handling.
///
/// The variable can be set to the following values:
///
/// - "0": GCController is not used.
/// - "1": GCController is used. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_MFI: *const ::core::ffi::c_char = c"SDL_JOYSTICK_MFI".as_ptr();

/// A variable controlling whether the RAWINPUT joystick drivers should be used
/// for better handling XInput-capable devices.
///
/// The variable can be set to the following values:
///
/// - "0": RAWINPUT drivers are not used. (default)
/// - "1": RAWINPUT drivers are used.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_RAWINPUT: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_RAWINPUT".as_ptr();

/// A variable controlling whether the RAWINPUT driver should pull correlated
/// data from XInput.
///
/// The variable can be set to the following values:
///
/// - "0": RAWINPUT driver will only use data from raw input APIs.
/// - "1": RAWINPUT driver will also pull data from XInput and
///   Windows.Gaming.Input, providing better trigger axes, guide button
///   presses, and rumble support for Xbox controllers. (default)
///
/// This hint should be set before a gamepad is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_RAWINPUT_CORRELATE_XINPUT".as_ptr();

/// A variable controlling whether the ROG Chakram mice should show up as
/// joysticks.
///
/// The variable can be set to the following values:
///
/// - "0": ROG Chakram mice do not show up as joysticks. (default)
/// - "1": ROG Chakram mice show up as joysticks.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ROG_CHAKRAM: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ROG_CHAKRAM".as_ptr();

/// A variable controlling whether a separate thread should be used for
/// handling joystick detection and raw input messages on Windows.
///
/// The variable can be set to the following values:
///
/// - "0": A separate thread is not used.
/// - "1": A separate thread is used for handling raw input messages. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_THREAD: *const ::core::ffi::c_char = c"SDL_JOYSTICK_THREAD".as_ptr();

/// A variable containing a list of throttle style controllers.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_THROTTLE_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_THROTTLE_DEVICES".as_ptr();

/// A variable containing a list of devices that are not throttle style
/// controllers.
///
/// This will override [`SDL_HINT_JOYSTICK_THROTTLE_DEVICES`] and the built in
/// device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_THROTTLE_DEVICES_EXCLUDED".as_ptr();

/// A variable controlling whether Windows.Gaming.Input should be used for
/// controller handling.
///
/// The variable can be set to the following values:
///
/// - "0": WGI is not used.
/// - "1": WGI is used. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_WGI: *const ::core::ffi::c_char = c"SDL_JOYSTICK_WGI".as_ptr();

/// A variable containing a list of wheel style controllers.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_WHEEL_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_WHEEL_DEVICES".as_ptr();

/// A variable containing a list of devices that are not wheel style
/// controllers.
///
/// This will override [`SDL_HINT_JOYSTICK_WHEEL_DEVICES`] and the built in device
/// list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_WHEEL_DEVICES_EXCLUDED".as_ptr();

/// A variable containing a list of devices known to have all axes centered at
/// zero.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint should be set before a controller is opened.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_ZERO_CENTERED_DEVICES".as_ptr();

/// A variable containing a list of devices and their desired number of haptic
/// (force feedback) enabled axis.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form plus the number of desired axes, e.g.
///
/// `0xAAAA/0xBBBB/1,0xCCCC/0xDDDD/3`
///
/// This hint supports a "wildcard" device that will set the number of haptic
/// axes on all initialized haptic devices which were not defined explicitly in
/// this hint.
///
/// `0xFFFF/0xFFFF/1`
///
/// This hint should be set before a controller is opened. The number of haptic
/// axes won't exceed the number of real axes found on the device.
///
/// ## Availability
/// This hint is available since SDL 3.2.5.
pub const SDL_HINT_JOYSTICK_HAPTIC_AXES: *const ::core::ffi::c_char =
    c"SDL_JOYSTICK_HAPTIC_AXES".as_ptr();

/// A variable that controls keycode representation in keyboard events.
///
/// This variable is a comma separated set of options for translating keycodes
/// in events:
///
/// - "none": Keycode options are cleared, this overrides other options.
/// - "hide_numpad": The numpad keysyms will be translated into their
///   non-numpad versions based on the current NumLock state. For example,
///   SDLK_KP_4 would become SDLK_4 if [`SDL_KMOD_NUM`] is set in the event
///   modifiers, and SDLK_LEFT if it is unset.
/// - "french_numbers": The number row on French keyboards is inverted, so
///   pressing the 1 key would yield the keycode SDLK_1, or '1', instead of
///   SDLK_AMPERSAND, or '&'
/// - "latin_letters": For keyboards using non-Latin letters, such as Russian
///   or Thai, the letter keys generate keycodes as though it had an en_US
///   layout. e.g. pressing the key associated with [`SDL_SCANCODE_A`] on a Russian
///   keyboard would yield 'a' instead of a Cyrillic letter.
///
/// The default value for this hint is "french_numbers,latin_letters"
///
/// Some platforms like Emscripten only provide modified keycodes and the
/// options are not used.
///
/// These options do not affect the return value of [`SDL_GetKeyFromScancode()`] or
/// [`SDL_GetScancodeFromKey()`], they just apply to the keycode included in key
/// events.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_KEYCODE_OPTIONS: *const ::core::ffi::c_char = c"SDL_KEYCODE_OPTIONS".as_ptr();

/// A variable that controls what KMSDRM device to use.
///
/// SDL might open something like "/dev/dri/cardNN" to access KMSDRM
/// functionality, where "NN" is a device index number. SDL makes a guess at
/// the best index to use (usually zero), but the app or user can set this hint
/// to a number between 0 and 99 to force selection.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_KMSDRM_DEVICE_INDEX: *const ::core::ffi::c_char =
    c"SDL_KMSDRM_DEVICE_INDEX".as_ptr();

/// A variable that controls whether SDL requires DRM master access in order to
/// initialize the KMSDRM video backend.
///
/// The DRM subsystem has a concept of a "DRM master" which is a DRM client
/// that has the ability to set planes, set cursor, etc. When SDL is DRM
/// master, it can draw to the screen using the SDL rendering APIs. Without DRM
/// master, SDL is still able to process input and query attributes of attached
/// displays, but it cannot change display state or draw to the screen
/// directly.
///
/// In some cases, it can be useful to have the KMSDRM backend even if it
/// cannot be used for rendering. An app may want to use SDL for input
/// processing while using another rendering API (such as an MMAL overlay on
/// Raspberry Pi) or using its own code to render to DRM overlays that SDL
/// doesn't support.
///
/// The variable can be set to the following values:
///
/// - "0": SDL will allow usage of the KMSDRM backend without DRM master.
/// - "1": SDL Will require DRM master to use the KMSDRM backend. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER: *const ::core::ffi::c_char =
    c"SDL_KMSDRM_REQUIRE_DRM_MASTER".as_ptr();

/// A variable controlling the default SDL log levels.
///
/// This variable is a comma separated set of category=level tokens that define
/// the default logging levels for SDL applications.
///
/// The category can be a numeric category, one of "app", "error", "assert",
/// "system", "audio", "video", "render", "input", "test", or `*` for any
/// unspecified category.
///
/// The level can be a numeric level, one of "verbose", "debug", "info",
/// "warn", "error", "critical", or "quiet" to disable that category.
///
/// You can omit the category if you want to set the logging level for all
/// categories.
///
/// If this hint isn't set, the default log levels are equivalent to:
///
/// `app=info,assert=warn,test=verbose,*=error`
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_LOGGING: *const ::core::ffi::c_char = c"SDL_LOGGING".as_ptr();

/// A variable controlling whether to force the application to become the
/// foreground process when launched on macOS.
///
/// The variable can be set to the following values:
///
/// - "0": The application is brought to the foreground when launched.
///   (default)
/// - "1": The application may remain in the background when launched.
///
/// This hint needs to be set before [`SDL_Init()`].
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAC_BACKGROUND_APP: *const ::core::ffi::c_char =
    c"SDL_MAC_BACKGROUND_APP".as_ptr();

/// A variable that determines whether Ctrl+Click should generate a right-click
/// event on macOS.
///
/// The variable can be set to the following values:
///
/// - "0": Ctrl+Click does not generate a right mouse button click event.
///   (default)
/// - "1": Ctrl+Click generated a right mouse button click event.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK: *const ::core::ffi::c_char =
    c"SDL_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK".as_ptr();

/// A variable controlling whether dispatching OpenGL context updates should
/// block the dispatching thread until the main thread finishes processing on
/// macOS.
///
/// The variable can be set to the following values:
///
/// - "0": Dispatching OpenGL context updates will block the dispatching thread
///   until the main thread finishes processing. (default)
/// - "1": Dispatching OpenGL context updates will allow the dispatching thread
///   to continue execution.
///
/// Generally you want the default, but if you have OpenGL code in a background
/// thread on a Mac, and the main thread hangs because it's waiting for that
/// background thread, but that background thread is also hanging because it's
/// waiting for the main thread to do an update, this might fix your issue.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH: *const ::core::ffi::c_char =
    c"SDL_MAC_OPENGL_ASYNC_DISPATCH".as_ptr();

/// A variable controlling whether the Option key on macOS should be remapped
/// to act as the Alt key.
///
/// The variable can be set to the following values:
///
/// - "none": The Option key is not remapped to Alt. (default)
/// - "only_left": Only the left Option key is remapped to Alt.
/// - "only_right": Only the right Option key is remapped to Alt.
/// - "both": Both Option keys are remapped to Alt.
///
/// This will prevent the triggering of key compositions that rely on the
/// Option key, but will still send the Alt modifier for keyboard events. In
/// the case that both Alt and Option are pressed, the Option key will be
/// ignored. This is particularly useful for applications like terminal
/// emulators and graphical user interfaces (GUIs) that rely on Alt key
/// functionality for shortcuts or navigation. This does not apply to
/// [`SDL_GetKeyFromScancode`] and only has an effect if IME is enabled.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAC_OPTION_AS_ALT: *const ::core::ffi::c_char =
    c"SDL_MAC_OPTION_AS_ALT".as_ptr();

/// A variable controlling whether [`SDL_EVENT_MOUSE_WHEEL`] event values will have
/// momentum on macOS.
///
/// The variable can be set to the following values:
///
/// - "0": The mouse wheel events will have no momentum. (default)
/// - "1": The mouse wheel events will have momentum.
///
/// This hint needs to be set before [`SDL_Init()`].
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAC_SCROLL_MOMENTUM: *const ::core::ffi::c_char =
    c"SDL_MAC_SCROLL_MOMENTUM".as_ptr();

/// Request [`SDL_AppIterate()`] be called at a specific rate.
///
/// If this is set to a number, it represents Hz, so "60" means try to iterate
/// 60 times per second. "0" means to iterate as fast as possible. Negative
/// values are illegal, but reserved, in case they are useful in a future
/// revision of SDL.
///
/// There are other strings that have special meaning. If set to "waitevent",
/// [`SDL_AppIterate`] will not be called until new event(s) have arrived (and been
/// processed by [`SDL_AppEvent`]). This can be useful for apps that are completely
/// idle except in response to input.
///
/// On some platforms, or if you are using [`SDL_main`] instead of [`SDL_AppIterate`],
/// this hint is ignored. When the hint can be used, it is allowed to be
/// changed at any time.
///
/// This defaults to 0, and specifying NULL for the hint's value will restore
/// the default.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MAIN_CALLBACK_RATE: *const ::core::ffi::c_char =
    c"SDL_MAIN_CALLBACK_RATE".as_ptr();

/// A variable controlling whether the mouse is captured while mouse buttons
/// are pressed.
///
/// The variable can be set to the following values:
///
/// - "0": The mouse is not captured while mouse buttons are pressed.
/// - "1": The mouse is captured while mouse buttons are pressed.
///
/// By default the mouse is captured while mouse buttons are pressed so if the
/// mouse is dragged outside the window, the application continues to receive
/// mouse events until the button is released.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_AUTO_CAPTURE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_AUTO_CAPTURE".as_ptr();

/// A variable setting the double click radius, in pixels.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS: *const ::core::ffi::c_char =
    c"SDL_MOUSE_DOUBLE_CLICK_RADIUS".as_ptr();

/// A variable setting the double click time, in milliseconds.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_DOUBLE_CLICK_TIME: *const ::core::ffi::c_char =
    c"SDL_MOUSE_DOUBLE_CLICK_TIME".as_ptr();

/// A variable setting which system cursor to use as the default cursor.
///
/// This should be an integer corresponding to the [`SDL_SystemCursor`] enum. The
/// default value is zero ([`SDL_SYSTEM_CURSOR_DEFAULT`]).
///
/// This hint needs to be set before [`SDL_Init()`].
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR: *const ::core::ffi::c_char =
    c"SDL_MOUSE_DEFAULT_SYSTEM_CURSOR".as_ptr();

/// A variable controlling whether warping a hidden mouse cursor will activate
/// relative mouse mode.
///
/// When this hint is set, the mouse cursor is hidden, and multiple warps to
/// the window center occur within a short time period, SDL will emulate mouse
/// warps using relative mouse mode. This can provide smoother and more
/// reliable mouse motion for some older games, which continuously calculate
/// the distance travelled by the mouse pointer and warp it back to the center
/// of the window, rather than using relative mouse motion.
///
/// Note that relative mouse mode may have different mouse acceleration
/// behavior than pointer warps.
///
/// If your application needs to repeatedly warp the hidden mouse cursor at a
/// high-frequency for other purposes, it should disable this hint.
///
/// The variable can be set to the following values:
///
/// - "0": Attempts to warp the mouse will always be made.
/// - "1": Some mouse warps will be emulated by forcing relative mouse mode.
///   (default)
///
/// If not set, this is automatically enabled unless an application uses
/// relative mouse mode directly.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_EMULATE_WARP_WITH_RELATIVE".as_ptr();

/// Allow mouse click events when clicking to focus an SDL window.
///
/// The variable can be set to the following values:
///
/// - "0": Ignore mouse clicks that activate a window. (default)
/// - "1": Generate events for mouse clicks that activate a window.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH: *const ::core::ffi::c_char =
    c"SDL_MOUSE_FOCUS_CLICKTHROUGH".as_ptr();

/// A variable setting the speed scale for mouse motion, in floating point,
/// when the mouse is not in relative mode.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_NORMAL_SPEED_SCALE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_NORMAL_SPEED_SCALE".as_ptr();

/// A variable controlling whether relative mouse mode constrains the mouse to
/// the center of the window.
///
/// Constraining to the center of the window works better for FPS games and
/// when the application is running over RDP. Constraining to the whole window
/// works better for 2D games and increases the chance that the mouse will be
/// in the correct position when using high DPI mice.
///
/// The variable can be set to the following values:
///
/// - "0": Relative mouse mode constrains the mouse to the window.
/// - "1": Relative mouse mode constrains the mouse to the center of the
///   window. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_RELATIVE_MODE_CENTER: *const ::core::ffi::c_char =
    c"SDL_MOUSE_RELATIVE_MODE_CENTER".as_ptr();

/// A variable setting the scale for mouse motion, in floating point, when the
/// mouse is in relative mode.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_RELATIVE_SPEED_SCALE".as_ptr();

/// A variable controlling whether the system mouse acceleration curve is used
/// for relative mouse motion.
///
/// The variable can be set to the following values:
///
/// - "0": Relative mouse motion will be unscaled. (default)
/// - "1": Relative mouse motion will be scaled using the system mouse
///   acceleration curve.
///
/// If [`SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE`] is set, that will be applied after
/// system speed scale.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_RELATIVE_SYSTEM_SCALE".as_ptr();

/// A variable controlling whether a motion event should be generated for mouse
/// warping in relative mode.
///
/// The variable can be set to the following values:
///
/// - "0": Warping the mouse will not generate a motion event in relative mode
/// - "1": Warping the mouse will generate a motion event in relative mode
///
/// By default warping the mouse will not generate motion events in relative
/// mode. This avoids the application having to filter out large relative
/// motion due to warping.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_RELATIVE_WARP_MOTION: *const ::core::ffi::c_char =
    c"SDL_MOUSE_RELATIVE_WARP_MOTION".as_ptr();

/// A variable controlling whether the hardware cursor stays visible when
/// relative mode is active.
///
/// This variable can be set to the following values:
///
/// - "0": The cursor will be hidden while relative mode is active (default)
/// - "1": The cursor will remain visible while relative mode is active
///
/// Note that for systems without raw hardware inputs, relative mode is
/// implemented using warping, so the hardware cursor will visibly warp between
/// frames if this is enabled on those systems.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE: *const ::core::ffi::c_char =
    c"SDL_MOUSE_RELATIVE_CURSOR_VISIBLE".as_ptr();

/// A variable controlling whether mouse events should generate synthetic touch
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": Mouse events will not generate touch events. (default for desktop
///   platforms)
/// - "1": Mouse events will generate touch events. (default for mobile
///   platforms, such as Android and iOS)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MOUSE_TOUCH_EVENTS: *const ::core::ffi::c_char =
    c"SDL_MOUSE_TOUCH_EVENTS".as_ptr();

/// A variable controlling whether the keyboard should be muted on the console.
///
/// Normally the keyboard is muted while SDL applications are running so that
/// keyboard input doesn't show up as key strokes on the console. This hint
/// allows you to turn that off for debugging purposes.
///
/// The variable can be set to the following values:
///
/// - "0": Allow keystrokes to go through to the console.
/// - "1": Mute keyboard input so it doesn't show up on the console. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_MUTE_CONSOLE_KEYBOARD: *const ::core::ffi::c_char =
    c"SDL_MUTE_CONSOLE_KEYBOARD".as_ptr();

/// Tell SDL not to catch the SIGINT or SIGTERM signals on POSIX platforms.
///
/// The variable can be set to the following values:
///
/// - "0": SDL will install a SIGINT and SIGTERM handler, and when it catches a
///   signal, convert it into an [`SDL_EVENT_QUIT`] event. (default)
/// - "1": SDL will not install a signal handler at all.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_NO_SIGNAL_HANDLERS: *const ::core::ffi::c_char =
    c"SDL_NO_SIGNAL_HANDLERS".as_ptr();

/// Specify the OpenGL library to load.
///
/// This hint should be set before creating an OpenGL window or creating an
/// OpenGL context. If this hint isn't set, SDL will choose a reasonable
/// default.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_OPENGL_LIBRARY: *const ::core::ffi::c_char = c"SDL_OPENGL_LIBRARY".as_ptr();

/// Specify the EGL library to load.
///
/// This hint should be set before creating an OpenGL window or creating an
/// OpenGL context. This hint is only considered if SDL is using EGL to manage
/// OpenGL contexts. If this hint isn't set, SDL will choose a reasonable
/// default.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_EGL_LIBRARY: *const ::core::ffi::c_char = c"SDL_EGL_LIBRARY".as_ptr();

/// A variable controlling what driver to use for OpenGL ES contexts.
///
/// On some platforms, currently Windows and X11, OpenGL drivers may support
/// creating contexts with an OpenGL ES profile. By default SDL uses these
/// profiles, when available, otherwise it attempts to load an OpenGL ES
/// library, e.g. that provided by the ANGLE project. This variable controls
/// whether SDL follows this default behaviour or will always load an OpenGL ES
/// library.
///
/// Circumstances where this is useful include - Testing an app with a
/// particular OpenGL ES implementation, e.g ANGLE, or emulator, e.g. those
/// from ARM, Imagination or Qualcomm. - Resolving OpenGL ES function addresses
/// at link time by linking with the OpenGL ES library instead of querying them
/// at run time with [`SDL_GL_GetProcAddress()`].
///
/// Caution: for an application to work with the default behaviour across
/// different OpenGL drivers it must query the OpenGL ES function addresses at
/// run time using [`SDL_GL_GetProcAddress()`].
///
/// This variable is ignored on most platforms because OpenGL ES is native or
/// not supported.
///
/// The variable can be set to the following values:
///
/// - "0": Use ES profile of OpenGL, if available. (default)
/// - "1": Load OpenGL ES library using the default library names.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_OPENGL_ES_DRIVER: *const ::core::ffi::c_char = c"SDL_OPENGL_ES_DRIVER".as_ptr();

/// Mechanism to specify openvr_api library location
///
/// By default, when using the OpenVR driver, it will search for the API
/// library in the current folder. But, if you wish to use a system API you can
/// specify that by using this hint. This should be the full or relative path
/// to a .dll on Windows or .so on Linux.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_OPENVR_LIBRARY: *const ::core::ffi::c_char = c"SDL_OPENVR_LIBRARY".as_ptr();

/// A variable controlling which orientations are allowed on iOS/Android.
///
/// In some circumstances it is necessary to be able to explicitly control
/// which UI orientations are allowed.
///
/// This variable is a space delimited list of the following values:
///
/// - "LandscapeLeft"
/// - "LandscapeRight"
/// - "Portrait"
/// - "PortraitUpsideDown"
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ORIENTATIONS: *const ::core::ffi::c_char = c"SDL_ORIENTATIONS".as_ptr();

/// A variable controlling the use of a sentinel event when polling the event
/// queue.
///
/// When polling for events, [`SDL_PumpEvents`] is used to gather new events from
/// devices. If a device keeps producing new events between calls to
/// [`SDL_PumpEvents`], a poll loop will become stuck until the new events stop.
/// This is most noticeable when moving a high frequency mouse.
///
/// The variable can be set to the following values:
///
/// - "0": Disable poll sentinels.
/// - "1": Enable poll sentinels. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_POLL_SENTINEL: *const ::core::ffi::c_char = c"SDL_POLL_SENTINEL".as_ptr();

/// Override for [`SDL_GetPreferredLocales()`].
///
/// If set, this will be favored over anything the OS might report for the
/// user's preferred locales. Changing this hint at runtime will not generate a
/// [`SDL_EVENT_LOCALE_CHANGED`] event (but if you can change the hint, you can
/// push your own event, if you want).
///
/// The format of this hint is a comma-separated list of language and locale,
/// combined with an underscore, as is a common format: "en_GB". Locale is
/// optional: "en". So you might have a list like this: "en_GB,jp,es_PT"
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_PREFERRED_LOCALES: *const ::core::ffi::c_char =
    c"SDL_PREFERRED_LOCALES".as_ptr();

/// A variable that decides whether to send [`SDL_EVENT_QUIT`] when closing the
/// last window.
///
/// The variable can be set to the following values:
///
/// - "0": SDL will not send an [`SDL_EVENT_QUIT`] event when the last window is
///   requesting to close. Note that in this case, there are still other
///   legitimate reasons one might get an [`SDL_EVENT_QUIT`] event: choosing "Quit"
///   from the macOS menu bar, sending a SIGINT (ctrl-c) on Unix, etc.
/// - "1": SDL will send a quit event when the last window is requesting to
///   close. (default)
///
/// If there is at least one active system tray icon, [`SDL_EVENT_QUIT`] will
/// instead be sent when both the last window will be closed and the last tray
/// icon will be destroyed.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE: *const ::core::ffi::c_char =
    c"SDL_QUIT_ON_LAST_WINDOW_CLOSE".as_ptr();

/// A variable controlling whether the Direct3D device is initialized for
/// thread-safe operations.
///
/// The variable can be set to the following values:
///
/// - "0": Thread-safety is not enabled. (default)
/// - "1": Thread-safety is enabled.
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_DIRECT3D_THREADSAFE: *const ::core::ffi::c_char =
    c"SDL_RENDER_DIRECT3D_THREADSAFE".as_ptr();

/// A variable controlling whether to enable Direct3D 11+'s Debug Layer.
///
/// This variable does not have any effect on the Direct3D 9 based renderer.
///
/// The variable can be set to the following values:
///
/// - "0": Disable Debug Layer use. (default)
/// - "1": Enable Debug Layer use.
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_DIRECT3D11_DEBUG: *const ::core::ffi::c_char =
    c"SDL_RENDER_DIRECT3D11_DEBUG".as_ptr();

/// A variable controlling whether to enable Vulkan Validation Layers.
///
/// This variable can be set to the following values:
///
/// - "0": Disable Validation Layer use
/// - "1": Enable Validation Layer use
///
/// By default, SDL does not use Vulkan Validation Layers.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_VULKAN_DEBUG: *const ::core::ffi::c_char =
    c"SDL_RENDER_VULKAN_DEBUG".as_ptr();

/// A variable controlling whether to create the GPU device in debug mode.
///
/// This variable can be set to the following values:
///
/// - "0": Disable debug mode use (default)
/// - "1": Enable debug mode use
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_GPU_DEBUG: *const ::core::ffi::c_char = c"SDL_RENDER_GPU_DEBUG".as_ptr();

/// A variable controlling whether to prefer a low-power GPU on multi-GPU
/// systems.
///
/// This variable can be set to the following values:
///
/// - "0": Prefer high-performance GPU (default)
/// - "1": Prefer low-power GPU
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_GPU_LOW_POWER: *const ::core::ffi::c_char =
    c"SDL_RENDER_GPU_LOW_POWER".as_ptr();

/// A variable specifying which render driver to use.
///
/// If the application doesn't pick a specific renderer to use, this variable
/// specifies the name of the preferred renderer. If the preferred renderer
/// can't be initialized, creating a renderer will fail.
///
/// This variable is case insensitive and can be set to the following values:
///
/// - "direct3d"
/// - "direct3d11"
/// - "direct3d12"
/// - "opengl"
/// - "opengles2"
/// - "opengles"
/// - "metal"
/// - "vulkan"
/// - "gpu"
/// - "software"
///
/// This hint accepts a comma-separated list of driver names, and each will be
/// tried in the order listed when creating a renderer until one succeeds or
/// all of them fail.
///
/// The default varies by platform, but it's the first one in the list that is
/// available on the current platform.
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_DRIVER: *const ::core::ffi::c_char = c"SDL_RENDER_DRIVER".as_ptr();

/// A variable controlling how the 2D render API renders lines.
///
/// The variable can be set to the following values:
///
/// - "0": Use the default line drawing method (Bresenham's line algorithm)
/// - "1": Use the driver point API using Bresenham's line algorithm (correct,
///   draws many points)
/// - "2": Use the driver line API (occasionally misses line endpoints based on
///   hardware driver quirks
/// - "3": Use the driver geometry API (correct, draws thicker diagonal lines)
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_LINE_METHOD: *const ::core::ffi::c_char =
    c"SDL_RENDER_LINE_METHOD".as_ptr();

/// A variable controlling whether the Metal render driver select low power
/// device over default one.
///
/// The variable can be set to the following values:
///
/// - "0": Use the preferred OS device. (default)
/// - "1": Select a low power device.
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE: *const ::core::ffi::c_char =
    c"SDL_RENDER_METAL_PREFER_LOW_POWER_DEVICE".as_ptr();

/// A variable controlling whether updates to the SDL screen surface should be
/// synchronized with the vertical refresh, to avoid tearing.
///
/// This hint overrides the application preference when creating a renderer.
///
/// The variable can be set to the following values:
///
/// - "0": Disable vsync. (default)
/// - "1": Enable vsync.
///
/// This hint should be set before creating a renderer.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RENDER_VSYNC: *const ::core::ffi::c_char = c"SDL_RENDER_VSYNC".as_ptr();

/// A variable to control whether the return key on the soft keyboard should
/// hide the soft keyboard on Android and iOS.
///
/// This hint sets the default value of [`SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN`].
///
/// The variable can be set to the following values:
///
/// - "0": The return key will be handled as a key event. (default)
/// - "1": The return key will hide the keyboard.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RETURN_KEY_HIDES_IME: *const ::core::ffi::c_char =
    c"SDL_RETURN_KEY_HIDES_IME".as_ptr();

/// A variable containing a list of ROG gamepad capable mice.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED`]
pub const SDL_HINT_ROG_GAMEPAD_MICE: *const ::core::ffi::c_char = c"SDL_ROG_GAMEPAD_MICE".as_ptr();

/// A variable containing a list of devices that are not ROG gamepad capable
/// mice.
///
/// This will override [`SDL_HINT_ROG_GAMEPAD_MICE`] and the built in device list.
///
/// The format of the string is a comma separated list of USB VID/PID pairs in
/// hexadecimal form, e.g.
///
/// `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
///
/// The variable can also take the form of "@file", in which case the named
/// file will be loaded and interpreted as the value of the variable.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED: *const ::core::ffi::c_char =
    c"SDL_ROG_GAMEPAD_MICE_EXCLUDED".as_ptr();

/// A variable controlling which Dispmanx layer to use on a Raspberry PI.
///
/// Also known as Z-order. The variable can take a negative or positive value.
/// The default is 10000.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_RPI_VIDEO_LAYER: *const ::core::ffi::c_char = c"SDL_RPI_VIDEO_LAYER".as_ptr();

/// Specify an "activity name" for screensaver inhibition.
///
/// Some platforms, notably Linux desktops, list the applications which are
/// inhibiting the screensaver or other power-saving features.
///
/// This hint lets you specify the "activity name" sent to the OS when
/// [`SDL_DisableScreenSaver()`] is used (or the screensaver is automatically
/// disabled). The contents of this hint are used when the screensaver is
/// disabled. You should use a string that describes what your program is doing
/// (and, therefore, why the screensaver is disabled). For example, "Playing a
/// game" or "Watching a video".
///
/// Setting this to "" or leaving it unset will have SDL use a reasonable
/// default: "Playing a game" or something similar.
///
/// This hint should be set before calling [`SDL_DisableScreenSaver()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME: *const ::core::ffi::c_char =
    c"SDL_SCREENSAVER_INHIBIT_ACTIVITY_NAME".as_ptr();

/// A variable controlling whether SDL calls dbus_shutdown() on quit.
///
/// This is useful as a debug tool to validate memory leaks, but shouldn't ever
/// be set in production applications, as other libraries used by the
/// application might use dbus under the hood and this can cause crashes if
/// they continue after [`SDL_Quit()`].
///
/// The variable can be set to the following values:
///
/// - "0": SDL will not call dbus_shutdown() on quit. (default)
/// - "1": SDL will call dbus_shutdown() on quit.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_SHUTDOWN_DBUS_ON_QUIT: *const ::core::ffi::c_char =
    c"SDL_SHUTDOWN_DBUS_ON_QUIT".as_ptr();

/// A variable that specifies a backend to use for title storage.
///
/// By default, SDL will try all available storage backends in a reasonable
/// order until it finds one that can work, but this hint allows the app or
/// user to force a specific target, such as "pc" if, say, you are on Steam but
/// want to avoid SteamRemoteStorage for title data.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_STORAGE_TITLE_DRIVER: *const ::core::ffi::c_char =
    c"SDL_STORAGE_TITLE_DRIVER".as_ptr();

/// A variable that specifies a backend to use for user storage.
///
/// By default, SDL will try all available storage backends in a reasonable
/// order until it finds one that can work, but this hint allows the app or
/// user to force a specific target, such as "pc" if, say, you are on Steam but
/// want to avoid SteamRemoteStorage for user data.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_STORAGE_USER_DRIVER: *const ::core::ffi::c_char =
    c"SDL_STORAGE_USER_DRIVER".as_ptr();

/// Specifies whether [`SDL_THREAD_PRIORITY_TIME_CRITICAL`] should be treated as
/// realtime.
///
/// On some platforms, like Linux, a realtime priority thread may be subject to
/// restrictions that require special handling by the application. This hint
/// exists to let SDL know that the app is prepared to handle said
/// restrictions.
///
/// On Linux, SDL will apply the following configuration to any thread that
/// becomes realtime:
///
/// - The SCHED_RESET_ON_FORK bit will be set on the scheduling policy,
/// - An RLIMIT_RTTIME budget will be configured to the rtkit specified limit.
/// - Exceeding this limit will result in the kernel sending SIGKILL to the
///   app, refer to the man pages for more information.
///
/// The variable can be set to the following values:
///
/// - "0": default platform specific behaviour
/// - "1": Force [`SDL_THREAD_PRIORITY_TIME_CRITICAL`] to a realtime scheduling
///   policy
///
/// This hint should be set before calling [`SDL_SetCurrentThreadPriority()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL: *const ::core::ffi::c_char =
    c"SDL_THREAD_FORCE_REALTIME_TIME_CRITICAL".as_ptr();

/// A string specifying additional information to use with
/// [`SDL_SetCurrentThreadPriority`].
///
/// By default [`SDL_SetCurrentThreadPriority`] will make appropriate system
/// changes in order to apply a thread priority. For example on systems using
/// pthreads the scheduler policy is changed automatically to a policy that
/// works well with a given priority. Code which has specific requirements can
/// override SDL's default behavior with this hint.
///
/// pthread hint values are "current", "other", "fifo" and "rr". Currently no
/// other platform hint values are defined but may be in the future.
///
/// On Linux, the kernel may send SIGKILL to realtime tasks which exceed the
/// distro configured execution budget for rtkit. This budget can be queried
/// through RLIMIT_RTTIME after calling [`SDL_SetCurrentThreadPriority()`].
///
/// This hint should be set before calling [`SDL_SetCurrentThreadPriority()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_THREAD_PRIORITY_POLICY: *const ::core::ffi::c_char =
    c"SDL_THREAD_PRIORITY_POLICY".as_ptr();

/// A variable that controls the timer resolution, in milliseconds.
///
/// The higher resolution the timer, the more frequently the CPU services timer
/// interrupts, and the more precise delays are, but this takes up power and
/// CPU time. This hint is only used on Windows.
///
/// See this blog post for more information:
/// <http://randomascii.wordpress.com/2013/07/08/windows-timer-resolution-megawatts-wasted/>
///
/// The default value is "1".
///
/// If this variable is set to "0", the system timer resolution is not set.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_TIMER_RESOLUTION: *const ::core::ffi::c_char = c"SDL_TIMER_RESOLUTION".as_ptr();

/// A variable controlling whether touch events should generate synthetic mouse
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": Touch events will not generate mouse events.
/// - "1": Touch events will generate mouse events. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_TOUCH_MOUSE_EVENTS: *const ::core::ffi::c_char =
    c"SDL_TOUCH_MOUSE_EVENTS".as_ptr();

/// A variable controlling whether trackpads should be treated as touch
/// devices.
///
/// On macOS (and possibly other platforms in the future), SDL will report
/// touches on a trackpad as mouse input, which is generally what users expect
/// from this device; however, these are often actually full multitouch-capable
/// touch devices, so it might be preferable to some apps to treat them as
/// such.
///
/// The variable can be set to the following values:
///
/// - "0": Trackpad will send mouse events. (default)
/// - "1": Trackpad will send touch events.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_TRACKPAD_IS_TOUCH_ONLY: *const ::core::ffi::c_char =
    c"SDL_TRACKPAD_IS_TOUCH_ONLY".as_ptr();

/// A variable controlling whether the Android / tvOS remotes should be listed
/// as joystick devices, instead of sending keyboard events.
///
/// The variable can be set to the following values:
///
/// - "0": Remotes send enter/escape/arrow key events.
/// - "1": Remotes are available as 2 axis, 2 button joysticks. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_TV_REMOTE_AS_JOYSTICK: *const ::core::ffi::c_char =
    c"SDL_TV_REMOTE_AS_JOYSTICK".as_ptr();

/// A variable controlling whether the screensaver is enabled.
///
/// The variable can be set to the following values:
///
/// - "0": Disable screensaver. (default)
/// - "1": Enable screensaver.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_ALLOW_SCREENSAVER: *const ::core::ffi::c_char =
    c"SDL_VIDEO_ALLOW_SCREENSAVER".as_ptr();

/// A comma separated list containing the names of the displays that SDL should
/// sort to the front of the display list.
///
/// When this hint is set, displays with matching name strings will be
/// prioritized in the list of displays, as exposed by calling
/// [`SDL_GetDisplays()`], with the first listed becoming the primary display. The
/// naming convention can vary depending on the environment, but it is usually
/// a connector name (e.g. 'DP-1', 'DP-2', 'HDMI-A-1',etc...).
///
/// On Wayland and X11 desktops, the connector names associated with displays
/// can typically be found by using the `xrandr` utility.
///
/// This hint is currently supported on the following drivers:
///
/// - KMSDRM (kmsdrm)
/// - Wayland (wayland)
/// - X11 (x11)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_DISPLAY_PRIORITY: *const ::core::ffi::c_char =
    c"SDL_VIDEO_DISPLAY_PRIORITY".as_ptr();

/// Tell the video driver that we only want a double buffer.
///
/// By default, most lowlevel 2D APIs will use a triple buffer scheme that
/// wastes no CPU time on waiting for vsync after issuing a flip, but
/// introduces a frame of latency. On the other hand, using a double buffer
/// scheme instead is recommended for cases where low latency is an important
/// factor because we save a whole frame of latency.
///
/// We do so by waiting for vsync immediately after issuing a flip, usually
/// just after eglSwapBuffers call in the backend's *_SwapWindow function.
///
/// This hint is currently supported on the following drivers:
///
/// - Raspberry Pi (raspberrypi)
/// - Wayland (wayland)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_DOUBLE_BUFFER: *const ::core::ffi::c_char =
    c"SDL_VIDEO_DOUBLE_BUFFER".as_ptr();

/// A variable that specifies a video backend to use.
///
/// By default, SDL will try all available video backends in a reasonable order
/// until it finds one that can work, but this hint allows the app or user to
/// force a specific target, such as "x11" if, say, you are on Wayland but want
/// to try talking to the X server instead.
///
/// This hint accepts a comma-separated list of driver names, and each will be
/// tried in the order listed during init, until one succeeds or all of them
/// fail.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_DRIVER: *const ::core::ffi::c_char = c"SDL_VIDEO_DRIVER".as_ptr();

/// A variable controlling whether the dummy video driver saves output frames.
///
/// - "0": Video frames are not saved to disk. (default)
/// - "1": Video frames are saved to files in the format "SDL_windowX-Y.bmp",
///   where X is the window ID, and Y is the frame number.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES: *const ::core::ffi::c_char =
    c"SDL_VIDEO_DUMMY_SAVE_FRAMES".as_ptr();

/// If eglGetPlatformDisplay fails, fall back to calling eglGetDisplay.
///
/// The variable can be set to one of the following values:
///
/// - "0": Do not fall back to eglGetDisplay.
/// - "1": Fall back to eglGetDisplay if eglGetPlatformDisplay fails. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK: *const ::core::ffi::c_char =
    c"SDL_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK".as_ptr();

/// A variable controlling whether the OpenGL context should be created with
/// EGL.
///
/// The variable can be set to the following values:
///
/// - "0": Use platform-specific GL context creation API (GLX, WGL, CGL, etc).
///   (default)
/// - "1": Use EGL
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_FORCE_EGL: *const ::core::ffi::c_char = c"SDL_VIDEO_FORCE_EGL".as_ptr();

/// A variable that specifies the policy for fullscreen Spaces on macOS.
///
/// The variable can be set to the following values:
///
/// - "0": Disable Spaces support (FULLSCREEN_DESKTOP won't use them and
///   [`SDL_WINDOW_RESIZABLE`] windows won't offer the "fullscreen" button on their
///   titlebars).
/// - "1": Enable Spaces support (FULLSCREEN_DESKTOP will use them and
///   [`SDL_WINDOW_RESIZABLE`] windows will offer the "fullscreen" button on their
///   titlebars). (default)
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES: *const ::core::ffi::c_char =
    c"SDL_VIDEO_MAC_FULLSCREEN_SPACES".as_ptr();

/// A variable that specifies the menu visibility when a window is fullscreen
/// in Spaces on macOS.
///
/// The variable can be set to the following values:
///
/// - "0": The menu will be hidden when the window is in a fullscreen space,
///   and not accessible by moving the mouse to the top of the screen.
/// - "1": The menu will be accessible when the window is in a fullscreen
///   space.
/// - "auto": The menu will be hidden if fullscreen mode was toggled on
///   programmatically via `SDL_SetWindowFullscreen()`, and accessible if
///   fullscreen was entered via the "fullscreen" button on the window title
///   bar. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY: *const ::core::ffi::c_char =
    c"SDL_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY".as_ptr();

/// A variable controlling whether fullscreen windows are minimized when they
/// lose focus.
///
/// The variable can be set to the following values:
///
/// - "0": Fullscreen windows will not be minimized when they lose focus.
///   (default)
/// - "1": Fullscreen windows are minimized when they lose focus.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS: *const ::core::ffi::c_char =
    c"SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS".as_ptr();

/// A variable controlling whether the offscreen video driver saves output
/// frames.
///
/// This only saves frames that are generated using software rendering, not
/// accelerated OpenGL rendering.
///
/// - "0": Video frames are not saved to disk. (default)
/// - "1": Video frames are saved to files in the format "SDL_windowX-Y.bmp",
///   where X is the window ID, and Y is the frame number.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES: *const ::core::ffi::c_char =
    c"SDL_VIDEO_OFFSCREEN_SAVE_FRAMES".as_ptr();

/// A variable controlling whether all window operations will block until
/// complete.
///
/// Window systems that run asynchronously may not have the results of window
/// operations that resize or move the window applied immediately upon the
/// return of the requesting function. Setting this hint will cause such
/// operations to block after every call until the pending operation has
/// completed. Setting this to '1' is the equivalent of calling
/// [`SDL_SyncWindow()`] after every function call.
///
/// Be aware that amount of time spent blocking while waiting for window
/// operations to complete can be quite lengthy, as animations may have to
/// complete, which can take upwards of multiple seconds in some cases.
///
/// The variable can be set to the following values:
///
/// - "0": Window operations are non-blocking. (default)
/// - "1": Window operations will block until completed.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS: *const ::core::ffi::c_char =
    c"SDL_VIDEO_SYNC_WINDOW_OPERATIONS".as_ptr();

/// A variable controlling whether the libdecor Wayland backend is allowed to
/// be used.
///
/// libdecor is used over xdg-shell when xdg-decoration protocol is
/// unavailable.
///
/// The variable can be set to the following values:
///
/// - "0": libdecor use is disabled.
/// - "1": libdecor use is enabled. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WAYLAND_ALLOW_LIBDECOR".as_ptr();

/// A variable controlling whether video mode emulation is enabled under
/// Wayland.
///
/// When this hint is set, a standard set of emulated CVT video modes will be
/// exposed for use by the application. If it is disabled, the only modes
/// exposed will be the logical desktop size and, in the case of a scaled
/// desktop, the native display resolution.
///
/// The variable can be set to the following values:
///
/// - "0": Video mode emulation is disabled.
/// - "1": Video mode emulation is enabled. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WAYLAND_MODE_EMULATION".as_ptr();

/// A variable controlling how modes with a non-native aspect ratio are
/// displayed under Wayland.
///
/// When this hint is set, the requested scaling will be used when displaying
/// fullscreen video modes that don't match the display's native aspect ratio.
/// This is contingent on compositor viewport support.
///
/// The variable can be set to the following values:
///
/// - "aspect" - Video modes will be displayed scaled, in their proper aspect
///   ratio, with black bars.
/// - "stretch" - Video modes will be scaled to fill the entire display.
///   (default)
/// - "none" - Video modes will be displayed as 1:1 with no scaling.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WAYLAND_MODE_SCALING: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WAYLAND_MODE_SCALING".as_ptr();

/// A variable controlling whether the libdecor Wayland backend is preferred
/// over native decorations.
///
/// When this hint is set, libdecor will be used to provide window decorations,
/// even if xdg-decoration is available. (Note that, by default, libdecor will
/// use xdg-decoration itself if available).
///
/// The variable can be set to the following values:
///
/// - "0": libdecor is enabled only if server-side decorations are unavailable.
///   (default)
/// - "1": libdecor is always enabled if available.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WAYLAND_PREFER_LIBDECOR".as_ptr();

/// A variable forcing non-DPI-aware Wayland windows to output at 1:1 scaling.
///
/// This must be set before initializing the video subsystem.
///
/// When this hint is set, Wayland windows that are not flagged as being
/// DPI-aware will be output with scaling designed to force 1:1 pixel mapping.
///
/// This is intended to allow legacy applications to be displayed without
/// desktop scaling being applied, and has issues with certain display
/// configurations, as this forces the window to behave in a way that Wayland
/// desktops were not designed to accommodate:
///
/// - Rounding errors can result with odd window sizes and/or desktop scales,
///   which can cause the window contents to appear slightly blurry.
/// - Positioning the window may be imprecise due to unit conversions and
///   rounding.
/// - The window may be unusably small on scaled desktops.
/// - The window may jump in size when moving between displays of different
///   scale factors.
/// - Displays may appear to overlap when using a multi-monitor setup with
///   scaling enabled.
/// - Possible loss of cursor precision due to the logical size of the window
///   being reduced.
///
/// New applications should be designed with proper DPI awareness handling
/// instead of enabling this.
///
/// The variable can be set to the following values:
///
/// - "0": Windows will be scaled normally.
/// - "1": Windows will be forced to scale to achieve 1:1 output.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WAYLAND_SCALE_TO_DISPLAY".as_ptr();

/// A variable specifying which shader compiler to preload when using the
/// Chrome ANGLE binaries.
///
/// SDL has EGL and OpenGL ES2 support on Windows via the ANGLE project. It can
/// use two different sets of binaries, those compiled by the user from source
/// or those provided by the Chrome browser. In the later case, these binaries
/// require that SDL loads a DLL providing the shader compiler.
///
/// The variable can be set to the following values:
///
/// - "d3dcompiler_46.dll" - best for Vista or later. (default)
/// - "d3dcompiler_43.dll" - for XP support.
/// - "none" - do not load any library, useful if you compiled ANGLE from
///   source and included the compiler in your binaries.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_WIN_D3DCOMPILER: *const ::core::ffi::c_char =
    c"SDL_VIDEO_WIN_D3DCOMPILER".as_ptr();

/// A variable controlling whether SDL should call XSelectInput() to enable
/// input events on X11 windows wrapped by SDL windows.
///
/// The variable can be set to the following values:
///
/// - "0": Don't call XSelectInput(), assuming the native window code has done
///   it already.
/// - "1": Call XSelectInput() to enable input events. (default)
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.10.
pub const SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_EXTERNAL_WINDOW_INPUT".as_ptr();

/// A variable controlling whether the X11 _NET_WM_BYPASS_COMPOSITOR hint
/// should be used.
///
/// The variable can be set to the following values:
///
/// - "0": Disable _NET_WM_BYPASS_COMPOSITOR.
/// - "1": Enable _NET_WM_BYPASS_COMPOSITOR. (default)
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR".as_ptr();

/// A variable controlling whether the X11 _NET_WM_PING protocol should be
/// supported.
///
/// By default SDL will use _NET_WM_PING, but for applications that know they
/// will not always be able to respond to ping requests in a timely manner they
/// can turn it off to avoid the window manager thinking the app is hung.
///
/// The variable can be set to the following values:
///
/// - "0": Disable _NET_WM_PING.
/// - "1": Enable _NET_WM_PING. (default)
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_NET_WM_PING: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_NET_WM_PING".as_ptr();

/// A variable controlling whether SDL uses DirectColor visuals.
///
/// The variable can be set to the following values:
///
/// - "0": Disable DirectColor visuals.
/// - "1": Enable DirectColor visuals. (default)
///
/// This hint should be set before initializing the video subsystem.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_NODIRECTCOLOR: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_NODIRECTCOLOR".as_ptr();

/// A variable forcing the content scaling factor for X11 displays.
///
/// The variable can be set to a floating point value in the range 1.0-10.0f
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_SCALING_FACTOR: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_SCALING_FACTOR".as_ptr();

/// A variable forcing the visual ID used for X11 display modes.
///
/// This hint should be set before initializing the video subsystem.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_VISUALID: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_VISUALID".as_ptr();

/// A variable forcing the visual ID chosen for new X11 windows.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_WINDOW_VISUALID: *const ::core::ffi::c_char =
    c"SDL_VIDEO_X11_WINDOW_VISUALID".as_ptr();

/// A variable controlling whether the X11 XRandR extension should be used.
///
/// The variable can be set to the following values:
///
/// - "0": Disable XRandR.
/// - "1": Enable XRandR. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VIDEO_X11_XRANDR: *const ::core::ffi::c_char = c"SDL_VIDEO_X11_XRANDR".as_ptr();

/// A variable controlling whether touch should be enabled on the back panel of
/// the PlayStation Vita.
///
/// The variable can be set to the following values:
///
/// - "0": Disable touch on the back panel.
/// - "1": Enable touch on the back panel. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_ENABLE_BACK_TOUCH: *const ::core::ffi::c_char =
    c"SDL_VITA_ENABLE_BACK_TOUCH".as_ptr();

/// A variable controlling whether touch should be enabled on the front panel
/// of the PlayStation Vita.
///
/// The variable can be set to the following values:
///
/// - "0": Disable touch on the front panel.
/// - "1": Enable touch on the front panel. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_ENABLE_FRONT_TOUCH: *const ::core::ffi::c_char =
    c"SDL_VITA_ENABLE_FRONT_TOUCH".as_ptr();

/// A variable controlling the module path on the PlayStation Vita.
///
/// This hint defaults to "app0:module"
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_MODULE_PATH: *const ::core::ffi::c_char = c"SDL_VITA_MODULE_PATH".as_ptr();

/// A variable controlling whether to perform PVR initialization on the
/// PlayStation Vita.
///
/// - "0": Skip PVR initialization.
/// - "1": Perform the normal PVR initialization. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_PVR_INIT: *const ::core::ffi::c_char = c"SDL_VITA_PVR_INIT".as_ptr();

/// A variable overriding the resolution reported on the PlayStation Vita.
///
/// The variable can be set to the following values:
///
/// - "544": 544p (default)
/// - "720": 725p for PSTV
/// - "1080": 1088i for PSTV
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_RESOLUTION: *const ::core::ffi::c_char = c"SDL_VITA_RESOLUTION".as_ptr();

/// A variable controlling whether OpenGL should be used instead of OpenGL ES
/// on the PlayStation Vita.
///
/// The variable can be set to the following values:
///
/// - "0": Use OpenGL ES. (default)
/// - "1": Use OpenGL.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_PVR_OPENGL: *const ::core::ffi::c_char = c"SDL_VITA_PVR_OPENGL".as_ptr();

/// A variable controlling which touchpad should generate synthetic mouse
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": Only front touchpad should generate mouse events. (default)
/// - "1": Only back touchpad should generate mouse events.
/// - "2": Both touchpads should generate mouse events.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VITA_TOUCH_MOUSE_DEVICE: *const ::core::ffi::c_char =
    c"SDL_VITA_TOUCH_MOUSE_DEVICE".as_ptr();

/// A variable overriding the display index used in [`SDL_Vulkan_CreateSurface()`]
///
/// The display index starts at 0, which is the default.
///
/// This hint should be set before calling [`SDL_Vulkan_CreateSurface()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VULKAN_DISPLAY: *const ::core::ffi::c_char = c"SDL_VULKAN_DISPLAY".as_ptr();

/// Specify the Vulkan library to load.
///
/// This hint should be set before creating a Vulkan window or calling
/// [`SDL_Vulkan_LoadLibrary()`].
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_VULKAN_LIBRARY: *const ::core::ffi::c_char = c"SDL_VULKAN_LIBRARY".as_ptr();

/// A variable controlling how the fact chunk affects the loading of a WAVE
/// file.
///
/// The fact chunk stores information about the number of samples of a WAVE
/// file. The Standards Update from Microsoft notes that this value can be used
/// to 'determine the length of the data in seconds'. This is especially useful
/// for compressed formats (for which this is a mandatory chunk) if they
/// produce multiple sample frames per block and truncating the block is not
/// allowed. The fact chunk can exactly specify how many sample frames there
/// should be in this case.
///
/// Unfortunately, most application seem to ignore the fact chunk and so SDL
/// ignores it by default as well.
///
/// The variable can be set to the following values:
///
/// - "truncate" - Use the number of samples to truncate the wave data if the
///   fact chunk is present and valid.
/// - "strict" - Like "truncate", but raise an error if the fact chunk is
///   invalid, not present for non-PCM formats, or if the data chunk doesn't
///   have that many samples.
/// - "ignorezero" - Like "truncate", but ignore fact chunk if the number of
///   samples is zero.
/// - "ignore" - Ignore fact chunk entirely. (default)
///
/// This hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WAVE_FACT_CHUNK: *const ::core::ffi::c_char = c"SDL_WAVE_FACT_CHUNK".as_ptr();

/// A variable controlling the maximum number of chunks in a WAVE file.
///
/// This sets an upper bound on the number of chunks in a WAVE file to avoid
/// wasting time on malformed or corrupt WAVE files. This defaults to "10000".
///
/// This hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WAVE_CHUNK_LIMIT: *const ::core::ffi::c_char = c"SDL_WAVE_CHUNK_LIMIT".as_ptr();

/// A variable controlling how the size of the RIFF chunk affects the loading
/// of a WAVE file.
///
/// The size of the RIFF chunk (which includes all the sub-chunks of the WAVE
/// file) is not always reliable. In case the size is wrong, it's possible to
/// just ignore it and step through the chunks until a fixed limit is reached.
///
/// Note that files that have trailing data unrelated to the WAVE file or
/// corrupt files may slow down the loading process without a reliable
/// boundary. By default, SDL stops after 10000 chunks to prevent wasting time.
/// Use [`SDL_HINT_WAVE_CHUNK_LIMIT`] to adjust this value.
///
/// The variable can be set to the following values:
///
/// - "force" - Always use the RIFF chunk size as a boundary for the chunk
///   search.
/// - "ignorezero" - Like "force", but a zero size searches up to 4 GiB.
///   (default)
/// - "ignore" - Ignore the RIFF chunk size and always search up to 4 GiB.
/// - "maximum" - Search for chunks until the end of file. (not recommended)
///
/// This hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WAVE_RIFF_CHUNK_SIZE: *const ::core::ffi::c_char =
    c"SDL_WAVE_RIFF_CHUNK_SIZE".as_ptr();

/// A variable controlling how a truncated WAVE file is handled.
///
/// A WAVE file is considered truncated if any of the chunks are incomplete or
/// the data chunk size is not a multiple of the block size. By default, SDL
/// decodes until the first incomplete block, as most applications seem to do.
///
/// The variable can be set to the following values:
///
/// - "verystrict" - Raise an error if the file is truncated.
/// - "strict" - Like "verystrict", but the size of the RIFF chunk is ignored.
/// - "dropframe" - Decode until the first incomplete sample frame.
/// - "dropblock" - Decode until the first incomplete block. (default)
///
/// This hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WAVE_TRUNCATION: *const ::core::ffi::c_char = c"SDL_WAVE_TRUNCATION".as_ptr();

/// A variable controlling whether the window is activated when the
/// [`SDL_RaiseWindow`] function is called.
///
/// The variable can be set to the following values:
///
/// - "0": The window is not activated when the [`SDL_RaiseWindow`] function is
///   called.
/// - "1": The window is activated when the [`SDL_RaiseWindow`] function is called.
///   (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED: *const ::core::ffi::c_char =
    c"SDL_WINDOW_ACTIVATE_WHEN_RAISED".as_ptr();

/// A variable controlling whether the window is activated when the
/// [`SDL_ShowWindow`] function is called.
///
/// The variable can be set to the following values:
///
/// - "0": The window is not activated when the [`SDL_ShowWindow`] function is
///   called.
/// - "1": The window is activated when the [`SDL_ShowWindow`] function is called.
///   (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN: *const ::core::ffi::c_char =
    c"SDL_WINDOW_ACTIVATE_WHEN_SHOWN".as_ptr();

/// If set to "0" then never set the top-most flag on an SDL Window even if the
/// application requests it.
///
/// This is a debugging aid for developers and not expected to be used by end
/// users.
///
/// The variable can be set to the following values:
///
/// - "0": don't allow topmost
/// - "1": allow topmost (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOW_ALLOW_TOPMOST: *const ::core::ffi::c_char =
    c"SDL_WINDOW_ALLOW_TOPMOST".as_ptr();

/// A variable controlling whether the window frame and title bar are
/// interactive when the cursor is hidden.
///
/// The variable can be set to the following values:
///
/// - "0": The window frame is not interactive when the cursor is hidden (no
///   move, resize, etc).
/// - "1": The window frame is interactive when the cursor is hidden. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN: *const ::core::ffi::c_char =
    c"SDL_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN".as_ptr();

/// A variable controlling whether SDL generates window-close events for Alt+F4
/// on Windows.
///
/// The variable can be set to the following values:
///
/// - "0": SDL will only do normal key handling for Alt+F4.
/// - "1": SDL will generate a window-close event when it sees Alt+F4.
///   (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_CLOSE_ON_ALT_F4".as_ptr();

/// A variable controlling whether menus can be opened with their keyboard
/// shortcut (Alt+mnemonic).
///
/// If the mnemonics are enabled, then menus can be opened by pressing the Alt
/// key and the corresponding mnemonic (for example, Alt+F opens the File
/// menu). However, in case an invalid mnemonic is pressed, Windows makes an
/// audible beep to convey that nothing happened. This is true even if the
/// window has no menu at all!
///
/// Because most SDL applications don't have menus, and some want to use the
/// Alt key for other purposes, SDL disables mnemonics (and the beeping) by
/// default.
///
/// Note: This also affects keyboard events: with mnemonics enabled, when a
/// menu is opened from the keyboard, you will not receive a KEYUP event for
/// the mnemonic key, and *might* not receive one for Alt.
///
/// The variable can be set to the following values:
///
/// - "0": Alt+mnemonic does nothing, no beeping. (default)
/// - "1": Alt+mnemonic opens menus, invalid mnemonics produce a beep.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_ENABLE_MENU_MNEMONICS".as_ptr();

/// A variable controlling whether the windows message loop is processed by
/// SDL.
///
/// The variable can be set to the following values:
///
/// - "0": The window message loop is not run.
/// - "1": The window message loop is processed in [`SDL_PumpEvents()`]. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_ENABLE_MESSAGELOOP".as_ptr();

/// A variable controlling whether GameInput is used for raw keyboard and mouse
/// on Windows.
///
/// The variable can be set to the following values:
///
/// - "0": GameInput is not used for raw keyboard and mouse events.
/// - "1": GameInput is used for raw keyboard and mouse events, if available.
///   (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_GAMEINPUT: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_GAMEINPUT".as_ptr();

/// A variable controlling whether raw keyboard events are used on Windows.
///
/// The variable can be set to the following values:
///
/// - "0": The Windows message loop is used for keyboard events. (default)
/// - "1": Low latency raw keyboard events are used.
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_RAW_KEYBOARD: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_RAW_KEYBOARD".as_ptr();

/// A variable controlling whether SDL uses Kernel Semaphores on Windows.
///
/// Kernel Semaphores are inter-process and require a context switch on every
/// interaction. On Windows 8 and newer, the WaitOnAddress API is available.
/// Using that and atomics to implement semaphores increases performance. SDL
/// will fall back to Kernel Objects on older OS versions or if forced to by
/// this hint.
///
/// The variable can be set to the following values:
///
/// - "0": Use Atomics and WaitOnAddress API when available, otherwise fall
///   back to Kernel Objects. (default)
/// - "1": Force the use of Kernel Objects in all cases.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_FORCE_SEMAPHORE_KERNEL".as_ptr();

/// A variable to specify custom icon resource id from RC file on Windows
/// platform.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_INTRESOURCE_ICON: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_INTRESOURCE_ICON".as_ptr();

/// A variable to specify custom icon resource id from RC file on Windows
/// platform.
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_INTRESOURCE_ICON_SMALL".as_ptr();

/// A variable controlling whether SDL uses the D3D9Ex API introduced in
/// Windows Vista, instead of normal D3D9.
///
/// Direct3D 9Ex contains changes to state management that can eliminate device
/// loss errors during scenarios like Alt+Tab or UAC prompts. D3D9Ex may
/// require some changes to your application to cope with the new behavior, so
/// this is disabled by default.
///
/// For more information on Direct3D 9Ex, see:
///
/// - <https://docs.microsoft.com/en-us/windows/win32/direct3darticles/graphics-apis-in-windows-vista#direct3d-9ex>
/// - <https://docs.microsoft.com/en-us/windows/win32/direct3darticles/direct3d-9ex-improvements>
///
/// The variable can be set to the following values:
///
/// - "0": Use the original Direct3D 9 API. (default)
/// - "1": Use the Direct3D 9Ex API on Vista and later (and fall back if D3D9Ex
///   is unavailable)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_USE_D3D9EX: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_USE_D3D9EX".as_ptr();

/// A variable controlling whether SDL will clear the window contents when the
/// WM_ERASEBKGND message is received.
///
/// The variable can be set to the following values:
///
/// - "0"/"never": Never clear the window.
/// - "1"/"initial": Clear the window when the first WM_ERASEBKGND event fires.
///   (default)
/// - "2"/"always": Clear the window on every WM_ERASEBKGND event.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE: *const ::core::ffi::c_char =
    c"SDL_WINDOWS_ERASE_BACKGROUND_MODE".as_ptr();

/// A variable controlling whether X11 windows are marked as override-redirect.
///
/// If set, this _might_ increase framerate at the expense of the desktop not
/// working as expected. Override-redirect windows aren't noticed by the window
/// manager at all.
///
/// You should probably only use this for fullscreen windows, and you probably
/// shouldn't even use it for that. But it's here if you want to try!
///
/// The variable can be set to the following values:
///
/// - "0": Do not mark the window as override-redirect. (default)
/// - "1": Mark the window as override-redirect.
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT: *const ::core::ffi::c_char =
    c"SDL_X11_FORCE_OVERRIDE_REDIRECT".as_ptr();

/// A variable specifying the type of an X11 window.
///
/// During [`SDL_CreateWindow`], SDL uses the _NET_WM_WINDOW_TYPE X11 property to
/// report to the window manager the type of window it wants to create. This
/// might be set to various things if [`SDL_WINDOW_TOOLTIP`] or
/// [`SDL_WINDOW_POPUP_MENU`], etc, were specified. For "normal" windows that
/// haven't set a specific type, this hint can be used to specify a custom
/// type. For example, a dock window might set this to
/// "_NET_WM_WINDOW_TYPE_DOCK".
///
/// This hint should be set before creating a window.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_X11_WINDOW_TYPE: *const ::core::ffi::c_char = c"SDL_X11_WINDOW_TYPE".as_ptr();

/// Specify the XCB library to load for the X11 driver.
///
/// The default is platform-specific, often "libX11-xcb.so.1".
///
/// This hint should be set before initializing the video subsystem.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_X11_XCB_LIBRARY: *const ::core::ffi::c_char = c"SDL_X11_XCB_LIBRARY".as_ptr();

/// A variable controlling whether XInput should be used for controller
/// handling.
///
/// The variable can be set to the following values:
///
/// - "0": XInput is not enabled.
/// - "1": XInput is enabled. (default)
///
/// This hint should be set before SDL is initialized.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_XINPUT_ENABLED: *const ::core::ffi::c_char = c"SDL_XINPUT_ENABLED".as_ptr();

/// A variable controlling response to [`SDL_assert`] failures.
///
/// The variable can be set to the following case-sensitive values:
///
/// - "abort": Program terminates immediately.
/// - "break": Program triggers a debugger breakpoint.
/// - "retry": Program reruns the SDL_assert's test again.
/// - "ignore": Program continues on, ignoring this assertion failure this
///   time.
/// - "always_ignore": Program continues on, ignoring this assertion failure
///   for the rest of the run.
///
/// Note that [`SDL_SetAssertionHandler`] offers a programmatic means to deal with
/// assertion failures through a callback, and this hint is largely intended to
/// be used via environment variables by end users and automated tools.
///
/// This hint should be set before an assertion failure is triggered and can be
/// changed at any time.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_ASSERT: *const ::core::ffi::c_char = c"SDL_ASSERT".as_ptr();

/// A variable controlling whether pen events should generate synthetic mouse
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": Pen events will not generate mouse events.
/// - "1": Pen events will generate mouse events. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_PEN_MOUSE_EVENTS: *const ::core::ffi::c_char = c"SDL_PEN_MOUSE_EVENTS".as_ptr();

/// A variable controlling whether pen events should generate synthetic touch
/// events.
///
/// The variable can be set to the following values:
///
/// - "0": Pen events will not generate touch events.
/// - "1": Pen events will generate touch events. (default)
///
/// This hint can be set anytime.
///
/// ## Availability
/// This hint is available since SDL 3.2.0.
pub const SDL_HINT_PEN_TOUCH_EVENTS: *const ::core::ffi::c_char = c"SDL_PEN_TOUCH_EVENTS".as_ptr();

/// An enumeration of hint priorities.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`DEFAULT`](SDL_HintPriority::DEFAULT) | [`SDL_HINT_DEFAULT`] | |
/// | [`NORMAL`](SDL_HintPriority::NORMAL) | [`SDL_HINT_NORMAL`] | |
/// | [`OVERRIDE`](SDL_HintPriority::OVERRIDE) | [`SDL_HINT_OVERRIDE`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_HintPriority(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_HintPriority {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_HintPriority> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_HintPriority) -> bool {
        self == &other.0
    }
}

impl From<SDL_HintPriority> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_HintPriority) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_HintPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::DEFAULT => "SDL_HINT_DEFAULT",
            Self::NORMAL => "SDL_HINT_NORMAL",
            Self::OVERRIDE => "SDL_HINT_OVERRIDE",

            _ => return write!(f, "SDL_HintPriority({})", self.0),
        })
    }
}

impl SDL_HintPriority {
    pub const DEFAULT: Self = Self((0 as ::core::ffi::c_int));
    pub const NORMAL: Self = Self((1 as ::core::ffi::c_int));
    pub const OVERRIDE: Self = Self((2 as ::core::ffi::c_int));
}

pub const SDL_HINT_DEFAULT: SDL_HintPriority = SDL_HintPriority::DEFAULT;
pub const SDL_HINT_NORMAL: SDL_HintPriority = SDL_HintPriority::NORMAL;
pub const SDL_HINT_OVERRIDE: SDL_HintPriority = SDL_HintPriority::OVERRIDE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_HintPriority {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::hints::METADATA_SDL_HintPriority;
}

extern "C" {
    /// Set a hint with a specific priority.
    ///
    /// The priority controls the behavior when setting a hint that already has a
    /// value. Hints will replace existing hints of their priority and lower.
    /// Environment variables are considered to have override priority.
    ///
    /// ## Parameters
    /// - `name`: the hint to set.
    /// - `value`: the value of the hint variable.
    /// - `priority`: the [`SDL_HintPriority`] level for the hint.
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
    /// - [`SDL_GetHint`]
    /// - [`SDL_ResetHint`]
    /// - [`SDL_SetHint`]
    pub fn SDL_SetHintWithPriority(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        priority: SDL_HintPriority,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set a hint with normal priority.
    ///
    /// Hints will not be set if there is an existing override hint or environment
    /// variable that takes precedence. You can use [`SDL_SetHintWithPriority()`] to
    /// set the hint with override priority instead.
    ///
    /// ## Parameters
    /// - `name`: the hint to set.
    /// - `value`: the value of the hint variable.
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
    /// - [`SDL_GetHint`]
    /// - [`SDL_ResetHint`]
    /// - [`SDL_SetHintWithPriority`]
    pub fn SDL_SetHint(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Reset a hint to the default value.
    ///
    /// This will reset a hint to the value of the environment variable, or NULL if
    /// the environment isn't set. Callbacks will be called normally with this
    /// change.
    ///
    /// ## Parameters
    /// - `name`: the hint to set.
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
    /// - [`SDL_SetHint`]
    /// - [`SDL_ResetHints`]
    pub fn SDL_ResetHint(name: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Reset all hints to the default values.
    ///
    /// This will reset all hints to the value of the associated environment
    /// variable, or NULL if the environment isn't set. Callbacks will be called
    /// normally with this change.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ResetHint`]
    pub fn SDL_ResetHints();
}

extern "C" {
    /// Get the value of a hint.
    ///
    /// ## Parameters
    /// - `name`: the hint to query.
    ///
    /// ## Return value
    /// Returns the string value of a hint or NULL if the hint isn't set.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, however the
    ///   return value only remains valid until the hint is changed; if
    ///   another thread might do so, the app should supply locks
    ///   and/or make a copy of the string. Note that using a hint
    ///   callback instead is always thread-safe, as SDL holds a lock
    ///   on the thread subsystem during the callback.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetHint`]
    /// - [`SDL_SetHintWithPriority`]
    pub fn SDL_GetHint(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the boolean value of a hint variable.
    ///
    /// ## Parameters
    /// - `name`: the name of the hint to get the boolean value from.
    /// - `default_value`: the value to return if the hint does not exist.
    ///
    /// ## Return value
    /// Returns the boolean value of a hint or the provided default value if the
    ///   hint does not exist.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetHint`]
    /// - [`SDL_SetHint`]
    pub fn SDL_GetHintBoolean(
        name: *const ::core::ffi::c_char,
        default_value: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

/// A callback used to send notifications of hint value changes.
///
/// This is called an initial time during [`SDL_AddHintCallback`] with the hint's
/// current value, and then again each time the hint's value changes.
///
/// ## Parameters
/// - `userdata`: what was passed as `userdata` to [`SDL_AddHintCallback()`].
/// - `name`: what was passed as `name` to [`SDL_AddHintCallback()`].
/// - `oldValue`: the previous hint value.
/// - `newValue`: the new value hint is to be set to.
///
/// ## Thread safety
/// This callback is fired from whatever thread is setting a new
///   hint value. SDL holds a lock on the hint subsystem when
///   calling this callback.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_AddHintCallback`]
pub type SDL_HintCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        oldValue: *const ::core::ffi::c_char,
        newValue: *const ::core::ffi::c_char,
    ),
>;

extern "C" {
    /// Add a function to watch a particular hint.
    ///
    /// The callback function is called _during_ this function, to provide it an
    /// initial value, and again each time the hint's value changes.
    ///
    /// ## Parameters
    /// - `name`: the hint to watch.
    /// - `callback`: An [`SDL_HintCallback`] function that will be called when the
    ///   hint value changes.
    /// - `userdata`: a pointer to pass to the callback function.
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
    /// - [`SDL_RemoveHintCallback`]
    pub fn SDL_AddHintCallback(
        name: *const ::core::ffi::c_char,
        callback: SDL_HintCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove a function watching a particular hint.
    ///
    /// ## Parameters
    /// - `name`: the hint being watched.
    /// - `callback`: an [`SDL_HintCallback`] function that will be called when the
    ///   hint value changes.
    /// - `userdata`: a pointer being passed to the callback function.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AddHintCallback`]
    pub fn SDL_RemoveHintCallback(
        name: *const ::core::ffi::c_char,
        callback: SDL_HintCallback,
        userdata: *mut ::core::ffi::c_void,
    );
}

#[cfg(doc)]
use crate::everything::*;
