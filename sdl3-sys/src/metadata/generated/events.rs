//! Metadata for items in the `crate::events` module

use super::*;

pub const METADATA_SDL_EventType: Group = Group {
    module: "events",
    kind: GroupKind::Enum,
    name: "SDL_EventType",
    short_name: "EventType",
    doc: Some(
        "The types of events that can be delivered.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_EVENT_FIRST",
            short_name: "FIRST",
            doc: Some("Unused (do not remove)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_QUIT",
            short_name: "QUIT",
            doc: Some("User-requested quit\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_TERMINATING",
            short_name: "TERMINATING",
            doc: Some(
                "The application is being terminated by the OS. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationWillTerminate()\nCalled on Android in onDestroy()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_LOW_MEMORY",
            short_name: "LOW_MEMORY",
            doc: Some(
                "The application is low on memory, free memory if possible. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationDidReceiveMemoryWarning()\nCalled on Android in onTrimMemory()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WILL_ENTER_BACKGROUND",
            short_name: "WILL_ENTER_BACKGROUND",
            doc: Some(
                "The application is about to enter the background. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationWillResignActive()\nCalled on Android in onPause()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DID_ENTER_BACKGROUND",
            short_name: "DID_ENTER_BACKGROUND",
            doc: Some(
                "The application did enter the background and may not get CPU for some time. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationDidEnterBackground()\nCalled on Android in onPause()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WILL_ENTER_FOREGROUND",
            short_name: "WILL_ENTER_FOREGROUND",
            doc: Some(
                "The application is about to enter the foreground. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationWillEnterForeground()\nCalled on Android in onResume()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DID_ENTER_FOREGROUND",
            short_name: "DID_ENTER_FOREGROUND",
            doc: Some(
                "The application is now interactive. This event must be handled in a callback set with [`SDL_AddEventWatch()`].\nCalled on iOS in applicationDidBecomeActive()\nCalled on Android in onResume()\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_LOCALE_CHANGED",
            short_name: "LOCALE_CHANGED",
            doc: Some("The user's locale preferences have changed.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_SYSTEM_THEME_CHANGED",
            short_name: "SYSTEM_THEME_CHANGED",
            doc: Some("The system theme changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_ORIENTATION",
            short_name: "DISPLAY_ORIENTATION",
            doc: Some("Display orientation has changed to data1\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_ADDED",
            short_name: "DISPLAY_ADDED",
            doc: Some("Display has been added to the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_REMOVED",
            short_name: "DISPLAY_REMOVED",
            doc: Some("Display has been removed from the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_MOVED",
            short_name: "DISPLAY_MOVED",
            doc: Some("Display has changed position\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED",
            short_name: "DISPLAY_DESKTOP_MODE_CHANGED",
            doc: Some("Display has changed desktop mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED",
            short_name: "DISPLAY_CURRENT_MODE_CHANGED",
            doc: Some("Display has changed current mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED",
            short_name: "DISPLAY_CONTENT_SCALE_CHANGED",
            doc: Some("Display has changed content scale\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_USABLE_BOUNDS_CHANGED",
            short_name: "DISPLAY_USABLE_BOUNDS_CHANGED",
            doc: Some("Display has changed usable bounds\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_FIRST",
            short_name: "DISPLAY_FIRST",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DISPLAY_LAST",
            short_name: "DISPLAY_LAST",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_SHOWN",
            short_name: "WINDOW_SHOWN",
            doc: Some("Window has been shown\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_HIDDEN",
            short_name: "WINDOW_HIDDEN",
            doc: Some("Window has been hidden\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_EXPOSED",
            short_name: "WINDOW_EXPOSED",
            doc: Some(
                "Window has been exposed and should be redrawn, and can be redrawn directly from event watchers for this event.\ndata1 is 1 for live-resize expose events, 0 otherwise.\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_MOVED",
            short_name: "WINDOW_MOVED",
            doc: Some("Window has been moved to data1, data2\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_RESIZED",
            short_name: "WINDOW_RESIZED",
            doc: Some("Window has been resized to data1xdata2\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED",
            short_name: "WINDOW_PIXEL_SIZE_CHANGED",
            doc: Some("The pixel size of the window has changed to data1xdata2\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_METAL_VIEW_RESIZED",
            short_name: "WINDOW_METAL_VIEW_RESIZED",
            doc: Some("The pixel size of a Metal view associated with the window has changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_MINIMIZED",
            short_name: "WINDOW_MINIMIZED",
            doc: Some("Window has been minimized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_MAXIMIZED",
            short_name: "WINDOW_MAXIMIZED",
            doc: Some("Window has been maximized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_RESTORED",
            short_name: "WINDOW_RESTORED",
            doc: Some("Window has been restored to normal size and position\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_MOUSE_ENTER",
            short_name: "WINDOW_MOUSE_ENTER",
            doc: Some("Window has gained mouse focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_MOUSE_LEAVE",
            short_name: "WINDOW_MOUSE_LEAVE",
            doc: Some("Window has lost mouse focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_FOCUS_GAINED",
            short_name: "WINDOW_FOCUS_GAINED",
            doc: Some("Window has gained keyboard focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_FOCUS_LOST",
            short_name: "WINDOW_FOCUS_LOST",
            doc: Some("Window has lost keyboard focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_CLOSE_REQUESTED",
            short_name: "WINDOW_CLOSE_REQUESTED",
            doc: Some("The window manager requests that the window be closed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_HIT_TEST",
            short_name: "WINDOW_HIT_TEST",
            doc: Some("Window had a hit test that wasn't [`SDL_HITTEST_NORMAL`]\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_ICCPROF_CHANGED",
            short_name: "WINDOW_ICCPROF_CHANGED",
            doc: Some("The ICC profile of the window's display has changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_DISPLAY_CHANGED",
            short_name: "WINDOW_DISPLAY_CHANGED",
            doc: Some("Window has been moved to display data1\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED",
            short_name: "WINDOW_DISPLAY_SCALE_CHANGED",
            doc: Some("Window display scale has been changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_SAFE_AREA_CHANGED",
            short_name: "WINDOW_SAFE_AREA_CHANGED",
            doc: Some("The window safe area has been changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_OCCLUDED",
            short_name: "WINDOW_OCCLUDED",
            doc: Some("The window has been occluded\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_ENTER_FULLSCREEN",
            short_name: "WINDOW_ENTER_FULLSCREEN",
            doc: Some("The window has entered fullscreen mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_LEAVE_FULLSCREEN",
            short_name: "WINDOW_LEAVE_FULLSCREEN",
            doc: Some("The window has left fullscreen mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_DESTROYED",
            short_name: "WINDOW_DESTROYED",
            doc: Some(
                "The window with the associated ID is being or has been destroyed. If this message is being handled\nin an event watcher, the window handle is still valid and can still be used to retrieve any properties\nassociated with the window. Otherwise, the handle has already been destroyed and all resources\nassociated with it are invalid\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_HDR_STATE_CHANGED",
            short_name: "WINDOW_HDR_STATE_CHANGED",
            doc: Some("Window HDR properties have changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_FIRST",
            short_name: "WINDOW_FIRST",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_WINDOW_LAST",
            short_name: "WINDOW_LAST",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_KEY_DOWN",
            short_name: "KEY_DOWN",
            doc: Some("Key pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_KEY_UP",
            short_name: "KEY_UP",
            doc: Some("Key released\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_TEXT_EDITING",
            short_name: "TEXT_EDITING",
            doc: Some("Keyboard text editing (composition)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_TEXT_INPUT",
            short_name: "TEXT_INPUT",
            doc: Some("Keyboard text input\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_KEYMAP_CHANGED",
            short_name: "KEYMAP_CHANGED",
            doc: Some(
                "Keymap changed due to a system event such as an\ninput language or keyboard layout change.\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_KEYBOARD_ADDED",
            short_name: "KEYBOARD_ADDED",
            doc: Some("A new keyboard has been inserted into the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_KEYBOARD_REMOVED",
            short_name: "KEYBOARD_REMOVED",
            doc: Some("A keyboard has been removed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_TEXT_EDITING_CANDIDATES",
            short_name: "TEXT_EDITING_CANDIDATES",
            doc: Some("Keyboard text editing candidates\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_SCREEN_KEYBOARD_SHOWN",
            short_name: "SCREEN_KEYBOARD_SHOWN",
            doc: Some("The on-screen keyboard has been shown\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_SCREEN_KEYBOARD_HIDDEN",
            short_name: "SCREEN_KEYBOARD_HIDDEN",
            doc: Some("The on-screen keyboard has been hidden\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_MOTION",
            short_name: "MOUSE_MOTION",
            doc: Some("Mouse moved\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_BUTTON_DOWN",
            short_name: "MOUSE_BUTTON_DOWN",
            doc: Some("Mouse button pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_BUTTON_UP",
            short_name: "MOUSE_BUTTON_UP",
            doc: Some("Mouse button released\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_WHEEL",
            short_name: "MOUSE_WHEEL",
            doc: Some("Mouse wheel motion\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_ADDED",
            short_name: "MOUSE_ADDED",
            doc: Some("A new mouse has been inserted into the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_MOUSE_REMOVED",
            short_name: "MOUSE_REMOVED",
            doc: Some("A mouse has been removed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_AXIS_MOTION",
            short_name: "JOYSTICK_AXIS_MOTION",
            doc: Some("Joystick axis motion\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_BALL_MOTION",
            short_name: "JOYSTICK_BALL_MOTION",
            doc: Some("Joystick trackball motion\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_HAT_MOTION",
            short_name: "JOYSTICK_HAT_MOTION",
            doc: Some("Joystick hat position change\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_BUTTON_DOWN",
            short_name: "JOYSTICK_BUTTON_DOWN",
            doc: Some("Joystick button pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_BUTTON_UP",
            short_name: "JOYSTICK_BUTTON_UP",
            doc: Some("Joystick button released\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_ADDED",
            short_name: "JOYSTICK_ADDED",
            doc: Some("A new joystick has been inserted into the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_REMOVED",
            short_name: "JOYSTICK_REMOVED",
            doc: Some("An opened joystick has been removed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_BATTERY_UPDATED",
            short_name: "JOYSTICK_BATTERY_UPDATED",
            doc: Some("Joystick battery level change\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_JOYSTICK_UPDATE_COMPLETE",
            short_name: "JOYSTICK_UPDATE_COMPLETE",
            doc: Some("Joystick update is complete\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_AXIS_MOTION",
            short_name: "GAMEPAD_AXIS_MOTION",
            doc: Some("Gamepad axis motion\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_BUTTON_DOWN",
            short_name: "GAMEPAD_BUTTON_DOWN",
            doc: Some("Gamepad button pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_BUTTON_UP",
            short_name: "GAMEPAD_BUTTON_UP",
            doc: Some("Gamepad button released\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_ADDED",
            short_name: "GAMEPAD_ADDED",
            doc: Some("A new gamepad has been inserted into the system\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_REMOVED",
            short_name: "GAMEPAD_REMOVED",
            doc: Some("A gamepad has been removed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_REMAPPED",
            short_name: "GAMEPAD_REMAPPED",
            doc: Some("The gamepad mapping was updated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN",
            short_name: "GAMEPAD_TOUCHPAD_DOWN",
            doc: Some("Gamepad touchpad was touched\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION",
            short_name: "GAMEPAD_TOUCHPAD_MOTION",
            doc: Some("Gamepad touchpad finger was moved\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_TOUCHPAD_UP",
            short_name: "GAMEPAD_TOUCHPAD_UP",
            doc: Some("Gamepad touchpad finger was lifted\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_SENSOR_UPDATE",
            short_name: "GAMEPAD_SENSOR_UPDATE",
            doc: Some("Gamepad sensor was updated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_UPDATE_COMPLETE",
            short_name: "GAMEPAD_UPDATE_COMPLETE",
            doc: Some("Gamepad update is complete\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED",
            short_name: "GAMEPAD_STEAM_HANDLE_UPDATED",
            doc: Some("Gamepad Steam handle has changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_FINGER_DOWN",
            short_name: "FINGER_DOWN",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_FINGER_UP",
            short_name: "FINGER_UP",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_FINGER_MOTION",
            short_name: "FINGER_MOTION",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_FINGER_CANCELED",
            short_name: "FINGER_CANCELED",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PINCH_BEGIN",
            short_name: "PINCH_BEGIN",
            doc: Some("Pinch gesture started\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PINCH_UPDATE",
            short_name: "PINCH_UPDATE",
            doc: Some("Pinch gesture updated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PINCH_END",
            short_name: "PINCH_END",
            doc: Some("Pinch gesture ended\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_CLIPBOARD_UPDATE",
            short_name: "CLIPBOARD_UPDATE",
            doc: Some("The clipboard changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DROP_FILE",
            short_name: "DROP_FILE",
            doc: Some("The system requests a file open\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DROP_TEXT",
            short_name: "DROP_TEXT",
            doc: Some("text/plain drag-and-drop event\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DROP_BEGIN",
            short_name: "DROP_BEGIN",
            doc: Some("A new set of drops is beginning (NULL filename)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DROP_COMPLETE",
            short_name: "DROP_COMPLETE",
            doc: Some("Current set of drops is now complete (NULL filename)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_DROP_POSITION",
            short_name: "DROP_POSITION",
            doc: Some("Position while moving over the window\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_AUDIO_DEVICE_ADDED",
            short_name: "AUDIO_DEVICE_ADDED",
            doc: Some("A new audio device is available\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_AUDIO_DEVICE_REMOVED",
            short_name: "AUDIO_DEVICE_REMOVED",
            doc: Some("An audio device has been removed.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED",
            short_name: "AUDIO_DEVICE_FORMAT_CHANGED",
            doc: Some("An audio device's format has been changed by the system.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_SENSOR_UPDATE",
            short_name: "SENSOR_UPDATE",
            doc: Some("A sensor was updated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_PROXIMITY_IN",
            short_name: "PEN_PROXIMITY_IN",
            doc: Some("Pressure-sensitive pen has become available\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_PROXIMITY_OUT",
            short_name: "PEN_PROXIMITY_OUT",
            doc: Some("Pressure-sensitive pen has become unavailable\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_DOWN",
            short_name: "PEN_DOWN",
            doc: Some("Pressure-sensitive pen touched drawing surface\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_UP",
            short_name: "PEN_UP",
            doc: Some("Pressure-sensitive pen stopped touching drawing surface\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_BUTTON_DOWN",
            short_name: "PEN_BUTTON_DOWN",
            doc: Some("Pressure-sensitive pen button pressed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_BUTTON_UP",
            short_name: "PEN_BUTTON_UP",
            doc: Some("Pressure-sensitive pen button released\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_MOTION",
            short_name: "PEN_MOTION",
            doc: Some("Pressure-sensitive pen is moving on the tablet\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PEN_AXIS",
            short_name: "PEN_AXIS",
            doc: Some("Pressure-sensitive pen angle/pressure/etc changed\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_CAMERA_DEVICE_ADDED",
            short_name: "CAMERA_DEVICE_ADDED",
            doc: Some("A new camera device is available\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_CAMERA_DEVICE_REMOVED",
            short_name: "CAMERA_DEVICE_REMOVED",
            doc: Some("A camera device has been removed.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_CAMERA_DEVICE_APPROVED",
            short_name: "CAMERA_DEVICE_APPROVED",
            doc: Some("A camera device has been approved for use by the user.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_CAMERA_DEVICE_DENIED",
            short_name: "CAMERA_DEVICE_DENIED",
            doc: Some("A camera device has been denied for use by the user.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_RENDER_TARGETS_RESET",
            short_name: "RENDER_TARGETS_RESET",
            doc: Some("The render targets have been reset and their contents need to be updated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_RENDER_DEVICE_RESET",
            short_name: "RENDER_DEVICE_RESET",
            doc: Some("The device has been reset and all textures need to be recreated\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_RENDER_DEVICE_LOST",
            short_name: "RENDER_DEVICE_LOST",
            doc: Some("The device has been lost and can't be recovered.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PRIVATE0",
            short_name: "PRIVATE0",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PRIVATE1",
            short_name: "PRIVATE1",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PRIVATE2",
            short_name: "PRIVATE2",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_PRIVATE3",
            short_name: "PRIVATE3",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_POLL_SENTINEL",
            short_name: "POLL_SENTINEL",
            doc: Some("Signals the end of an event poll cycle\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_USER",
            short_name: "USER",
            doc: Some(
                "Events [`SDL_EVENT_USER`] through [`SDL_EVENT_LAST`] are for your use,\nand should be allocated with [`SDL_RegisterEvents()`]\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_LAST",
            short_name: "LAST",
            doc: Some("*  This last event is only for bounding internal arrays\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_EVENT_ENUM_PADDING",
            short_name: "ENUM_PADDING",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_EventAction: Group = Group {
    module: "events",
    kind: GroupKind::Enum,
    name: "SDL_EventAction",
    short_name: "EventAction",
    doc: Some(
        "The type of action to request from [`SDL_PeepEvents()`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ADDEVENT",
            short_name: "ADDEVENT",
            doc: Some("Add events to the back of the queue.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PEEKEVENT",
            short_name: "PEEKEVENT",
            doc: Some("Check but don't remove events from the queue front.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GETEVENT",
            short_name: "GETEVENT",
            doc: Some("Retrieve/remove events from the front of the queue.\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_CommonEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_CommonEvent",
    doc: Some(
        "Fields shared by every event\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "Event type, shared with all events, Uint32 to cover user events which are not in the [`SDL_EventType`] enumeration\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
    ],
};
pub const METADATA_SDL_DisplayEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_DisplayEvent",
    doc: Some(
        "Display state change event data (event.display.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("SDL_EVENT_DISPLAY_*\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "displayID",
            doc: Some("The associated display\n"),
            available_since: None,
            ty: "SDL_DisplayID",
        },
        Field {
            name: "data1",
            doc: Some("event dependent data\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "data2",
            doc: Some("event dependent data\n"),
            available_since: None,
            ty: "Sint32",
        },
    ],
};
pub const METADATA_SDL_WindowEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_WindowEvent",
    doc: Some(
        "Window state change event data (event.window.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("SDL_EVENT_WINDOW_*\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The associated window\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "data1",
            doc: Some("event dependent data\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "data2",
            doc: Some("event dependent data\n"),
            available_since: None,
            ty: "Sint32",
        },
    ],
};
pub const METADATA_SDL_KeyboardDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_KeyboardDeviceEvent",
    doc: Some(
        "Keyboard device event structure (event.kdevice.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_KEYBOARD_ADDED`] or [`SDL_EVENT_KEYBOARD_REMOVED`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The keyboard instance id\n"),
            available_since: None,
            ty: "SDL_KeyboardID",
        },
    ],
};
pub const METADATA_SDL_KeyboardEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_KeyboardEvent",
    doc: Some(
        "Keyboard button event structure (event.key.*)\n\nThe `key` is the base [`SDL_Keycode`] generated by pressing the `scancode`\nusing the current keyboard layout, applying any options specified in\n[`SDL_HINT_KEYCODE_OPTIONS`]. You can get the [`SDL_Keycode`] corresponding to the\nevent scancode and modifiers directly from the keyboard layout, bypassing\n[`SDL_HINT_KEYCODE_OPTIONS`], by calling [`SDL_GetKeyFromScancode()`].\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetKeyFromScancode`]\n- [`SDL_HINT_KEYCODE_OPTIONS`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_KEY_DOWN`] or [`SDL_EVENT_KEY_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with keyboard focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The keyboard instance id, or 0 if unknown or virtual\n"),
            available_since: None,
            ty: "SDL_KeyboardID",
        },
        Field {
            name: "scancode",
            doc: Some("SDL physical key code\n"),
            available_since: None,
            ty: "SDL_Scancode",
        },
        Field {
            name: "key",
            doc: Some("SDL virtual key code\n"),
            available_since: None,
            ty: "SDL_Keycode",
        },
        Field {
            name: "r#mod",
            doc: Some("current key modifiers\n"),
            available_since: None,
            ty: "SDL_Keymod",
        },
        Field {
            name: "raw",
            doc: Some("The platform dependent scancode for this event\n"),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "down",
            doc: Some("true if the key is pressed\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "repeat",
            doc: Some("true if this is a key repeat\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
    ],
};
pub const METADATA_SDL_TextEditingEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_TextEditingEvent",
    doc: Some(
        "Keyboard text editing event structure (event.edit.*)\n\nThe start cursor is the position, in UTF-8 characters, where new typing\nwill be inserted into the editing text. The length is the number of UTF-8\ncharacters that will be replaced by new typing.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_TEXT_EDITING`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with keyboard focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "text",
            doc: Some("The editing text\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "start",
            doc: Some("The start cursor of selected editing text, or -1 if not set\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "length",
            doc: Some("The length of selected editing text, or -1 if not set\n"),
            available_since: None,
            ty: "Sint32",
        },
    ],
};
pub const METADATA_SDL_TextEditingCandidatesEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_TextEditingCandidatesEvent",
    doc: Some(
        "Keyboard IME candidates event structure (event.edit_candidates.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_TEXT_EDITING_CANDIDATES`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with keyboard focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "candidates",
            doc: Some("The list of candidates, or NULL if there are no candidates available\n"),
            available_since: None,
            ty: "*const *const ::core::ffi::c_char",
        },
        Field {
            name: "num_candidates",
            doc: Some("The number of strings in `candidates`\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "selected_candidate",
            doc: Some("The index of the selected candidate, or -1 if no candidate is selected\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "horizontal",
            doc: Some("true if the list is horizontal, false if it's vertical\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding3",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_TextInputEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_TextInputEvent",
    doc: Some(
        "Keyboard text input event structure (event.text.*)\n\nThis event will never be delivered unless text input is enabled by calling\n[`SDL_StartTextInput()`]. Text input is disabled by default!\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_StartTextInput`]\n- [`SDL_StopTextInput`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_TEXT_INPUT`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with keyboard focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "text",
            doc: Some("The input text, UTF-8 encoded\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
    ],
};
pub const METADATA_SDL_MouseDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_MouseDeviceEvent",
    doc: Some(
        "Mouse device event structure (event.mdevice.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_MOUSE_ADDED`] or [`SDL_EVENT_MOUSE_REMOVED`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The mouse instance id\n"),
            available_since: None,
            ty: "SDL_MouseID",
        },
    ],
};
pub const METADATA_SDL_MouseMotionEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_MouseMotionEvent",
    doc: Some(
        "Mouse motion event structure (event.motion.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_MOUSE_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with mouse focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some(
                "The mouse instance id in relative mode, [`SDL_TOUCH_MOUSEID`] for touch events, or 0\n",
            ),
            available_since: None,
            ty: "SDL_MouseID",
        },
        Field {
            name: "state",
            doc: Some("The current button state\n"),
            available_since: None,
            ty: "SDL_MouseButtonFlags",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "xrel",
            doc: Some("The relative motion in the X direction\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "yrel",
            doc: Some("The relative motion in the Y direction\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_MouseButtonEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_MouseButtonEvent",
    doc: Some(
        "Mouse button event structure (event.button.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_MOUSE_BUTTON_DOWN`] or [`SDL_EVENT_MOUSE_BUTTON_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with mouse focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some(
                "The mouse instance id in relative mode, [`SDL_TOUCH_MOUSEID`] for touch events, or 0\n",
            ),
            available_since: None,
            ty: "SDL_MouseID",
        },
        Field {
            name: "button",
            doc: Some("The mouse button index\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "down",
            doc: Some("true if the button is pressed\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "clicks",
            doc: Some("1 for single-click, 2 for double-click, etc.\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_MouseWheelEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_MouseWheelEvent",
    doc: Some(
        "Mouse wheel event structure (event.wheel.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_MOUSE_WHEEL`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with mouse focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The mouse instance id in relative mode or 0\n"),
            available_since: None,
            ty: "SDL_MouseID",
        },
        Field {
            name: "x",
            doc: Some(
                "The amount scrolled horizontally, positive to the right and negative to the left\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some(
                "The amount scrolled vertically, positive away from the user and negative toward the user\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "direction",
            doc: Some(
                "Set to one of the SDL_MOUSEWHEEL_* defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back\n",
            ),
            available_since: None,
            ty: "SDL_MouseWheelDirection",
        },
        Field {
            name: "mouse_x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "mouse_y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "integer_x",
            doc: Some(
                "The amount scrolled horizontally, accumulated to whole scroll \"ticks\" (added in 3.2.12)\n",
            ),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "integer_y",
            doc: Some(
                "The amount scrolled vertically, accumulated to whole scroll \"ticks\" (added in 3.2.12)\n",
            ),
            available_since: None,
            ty: "Sint32",
        },
    ],
};
pub const METADATA_SDL_JoyAxisEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyAxisEvent",
    doc: Some(
        "Joystick axis motion event structure (event.jaxis.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_JOYSTICK_AXIS_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "axis",
            doc: Some("The joystick axis index\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding3",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "value",
            doc: Some("The axis value (range: -32768 to 32767)\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "padding4",
            doc: None,
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_JoyBallEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyBallEvent",
    doc: Some(
        "Joystick trackball motion event structure (event.jball.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_JOYSTICK_BALL_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "ball",
            doc: Some("The joystick trackball index\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding3",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "xrel",
            doc: Some("The relative motion in the X direction\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "yrel",
            doc: Some("The relative motion in the Y direction\n"),
            available_since: None,
            ty: "Sint16",
        },
    ],
};
pub const METADATA_SDL_JoyHatEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyHatEvent",
    doc: Some(
        "Joystick hat position change event structure (event.jhat.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_JOYSTICK_HAT_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "hat",
            doc: Some("The joystick hat index\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "value",
            doc: Some(
                "The hat position value.\n## See also\n- [`SDL_HAT_LEFTUP`] [`SDL_HAT_UP`] [`SDL_HAT_RIGHTUP`]\n- [`SDL_HAT_LEFT`] [`SDL_HAT_CENTERED`] [`SDL_HAT_RIGHT`]\n- [`SDL_HAT_LEFTDOWN`] [`SDL_HAT_DOWN`] [`SDL_HAT_RIGHTDOWN`]\n\nNote that zero means the POV is centered.\n",
            ),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_JoyButtonEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyButtonEvent",
    doc: Some(
        "Joystick button event structure (event.jbutton.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_JOYSTICK_BUTTON_DOWN`] or [`SDL_EVENT_JOYSTICK_BUTTON_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "button",
            doc: Some("The joystick button index\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "down",
            doc: Some("true if the button is pressed\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_JoyDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyDeviceEvent",
    doc: Some(
        "Joystick device event structure (event.jdevice.*)\n\nSDL will send JOYSTICK_ADDED events for devices that are already plugged in\nduring [`SDL_Init`].\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GamepadDeviceEvent`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_JOYSTICK_ADDED`] or [`SDL_EVENT_JOYSTICK_REMOVED`] or [`SDL_EVENT_JOYSTICK_UPDATE_COMPLETE`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
    ],
};
pub const METADATA_SDL_JoyBatteryEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_JoyBatteryEvent",
    doc: Some(
        "Joystick battery level change event structure (event.jbattery.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_JOYSTICK_BATTERY_UPDATED`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "state",
            doc: Some("The joystick battery state\n"),
            available_since: None,
            ty: "SDL_PowerState",
        },
        Field {
            name: "percent",
            doc: Some("The joystick battery percent charge remaining\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
pub const METADATA_SDL_GamepadAxisEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_GamepadAxisEvent",
    doc: Some(
        "Gamepad axis motion event structure (event.gaxis.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_GAMEPAD_AXIS_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "axis",
            doc: Some("The gamepad axis ([`SDL_GamepadAxis`])\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding3",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "value",
            doc: Some("The axis value (range: -32768 to 32767)\n"),
            available_since: None,
            ty: "Sint16",
        },
        Field {
            name: "padding4",
            doc: None,
            available_since: None,
            ty: "Uint16",
        },
    ],
};
pub const METADATA_SDL_GamepadButtonEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_GamepadButtonEvent",
    doc: Some(
        "Gamepad button event structure (event.gbutton.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_GAMEPAD_BUTTON_DOWN`] or [`SDL_EVENT_GAMEPAD_BUTTON_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "button",
            doc: Some("The gamepad button ([`SDL_GamepadButton`])\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "down",
            doc: Some("true if the button is pressed\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_GamepadDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_GamepadDeviceEvent",
    doc: Some(
        "Gamepad device event structure (event.gdevice.*)\n\nJoysticks that are supported gamepads receive both an [`SDL_JoyDeviceEvent`]\nand an [`SDL_GamepadDeviceEvent`].\n\nSDL will send GAMEPAD_ADDED events for joysticks that are already plugged\nin during [`SDL_Init()`] and are recognized as gamepads. It will also send\nevents for joysticks that get gamepad mappings at runtime.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_JoyDeviceEvent`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_GAMEPAD_ADDED`], [`SDL_EVENT_GAMEPAD_REMOVED`], or [`SDL_EVENT_GAMEPAD_REMAPPED`], [`SDL_EVENT_GAMEPAD_UPDATE_COMPLETE`] or [`SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
    ],
};
pub const METADATA_SDL_GamepadTouchpadEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_GamepadTouchpadEvent",
    doc: Some(
        "Gamepad touchpad event structure (event.gtouchpad.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN`] or [`SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION`] or [`SDL_EVENT_GAMEPAD_TOUCHPAD_UP`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "touchpad",
            doc: Some("The index of the touchpad\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "finger",
            doc: Some("The index of the finger on the touchpad\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "x",
            doc: Some("Normalized in the range 0...1 with 0 being on the left\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Normalized in the range 0...1 with 0 being at the top\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "pressure",
            doc: Some("Normalized in the range 0...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_GamepadSensorEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_GamepadSensorEvent",
    doc: Some(
        "Gamepad sensor event structure (event.gsensor.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_GAMEPAD_SENSOR_UPDATE`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The joystick instance id\n"),
            available_since: None,
            ty: "SDL_JoystickID",
        },
        Field {
            name: "sensor",
            doc: Some("The type of the sensor, one of the values of [`SDL_SensorType`]\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "data",
            doc: Some("Up to 3 values from the sensor, as defined in SDL_sensor.h\n"),
            available_since: None,
            ty: "[::core::ffi::c_float; 3]",
        },
        Field {
            name: "sensor_timestamp",
            doc: Some(
                "The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the system clock\n",
            ),
            available_since: None,
            ty: "Uint64",
        },
    ],
};
pub const METADATA_SDL_AudioDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_AudioDeviceEvent",
    doc: Some(
        "Audio device event structure (event.adevice.*)\n\nNote that SDL will send a [`SDL_EVENT_AUDIO_DEVICE_ADDED`] event for every\ndevice it discovers during initialization. After that, this event will only\narrive when a device is hotplugged during the program's run.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_AUDIO_DEVICE_ADDED`], or [`SDL_EVENT_AUDIO_DEVICE_REMOVED`], or [`SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("[`SDL_AudioDeviceID`] for the device being added or removed or changing\n"),
            available_since: None,
            ty: "SDL_AudioDeviceID",
        },
        Field {
            name: "recording",
            doc: Some("false if a playback device, true if a recording device.\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "padding1",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding2",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "padding3",
            doc: None,
            available_since: None,
            ty: "Uint8",
        },
    ],
};
pub const METADATA_SDL_CameraDeviceEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_CameraDeviceEvent",
    doc: Some(
        "Camera device event structure (event.cdevice.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_CAMERA_DEVICE_ADDED`], [`SDL_EVENT_CAMERA_DEVICE_REMOVED`], [`SDL_EVENT_CAMERA_DEVICE_APPROVED`], [`SDL_EVENT_CAMERA_DEVICE_DENIED`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("[`SDL_CameraID`] for the device being added or removed or changing\n"),
            available_since: None,
            ty: "SDL_CameraID",
        },
    ],
};
pub const METADATA_SDL_RenderEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_RenderEvent",
    doc: Some(
        "Renderer event structure (event.render.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_RENDER_TARGETS_RESET`], [`SDL_EVENT_RENDER_DEVICE_RESET`], [`SDL_EVENT_RENDER_DEVICE_LOST`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window containing the renderer in question.\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
    ],
};
pub const METADATA_SDL_TouchFingerEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_TouchFingerEvent",
    doc: Some(
        "Touch finger event structure (event.tfinger.*)\n\nCoordinates in this event are normalized. `x` and `y` are normalized to a\nrange between 0.0f and 1.0f, relative to the window, so (0,0) is the top\nleft and (1,1) is the bottom right. Delta coordinates `dx` and `dy` are\nnormalized in the ranges of -1.0f (traversed all the way from the bottom or\nright to all the way up or left) to 1.0f (traversed all the way from the\ntop or left to all the way down or right).\n\nNote that while the coordinates are _normalized_, they are not _clamped_,\nwhich means in some circumstances you can get a value outside of this\nrange. For example, a renderer using logical presentation might give a\nnegative value when the touch is in the letterboxing. Some platforms might\nreport a touch outside of the window, which will also be outside of the\nrange.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_FINGER_DOWN`], [`SDL_EVENT_FINGER_UP`], [`SDL_EVENT_FINGER_MOTION`], or [`SDL_EVENT_FINGER_CANCELED`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "touchID",
            doc: Some("The touch device id\n"),
            available_since: None,
            ty: "SDL_TouchID",
        },
        Field {
            name: "fingerID",
            doc: None,
            available_since: None,
            ty: "SDL_FingerID",
        },
        Field {
            name: "x",
            doc: Some("Normalized in the range 0...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Normalized in the range 0...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "dx",
            doc: Some("Normalized in the range -1...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "dy",
            doc: Some("Normalized in the range -1...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "pressure",
            doc: Some("Normalized in the range 0...1\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "windowID",
            doc: Some("The window underneath the finger, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
    ],
};
pub const METADATA_SDL_PinchFingerEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PinchFingerEvent",
    doc: Some("* Pinch event structure (event.pinch.*)\n"),
    available_since: None,
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "::SDL_EVENT_PINCH_BEGIN or ::SDL_EVENT_PINCH_UPDATE or ::SDL_EVENT_PINCH_END\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "scale",
            doc: Some(
                "The scale change since the last [`SDL_EVENT_PINCH_UPDATE`]. Scale < 1 is \"zoom out\". Scale > 1 is \"zoom in\".\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "windowID",
            doc: Some("The window underneath the finger, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
    ],
};
pub const METADATA_SDL_PenProximityEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PenProximityEvent",
    doc: Some(
        "Pressure-sensitive pen proximity event structure (event.pproximity.*)\n\nWhen a pen becomes visible to the system (it is close enough to a tablet,\netc), SDL will send an [`SDL_EVENT_PEN_PROXIMITY_IN`] event with the new pen's\nID. This ID is valid until the pen leaves proximity again (has been removed\nfrom the tablet's area, the tablet has been unplugged, etc). If the same\npen reenters proximity again, it will be given a new ID.\n\nNote that \"proximity\" means \"close enough for the tablet to know the tool\nis there.\" The pen touching and lifting off from the tablet while not\nleaving the area are handled by [`SDL_EVENT_PEN_DOWN`] and [`SDL_EVENT_PEN_UP`].\n\nNot all platforms have a window associated with the pen during proximity\nevents. Some wait until motion/button/etc events to offer this info.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_PEN_PROXIMITY_IN`] or [`SDL_EVENT_PEN_PROXIMITY_OUT`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with pen focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The pen instance id\n"),
            available_since: None,
            ty: "SDL_PenID",
        },
    ],
};
pub const METADATA_SDL_PenMotionEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PenMotionEvent",
    doc: Some(
        "Pressure-sensitive pen motion event structure (event.pmotion.*)\n\nDepending on the hardware, you may get motion events when the pen is not\ntouching a tablet, for tracking a pen even when it isn't drawing. You\nshould listen for [`SDL_EVENT_PEN_DOWN`] and [`SDL_EVENT_PEN_UP`] events, or check\n`pen_state & SDL_PEN_INPUT_DOWN` to decide if a pen is \"drawing\" when\ndealing with pen motion.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_PEN_MOTION`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with pen focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The pen instance id\n"),
            available_since: None,
            ty: "SDL_PenID",
        },
        Field {
            name: "pen_state",
            doc: Some("Complete pen input state at time of event\n"),
            available_since: None,
            ty: "SDL_PenInputFlags",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_PenTouchEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PenTouchEvent",
    doc: Some(
        "Pressure-sensitive pen touched event structure (event.ptouch.*)\n\nThese events come when a pen touches a surface (a tablet, etc), or lifts\noff from one.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_PEN_DOWN`] or [`SDL_EVENT_PEN_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with pen focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The pen instance id\n"),
            available_since: None,
            ty: "SDL_PenID",
        },
        Field {
            name: "pen_state",
            doc: Some("Complete pen input state at time of event\n"),
            available_since: None,
            ty: "SDL_PenInputFlags",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "eraser",
            doc: Some("true if eraser end is used (not all pens support this).\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "down",
            doc: Some("true if the pen is touching or false if the pen is lifted off\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
    ],
};
pub const METADATA_SDL_PenButtonEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PenButtonEvent",
    doc: Some(
        "Pressure-sensitive pen button event structure (event.pbutton.*)\n\nThis is for buttons on the pen itself that the user might click. The pen\nitself pressing down to draw triggers a [`SDL_EVENT_PEN_DOWN`] event instead.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_PEN_BUTTON_DOWN`] or [`SDL_EVENT_PEN_BUTTON_UP`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with mouse focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The pen instance id\n"),
            available_since: None,
            ty: "SDL_PenID",
        },
        Field {
            name: "pen_state",
            doc: Some("Complete pen input state at time of event\n"),
            available_since: None,
            ty: "SDL_PenInputFlags",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "button",
            doc: Some("The pen button index (first button is 1).\n"),
            available_since: None,
            ty: "Uint8",
        },
        Field {
            name: "down",
            doc: Some("true if the button is pressed\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
    ],
};
pub const METADATA_SDL_PenAxisEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_PenAxisEvent",
    doc: Some(
        "Pressure-sensitive pen pressure / angle event structure (event.paxis.*)\n\nYou might get some of these events even if the pen isn't touching the\ntablet.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_PEN_AXIS`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window with pen focus, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "which",
            doc: Some("The pen instance id\n"),
            available_since: None,
            ty: "SDL_PenID",
        },
        Field {
            name: "pen_state",
            doc: Some("Complete pen input state at time of event\n"),
            available_since: None,
            ty: "SDL_PenInputFlags",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "axis",
            doc: Some("Axis that has changed\n"),
            available_since: None,
            ty: "SDL_PenAxis",
        },
        Field {
            name: "value",
            doc: Some("New value of axis\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
    ],
};
pub const METADATA_SDL_DropEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_DropEvent",
    doc: Some(
        "An event used to drop text or request a file open by the system\n(event.drop.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_DROP_BEGIN`] or [`SDL_EVENT_DROP_FILE`] or [`SDL_EVENT_DROP_TEXT`] or [`SDL_EVENT_DROP_COMPLETE`] or [`SDL_EVENT_DROP_POSITION`]\n",
            ),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The window that was dropped on, if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "x",
            doc: Some("X coordinate, relative to window (not on begin)\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "y",
            doc: Some("Y coordinate, relative to window (not on begin)\n"),
            available_since: None,
            ty: "::core::ffi::c_float",
        },
        Field {
            name: "source",
            doc: Some(
                "The source app that sent this drop event, or NULL if that isn't available\n",
            ),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "data",
            doc: Some(
                "The text for [`SDL_EVENT_DROP_TEXT`] and the file name for [`SDL_EVENT_DROP_FILE`], NULL for other events\n",
            ),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
    ],
};
pub const METADATA_SDL_ClipboardEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_ClipboardEvent",
    doc: Some(
        "An event triggered when the clipboard contents have changed\n(event.clipboard.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_CLIPBOARD_UPDATE`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "owner",
            doc: Some("are we owning the clipboard (internal update)\n"),
            available_since: None,
            ty: "::core::primitive::bool",
        },
        Field {
            name: "num_mime_types",
            doc: Some("number of mime types\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "mime_types",
            doc: Some("current mime types\n"),
            available_since: None,
            ty: "*mut *const ::core::ffi::c_char",
        },
    ],
};
pub const METADATA_SDL_SensorEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_SensorEvent",
    doc: Some(
        "Sensor event structure (event.sensor.*)\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_SENSOR_UPDATE`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "which",
            doc: Some("The instance ID of the sensor\n"),
            available_since: None,
            ty: "SDL_SensorID",
        },
        Field {
            name: "data",
            doc: Some(
                "Up to 6 values from the sensor - additional values can be queried using [`SDL_GetSensorData()`]\n",
            ),
            available_since: None,
            ty: "[::core::ffi::c_float; 6]",
        },
        Field {
            name: "sensor_timestamp",
            doc: Some(
                "The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the system clock\n",
            ),
            available_since: None,
            ty: "Uint64",
        },
    ],
};
pub const METADATA_SDL_QuitEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_QuitEvent",
    doc: Some(
        "The \"quit requested\" event\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some("[`SDL_EVENT_QUIT`]\n"),
            available_since: None,
            ty: "SDL_EventType",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
    ],
};
pub const METADATA_SDL_UserEvent: Struct = Struct {
    module: "events",
    kind: StructKind::Struct,
    name: "SDL_UserEvent",
    doc: Some(
        "A user-defined event type (event.user.*)\n\nThis event is unique; it is never created by SDL, but only by the\napplication. The event can be pushed onto the event queue using\n[`SDL_PushEvent()`]. The contents of the structure members are completely up to\nthe programmer; the only requirement is that '''type''' is a value obtained\nfrom [`SDL_RegisterEvents()`].\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "[`SDL_EVENT_USER`] through [`SDL_EVENT_LAST`], Uint32 because these are not in the [`SDL_EventType`] enumeration\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "reserved",
            doc: None,
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "timestamp",
            doc: Some("In nanoseconds, populated using [`SDL_GetTicksNS()`]\n"),
            available_since: None,
            ty: "Uint64",
        },
        Field {
            name: "windowID",
            doc: Some("The associated window if any\n"),
            available_since: None,
            ty: "SDL_WindowID",
        },
        Field {
            name: "code",
            doc: Some("User defined event code\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "data1",
            doc: Some("User defined data pointer\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
        Field {
            name: "data2",
            doc: Some("User defined data pointer\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_void",
        },
    ],
};
pub const METADATA_SDL_Event: Struct = Struct {
    module: "events",
    kind: StructKind::Union,
    name: "SDL_Event",
    doc: Some(
        "The structure for all events in SDL.\n\nThe [`SDL_Event`] structure is the core of all event handling in SDL. [`SDL_Event`]\nis a union of all event structures used in SDL.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## Notes for `sdl3-sys`\nThis struct has padding fields which shouldn't be accessed directly; use struct update syntax with e.g. `..Default::default()` for manual construction.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "r#type",
            doc: Some(
                "Event type, shared with all events, Uint32 to cover user events which are not in the [`SDL_EventType`] enumeration\n",
            ),
            available_since: None,
            ty: "Uint32",
        },
        Field {
            name: "common",
            doc: Some("Common event data\n"),
            available_since: None,
            ty: "SDL_CommonEvent",
        },
        Field {
            name: "display",
            doc: Some("Display event data\n"),
            available_since: None,
            ty: "SDL_DisplayEvent",
        },
        Field {
            name: "window",
            doc: Some("Window event data\n"),
            available_since: None,
            ty: "SDL_WindowEvent",
        },
        Field {
            name: "kdevice",
            doc: Some("Keyboard device change event data\n"),
            available_since: None,
            ty: "SDL_KeyboardDeviceEvent",
        },
        Field {
            name: "key",
            doc: Some("Keyboard event data\n"),
            available_since: None,
            ty: "SDL_KeyboardEvent",
        },
        Field {
            name: "edit",
            doc: Some("Text editing event data\n"),
            available_since: None,
            ty: "SDL_TextEditingEvent",
        },
        Field {
            name: "edit_candidates",
            doc: Some("Text editing candidates event data\n"),
            available_since: None,
            ty: "SDL_TextEditingCandidatesEvent",
        },
        Field {
            name: "text",
            doc: Some("Text input event data\n"),
            available_since: None,
            ty: "SDL_TextInputEvent",
        },
        Field {
            name: "mdevice",
            doc: Some("Mouse device change event data\n"),
            available_since: None,
            ty: "SDL_MouseDeviceEvent",
        },
        Field {
            name: "motion",
            doc: Some("Mouse motion event data\n"),
            available_since: None,
            ty: "SDL_MouseMotionEvent",
        },
        Field {
            name: "button",
            doc: Some("Mouse button event data\n"),
            available_since: None,
            ty: "SDL_MouseButtonEvent",
        },
        Field {
            name: "wheel",
            doc: Some("Mouse wheel event data\n"),
            available_since: None,
            ty: "SDL_MouseWheelEvent",
        },
        Field {
            name: "jdevice",
            doc: Some("Joystick device change event data\n"),
            available_since: None,
            ty: "SDL_JoyDeviceEvent",
        },
        Field {
            name: "jaxis",
            doc: Some("Joystick axis event data\n"),
            available_since: None,
            ty: "SDL_JoyAxisEvent",
        },
        Field {
            name: "jball",
            doc: Some("Joystick ball event data\n"),
            available_since: None,
            ty: "SDL_JoyBallEvent",
        },
        Field {
            name: "jhat",
            doc: Some("Joystick hat event data\n"),
            available_since: None,
            ty: "SDL_JoyHatEvent",
        },
        Field {
            name: "jbutton",
            doc: Some("Joystick button event data\n"),
            available_since: None,
            ty: "SDL_JoyButtonEvent",
        },
        Field {
            name: "jbattery",
            doc: Some("Joystick battery event data\n"),
            available_since: None,
            ty: "SDL_JoyBatteryEvent",
        },
        Field {
            name: "gdevice",
            doc: Some("Gamepad device event data\n"),
            available_since: None,
            ty: "SDL_GamepadDeviceEvent",
        },
        Field {
            name: "gaxis",
            doc: Some("Gamepad axis event data\n"),
            available_since: None,
            ty: "SDL_GamepadAxisEvent",
        },
        Field {
            name: "gbutton",
            doc: Some("Gamepad button event data\n"),
            available_since: None,
            ty: "SDL_GamepadButtonEvent",
        },
        Field {
            name: "gtouchpad",
            doc: Some("Gamepad touchpad event data\n"),
            available_since: None,
            ty: "SDL_GamepadTouchpadEvent",
        },
        Field {
            name: "gsensor",
            doc: Some("Gamepad sensor event data\n"),
            available_since: None,
            ty: "SDL_GamepadSensorEvent",
        },
        Field {
            name: "adevice",
            doc: Some("Audio device event data\n"),
            available_since: None,
            ty: "SDL_AudioDeviceEvent",
        },
        Field {
            name: "cdevice",
            doc: Some("Camera device event data\n"),
            available_since: None,
            ty: "SDL_CameraDeviceEvent",
        },
        Field {
            name: "sensor",
            doc: Some("Sensor event data\n"),
            available_since: None,
            ty: "SDL_SensorEvent",
        },
        Field {
            name: "quit",
            doc: Some("Quit request event data\n"),
            available_since: None,
            ty: "SDL_QuitEvent",
        },
        Field {
            name: "user",
            doc: Some("Custom event data\n"),
            available_since: None,
            ty: "SDL_UserEvent",
        },
        Field {
            name: "tfinger",
            doc: Some("Touch finger event data\n"),
            available_since: None,
            ty: "SDL_TouchFingerEvent",
        },
        Field {
            name: "pinch",
            doc: Some("Pinch event data\n"),
            available_since: None,
            ty: "SDL_PinchFingerEvent",
        },
        Field {
            name: "pproximity",
            doc: Some("Pen proximity event data\n"),
            available_since: None,
            ty: "SDL_PenProximityEvent",
        },
        Field {
            name: "ptouch",
            doc: Some("Pen tip touching event data\n"),
            available_since: None,
            ty: "SDL_PenTouchEvent",
        },
        Field {
            name: "pmotion",
            doc: Some("Pen motion event data\n"),
            available_since: None,
            ty: "SDL_PenMotionEvent",
        },
        Field {
            name: "pbutton",
            doc: Some("Pen button event data\n"),
            available_since: None,
            ty: "SDL_PenButtonEvent",
        },
        Field {
            name: "paxis",
            doc: Some("Pen axis event data\n"),
            available_since: None,
            ty: "SDL_PenAxisEvent",
        },
        Field {
            name: "render",
            doc: Some("Render event data\n"),
            available_since: None,
            ty: "SDL_RenderEvent",
        },
        Field {
            name: "drop",
            doc: Some("Drag and drop event data\n"),
            available_since: None,
            ty: "SDL_DropEvent",
        },
        Field {
            name: "clipboard",
            doc: Some("Clipboard event data\n"),
            available_since: None,
            ty: "SDL_ClipboardEvent",
        },
        Field {
            name: "padding",
            doc: None,
            available_since: None,
            ty: "[Uint8; 128]",
        },
    ],
};
