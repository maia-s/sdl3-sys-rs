//! Event queue management.

use super::stdinc::*;

use super::audio::*;

use super::camera::*;

use super::error::*;

use super::gamepad::*;

use super::joystick::*;

use super::keyboard::*;

use super::keycode::*;

use super::mouse::*;

use super::pen::*;

use super::power::*;

use super::sensor::*;

use super::scancode::*;

use super::touch::*;

use super::video::*;

/// The types of events that can be delivered.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`FIRST`](SDL_EventType::FIRST) | [`SDL_EVENT_FIRST`] | Unused (do not remove) |
/// | [`QUIT`](SDL_EventType::QUIT) | [`SDL_EVENT_QUIT`] | User-requested quit |
/// | [`TERMINATING`](SDL_EventType::TERMINATING) | [`SDL_EVENT_TERMINATING`] | The application is being terminated by the OS. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationWillTerminate() Called on Android in onDestroy() |
/// | [`LOW_MEMORY`](SDL_EventType::LOW_MEMORY) | [`SDL_EVENT_LOW_MEMORY`] | The application is low on memory, free memory if possible. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationDidReceiveMemoryWarning() Called on Android in onTrimMemory() |
/// | [`WILL_ENTER_BACKGROUND`](SDL_EventType::WILL_ENTER_BACKGROUND) | [`SDL_EVENT_WILL_ENTER_BACKGROUND`] | The application is about to enter the background. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationWillResignActive() Called on Android in onPause() |
/// | [`DID_ENTER_BACKGROUND`](SDL_EventType::DID_ENTER_BACKGROUND) | [`SDL_EVENT_DID_ENTER_BACKGROUND`] | The application did enter the background and may not get CPU for some time. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationDidEnterBackground() Called on Android in onPause() |
/// | [`WILL_ENTER_FOREGROUND`](SDL_EventType::WILL_ENTER_FOREGROUND) | [`SDL_EVENT_WILL_ENTER_FOREGROUND`] | The application is about to enter the foreground. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationWillEnterForeground() Called on Android in onResume() |
/// | [`DID_ENTER_FOREGROUND`](SDL_EventType::DID_ENTER_FOREGROUND) | [`SDL_EVENT_DID_ENTER_FOREGROUND`] | The application is now interactive. This event must be handled in a callback set with [`SDL_AddEventWatch()`]. Called on iOS in applicationDidBecomeActive() Called on Android in onResume() |
/// | [`LOCALE_CHANGED`](SDL_EventType::LOCALE_CHANGED) | [`SDL_EVENT_LOCALE_CHANGED`] | The user's locale preferences have changed. |
/// | [`SYSTEM_THEME_CHANGED`](SDL_EventType::SYSTEM_THEME_CHANGED) | [`SDL_EVENT_SYSTEM_THEME_CHANGED`] | The system theme changed |
/// | [`DISPLAY_ORIENTATION`](SDL_EventType::DISPLAY_ORIENTATION) | [`SDL_EVENT_DISPLAY_ORIENTATION`] | Display orientation has changed to data1 |
/// | [`DISPLAY_ADDED`](SDL_EventType::DISPLAY_ADDED) | [`SDL_EVENT_DISPLAY_ADDED`] | Display has been added to the system |
/// | [`DISPLAY_REMOVED`](SDL_EventType::DISPLAY_REMOVED) | [`SDL_EVENT_DISPLAY_REMOVED`] | Display has been removed from the system |
/// | [`DISPLAY_MOVED`](SDL_EventType::DISPLAY_MOVED) | [`SDL_EVENT_DISPLAY_MOVED`] | Display has changed position |
/// | [`DISPLAY_DESKTOP_MODE_CHANGED`](SDL_EventType::DISPLAY_DESKTOP_MODE_CHANGED) | [`SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED`] | Display has changed desktop mode |
/// | [`DISPLAY_CURRENT_MODE_CHANGED`](SDL_EventType::DISPLAY_CURRENT_MODE_CHANGED) | [`SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED`] | Display has changed current mode |
/// | [`DISPLAY_CONTENT_SCALE_CHANGED`](SDL_EventType::DISPLAY_CONTENT_SCALE_CHANGED) | [`SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED`] | Display has changed content scale |
/// | [`DISPLAY_FIRST`](SDL_EventType::DISPLAY_FIRST) | [`SDL_EVENT_DISPLAY_FIRST`] | |
/// | [`DISPLAY_LAST`](SDL_EventType::DISPLAY_LAST) | [`SDL_EVENT_DISPLAY_LAST`] | |
/// | [`WINDOW_SHOWN`](SDL_EventType::WINDOW_SHOWN) | [`SDL_EVENT_WINDOW_SHOWN`] | Window has been shown |
/// | [`WINDOW_HIDDEN`](SDL_EventType::WINDOW_HIDDEN) | [`SDL_EVENT_WINDOW_HIDDEN`] | Window has been hidden |
/// | [`WINDOW_EXPOSED`](SDL_EventType::WINDOW_EXPOSED) | [`SDL_EVENT_WINDOW_EXPOSED`] | Window has been exposed and should be redrawn, and can be redrawn directly from event watchers for this event |
/// | [`WINDOW_MOVED`](SDL_EventType::WINDOW_MOVED) | [`SDL_EVENT_WINDOW_MOVED`] | Window has been moved to data1, data2 |
/// | [`WINDOW_RESIZED`](SDL_EventType::WINDOW_RESIZED) | [`SDL_EVENT_WINDOW_RESIZED`] | Window has been resized to data1xdata2 |
/// | [`WINDOW_PIXEL_SIZE_CHANGED`](SDL_EventType::WINDOW_PIXEL_SIZE_CHANGED) | [`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] | The pixel size of the window has changed to data1xdata2 |
/// | [`WINDOW_METAL_VIEW_RESIZED`](SDL_EventType::WINDOW_METAL_VIEW_RESIZED) | [`SDL_EVENT_WINDOW_METAL_VIEW_RESIZED`] | The pixel size of a Metal view associated with the window has changed |
/// | [`WINDOW_MINIMIZED`](SDL_EventType::WINDOW_MINIMIZED) | [`SDL_EVENT_WINDOW_MINIMIZED`] | Window has been minimized |
/// | [`WINDOW_MAXIMIZED`](SDL_EventType::WINDOW_MAXIMIZED) | [`SDL_EVENT_WINDOW_MAXIMIZED`] | Window has been maximized |
/// | [`WINDOW_RESTORED`](SDL_EventType::WINDOW_RESTORED) | [`SDL_EVENT_WINDOW_RESTORED`] | Window has been restored to normal size and position |
/// | [`WINDOW_MOUSE_ENTER`](SDL_EventType::WINDOW_MOUSE_ENTER) | [`SDL_EVENT_WINDOW_MOUSE_ENTER`] | Window has gained mouse focus |
/// | [`WINDOW_MOUSE_LEAVE`](SDL_EventType::WINDOW_MOUSE_LEAVE) | [`SDL_EVENT_WINDOW_MOUSE_LEAVE`] | Window has lost mouse focus |
/// | [`WINDOW_FOCUS_GAINED`](SDL_EventType::WINDOW_FOCUS_GAINED) | [`SDL_EVENT_WINDOW_FOCUS_GAINED`] | Window has gained keyboard focus |
/// | [`WINDOW_FOCUS_LOST`](SDL_EventType::WINDOW_FOCUS_LOST) | [`SDL_EVENT_WINDOW_FOCUS_LOST`] | Window has lost keyboard focus |
/// | [`WINDOW_CLOSE_REQUESTED`](SDL_EventType::WINDOW_CLOSE_REQUESTED) | [`SDL_EVENT_WINDOW_CLOSE_REQUESTED`] | The window manager requests that the window be closed |
/// | [`WINDOW_HIT_TEST`](SDL_EventType::WINDOW_HIT_TEST) | [`SDL_EVENT_WINDOW_HIT_TEST`] | Window had a hit test that wasn't [`SDL_HITTEST_NORMAL`] |
/// | [`WINDOW_ICCPROF_CHANGED`](SDL_EventType::WINDOW_ICCPROF_CHANGED) | [`SDL_EVENT_WINDOW_ICCPROF_CHANGED`] | The ICC profile of the window's display has changed |
/// | [`WINDOW_DISPLAY_CHANGED`](SDL_EventType::WINDOW_DISPLAY_CHANGED) | [`SDL_EVENT_WINDOW_DISPLAY_CHANGED`] | Window has been moved to display data1 |
/// | [`WINDOW_DISPLAY_SCALE_CHANGED`](SDL_EventType::WINDOW_DISPLAY_SCALE_CHANGED) | [`SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED`] | Window display scale has been changed |
/// | [`WINDOW_SAFE_AREA_CHANGED`](SDL_EventType::WINDOW_SAFE_AREA_CHANGED) | [`SDL_EVENT_WINDOW_SAFE_AREA_CHANGED`] | The window safe area has been changed |
/// | [`WINDOW_OCCLUDED`](SDL_EventType::WINDOW_OCCLUDED) | [`SDL_EVENT_WINDOW_OCCLUDED`] | The window has been occluded |
/// | [`WINDOW_ENTER_FULLSCREEN`](SDL_EventType::WINDOW_ENTER_FULLSCREEN) | [`SDL_EVENT_WINDOW_ENTER_FULLSCREEN`] | The window has entered fullscreen mode |
/// | [`WINDOW_LEAVE_FULLSCREEN`](SDL_EventType::WINDOW_LEAVE_FULLSCREEN) | [`SDL_EVENT_WINDOW_LEAVE_FULLSCREEN`] | The window has left fullscreen mode |
/// | [`WINDOW_DESTROYED`](SDL_EventType::WINDOW_DESTROYED) | [`SDL_EVENT_WINDOW_DESTROYED`] | The window with the associated ID is being or has been destroyed. If this message is being handled in an event watcher, the window handle is still valid and can still be used to retrieve any properties associated with the window. Otherwise, the handle has already been destroyed and all resources associated with it are invalid |
/// | [`WINDOW_HDR_STATE_CHANGED`](SDL_EventType::WINDOW_HDR_STATE_CHANGED) | [`SDL_EVENT_WINDOW_HDR_STATE_CHANGED`] | Window HDR properties have changed |
/// | [`WINDOW_FIRST`](SDL_EventType::WINDOW_FIRST) | [`SDL_EVENT_WINDOW_FIRST`] | |
/// | [`WINDOW_LAST`](SDL_EventType::WINDOW_LAST) | [`SDL_EVENT_WINDOW_LAST`] | |
/// | [`KEY_DOWN`](SDL_EventType::KEY_DOWN) | [`SDL_EVENT_KEY_DOWN`] | Key pressed |
/// | [`KEY_UP`](SDL_EventType::KEY_UP) | [`SDL_EVENT_KEY_UP`] | Key released |
/// | [`TEXT_EDITING`](SDL_EventType::TEXT_EDITING) | [`SDL_EVENT_TEXT_EDITING`] | Keyboard text editing (composition) |
/// | [`TEXT_INPUT`](SDL_EventType::TEXT_INPUT) | [`SDL_EVENT_TEXT_INPUT`] | Keyboard text input |
/// | [`KEYMAP_CHANGED`](SDL_EventType::KEYMAP_CHANGED) | [`SDL_EVENT_KEYMAP_CHANGED`] | Keymap changed due to a system event such as an input language or keyboard layout change. |
/// | [`KEYBOARD_ADDED`](SDL_EventType::KEYBOARD_ADDED) | [`SDL_EVENT_KEYBOARD_ADDED`] | A new keyboard has been inserted into the system |
/// | [`KEYBOARD_REMOVED`](SDL_EventType::KEYBOARD_REMOVED) | [`SDL_EVENT_KEYBOARD_REMOVED`] | A keyboard has been removed |
/// | [`TEXT_EDITING_CANDIDATES`](SDL_EventType::TEXT_EDITING_CANDIDATES) | [`SDL_EVENT_TEXT_EDITING_CANDIDATES`] | Keyboard text editing candidates |
/// | [`MOUSE_MOTION`](SDL_EventType::MOUSE_MOTION) | [`SDL_EVENT_MOUSE_MOTION`] | Mouse moved |
/// | [`MOUSE_BUTTON_DOWN`](SDL_EventType::MOUSE_BUTTON_DOWN) | [`SDL_EVENT_MOUSE_BUTTON_DOWN`] | Mouse button pressed |
/// | [`MOUSE_BUTTON_UP`](SDL_EventType::MOUSE_BUTTON_UP) | [`SDL_EVENT_MOUSE_BUTTON_UP`] | Mouse button released |
/// | [`MOUSE_WHEEL`](SDL_EventType::MOUSE_WHEEL) | [`SDL_EVENT_MOUSE_WHEEL`] | Mouse wheel motion |
/// | [`MOUSE_ADDED`](SDL_EventType::MOUSE_ADDED) | [`SDL_EVENT_MOUSE_ADDED`] | A new mouse has been inserted into the system |
/// | [`MOUSE_REMOVED`](SDL_EventType::MOUSE_REMOVED) | [`SDL_EVENT_MOUSE_REMOVED`] | A mouse has been removed |
/// | [`JOYSTICK_AXIS_MOTION`](SDL_EventType::JOYSTICK_AXIS_MOTION) | [`SDL_EVENT_JOYSTICK_AXIS_MOTION`] | Joystick axis motion |
/// | [`JOYSTICK_BALL_MOTION`](SDL_EventType::JOYSTICK_BALL_MOTION) | [`SDL_EVENT_JOYSTICK_BALL_MOTION`] | Joystick trackball motion |
/// | [`JOYSTICK_HAT_MOTION`](SDL_EventType::JOYSTICK_HAT_MOTION) | [`SDL_EVENT_JOYSTICK_HAT_MOTION`] | Joystick hat position change |
/// | [`JOYSTICK_BUTTON_DOWN`](SDL_EventType::JOYSTICK_BUTTON_DOWN) | [`SDL_EVENT_JOYSTICK_BUTTON_DOWN`] | Joystick button pressed |
/// | [`JOYSTICK_BUTTON_UP`](SDL_EventType::JOYSTICK_BUTTON_UP) | [`SDL_EVENT_JOYSTICK_BUTTON_UP`] | Joystick button released |
/// | [`JOYSTICK_ADDED`](SDL_EventType::JOYSTICK_ADDED) | [`SDL_EVENT_JOYSTICK_ADDED`] | A new joystick has been inserted into the system |
/// | [`JOYSTICK_REMOVED`](SDL_EventType::JOYSTICK_REMOVED) | [`SDL_EVENT_JOYSTICK_REMOVED`] | An opened joystick has been removed |
/// | [`JOYSTICK_BATTERY_UPDATED`](SDL_EventType::JOYSTICK_BATTERY_UPDATED) | [`SDL_EVENT_JOYSTICK_BATTERY_UPDATED`] | Joystick battery level change |
/// | [`JOYSTICK_UPDATE_COMPLETE`](SDL_EventType::JOYSTICK_UPDATE_COMPLETE) | [`SDL_EVENT_JOYSTICK_UPDATE_COMPLETE`] | Joystick update is complete |
/// | [`GAMEPAD_AXIS_MOTION`](SDL_EventType::GAMEPAD_AXIS_MOTION) | [`SDL_EVENT_GAMEPAD_AXIS_MOTION`] | Gamepad axis motion |
/// | [`GAMEPAD_BUTTON_DOWN`](SDL_EventType::GAMEPAD_BUTTON_DOWN) | [`SDL_EVENT_GAMEPAD_BUTTON_DOWN`] | Gamepad button pressed |
/// | [`GAMEPAD_BUTTON_UP`](SDL_EventType::GAMEPAD_BUTTON_UP) | [`SDL_EVENT_GAMEPAD_BUTTON_UP`] | Gamepad button released |
/// | [`GAMEPAD_ADDED`](SDL_EventType::GAMEPAD_ADDED) | [`SDL_EVENT_GAMEPAD_ADDED`] | A new gamepad has been inserted into the system |
/// | [`GAMEPAD_REMOVED`](SDL_EventType::GAMEPAD_REMOVED) | [`SDL_EVENT_GAMEPAD_REMOVED`] | A gamepad has been removed |
/// | [`GAMEPAD_REMAPPED`](SDL_EventType::GAMEPAD_REMAPPED) | [`SDL_EVENT_GAMEPAD_REMAPPED`] | The gamepad mapping was updated |
/// | [`GAMEPAD_TOUCHPAD_DOWN`](SDL_EventType::GAMEPAD_TOUCHPAD_DOWN) | [`SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN`] | Gamepad touchpad was touched |
/// | [`GAMEPAD_TOUCHPAD_MOTION`](SDL_EventType::GAMEPAD_TOUCHPAD_MOTION) | [`SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION`] | Gamepad touchpad finger was moved |
/// | [`GAMEPAD_TOUCHPAD_UP`](SDL_EventType::GAMEPAD_TOUCHPAD_UP) | [`SDL_EVENT_GAMEPAD_TOUCHPAD_UP`] | Gamepad touchpad finger was lifted |
/// | [`GAMEPAD_SENSOR_UPDATE`](SDL_EventType::GAMEPAD_SENSOR_UPDATE) | [`SDL_EVENT_GAMEPAD_SENSOR_UPDATE`] | Gamepad sensor was updated |
/// | [`GAMEPAD_UPDATE_COMPLETE`](SDL_EventType::GAMEPAD_UPDATE_COMPLETE) | [`SDL_EVENT_GAMEPAD_UPDATE_COMPLETE`] | Gamepad update is complete |
/// | [`GAMEPAD_STEAM_HANDLE_UPDATED`](SDL_EventType::GAMEPAD_STEAM_HANDLE_UPDATED) | [`SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED`] | Gamepad Steam handle has changed |
/// | [`FINGER_DOWN`](SDL_EventType::FINGER_DOWN) | [`SDL_EVENT_FINGER_DOWN`] | |
/// | [`FINGER_UP`](SDL_EventType::FINGER_UP) | [`SDL_EVENT_FINGER_UP`] | |
/// | [`FINGER_MOTION`](SDL_EventType::FINGER_MOTION) | [`SDL_EVENT_FINGER_MOTION`] | |
/// | [`CLIPBOARD_UPDATE`](SDL_EventType::CLIPBOARD_UPDATE) | [`SDL_EVENT_CLIPBOARD_UPDATE`] | The clipboard or primary selection changed |
/// | [`DROP_FILE`](SDL_EventType::DROP_FILE) | [`SDL_EVENT_DROP_FILE`] | The system requests a file open |
/// | [`DROP_TEXT`](SDL_EventType::DROP_TEXT) | [`SDL_EVENT_DROP_TEXT`] | text/plain drag-and-drop event |
/// | [`DROP_BEGIN`](SDL_EventType::DROP_BEGIN) | [`SDL_EVENT_DROP_BEGIN`] | A new set of drops is beginning (NULL filename) |
/// | [`DROP_COMPLETE`](SDL_EventType::DROP_COMPLETE) | [`SDL_EVENT_DROP_COMPLETE`] | Current set of drops is now complete (NULL filename) |
/// | [`DROP_POSITION`](SDL_EventType::DROP_POSITION) | [`SDL_EVENT_DROP_POSITION`] | Position while moving over the window |
/// | [`AUDIO_DEVICE_ADDED`](SDL_EventType::AUDIO_DEVICE_ADDED) | [`SDL_EVENT_AUDIO_DEVICE_ADDED`] | A new audio device is available |
/// | [`AUDIO_DEVICE_REMOVED`](SDL_EventType::AUDIO_DEVICE_REMOVED) | [`SDL_EVENT_AUDIO_DEVICE_REMOVED`] | An audio device has been removed. |
/// | [`AUDIO_DEVICE_FORMAT_CHANGED`](SDL_EventType::AUDIO_DEVICE_FORMAT_CHANGED) | [`SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED`] | An audio device's format has been changed by the system. |
/// | [`SENSOR_UPDATE`](SDL_EventType::SENSOR_UPDATE) | [`SDL_EVENT_SENSOR_UPDATE`] | A sensor was updated |
/// | [`PEN_PROXIMITY_IN`](SDL_EventType::PEN_PROXIMITY_IN) | [`SDL_EVENT_PEN_PROXIMITY_IN`] | Pressure-sensitive pen has become available |
/// | [`PEN_PROXIMITY_OUT`](SDL_EventType::PEN_PROXIMITY_OUT) | [`SDL_EVENT_PEN_PROXIMITY_OUT`] | Pressure-sensitive pen has become unavailable |
/// | [`PEN_DOWN`](SDL_EventType::PEN_DOWN) | [`SDL_EVENT_PEN_DOWN`] | Pressure-sensitive pen touched drawing surface |
/// | [`PEN_UP`](SDL_EventType::PEN_UP) | [`SDL_EVENT_PEN_UP`] | Pressure-sensitive pen stopped touching drawing surface |
/// | [`PEN_BUTTON_DOWN`](SDL_EventType::PEN_BUTTON_DOWN) | [`SDL_EVENT_PEN_BUTTON_DOWN`] | Pressure-sensitive pen button pressed |
/// | [`PEN_BUTTON_UP`](SDL_EventType::PEN_BUTTON_UP) | [`SDL_EVENT_PEN_BUTTON_UP`] | Pressure-sensitive pen button released |
/// | [`PEN_MOTION`](SDL_EventType::PEN_MOTION) | [`SDL_EVENT_PEN_MOTION`] | Pressure-sensitive pen is moving on the tablet |
/// | [`PEN_AXIS`](SDL_EventType::PEN_AXIS) | [`SDL_EVENT_PEN_AXIS`] | Pressure-sensitive pen angle/pressure/etc changed |
/// | [`CAMERA_DEVICE_ADDED`](SDL_EventType::CAMERA_DEVICE_ADDED) | [`SDL_EVENT_CAMERA_DEVICE_ADDED`] | A new camera device is available |
/// | [`CAMERA_DEVICE_REMOVED`](SDL_EventType::CAMERA_DEVICE_REMOVED) | [`SDL_EVENT_CAMERA_DEVICE_REMOVED`] | A camera device has been removed. |
/// | [`CAMERA_DEVICE_APPROVED`](SDL_EventType::CAMERA_DEVICE_APPROVED) | [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] | A camera device has been approved for use by the user. |
/// | [`CAMERA_DEVICE_DENIED`](SDL_EventType::CAMERA_DEVICE_DENIED) | [`SDL_EVENT_CAMERA_DEVICE_DENIED`] | A camera device has been denied for use by the user. |
/// | [`RENDER_TARGETS_RESET`](SDL_EventType::RENDER_TARGETS_RESET) | [`SDL_EVENT_RENDER_TARGETS_RESET`] | The render targets have been reset and their contents need to be updated |
/// | [`RENDER_DEVICE_RESET`](SDL_EventType::RENDER_DEVICE_RESET) | [`SDL_EVENT_RENDER_DEVICE_RESET`] | The device has been reset and all textures need to be recreated |
/// | [`RENDER_DEVICE_LOST`](SDL_EventType::RENDER_DEVICE_LOST) | [`SDL_EVENT_RENDER_DEVICE_LOST`] | The device has been lost and can't be recovered. |
/// | [`PRIVATE0`](SDL_EventType::PRIVATE0) | [`SDL_EVENT_PRIVATE0`] | |
/// | [`PRIVATE1`](SDL_EventType::PRIVATE1) | [`SDL_EVENT_PRIVATE1`] | |
/// | [`PRIVATE2`](SDL_EventType::PRIVATE2) | [`SDL_EVENT_PRIVATE2`] | |
/// | [`PRIVATE3`](SDL_EventType::PRIVATE3) | [`SDL_EVENT_PRIVATE3`] | |
/// | [`POLL_SENTINEL`](SDL_EventType::POLL_SENTINEL) | [`SDL_EVENT_POLL_SENTINEL`] | Signals the end of an event poll cycle |
/// | [`USER`](SDL_EventType::USER) | [`SDL_EVENT_USER`] |  Events [`SDL_EVENT_USER`] through [`SDL_EVENT_LAST`] are for your use, and should be allocated with [`SDL_RegisterEvents()`] |
/// | [`LAST`](SDL_EventType::LAST) | [`SDL_EVENT_LAST`] | *  This last event is only for bounding internal arrays |
/// | [`ENUM_PADDING`](SDL_EventType::ENUM_PADDING) | [`SDL_EVENT_ENUM_PADDING`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_EventType(pub Uint32);

impl From<SDL_EventType> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_EventType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_EventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::FIRST => "SDL_EVENT_FIRST",
            Self::QUIT => "SDL_EVENT_QUIT",
            Self::TERMINATING => "SDL_EVENT_TERMINATING",
            Self::LOW_MEMORY => "SDL_EVENT_LOW_MEMORY",
            Self::WILL_ENTER_BACKGROUND => "SDL_EVENT_WILL_ENTER_BACKGROUND",
            Self::DID_ENTER_BACKGROUND => "SDL_EVENT_DID_ENTER_BACKGROUND",
            Self::WILL_ENTER_FOREGROUND => "SDL_EVENT_WILL_ENTER_FOREGROUND",
            Self::DID_ENTER_FOREGROUND => "SDL_EVENT_DID_ENTER_FOREGROUND",
            Self::LOCALE_CHANGED => "SDL_EVENT_LOCALE_CHANGED",
            Self::SYSTEM_THEME_CHANGED => "SDL_EVENT_SYSTEM_THEME_CHANGED",
            Self::DISPLAY_ORIENTATION => "SDL_EVENT_DISPLAY_ORIENTATION",
            Self::DISPLAY_ADDED => "SDL_EVENT_DISPLAY_ADDED",
            Self::DISPLAY_REMOVED => "SDL_EVENT_DISPLAY_REMOVED",
            Self::DISPLAY_MOVED => "SDL_EVENT_DISPLAY_MOVED",
            Self::DISPLAY_DESKTOP_MODE_CHANGED => "SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED",
            Self::DISPLAY_CURRENT_MODE_CHANGED => "SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED",
            Self::DISPLAY_CONTENT_SCALE_CHANGED => "SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED",
            Self::DISPLAY_FIRST => "SDL_EVENT_DISPLAY_FIRST",
            Self::DISPLAY_LAST => "SDL_EVENT_DISPLAY_LAST",
            Self::WINDOW_SHOWN => "SDL_EVENT_WINDOW_SHOWN",
            Self::WINDOW_HIDDEN => "SDL_EVENT_WINDOW_HIDDEN",
            Self::WINDOW_EXPOSED => "SDL_EVENT_WINDOW_EXPOSED",
            Self::WINDOW_MOVED => "SDL_EVENT_WINDOW_MOVED",
            Self::WINDOW_RESIZED => "SDL_EVENT_WINDOW_RESIZED",
            Self::WINDOW_PIXEL_SIZE_CHANGED => "SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED",
            Self::WINDOW_METAL_VIEW_RESIZED => "SDL_EVENT_WINDOW_METAL_VIEW_RESIZED",
            Self::WINDOW_MINIMIZED => "SDL_EVENT_WINDOW_MINIMIZED",
            Self::WINDOW_MAXIMIZED => "SDL_EVENT_WINDOW_MAXIMIZED",
            Self::WINDOW_RESTORED => "SDL_EVENT_WINDOW_RESTORED",
            Self::WINDOW_MOUSE_ENTER => "SDL_EVENT_WINDOW_MOUSE_ENTER",
            Self::WINDOW_MOUSE_LEAVE => "SDL_EVENT_WINDOW_MOUSE_LEAVE",
            Self::WINDOW_FOCUS_GAINED => "SDL_EVENT_WINDOW_FOCUS_GAINED",
            Self::WINDOW_FOCUS_LOST => "SDL_EVENT_WINDOW_FOCUS_LOST",
            Self::WINDOW_CLOSE_REQUESTED => "SDL_EVENT_WINDOW_CLOSE_REQUESTED",
            Self::WINDOW_HIT_TEST => "SDL_EVENT_WINDOW_HIT_TEST",
            Self::WINDOW_ICCPROF_CHANGED => "SDL_EVENT_WINDOW_ICCPROF_CHANGED",
            Self::WINDOW_DISPLAY_CHANGED => "SDL_EVENT_WINDOW_DISPLAY_CHANGED",
            Self::WINDOW_DISPLAY_SCALE_CHANGED => "SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED",
            Self::WINDOW_SAFE_AREA_CHANGED => "SDL_EVENT_WINDOW_SAFE_AREA_CHANGED",
            Self::WINDOW_OCCLUDED => "SDL_EVENT_WINDOW_OCCLUDED",
            Self::WINDOW_ENTER_FULLSCREEN => "SDL_EVENT_WINDOW_ENTER_FULLSCREEN",
            Self::WINDOW_LEAVE_FULLSCREEN => "SDL_EVENT_WINDOW_LEAVE_FULLSCREEN",
            Self::WINDOW_DESTROYED => "SDL_EVENT_WINDOW_DESTROYED",
            Self::WINDOW_HDR_STATE_CHANGED => "SDL_EVENT_WINDOW_HDR_STATE_CHANGED",
            Self::WINDOW_FIRST => "SDL_EVENT_WINDOW_FIRST",
            Self::WINDOW_LAST => "SDL_EVENT_WINDOW_LAST",
            Self::KEY_DOWN => "SDL_EVENT_KEY_DOWN",
            Self::KEY_UP => "SDL_EVENT_KEY_UP",
            Self::TEXT_EDITING => "SDL_EVENT_TEXT_EDITING",
            Self::TEXT_INPUT => "SDL_EVENT_TEXT_INPUT",
            Self::KEYMAP_CHANGED => "SDL_EVENT_KEYMAP_CHANGED",
            Self::KEYBOARD_ADDED => "SDL_EVENT_KEYBOARD_ADDED",
            Self::KEYBOARD_REMOVED => "SDL_EVENT_KEYBOARD_REMOVED",
            Self::TEXT_EDITING_CANDIDATES => "SDL_EVENT_TEXT_EDITING_CANDIDATES",
            Self::MOUSE_MOTION => "SDL_EVENT_MOUSE_MOTION",
            Self::MOUSE_BUTTON_DOWN => "SDL_EVENT_MOUSE_BUTTON_DOWN",
            Self::MOUSE_BUTTON_UP => "SDL_EVENT_MOUSE_BUTTON_UP",
            Self::MOUSE_WHEEL => "SDL_EVENT_MOUSE_WHEEL",
            Self::MOUSE_ADDED => "SDL_EVENT_MOUSE_ADDED",
            Self::MOUSE_REMOVED => "SDL_EVENT_MOUSE_REMOVED",
            Self::JOYSTICK_AXIS_MOTION => "SDL_EVENT_JOYSTICK_AXIS_MOTION",
            Self::JOYSTICK_BALL_MOTION => "SDL_EVENT_JOYSTICK_BALL_MOTION",
            Self::JOYSTICK_HAT_MOTION => "SDL_EVENT_JOYSTICK_HAT_MOTION",
            Self::JOYSTICK_BUTTON_DOWN => "SDL_EVENT_JOYSTICK_BUTTON_DOWN",
            Self::JOYSTICK_BUTTON_UP => "SDL_EVENT_JOYSTICK_BUTTON_UP",
            Self::JOYSTICK_ADDED => "SDL_EVENT_JOYSTICK_ADDED",
            Self::JOYSTICK_REMOVED => "SDL_EVENT_JOYSTICK_REMOVED",
            Self::JOYSTICK_BATTERY_UPDATED => "SDL_EVENT_JOYSTICK_BATTERY_UPDATED",
            Self::JOYSTICK_UPDATE_COMPLETE => "SDL_EVENT_JOYSTICK_UPDATE_COMPLETE",
            Self::GAMEPAD_AXIS_MOTION => "SDL_EVENT_GAMEPAD_AXIS_MOTION",
            Self::GAMEPAD_BUTTON_DOWN => "SDL_EVENT_GAMEPAD_BUTTON_DOWN",
            Self::GAMEPAD_BUTTON_UP => "SDL_EVENT_GAMEPAD_BUTTON_UP",
            Self::GAMEPAD_ADDED => "SDL_EVENT_GAMEPAD_ADDED",
            Self::GAMEPAD_REMOVED => "SDL_EVENT_GAMEPAD_REMOVED",
            Self::GAMEPAD_REMAPPED => "SDL_EVENT_GAMEPAD_REMAPPED",
            Self::GAMEPAD_TOUCHPAD_DOWN => "SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN",
            Self::GAMEPAD_TOUCHPAD_MOTION => "SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION",
            Self::GAMEPAD_TOUCHPAD_UP => "SDL_EVENT_GAMEPAD_TOUCHPAD_UP",
            Self::GAMEPAD_SENSOR_UPDATE => "SDL_EVENT_GAMEPAD_SENSOR_UPDATE",
            Self::GAMEPAD_UPDATE_COMPLETE => "SDL_EVENT_GAMEPAD_UPDATE_COMPLETE",
            Self::GAMEPAD_STEAM_HANDLE_UPDATED => "SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED",
            Self::FINGER_DOWN => "SDL_EVENT_FINGER_DOWN",
            Self::FINGER_UP => "SDL_EVENT_FINGER_UP",
            Self::FINGER_MOTION => "SDL_EVENT_FINGER_MOTION",
            Self::CLIPBOARD_UPDATE => "SDL_EVENT_CLIPBOARD_UPDATE",
            Self::DROP_FILE => "SDL_EVENT_DROP_FILE",
            Self::DROP_TEXT => "SDL_EVENT_DROP_TEXT",
            Self::DROP_BEGIN => "SDL_EVENT_DROP_BEGIN",
            Self::DROP_COMPLETE => "SDL_EVENT_DROP_COMPLETE",
            Self::DROP_POSITION => "SDL_EVENT_DROP_POSITION",
            Self::AUDIO_DEVICE_ADDED => "SDL_EVENT_AUDIO_DEVICE_ADDED",
            Self::AUDIO_DEVICE_REMOVED => "SDL_EVENT_AUDIO_DEVICE_REMOVED",
            Self::AUDIO_DEVICE_FORMAT_CHANGED => "SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED",
            Self::SENSOR_UPDATE => "SDL_EVENT_SENSOR_UPDATE",
            Self::PEN_PROXIMITY_IN => "SDL_EVENT_PEN_PROXIMITY_IN",
            Self::PEN_PROXIMITY_OUT => "SDL_EVENT_PEN_PROXIMITY_OUT",
            Self::PEN_DOWN => "SDL_EVENT_PEN_DOWN",
            Self::PEN_UP => "SDL_EVENT_PEN_UP",
            Self::PEN_BUTTON_DOWN => "SDL_EVENT_PEN_BUTTON_DOWN",
            Self::PEN_BUTTON_UP => "SDL_EVENT_PEN_BUTTON_UP",
            Self::PEN_MOTION => "SDL_EVENT_PEN_MOTION",
            Self::PEN_AXIS => "SDL_EVENT_PEN_AXIS",
            Self::CAMERA_DEVICE_ADDED => "SDL_EVENT_CAMERA_DEVICE_ADDED",
            Self::CAMERA_DEVICE_REMOVED => "SDL_EVENT_CAMERA_DEVICE_REMOVED",
            Self::CAMERA_DEVICE_APPROVED => "SDL_EVENT_CAMERA_DEVICE_APPROVED",
            Self::CAMERA_DEVICE_DENIED => "SDL_EVENT_CAMERA_DEVICE_DENIED",
            Self::RENDER_TARGETS_RESET => "SDL_EVENT_RENDER_TARGETS_RESET",
            Self::RENDER_DEVICE_RESET => "SDL_EVENT_RENDER_DEVICE_RESET",
            Self::RENDER_DEVICE_LOST => "SDL_EVENT_RENDER_DEVICE_LOST",
            Self::PRIVATE0 => "SDL_EVENT_PRIVATE0",
            Self::PRIVATE1 => "SDL_EVENT_PRIVATE1",
            Self::PRIVATE2 => "SDL_EVENT_PRIVATE2",
            Self::PRIVATE3 => "SDL_EVENT_PRIVATE3",
            Self::POLL_SENTINEL => "SDL_EVENT_POLL_SENTINEL",
            Self::USER => "SDL_EVENT_USER",
            Self::LAST => "SDL_EVENT_LAST",
            Self::ENUM_PADDING => "SDL_EVENT_ENUM_PADDING",

            _ => return write!(f, "SDL_EventType({})", self.0),
        })
    }
}

impl SDL_EventType {
    /// Unused (do not remove)
    pub const FIRST: Self = Self(0);
    /// User-requested quit
    pub const QUIT: Self = Self(0x100);
    /// The application is being terminated by the OS. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationWillTerminate()
    /// Called on Android in onDestroy()
    pub const TERMINATING: Self = Self(257);
    /// The application is low on memory, free memory if possible. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationDidReceiveMemoryWarning()
    /// Called on Android in onTrimMemory()
    pub const LOW_MEMORY: Self = Self(258);
    /// The application is about to enter the background. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationWillResignActive()
    /// Called on Android in onPause()
    pub const WILL_ENTER_BACKGROUND: Self = Self(259);
    /// The application did enter the background and may not get CPU for some time. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationDidEnterBackground()
    /// Called on Android in onPause()
    pub const DID_ENTER_BACKGROUND: Self = Self(260);
    /// The application is about to enter the foreground. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationWillEnterForeground()
    /// Called on Android in onResume()
    pub const WILL_ENTER_FOREGROUND: Self = Self(261);
    /// The application is now interactive. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
    /// Called on iOS in applicationDidBecomeActive()
    /// Called on Android in onResume()
    pub const DID_ENTER_FOREGROUND: Self = Self(262);
    /// The user's locale preferences have changed.
    pub const LOCALE_CHANGED: Self = Self(263);
    /// The system theme changed
    pub const SYSTEM_THEME_CHANGED: Self = Self(264);
    /// Display orientation has changed to data1
    pub const DISPLAY_ORIENTATION: Self = Self(0x151);
    /// Display has been added to the system
    pub const DISPLAY_ADDED: Self = Self(338);
    /// Display has been removed from the system
    pub const DISPLAY_REMOVED: Self = Self(339);
    /// Display has changed position
    pub const DISPLAY_MOVED: Self = Self(340);
    /// Display has changed desktop mode
    pub const DISPLAY_DESKTOP_MODE_CHANGED: Self = Self(341);
    /// Display has changed current mode
    pub const DISPLAY_CURRENT_MODE_CHANGED: Self = Self(342);
    /// Display has changed content scale
    pub const DISPLAY_CONTENT_SCALE_CHANGED: Self = Self(343);
    pub const DISPLAY_FIRST: Self = SDL_EVENT_DISPLAY_ORIENTATION;
    pub const DISPLAY_LAST: Self = SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED;
    /// Window has been shown
    pub const WINDOW_SHOWN: Self = Self(0x202);
    /// Window has been hidden
    pub const WINDOW_HIDDEN: Self = Self(515);
    /// Window has been exposed and should be redrawn, and can be redrawn directly from event watchers for this event
    pub const WINDOW_EXPOSED: Self = Self(516);
    /// Window has been moved to data1, data2
    pub const WINDOW_MOVED: Self = Self(517);
    /// Window has been resized to data1xdata2
    pub const WINDOW_RESIZED: Self = Self(518);
    /// The pixel size of the window has changed to data1xdata2
    pub const WINDOW_PIXEL_SIZE_CHANGED: Self = Self(519);
    /// The pixel size of a Metal view associated with the window has changed
    pub const WINDOW_METAL_VIEW_RESIZED: Self = Self(520);
    /// Window has been minimized
    pub const WINDOW_MINIMIZED: Self = Self(521);
    /// Window has been maximized
    pub const WINDOW_MAXIMIZED: Self = Self(522);
    /// Window has been restored to normal size and position
    pub const WINDOW_RESTORED: Self = Self(523);
    /// Window has gained mouse focus
    pub const WINDOW_MOUSE_ENTER: Self = Self(524);
    /// Window has lost mouse focus
    pub const WINDOW_MOUSE_LEAVE: Self = Self(525);
    /// Window has gained keyboard focus
    pub const WINDOW_FOCUS_GAINED: Self = Self(526);
    /// Window has lost keyboard focus
    pub const WINDOW_FOCUS_LOST: Self = Self(527);
    /// The window manager requests that the window be closed
    pub const WINDOW_CLOSE_REQUESTED: Self = Self(528);
    /// Window had a hit test that wasn't [`SDL_HITTEST_NORMAL`]
    pub const WINDOW_HIT_TEST: Self = Self(529);
    /// The ICC profile of the window's display has changed
    pub const WINDOW_ICCPROF_CHANGED: Self = Self(530);
    /// Window has been moved to display data1
    pub const WINDOW_DISPLAY_CHANGED: Self = Self(531);
    /// Window display scale has been changed
    pub const WINDOW_DISPLAY_SCALE_CHANGED: Self = Self(532);
    /// The window safe area has been changed
    pub const WINDOW_SAFE_AREA_CHANGED: Self = Self(533);
    /// The window has been occluded
    pub const WINDOW_OCCLUDED: Self = Self(534);
    /// The window has entered fullscreen mode
    pub const WINDOW_ENTER_FULLSCREEN: Self = Self(535);
    /// The window has left fullscreen mode
    pub const WINDOW_LEAVE_FULLSCREEN: Self = Self(536);
    /// The window with the associated ID is being or has been destroyed. If this message is being handled
    /// in an event watcher, the window handle is still valid and can still be used to retrieve any properties
    /// associated with the window. Otherwise, the handle has already been destroyed and all resources
    /// associated with it are invalid
    pub const WINDOW_DESTROYED: Self = Self(537);
    /// Window HDR properties have changed
    pub const WINDOW_HDR_STATE_CHANGED: Self = Self(538);
    pub const WINDOW_FIRST: Self = SDL_EVENT_WINDOW_SHOWN;
    pub const WINDOW_LAST: Self = SDL_EVENT_WINDOW_HDR_STATE_CHANGED;
    /// Key pressed
    pub const KEY_DOWN: Self = Self(0x300);
    /// Key released
    pub const KEY_UP: Self = Self(769);
    /// Keyboard text editing (composition)
    pub const TEXT_EDITING: Self = Self(770);
    /// Keyboard text input
    pub const TEXT_INPUT: Self = Self(771);
    /// Keymap changed due to a system event such as an
    /// input language or keyboard layout change.
    pub const KEYMAP_CHANGED: Self = Self(772);
    /// A new keyboard has been inserted into the system
    pub const KEYBOARD_ADDED: Self = Self(773);
    /// A keyboard has been removed
    pub const KEYBOARD_REMOVED: Self = Self(774);
    /// Keyboard text editing candidates
    pub const TEXT_EDITING_CANDIDATES: Self = Self(775);
    /// Mouse moved
    pub const MOUSE_MOTION: Self = Self(0x400);
    /// Mouse button pressed
    pub const MOUSE_BUTTON_DOWN: Self = Self(1025);
    /// Mouse button released
    pub const MOUSE_BUTTON_UP: Self = Self(1026);
    /// Mouse wheel motion
    pub const MOUSE_WHEEL: Self = Self(1027);
    /// A new mouse has been inserted into the system
    pub const MOUSE_ADDED: Self = Self(1028);
    /// A mouse has been removed
    pub const MOUSE_REMOVED: Self = Self(1029);
    /// Joystick axis motion
    pub const JOYSTICK_AXIS_MOTION: Self = Self(0x600);
    /// Joystick trackball motion
    pub const JOYSTICK_BALL_MOTION: Self = Self(1537);
    /// Joystick hat position change
    pub const JOYSTICK_HAT_MOTION: Self = Self(1538);
    /// Joystick button pressed
    pub const JOYSTICK_BUTTON_DOWN: Self = Self(1539);
    /// Joystick button released
    pub const JOYSTICK_BUTTON_UP: Self = Self(1540);
    /// A new joystick has been inserted into the system
    pub const JOYSTICK_ADDED: Self = Self(1541);
    /// An opened joystick has been removed
    pub const JOYSTICK_REMOVED: Self = Self(1542);
    /// Joystick battery level change
    pub const JOYSTICK_BATTERY_UPDATED: Self = Self(1543);
    /// Joystick update is complete
    pub const JOYSTICK_UPDATE_COMPLETE: Self = Self(1544);
    /// Gamepad axis motion
    pub const GAMEPAD_AXIS_MOTION: Self = Self(0x650);
    /// Gamepad button pressed
    pub const GAMEPAD_BUTTON_DOWN: Self = Self(1617);
    /// Gamepad button released
    pub const GAMEPAD_BUTTON_UP: Self = Self(1618);
    /// A new gamepad has been inserted into the system
    pub const GAMEPAD_ADDED: Self = Self(1619);
    /// A gamepad has been removed
    pub const GAMEPAD_REMOVED: Self = Self(1620);
    /// The gamepad mapping was updated
    pub const GAMEPAD_REMAPPED: Self = Self(1621);
    /// Gamepad touchpad was touched
    pub const GAMEPAD_TOUCHPAD_DOWN: Self = Self(1622);
    /// Gamepad touchpad finger was moved
    pub const GAMEPAD_TOUCHPAD_MOTION: Self = Self(1623);
    /// Gamepad touchpad finger was lifted
    pub const GAMEPAD_TOUCHPAD_UP: Self = Self(1624);
    /// Gamepad sensor was updated
    pub const GAMEPAD_SENSOR_UPDATE: Self = Self(1625);
    /// Gamepad update is complete
    pub const GAMEPAD_UPDATE_COMPLETE: Self = Self(1626);
    /// Gamepad Steam handle has changed
    pub const GAMEPAD_STEAM_HANDLE_UPDATED: Self = Self(1627);
    pub const FINGER_DOWN: Self = Self(0x700);
    pub const FINGER_UP: Self = Self(1793);
    pub const FINGER_MOTION: Self = Self(1794);
    /// The clipboard or primary selection changed
    pub const CLIPBOARD_UPDATE: Self = Self(0x900);
    /// The system requests a file open
    pub const DROP_FILE: Self = Self(0x1000);
    /// text/plain drag-and-drop event
    pub const DROP_TEXT: Self = Self(4097);
    /// A new set of drops is beginning (NULL filename)
    pub const DROP_BEGIN: Self = Self(4098);
    /// Current set of drops is now complete (NULL filename)
    pub const DROP_COMPLETE: Self = Self(4099);
    /// Position while moving over the window
    pub const DROP_POSITION: Self = Self(4100);
    /// A new audio device is available
    pub const AUDIO_DEVICE_ADDED: Self = Self(0x1100);
    /// An audio device has been removed.
    pub const AUDIO_DEVICE_REMOVED: Self = Self(4353);
    /// An audio device's format has been changed by the system.
    pub const AUDIO_DEVICE_FORMAT_CHANGED: Self = Self(4354);
    /// A sensor was updated
    pub const SENSOR_UPDATE: Self = Self(0x1200);
    /// Pressure-sensitive pen has become available
    pub const PEN_PROXIMITY_IN: Self = Self(0x1300);
    /// Pressure-sensitive pen has become unavailable
    pub const PEN_PROXIMITY_OUT: Self = Self(4865);
    /// Pressure-sensitive pen touched drawing surface
    pub const PEN_DOWN: Self = Self(4866);
    /// Pressure-sensitive pen stopped touching drawing surface
    pub const PEN_UP: Self = Self(4867);
    /// Pressure-sensitive pen button pressed
    pub const PEN_BUTTON_DOWN: Self = Self(4868);
    /// Pressure-sensitive pen button released
    pub const PEN_BUTTON_UP: Self = Self(4869);
    /// Pressure-sensitive pen is moving on the tablet
    pub const PEN_MOTION: Self = Self(4870);
    /// Pressure-sensitive pen angle/pressure/etc changed
    pub const PEN_AXIS: Self = Self(4871);
    /// A new camera device is available
    pub const CAMERA_DEVICE_ADDED: Self = Self(0x1400);
    /// A camera device has been removed.
    pub const CAMERA_DEVICE_REMOVED: Self = Self(5121);
    /// A camera device has been approved for use by the user.
    pub const CAMERA_DEVICE_APPROVED: Self = Self(5122);
    /// A camera device has been denied for use by the user.
    pub const CAMERA_DEVICE_DENIED: Self = Self(5123);
    /// The render targets have been reset and their contents need to be updated
    pub const RENDER_TARGETS_RESET: Self = Self(0x2000);
    /// The device has been reset and all textures need to be recreated
    pub const RENDER_DEVICE_RESET: Self = Self(8193);
    /// The device has been lost and can't be recovered.
    pub const RENDER_DEVICE_LOST: Self = Self(8194);
    pub const PRIVATE0: Self = Self(0x4000);
    pub const PRIVATE1: Self = Self(16385);
    pub const PRIVATE2: Self = Self(16386);
    pub const PRIVATE3: Self = Self(16387);
    /// Signals the end of an event poll cycle
    pub const POLL_SENTINEL: Self = Self(0x7f00);
    ///  Events [`SDL_EVENT_USER`] through [`SDL_EVENT_LAST`] are for your use,
    /// and should be allocated with [`SDL_RegisterEvents()`]
    pub const USER: Self = Self(0x8000);
    /// *  This last event is only for bounding internal arrays
    pub const LAST: Self = Self(0xffff);
    pub const ENUM_PADDING: Self = Self(0x7fffffff);
}

/// Unused (do not remove)
pub const SDL_EVENT_FIRST: SDL_EventType = SDL_EventType::FIRST;
/// User-requested quit
pub const SDL_EVENT_QUIT: SDL_EventType = SDL_EventType::QUIT;
/// The application is being terminated by the OS. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationWillTerminate()
/// Called on Android in onDestroy()
pub const SDL_EVENT_TERMINATING: SDL_EventType = SDL_EventType::TERMINATING;
/// The application is low on memory, free memory if possible. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationDidReceiveMemoryWarning()
/// Called on Android in onTrimMemory()
pub const SDL_EVENT_LOW_MEMORY: SDL_EventType = SDL_EventType::LOW_MEMORY;
/// The application is about to enter the background. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationWillResignActive()
/// Called on Android in onPause()
pub const SDL_EVENT_WILL_ENTER_BACKGROUND: SDL_EventType = SDL_EventType::WILL_ENTER_BACKGROUND;
/// The application did enter the background and may not get CPU for some time. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationDidEnterBackground()
/// Called on Android in onPause()
pub const SDL_EVENT_DID_ENTER_BACKGROUND: SDL_EventType = SDL_EventType::DID_ENTER_BACKGROUND;
/// The application is about to enter the foreground. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationWillEnterForeground()
/// Called on Android in onResume()
pub const SDL_EVENT_WILL_ENTER_FOREGROUND: SDL_EventType = SDL_EventType::WILL_ENTER_FOREGROUND;
/// The application is now interactive. This event must be handled in a callback set with [`SDL_AddEventWatch()`].
/// Called on iOS in applicationDidBecomeActive()
/// Called on Android in onResume()
pub const SDL_EVENT_DID_ENTER_FOREGROUND: SDL_EventType = SDL_EventType::DID_ENTER_FOREGROUND;
/// The user's locale preferences have changed.
pub const SDL_EVENT_LOCALE_CHANGED: SDL_EventType = SDL_EventType::LOCALE_CHANGED;
/// The system theme changed
pub const SDL_EVENT_SYSTEM_THEME_CHANGED: SDL_EventType = SDL_EventType::SYSTEM_THEME_CHANGED;
/// Display orientation has changed to data1
pub const SDL_EVENT_DISPLAY_ORIENTATION: SDL_EventType = SDL_EventType::DISPLAY_ORIENTATION;
/// Display has been added to the system
pub const SDL_EVENT_DISPLAY_ADDED: SDL_EventType = SDL_EventType::DISPLAY_ADDED;
/// Display has been removed from the system
pub const SDL_EVENT_DISPLAY_REMOVED: SDL_EventType = SDL_EventType::DISPLAY_REMOVED;
/// Display has changed position
pub const SDL_EVENT_DISPLAY_MOVED: SDL_EventType = SDL_EventType::DISPLAY_MOVED;
/// Display has changed desktop mode
pub const SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED: SDL_EventType =
    SDL_EventType::DISPLAY_DESKTOP_MODE_CHANGED;
/// Display has changed current mode
pub const SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED: SDL_EventType =
    SDL_EventType::DISPLAY_CURRENT_MODE_CHANGED;
/// Display has changed content scale
pub const SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED: SDL_EventType =
    SDL_EventType::DISPLAY_CONTENT_SCALE_CHANGED;
pub const SDL_EVENT_DISPLAY_FIRST: SDL_EventType = SDL_EventType::DISPLAY_FIRST;
pub const SDL_EVENT_DISPLAY_LAST: SDL_EventType = SDL_EventType::DISPLAY_LAST;
/// Window has been shown
pub const SDL_EVENT_WINDOW_SHOWN: SDL_EventType = SDL_EventType::WINDOW_SHOWN;
/// Window has been hidden
pub const SDL_EVENT_WINDOW_HIDDEN: SDL_EventType = SDL_EventType::WINDOW_HIDDEN;
/// Window has been exposed and should be redrawn, and can be redrawn directly from event watchers for this event
pub const SDL_EVENT_WINDOW_EXPOSED: SDL_EventType = SDL_EventType::WINDOW_EXPOSED;
/// Window has been moved to data1, data2
pub const SDL_EVENT_WINDOW_MOVED: SDL_EventType = SDL_EventType::WINDOW_MOVED;
/// Window has been resized to data1xdata2
pub const SDL_EVENT_WINDOW_RESIZED: SDL_EventType = SDL_EventType::WINDOW_RESIZED;
/// The pixel size of the window has changed to data1xdata2
pub const SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED: SDL_EventType =
    SDL_EventType::WINDOW_PIXEL_SIZE_CHANGED;
/// The pixel size of a Metal view associated with the window has changed
pub const SDL_EVENT_WINDOW_METAL_VIEW_RESIZED: SDL_EventType =
    SDL_EventType::WINDOW_METAL_VIEW_RESIZED;
/// Window has been minimized
pub const SDL_EVENT_WINDOW_MINIMIZED: SDL_EventType = SDL_EventType::WINDOW_MINIMIZED;
/// Window has been maximized
pub const SDL_EVENT_WINDOW_MAXIMIZED: SDL_EventType = SDL_EventType::WINDOW_MAXIMIZED;
/// Window has been restored to normal size and position
pub const SDL_EVENT_WINDOW_RESTORED: SDL_EventType = SDL_EventType::WINDOW_RESTORED;
/// Window has gained mouse focus
pub const SDL_EVENT_WINDOW_MOUSE_ENTER: SDL_EventType = SDL_EventType::WINDOW_MOUSE_ENTER;
/// Window has lost mouse focus
pub const SDL_EVENT_WINDOW_MOUSE_LEAVE: SDL_EventType = SDL_EventType::WINDOW_MOUSE_LEAVE;
/// Window has gained keyboard focus
pub const SDL_EVENT_WINDOW_FOCUS_GAINED: SDL_EventType = SDL_EventType::WINDOW_FOCUS_GAINED;
/// Window has lost keyboard focus
pub const SDL_EVENT_WINDOW_FOCUS_LOST: SDL_EventType = SDL_EventType::WINDOW_FOCUS_LOST;
/// The window manager requests that the window be closed
pub const SDL_EVENT_WINDOW_CLOSE_REQUESTED: SDL_EventType = SDL_EventType::WINDOW_CLOSE_REQUESTED;
/// Window had a hit test that wasn't [`SDL_HITTEST_NORMAL`]
pub const SDL_EVENT_WINDOW_HIT_TEST: SDL_EventType = SDL_EventType::WINDOW_HIT_TEST;
/// The ICC profile of the window's display has changed
pub const SDL_EVENT_WINDOW_ICCPROF_CHANGED: SDL_EventType = SDL_EventType::WINDOW_ICCPROF_CHANGED;
/// Window has been moved to display data1
pub const SDL_EVENT_WINDOW_DISPLAY_CHANGED: SDL_EventType = SDL_EventType::WINDOW_DISPLAY_CHANGED;
/// Window display scale has been changed
pub const SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED: SDL_EventType =
    SDL_EventType::WINDOW_DISPLAY_SCALE_CHANGED;
/// The window safe area has been changed
pub const SDL_EVENT_WINDOW_SAFE_AREA_CHANGED: SDL_EventType =
    SDL_EventType::WINDOW_SAFE_AREA_CHANGED;
/// The window has been occluded
pub const SDL_EVENT_WINDOW_OCCLUDED: SDL_EventType = SDL_EventType::WINDOW_OCCLUDED;
/// The window has entered fullscreen mode
pub const SDL_EVENT_WINDOW_ENTER_FULLSCREEN: SDL_EventType = SDL_EventType::WINDOW_ENTER_FULLSCREEN;
/// The window has left fullscreen mode
pub const SDL_EVENT_WINDOW_LEAVE_FULLSCREEN: SDL_EventType = SDL_EventType::WINDOW_LEAVE_FULLSCREEN;
/// The window with the associated ID is being or has been destroyed. If this message is being handled
/// in an event watcher, the window handle is still valid and can still be used to retrieve any properties
/// associated with the window. Otherwise, the handle has already been destroyed and all resources
/// associated with it are invalid
pub const SDL_EVENT_WINDOW_DESTROYED: SDL_EventType = SDL_EventType::WINDOW_DESTROYED;
/// Window HDR properties have changed
pub const SDL_EVENT_WINDOW_HDR_STATE_CHANGED: SDL_EventType =
    SDL_EventType::WINDOW_HDR_STATE_CHANGED;
pub const SDL_EVENT_WINDOW_FIRST: SDL_EventType = SDL_EventType::WINDOW_FIRST;
pub const SDL_EVENT_WINDOW_LAST: SDL_EventType = SDL_EventType::WINDOW_LAST;
/// Key pressed
pub const SDL_EVENT_KEY_DOWN: SDL_EventType = SDL_EventType::KEY_DOWN;
/// Key released
pub const SDL_EVENT_KEY_UP: SDL_EventType = SDL_EventType::KEY_UP;
/// Keyboard text editing (composition)
pub const SDL_EVENT_TEXT_EDITING: SDL_EventType = SDL_EventType::TEXT_EDITING;
/// Keyboard text input
pub const SDL_EVENT_TEXT_INPUT: SDL_EventType = SDL_EventType::TEXT_INPUT;
/// Keymap changed due to a system event such as an
/// input language or keyboard layout change.
pub const SDL_EVENT_KEYMAP_CHANGED: SDL_EventType = SDL_EventType::KEYMAP_CHANGED;
/// A new keyboard has been inserted into the system
pub const SDL_EVENT_KEYBOARD_ADDED: SDL_EventType = SDL_EventType::KEYBOARD_ADDED;
/// A keyboard has been removed
pub const SDL_EVENT_KEYBOARD_REMOVED: SDL_EventType = SDL_EventType::KEYBOARD_REMOVED;
/// Keyboard text editing candidates
pub const SDL_EVENT_TEXT_EDITING_CANDIDATES: SDL_EventType = SDL_EventType::TEXT_EDITING_CANDIDATES;
/// Mouse moved
pub const SDL_EVENT_MOUSE_MOTION: SDL_EventType = SDL_EventType::MOUSE_MOTION;
/// Mouse button pressed
pub const SDL_EVENT_MOUSE_BUTTON_DOWN: SDL_EventType = SDL_EventType::MOUSE_BUTTON_DOWN;
/// Mouse button released
pub const SDL_EVENT_MOUSE_BUTTON_UP: SDL_EventType = SDL_EventType::MOUSE_BUTTON_UP;
/// Mouse wheel motion
pub const SDL_EVENT_MOUSE_WHEEL: SDL_EventType = SDL_EventType::MOUSE_WHEEL;
/// A new mouse has been inserted into the system
pub const SDL_EVENT_MOUSE_ADDED: SDL_EventType = SDL_EventType::MOUSE_ADDED;
/// A mouse has been removed
pub const SDL_EVENT_MOUSE_REMOVED: SDL_EventType = SDL_EventType::MOUSE_REMOVED;
/// Joystick axis motion
pub const SDL_EVENT_JOYSTICK_AXIS_MOTION: SDL_EventType = SDL_EventType::JOYSTICK_AXIS_MOTION;
/// Joystick trackball motion
pub const SDL_EVENT_JOYSTICK_BALL_MOTION: SDL_EventType = SDL_EventType::JOYSTICK_BALL_MOTION;
/// Joystick hat position change
pub const SDL_EVENT_JOYSTICK_HAT_MOTION: SDL_EventType = SDL_EventType::JOYSTICK_HAT_MOTION;
/// Joystick button pressed
pub const SDL_EVENT_JOYSTICK_BUTTON_DOWN: SDL_EventType = SDL_EventType::JOYSTICK_BUTTON_DOWN;
/// Joystick button released
pub const SDL_EVENT_JOYSTICK_BUTTON_UP: SDL_EventType = SDL_EventType::JOYSTICK_BUTTON_UP;
/// A new joystick has been inserted into the system
pub const SDL_EVENT_JOYSTICK_ADDED: SDL_EventType = SDL_EventType::JOYSTICK_ADDED;
/// An opened joystick has been removed
pub const SDL_EVENT_JOYSTICK_REMOVED: SDL_EventType = SDL_EventType::JOYSTICK_REMOVED;
/// Joystick battery level change
pub const SDL_EVENT_JOYSTICK_BATTERY_UPDATED: SDL_EventType =
    SDL_EventType::JOYSTICK_BATTERY_UPDATED;
/// Joystick update is complete
pub const SDL_EVENT_JOYSTICK_UPDATE_COMPLETE: SDL_EventType =
    SDL_EventType::JOYSTICK_UPDATE_COMPLETE;
/// Gamepad axis motion
pub const SDL_EVENT_GAMEPAD_AXIS_MOTION: SDL_EventType = SDL_EventType::GAMEPAD_AXIS_MOTION;
/// Gamepad button pressed
pub const SDL_EVENT_GAMEPAD_BUTTON_DOWN: SDL_EventType = SDL_EventType::GAMEPAD_BUTTON_DOWN;
/// Gamepad button released
pub const SDL_EVENT_GAMEPAD_BUTTON_UP: SDL_EventType = SDL_EventType::GAMEPAD_BUTTON_UP;
/// A new gamepad has been inserted into the system
pub const SDL_EVENT_GAMEPAD_ADDED: SDL_EventType = SDL_EventType::GAMEPAD_ADDED;
/// A gamepad has been removed
pub const SDL_EVENT_GAMEPAD_REMOVED: SDL_EventType = SDL_EventType::GAMEPAD_REMOVED;
/// The gamepad mapping was updated
pub const SDL_EVENT_GAMEPAD_REMAPPED: SDL_EventType = SDL_EventType::GAMEPAD_REMAPPED;
/// Gamepad touchpad was touched
pub const SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN: SDL_EventType = SDL_EventType::GAMEPAD_TOUCHPAD_DOWN;
/// Gamepad touchpad finger was moved
pub const SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION: SDL_EventType = SDL_EventType::GAMEPAD_TOUCHPAD_MOTION;
/// Gamepad touchpad finger was lifted
pub const SDL_EVENT_GAMEPAD_TOUCHPAD_UP: SDL_EventType = SDL_EventType::GAMEPAD_TOUCHPAD_UP;
/// Gamepad sensor was updated
pub const SDL_EVENT_GAMEPAD_SENSOR_UPDATE: SDL_EventType = SDL_EventType::GAMEPAD_SENSOR_UPDATE;
/// Gamepad update is complete
pub const SDL_EVENT_GAMEPAD_UPDATE_COMPLETE: SDL_EventType = SDL_EventType::GAMEPAD_UPDATE_COMPLETE;
/// Gamepad Steam handle has changed
pub const SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED: SDL_EventType =
    SDL_EventType::GAMEPAD_STEAM_HANDLE_UPDATED;
pub const SDL_EVENT_FINGER_DOWN: SDL_EventType = SDL_EventType::FINGER_DOWN;
pub const SDL_EVENT_FINGER_UP: SDL_EventType = SDL_EventType::FINGER_UP;
pub const SDL_EVENT_FINGER_MOTION: SDL_EventType = SDL_EventType::FINGER_MOTION;
/// The clipboard or primary selection changed
pub const SDL_EVENT_CLIPBOARD_UPDATE: SDL_EventType = SDL_EventType::CLIPBOARD_UPDATE;
/// The system requests a file open
pub const SDL_EVENT_DROP_FILE: SDL_EventType = SDL_EventType::DROP_FILE;
/// text/plain drag-and-drop event
pub const SDL_EVENT_DROP_TEXT: SDL_EventType = SDL_EventType::DROP_TEXT;
/// A new set of drops is beginning (NULL filename)
pub const SDL_EVENT_DROP_BEGIN: SDL_EventType = SDL_EventType::DROP_BEGIN;
/// Current set of drops is now complete (NULL filename)
pub const SDL_EVENT_DROP_COMPLETE: SDL_EventType = SDL_EventType::DROP_COMPLETE;
/// Position while moving over the window
pub const SDL_EVENT_DROP_POSITION: SDL_EventType = SDL_EventType::DROP_POSITION;
/// A new audio device is available
pub const SDL_EVENT_AUDIO_DEVICE_ADDED: SDL_EventType = SDL_EventType::AUDIO_DEVICE_ADDED;
/// An audio device has been removed.
pub const SDL_EVENT_AUDIO_DEVICE_REMOVED: SDL_EventType = SDL_EventType::AUDIO_DEVICE_REMOVED;
/// An audio device's format has been changed by the system.
pub const SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED: SDL_EventType =
    SDL_EventType::AUDIO_DEVICE_FORMAT_CHANGED;
/// A sensor was updated
pub const SDL_EVENT_SENSOR_UPDATE: SDL_EventType = SDL_EventType::SENSOR_UPDATE;
/// Pressure-sensitive pen has become available
pub const SDL_EVENT_PEN_PROXIMITY_IN: SDL_EventType = SDL_EventType::PEN_PROXIMITY_IN;
/// Pressure-sensitive pen has become unavailable
pub const SDL_EVENT_PEN_PROXIMITY_OUT: SDL_EventType = SDL_EventType::PEN_PROXIMITY_OUT;
/// Pressure-sensitive pen touched drawing surface
pub const SDL_EVENT_PEN_DOWN: SDL_EventType = SDL_EventType::PEN_DOWN;
/// Pressure-sensitive pen stopped touching drawing surface
pub const SDL_EVENT_PEN_UP: SDL_EventType = SDL_EventType::PEN_UP;
/// Pressure-sensitive pen button pressed
pub const SDL_EVENT_PEN_BUTTON_DOWN: SDL_EventType = SDL_EventType::PEN_BUTTON_DOWN;
/// Pressure-sensitive pen button released
pub const SDL_EVENT_PEN_BUTTON_UP: SDL_EventType = SDL_EventType::PEN_BUTTON_UP;
/// Pressure-sensitive pen is moving on the tablet
pub const SDL_EVENT_PEN_MOTION: SDL_EventType = SDL_EventType::PEN_MOTION;
/// Pressure-sensitive pen angle/pressure/etc changed
pub const SDL_EVENT_PEN_AXIS: SDL_EventType = SDL_EventType::PEN_AXIS;
/// A new camera device is available
pub const SDL_EVENT_CAMERA_DEVICE_ADDED: SDL_EventType = SDL_EventType::CAMERA_DEVICE_ADDED;
/// A camera device has been removed.
pub const SDL_EVENT_CAMERA_DEVICE_REMOVED: SDL_EventType = SDL_EventType::CAMERA_DEVICE_REMOVED;
/// A camera device has been approved for use by the user.
pub const SDL_EVENT_CAMERA_DEVICE_APPROVED: SDL_EventType = SDL_EventType::CAMERA_DEVICE_APPROVED;
/// A camera device has been denied for use by the user.
pub const SDL_EVENT_CAMERA_DEVICE_DENIED: SDL_EventType = SDL_EventType::CAMERA_DEVICE_DENIED;
/// The render targets have been reset and their contents need to be updated
pub const SDL_EVENT_RENDER_TARGETS_RESET: SDL_EventType = SDL_EventType::RENDER_TARGETS_RESET;
/// The device has been reset and all textures need to be recreated
pub const SDL_EVENT_RENDER_DEVICE_RESET: SDL_EventType = SDL_EventType::RENDER_DEVICE_RESET;
/// The device has been lost and can't be recovered.
pub const SDL_EVENT_RENDER_DEVICE_LOST: SDL_EventType = SDL_EventType::RENDER_DEVICE_LOST;
pub const SDL_EVENT_PRIVATE0: SDL_EventType = SDL_EventType::PRIVATE0;
pub const SDL_EVENT_PRIVATE1: SDL_EventType = SDL_EventType::PRIVATE1;
pub const SDL_EVENT_PRIVATE2: SDL_EventType = SDL_EventType::PRIVATE2;
pub const SDL_EVENT_PRIVATE3: SDL_EventType = SDL_EventType::PRIVATE3;
/// Signals the end of an event poll cycle
pub const SDL_EVENT_POLL_SENTINEL: SDL_EventType = SDL_EventType::POLL_SENTINEL;
///  Events [`SDL_EVENT_USER`] through [`SDL_EVENT_LAST`] are for your use,
/// and should be allocated with [`SDL_RegisterEvents()`]
pub const SDL_EVENT_USER: SDL_EventType = SDL_EventType::USER;
/// *  This last event is only for bounding internal arrays
pub const SDL_EVENT_LAST: SDL_EventType = SDL_EventType::LAST;
pub const SDL_EVENT_ENUM_PADDING: SDL_EventType = SDL_EventType::ENUM_PADDING;

/// Fields shared by every event
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_CommonEvent {
    /// Event type, shared with all events, Uint32 to cover user events which are not in the [`SDL_EventType`] enumeration
    pub r#type: Uint32,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
}

/// Display state change event data (event.display.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_DisplayEvent {
    /// SDL_DISPLAYEVENT_*
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The associated display
    pub displayID: SDL_DisplayID,
    /// event dependent data
    pub data1: Sint32,
    /// event dependent data
    pub data2: Sint32,
}

/// Window state change event data (event.window.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_WindowEvent {
    /// SDL_EVENT_WINDOW_*
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The associated window
    pub windowID: SDL_WindowID,
    /// event dependent data
    pub data1: Sint32,
    /// event dependent data
    pub data2: Sint32,
}

/// Keyboard device event structure (event.kdevice.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_KeyboardDeviceEvent {
    /// [`SDL_EVENT_KEYBOARD_ADDED`] or [`SDL_EVENT_KEYBOARD_REMOVED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The keyboard instance id
    pub which: SDL_KeyboardID,
}

/// Keyboard button event structure (event.key.*)
///
/// The `key` is the base [`SDL_Keycode`] generated by pressing the `scancode`
/// using the current keyboard layout, applying any options specified in
/// [`SDL_HINT_KEYCODE_OPTIONS`]. You can get the [`SDL_Keycode`] corresponding to the
/// event scancode and modifiers directly from the keyboard layout, bypassing
/// [`SDL_HINT_KEYCODE_OPTIONS`], by calling [`SDL_GetKeyFromScancode()`].
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetKeyFromScancode`]
/// - [`SDL_HINT_KEYCODE_OPTIONS`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_KeyboardEvent {
    /// [`SDL_EVENT_KEY_DOWN`] or [`SDL_EVENT_KEY_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with keyboard focus, if any
    pub windowID: SDL_WindowID,
    /// The keyboard instance id, or 0 if unknown or virtual
    pub which: SDL_KeyboardID,
    /// SDL physical key code
    pub scancode: SDL_Scancode,
    /// SDL virtual key code
    pub key: SDL_Keycode,
    /// current key modifiers
    pub r#mod: SDL_Keymod,
    /// The platform dependent scancode for this event
    pub raw: Uint16,
    /// true if the key is pressed
    pub down: ::core::primitive::bool,
    /// true if this is a key repeat
    pub repeat: ::core::primitive::bool,
}

/// Keyboard text editing event structure (event.edit.*)
///
/// The start cursor is the position, in UTF-8 characters, where new typing
/// will be inserted into the editing text. The length is the number of UTF-8
/// characters that will be replaced by new typing.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TextEditingEvent {
    /// [`SDL_EVENT_TEXT_EDITING`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with keyboard focus, if any
    pub windowID: SDL_WindowID,
    /// The editing text
    pub text: *const ::core::ffi::c_char,
    /// The start cursor of selected editing text, or -1 if not set
    pub start: Sint32,
    /// The length of selected editing text, or -1 if not set
    pub length: Sint32,
}

/// Keyboard IME candidates event structure (event.edit_candidates.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TextEditingCandidatesEvent {
    /// [`SDL_EVENT_TEXT_EDITING_CANDIDATES`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with keyboard focus, if any
    pub windowID: SDL_WindowID,
    /// The list of candidates, or NULL if there are no candidates available
    pub candidates: *const *const ::core::ffi::c_char,
    /// The number of strings in `candidates`
    pub num_candidates: Sint32,
    /// The index of the selected candidate, or -1 if no candidate is selected
    pub selected_candidate: Sint32,
    /// true if the list is horizontal, false if it's vertical
    pub horizontal: ::core::primitive::bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

/// Keyboard text input event structure (event.text.*)
///
/// This event will never be delivered unless text input is enabled by calling
/// [`SDL_StartTextInput()`]. Text input is disabled by default!
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_StartTextInput`]
/// - [`SDL_StopTextInput`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TextInputEvent {
    /// [`SDL_EVENT_TEXT_INPUT`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with keyboard focus, if any
    pub windowID: SDL_WindowID,
    /// The input text, UTF-8 encoded
    pub text: *const ::core::ffi::c_char,
}

/// Mouse device event structure (event.mdevice.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MouseDeviceEvent {
    /// [`SDL_EVENT_MOUSE_ADDED`] or [`SDL_EVENT_MOUSE_REMOVED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The mouse instance id
    pub which: SDL_MouseID,
}

/// Mouse motion event structure (event.motion.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MouseMotionEvent {
    /// [`SDL_EVENT_MOUSE_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The mouse instance id or [`SDL_TOUCH_MOUSEID`]
    pub which: SDL_MouseID,
    /// The current button state
    pub state: SDL_MouseButtonFlags,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
    /// The relative motion in the X direction
    pub xrel: ::core::ffi::c_float,
    /// The relative motion in the Y direction
    pub yrel: ::core::ffi::c_float,
}

/// Mouse button event structure (event.button.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MouseButtonEvent {
    /// [`SDL_EVENT_MOUSE_BUTTON_DOWN`] or [`SDL_EVENT_MOUSE_BUTTON_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The mouse instance id, [`SDL_TOUCH_MOUSEID`]
    pub which: SDL_MouseID,
    /// The mouse button index
    pub button: Uint8,
    /// true if the button is pressed
    pub down: ::core::primitive::bool,
    /// 1 for single-click, 2 for double-click, etc.
    pub clicks: Uint8,
    pub padding: Uint8,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
}

/// Mouse wheel event structure (event.wheel.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_MouseWheelEvent {
    /// [`SDL_EVENT_MOUSE_WHEEL`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The mouse instance id, [`SDL_TOUCH_MOUSEID`]
    pub which: SDL_MouseID,
    /// The amount scrolled horizontally, positive to the right and negative to the left
    pub x: ::core::ffi::c_float,
    /// The amount scrolled vertically, positive away from the user and negative toward the user
    pub y: ::core::ffi::c_float,
    /// Set to one of the SDL_MOUSEWHEEL_* defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back
    pub direction: SDL_MouseWheelDirection,
    /// X coordinate, relative to window
    pub mouse_x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub mouse_y: ::core::ffi::c_float,
}

/// Joystick axis motion event structure (event.jaxis.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyAxisEvent {
    /// [`SDL_EVENT_JOYSTICK_AXIS_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The joystick axis index
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// The axis value (range: -32768 to 32767)
    pub value: Sint16,
    pub padding4: Uint16,
}

/// Joystick trackball motion event structure (event.jball.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyBallEvent {
    /// [`SDL_EVENT_JOYSTICK_BALL_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The joystick trackball index
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// The relative motion in the X direction
    pub xrel: Sint16,
    /// The relative motion in the Y direction
    pub yrel: Sint16,
}

/// Joystick hat position change event structure (event.jhat.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyHatEvent {
    /// [`SDL_EVENT_JOYSTICK_HAT_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The joystick hat index
    pub hat: Uint8,
    /// The hat position value.
    /// ### See also
    /// - [`SDL_HAT_LEFTUP`] [`SDL_HAT_UP`] [`SDL_HAT_RIGHTUP`]
    /// - [`SDL_HAT_LEFT`] [`SDL_HAT_CENTERED`] [`SDL_HAT_RIGHT`]
    /// - [`SDL_HAT_LEFTDOWN`] [`SDL_HAT_DOWN`] [`SDL_HAT_RIGHTDOWN`]
    ///
    /// Note that zero means the POV is centered.
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

/// Joystick button event structure (event.jbutton.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyButtonEvent {
    /// [`SDL_EVENT_JOYSTICK_BUTTON_DOWN`] or [`SDL_EVENT_JOYSTICK_BUTTON_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The joystick button index
    pub button: Uint8,
    /// true if the button is pressed
    pub down: ::core::primitive::bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

/// Joystick device event structure (event.jdevice.*)
///
/// SDL will send JOYSTICK_ADDED events for devices that are already plugged in
/// during [`SDL_Init`].
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GamepadDeviceEvent`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyDeviceEvent {
    /// [`SDL_EVENT_JOYSTICK_ADDED`] or [`SDL_EVENT_JOYSTICK_REMOVED`] or [`SDL_EVENT_JOYSTICK_UPDATE_COMPLETE`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
}

/// Joystick battery level change event structure (event.jbattery.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_JoyBatteryEvent {
    /// [`SDL_EVENT_JOYSTICK_BATTERY_UPDATED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The joystick battery state
    pub state: SDL_PowerState,
    /// The joystick battery percent charge remaining
    pub percent: ::core::ffi::c_int,
}

/// Gamepad axis motion event structure (event.gaxis.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadAxisEvent {
    /// [`SDL_EVENT_GAMEPAD_AXIS_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The gamepad axis ([`SDL_GamepadAxis`])
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// The axis value (range: -32768 to 32767)
    pub value: Sint16,
    pub padding4: Uint16,
}

/// Gamepad button event structure (event.gbutton.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadButtonEvent {
    /// [`SDL_EVENT_GAMEPAD_BUTTON_DOWN`] or [`SDL_EVENT_GAMEPAD_BUTTON_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The gamepad button ([`SDL_GamepadButton`])
    pub button: Uint8,
    /// true if the button is pressed
    pub down: ::core::primitive::bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

/// Gamepad device event structure (event.gdevice.*)
///
/// Joysticks that are supported gamepads receive both an [`SDL_JoyDeviceEvent`]
/// and an [`SDL_GamepadDeviceEvent`].
///
/// SDL will send GAMEPAD_ADDED events for joysticks that are already plugged
/// in during [`SDL_Init()`] and are recognized as gamepads. It will also send
/// events for joysticks that get gamepad mappings at runtime.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_JoyDeviceEvent`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadDeviceEvent {
    /// [`SDL_EVENT_GAMEPAD_ADDED`], [`SDL_EVENT_GAMEPAD_REMOVED`], or [`SDL_EVENT_GAMEPAD_REMAPPED`], [`SDL_EVENT_GAMEPAD_UPDATE_COMPLETE`] or [`SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
}

/// Gamepad touchpad event structure (event.gtouchpad.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadTouchpadEvent {
    /// [`SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN`] or [`SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION`] or [`SDL_EVENT_GAMEPAD_TOUCHPAD_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The index of the touchpad
    pub touchpad: Sint32,
    /// The index of the finger on the touchpad
    pub finger: Sint32,
    /// Normalized in the range 0...1 with 0 being on the left
    pub x: ::core::ffi::c_float,
    /// Normalized in the range 0...1 with 0 being at the top
    pub y: ::core::ffi::c_float,
    /// Normalized in the range 0...1
    pub pressure: ::core::ffi::c_float,
}

/// Gamepad sensor event structure (event.gsensor.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GamepadSensorEvent {
    /// [`SDL_EVENT_GAMEPAD_SENSOR_UPDATE`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The joystick instance id
    pub which: SDL_JoystickID,
    /// The type of the sensor, one of the values of [`SDL_SensorType`]
    pub sensor: Sint32,
    /// Up to 3 values from the sensor, as defined in SDL_sensor.h
    pub data: [::core::ffi::c_float; 3],
    /// The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the system clock
    pub sensor_timestamp: Uint64,
}

/// Audio device event structure (event.adevice.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AudioDeviceEvent {
    /// [`SDL_EVENT_AUDIO_DEVICE_ADDED`], or [`SDL_EVENT_AUDIO_DEVICE_REMOVED`], or [`SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// [`SDL_AudioDeviceID`] for the device being added or removed or changing
    pub which: SDL_AudioDeviceID,
    /// false if a playback device, true if a recording device.
    pub recording: ::core::primitive::bool,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}

/// Camera device event structure (event.cdevice.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_CameraDeviceEvent {
    /// [`SDL_EVENT_CAMERA_DEVICE_ADDED`], [`SDL_EVENT_CAMERA_DEVICE_REMOVED`], [`SDL_EVENT_CAMERA_DEVICE_APPROVED`], [`SDL_EVENT_CAMERA_DEVICE_DENIED`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// [`SDL_CameraID`] for the device being added or removed or changing
    pub which: SDL_CameraID,
}

/// Touch finger event structure (event.tfinger.*)
///
/// Coordinates in this event are normalized. `x` and `y` are normalized to a
/// range between 0.0f and 1.0f, relative to the window, so (0,0) is the top
/// left and (1,1) is the bottom right. Delta coordinates `dx` and `dy` are
/// normalized in the ranges of -1.0f (traversed all the way from the bottom or
/// right to all the way up or left) to 1.0f (traversed all the way from the
/// top or left to all the way down or right).
///
/// Note that while the coordinates are _normalized_, they are not _clamped_,
/// which means in some circumstances you can get a value outside of this
/// range. For example, a renderer using logical presentation might give a
/// negative value when the touch is in the letterboxing. Some platforms might
/// report a touch outside of the window, which will also be outside of the
/// range.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TouchFingerEvent {
    /// [`SDL_EVENT_FINGER_MOTION`] or [`SDL_EVENT_FINGER_DOWN`] or [`SDL_EVENT_FINGER_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The touch device id
    pub touchID: SDL_TouchID,
    pub fingerID: SDL_FingerID,
    /// Normalized in the range 0...1
    pub x: ::core::ffi::c_float,
    /// Normalized in the range 0...1
    pub y: ::core::ffi::c_float,
    /// Normalized in the range -1...1
    pub dx: ::core::ffi::c_float,
    /// Normalized in the range -1...1
    pub dy: ::core::ffi::c_float,
    /// Normalized in the range 0...1
    pub pressure: ::core::ffi::c_float,
    /// The window underneath the finger, if any
    pub windowID: SDL_WindowID,
}

/// Pressure-sensitive pen proximity event structure (event.pmotion.*)
///
/// When a pen becomes visible to the system (it is close enough to a tablet,
/// etc), SDL will send an [`SDL_EVENT_PEN_PROXIMITY_IN`] event with the new pen's
/// ID. This ID is valid until the pen leaves proximity again (has been removed
/// from the tablet's area, the tablet has been unplugged, etc). If the same
/// pen reenters proximity again, it will be given a new ID.
///
/// Note that "proximity" means "close enough for the tablet to know the tool
/// is there." The pen touching and lifting off from the tablet while not
/// leaving the area are handled by [`SDL_EVENT_PEN_DOWN`] and [`SDL_EVENT_PEN_UP`].
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenProximityEvent {
    /// [`SDL_EVENT_PEN_PROXIMITY_IN`] or [`SDL_EVENT_PEN_PROXIMITY_OUT`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The pen instance id
    pub which: SDL_PenID,
}

/// Pressure-sensitive pen motion event structure (event.pmotion.*)
///
/// Depending on the hardware, you may get motion events when the pen is not
/// touching a tablet, for tracking a pen even when it isn't drawing. You
/// should listen for [`SDL_EVENT_PEN_DOWN`] and [`SDL_EVENT_PEN_UP`] events, or check
/// `pen_state & SDL_PEN_INPUT_DOWN` to decide if a pen is "drawing" when
/// dealing with pen motion.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenMotionEvent {
    /// [`SDL_EVENT_PEN_MOTION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The pen instance id
    pub which: SDL_PenID,
    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
}

/// Pressure-sensitive pen touched event structure (event.ptouch.*)
///
/// These events come when a pen touches a surface (a tablet, etc), or lifts
/// off from one.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenTouchEvent {
    /// [`SDL_EVENT_PEN_DOWN`] or [`SDL_EVENT_PEN_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with pen focus, if any
    pub windowID: SDL_WindowID,
    /// The pen instance id
    pub which: SDL_PenID,
    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
    /// true if eraser end is used (not all pens support this).
    pub eraser: ::core::primitive::bool,
    /// true if the pen is touching or false if the pen is lifted off
    pub down: ::core::primitive::bool,
}

/// Pressure-sensitive pen button event structure (event.pbutton.*)
///
/// This is for buttons on the pen itself that the user might click. The pen
/// itself pressing down to draw triggers a [`SDL_EVENT_PEN_DOWN`] event instead.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenButtonEvent {
    /// [`SDL_EVENT_PEN_BUTTON_DOWN`] or [`SDL_EVENT_PEN_BUTTON_UP`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with mouse focus, if any
    pub windowID: SDL_WindowID,
    /// The pen instance id
    pub which: SDL_PenID,
    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
    /// The pen button index (first button is 1).
    pub button: Uint8,
    /// true if the button is pressed
    pub down: ::core::primitive::bool,
}

/// Pressure-sensitive pen pressure / angle event structure (event.paxis.*)
///
/// You might get some of these events even if the pen isn't touching the
/// tablet.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PenAxisEvent {
    /// [`SDL_EVENT_PEN_AXIS`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window with pen focus, if any
    pub windowID: SDL_WindowID,
    /// The pen instance id
    pub which: SDL_PenID,
    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,
    /// X coordinate, relative to window
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window
    pub y: ::core::ffi::c_float,
    /// Axis that has changed
    pub axis: SDL_PenAxis,
    /// New value of axis
    pub value: ::core::ffi::c_float,
}

/// An event used to drop text or request a file open by the system
/// (event.drop.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_DropEvent {
    /// [`SDL_EVENT_DROP_BEGIN`] or [`SDL_EVENT_DROP_FILE`] or [`SDL_EVENT_DROP_TEXT`] or [`SDL_EVENT_DROP_COMPLETE`] or [`SDL_EVENT_DROP_POSITION`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The window that was dropped on, if any
    pub windowID: SDL_WindowID,
    /// X coordinate, relative to window (not on begin)
    pub x: ::core::ffi::c_float,
    /// Y coordinate, relative to window (not on begin)
    pub y: ::core::ffi::c_float,
    /// The source app that sent this drop event, or NULL if that isn't available
    pub source: *const ::core::ffi::c_char,
    /// The text for [`SDL_EVENT_DROP_TEXT`] and the file name for [`SDL_EVENT_DROP_FILE`], NULL for other events
    pub data: *const ::core::ffi::c_char,
}

/// An event triggered when the clipboard contents have changed
/// (event.clipboard.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_ClipboardEvent {
    /// [`SDL_EVENT_CLIPBOARD_UPDATE`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// are we owning the clipboard (internal update)
    pub owner: ::core::primitive::bool,
    /// number of mime types
    pub n_mime_types: Sint32,
    /// current mime types
    pub mime_types: *mut *const ::core::ffi::c_char,
}

/// Sensor event structure (event.sensor.*)
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_SensorEvent {
    /// [`SDL_EVENT_SENSOR_UPDATE`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The instance ID of the sensor
    pub which: SDL_SensorID,
    /// Up to 6 values from the sensor - additional values can be queried using [`SDL_GetSensorData()`]
    pub data: [::core::ffi::c_float; 6],
    /// The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the system clock
    pub sensor_timestamp: Uint64,
}

/// The "quit requested" event
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_QuitEvent {
    /// [`SDL_EVENT_QUIT`]
    pub r#type: SDL_EventType,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
}

/// A user-defined event type (event.user.*)
///
/// This event is unique; it is never created by SDL, but only by the
/// application. The event can be pushed onto the event queue using
/// [`SDL_PushEvent()`]. The contents of the structure members are completely up to
/// the programmer; the only requirement is that '''type''' is a value obtained
/// from [`SDL_RegisterEvents()`].
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_UserEvent {
    /// [`SDL_EVENT_USER`] through SDL_EVENT_LAST-1, Uint32 because these are not in the [`SDL_EventType`] enumeration
    pub r#type: Uint32,
    pub reserved: Uint32,
    /// In nanoseconds, populated using [`SDL_GetTicksNS()`]
    pub timestamp: Uint64,
    /// The associated window if any
    pub windowID: SDL_WindowID,
    /// User defined event code
    pub code: Sint32,
    /// User defined data pointer
    pub data1: *mut ::core::ffi::c_void,
    /// User defined data pointer
    pub data2: *mut ::core::ffi::c_void,
}

/// The structure for all events in SDL.
///
/// The [`SDL_Event`] structure is the core of all event handling in SDL. [`SDL_Event`]
/// is a union of all event structures used in SDL.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_Event {
    /// Event type, shared with all events, Uint32 to cover user events which are not in the [`SDL_EventType`] enumeration
    pub r#type: Uint32,
    /// Common event data
    pub common: SDL_CommonEvent,
    /// Display event data
    pub display: SDL_DisplayEvent,
    /// Window event data
    pub window: SDL_WindowEvent,
    /// Keyboard device change event data
    pub kdevice: SDL_KeyboardDeviceEvent,
    /// Keyboard event data
    pub key: SDL_KeyboardEvent,
    /// Text editing event data
    pub edit: SDL_TextEditingEvent,
    /// Text editing candidates event data
    pub edit_candidates: SDL_TextEditingCandidatesEvent,
    /// Text input event data
    pub text: SDL_TextInputEvent,
    /// Mouse device change event data
    pub mdevice: SDL_MouseDeviceEvent,
    /// Mouse motion event data
    pub motion: SDL_MouseMotionEvent,
    /// Mouse button event data
    pub button: SDL_MouseButtonEvent,
    /// Mouse wheel event data
    pub wheel: SDL_MouseWheelEvent,
    /// Joystick device change event data
    pub jdevice: SDL_JoyDeviceEvent,
    /// Joystick axis event data
    pub jaxis: SDL_JoyAxisEvent,
    /// Joystick ball event data
    pub jball: SDL_JoyBallEvent,
    /// Joystick hat event data
    pub jhat: SDL_JoyHatEvent,
    /// Joystick button event data
    pub jbutton: SDL_JoyButtonEvent,
    /// Joystick battery event data
    pub jbattery: SDL_JoyBatteryEvent,
    /// Gamepad device event data
    pub gdevice: SDL_GamepadDeviceEvent,
    /// Gamepad axis event data
    pub gaxis: SDL_GamepadAxisEvent,
    /// Gamepad button event data
    pub gbutton: SDL_GamepadButtonEvent,
    /// Gamepad touchpad event data
    pub gtouchpad: SDL_GamepadTouchpadEvent,
    /// Gamepad sensor event data
    pub gsensor: SDL_GamepadSensorEvent,
    /// Audio device event data
    pub adevice: SDL_AudioDeviceEvent,
    /// Camera device event data
    pub cdevice: SDL_CameraDeviceEvent,
    /// Sensor event data
    pub sensor: SDL_SensorEvent,
    /// Quit request event data
    pub quit: SDL_QuitEvent,
    /// Custom event data
    pub user: SDL_UserEvent,
    /// Touch finger event data
    pub tfinger: SDL_TouchFingerEvent,
    /// Pen proximity event data
    pub pproximity: SDL_PenProximityEvent,
    /// Pen tip touching event data
    pub ptouch: SDL_PenTouchEvent,
    /// Pen motion event data
    pub pmotion: SDL_PenMotionEvent,
    /// Pen button event data
    pub pbutton: SDL_PenButtonEvent,
    /// Pen axis event data
    pub paxis: SDL_PenAxisEvent,
    /// Drag and drop event data
    pub drop: SDL_DropEvent,
    /// Clipboard event data
    pub clipboard: SDL_ClipboardEvent,
    pub padding: [Uint8; 128],
}

const _: () = ::core::assert!(
    (::core::mem::size_of::<SDL_Event>() == crate::size_of_field!(SDL_Event, padding))
);

extern "C" {
    /// Pump the event loop, gathering events from the input devices.
    ///
    /// This function updates the event queue and internal input device state.
    ///
    /// [`SDL_PumpEvents()`] gathers all the pending input information from devices and
    /// places it in the event queue. Without calls to [`SDL_PumpEvents()`] no events
    /// would ever be placed on the queue. Often the need for calls to
    /// [`SDL_PumpEvents()`] is hidden from the user since [`SDL_PollEvent()`] and
    /// [`SDL_WaitEvent()`] implicitly call [`SDL_PumpEvents()`]. However, if you are not
    /// polling or waiting for events (e.g. you are filtering them), then you must
    /// call [`SDL_PumpEvents()`] to force an event queue update.
    ///
    /// ### Thread safety
    /// This should only be run in the thread that initialized the
    ///   video subsystem, and for extra safety, you should consider
    ///   only doing those things on the main thread in any case.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PollEvent`]
    /// - [`SDL_WaitEvent`]
    pub fn SDL_PumpEvents();
}

/// The type of action to request from [`SDL_PeepEvents()`].
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`ADDEVENT`](SDL_EventAction::ADDEVENT) | [`SDL_ADDEVENT`] | Add events to the back of the queue. |
/// | [`PEEKEVENT`](SDL_EventAction::PEEKEVENT) | [`SDL_PEEKEVENT`] | Check but don't remove events from the queue front. |
/// | [`GETEVENT`](SDL_EventAction::GETEVENT) | [`SDL_GETEVENT`] | Retrieve/remove events from the front of the queue. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_EventAction(pub ::core::ffi::c_int);

impl From<SDL_EventAction> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_EventAction) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_EventAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::ADDEVENT => "SDL_ADDEVENT",
            Self::PEEKEVENT => "SDL_PEEKEVENT",
            Self::GETEVENT => "SDL_GETEVENT",

            _ => return write!(f, "SDL_EventAction({})", self.0),
        })
    }
}

impl SDL_EventAction {
    /// Add events to the back of the queue.
    pub const ADDEVENT: Self = Self(0);
    /// Check but don't remove events from the queue front.
    pub const PEEKEVENT: Self = Self(1);
    /// Retrieve/remove events from the front of the queue.
    pub const GETEVENT: Self = Self(2);
}

/// Add events to the back of the queue.
pub const SDL_ADDEVENT: SDL_EventAction = SDL_EventAction::ADDEVENT;
/// Check but don't remove events from the queue front.
pub const SDL_PEEKEVENT: SDL_EventAction = SDL_EventAction::PEEKEVENT;
/// Retrieve/remove events from the front of the queue.
pub const SDL_GETEVENT: SDL_EventAction = SDL_EventAction::GETEVENT;

extern "C" {
    /// Check the event queue for messages and optionally return them.
    ///
    /// `action` may be any of the following:
    ///
    /// - [`SDL_ADDEVENT`]: up to `numevents` events will be added to the back of the
    ///   event queue.
    /// - [`SDL_PEEKEVENT`]: `numevents` events at the front of the event queue,
    ///   within the specified minimum and maximum type, will be returned to the
    ///   caller and will _not_ be removed from the queue. If you pass NULL for
    ///   `events`, then `numevents` is ignored and the total number of matching
    ///   events will be returned.
    /// - [`SDL_GETEVENT`]: up to `numevents` events at the front of the event queue,
    ///   within the specified minimum and maximum type, will be returned to the
    ///   caller and will be removed from the queue.
    ///
    /// You may have to call [`SDL_PumpEvents()`] before calling this function.
    /// Otherwise, the events may not be ready to be filtered when you call
    /// [`SDL_PeepEvents()`].
    ///
    /// ### Parameters
    /// - `events`: destination buffer for the retrieved events, may be NULL to
    ///   leave the events in the queue and return the number of events
    ///   that would have been stored.
    /// - `numevents`: if action is [`SDL_ADDEVENT`], the number of events to add
    ///   back to the event queue; if action is [`SDL_PEEKEVENT`] or
    ///   [`SDL_GETEVENT`], the maximum number of events to retrieve.
    /// - `action`: action to take; see \[\[#action|Remarks\]\] for details.
    /// - `minType`: minimum value of the event type to be considered;
    ///   [`SDL_EVENT_FIRST`] is a safe choice.
    /// - `maxType`: maximum value of the event type to be considered;
    ///   [`SDL_EVENT_LAST`] is a safe choice.
    ///
    /// ### Return value
    /// Returns the number of events actually stored or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PollEvent`]
    /// - [`SDL_PumpEvents`]
    /// - [`SDL_PushEvent`]
    pub fn SDL_PeepEvents(
        events: *mut SDL_Event,
        numevents: ::core::ffi::c_int,
        action: SDL_EventAction,
        minType: Uint32,
        maxType: Uint32,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Check for the existence of a certain event type in the event queue.
    ///
    /// If you need to check for a range of event types, use [`SDL_HasEvents()`]
    /// instead.
    ///
    /// ### Parameters
    /// - `type`: the type of event to be queried; see [`SDL_EventType`] for details.
    ///
    /// ### Return value
    /// Returns true if events matching `type` are present, or false if events
    ///   matching `type` are not present.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasEvents`]
    pub fn SDL_HasEvent(r#type: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Check for the existence of certain event types in the event queue.
    ///
    /// If you need to check for a single event type, use [`SDL_HasEvent()`] instead.
    ///
    /// ### Parameters
    /// - `minType`: the low end of event type to be queried, inclusive; see
    ///   [`SDL_EventType`] for details.
    /// - `maxType`: the high end of event type to be queried, inclusive; see
    ///   [`SDL_EventType`] for details.
    ///
    /// ### Return value
    /// Returns true if events with type >= `minType` and <= `maxType` are
    ///   present, or false if not.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasEvents`]
    pub fn SDL_HasEvents(minType: Uint32, maxType: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Clear events of a specific type from the event queue.
    ///
    /// This will unconditionally remove any events from the queue that match
    /// `type`. If you need to remove a range of event types, use [`SDL_FlushEvents()`]
    /// instead.
    ///
    /// It's also normal to just ignore events you don't care about in your event
    /// loop without calling this function.
    ///
    /// This function only affects currently queued events. If you want to make
    /// sure that all pending OS events are flushed, you can call [`SDL_PumpEvents()`]
    /// on the main thread immediately before the flush call.
    ///
    /// If you have user events with custom data that needs to be freed, you should
    /// use [`SDL_PeepEvents()`] to remove and clean up those events before calling
    /// this function.
    ///
    /// ### Parameters
    /// - `type`: the type of event to be cleared; see [`SDL_EventType`] for details.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_FlushEvents`]
    pub fn SDL_FlushEvent(r#type: Uint32);
}

extern "C" {
    /// Clear events of a range of types from the event queue.
    ///
    /// This will unconditionally remove any events from the queue that are in the
    /// range of `minType` to `maxType`, inclusive. If you need to remove a single
    /// event type, use [`SDL_FlushEvent()`] instead.
    ///
    /// It's also normal to just ignore events you don't care about in your event
    /// loop without calling this function.
    ///
    /// This function only affects currently queued events. If you want to make
    /// sure that all pending OS events are flushed, you can call [`SDL_PumpEvents()`]
    /// on the main thread immediately before the flush call.
    ///
    /// ### Parameters
    /// - `minType`: the low end of event type to be cleared, inclusive; see
    ///   [`SDL_EventType`] for details.
    /// - `maxType`: the high end of event type to be cleared, inclusive; see
    ///   [`SDL_EventType`] for details.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_FlushEvent`]
    pub fn SDL_FlushEvents(minType: Uint32, maxType: Uint32);
}

extern "C" {
    /// Poll for currently pending events.
    ///
    /// If `event` is not NULL, the next event is removed from the queue and stored
    /// in the [`SDL_Event`] structure pointed to by `event`. The 1 returned refers to
    /// this event, immediately stored in the SDL Event structure -- not an event
    /// to follow.
    ///
    /// If `event` is NULL, it simply returns 1 if there is an event in the queue,
    /// but will not remove it from the queue.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents()`], you can only call
    /// this function in the thread that set the video mode.
    ///
    /// [`SDL_PollEvent()`] is the favored way of receiving system events since it can
    /// be done from the main loop and does not suspend the main loop while waiting
    /// on an event to be posted.
    ///
    /// The common practice is to fully process the event queue once every frame,
    /// usually as a first step before updating the game's state:
    ///
    /// ```c
    /// while (game_is_still_running) {
    ///     SDL_Event event;
    ///     while (SDL_PollEvent(&event)) {  // poll until all events are handled!
    ///         // decide what to do with this event.
    ///     }
    ///
    ///     // update game state, draw the current frame
    /// }
    /// ```
    ///
    /// ### Parameters
    /// - `event`: the [`SDL_Event`] structure to be filled with the next event from
    ///   the queue, or NULL.
    ///
    /// ### Return value
    /// Returns true if this got an event or false if there are none available.
    ///
    /// ### Thread safety
    /// This should only be run in the thread that initialized the
    ///   video subsystem, and for extra safety, you should consider
    ///   only doing those things on the main thread in any case.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PushEvent`]
    /// - [`SDL_WaitEvent`]
    /// - [`SDL_WaitEventTimeout`]
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait indefinitely for the next available event.
    ///
    /// If `event` is not NULL, the next event is removed from the queue and stored
    /// in the [`SDL_Event`] structure pointed to by `event`.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents()`], you can only call
    /// this function in the thread that initialized the video subsystem.
    ///
    /// ### Parameters
    /// - `event`: the [`SDL_Event`] structure to be filled in with the next event
    ///   from the queue, or NULL.
    ///
    /// ### Return value
    /// Returns true on success or false if there was an error while waiting for
    ///   events; call [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// This should only be run in the thread that initialized the
    ///   video subsystem, and for extra safety, you should consider
    ///   only doing those things on the main thread in any case.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PollEvent`]
    /// - [`SDL_PushEvent`]
    /// - [`SDL_WaitEventTimeout`]
    pub fn SDL_WaitEvent(event: *mut SDL_Event) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait until the specified timeout (in milliseconds) for the next available
    /// event.
    ///
    /// If `event` is not NULL, the next event is removed from the queue and stored
    /// in the [`SDL_Event`] structure pointed to by `event`.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents()`], you can only call
    /// this function in the thread that initialized the video subsystem.
    ///
    /// The timeout is not guaranteed, the actual wait time could be longer due to
    /// system scheduling.
    ///
    /// ### Parameters
    /// - `event`: the [`SDL_Event`] structure to be filled in with the next event
    ///   from the queue, or NULL.
    /// - `timeoutMS`: the maximum number of milliseconds to wait for the next
    ///   available event.
    ///
    /// ### Return value
    /// Returns true if this got an event or false if the timeout elapsed without
    ///   any events available.
    ///
    /// ### Thread safety
    /// This should only be run in the thread that initialized the
    ///   video subsystem, and for extra safety, you should consider
    ///   only doing those things on the main thread in any case.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PollEvent`]
    /// - [`SDL_PushEvent`]
    /// - [`SDL_WaitEvent`]
    pub fn SDL_WaitEventTimeout(
        event: *mut SDL_Event,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Add an event to the event queue.
    ///
    /// The event queue can actually be used as a two way communication channel.
    /// Not only can events be read from the queue, but the user can also push
    /// their own events onto it. `event` is a pointer to the event structure you
    /// wish to push onto the queue. The event is copied into the queue, and the
    /// caller may dispose of the memory pointed to after [`SDL_PushEvent()`] returns.
    ///
    /// Note: Pushing device input events onto the queue doesn't modify the state
    /// of the device within SDL.
    ///
    /// Note: Events pushed onto the queue with [`SDL_PushEvent()`] get passed through
    /// the event filter but events added with [`SDL_PeepEvents()`] do not.
    ///
    /// For pushing application-specific events, please use [`SDL_RegisterEvents()`] to
    /// get an event type that does not conflict with other code that also wants
    /// its own custom event types.
    ///
    /// ### Parameters
    /// - `event`: the [`SDL_Event`] to be added to the queue.
    ///
    /// ### Return value
    /// Returns true on success, false if the event was filtered or on failure;
    ///   call [`SDL_GetError()`] for more information. A common reason for
    ///   error is the event queue being full.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PeepEvents`]
    /// - [`SDL_PollEvent`]
    /// - [`SDL_RegisterEvents`]
    pub fn SDL_PushEvent(event: *mut SDL_Event) -> ::core::primitive::bool;
}

/// A function pointer used for callbacks that watch the event queue.
///
/// ### Parameters
/// - `userdata`: what was passed as `userdata` to [`SDL_SetEventFilter()`] or
///   [`SDL_AddEventWatch`], etc.
/// - `event`: the event that triggered the callback.
///
/// ### Return value
/// Returns true to permit event to be added to the queue, and false to
///   disallow it. When used with [`SDL_AddEventWatch`], the return value is
///   ignored.
///
/// ### Thread safety
/// SDL may call this callback at any time from any thread; the
///   application is responsible for locking resources the callback
///   touches that need to be protected.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_SetEventFilter`]
/// - [`SDL_AddEventWatch`]
pub type SDL_EventFilter = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        event: *mut SDL_Event,
    ) -> ::core::primitive::bool,
>;

extern "C" {
    /// Set up a filter to process all events before they change internal state and
    /// are posted to the internal event queue.
    ///
    /// If the filter function returns true when called, then the event will be
    /// added to the internal queue. If it returns false, then the event will be
    /// dropped from the queue, but the internal state will still be updated. This
    /// allows selective filtering of dynamically arriving events.
    ///
    /// **WARNING**: Be very careful of what you do in the event filter function,
    /// as it may run in a different thread!
    ///
    /// On platforms that support it, if the quit event is generated by an
    /// interrupt signal (e.g. pressing Ctrl-C), it will be delivered to the
    /// application at the next event poll.
    ///
    /// There is one caveat when dealing with the [`SDL_QuitEvent`] event type. The
    /// event filter is only called when the window manager desires to close the
    /// application window. If the event filter returns 1, then the window will be
    /// closed, otherwise the window will remain open if possible.
    ///
    /// Note: Disabled events never make it to the event filter function; see
    /// [`SDL_SetEventEnabled()`].
    ///
    /// Note: If you just want to inspect events without filtering, you should use
    /// [`SDL_AddEventWatch()`] instead.
    ///
    /// Note: Events pushed onto the queue with [`SDL_PushEvent()`] get passed through
    /// the event filter, but events pushed onto the queue with [`SDL_PeepEvents()`] do
    /// not.
    ///
    /// ### Parameters
    /// - `filter`: an [`SDL_EventFilter`] function to call when an event happens.
    /// - `userdata`: a pointer that is passed to `filter`.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddEventWatch`]
    /// - [`SDL_SetEventEnabled`]
    /// - [`SDL_GetEventFilter`]
    /// - [`SDL_PeepEvents`]
    /// - [`SDL_PushEvent`]
    pub fn SDL_SetEventFilter(filter: SDL_EventFilter, userdata: *mut ::core::ffi::c_void);
}

extern "C" {
    /// Query the current event filter.
    ///
    /// This function can be used to "chain" filters, by saving the existing filter
    /// before replacing it with a function that will call that saved filter.
    ///
    /// ### Parameters
    /// - `filter`: the current callback function will be stored here.
    /// - `userdata`: the pointer that is passed to the current event filter will
    ///   be stored here.
    ///
    /// ### Return value
    /// Returns true on success or false if there is no event filter set.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetEventFilter`]
    pub fn SDL_GetEventFilter(
        filter: *mut SDL_EventFilter,
        userdata: *mut *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Add a callback to be triggered when an event is added to the event queue.
    ///
    /// `filter` will be called when an event happens, and its return value is
    /// ignored.
    ///
    /// **WARNING**: Be very careful of what you do in the event filter function,
    /// as it may run in a different thread!
    ///
    /// If the quit event is generated by a signal (e.g. SIGINT), it will bypass
    /// the internal queue and be delivered to the watch callback immediately, and
    /// arrive at the next event poll.
    ///
    /// Note: the callback is called for events posted by the user through
    /// [`SDL_PushEvent()`], but not for disabled events, nor for events by a filter
    /// callback set with [`SDL_SetEventFilter()`], nor for events posted by the user
    /// through [`SDL_PeepEvents()`].
    ///
    /// ### Parameters
    /// - `filter`: an [`SDL_EventFilter`] function to call when an event happens.
    /// - `userdata`: a pointer that is passed to `filter`.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_RemoveEventWatch`]
    /// - [`SDL_SetEventFilter`]
    pub fn SDL_AddEventWatch(
        filter: SDL_EventFilter,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove an event watch callback added with [`SDL_AddEventWatch()`].
    ///
    /// This function takes the same input as [`SDL_AddEventWatch()`] to identify and
    /// delete the corresponding callback.
    ///
    /// ### Parameters
    /// - `filter`: the function originally passed to [`SDL_AddEventWatch()`].
    /// - `userdata`: the pointer originally passed to [`SDL_AddEventWatch()`].
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AddEventWatch`]
    pub fn SDL_RemoveEventWatch(filter: SDL_EventFilter, userdata: *mut ::core::ffi::c_void);
}

extern "C" {
    /// Run a specific filter function on the current event queue, removing any
    /// events for which the filter returns false.
    ///
    /// See [`SDL_SetEventFilter()`] for more information. Unlike [`SDL_SetEventFilter()`],
    /// this function does not change the filter permanently, it only uses the
    /// supplied filter until this function returns.
    ///
    /// ### Parameters
    /// - `filter`: the [`SDL_EventFilter`] function to call when an event happens.
    /// - `userdata`: a pointer that is passed to `filter`.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetEventFilter`]
    /// - [`SDL_SetEventFilter`]
    pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *mut ::core::ffi::c_void);
}

extern "C" {
    /// Set the state of processing events by type.
    ///
    /// ### Parameters
    /// - `type`: the type of event; see [`SDL_EventType`] for details.
    /// - `enabled`: whether to process the event or not.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_EventEnabled`]
    pub fn SDL_SetEventEnabled(r#type: Uint32, enabled: ::core::primitive::bool);
}

extern "C" {
    /// Query the state of processing events by type.
    ///
    /// ### Parameters
    /// - `type`: the type of event; see [`SDL_EventType`] for details.
    ///
    /// ### Return value
    /// Returns true if the event is being processed, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetEventEnabled`]
    pub fn SDL_EventEnabled(r#type: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Allocate a set of user-defined events, and return the beginning event
    /// number for that set of events.
    ///
    /// ### Parameters
    /// - `numevents`: the number of events to be allocated.
    ///
    /// ### Return value
    /// Returns the beginning event number, or 0 if numevents is invalid or if
    ///   there are not enough user-defined events left.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PushEvent`]
    pub fn SDL_RegisterEvents(numevents: ::core::ffi::c_int) -> Uint32;
}

extern "C" {
    /// Get window associated with an event.
    ///
    /// ### Parameters
    /// - `event`: an event containing a `windowID`.
    ///
    /// ### Return value
    /// Returns the associated window on success or NULL if there is none.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PollEvent`]
    /// - [`SDL_WaitEvent`]
    /// - [`SDL_WaitEventTimeout`]
    pub fn SDL_GetWindowFromEvent(event: *const SDL_Event) -> *mut SDL_Window;
}

#[cfg(doc)]
use crate::everything::*;
