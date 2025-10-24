//! Metadata for items in the `crate::hints` module

use super::*;

pub const METADATA_SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED",
    short_name: "ALLOW_ALT_TAB_WHILE_GRABBED",
    value: crate::hints::SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED,
    doc: Some(
        "Specify the behavior of Alt+Tab while the keyboard is grabbed.\n\nBy default, SDL emulates Alt+Tab functionality while the keyboard is\ngrabbed and your window is full-screen. This prevents the user from getting\nstuck in your application if you've enabled keyboard grab.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will not handle Alt+Tab. Your application is responsible for\nhandling Alt+Tab while the keyboard is grabbed.\n- \"1\": SDL will minimize your window when Alt+Tab is pressed (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY",
    short_name: "ANDROID_ALLOW_RECREATE_ACTIVITY",
    value: crate::hints::SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY,
    doc: Some(
        "A variable to control whether the SDL activity is allowed to be re-created.\n\nIf this hint is true, the activity can be recreated on demand by the OS,\nand Java static data and C++ static data remain with their current values.\nIf this hint is false, then SDL will call exit() when you return from your\nmain function and the application will be terminated and then started fresh\neach time.\n\nThe variable can be set to the following values:\n\n- \"0\": The application starts fresh at each launch. (default)\n- \"1\": The application activity can be recreated by the OS.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ANDROID_BLOCK_ON_PAUSE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ANDROID_BLOCK_ON_PAUSE",
    short_name: "ANDROID_BLOCK_ON_PAUSE",
    value: crate::hints::SDL_HINT_ANDROID_BLOCK_ON_PAUSE,
    doc: Some(
        "A variable to control whether the event loop will block itself when the app\nis paused.\n\nThe variable can be set to the following values:\n\n- \"0\": Non blocking.\n- \"1\": Blocking. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ANDROID_LOW_LATENCY_AUDIO: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ANDROID_LOW_LATENCY_AUDIO",
    short_name: "ANDROID_LOW_LATENCY_AUDIO",
    value: crate::hints::SDL_HINT_ANDROID_LOW_LATENCY_AUDIO,
    doc: Some(
        "A variable to control whether low latency audio should be enabled.\n\nSome devices have poor quality output when this is enabled, but this is\nusually an improvement in audio latency.\n\nThe variable can be set to the following values:\n\n- \"0\": Low latency audio is not enabled.\n- \"1\": Low latency audio is enabled. (default)\n\nThis hint should be set before SDL audio is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ANDROID_TRAP_BACK_BUTTON: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ANDROID_TRAP_BACK_BUTTON",
    short_name: "ANDROID_TRAP_BACK_BUTTON",
    value: crate::hints::SDL_HINT_ANDROID_TRAP_BACK_BUTTON,
    doc: Some(
        "A variable to control whether we trap the Android back button to handle it\nmanually.\n\nThis is necessary for the right mouse button to work on some Android\ndevices, or to be able to trap the back button for use in your code\nreliably. If this hint is true, the back button will show up as an\n[`SDL_EVENT_KEY_DOWN`] / [`SDL_EVENT_KEY_UP`] pair with a keycode of\n[`SDL_SCANCODE_AC_BACK`].\n\nThe variable can be set to the following values:\n\n- \"0\": Back button will be handled as usual for system. (default)\n- \"1\": Back button will be trapped, allowing you to handle the key press\nmanually. (This will also let right mouse click work on systems where the\nright mouse button functions as back.)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_APP_ID: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_APP_ID",
    short_name: "APP_ID",
    value: crate::hints::SDL_HINT_APP_ID,
    doc: Some(
        "A variable setting the app ID string.\n\nThis string is used by desktop compositors to identify and group windows\ntogether, as well as match applications with associated desktop settings\nand icons.\n\nThis will override [`SDL_PROP_APP_METADATA_IDENTIFIER_STRING`], if set by the\napplication.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_APP_NAME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_APP_NAME",
    short_name: "APP_NAME",
    value: crate::hints::SDL_HINT_APP_NAME,
    doc: Some(
        "A variable setting the application name.\n\nThis hint lets you specify the application name sent to the OS when\nrequired. For example, this will often appear in volume control applets for\naudio streams, and in lists of applications which are inhibiting the\nscreensaver. You should use a string that describes your program (\"My Game\n2: The Revenge\")\n\nThis will override [`SDL_PROP_APP_METADATA_NAME_STRING`], if set by the\napplication.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS",
    short_name: "APPLE_TV_CONTROLLER_UI_EVENTS",
    value: crate::hints::SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS,
    doc: Some(
        "A variable controlling whether controllers used with the Apple TV generate\nUI events.\n\nWhen UI events are generated by controller input, the app will be\nbackgrounded when the Apple TV remote's menu button is pressed, and when\nthe pause or B buttons on gamepads are pressed.\n\nMore information about properly making use of controllers for the Apple TV\ncan be found here:\n<https://developer.apple.com/tvos/human-interface-guidelines/remote-and-controllers/>\n\nThe variable can be set to the following values:\n\n- \"0\": Controller input does not generate UI events. (default)\n- \"1\": Controller input generates UI events.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION",
    short_name: "APPLE_TV_REMOTE_ALLOW_ROTATION",
    value: crate::hints::SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION,
    doc: Some(
        "A variable controlling whether the Apple TV remote's joystick axes will\nautomatically match the rotation of the remote.\n\nThe variable can be set to the following values:\n\n- \"0\": Remote orientation does not affect joystick axes. (default)\n- \"1\": Joystick axes are based on the orientation of the remote.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE",
    short_name: "AUDIO_ALSA_DEFAULT_DEVICE",
    value: crate::hints::SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE,
    doc: Some(
        "Specify the default ALSA audio device name.\n\nThis variable is a specific audio device to open when the \"default\" audio\ndevice is used.\n\nThis hint will be ignored when opening the default playback device if\n[`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`] is set, or when opening the\ndefault recording device if [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`] is\nset.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`]\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE",
    short_name: "AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE",
    value: crate::hints::SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE,
    doc: Some(
        "Specify the default ALSA audio playback device name.\n\nThis variable is a specific audio device to open for playback, when the\n\"default\" audio device is used.\n\nIf this hint isn't set, SDL will check [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]\nbefore choosing a reasonable default.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE`]\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE",
    short_name: "AUDIO_ALSA_DEFAULT_RECORDING_DEVICE",
    value: crate::hints::SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE,
    doc: Some(
        "Specify the default ALSA audio recording device name.\n\nThis variable is a specific audio device to open for recording, when the\n\"default\" audio device is used.\n\nIf this hint isn't set, SDL will check [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]\nbefore choosing a reasonable default.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE`]\n- [`SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_CATEGORY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_CATEGORY",
    short_name: "AUDIO_CATEGORY",
    value: crate::hints::SDL_HINT_AUDIO_CATEGORY,
    doc: Some(
        "A variable controlling the audio category on iOS and macOS.\n\nThe variable can be set to the following values:\n\n- \"ambient\": Use the AVAudioSessionCategoryAmbient audio category, will be\nmuted by the phone mute switch (default)\n- \"playback\": Use the AVAudioSessionCategoryPlayback category.\n\nFor more information, see Apple's documentation:\n<https://developer.apple.com/library/content/documentation/Audio/Conceptual/AudioSessionProgrammingGuide/AudioSessionCategoriesandModes/AudioSessionCategoriesandModes.html>\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_CHANNELS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_CHANNELS",
    short_name: "AUDIO_CHANNELS",
    value: crate::hints::SDL_HINT_AUDIO_CHANNELS,
    doc: Some(
        "A variable controlling the default audio channel count.\n\nIf the application doesn't specify the audio channel count when opening the\ndevice, this hint can be used to specify a default channel count that will\nbe used. This defaults to \"1\" for recording and \"2\" for playback devices.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME",
    short_name: "AUDIO_DEVICE_APP_ICON_NAME",
    value: crate::hints::SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME,
    doc: Some(
        "Specify an application icon name for an audio device.\n\nSome audio backends (such as Pulseaudio and Pipewire) allow you to set an\nXDG icon name for your application. Among other things, this icon might\nshow up in a system control panel that lets the user adjust the volume on\nspecific audio streams instead of using one giant master volume slider.\nNote that this is unrelated to the icon used by the windowing system, which\nmay be set with [`SDL_SetWindowIcon`] (or via desktop file on Wayland).\n\nSetting this to \"\" or leaving it unset will have SDL use a reasonable\ndefault, \"applications-games\", which is likely to be installed. See\n<https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html>\nand\n<https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html>\nfor the relevant XDG icon specs.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES",
    short_name: "AUDIO_DEVICE_SAMPLE_FRAMES",
    value: crate::hints::SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES,
    doc: Some(
        "A variable controlling device buffer size.\n\nThis hint is an integer > 0, that represents the size of the device's\nbuffer in sample frames (stereo audio data in 16-bit format is 4 bytes per\nsample frame, for example).\n\nSDL3 generally decides this value on behalf of the app, but if for some\nreason the app needs to dictate this (because they want either lower\nlatency or higher throughput AND ARE WILLING TO DEAL WITH what that might\nrequire of the app), they can specify it.\n\nSDL will try to accommodate this value, but there is no promise you'll get\nthe buffer size requested. Many platforms won't honor this request at all,\nor might adjust it.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DEVICE_STREAM_NAME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DEVICE_STREAM_NAME",
    short_name: "AUDIO_DEVICE_STREAM_NAME",
    value: crate::hints::SDL_HINT_AUDIO_DEVICE_STREAM_NAME,
    doc: Some(
        "Specify an audio stream name for an audio device.\n\nSome audio backends (such as PulseAudio) allow you to describe your audio\nstream. Among other things, this description might show up in a system\ncontrol panel that lets the user adjust the volume on specific audio\nstreams instead of using one giant master volume slider.\n\nThis hints lets you transmit that information to the OS. The contents of\nthis hint are used while opening an audio device. You should use a string\nthat describes your what your program is playing (\"audio stream\" is\nprobably sufficient in many cases, but this could be useful for something\nlike \"team chat\" if you have a headset playing VoIP audio separately).\n\nSetting this to \"\" or leaving it unset will have SDL use a reasonable\ndefault: \"audio stream\" or something similar.\n\nNote that while this talks about audio streams, this is an OS-level\nconcept, so it applies to a physical audio device in this case, and not an\n[`SDL_AudioStream`], nor an SDL logical audio device.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DEVICE_STREAM_ROLE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DEVICE_STREAM_ROLE",
    short_name: "AUDIO_DEVICE_STREAM_ROLE",
    value: crate::hints::SDL_HINT_AUDIO_DEVICE_STREAM_ROLE,
    doc: Some(
        "Specify an application role for an audio device.\n\nSome audio backends (such as Pipewire) allow you to describe the role of\nyour audio stream. Among other things, this description might show up in a\nsystem control panel or software for displaying and manipulating media\nplayback/recording graphs.\n\nThis hints lets you transmit that information to the OS. The contents of\nthis hint are used while opening an audio device. You should use a string\nthat describes your what your program is playing (Game, Music, Movie,\netc...).\n\nSetting this to \"\" or leaving it unset will have SDL use a reasonable\ndefault: \"Game\" or something similar.\n\nNote that while this talks about audio streams, this is an OS-level\nconcept, so it applies to a physical audio device in this case, and not an\n[`SDL_AudioStream`], nor an SDL logical audio device.\n\nFor Windows WASAPI audio, the following roles are supported, and map to\n`AUDIO_STREAM_CATEGORY`:\n\n- \"Other\" (default)\n- \"Communications\" - Real-time communications, such as VOIP or chat\n- \"Game\" - Game audio\n- \"GameChat\" - Game chat audio, similar to \"Communications\" except that\nthis will not attenuate other audio streams\n- \"Movie\" - Music or sound with dialog\n- \"Media\" - Music or sound without dialog\n\nIf your application applies its own echo cancellation, gain control, and\nnoise reduction it should also set [`SDL_HINT_AUDIO_DEVICE_RAW_STREAM`].\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DEVICE_RAW_STREAM: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DEVICE_RAW_STREAM",
    short_name: "AUDIO_DEVICE_RAW_STREAM",
    value: crate::hints::SDL_HINT_AUDIO_DEVICE_RAW_STREAM,
    doc: Some(
        "Specify whether this audio device should do audio processing.\n\nSome operating systems perform echo cancellation, gain control, and noise\nreduction as needed. If your application already handles these, you can set\nthis hint to prevent the OS from doing additional audio processing.\n\nThis corresponds to the WASAPI audio option `AUDCLNT_STREAMOPTIONS_RAW`.\n\nThe variable can be set to the following values:\n\n- \"0\": audio processing can be done by the OS. (default)\n- \"1\": audio processing is done by the application.\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DISK_INPUT_FILE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DISK_INPUT_FILE",
    short_name: "AUDIO_DISK_INPUT_FILE",
    value: crate::hints::SDL_HINT_AUDIO_DISK_INPUT_FILE,
    doc: Some(
        "Specify the input file when recording audio using the disk audio driver.\n\nThis defaults to \"sdlaudio-in.raw\"\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DISK_OUTPUT_FILE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DISK_OUTPUT_FILE",
    short_name: "AUDIO_DISK_OUTPUT_FILE",
    value: crate::hints::SDL_HINT_AUDIO_DISK_OUTPUT_FILE,
    doc: Some(
        "Specify the output file when playing audio using the disk audio driver.\n\nThis defaults to \"sdlaudio.raw\"\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DISK_TIMESCALE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DISK_TIMESCALE",
    short_name: "AUDIO_DISK_TIMESCALE",
    value: crate::hints::SDL_HINT_AUDIO_DISK_TIMESCALE,
    doc: Some(
        "A variable controlling the audio rate when using the disk audio driver.\n\nThe disk audio driver normally simulates real-time for the audio rate that\nwas specified, but you can use this variable to adjust this rate higher or\nlower down to 0. The default value is \"1.0\".\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DRIVER",
    short_name: "AUDIO_DRIVER",
    value: crate::hints::SDL_HINT_AUDIO_DRIVER,
    doc: Some(
        "A variable that specifies an audio backend to use.\n\nBy default, SDL will try all available audio backends in a reasonable order\nuntil it finds one that can work, but this hint allows the app or user to\nforce a specific driver, such as \"pipewire\" if, say, you are on PulseAudio\nbut want to try talking to the lower level instead.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_DUMMY_TIMESCALE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_DUMMY_TIMESCALE",
    short_name: "AUDIO_DUMMY_TIMESCALE",
    value: crate::hints::SDL_HINT_AUDIO_DUMMY_TIMESCALE,
    doc: Some(
        "A variable controlling the audio rate when using the dummy audio driver.\n\nThe dummy audio driver normally simulates real-time for the audio rate that\nwas specified, but you can use this variable to adjust this rate higher or\nlower down to 0. The default value is \"1.0\".\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_FORMAT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_FORMAT",
    short_name: "AUDIO_FORMAT",
    value: crate::hints::SDL_HINT_AUDIO_FORMAT,
    doc: Some(
        "A variable controlling the default audio format.\n\nIf the application doesn't specify the audio format when opening the\ndevice, this hint can be used to specify a default format that will be\nused.\n\nThe variable can be set to the following values:\n\n- \"U8\": Unsigned 8-bit audio\n- \"S8\": Signed 8-bit audio\n- \"S16LE\": Signed 16-bit little-endian audio\n- \"S16BE\": Signed 16-bit big-endian audio\n- \"S16\": Signed 16-bit native-endian audio (default)\n- \"S32LE\": Signed 32-bit little-endian audio\n- \"S32BE\": Signed 32-bit big-endian audio\n- \"S32\": Signed 32-bit native-endian audio\n- \"F32LE\": Floating point little-endian audio\n- \"F32BE\": Floating point big-endian audio\n- \"F32\": Floating point native-endian audio\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_FREQUENCY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_FREQUENCY",
    short_name: "AUDIO_FREQUENCY",
    value: crate::hints::SDL_HINT_AUDIO_FREQUENCY,
    doc: Some(
        "A variable controlling the default audio frequency.\n\nIf the application doesn't specify the audio frequency when opening the\ndevice, this hint can be used to specify a default frequency that will be\nused. This defaults to \"44100\".\n\nThis hint should be set before an audio device is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUDIO_INCLUDE_MONITORS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUDIO_INCLUDE_MONITORS",
    short_name: "AUDIO_INCLUDE_MONITORS",
    value: crate::hints::SDL_HINT_AUDIO_INCLUDE_MONITORS,
    doc: Some(
        "A variable that causes SDL to not ignore audio \"monitors\".\n\nThis is currently only used by the PulseAudio driver.\n\nBy default, SDL ignores audio devices that aren't associated with physical\nhardware. Changing this hint to \"1\" will expose anything SDL sees that\nappears to be an audio source or sink. This will add \"devices\" to the list\nthat the user probably doesn't want or need, but it can be useful in\nscenarios where you want to hook up SDL to some sort of virtual device,\netc.\n\nThe variable can be set to the following values:\n\n- \"0\": Audio monitor devices will be ignored. (default)\n- \"1\": Audio monitor devices will show up in the device list.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUTO_UPDATE_JOYSTICKS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUTO_UPDATE_JOYSTICKS",
    short_name: "AUTO_UPDATE_JOYSTICKS",
    value: crate::hints::SDL_HINT_AUTO_UPDATE_JOYSTICKS,
    doc: Some(
        "A variable controlling whether SDL updates joystick state when getting\ninput events.\n\nThe variable can be set to the following values:\n\n- \"0\": You'll call [`SDL_UpdateJoysticks()`] manually.\n- \"1\": SDL will automatically call [`SDL_UpdateJoysticks()`]. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_AUTO_UPDATE_SENSORS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_AUTO_UPDATE_SENSORS",
    short_name: "AUTO_UPDATE_SENSORS",
    value: crate::hints::SDL_HINT_AUTO_UPDATE_SENSORS,
    doc: Some(
        "A variable controlling whether SDL updates sensor state when getting input\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": You'll call [`SDL_UpdateSensors()`] manually.\n- \"1\": SDL will automatically call [`SDL_UpdateSensors()`]. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_BMP_SAVE_LEGACY_FORMAT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_BMP_SAVE_LEGACY_FORMAT",
    short_name: "BMP_SAVE_LEGACY_FORMAT",
    value: crate::hints::SDL_HINT_BMP_SAVE_LEGACY_FORMAT,
    doc: Some(
        "Prevent SDL from using version 4 of the bitmap header when saving BMPs.\n\nThe bitmap header version 4 is required for proper alpha channel support\nand SDL will use it when required. Should this not be desired, this hint\ncan force the use of the 40 byte header version which is supported\neverywhere.\n\nThe variable can be set to the following values:\n\n- \"0\": Surfaces with a colorkey or an alpha channel are saved to a 32-bit\nBMP file with an alpha mask. SDL will use the bitmap header version 4 and\nset the alpha mask accordingly. (default)\n- \"1\": Surfaces with a colorkey or an alpha channel are saved to a 32-bit\nBMP file without an alpha mask. The alpha channel data will be in the\nfile, but applications are going to ignore it.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_CAMERA_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_CAMERA_DRIVER",
    short_name: "CAMERA_DRIVER",
    value: crate::hints::SDL_HINT_CAMERA_DRIVER,
    doc: Some(
        "A variable that decides what camera backend to use.\n\nBy default, SDL will try all available camera backends in a reasonable\norder until it finds one that can work, but this hint allows the app or\nuser to force a specific target, such as \"directshow\" if, say, you are on\nWindows Media Foundations but want to try DirectShow instead.\n\nThe default value is unset, in which case SDL will try to figure out the\nbest camera backend on your behalf. This hint needs to be set before\n[`SDL_Init()`] is called to be useful.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_CPU_FEATURE_MASK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_CPU_FEATURE_MASK",
    short_name: "CPU_FEATURE_MASK",
    value: crate::hints::SDL_HINT_CPU_FEATURE_MASK,
    doc: Some(
        "A variable that limits what CPU features are available.\n\nBy default, SDL marks all features the current CPU supports as available.\nThis hint allows the enabled features to be limited to a subset.\n\nWhen the hint is unset, or empty, SDL will enable all detected CPU\nfeatures.\n\nThe variable can be set to a comma separated list containing the following\nitems:\n\n- \"all\"\n- \"altivec\"\n- \"sse\"\n- \"sse2\"\n- \"sse3\"\n- \"sse41\"\n- \"sse42\"\n- \"avx\"\n- \"avx2\"\n- \"avx512f\"\n- \"arm-simd\"\n- \"neon\"\n- \"lsx\"\n- \"lasx\"\n\nThe items can be prefixed by '+'/'-' to add/remove features.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_DIRECTINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_DIRECTINPUT",
    short_name: "JOYSTICK_DIRECTINPUT",
    value: crate::hints::SDL_HINT_JOYSTICK_DIRECTINPUT,
    doc: Some(
        "A variable controlling whether DirectInput should be used for controllers.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable DirectInput detection.\n- \"1\": Enable DirectInput detection. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_FILE_DIALOG_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_FILE_DIALOG_DRIVER",
    short_name: "FILE_DIALOG_DRIVER",
    value: crate::hints::SDL_HINT_FILE_DIALOG_DRIVER,
    doc: Some(
        "A variable that specifies a dialog backend to use.\n\nBy default, SDL will try all available dialog backends in a reasonable\norder until it finds one that can work, but this hint allows the app or\nuser to force a specific target.\n\nIf the specified target does not exist or is not available, the\ndialog-related function calls will fail.\n\nThis hint currently only applies to platforms using the generic \"Unix\"\ndialog implementation, but may be extended to more platforms in the future.\nNote that some Unix and Unix-like platforms have their own implementation,\nsuch as macOS and Haiku.\n\nThe variable can be set to the following values:\n\n- NULL: Select automatically (default, all platforms)\n- \"portal\": Use XDG Portals through DBus (Unix only)\n- \"zenity\": Use the Zenity program (Unix only)\n\nMore options may be added in the future.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_DISPLAY_USABLE_BOUNDS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_DISPLAY_USABLE_BOUNDS",
    short_name: "DISPLAY_USABLE_BOUNDS",
    value: crate::hints::SDL_HINT_DISPLAY_USABLE_BOUNDS,
    doc: Some(
        "Override for [`SDL_GetDisplayUsableBounds()`].\n\nIf set, this hint will override the expected results for\n[`SDL_GetDisplayUsableBounds()`] for display index 0. Generally you don't want\nto do this, but this allows an embedded system to request that some of the\nscreen be reserved for other uses when paired with a well-behaved\napplication.\n\nThe contents of this hint must be 4 comma-separated integers, the first is\nthe bounds x, then y, width and height, in that order.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_INVALID_PARAM_CHECKS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_INVALID_PARAM_CHECKS",
    short_name: "INVALID_PARAM_CHECKS",
    value: crate::hints::SDL_HINT_INVALID_PARAM_CHECKS,
    doc: Some(
        "Set the level of checking for invalid parameters passed to SDL functions.\n\nThe variable can be set to the following values:\n\n- \"1\": Enable fast parameter error checking, e.g. quick NULL checks, etc.\n- \"2\": Enable full parameter error checking, e.g. validating objects are\nthe correct type, etc. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_EMSCRIPTEN_ASYNCIFY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EMSCRIPTEN_ASYNCIFY",
    short_name: "EMSCRIPTEN_ASYNCIFY",
    value: crate::hints::SDL_HINT_EMSCRIPTEN_ASYNCIFY,
    doc: Some(
        "Disable giving back control to the browser automatically when running with\nasyncify.\n\nWith -s ASYNCIFY, SDL calls emscripten_sleep during operations such as\nrefreshing the screen or polling events.\n\nThis hint only applies to the emscripten platform.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable emscripten_sleep calls (if you give back browser control\nmanually or use asyncify for other purposes).\n- \"1\": Enable emscripten_sleep calls. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR",
    short_name: "EMSCRIPTEN_CANVAS_SELECTOR",
    value: crate::hints::SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR,
    doc: Some(
        "Specify the CSS selector used for the \"default\" window/canvas.\n\nThis hint only applies to the emscripten platform.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT",
    short_name: "EMSCRIPTEN_KEYBOARD_ELEMENT",
    value: crate::hints::SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT,
    doc: Some(
        "Override the binding element for keyboard inputs for Emscripten builds.\n\nThis hint only applies to the emscripten platform.\n\nThe variable can be one of:\n\n- \"#window\": the javascript window object\n- \"#document\": the javascript document object\n- \"#screen\": the javascript window.screen object\n- \"#canvas\": the WebGL canvas element\n- \"#none\": Don't bind anything at all\n- any other string without a leading # sign applies to the element on the\npage with that ID.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EMSCRIPTEN_FILL_DOCUMENT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EMSCRIPTEN_FILL_DOCUMENT",
    short_name: "EMSCRIPTEN_FILL_DOCUMENT",
    value: crate::hints::SDL_HINT_EMSCRIPTEN_FILL_DOCUMENT,
    doc: Some(
        "Dictate that windows on Emscripten will fill the whole browser window.\n\nWhen enabled, the canvas element fills the entire document. Resize events\nwill be generated as the browser window is resized, as that will adjust the\ncanvas size as well. The canvas will cover anything else on the page,\nincluding any controls provided by Emscripten in its generated HTML file\n(in fact, any elements on the page that aren't the canvas will be moved\ninto a hidden `div` element).\n\nOften times this is desirable for a browser-based game, but it means\nseveral things that we expect of an SDL window on other platforms might not\nwork as expected, such as minimum window sizes and aspect ratios.\n\nThis hint overrides [`SDL_PROP_WINDOW_CREATE_EMSCRIPTEN_FILL_DOCUMENT_BOOLEAN`]\nproperties when creating an SDL window.\n\nThis hint only applies to the Emscripten platform.\n\nThis hint can be set at any time (before creating the window, or to toggle\nits state later). Only one window can fill the document at a time.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_ENABLE_SCREEN_KEYBOARD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ENABLE_SCREEN_KEYBOARD",
    short_name: "ENABLE_SCREEN_KEYBOARD",
    value: crate::hints::SDL_HINT_ENABLE_SCREEN_KEYBOARD,
    doc: Some(
        "A variable that controls whether the on-screen keyboard should be shown\nwhen text input is active.\n\nThe variable can be set to the following values:\n\n- \"auto\": The on-screen keyboard will be shown if there is no physical\nkeyboard attached. (default)\n- \"0\": Do not show the on-screen keyboard.\n- \"1\": Show the on-screen keyboard, if available.\n\nThis hint must be set before [`SDL_StartTextInput()`] is called\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EVDEV_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EVDEV_DEVICES",
    short_name: "EVDEV_DEVICES",
    value: crate::hints::SDL_HINT_EVDEV_DEVICES,
    doc: Some(
        "A variable containing a list of evdev devices to use if udev is not\navailable.\n\nThe list of devices is in the form:\n\ndeviceclass:path\\[,deviceclass:path\\[,...\\]\\]\n\nwhere device class is an integer representing the SDL_UDEV_deviceclass and\npath is the full path to the event device.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EVENT_LOGGING: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EVENT_LOGGING",
    short_name: "EVENT_LOGGING",
    value: crate::hints::SDL_HINT_EVENT_LOGGING,
    doc: Some(
        "A variable controlling verbosity of the logging of SDL events pushed onto\nthe internal queue.\n\nThe variable can be set to the following values, from least to most\nverbose:\n\n- \"0\": Don't log any events. (default)\n- \"1\": Log most events (other than the really spammy ones).\n- \"2\": Include mouse and finger motion events.\n\nThis is generally meant to be used to debug SDL itself, but can be useful\nfor application developers that need better visibility into what is going\non in the event queue. Logged events are sent through [`SDL_Log()`], which\nmeans by default they appear on stdout on most platforms or maybe\nOutputDebugString() on Windows, and can be funneled by the app with\n[`SDL_SetLogOutputFunction()`], etc.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_FORCE_RAISEWINDOW: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_FORCE_RAISEWINDOW",
    short_name: "FORCE_RAISEWINDOW",
    value: crate::hints::SDL_HINT_FORCE_RAISEWINDOW,
    doc: Some(
        "A variable controlling whether raising the window should be done more\nforcefully.\n\nThe variable can be set to the following values:\n\n- \"0\": Honor the OS policy for raising windows. (default)\n- \"1\": Force the window to be raised, overriding any OS policy.\n\nAt present, this is only an issue under MS Windows, which makes it nearly\nimpossible to programmatically move a window to the foreground, for\n\"security\" reasons. See <http://stackoverflow.com/a/34414846> for a\ndiscussion.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_FRAMEBUFFER_ACCELERATION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_FRAMEBUFFER_ACCELERATION",
    short_name: "FRAMEBUFFER_ACCELERATION",
    value: crate::hints::SDL_HINT_FRAMEBUFFER_ACCELERATION,
    doc: Some(
        "A variable controlling how 3D acceleration is used to accelerate the SDL\nscreen surface.\n\nSDL can try to accelerate the SDL screen surface by using streaming\ntextures with a 3D rendering engine. This variable controls whether and how\nthis is done.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable 3D acceleration\n- \"1\": Enable 3D acceleration, using the default renderer. (default)\n- \"X\": Enable 3D acceleration, using X where X is one of the valid\nrendering drivers. (e.g. \"direct3d\", \"opengl\", etc.)\n\nThis hint should be set before calling [`SDL_GetWindowSurface()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLERCONFIG: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLERCONFIG",
    short_name: "GAMECONTROLLERCONFIG",
    value: crate::hints::SDL_HINT_GAMECONTROLLERCONFIG,
    doc: Some(
        "A variable that lets you manually hint extra gamecontroller db entries.\n\nThe variable should be newline delimited rows of gamecontroller config\ndata, see SDL_gamepad.h\n\nYou can update mappings after SDL is initialized with\n[`SDL_GetGamepadMappingForGUID()`] and [`SDL_AddGamepadMapping()`]\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLERCONFIG_FILE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLERCONFIG_FILE",
    short_name: "GAMECONTROLLERCONFIG_FILE",
    value: crate::hints::SDL_HINT_GAMECONTROLLERCONFIG_FILE,
    doc: Some(
        "A variable that lets you provide a file with extra gamecontroller db\nentries.\n\nThe file should contain lines of gamecontroller config data, see\nSDL_gamepad.h\n\nYou can update mappings after SDL is initialized with\n[`SDL_GetGamepadMappingForGUID()`] and [`SDL_AddGamepadMapping()`]\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLERTYPE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLERTYPE",
    short_name: "GAMECONTROLLERTYPE",
    value: crate::hints::SDL_HINT_GAMECONTROLLERTYPE,
    doc: Some(
        "A variable that overrides the automatic controller type detection.\n\nThe variable should be comma separated entries, in the form: VID/PID=type\n\nThe VID and PID should be hexadecimal with exactly 4 digits, e.g. 0x00fd\n\nThis hint affects what low level protocol is used with the HIDAPI driver.\n\nThe variable can be set to the following values:\n\n- \"Xbox360\"\n- \"XboxOne\"\n- \"PS3\"\n- \"PS4\"\n- \"PS5\"\n- \"SwitchPro\"\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES",
    short_name: "GAMECONTROLLER_IGNORE_DEVICES",
    value: crate::hints::SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES,
    doc: Some(
        "A variable containing a list of devices to skip when scanning for game\ncontrollers.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n0xAAAA/0xBBBB,0xCCCC/0xDDDD\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT",
    short_name: "GAMECONTROLLER_IGNORE_DEVICES_EXCEPT",
    value: crate::hints::SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT,
    doc: Some(
        "If set, all devices will be skipped when scanning for game controllers\nexcept for the ones listed in this variable.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n0xAAAA/0xBBBB,0xCCCC/0xDDDD\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GAMECONTROLLER_SENSOR_FUSION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GAMECONTROLLER_SENSOR_FUSION",
    short_name: "GAMECONTROLLER_SENSOR_FUSION",
    value: crate::hints::SDL_HINT_GAMECONTROLLER_SENSOR_FUSION,
    doc: Some(
        "A variable that controls whether the device's built-in accelerometer and\ngyro should be used as sensors for gamepads.\n\nThe variable can be set to the following values:\n\n- \"0\": Sensor fusion is disabled\n- \"1\": Sensor fusion is enabled for all controllers that lack sensors\n\nOr the variable can be a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n0xAAAA/0xBBBB,0xCCCC/0xDDDD\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint should be set before a gamepad is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT",
    short_name: "GDK_TEXTINPUT_DEFAULT_TEXT",
    value: crate::hints::SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT,
    doc: Some(
        "This variable sets the default text of the TextInput window on GDK\nplatforms.\n\nThis hint is available only if SDL_GDK_TEXTINPUT defined.\n\nThis hint should be set before calling [`SDL_StartTextInput()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GDK_TEXTINPUT_DESCRIPTION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GDK_TEXTINPUT_DESCRIPTION",
    short_name: "GDK_TEXTINPUT_DESCRIPTION",
    value: crate::hints::SDL_HINT_GDK_TEXTINPUT_DESCRIPTION,
    doc: Some(
        "This variable sets the description of the TextInput window on GDK\nplatforms.\n\nThis hint is available only if SDL_GDK_TEXTINPUT defined.\n\nThis hint should be set before calling [`SDL_StartTextInput()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH",
    short_name: "GDK_TEXTINPUT_MAX_LENGTH",
    value: crate::hints::SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH,
    doc: Some(
        "This variable sets the maximum input length of the TextInput window on GDK\nplatforms.\n\nThe value must be a stringified integer, for example \"10\" to allow for up\nto 10 characters of text input.\n\nThis hint is available only if SDL_GDK_TEXTINPUT defined.\n\nThis hint should be set before calling [`SDL_StartTextInput()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GDK_TEXTINPUT_SCOPE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GDK_TEXTINPUT_SCOPE",
    short_name: "GDK_TEXTINPUT_SCOPE",
    value: crate::hints::SDL_HINT_GDK_TEXTINPUT_SCOPE,
    doc: Some(
        "This variable sets the input scope of the TextInput window on GDK\nplatforms.\n\nSet this hint to change the XGameUiTextEntryInputScope value that will be\npassed to the window creation function. The value must be a stringified\ninteger, for example \"0\" for XGameUiTextEntryInputScope::Default.\n\nThis hint is available only if SDL_GDK_TEXTINPUT defined.\n\nThis hint should be set before calling [`SDL_StartTextInput()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GDK_TEXTINPUT_TITLE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GDK_TEXTINPUT_TITLE",
    short_name: "GDK_TEXTINPUT_TITLE",
    value: crate::hints::SDL_HINT_GDK_TEXTINPUT_TITLE,
    doc: Some(
        "This variable sets the title of the TextInput window on GDK platforms.\n\nThis hint is available only if SDL_GDK_TEXTINPUT defined.\n\nThis hint should be set before calling [`SDL_StartTextInput()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_HIDAPI_LIBUSB: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_HIDAPI_LIBUSB",
    short_name: "HIDAPI_LIBUSB",
    value: crate::hints::SDL_HINT_HIDAPI_LIBUSB,
    doc: Some(
        "A variable to control whether HIDAPI uses libusb for device access.\n\nBy default libusb will only be used for a few devices that require direct\nUSB access, and this can be controlled with\n[`SDL_HINT_HIDAPI_LIBUSB_WHITELIST`].\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI will not use libusb for device access.\n- \"1\": HIDAPI will use libusb for device access if available. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_HIDAPI_LIBUSB_WHITELIST: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_HIDAPI_LIBUSB_WHITELIST",
    short_name: "HIDAPI_LIBUSB_WHITELIST",
    value: crate::hints::SDL_HINT_HIDAPI_LIBUSB_WHITELIST,
    doc: Some(
        "A variable to control whether HIDAPI uses libusb only for whitelisted\ndevices.\n\nBy default libusb will only be used for a few devices that require direct\nUSB access.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI will use libusb for all device access.\n- \"1\": HIDAPI will use libusb only for whitelisted devices. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_HIDAPI_UDEV: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_HIDAPI_UDEV",
    short_name: "HIDAPI_UDEV",
    value: crate::hints::SDL_HINT_HIDAPI_UDEV,
    doc: Some(
        "A variable to control whether HIDAPI uses udev for device detection.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI will poll for device changes.\n- \"1\": HIDAPI will use udev for device detection. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_GPU_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_GPU_DRIVER",
    short_name: "GPU_DRIVER",
    value: crate::hints::SDL_HINT_GPU_DRIVER,
    doc: Some(
        "A variable that specifies a GPU backend to use.\n\nBy default, SDL will try all available GPU backends in a reasonable order\nuntil it finds one that can work, but this hint allows the app or user to\nforce a specific target, such as \"direct3d12\" if, say, your hardware\nsupports Vulkan but you want to try using D3D12 instead.\n\nThis hint should be set before any GPU functions are called.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS",
    short_name: "HIDAPI_ENUMERATE_ONLY_CONTROLLERS",
    value: crate::hints::SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS,
    doc: Some(
        "A variable to control whether [`SDL_hid_enumerate()`] enumerates all HID\ndevices or only controllers.\n\nThe variable can be set to the following values:\n\n- \"0\": [`SDL_hid_enumerate()`] will enumerate all HID devices.\n- \"1\": [`SDL_hid_enumerate()`] will only enumerate controllers. (default)\n\nBy default SDL will only enumerate controllers, to reduce risk of hanging\nor crashing on devices with bad drivers and avoiding macOS keyboard capture\npermission prompts.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_HIDAPI_IGNORE_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_HIDAPI_IGNORE_DEVICES",
    short_name: "HIDAPI_IGNORE_DEVICES",
    value: crate::hints::SDL_HINT_HIDAPI_IGNORE_DEVICES,
    doc: Some(
        "A variable containing a list of devices to ignore in [`SDL_hid_enumerate()`].\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nFor example, to ignore the Shanwan DS3 controller and any Valve controller,\nyou might use the string \"0x2563/0x0523,0x28de/0x0000\"\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_IME_IMPLEMENTED_UI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_IME_IMPLEMENTED_UI",
    short_name: "IME_IMPLEMENTED_UI",
    value: crate::hints::SDL_HINT_IME_IMPLEMENTED_UI,
    doc: Some(
        "A variable describing what IME UI elements the application can display.\n\nBy default IME UI is handled using native components by the OS where\npossible, however this can interfere with or not be visible when exclusive\nfullscreen mode is used.\n\nThe variable can be set to a comma separated list containing the following\nitems:\n\n- \"none\" or \"0\": The application can't render any IME elements, and native\nUI should be used. (default)\n- \"composition\": The application handles [`SDL_EVENT_TEXT_EDITING`] events and\ncan render the composition text.\n- \"candidates\": The application handles [`SDL_EVENT_TEXT_EDITING_CANDIDATES`]\nand can render the candidate list.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_IOS_HIDE_HOME_INDICATOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_IOS_HIDE_HOME_INDICATOR",
    short_name: "IOS_HIDE_HOME_INDICATOR",
    value: crate::hints::SDL_HINT_IOS_HIDE_HOME_INDICATOR,
    doc: Some(
        "A variable controlling whether the home indicator bar on iPhone X and later\nshould be hidden.\n\nThe variable can be set to the following values:\n\n- \"0\": The indicator bar is not hidden. (default for windowed applications)\n- \"1\": The indicator bar is hidden and is shown when the screen is touched\n(useful for movie playback applications).\n- \"2\": The indicator bar is dim and the first swipe makes it visible and\nthe second swipe performs the \"home\" action. (default for fullscreen\napplications)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS",
    short_name: "JOYSTICK_ALLOW_BACKGROUND_EVENTS",
    value: crate::hints::SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS,
    doc: Some(
        "A variable that lets you enable joystick (and gamecontroller) events even\nwhen your app is in the background.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable joystick & gamecontroller input events when the application\nis in the background. (default)\n- \"1\": Enable joystick & gamecontroller input events when the application\nis in the background.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES",
    short_name: "JOYSTICK_ARCADESTICK_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES,
    doc: Some(
        "A variable containing a list of arcade stick style controllers.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that are not arcade stick style\ncontrollers.\n\nThis will override [`SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES`] and the built in\ndevice list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_BLACKLIST_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_BLACKLIST_DEVICES",
    short_name: "JOYSTICK_BLACKLIST_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_BLACKLIST_DEVICES,
    doc: Some(
        "A variable containing a list of devices that should not be considered\njoysticks.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_BLACKLIST_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that should be considered\njoysticks.\n\nThis will override [`SDL_HINT_JOYSTICK_BLACKLIST_DEVICES`] and the built in\ndevice list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_DEVICE",
    short_name: "JOYSTICK_DEVICE",
    value: crate::hints::SDL_HINT_JOYSTICK_DEVICE,
    doc: Some(
        "A variable containing a comma separated list of devices to open as\njoysticks.\n\nThis variable is currently only used by the Linux joystick driver.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ENHANCED_REPORTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ENHANCED_REPORTS",
    short_name: "JOYSTICK_ENHANCED_REPORTS",
    value: crate::hints::SDL_HINT_JOYSTICK_ENHANCED_REPORTS,
    doc: Some(
        "A variable controlling whether enhanced reports should be used for\ncontrollers when using the HIDAPI driver.\n\nEnhanced reports allow rumble and effects on Bluetooth PlayStation\ncontrollers and gyro on Nintendo Switch controllers, but break Windows\nDirectInput for other applications that don't use SDL.\n\nOnce enhanced reports are enabled, they can't be disabled on PlayStation\ncontrollers without power cycling the controller.\n\nThe variable can be set to the following values:\n\n- \"0\": enhanced reports are not enabled.\n- \"1\": enhanced reports are enabled. (default)\n- \"auto\": enhanced features are advertised to the application, but SDL\ndoesn't change the controller report mode unless the application uses\nthem.\n\nThis hint can be enabled anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES",
    short_name: "JOYSTICK_FLIGHTSTICK_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES,
    doc: Some(
        "A variable containing a list of flightstick style controllers.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of @file, in which case the named file\nwill be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that are not flightstick style\ncontrollers.\n\nThis will override [`SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES`] and the built in\ndevice list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_GAMEINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_GAMEINPUT",
    short_name: "JOYSTICK_GAMEINPUT",
    value: crate::hints::SDL_HINT_JOYSTICK_GAMEINPUT,
    doc: Some(
        "A variable controlling whether GameInput should be used for controller\nhandling on Windows.\n\nThe variable can be set to the following values:\n\n- \"0\": GameInput is not used.\n- \"1\": GameInput is used.\n\nThe default is \"1\" on GDK platforms, and \"0\" otherwise.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_GAMECUBE_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_GAMECUBE_DEVICES",
    short_name: "JOYSTICK_GAMECUBE_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_GAMECUBE_DEVICES,
    doc: Some(
        "A variable containing a list of devices known to have a GameCube form\nfactor.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_GAMECUBE_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices known not to have a GameCube form\nfactor.\n\nThis will override [`SDL_HINT_JOYSTICK_GAMECUBE_DEVICES`] and the built in\ndevice list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI",
    short_name: "JOYSTICK_HIDAPI",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI,
    doc: Some(
        "A variable controlling whether the HIDAPI joystick drivers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI drivers are not used.\n- \"1\": HIDAPI drivers are used. (default)\n\nThis variable is the default for all drivers, but can be overridden by the\nhints for specific drivers below.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS",
    short_name: "JOYSTICK_HIDAPI_COMBINE_JOY_CONS",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS,
    doc: Some(
        "A variable controlling whether Nintendo Switch Joy-Con controllers will be\ncombined into a single Pro-like controller when using the HIDAPI driver.\n\nThe variable can be set to the following values:\n\n- \"0\": Left and right Joy-Con controllers will not be combined and each\nwill be a mini-gamepad.\n- \"1\": Left and right Joy-Con controllers will be combined into a single\ncontroller. (default)\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE",
    short_name: "JOYSTICK_HIDAPI_GAMECUBE",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo GameCube\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE",
    short_name: "JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE,
    doc: Some(
        "A variable controlling whether rumble is used to implement the GameCube\ncontroller's 3 rumble modes, Stop(0), Rumble(1), and StopHard(2).\n\nThis is useful for applications that need full compatibility for things\nlike ADSR envelopes. - Stop is implemented by setting low_frequency_rumble\nto 0 and high_frequency_rumble >0 - Rumble is both at any arbitrary value -\nStopHard is implemented by setting both low_frequency_rumble and\nhigh_frequency_rumble to 0\n\nThe variable can be set to the following values:\n\n- \"0\": Normal rumble behavior is behavior is used. (default)\n- \"1\": Proper GameCube controller rumble behavior is used.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS",
    short_name: "JOYSTICK_HIDAPI_JOY_CONS",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo Switch\nJoy-Cons should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED",
    short_name: "JOYSTICK_HIDAPI_JOYCON_HOME_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED,
    doc: Some(
        "A variable controlling whether the Home button LED should be turned on when\na Nintendo Switch Joy-Con controller is opened.\n\nThe variable can be set to the following values:\n\n- \"0\": home button LED is turned off\n- \"1\": home button LED is turned on\n\nBy default the Home button LED state is not changed. This hint can also be\nset to a floating point value between 0.0 and 1.0 which controls the\nbrightness of the Home button LED.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_LUNA: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_LUNA",
    short_name: "JOYSTICK_HIDAPI_LUNA",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_LUNA,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Amazon Luna\ncontrollers connected via Bluetooth should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC",
    short_name: "JOYSTICK_HIDAPI_NINTENDO_CLASSIC",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo Online\nclassic controllers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS3: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS3",
    short_name: "JOYSTICK_HIDAPI_PS3",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS3,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for PS3 controllers should\nbe used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`] on macOS, and \"0\" on\nother platforms.\n\nFor official Sony driver (sixaxis.sys) use\n[`SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER`]. See\n<https://github.com/ViGEm/DsHidMini> for an alternative driver on Windows.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER",
    short_name: "JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER,
    doc: Some(
        "A variable controlling whether the Sony driver (sixaxis.sys) for PS3\ncontrollers (Sixaxis/DualShock 3) should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": Sony driver (sixaxis.sys) is not used.\n- \"1\": Sony driver (sixaxis.sys) is used.\n\nThe default value is 0.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS4: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS4",
    short_name: "JOYSTICK_HIDAPI_PS4",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS4,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for PS4 controllers should\nbe used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL",
    short_name: "JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL,
    doc: Some(
        "A variable controlling the update rate of the PS4 controller over Bluetooth\nwhen using the HIDAPI driver.\n\nThis defaults to 4 ms, to match the behavior over USB, and to be more\nfriendly to other Bluetooth devices and older Bluetooth hardware on the\ncomputer. It can be set to \"1\" (1000Hz), \"2\" (500Hz) and \"4\" (250Hz)\n\nThis hint can be set anytime, but only takes effect when extended input\nreports are enabled.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS5: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS5",
    short_name: "JOYSTICK_HIDAPI_PS5",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS5,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for PS5 controllers should\nbe used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED",
    short_name: "JOYSTICK_HIDAPI_PS5_PLAYER_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED,
    doc: Some(
        "A variable controlling whether the player LEDs should be lit to indicate\nwhich player is associated with a PS5 controller.\n\nThe variable can be set to the following values:\n\n- \"0\": player LEDs are not enabled.\n- \"1\": player LEDs are enabled. (default)\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SHIELD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SHIELD",
    short_name: "JOYSTICK_HIDAPI_SHIELD",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SHIELD,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for NVIDIA SHIELD\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_STADIA: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_STADIA",
    short_name: "JOYSTICK_HIDAPI_STADIA",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_STADIA,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Google Stadia\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_STEAM: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM",
    short_name: "JOYSTICK_HIDAPI_STEAM",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_STEAM,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Bluetooth Steam\nControllers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used. (default)\n- \"1\": HIDAPI driver is used for Steam Controllers, which requires\nBluetooth access and may prompt the user for permission on iOS and\nAndroid.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED",
    short_name: "JOYSTICK_HIDAPI_STEAM_HOME_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED,
    doc: Some(
        "A variable controlling whether the Steam button LED should be turned on\nwhen a Steam controller is opened.\n\nThe variable can be set to the following values:\n\n- \"0\": Steam button LED is turned off.\n- \"1\": Steam button LED is turned on.\n\nBy default the Steam button LED state is not changed. This hint can also be\nset to a floating point value between 0.0 and 1.0 which controls the\nbrightness of the Steam button LED.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK",
    short_name: "JOYSTICK_HIDAPI_STEAMDECK",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for the Steam Deck builtin\ncontroller should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI",
    short_name: "JOYSTICK_HIDAPI_STEAM_HORI",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for HORI licensed Steam\ncontrollers should be used.\n\nThis variable can be set to the following values: \"0\" - HIDAPI driver is\nnot used \"1\" - HIDAPI driver is used\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_LG4FF: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_LG4FF",
    short_name: "JOYSTICK_HIDAPI_LG4FF",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_LG4FF,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for some Logitech wheels\nshould be used.\n\nThis variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used\n- \"1\": HIDAPI driver is used\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_8BITDO: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_8BITDO",
    short_name: "JOYSTICK_HIDAPI_8BITDO",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_8BITDO,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for 8BitDo controllers\nshould be used.\n\nThis variable can be set to the following values:\n\n\"0\" - HIDAPI driver is not used. \"1\" - HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SINPUT",
    short_name: "JOYSTICK_HIDAPI_SINPUT",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SINPUT,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for SInput controllers\nshould be used.\n\nMore info - <https://github.com/HandHeldLegend/SInput-HID>\n\nThis variable can be set to the following values:\n\n\"0\" - HIDAPI driver is not used. \"1\" - HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_ZUIKI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_ZUIKI",
    short_name: "JOYSTICK_HIDAPI_ZUIKI",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_ZUIKI,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for ZUIKI controllers\nshould be used.\n\nThis variable can be set to the following values:\n\n\"0\" - HIDAPI driver is not used. \"1\" - HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_FLYDIGI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_FLYDIGI",
    short_name: "JOYSTICK_HIDAPI_FLYDIGI",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_FLYDIGI,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Flydigi controllers\nshould be used.\n\nThis variable can be set to the following values:\n\n\"0\" - HIDAPI driver is not used. \"1\" - HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`]\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SWITCH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH",
    short_name: "JOYSTICK_HIDAPI_SWITCH",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SWITCH,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo Switch\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED",
    short_name: "JOYSTICK_HIDAPI_SWITCH_HOME_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED,
    doc: Some(
        "A variable controlling whether the Home button LED should be turned on when\na Nintendo Switch Pro controller is opened.\n\nThe variable can be set to the following values:\n\n- \"0\": Home button LED is turned off.\n- \"1\": Home button LED is turned on.\n\nBy default the Home button LED state is not changed. This hint can also be\nset to a floating point value between 0.0 and 1.0 which controls the\nbrightness of the Home button LED.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED",
    short_name: "JOYSTICK_HIDAPI_SWITCH_PLAYER_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED,
    doc: Some(
        "A variable controlling whether the player LEDs should be lit to indicate\nwhich player is associated with a Nintendo Switch controller.\n\nThe variable can be set to the following values:\n\n- \"0\": Player LEDs are not enabled.\n- \"1\": Player LEDs are enabled. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_SWITCH2: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH2",
    short_name: "JOYSTICK_HIDAPI_SWITCH2",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_SWITCH2,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo Switch 2\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS",
    short_name: "JOYSTICK_HIDAPI_VERTICAL_JOY_CONS",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS,
    doc: Some(
        "A variable controlling whether Nintendo Switch Joy-Con controllers will be\nin vertical mode when using the HIDAPI driver.\n\nThe variable can be set to the following values:\n\n- \"0\": Left and right Joy-Con controllers will not be in vertical mode.\n(default)\n- \"1\": Left and right Joy-Con controllers will be in vertical mode.\n\nThis hint should be set before opening a Joy-Con controller.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_WII: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_WII",
    short_name: "JOYSTICK_HIDAPI_WII",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_WII,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for Nintendo Wii and Wii U\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThis driver doesn't work with the dolphinbar, so the default is false for\nnow.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED",
    short_name: "JOYSTICK_HIDAPI_WII_PLAYER_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED,
    doc: Some(
        "A variable controlling whether the player LEDs should be lit to indicate\nwhich player is associated with a Wii controller.\n\nThe variable can be set to the following values:\n\n- \"0\": Player LEDs are not enabled.\n- \"1\": Player LEDs are enabled. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX",
    short_name: "JOYSTICK_HIDAPI_XBOX",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for XBox controllers\nshould be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is \"0\" on Windows, otherwise the value of\n[`SDL_HINT_JOYSTICK_HIDAPI`]\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX_360: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360",
    short_name: "JOYSTICK_HIDAPI_XBOX_360",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX_360,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for XBox 360 controllers\nshould be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX`]\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED",
    short_name: "JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED,
    doc: Some(
        "A variable controlling whether the player LEDs should be lit to indicate\nwhich player is associated with an Xbox 360 controller.\n\nThe variable can be set to the following values:\n\n- \"0\": Player LEDs are not enabled.\n- \"1\": Player LEDs are enabled. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS",
    short_name: "JOYSTICK_HIDAPI_XBOX_360_WIRELESS",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for XBox 360 wireless\ncontrollers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX_360`]\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE",
    short_name: "JOYSTICK_HIDAPI_XBOX_ONE",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE,
    doc: Some(
        "A variable controlling whether the HIDAPI driver for XBox One controllers\nshould be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED",
    short_name: "JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED,
    doc: Some(
        "A variable controlling whether the Home button LED should be turned on when\nan Xbox One controller is opened.\n\nThe variable can be set to the following values:\n\n- \"0\": Home button LED is turned off.\n- \"1\": Home button LED is turned on.\n\nBy default the Home button LED state is not changed. This hint can also be\nset to a floating point value between 0.0 and 1.0 which controls the\nbrightness of the Home button LED. The default brightness is 0.4.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_GIP: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_GIP",
    short_name: "JOYSTICK_HIDAPI_GIP",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_GIP,
    doc: Some(
        "A variable controlling whether the new HIDAPI driver for wired Xbox One\n(GIP) controllers should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": HIDAPI driver is not used.\n- \"1\": HIDAPI driver is used.\n\nThe default is the value of [`SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE`].\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA",
    short_name: "JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA",
    value: crate::hints::SDL_HINT_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA,
    doc: Some(
        "A variable controlling whether the new HIDAPI driver for wired Xbox One\n(GIP) controllers should reset the controller if it can't get the metadata\nfrom the controller.\n\nThe variable can be set to the following values:\n\n- \"0\": Assume this is a generic controller.\n- \"1\": Reset the controller to get metadata.\n\nBy default the controller is not reset.\n\nThis hint should be set before initializing joysticks and gamepads.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_IOKIT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_IOKIT",
    short_name: "JOYSTICK_IOKIT",
    value: crate::hints::SDL_HINT_JOYSTICK_IOKIT,
    doc: Some(
        "A variable controlling whether IOKit should be used for controller\nhandling.\n\nThe variable can be set to the following values:\n\n- \"0\": IOKit is not used.\n- \"1\": IOKit is used. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_LINUX_CLASSIC: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_LINUX_CLASSIC",
    short_name: "JOYSTICK_LINUX_CLASSIC",
    value: crate::hints::SDL_HINT_JOYSTICK_LINUX_CLASSIC,
    doc: Some(
        "A variable controlling whether to use the classic /dev/input/js* joystick\ninterface or the newer /dev/input/event* joystick interface on Linux.\n\nThe variable can be set to the following values:\n\n- \"0\": Use /dev/input/event* (default)\n- \"1\": Use /dev/input/js*\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_LINUX_DEADZONES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_LINUX_DEADZONES",
    short_name: "JOYSTICK_LINUX_DEADZONES",
    value: crate::hints::SDL_HINT_JOYSTICK_LINUX_DEADZONES,
    doc: Some(
        "A variable controlling whether joysticks on Linux adhere to their\nHID-defined deadzones or return unfiltered values.\n\nThe variable can be set to the following values:\n\n- \"0\": Return unfiltered joystick axis values. (default)\n- \"1\": Return axis values with deadzones taken into account.\n\nThis hint should be set before a controller is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS",
    short_name: "JOYSTICK_LINUX_DIGITAL_HATS",
    value: crate::hints::SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS,
    doc: Some(
        "A variable controlling whether joysticks on Linux will always treat 'hat'\naxis inputs (ABS_HAT0X - ABS_HAT3Y) as 8-way digital hats without checking\nwhether they may be analog.\n\nThe variable can be set to the following values:\n\n- \"0\": Only map hat axis inputs to digital hat outputs if the input axes\nappear to actually be digital. (default)\n- \"1\": Always handle the input axes numbered ABS_HAT0X to ABS_HAT3Y as\ndigital hats.\n\nThis hint should be set before a controller is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES",
    short_name: "JOYSTICK_LINUX_HAT_DEADZONES",
    value: crate::hints::SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES,
    doc: Some(
        "A variable controlling whether digital hats on Linux will apply deadzones\nto their underlying input axes or use unfiltered values.\n\nThe variable can be set to the following values:\n\n- \"0\": Return digital hat values based on unfiltered input axis values.\n- \"1\": Return digital hat values with deadzones on the input axes taken\ninto account. (default)\n\nThis hint should be set before a controller is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_MFI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_MFI",
    short_name: "JOYSTICK_MFI",
    value: crate::hints::SDL_HINT_JOYSTICK_MFI,
    doc: Some(
        "A variable controlling whether GCController should be used for controller\nhandling.\n\nThe variable can be set to the following values:\n\n- \"0\": GCController is not used.\n- \"1\": GCController is used. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_RAWINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_RAWINPUT",
    short_name: "JOYSTICK_RAWINPUT",
    value: crate::hints::SDL_HINT_JOYSTICK_RAWINPUT,
    doc: Some(
        "A variable controlling whether the RAWINPUT joystick drivers should be used\nfor better handling XInput-capable devices.\n\nThe variable can be set to the following values:\n\n- \"0\": RAWINPUT drivers are not used. (default)\n- \"1\": RAWINPUT drivers are used.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT",
    short_name: "JOYSTICK_RAWINPUT_CORRELATE_XINPUT",
    value: crate::hints::SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT,
    doc: Some(
        "A variable controlling whether the RAWINPUT driver should pull correlated\ndata from XInput.\n\nThe variable can be set to the following values:\n\n- \"0\": RAWINPUT driver will only use data from raw input APIs.\n- \"1\": RAWINPUT driver will also pull data from XInput and\nWindows.Gaming.Input, providing better trigger axes, guide button\npresses, and rumble support for Xbox controllers. (default)\n\nThis hint should be set before a gamepad is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ROG_CHAKRAM: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ROG_CHAKRAM",
    short_name: "JOYSTICK_ROG_CHAKRAM",
    value: crate::hints::SDL_HINT_JOYSTICK_ROG_CHAKRAM,
    doc: Some(
        "A variable controlling whether the ROG Chakram mice should show up as\njoysticks.\n\nThe variable can be set to the following values:\n\n- \"0\": ROG Chakram mice do not show up as joysticks. (default)\n- \"1\": ROG Chakram mice show up as joysticks.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_THREAD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_THREAD",
    short_name: "JOYSTICK_THREAD",
    value: crate::hints::SDL_HINT_JOYSTICK_THREAD,
    doc: Some(
        "A variable controlling whether a separate thread should be used for\nhandling joystick detection and raw input messages on Windows.\n\nThe variable can be set to the following values:\n\n- \"0\": A separate thread is not used.\n- \"1\": A separate thread is used for handling raw input messages. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_THROTTLE_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_THROTTLE_DEVICES",
    short_name: "JOYSTICK_THROTTLE_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_THROTTLE_DEVICES,
    doc: Some(
        "A variable containing a list of throttle style controllers.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_THROTTLE_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that are not throttle style\ncontrollers.\n\nThis will override [`SDL_HINT_JOYSTICK_THROTTLE_DEVICES`] and the built in\ndevice list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_WGI: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_WGI",
    short_name: "JOYSTICK_WGI",
    value: crate::hints::SDL_HINT_JOYSTICK_WGI,
    doc: Some(
        "A variable controlling whether Windows.Gaming.Input should be used for\ncontroller handling.\n\nThe variable can be set to the following values:\n\n- \"0\": WGI is not used. (default)\n- \"1\": WGI is used.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_WHEEL_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_WHEEL_DEVICES",
    short_name: "JOYSTICK_WHEEL_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_WHEEL_DEVICES,
    doc: Some(
        "A variable containing a list of wheel style controllers.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED",
    short_name: "JOYSTICK_WHEEL_DEVICES_EXCLUDED",
    value: crate::hints::SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that are not wheel style\ncontrollers.\n\nThis will override [`SDL_HINT_JOYSTICK_WHEEL_DEVICES`] and the built in device\nlist.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES",
    short_name: "JOYSTICK_ZERO_CENTERED_DEVICES",
    value: crate::hints::SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES,
    doc: Some(
        "A variable containing a list of devices known to have all axes centered at\nzero.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint should be set before a controller is opened.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_JOYSTICK_HAPTIC_AXES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_JOYSTICK_HAPTIC_AXES",
    short_name: "JOYSTICK_HAPTIC_AXES",
    value: crate::hints::SDL_HINT_JOYSTICK_HAPTIC_AXES,
    doc: Some(
        "A variable containing a list of devices and their desired number of haptic\n(force feedback) enabled axis.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form plus the number of desired axes, e.g.\n\n`0xAAAA/0xBBBB/1,0xCCCC/0xDDDD/3`\n\nThis hint supports a \"wildcard\" device that will set the number of haptic\naxes on all initialized haptic devices which were not defined explicitly in\nthis hint.\n\n`0xFFFF/0xFFFF/1`\n\nThis hint should be set before a controller is opened. The number of haptic\naxes won't exceed the number of real axes found on the device.\n\n## Availability\nThis hint is available since SDL 3.2.5.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 5)),
};
pub const METADATA_SDL_HINT_KEYCODE_OPTIONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_KEYCODE_OPTIONS",
    short_name: "KEYCODE_OPTIONS",
    value: crate::hints::SDL_HINT_KEYCODE_OPTIONS,
    doc: Some(
        "A variable that controls keycode representation in keyboard events.\n\nThis variable is a comma separated set of options for translating keycodes\nin events:\n\n- \"none\": Keycode options are cleared, this overrides other options.\n- \"hide_numpad\": The numpad keysyms will be translated into their\nnon-numpad versions based on the current NumLock state. For example,\nSDLK_KP_4 would become SDLK_4 if [`SDL_KMOD_NUM`] is set in the event\nmodifiers, and SDLK_LEFT if it is unset.\n- \"french_numbers\": The number row on French keyboards is inverted, so\npressing the 1 key would yield the keycode SDLK_1, or '1', instead of\nSDLK_AMPERSAND, or '&'\n- \"latin_letters\": For keyboards using non-Latin letters, such as Russian\nor Thai, the letter keys generate keycodes as though it had an English\nQWERTY layout. e.g. pressing the key associated with [`SDL_SCANCODE_A`] on a\nRussian keyboard would yield 'a' instead of a Cyrillic letter.\n\nThe default value for this hint is \"french_numbers,latin_letters\"\n\nSome platforms like Emscripten only provide modified keycodes and the\noptions are not used.\n\nThese options do not affect the return value of [`SDL_GetKeyFromScancode()`] or\n[`SDL_GetScancodeFromKey()`], they just apply to the keycode included in key\nevents.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_KMSDRM_DEVICE_INDEX: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_KMSDRM_DEVICE_INDEX",
    short_name: "KMSDRM_DEVICE_INDEX",
    value: crate::hints::SDL_HINT_KMSDRM_DEVICE_INDEX,
    doc: Some(
        "A variable that controls what KMSDRM device to use.\n\nSDL might open something like \"/dev/dri/cardNN\" to access KMSDRM\nfunctionality, where \"NN\" is a device index number. SDL makes a guess at\nthe best index to use (usually zero), but the app or user can set this hint\nto a number between 0 and 99 to force selection.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER",
    short_name: "KMSDRM_REQUIRE_DRM_MASTER",
    value: crate::hints::SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER,
    doc: Some(
        "A variable that controls whether SDL requires DRM master access in order to\ninitialize the KMSDRM video backend.\n\nThe DRM subsystem has a concept of a \"DRM master\" which is a DRM client\nthat has the ability to set planes, set cursor, etc. When SDL is DRM\nmaster, it can draw to the screen using the SDL rendering APIs. Without DRM\nmaster, SDL is still able to process input and query attributes of attached\ndisplays, but it cannot change display state or draw to the screen\ndirectly.\n\nIn some cases, it can be useful to have the KMSDRM backend even if it\ncannot be used for rendering. An app may want to use SDL for input\nprocessing while using another rendering API (such as an MMAL overlay on\nRaspberry Pi) or using its own code to render to DRM overlays that SDL\ndoesn't support.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will allow usage of the KMSDRM backend without DRM master.\n- \"1\": SDL Will require DRM master to use the KMSDRM backend. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_KMSDRM_ATOMIC: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_KMSDRM_ATOMIC",
    short_name: "KMSDRM_ATOMIC",
    value: crate::hints::SDL_HINT_KMSDRM_ATOMIC,
    doc: Some(
        "A variable that controls whether KMSDRM will use \"atomic\" functionality.\n\nThe KMSDRM backend can use atomic commits, if both DRM_CLIENT_CAP_ATOMIC\nand DRM_CLIENT_CAP_UNIVERSAL_PLANES is supported by the system. As of SDL\n3.4.0, it will favor this functionality, but in case this doesn't work well\non a given system or other surprises, this hint can be used to disable it.\n\nThis hint can not enable the functionality if it isn't available.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will not use the KMSDRM \"atomic\" functionality.\n- \"1\": SDL will allow usage of the KMSDRM \"atomic\" functionality. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_LOGGING: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_LOGGING",
    short_name: "LOGGING",
    value: crate::hints::SDL_HINT_LOGGING,
    doc: Some(
        "A variable controlling the default SDL log levels.\n\nThis variable is a comma separated set of category=level tokens that define\nthe default logging levels for SDL applications.\n\nThe category can be a numeric category, one of \"app\", \"error\", \"assert\",\n\"system\", \"audio\", \"video\", \"render\", \"input\", \"test\", or `*` for any\nunspecified category.\n\nThe level can be a numeric level, one of \"verbose\", \"debug\", \"info\",\n\"warn\", \"error\", \"critical\", or \"quiet\" to disable that category.\n\nYou can omit the category if you want to set the logging level for all\ncategories.\n\nIf this hint isn't set, the default log levels are equivalent to:\n\n`app=info,assert=warn,test=verbose,*=error`\n\nIf the `DEBUG_INVOCATION` environment variable is set to \"1\", the default\nlog levels are equivalent to:\n\n`assert=warn,test=verbose,*=debug`\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_BACKGROUND_APP: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_BACKGROUND_APP",
    short_name: "MAC_BACKGROUND_APP",
    value: crate::hints::SDL_HINT_MAC_BACKGROUND_APP,
    doc: Some(
        "A variable controlling whether to force the application to become the\nforeground process when launched on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": The application is brought to the foreground when launched.\n(default)\n- \"1\": The application may remain in the background when launched.\n\nThis hint needs to be set before [`SDL_Init()`].\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK",
    short_name: "MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK",
    value: crate::hints::SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK,
    doc: Some(
        "A variable that determines whether Ctrl+Click should generate a right-click\nevent on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": Ctrl+Click does not generate a right mouse button click event.\n(default)\n- \"1\": Ctrl+Click generated a right mouse button click event.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH",
    short_name: "MAC_OPENGL_ASYNC_DISPATCH",
    value: crate::hints::SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH,
    doc: Some(
        "A variable controlling whether dispatching OpenGL context updates should\nblock the dispatching thread until the main thread finishes processing on\nmacOS.\n\nThe variable can be set to the following values:\n\n- \"0\": Dispatching OpenGL context updates will block the dispatching thread\nuntil the main thread finishes processing. (default)\n- \"1\": Dispatching OpenGL context updates will allow the dispatching thread\nto continue execution.\n\nGenerally you want the default, but if you have OpenGL code in a background\nthread on a Mac, and the main thread hangs because it's waiting for that\nbackground thread, but that background thread is also hanging because it's\nwaiting for the main thread to do an update, this might fix your issue.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_OPTION_AS_ALT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_OPTION_AS_ALT",
    short_name: "MAC_OPTION_AS_ALT",
    value: crate::hints::SDL_HINT_MAC_OPTION_AS_ALT,
    doc: Some(
        "A variable controlling whether the Option key on macOS should be remapped\nto act as the Alt key.\n\nThe variable can be set to the following values:\n\n- \"none\": The Option key is not remapped to Alt. (default)\n- \"only_left\": Only the left Option key is remapped to Alt.\n- \"only_right\": Only the right Option key is remapped to Alt.\n- \"both\": Both Option keys are remapped to Alt.\n\nThis will prevent the triggering of key compositions that rely on the\nOption key, but will still send the Alt modifier for keyboard events. In\nthe case that both Alt and Option are pressed, the Option key will be\nignored. This is particularly useful for applications like terminal\nemulators and graphical user interfaces (GUIs) that rely on Alt key\nfunctionality for shortcuts or navigation. This does not apply to\n[`SDL_GetKeyFromScancode`] and only has an effect if IME is enabled.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_SCROLL_MOMENTUM: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_SCROLL_MOMENTUM",
    short_name: "MAC_SCROLL_MOMENTUM",
    value: crate::hints::SDL_HINT_MAC_SCROLL_MOMENTUM,
    doc: Some(
        "A variable controlling whether [`SDL_EVENT_MOUSE_WHEEL`] event values will have\nmomentum on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": The mouse wheel events will have no momentum. (default)\n- \"1\": The mouse wheel events will have momentum.\n\nThis hint needs to be set before [`SDL_Init()`].\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MAC_PRESS_AND_HOLD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAC_PRESS_AND_HOLD",
    short_name: "MAC_PRESS_AND_HOLD",
    value: crate::hints::SDL_HINT_MAC_PRESS_AND_HOLD,
    doc: Some(
        "A variable controlling whether holding down a key will repeat the pressed\nkey or open the accents menu on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": Holding a key will open the accents menu for that key.\n- \"1\": Holding a key will repeat the pressed key. (default)\n\nThis hint needs to be set before [`SDL_Init()`].\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_MAIN_CALLBACK_RATE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MAIN_CALLBACK_RATE",
    short_name: "MAIN_CALLBACK_RATE",
    value: crate::hints::SDL_HINT_MAIN_CALLBACK_RATE,
    doc: Some(
        "Request [`SDL_AppIterate()`] be called at a specific rate.\n\nIf this is set to a number, it represents Hz, so \"60\" means try to iterate\n60 times per second. \"0\" means to iterate as fast as possible. Negative\nvalues are illegal, but reserved, in case they are useful in a future\nrevision of SDL.\n\nThere are other strings that have special meaning. If set to \"waitevent\",\n[`SDL_AppIterate`] will not be called until new event(s) have arrived (and been\nprocessed by [`SDL_AppEvent`]). This can be useful for apps that are completely\nidle except in response to input.\n\nOn some platforms, or if you are using [`SDL_main`] instead of [`SDL_AppIterate`],\nthis hint is ignored. When the hint can be used, it is allowed to be\nchanged at any time.\n\nThis defaults to 0, and specifying NULL for the hint's value will restore\nthe default.\n\nThis doesn't have to be an integer value. For example, \"59.94\" won't be\nrounded to an integer rate; the digits after the decimal are actually\nrespected.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_AUTO_CAPTURE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_AUTO_CAPTURE",
    short_name: "MOUSE_AUTO_CAPTURE",
    value: crate::hints::SDL_HINT_MOUSE_AUTO_CAPTURE,
    doc: Some(
        "A variable controlling whether the mouse is captured while mouse buttons\nare pressed.\n\nThe variable can be set to the following values:\n\n- \"0\": The mouse is not captured while mouse buttons are pressed.\n- \"1\": The mouse is captured while mouse buttons are pressed.\n\nBy default the mouse is captured while mouse buttons are pressed so if the\nmouse is dragged outside the window, the application continues to receive\nmouse events until the button is released.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS",
    short_name: "MOUSE_DOUBLE_CLICK_RADIUS",
    value: crate::hints::SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS,
    doc: Some(
        "A variable setting the double click radius, in pixels.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_DOUBLE_CLICK_TIME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_DOUBLE_CLICK_TIME",
    short_name: "MOUSE_DOUBLE_CLICK_TIME",
    value: crate::hints::SDL_HINT_MOUSE_DOUBLE_CLICK_TIME,
    doc: Some(
        "A variable setting the double click time, in milliseconds.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR",
    short_name: "MOUSE_DEFAULT_SYSTEM_CURSOR",
    value: crate::hints::SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR,
    doc: Some(
        "A variable setting which system cursor to use as the default cursor.\n\nThis should be an integer corresponding to the [`SDL_SystemCursor`] enum. The\ndefault value is zero ([`SDL_SYSTEM_CURSOR_DEFAULT`]).\n\nThis hint needs to be set before [`SDL_Init()`].\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE",
    short_name: "MOUSE_EMULATE_WARP_WITH_RELATIVE",
    value: crate::hints::SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE,
    doc: Some(
        "A variable controlling whether warping a hidden mouse cursor will activate\nrelative mouse mode.\n\nWhen this hint is set, the mouse cursor is hidden, and multiple warps to\nthe window center occur within a short time period, SDL will emulate mouse\nwarps using relative mouse mode. This can provide smoother and more\nreliable mouse motion for some older games, which continuously calculate\nthe distance traveled by the mouse pointer and warp it back to the center\nof the window, rather than using relative mouse motion.\n\nNote that relative mouse mode may have different mouse acceleration\nbehavior than pointer warps.\n\nIf your application needs to repeatedly warp the hidden mouse cursor at a\nhigh-frequency for other purposes, it should disable this hint.\n\nThe variable can be set to the following values:\n\n- \"0\": Attempts to warp the mouse will always be made.\n- \"1\": Some mouse warps will be emulated by forcing relative mouse mode.\n(default)\n\nIf not set, this is automatically enabled unless an application uses\nrelative mouse mode directly.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH",
    short_name: "MOUSE_FOCUS_CLICKTHROUGH",
    value: crate::hints::SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH,
    doc: Some(
        "Allow mouse click events when clicking to focus an SDL window.\n\nThe variable can be set to the following values:\n\n- \"0\": Ignore mouse clicks that activate a window. (default)\n- \"1\": Generate events for mouse clicks that activate a window.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_NORMAL_SPEED_SCALE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_NORMAL_SPEED_SCALE",
    short_name: "MOUSE_NORMAL_SPEED_SCALE",
    value: crate::hints::SDL_HINT_MOUSE_NORMAL_SPEED_SCALE,
    doc: Some(
        "A variable setting the speed scale for mouse motion, in floating point,\nwhen the mouse is not in relative mode.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_RELATIVE_MODE_CENTER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_RELATIVE_MODE_CENTER",
    short_name: "MOUSE_RELATIVE_MODE_CENTER",
    value: crate::hints::SDL_HINT_MOUSE_RELATIVE_MODE_CENTER,
    doc: Some(
        "A variable controlling whether relative mouse mode constrains the mouse to\nthe center of the window.\n\nConstraining to the center of the window works better for FPS games and\nwhen the application is running over RDP. Constraining to the whole window\nworks better for 2D games and increases the chance that the mouse will be\nin the correct position when using high DPI mice.\n\nThe variable can be set to the following values:\n\n- \"0\": Relative mouse mode constrains the mouse to the window.\n- \"1\": Relative mouse mode constrains the mouse to the center of the\nwindow. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE",
    short_name: "MOUSE_RELATIVE_SPEED_SCALE",
    value: crate::hints::SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE,
    doc: Some(
        "A variable setting the scale for mouse motion, in floating point, when the\nmouse is in relative mode.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE",
    short_name: "MOUSE_RELATIVE_SYSTEM_SCALE",
    value: crate::hints::SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE,
    doc: Some(
        "A variable controlling whether the system mouse acceleration curve is used\nfor relative mouse motion.\n\nThe variable can be set to the following values:\n\n- \"0\": Relative mouse motion will be unscaled. (default)\n- \"1\": Relative mouse motion will be scaled using the system mouse\nacceleration curve.\n\nIf [`SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE`] is set, that will be applied after\nsystem speed scale.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_RELATIVE_WARP_MOTION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_RELATIVE_WARP_MOTION",
    short_name: "MOUSE_RELATIVE_WARP_MOTION",
    value: crate::hints::SDL_HINT_MOUSE_RELATIVE_WARP_MOTION,
    doc: Some(
        "A variable controlling whether a motion event should be generated for mouse\nwarping in relative mode.\n\nThe variable can be set to the following values:\n\n- \"0\": Warping the mouse will not generate a motion event in relative mode\n- \"1\": Warping the mouse will generate a motion event in relative mode\n\nBy default warping the mouse will not generate motion events in relative\nmode. This avoids the application having to filter out large relative\nmotion due to warping.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE",
    short_name: "MOUSE_RELATIVE_CURSOR_VISIBLE",
    value: crate::hints::SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE,
    doc: Some(
        "A variable controlling whether the hardware cursor stays visible when\nrelative mode is active.\n\nThis variable can be set to the following values:\n\n- \"0\": The cursor will be hidden while relative mode is active (default)\n- \"1\": The cursor will remain visible while relative mode is active\n\nNote that for systems without raw hardware inputs, relative mode is\nimplemented using warping, so the hardware cursor will visibly warp between\nframes if this is enabled on those systems.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MOUSE_TOUCH_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MOUSE_TOUCH_EVENTS",
    short_name: "MOUSE_TOUCH_EVENTS",
    value: crate::hints::SDL_HINT_MOUSE_TOUCH_EVENTS,
    doc: Some(
        "A variable controlling whether mouse events should generate synthetic touch\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": Mouse events will not generate touch events. (default for desktop\nplatforms)\n- \"1\": Mouse events will generate touch events. (default for mobile\nplatforms, such as Android and iOS)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_MUTE_CONSOLE_KEYBOARD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_MUTE_CONSOLE_KEYBOARD",
    short_name: "MUTE_CONSOLE_KEYBOARD",
    value: crate::hints::SDL_HINT_MUTE_CONSOLE_KEYBOARD,
    doc: Some(
        "A variable controlling whether the keyboard should be muted on the console.\n\nNormally the keyboard is muted while SDL applications are running so that\nkeyboard input doesn't show up as key strokes on the console. This hint\nallows you to turn that off for debugging purposes.\n\nThe variable can be set to the following values:\n\n- \"0\": Allow keystrokes to go through to the console.\n- \"1\": Mute keyboard input so it doesn't show up on the console. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_NO_SIGNAL_HANDLERS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_NO_SIGNAL_HANDLERS",
    short_name: "NO_SIGNAL_HANDLERS",
    value: crate::hints::SDL_HINT_NO_SIGNAL_HANDLERS,
    doc: Some(
        "Tell SDL not to catch the SIGINT or SIGTERM signals on POSIX platforms.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will install a SIGINT and SIGTERM handler, and when it catches a\nsignal, convert it into an [`SDL_EVENT_QUIT`] event. (default)\n- \"1\": SDL will not install a signal handler at all.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_OPENGL_LIBRARY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_OPENGL_LIBRARY",
    short_name: "OPENGL_LIBRARY",
    value: crate::hints::SDL_HINT_OPENGL_LIBRARY,
    doc: Some(
        "Specify the OpenGL library to load.\n\nThis hint should be set before creating an OpenGL window or creating an\nOpenGL context. If this hint isn't set, SDL will choose a reasonable\ndefault.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_EGL_LIBRARY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_EGL_LIBRARY",
    short_name: "EGL_LIBRARY",
    value: crate::hints::SDL_HINT_EGL_LIBRARY,
    doc: Some(
        "Specify the EGL library to load.\n\nThis hint should be set before creating an OpenGL window or creating an\nOpenGL context. This hint is only considered if SDL is using EGL to manage\nOpenGL contexts. If this hint isn't set, SDL will choose a reasonable\ndefault.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_OPENGL_ES_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_OPENGL_ES_DRIVER",
    short_name: "OPENGL_ES_DRIVER",
    value: crate::hints::SDL_HINT_OPENGL_ES_DRIVER,
    doc: Some(
        "A variable controlling what driver to use for OpenGL ES contexts.\n\nOn some platforms, currently Windows and X11, OpenGL drivers may support\ncreating contexts with an OpenGL ES profile. By default SDL uses these\nprofiles, when available, otherwise it attempts to load an OpenGL ES\nlibrary, e.g. that provided by the ANGLE project. This variable controls\nwhether SDL follows this default behaviour or will always load an OpenGL ES\nlibrary.\n\nCircumstances where this is useful include - Testing an app with a\nparticular OpenGL ES implementation, e.g ANGLE, or emulator, e.g. those\nfrom ARM, Imagination or Qualcomm. - Resolving OpenGL ES function addresses\nat link time by linking with the OpenGL ES library instead of querying them\nat run time with [`SDL_GL_GetProcAddress()`].\n\nCaution: for an application to work with the default behaviour across\ndifferent OpenGL drivers it must query the OpenGL ES function addresses at\nrun time using [`SDL_GL_GetProcAddress()`].\n\nThis variable is ignored on most platforms because OpenGL ES is native or\nnot supported.\n\nThe variable can be set to the following values:\n\n- \"0\": Use ES profile of OpenGL, if available. (default)\n- \"1\": Load OpenGL ES library using the default library names.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_OPENVR_LIBRARY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_OPENVR_LIBRARY",
    short_name: "OPENVR_LIBRARY",
    value: crate::hints::SDL_HINT_OPENVR_LIBRARY,
    doc: Some(
        "Mechanism to specify openvr_api library location\n\nBy default, when using the OpenVR driver, it will search for the API\nlibrary in the current folder. But, if you wish to use a system API you can\nspecify that by using this hint. This should be the full or relative path\nto a .dll on Windows or .so on Linux.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ORIENTATIONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ORIENTATIONS",
    short_name: "ORIENTATIONS",
    value: crate::hints::SDL_HINT_ORIENTATIONS,
    doc: Some(
        "A variable controlling which orientations are allowed on iOS/Android.\n\nIn some circumstances it is necessary to be able to explicitly control\nwhich UI orientations are allowed.\n\nThis variable is a space delimited list of the following values:\n\n- \"LandscapeLeft\"\n- \"LandscapeRight\"\n- \"Portrait\"\n- \"PortraitUpsideDown\"\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_POLL_SENTINEL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_POLL_SENTINEL",
    short_name: "POLL_SENTINEL",
    value: crate::hints::SDL_HINT_POLL_SENTINEL,
    doc: Some(
        "A variable controlling the use of a sentinel event when polling the event\nqueue.\n\nWhen polling for events, [`SDL_PumpEvents`] is used to gather new events from\ndevices. If a device keeps producing new events between calls to\n[`SDL_PumpEvents`], a poll loop will become stuck until the new events stop.\nThis is most noticeable when moving a high frequency mouse.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable poll sentinels.\n- \"1\": Enable poll sentinels. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_PREFERRED_LOCALES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PREFERRED_LOCALES",
    short_name: "PREFERRED_LOCALES",
    value: crate::hints::SDL_HINT_PREFERRED_LOCALES,
    doc: Some(
        "Override for [`SDL_GetPreferredLocales()`].\n\nIf set, this will be favored over anything the OS might report for the\nuser's preferred locales. Changing this hint at runtime will not generate a\n[`SDL_EVENT_LOCALE_CHANGED`] event (but if you can change the hint, you can\npush your own event, if you want).\n\nThe format of this hint is a comma-separated list of language and locale,\ncombined with an underscore, as is a common format: \"en_GB\". Locale is\noptional: \"en\". So you might have a list like this: \"en_GB,jp,es_PT\"\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE",
    short_name: "QUIT_ON_LAST_WINDOW_CLOSE",
    value: crate::hints::SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE,
    doc: Some(
        "A variable that decides whether to send [`SDL_EVENT_QUIT`] when closing the\nlast window.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will not send an [`SDL_EVENT_QUIT`] event when the last window is\nrequesting to close. Note that in this case, there are still other\nlegitimate reasons one might get an [`SDL_EVENT_QUIT`] event: choosing \"Quit\"\nfrom the macOS menu bar, sending a SIGINT (ctrl-c) on Unix, etc.\n- \"1\": SDL will send a quit event when the last window is requesting to\nclose. (default)\n\nIf there is at least one active system tray icon, [`SDL_EVENT_QUIT`] will\ninstead be sent when both the last window will be closed and the last tray\nicon will be destroyed.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_DIRECT3D_THREADSAFE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_DIRECT3D_THREADSAFE",
    short_name: "RENDER_DIRECT3D_THREADSAFE",
    value: crate::hints::SDL_HINT_RENDER_DIRECT3D_THREADSAFE,
    doc: Some(
        "A variable controlling whether the Direct3D device is initialized for\nthread-safe operations.\n\nThe variable can be set to the following values:\n\n- \"0\": Thread-safety is not enabled. (default)\n- \"1\": Thread-safety is enabled.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_DIRECT3D11_DEBUG: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_DIRECT3D11_DEBUG",
    short_name: "RENDER_DIRECT3D11_DEBUG",
    value: crate::hints::SDL_HINT_RENDER_DIRECT3D11_DEBUG,
    doc: Some(
        "A variable controlling whether to enable Direct3D 11+'s Debug Layer.\n\nThis variable does not have any effect on the Direct3D 9 based renderer.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable Debug Layer use. (default)\n- \"1\": Enable Debug Layer use.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_DIRECT3D11_WARP: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_DIRECT3D11_WARP",
    short_name: "RENDER_DIRECT3D11_WARP",
    value: crate::hints::SDL_HINT_RENDER_DIRECT3D11_WARP,
    doc: Some(
        "A variable controlling whether to use the Direct3D 11 WARP software\nrasterizer.\n\nFor more information, see:\n<https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp>\n\nThe variable can be set to the following values:\n\n- \"0\": Disable WARP rasterizer. (default)\n- \"1\": Enable WARP rasterizer.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_RENDER_VULKAN_DEBUG: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_VULKAN_DEBUG",
    short_name: "RENDER_VULKAN_DEBUG",
    value: crate::hints::SDL_HINT_RENDER_VULKAN_DEBUG,
    doc: Some(
        "A variable controlling whether to enable Vulkan Validation Layers.\n\nThis variable can be set to the following values:\n\n- \"0\": Disable Validation Layer use\n- \"1\": Enable Validation Layer use\n\nBy default, SDL does not use Vulkan Validation Layers.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_GPU_DEBUG: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_GPU_DEBUG",
    short_name: "RENDER_GPU_DEBUG",
    value: crate::hints::SDL_HINT_RENDER_GPU_DEBUG,
    doc: Some(
        "A variable controlling whether to create the GPU device in debug mode.\n\nThis variable can be set to the following values:\n\n- \"0\": Disable debug mode use (default)\n- \"1\": Enable debug mode use\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_GPU_LOW_POWER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_GPU_LOW_POWER",
    short_name: "RENDER_GPU_LOW_POWER",
    value: crate::hints::SDL_HINT_RENDER_GPU_LOW_POWER,
    doc: Some(
        "A variable controlling whether to prefer a low-power GPU on multi-GPU\nsystems.\n\nThis variable can be set to the following values:\n\n- \"0\": Prefer high-performance GPU (default)\n- \"1\": Prefer low-power GPU\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_DRIVER",
    short_name: "RENDER_DRIVER",
    value: crate::hints::SDL_HINT_RENDER_DRIVER,
    doc: Some(
        "A variable specifying which render driver to use.\n\nIf the application doesn't pick a specific renderer to use, this variable\nspecifies the name of the preferred renderer. If the preferred renderer\ncan't be initialized, creating a renderer will fail.\n\nThis variable is case insensitive and can be set to the following values:\n\n- \"direct3d\"\n- \"direct3d11\"\n- \"direct3d12\"\n- \"opengl\"\n- \"opengles2\"\n- \"opengles\"\n- \"metal\"\n- \"vulkan\"\n- \"gpu\"\n- \"software\"\n\nThis hint accepts a comma-separated list of driver names, and each will be\ntried in the order listed when creating a renderer until one succeeds or\nall of them fail.\n\nThe default varies by platform, but it's the first one in the list that is\navailable on the current platform.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_LINE_METHOD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_LINE_METHOD",
    short_name: "RENDER_LINE_METHOD",
    value: crate::hints::SDL_HINT_RENDER_LINE_METHOD,
    doc: Some(
        "A variable controlling how the 2D render API renders lines.\n\nThe variable can be set to the following values:\n\n- \"0\": Use the default line drawing method (Bresenham's line algorithm)\n- \"1\": Use the driver point API using Bresenham's line algorithm (correct,\ndraws many points)\n- \"2\": Use the driver line API (occasionally misses line endpoints based on\nhardware driver quirks\n- \"3\": Use the driver geometry API (correct, draws thicker diagonal lines)\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE",
    short_name: "RENDER_METAL_PREFER_LOW_POWER_DEVICE",
    value: crate::hints::SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE,
    doc: Some(
        "A variable controlling whether the Metal render driver select low power\ndevice over default one.\n\nThe variable can be set to the following values:\n\n- \"0\": Use the preferred OS device. (default)\n- \"1\": Select a low power device.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RENDER_VSYNC: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RENDER_VSYNC",
    short_name: "RENDER_VSYNC",
    value: crate::hints::SDL_HINT_RENDER_VSYNC,
    doc: Some(
        "A variable controlling whether updates to the SDL screen surface should be\nsynchronized with the vertical refresh, to avoid tearing.\n\nThis hint overrides the application preference when creating a renderer.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable vsync. (default)\n- \"1\": Enable vsync.\n\nThis hint should be set before creating a renderer.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_RETURN_KEY_HIDES_IME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RETURN_KEY_HIDES_IME",
    short_name: "RETURN_KEY_HIDES_IME",
    value: crate::hints::SDL_HINT_RETURN_KEY_HIDES_IME,
    doc: Some(
        "A variable to control whether the return key on the soft keyboard should\nhide the soft keyboard on Android and iOS.\n\nThis hint sets the default value of [`SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN`].\n\nThe variable can be set to the following values:\n\n- \"0\": The return key will be handled as a key event. (default)\n- \"1\": The return key will hide the keyboard.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ROG_GAMEPAD_MICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ROG_GAMEPAD_MICE",
    short_name: "ROG_GAMEPAD_MICE",
    value: crate::hints::SDL_HINT_ROG_GAMEPAD_MICE,
    doc: Some(
        "A variable containing a list of ROG gamepad capable mice.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED",
    short_name: "ROG_GAMEPAD_MICE_EXCLUDED",
    value: crate::hints::SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED,
    doc: Some(
        "A variable containing a list of devices that are not ROG gamepad capable\nmice.\n\nThis will override [`SDL_HINT_ROG_GAMEPAD_MICE`] and the built in device list.\n\nThe format of the string is a comma separated list of USB VID/PID pairs in\nhexadecimal form, e.g.\n\n`0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n\nThe variable can also take the form of \"@file\", in which case the named\nfile will be loaded and interpreted as the value of the variable.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_PS2_GS_WIDTH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PS2_GS_WIDTH",
    short_name: "PS2_GS_WIDTH",
    value: crate::hints::SDL_HINT_PS2_GS_WIDTH,
    doc: Some(
        "Variable controlling the width of the PS2's framebuffer in pixels\n\nBy default, this variable is \"640\"\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_PS2_GS_HEIGHT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PS2_GS_HEIGHT",
    short_name: "PS2_GS_HEIGHT",
    value: crate::hints::SDL_HINT_PS2_GS_HEIGHT,
    doc: Some(
        "Variable controlling the height of the PS2's framebuffer in pixels\n\nBy default, this variable is \"448\"\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_PS2_GS_PROGRESSIVE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PS2_GS_PROGRESSIVE",
    short_name: "PS2_GS_PROGRESSIVE",
    value: crate::hints::SDL_HINT_PS2_GS_PROGRESSIVE,
    doc: Some(
        "Variable controlling whether the signal is interlaced or progressive\n\n- \"0\": Image is interlaced. (default)\n- \"1\": Image is progressive\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_PS2_GS_MODE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PS2_GS_MODE",
    short_name: "PS2_GS_MODE",
    value: crate::hints::SDL_HINT_PS2_GS_MODE,
    doc: Some(
        "Variable controlling the video mode of the console\n\n- \"\": Console-native. (default)\n- \"NTSC\": 60hz region\n- \"PAL\": 50hz region\n",
    ),
    available_since: None,
};
pub const METADATA_SDL_HINT_RPI_VIDEO_LAYER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_RPI_VIDEO_LAYER",
    short_name: "RPI_VIDEO_LAYER",
    value: crate::hints::SDL_HINT_RPI_VIDEO_LAYER,
    doc: Some(
        "A variable controlling which Dispmanx layer to use on a Raspberry PI.\n\nAlso known as Z-order. The variable can take a negative or positive value.\nThe default is 10000.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME",
    short_name: "SCREENSAVER_INHIBIT_ACTIVITY_NAME",
    value: crate::hints::SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME,
    doc: Some(
        "Specify an \"activity name\" for screensaver inhibition.\n\nSome platforms, notably Linux desktops, list the applications which are\ninhibiting the screensaver or other power-saving features.\n\nThis hint lets you specify the \"activity name\" sent to the OS when\n[`SDL_DisableScreenSaver()`] is used (or the screensaver is automatically\ndisabled). The contents of this hint are used when the screensaver is\ndisabled. You should use a string that describes what your program is doing\n(and, therefore, why the screensaver is disabled). For example, \"Playing a\ngame\" or \"Watching a video\".\n\nSetting this to \"\" or leaving it unset will have SDL use a reasonable\ndefault: \"Playing a game\" or something similar.\n\nThis hint should be set before calling [`SDL_DisableScreenSaver()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_SHUTDOWN_DBUS_ON_QUIT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_SHUTDOWN_DBUS_ON_QUIT",
    short_name: "SHUTDOWN_DBUS_ON_QUIT",
    value: crate::hints::SDL_HINT_SHUTDOWN_DBUS_ON_QUIT,
    doc: Some(
        "A variable controlling whether SDL calls dbus_shutdown() on quit.\n\nThis is useful as a debug tool to validate memory leaks, but shouldn't ever\nbe set in production applications, as other libraries used by the\napplication might use dbus under the hood and this can cause crashes if\nthey continue after [`SDL_Quit()`].\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will not call dbus_shutdown() on quit. (default)\n- \"1\": SDL will call dbus_shutdown() on quit.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_STORAGE_TITLE_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_STORAGE_TITLE_DRIVER",
    short_name: "STORAGE_TITLE_DRIVER",
    value: crate::hints::SDL_HINT_STORAGE_TITLE_DRIVER,
    doc: Some(
        "A variable that specifies a backend to use for title storage.\n\nBy default, SDL will try all available storage backends in a reasonable\norder until it finds one that can work, but this hint allows the app or\nuser to force a specific target, such as \"pc\" if, say, you are on Steam but\nwant to avoid SteamRemoteStorage for title data.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_STORAGE_USER_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_STORAGE_USER_DRIVER",
    short_name: "STORAGE_USER_DRIVER",
    value: crate::hints::SDL_HINT_STORAGE_USER_DRIVER,
    doc: Some(
        "A variable that specifies a backend to use for user storage.\n\nBy default, SDL will try all available storage backends in a reasonable\norder until it finds one that can work, but this hint allows the app or\nuser to force a specific target, such as \"pc\" if, say, you are on Steam but\nwant to avoid SteamRemoteStorage for user data.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL",
    short_name: "THREAD_FORCE_REALTIME_TIME_CRITICAL",
    value: crate::hints::SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL,
    doc: Some(
        "Specifies whether [`SDL_THREAD_PRIORITY_TIME_CRITICAL`] should be treated as\nrealtime.\n\nOn some platforms, like Linux, a realtime priority thread may be subject to\nrestrictions that require special handling by the application. This hint\nexists to let SDL know that the app is prepared to handle said\nrestrictions.\n\nOn Linux, SDL will apply the following configuration to any thread that\nbecomes realtime:\n\n- The SCHED_RESET_ON_FORK bit will be set on the scheduling policy,\n- An RLIMIT_RTTIME budget will be configured to the rtkit specified limit.\n- Exceeding this limit will result in the kernel sending SIGKILL to the\napp, refer to the man pages for more information.\n\nThe variable can be set to the following values:\n\n- \"0\": default platform specific behaviour\n- \"1\": Force [`SDL_THREAD_PRIORITY_TIME_CRITICAL`] to a realtime scheduling\npolicy\n\nThis hint should be set before calling [`SDL_SetCurrentThreadPriority()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_THREAD_PRIORITY_POLICY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_THREAD_PRIORITY_POLICY",
    short_name: "THREAD_PRIORITY_POLICY",
    value: crate::hints::SDL_HINT_THREAD_PRIORITY_POLICY,
    doc: Some(
        "A string specifying additional information to use with\n[`SDL_SetCurrentThreadPriority`].\n\nBy default [`SDL_SetCurrentThreadPriority`] will make appropriate system\nchanges in order to apply a thread priority. For example on systems using\npthreads the scheduler policy is changed automatically to a policy that\nworks well with a given priority. Code which has specific requirements can\noverride SDL's default behavior with this hint.\n\npthread hint values are \"current\", \"other\", \"fifo\" and \"rr\". Currently no\nother platform hint values are defined but may be in the future.\n\nOn Linux, the kernel may send SIGKILL to realtime tasks which exceed the\ndistro configured execution budget for rtkit. This budget can be queried\nthrough RLIMIT_RTTIME after calling [`SDL_SetCurrentThreadPriority()`].\n\nThis hint should be set before calling [`SDL_SetCurrentThreadPriority()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_TIMER_RESOLUTION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_TIMER_RESOLUTION",
    short_name: "TIMER_RESOLUTION",
    value: crate::hints::SDL_HINT_TIMER_RESOLUTION,
    doc: Some(
        "A variable that controls the timer resolution, in milliseconds.\n\nThe higher resolution the timer, the more frequently the CPU services timer\ninterrupts, and the more precise delays are, but this takes up power and\nCPU time. This hint is only used on Windows.\n\nSee this blog post for more information:\n<http://randomascii.wordpress.com/2013/07/08/windows-timer-resolution-megawatts-wasted/>\n\nThe default value is \"1\".\n\nIf this variable is set to \"0\", the system timer resolution is not set.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_TOUCH_MOUSE_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_TOUCH_MOUSE_EVENTS",
    short_name: "TOUCH_MOUSE_EVENTS",
    value: crate::hints::SDL_HINT_TOUCH_MOUSE_EVENTS,
    doc: Some(
        "A variable controlling whether touch events should generate synthetic mouse\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": Touch events will not generate mouse events.\n- \"1\": Touch events will generate mouse events. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_TRACKPAD_IS_TOUCH_ONLY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_TRACKPAD_IS_TOUCH_ONLY",
    short_name: "TRACKPAD_IS_TOUCH_ONLY",
    value: crate::hints::SDL_HINT_TRACKPAD_IS_TOUCH_ONLY,
    doc: Some(
        "A variable controlling whether trackpads should be treated as touch\ndevices.\n\nOn macOS (and possibly other platforms in the future), SDL will report\ntouches on a trackpad as mouse input, which is generally what users expect\nfrom this device; however, these are often actually full multitouch-capable\ntouch devices, so it might be preferable to some apps to treat them as\nsuch.\n\nThe variable can be set to the following values:\n\n- \"0\": Trackpad will send mouse events. (default)\n- \"1\": Trackpad will send touch events.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_TV_REMOTE_AS_JOYSTICK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_TV_REMOTE_AS_JOYSTICK",
    short_name: "TV_REMOTE_AS_JOYSTICK",
    value: crate::hints::SDL_HINT_TV_REMOTE_AS_JOYSTICK,
    doc: Some(
        "A variable controlling whether the Android / tvOS remotes should be listed\nas joystick devices, instead of sending keyboard events.\n\nThe variable can be set to the following values:\n\n- \"0\": Remotes send enter/escape/arrow key events.\n- \"1\": Remotes are available as 2 axis, 2 button joysticks. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_ALLOW_SCREENSAVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_ALLOW_SCREENSAVER",
    short_name: "VIDEO_ALLOW_SCREENSAVER",
    value: crate::hints::SDL_HINT_VIDEO_ALLOW_SCREENSAVER,
    doc: Some(
        "A variable controlling whether the screensaver is enabled.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable screensaver. (default)\n- \"1\": Enable screensaver.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_DISPLAY_PRIORITY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_DISPLAY_PRIORITY",
    short_name: "VIDEO_DISPLAY_PRIORITY",
    value: crate::hints::SDL_HINT_VIDEO_DISPLAY_PRIORITY,
    doc: Some(
        "A comma separated list containing the names of the displays that SDL should\nsort to the front of the display list.\n\nWhen this hint is set, displays with matching name strings will be\nprioritized in the list of displays, as exposed by calling\n[`SDL_GetDisplays()`], with the first listed becoming the primary display. The\nnaming convention can vary depending on the environment, but it is usually\na connector name (e.g. 'DP-1', 'DP-2', 'HDMI-A-1',etc...).\n\nOn Wayland and X11 desktops, the connector names associated with displays\ncan typically be found by using the `xrandr` utility.\n\nThis hint is currently supported on the following drivers:\n\n- KMSDRM (kmsdrm)\n- Wayland (wayland)\n- X11 (x11)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_DOUBLE_BUFFER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_DOUBLE_BUFFER",
    short_name: "VIDEO_DOUBLE_BUFFER",
    value: crate::hints::SDL_HINT_VIDEO_DOUBLE_BUFFER,
    doc: Some(
        "Tell the video driver that we only want a double buffer.\n\nBy default, most lowlevel 2D APIs will use a triple buffer scheme that\nwastes no CPU time on waiting for vsync after issuing a flip, but\nintroduces a frame of latency. On the other hand, using a double buffer\nscheme instead is recommended for cases where low latency is an important\nfactor because we save a whole frame of latency.\n\nWe do so by waiting for vsync immediately after issuing a flip, usually\njust after eglSwapBuffers call in the backend's *_SwapWindow function.\n\nThis hint is currently supported on the following drivers:\n\n- Raspberry Pi (raspberrypi)\n- Wayland (wayland)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_DRIVER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_DRIVER",
    short_name: "VIDEO_DRIVER",
    value: crate::hints::SDL_HINT_VIDEO_DRIVER,
    doc: Some(
        "A variable that specifies a video backend to use.\n\nBy default, SDL will try all available video backends in a reasonable order\nuntil it finds one that can work, but this hint allows the app or user to\nforce a specific target, such as \"x11\" if, say, you are on Wayland but want\nto try talking to the X server instead.\n\nThis hint accepts a comma-separated list of driver names, and each will be\ntried in the order listed during init, until one succeeds or all of them\nfail.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES",
    short_name: "VIDEO_DUMMY_SAVE_FRAMES",
    value: crate::hints::SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES,
    doc: Some(
        "A variable controlling whether the dummy video driver saves output frames.\n\n- \"0\": Video frames are not saved to disk. (default)\n- \"1\": Video frames are saved to files in the format \"SDL_windowX-Y.bmp\",\nwhere X is the window ID, and Y is the frame number.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK",
    short_name: "VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK",
    value: crate::hints::SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK,
    doc: Some(
        "If eglGetPlatformDisplay fails, fall back to calling eglGetDisplay.\n\nThe variable can be set to one of the following values:\n\n- \"0\": Do not fall back to eglGetDisplay.\n- \"1\": Fall back to eglGetDisplay if eglGetPlatformDisplay fails. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_FORCE_EGL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_FORCE_EGL",
    short_name: "VIDEO_FORCE_EGL",
    value: crate::hints::SDL_HINT_VIDEO_FORCE_EGL,
    doc: Some(
        "A variable controlling whether the OpenGL context should be created with\nEGL.\n\nThe variable can be set to the following values:\n\n- \"0\": Use platform-specific GL context creation API (GLX, WGL, CGL, etc).\n(default)\n- \"1\": Use EGL\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES",
    short_name: "VIDEO_MAC_FULLSCREEN_SPACES",
    value: crate::hints::SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES,
    doc: Some(
        "A variable that specifies the policy for fullscreen Spaces on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable Spaces support (FULLSCREEN_DESKTOP won't use them and\n[`SDL_WINDOW_RESIZABLE`] windows won't offer the \"fullscreen\" button on their\ntitlebars).\n- \"1\": Enable Spaces support (FULLSCREEN_DESKTOP will use them and\n[`SDL_WINDOW_RESIZABLE`] windows will offer the \"fullscreen\" button on their\ntitlebars). (default)\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY",
    short_name: "VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY",
    value: crate::hints::SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY,
    doc: Some(
        "A variable that specifies the menu visibility when a window is fullscreen\nin Spaces on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": The menu will be hidden when the window is in a fullscreen space,\nand not accessible by moving the mouse to the top of the screen.\n- \"1\": The menu will be accessible when the window is in a fullscreen\nspace.\n- \"auto\": The menu will be hidden if fullscreen mode was toggled on\nprogrammatically via `SDL_SetWindowFullscreen()`, and accessible if\nfullscreen was entered via the \"fullscreen\" button on the window title\nbar. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_METAL_AUTO_RESIZE_DRAWABLE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_METAL_AUTO_RESIZE_DRAWABLE",
    short_name: "VIDEO_METAL_AUTO_RESIZE_DRAWABLE",
    value: crate::hints::SDL_HINT_VIDEO_METAL_AUTO_RESIZE_DRAWABLE,
    doc: Some(
        "A variable indicating whether the metal layer drawable size should be\nupdated for the [`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] event on macOS.\n\nThe variable can be set to the following values:\n\n- \"0\": the metal layer drawable size will not be updated on the\n[`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] event.\n- \"1\": the metal layer drawable size will be updated on the\n[`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] event. (default)\n\nThis hint should be set before [`SDL_Metal_CreateView`] called.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE",
    short_name: "VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE",
    value: crate::hints::SDL_HINT_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE,
    doc: Some(
        "A variable controlling whether SDL will attempt to automatically set the\ndestination display to a mode most closely matching that of the previous\ndisplay if an exclusive fullscreen window is moved onto it.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will not attempt to automatically set a matching mode on the\ndestination display. If an exclusive fullscreen window is moved to a new\ndisplay, the window will become fullscreen desktop.\n- \"1\": SDL will attempt to automatically set a mode on the destination\ndisplay that most closely matches the mode of the display that the\nexclusive fullscreen window was previously on. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS",
    short_name: "VIDEO_MINIMIZE_ON_FOCUS_LOSS",
    value: crate::hints::SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS,
    doc: Some(
        "A variable controlling whether fullscreen windows are minimized when they\nlose focus.\n\nThe variable can be set to the following values:\n\n- \"0\": Fullscreen windows will not be minimized when they lose focus.\n(default)\n- \"1\": Fullscreen windows are minimized when they lose focus.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES",
    short_name: "VIDEO_OFFSCREEN_SAVE_FRAMES",
    value: crate::hints::SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES,
    doc: Some(
        "A variable controlling whether the offscreen video driver saves output\nframes.\n\nThis only saves frames that are generated using software rendering, not\naccelerated OpenGL rendering.\n\n- \"0\": Video frames are not saved to disk. (default)\n- \"1\": Video frames are saved to files in the format \"SDL_windowX-Y.bmp\",\nwhere X is the window ID, and Y is the frame number.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS",
    short_name: "VIDEO_SYNC_WINDOW_OPERATIONS",
    value: crate::hints::SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS,
    doc: Some(
        "A variable controlling whether all window operations will block until\ncomplete.\n\nWindow systems that run asynchronously may not have the results of window\noperations that resize or move the window applied immediately upon the\nreturn of the requesting function. Setting this hint will cause such\noperations to block after every call until the pending operation has\ncompleted. Setting this to '1' is the equivalent of calling\n[`SDL_SyncWindow()`] after every function call.\n\nBe aware that amount of time spent blocking while waiting for window\noperations to complete can be quite lengthy, as animations may have to\ncomplete, which can take upwards of multiple seconds in some cases.\n\nThe variable can be set to the following values:\n\n- \"0\": Window operations are non-blocking. (default)\n- \"1\": Window operations will block until completed.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR",
    short_name: "VIDEO_WAYLAND_ALLOW_LIBDECOR",
    value: crate::hints::SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR,
    doc: Some(
        "A variable controlling whether the libdecor Wayland backend is allowed to\nbe used.\n\nlibdecor is used over xdg-shell when xdg-decoration protocol is\nunavailable.\n\nThe variable can be set to the following values:\n\n- \"0\": libdecor use is disabled.\n- \"1\": libdecor use is enabled. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION",
    short_name: "VIDEO_WAYLAND_MODE_EMULATION",
    value: crate::hints::SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION,
    doc: Some(
        "A variable controlling whether video mode emulation is enabled under\nWayland.\n\nWhen this hint is set, a standard set of emulated CVT video modes will be\nexposed for use by the application. If it is disabled, the only modes\nexposed will be the logical desktop size and, in the case of a scaled\ndesktop, the native display resolution.\n\nThe variable can be set to the following values:\n\n- \"0\": Video mode emulation is disabled.\n- \"1\": Video mode emulation is enabled. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WAYLAND_MODE_SCALING: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WAYLAND_MODE_SCALING",
    short_name: "VIDEO_WAYLAND_MODE_SCALING",
    value: crate::hints::SDL_HINT_VIDEO_WAYLAND_MODE_SCALING,
    doc: Some(
        "A variable controlling how modes with a non-native aspect ratio are\ndisplayed under Wayland.\n\nWhen this hint is set, the requested scaling will be used when displaying\nfullscreen video modes that don't match the display's native aspect ratio.\nThis is contingent on compositor viewport support.\n\nThe variable can be set to the following values:\n\n- \"aspect\" - Video modes will be displayed scaled, in their proper aspect\nratio, with black bars.\n- \"stretch\" - Video modes will be scaled to fill the entire display.\n(default)\n- \"none\" - Video modes will be displayed as 1:1 with no scaling.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR",
    short_name: "VIDEO_WAYLAND_PREFER_LIBDECOR",
    value: crate::hints::SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR,
    doc: Some(
        "A variable controlling whether the libdecor Wayland backend is preferred\nover native decorations.\n\nWhen this hint is set, libdecor will be used to provide window decorations,\neven if xdg-decoration is available. (Note that, by default, libdecor will\nuse xdg-decoration itself if available).\n\nThe variable can be set to the following values:\n\n- \"0\": libdecor is enabled only if server-side decorations are unavailable.\n(default)\n- \"1\": libdecor is always enabled if available.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY",
    short_name: "VIDEO_WAYLAND_SCALE_TO_DISPLAY",
    value: crate::hints::SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY,
    doc: Some(
        "A variable forcing non-DPI-aware Wayland windows to output at 1:1 scaling.\n\nThis must be set before initializing the video subsystem.\n\nWhen this hint is set, Wayland windows that are not flagged as being\nDPI-aware will be output with scaling designed to force 1:1 pixel mapping.\n\nThis is intended to allow legacy applications to be displayed without\ndesktop scaling being applied, and has issues with certain display\nconfigurations, as this forces the window to behave in a way that Wayland\ndesktops were not designed to accommodate:\n\n- Rounding errors can result with odd window sizes and/or desktop scales,\nwhich can cause the window contents to appear slightly blurry.\n- Positioning the window may be imprecise due to unit conversions and\nrounding.\n- The window may be unusably small on scaled desktops.\n- The window may jump in size when moving between displays of different\nscale factors.\n- Displays may appear to overlap when using a multi-monitor setup with\nscaling enabled.\n- Possible loss of cursor precision due to the logical size of the window\nbeing reduced.\n\nNew applications should be designed with proper DPI awareness handling\ninstead of enabling this.\n\nThe variable can be set to the following values:\n\n- \"0\": Windows will be scaled normally.\n- \"1\": Windows will be forced to scale to achieve 1:1 output.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_WIN_D3DCOMPILER: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_WIN_D3DCOMPILER",
    short_name: "VIDEO_WIN_D3DCOMPILER",
    value: crate::hints::SDL_HINT_VIDEO_WIN_D3DCOMPILER,
    doc: Some(
        "A variable specifying which shader compiler to preload when using the\nChrome ANGLE binaries.\n\nSDL has EGL and OpenGL ES2 support on Windows via the ANGLE project. It can\nuse two different sets of binaries, those compiled by the user from source\nor those provided by the Chrome browser. In the later case, these binaries\nrequire that SDL loads a DLL providing the shader compiler.\n\nThe variable can be set to the following values:\n\n- \"d3dcompiler_46.dll\" - best for Vista or later. (default)\n- \"d3dcompiler_43.dll\" - for XP support.\n- \"none\" - do not load any library, useful if you compiled ANGLE from\nsource and included the compiler in your binaries.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT",
    short_name: "VIDEO_X11_EXTERNAL_WINDOW_INPUT",
    value: crate::hints::SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT,
    doc: Some(
        "A variable controlling whether SDL should call XSelectInput() to enable\ninput events on X11 windows wrapped by SDL windows.\n\nThe variable can be set to the following values:\n\n- \"0\": Don't call XSelectInput(), assuming the native window code has done\nit already.\n- \"1\": Call XSelectInput() to enable input events. (default)\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.10.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 10)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR",
    short_name: "VIDEO_X11_NET_WM_BYPASS_COMPOSITOR",
    value: crate::hints::SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR,
    doc: Some(
        "A variable controlling whether the X11 _NET_WM_BYPASS_COMPOSITOR hint\nshould be used.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable _NET_WM_BYPASS_COMPOSITOR.\n- \"1\": Enable _NET_WM_BYPASS_COMPOSITOR. (default)\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_NET_WM_PING: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_NET_WM_PING",
    short_name: "VIDEO_X11_NET_WM_PING",
    value: crate::hints::SDL_HINT_VIDEO_X11_NET_WM_PING,
    doc: Some(
        "A variable controlling whether the X11 _NET_WM_PING protocol should be\nsupported.\n\nBy default SDL will use _NET_WM_PING, but for applications that know they\nwill not always be able to respond to ping requests in a timely manner they\ncan turn it off to avoid the window manager thinking the app is hung.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable _NET_WM_PING.\n- \"1\": Enable _NET_WM_PING. (default)\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_NODIRECTCOLOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_NODIRECTCOLOR",
    short_name: "VIDEO_X11_NODIRECTCOLOR",
    value: crate::hints::SDL_HINT_VIDEO_X11_NODIRECTCOLOR,
    doc: Some(
        "A variable controlling whether SDL uses DirectColor visuals.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable DirectColor visuals.\n- \"1\": Enable DirectColor visuals. (default)\n\nThis hint should be set before initializing the video subsystem.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_SCALING_FACTOR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_SCALING_FACTOR",
    short_name: "VIDEO_X11_SCALING_FACTOR",
    value: crate::hints::SDL_HINT_VIDEO_X11_SCALING_FACTOR,
    doc: Some(
        "A variable forcing the content scaling factor for X11 displays.\n\nThe variable can be set to a floating point value in the range 1.0-10.0f\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_VISUALID: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_VISUALID",
    short_name: "VIDEO_X11_VISUALID",
    value: crate::hints::SDL_HINT_VIDEO_X11_VISUALID,
    doc: Some(
        "A variable forcing the visual ID used for X11 display modes.\n\nThis hint should be set before initializing the video subsystem.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_WINDOW_VISUALID: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_WINDOW_VISUALID",
    short_name: "VIDEO_X11_WINDOW_VISUALID",
    value: crate::hints::SDL_HINT_VIDEO_X11_WINDOW_VISUALID,
    doc: Some(
        "A variable forcing the visual ID chosen for new X11 windows.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VIDEO_X11_XRANDR: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VIDEO_X11_XRANDR",
    short_name: "VIDEO_X11_XRANDR",
    value: crate::hints::SDL_HINT_VIDEO_X11_XRANDR,
    doc: Some(
        "A variable controlling whether the X11 XRandR extension should be used.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable XRandR.\n- \"1\": Enable XRandR. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_ENABLE_BACK_TOUCH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_ENABLE_BACK_TOUCH",
    short_name: "VITA_ENABLE_BACK_TOUCH",
    value: crate::hints::SDL_HINT_VITA_ENABLE_BACK_TOUCH,
    doc: Some(
        "A variable controlling whether touch should be enabled on the back panel of\nthe PlayStation Vita.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable touch on the back panel.\n- \"1\": Enable touch on the back panel. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_ENABLE_FRONT_TOUCH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_ENABLE_FRONT_TOUCH",
    short_name: "VITA_ENABLE_FRONT_TOUCH",
    value: crate::hints::SDL_HINT_VITA_ENABLE_FRONT_TOUCH,
    doc: Some(
        "A variable controlling whether touch should be enabled on the front panel\nof the PlayStation Vita.\n\nThe variable can be set to the following values:\n\n- \"0\": Disable touch on the front panel.\n- \"1\": Enable touch on the front panel. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_MODULE_PATH: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_MODULE_PATH",
    short_name: "VITA_MODULE_PATH",
    value: crate::hints::SDL_HINT_VITA_MODULE_PATH,
    doc: Some(
        "A variable controlling the module path on the PlayStation Vita.\n\nThis hint defaults to \"app0:module\"\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_PVR_INIT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_PVR_INIT",
    short_name: "VITA_PVR_INIT",
    value: crate::hints::SDL_HINT_VITA_PVR_INIT,
    doc: Some(
        "A variable controlling whether to perform PVR initialization on the\nPlayStation Vita.\n\n- \"0\": Skip PVR initialization.\n- \"1\": Perform the normal PVR initialization. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_RESOLUTION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_RESOLUTION",
    short_name: "VITA_RESOLUTION",
    value: crate::hints::SDL_HINT_VITA_RESOLUTION,
    doc: Some(
        "A variable overriding the resolution reported on the PlayStation Vita.\n\nThe variable can be set to the following values:\n\n- \"544\": 544p (default)\n- \"720\": 725p for PSTV\n- \"1080\": 1088i for PSTV\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_PVR_OPENGL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_PVR_OPENGL",
    short_name: "VITA_PVR_OPENGL",
    value: crate::hints::SDL_HINT_VITA_PVR_OPENGL,
    doc: Some(
        "A variable controlling whether OpenGL should be used instead of OpenGL ES\non the PlayStation Vita.\n\nThe variable can be set to the following values:\n\n- \"0\": Use OpenGL ES. (default)\n- \"1\": Use OpenGL.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VITA_TOUCH_MOUSE_DEVICE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VITA_TOUCH_MOUSE_DEVICE",
    short_name: "VITA_TOUCH_MOUSE_DEVICE",
    value: crate::hints::SDL_HINT_VITA_TOUCH_MOUSE_DEVICE,
    doc: Some(
        "A variable controlling which touchpad should generate synthetic mouse\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": Only front touchpad should generate mouse events. (default)\n- \"1\": Only back touchpad should generate mouse events.\n- \"2\": Both touchpads should generate mouse events.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VULKAN_DISPLAY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VULKAN_DISPLAY",
    short_name: "VULKAN_DISPLAY",
    value: crate::hints::SDL_HINT_VULKAN_DISPLAY,
    doc: Some(
        "A variable overriding the display index used in [`SDL_Vulkan_CreateSurface()`]\n\nThe display index starts at 0, which is the default.\n\nThis hint should be set before calling [`SDL_Vulkan_CreateSurface()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_VULKAN_LIBRARY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_VULKAN_LIBRARY",
    short_name: "VULKAN_LIBRARY",
    value: crate::hints::SDL_HINT_VULKAN_LIBRARY,
    doc: Some(
        "Specify the Vulkan library to load.\n\nThis hint should be set before creating a Vulkan window or calling\n[`SDL_Vulkan_LoadLibrary()`].\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WAVE_FACT_CHUNK: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WAVE_FACT_CHUNK",
    short_name: "WAVE_FACT_CHUNK",
    value: crate::hints::SDL_HINT_WAVE_FACT_CHUNK,
    doc: Some(
        "A variable controlling how the fact chunk affects the loading of a WAVE\nfile.\n\nThe fact chunk stores information about the number of samples of a WAVE\nfile. The Standards Update from Microsoft notes that this value can be used\nto 'determine the length of the data in seconds'. This is especially useful\nfor compressed formats (for which this is a mandatory chunk) if they\nproduce multiple sample frames per block and truncating the block is not\nallowed. The fact chunk can exactly specify how many sample frames there\nshould be in this case.\n\nUnfortunately, most application seem to ignore the fact chunk and so SDL\nignores it by default as well.\n\nThe variable can be set to the following values:\n\n- \"truncate\" - Use the number of samples to truncate the wave data if the\nfact chunk is present and valid.\n- \"strict\" - Like \"truncate\", but raise an error if the fact chunk is\ninvalid, not present for non-PCM formats, or if the data chunk doesn't\nhave that many samples.\n- \"ignorezero\" - Like \"truncate\", but ignore fact chunk if the number of\nsamples is zero.\n- \"ignore\" - Ignore fact chunk entirely. (default)\n\nThis hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WAVE_CHUNK_LIMIT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WAVE_CHUNK_LIMIT",
    short_name: "WAVE_CHUNK_LIMIT",
    value: crate::hints::SDL_HINT_WAVE_CHUNK_LIMIT,
    doc: Some(
        "A variable controlling the maximum number of chunks in a WAVE file.\n\nThis sets an upper bound on the number of chunks in a WAVE file to avoid\nwasting time on malformed or corrupt WAVE files. This defaults to \"10000\".\n\nThis hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WAVE_RIFF_CHUNK_SIZE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WAVE_RIFF_CHUNK_SIZE",
    short_name: "WAVE_RIFF_CHUNK_SIZE",
    value: crate::hints::SDL_HINT_WAVE_RIFF_CHUNK_SIZE,
    doc: Some(
        "A variable controlling how the size of the RIFF chunk affects the loading\nof a WAVE file.\n\nThe size of the RIFF chunk (which includes all the sub-chunks of the WAVE\nfile) is not always reliable. In case the size is wrong, it's possible to\njust ignore it and step through the chunks until a fixed limit is reached.\n\nNote that files that have trailing data unrelated to the WAVE file or\ncorrupt files may slow down the loading process without a reliable\nboundary. By default, SDL stops after 10000 chunks to prevent wasting time.\nUse [`SDL_HINT_WAVE_CHUNK_LIMIT`] to adjust this value.\n\nThe variable can be set to the following values:\n\n- \"force\" - Always use the RIFF chunk size as a boundary for the chunk\nsearch.\n- \"ignorezero\" - Like \"force\", but a zero size searches up to 4 GiB.\n(default)\n- \"ignore\" - Ignore the RIFF chunk size and always search up to 4 GiB.\n- \"maximum\" - Search for chunks until the end of file. (not recommended)\n\nThis hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WAVE_TRUNCATION: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WAVE_TRUNCATION",
    short_name: "WAVE_TRUNCATION",
    value: crate::hints::SDL_HINT_WAVE_TRUNCATION,
    doc: Some(
        "A variable controlling how a truncated WAVE file is handled.\n\nA WAVE file is considered truncated if any of the chunks are incomplete or\nthe data chunk size is not a multiple of the block size. By default, SDL\ndecodes until the first incomplete block, as most applications seem to do.\n\nThe variable can be set to the following values:\n\n- \"verystrict\" - Raise an error if the file is truncated.\n- \"strict\" - Like \"verystrict\", but the size of the RIFF chunk is ignored.\n- \"dropframe\" - Decode until the first incomplete sample frame.\n- \"dropblock\" - Decode until the first incomplete block. (default)\n\nThis hint should be set before calling [`SDL_LoadWAV()`] or [`SDL_LoadWAV_IO()`]\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED",
    short_name: "WINDOW_ACTIVATE_WHEN_RAISED",
    value: crate::hints::SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED,
    doc: Some(
        "A variable controlling whether the window is activated when the\n[`SDL_RaiseWindow`] function is called.\n\nThe variable can be set to the following values:\n\n- \"0\": The window is not activated when the [`SDL_RaiseWindow`] function is\ncalled.\n- \"1\": The window is activated when the [`SDL_RaiseWindow`] function is called.\n(default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN",
    short_name: "WINDOW_ACTIVATE_WHEN_SHOWN",
    value: crate::hints::SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN,
    doc: Some(
        "A variable controlling whether the window is activated when the\n[`SDL_ShowWindow`] function is called.\n\nThe variable can be set to the following values:\n\n- \"0\": The window is not activated when the [`SDL_ShowWindow`] function is\ncalled.\n- \"1\": The window is activated when the [`SDL_ShowWindow`] function is called.\n(default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOW_ALLOW_TOPMOST: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOW_ALLOW_TOPMOST",
    short_name: "WINDOW_ALLOW_TOPMOST",
    value: crate::hints::SDL_HINT_WINDOW_ALLOW_TOPMOST,
    doc: Some(
        "If set to \"0\" then never set the top-most flag on an SDL Window even if the\napplication requests it.\n\nThis is a debugging aid for developers and not expected to be used by end\nusers.\n\nThe variable can be set to the following values:\n\n- \"0\": don't allow topmost\n- \"1\": allow topmost (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN",
    short_name: "WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN",
    value: crate::hints::SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN,
    doc: Some(
        "A variable controlling whether the window frame and title bar are\ninteractive when the cursor is hidden.\n\nThe variable can be set to the following values:\n\n- \"0\": The window frame is not interactive when the cursor is hidden (no\nmove, resize, etc).\n- \"1\": The window frame is interactive when the cursor is hidden. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4",
    short_name: "WINDOWS_CLOSE_ON_ALT_F4",
    value: crate::hints::SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4,
    doc: Some(
        "A variable controlling whether SDL generates window-close events for Alt+F4\non Windows.\n\nThe variable can be set to the following values:\n\n- \"0\": SDL will only do normal key handling for Alt+F4.\n- \"1\": SDL will generate a window-close event when it sees Alt+F4.\n(default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS",
    short_name: "WINDOWS_ENABLE_MENU_MNEMONICS",
    value: crate::hints::SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS,
    doc: Some(
        "A variable controlling whether menus can be opened with their keyboard\nshortcut (Alt+mnemonic).\n\nIf the mnemonics are enabled, then menus can be opened by pressing the Alt\nkey and the corresponding mnemonic (for example, Alt+F opens the File\nmenu). However, in case an invalid mnemonic is pressed, Windows makes an\naudible beep to convey that nothing happened. This is true even if the\nwindow has no menu at all!\n\nBecause most SDL applications don't have menus, and some want to use the\nAlt key for other purposes, SDL disables mnemonics (and the beeping) by\ndefault.\n\nNote: This also affects keyboard events: with mnemonics enabled, when a\nmenu is opened from the keyboard, you will not receive a KEYUP event for\nthe mnemonic key, and *might* not receive one for Alt.\n\nThe variable can be set to the following values:\n\n- \"0\": Alt+mnemonic does nothing, no beeping. (default)\n- \"1\": Alt+mnemonic opens menus, invalid mnemonics produce a beep.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP",
    short_name: "WINDOWS_ENABLE_MESSAGELOOP",
    value: crate::hints::SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP,
    doc: Some(
        "A variable controlling whether the windows message loop is processed by\nSDL.\n\nThe variable can be set to the following values:\n\n- \"0\": The window message loop is not run.\n- \"1\": The window message loop is processed in [`SDL_PumpEvents()`]. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_GAMEINPUT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_GAMEINPUT",
    short_name: "WINDOWS_GAMEINPUT",
    value: crate::hints::SDL_HINT_WINDOWS_GAMEINPUT,
    doc: Some(
        "A variable controlling whether GameInput is used for raw keyboard and mouse\non Windows.\n\nThe variable can be set to the following values:\n\n- \"0\": GameInput is not used for raw keyboard and mouse events. (default)\n- \"1\": GameInput is used for raw keyboard and mouse events, if available.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_RAW_KEYBOARD: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_RAW_KEYBOARD",
    short_name: "WINDOWS_RAW_KEYBOARD",
    value: crate::hints::SDL_HINT_WINDOWS_RAW_KEYBOARD,
    doc: Some(
        "A variable controlling whether raw keyboard events are used on Windows.\n\nThe variable can be set to the following values:\n\n- \"0\": The Windows message loop is used for keyboard events. (default)\n- \"1\": Low latency raw keyboard events are used.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS",
    short_name: "WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS",
    value: crate::hints::SDL_HINT_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS,
    doc: Some(
        "A variable controlling whether or not the RIDEV_NOHOTKEYS flag is set when\nenabling Windows raw keyboard events.\n\nThis blocks any hotkeys that have been registered by applications from\nhaving any effect beyond generating raw WM_INPUT events.\n\nThis flag does not affect system-hotkeys like ALT-TAB or CTRL-ALT-DEL, but\ndoes affect the Windows Logo key since it is a userland hotkey registered\nby explorer.exe.\n\nThe variable can be set to the following values:\n\n- \"0\": Hotkeys are not excluded. (default)\n- \"1\": Hotkeys are excluded.\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL",
    short_name: "WINDOWS_FORCE_SEMAPHORE_KERNEL",
    value: crate::hints::SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL,
    doc: Some(
        "A variable controlling whether SDL uses Kernel Semaphores on Windows.\n\nKernel Semaphores are inter-process and require a context switch on every\ninteraction. On Windows 8 and newer, the WaitOnAddress API is available.\nUsing that and atomics to implement semaphores increases performance. SDL\nwill fall back to Kernel Objects on older OS versions or if forced to by\nthis hint.\n\nThe variable can be set to the following values:\n\n- \"0\": Use Atomics and WaitOnAddress API when available, otherwise fall\nback to Kernel Objects. (default)\n- \"1\": Force the use of Kernel Objects in all cases.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_INTRESOURCE_ICON: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_INTRESOURCE_ICON",
    short_name: "WINDOWS_INTRESOURCE_ICON",
    value: crate::hints::SDL_HINT_WINDOWS_INTRESOURCE_ICON,
    doc: Some(
        "A variable to specify custom icon resource id from RC file on Windows\nplatform.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL",
    short_name: "WINDOWS_INTRESOURCE_ICON_SMALL",
    value: crate::hints::SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL,
    doc: Some(
        "A variable to specify custom icon resource id from RC file on Windows\nplatform.\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_USE_D3D9EX: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_USE_D3D9EX",
    short_name: "WINDOWS_USE_D3D9EX",
    value: crate::hints::SDL_HINT_WINDOWS_USE_D3D9EX,
    doc: Some(
        "A variable controlling whether SDL uses the D3D9Ex API introduced in\nWindows Vista, instead of normal D3D9.\n\nDirect3D 9Ex contains changes to state management that can eliminate device\nloss errors during scenarios like Alt+Tab or UAC prompts. D3D9Ex may\nrequire some changes to your application to cope with the new behavior, so\nthis is disabled by default.\n\nFor more information on Direct3D 9Ex, see:\n\n- <https://docs.microsoft.com/en-us/windows/win32/direct3darticles/graphics-apis-in-windows-vista#direct3d-9ex>\n- <https://docs.microsoft.com/en-us/windows/win32/direct3darticles/direct3d-9ex-improvements>\n\nThe variable can be set to the following values:\n\n- \"0\": Use the original Direct3D 9 API. (default)\n- \"1\": Use the Direct3D 9Ex API on Vista and later (and fall back if D3D9Ex\nis unavailable)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE",
    short_name: "WINDOWS_ERASE_BACKGROUND_MODE",
    value: crate::hints::SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE,
    doc: Some(
        "A variable controlling whether SDL will clear the window contents when the\nWM_ERASEBKGND message is received.\n\nThe variable can be set to the following values:\n\n- \"0\"/\"never\": Never clear the window.\n- \"1\"/\"initial\": Clear the window when the first WM_ERASEBKGND event fires.\n(default)\n- \"2\"/\"always\": Clear the window on every WM_ERASEBKGND event.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT",
    short_name: "X11_FORCE_OVERRIDE_REDIRECT",
    value: crate::hints::SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT,
    doc: Some(
        "A variable controlling whether X11 windows are marked as override-redirect.\n\nIf set, this _might_ increase framerate at the expense of the desktop not\nworking as expected. Override-redirect windows aren't noticed by the window\nmanager at all.\n\nYou should probably only use this for fullscreen windows, and you probably\nshouldn't even use it for that. But it's here if you want to try!\n\nThe variable can be set to the following values:\n\n- \"0\": Do not mark the window as override-redirect. (default)\n- \"1\": Mark the window as override-redirect.\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_X11_WINDOW_TYPE: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_X11_WINDOW_TYPE",
    short_name: "X11_WINDOW_TYPE",
    value: crate::hints::SDL_HINT_X11_WINDOW_TYPE,
    doc: Some(
        "A variable specifying the type of an X11 window.\n\nDuring [`SDL_CreateWindow`], SDL uses the _NET_WM_WINDOW_TYPE X11 property to\nreport to the window manager the type of window it wants to create. This\nmight be set to various things if [`SDL_WINDOW_TOOLTIP`] or\n[`SDL_WINDOW_POPUP_MENU`], etc, were specified. For \"normal\" windows that\nhaven't set a specific type, this hint can be used to specify a custom\ntype. For example, a dock window might set this to\n\"_NET_WM_WINDOW_TYPE_DOCK\".\n\nThis hint should be set before creating a window.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_X11_XCB_LIBRARY: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_X11_XCB_LIBRARY",
    short_name: "X11_XCB_LIBRARY",
    value: crate::hints::SDL_HINT_X11_XCB_LIBRARY,
    doc: Some(
        "Specify the XCB library to load for the X11 driver.\n\nThe default is platform-specific, often \"libX11-xcb.so.1\".\n\nThis hint should be set before initializing the video subsystem.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_XINPUT_ENABLED: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_XINPUT_ENABLED",
    short_name: "XINPUT_ENABLED",
    value: crate::hints::SDL_HINT_XINPUT_ENABLED,
    doc: Some(
        "A variable controlling whether XInput should be used for controller\nhandling.\n\nThe variable can be set to the following values:\n\n- \"0\": XInput is not enabled.\n- \"1\": XInput is enabled. (default)\n\nThis hint should be set before SDL is initialized.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_ASSERT: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_ASSERT",
    short_name: "ASSERT",
    value: crate::hints::SDL_HINT_ASSERT,
    doc: Some(
        "A variable controlling response to [`SDL_assert`] failures.\n\nThe variable can be set to the following case-sensitive values:\n\n- \"abort\": Program terminates immediately.\n- \"break\": Program triggers a debugger breakpoint.\n- \"retry\": Program reruns the SDL_assert's test again.\n- \"ignore\": Program continues on, ignoring this assertion failure this\ntime.\n- \"always_ignore\": Program continues on, ignoring this assertion failure\nfor the rest of the run.\n\nNote that [`SDL_SetAssertionHandler`] offers a programmatic means to deal with\nassertion failures through a callback, and this hint is largely intended to\nbe used via environment variables by end users and automated tools.\n\nThis hint should be set before an assertion failure is triggered and can be\nchanged at any time.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_PEN_MOUSE_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PEN_MOUSE_EVENTS",
    short_name: "PEN_MOUSE_EVENTS",
    value: crate::hints::SDL_HINT_PEN_MOUSE_EVENTS,
    doc: Some(
        "A variable controlling whether pen events should generate synthetic mouse\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": Pen events will not generate mouse events.\n- \"1\": Pen events will generate mouse events. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HINT_PEN_TOUCH_EVENTS: Hint = Hint {
    module: "hints",
    name: "SDL_HINT_PEN_TOUCH_EVENTS",
    short_name: "PEN_TOUCH_EVENTS",
    value: crate::hints::SDL_HINT_PEN_TOUCH_EVENTS,
    doc: Some(
        "A variable controlling whether pen events should generate synthetic touch\nevents.\n\nThe variable can be set to the following values:\n\n- \"0\": Pen events will not generate touch events.\n- \"1\": Pen events will generate touch events. (default)\n\nThis hint can be set anytime.\n\n## Availability\nThis hint is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
};
pub const METADATA_SDL_HintPriority: Group = Group {
    module: "hints",
    kind: GroupKind::Enum,
    name: "SDL_HintPriority",
    short_name: "HintPriority",
    doc: Some(
        "An enumeration of hint priorities.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_HINT_DEFAULT",
            short_name: "DEFAULT",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_HINT_NORMAL",
            short_name: "NORMAL",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_HINT_OVERRIDE",
            short_name: "OVERRIDE",
            doc: None,
            available_since: None,
        },
    ],
};
