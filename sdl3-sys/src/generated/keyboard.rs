//! SDL keyboard management.

use super::stdinc::*;

use super::error::*;

use super::keycode::*;

use super::properties::*;

use super::rect::*;

use super::scancode::*;

use super::video::*;

/// This is a unique ID for a keyboard for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// If the keyboard is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_KeyboardID = Uint32;

extern "C" {
    /// Return whether a keyboard is currently connected.
    ///
    /// ### Return value
    /// Returns true if a keyboard is connected, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyboards`]
    pub fn SDL_HasKeyboard() -> ::core::primitive::bool;
}

extern "C" {
    /// Get a list of currently connected keyboards.
    ///
    /// Note that this will include any device or virtual driver that includes
    /// keyboard functionality, including some mice, KVM switches, motherboard
    /// power buttons, etc. You should wait for input from a device before you
    /// consider it actively in use.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of keyboards returned, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns a 0 terminated array of keyboards instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyboardNameForID`]
    /// - [`SDL_HasKeyboard`]
    pub fn SDL_GetKeyboards(count: *mut ::core::ffi::c_int) -> *mut SDL_KeyboardID;
}

extern "C" {
    /// Get the name of a keyboard.
    ///
    /// This function returns "" if the keyboard doesn't have a name.
    ///
    /// ### Parameters
    /// - `instance_id`: the keyboard instance ID.
    ///
    /// ### Return value
    /// Returns the name of the selected keyboard or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyboards`]
    pub fn SDL_GetKeyboardNameForID(instance_id: SDL_KeyboardID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query the window which currently has keyboard focus.
    ///
    /// ### Return value
    /// Returns the window with keyboard focus.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetKeyboardFocus() -> *mut SDL_Window;
}

extern "C" {
    /// Get a snapshot of the current state of the keyboard.
    ///
    /// The pointer returned is a pointer to an internal SDL array. It will be
    /// valid for the whole lifetime of the application and should not be freed by
    /// the caller.
    ///
    /// A array element with a value of true means that the key is pressed and a
    /// value of false means that it is not. Indexes into this array are obtained
    /// by using [`SDL_Scancode`] values.
    ///
    /// Use [`SDL_PumpEvents()`] to update the state array.
    ///
    /// This function gives you the current state after all events have been
    /// processed, so if a key or button has been pressed and released before you
    /// process events, then the pressed state will never show up in the
    /// [`SDL_GetKeyboardState()`] calls.
    ///
    /// Note: This function doesn't take into account whether shift has been
    /// pressed or not.
    ///
    /// ### Parameters
    /// - `numkeys`: if non-NULL, receives the length of the returned array.
    ///
    /// ### Return value
    /// Returns a pointer to an array of key states.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_PumpEvents`]
    /// - [`SDL_ResetKeyboard`]
    pub fn SDL_GetKeyboardState(numkeys: *mut ::core::ffi::c_int)
        -> *const ::core::primitive::bool;
}

extern "C" {
    /// Clear the state of the keyboard.
    ///
    /// This function will generate key up events for all pressed keys.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyboardState`]
    pub fn SDL_ResetKeyboard();
}

extern "C" {
    /// Get the current key modifier state for the keyboard.
    ///
    /// ### Return value
    /// Returns an OR'd combination of the modifier keys for the keyboard. See
    ///   [`SDL_Keymod`] for details.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyboardState`]
    /// - [`SDL_SetModState`]
    pub fn SDL_GetModState() -> SDL_Keymod;
}

extern "C" {
    /// Set the current key modifier state for the keyboard.
    ///
    /// The inverse of [`SDL_GetModState()`], [`SDL_SetModState()`] allows you to impose
    /// modifier key states on your application. Simply pass your desired modifier
    /// states into `modstate`. This value may be a bitwise, OR'd combination of
    /// [`SDL_Keymod`] values.
    ///
    /// This does not change the keyboard state, only the key modifier flags that
    /// SDL reports.
    ///
    /// ### Parameters
    /// - `modstate`: the desired [`SDL_Keymod`] for the keyboard.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetModState`]
    pub fn SDL_SetModState(modstate: SDL_Keymod);
}

extern "C" {
    /// Get the key code corresponding to the given scancode according to the
    /// current keyboard layout.
    ///
    /// If you want to get the keycode as it would be delivered in key events,
    /// including options specified in [`SDL_HINT_KEYCODE_OPTIONS`], then you should
    /// pass `key_event` as true. Otherwise this function simply translates the
    /// scancode based on the given modifier state.
    ///
    /// ### Parameters
    /// - `scancode`: the desired [`SDL_Scancode`] to query.
    /// - `modstate`: the modifier state to use when translating the scancode to
    ///   a keycode.
    /// - `key_event`: true if the keycode will be used in key events.
    ///
    /// ### Return value
    /// Returns the [`SDL_Keycode`] that corresponds to the given [`SDL_Scancode`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyName`]
    /// - [`SDL_GetScancodeFromKey`]
    pub fn SDL_GetKeyFromScancode(
        scancode: SDL_Scancode,
        modstate: SDL_Keymod,
        key_event: ::core::primitive::bool,
    ) -> SDL_Keycode;
}

extern "C" {
    /// Get the scancode corresponding to the given key code according to the
    /// current keyboard layout.
    ///
    /// Note that there may be multiple scancode+modifier states that can generate
    /// this keycode, this will just return the first one found.
    ///
    /// ### Parameters
    /// - `key`: the desired [`SDL_Keycode`] to query.
    /// - `modstate`: a pointer to the modifier state that would be used when the
    ///   scancode generates this key, may be NULL.
    ///
    /// ### Return value
    /// Returns the [`SDL_Scancode`] that corresponds to the given [`SDL_Keycode`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyFromScancode`]
    /// - [`SDL_GetScancodeName`]
    pub fn SDL_GetScancodeFromKey(key: SDL_Keycode, modstate: *mut SDL_Keymod) -> SDL_Scancode;
}

extern "C" {
    /// Set a human-readable name for a scancode.
    ///
    /// ### Parameters
    /// - `scancode`: the desired [`SDL_Scancode`].
    /// - `name`: the name to use for the scancode, encoded as UTF-8. The string
    ///   is not copied, so the pointer given to this function must stay
    ///   valid while SDL is being used.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetScancodeName`]
    pub fn SDL_SetScancodeName(
        scancode: SDL_Scancode,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get a human-readable name for a scancode.
    ///
    /// **Warning**: The returned name is by design not stable across platforms,
    /// e.g. the name for [`SDL_SCANCODE_LGUI`] is "Left GUI" under Linux but "Left
    /// Windows" under Microsoft Windows, and some scancodes like
    /// [`SDL_SCANCODE_NONUSBACKSLASH`] don't have any name at all. There are even
    /// scancodes that share names, e.g. [`SDL_SCANCODE_RETURN`] and
    /// [`SDL_SCANCODE_RETURN2`] (both called "Return"). This function is therefore
    /// unsuitable for creating a stable cross-platform two-way mapping between
    /// strings and scancodes.
    ///
    /// ### Parameters
    /// - `scancode`: the desired [`SDL_Scancode`] to query.
    ///
    /// ### Return value
    /// Returns a pointer to the name for the scancode. If the scancode doesn't
    ///   have a name this function returns an empty string ("").
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetScancodeFromKey`]
    /// - [`SDL_GetScancodeFromName`]
    /// - [`SDL_SetScancodeName`]
    pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a scancode from a human-readable name.
    ///
    /// ### Parameters
    /// - `name`: the human-readable scancode name.
    ///
    /// ### Return value
    /// Returns the [`SDL_Scancode`], or [`SDL_SCANCODE_UNKNOWN`] if the name wasn't
    ///   recognized; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyFromName`]
    /// - [`SDL_GetScancodeFromKey`]
    /// - [`SDL_GetScancodeName`]
    pub fn SDL_GetScancodeFromName(name: *const ::core::ffi::c_char) -> SDL_Scancode;
}

extern "C" {
    /// Get a human-readable name for a key.
    ///
    /// If the key doesn't have a name, this function returns an empty string ("").
    ///
    /// ### Parameters
    /// - `key`: the desired [`SDL_Keycode`] to query.
    ///
    /// ### Return value
    /// Returns a UTF-8 encoded string of the key name.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyFromName`]
    /// - [`SDL_GetKeyFromScancode`]
    /// - [`SDL_GetScancodeFromKey`]
    pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a key code from a human-readable name.
    ///
    /// ### Parameters
    /// - `name`: the human-readable key name.
    ///
    /// ### Return value
    /// Returns key code, or `SDLK_UNKNOWN` if the name wasn't recognized; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetKeyFromScancode`]
    /// - [`SDL_GetKeyName`]
    /// - [`SDL_GetScancodeFromName`]
    pub fn SDL_GetKeyFromName(name: *const ::core::ffi::c_char) -> SDL_Keycode;
}

extern "C" {
    /// Start accepting Unicode text input events in a window.
    ///
    /// This function will enable text input ([`SDL_EVENT_TEXT_INPUT`] and
    /// [`SDL_EVENT_TEXT_EDITING`] events) in the specified window. Please use this
    /// function paired with [`SDL_StopTextInput()`].
    ///
    /// Text input events are not received by default.
    ///
    /// On some platforms using this function shows the screen keyboard and/or
    /// activates an IME, which can prevent some key press events from being passed
    /// through.
    ///
    /// ### Parameters
    /// - `window`: the window to enable text input.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetTextInputArea`]
    /// - [`SDL_StartTextInputWithProperties`]
    /// - [`SDL_StopTextInput`]
    /// - [`SDL_TextInputActive`]
    pub fn SDL_StartTextInput(window: *mut SDL_Window) -> ::core::primitive::bool;
}

/// Text input type.
///
/// These are the valid values for [`SDL_PROP_TEXTINPUT_TYPE_NUMBER`]. Not every
/// value is valid on every platform, but where a value isn't supported, a
/// reasonable fallback will be used.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_StartTextInputWithProperties`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`TEXT`](SDL_TextInputType::TEXT) | [`SDL_TEXTINPUT_TYPE_TEXT`] | The input is text |
/// | [`TEXT_NAME`](SDL_TextInputType::TEXT_NAME) | [`SDL_TEXTINPUT_TYPE_TEXT_NAME`] | The input is a person's name |
/// | [`TEXT_EMAIL`](SDL_TextInputType::TEXT_EMAIL) | [`SDL_TEXTINPUT_TYPE_TEXT_EMAIL`] | The input is an e-mail address |
/// | [`TEXT_USERNAME`](SDL_TextInputType::TEXT_USERNAME) | [`SDL_TEXTINPUT_TYPE_TEXT_USERNAME`] | The input is a username |
/// | [`TEXT_PASSWORD_HIDDEN`](SDL_TextInputType::TEXT_PASSWORD_HIDDEN) | [`SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN`] | The input is a secure password that is hidden |
/// | [`TEXT_PASSWORD_VISIBLE`](SDL_TextInputType::TEXT_PASSWORD_VISIBLE) | [`SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE`] | The input is a secure password that is visible |
/// | [`NUMBER`](SDL_TextInputType::NUMBER) | [`SDL_TEXTINPUT_TYPE_NUMBER`] | The input is a number |
/// | [`NUMBER_PASSWORD_HIDDEN`](SDL_TextInputType::NUMBER_PASSWORD_HIDDEN) | [`SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN`] | The input is a secure PIN that is hidden |
/// | [`NUMBER_PASSWORD_VISIBLE`](SDL_TextInputType::NUMBER_PASSWORD_VISIBLE) | [`SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE`] | The input is a secure PIN that is visible |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_TextInputType(pub ::core::ffi::c_int);

impl From<SDL_TextInputType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_TextInputType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_TextInputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::TEXT => "SDL_TEXTINPUT_TYPE_TEXT",
            Self::TEXT_NAME => "SDL_TEXTINPUT_TYPE_TEXT_NAME",
            Self::TEXT_EMAIL => "SDL_TEXTINPUT_TYPE_TEXT_EMAIL",
            Self::TEXT_USERNAME => "SDL_TEXTINPUT_TYPE_TEXT_USERNAME",
            Self::TEXT_PASSWORD_HIDDEN => "SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN",
            Self::TEXT_PASSWORD_VISIBLE => "SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE",
            Self::NUMBER => "SDL_TEXTINPUT_TYPE_NUMBER",
            Self::NUMBER_PASSWORD_HIDDEN => "SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN",
            Self::NUMBER_PASSWORD_VISIBLE => "SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE",

            _ => return write!(f, "SDL_TextInputType({})", self.0),
        })
    }
}

impl SDL_TextInputType {
    /// The input is text
    pub const TEXT: Self = Self(0);
    /// The input is a person's name
    pub const TEXT_NAME: Self = Self(1);
    /// The input is an e-mail address
    pub const TEXT_EMAIL: Self = Self(2);
    /// The input is a username
    pub const TEXT_USERNAME: Self = Self(3);
    /// The input is a secure password that is hidden
    pub const TEXT_PASSWORD_HIDDEN: Self = Self(4);
    /// The input is a secure password that is visible
    pub const TEXT_PASSWORD_VISIBLE: Self = Self(5);
    /// The input is a number
    pub const NUMBER: Self = Self(6);
    /// The input is a secure PIN that is hidden
    pub const NUMBER_PASSWORD_HIDDEN: Self = Self(7);
    /// The input is a secure PIN that is visible
    pub const NUMBER_PASSWORD_VISIBLE: Self = Self(8);
}

/// The input is text
pub const SDL_TEXTINPUT_TYPE_TEXT: SDL_TextInputType = SDL_TextInputType::TEXT;
/// The input is a person's name
pub const SDL_TEXTINPUT_TYPE_TEXT_NAME: SDL_TextInputType = SDL_TextInputType::TEXT_NAME;
/// The input is an e-mail address
pub const SDL_TEXTINPUT_TYPE_TEXT_EMAIL: SDL_TextInputType = SDL_TextInputType::TEXT_EMAIL;
/// The input is a username
pub const SDL_TEXTINPUT_TYPE_TEXT_USERNAME: SDL_TextInputType = SDL_TextInputType::TEXT_USERNAME;
/// The input is a secure password that is hidden
pub const SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN: SDL_TextInputType =
    SDL_TextInputType::TEXT_PASSWORD_HIDDEN;
/// The input is a secure password that is visible
pub const SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE: SDL_TextInputType =
    SDL_TextInputType::TEXT_PASSWORD_VISIBLE;
/// The input is a number
pub const SDL_TEXTINPUT_TYPE_NUMBER: SDL_TextInputType = SDL_TextInputType::NUMBER;
/// The input is a secure PIN that is hidden
pub const SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN: SDL_TextInputType =
    SDL_TextInputType::NUMBER_PASSWORD_HIDDEN;
/// The input is a secure PIN that is visible
pub const SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE: SDL_TextInputType =
    SDL_TextInputType::NUMBER_PASSWORD_VISIBLE;

/// Auto capitalization type.
///
/// These are the valid values for
/// [`SDL_PROP_TEXTINPUT_AUTOCAPITALIZATION_NUMBER`]. Not every value is valid on
/// every platform, but where a value isn't supported, a reasonable fallback
/// will be used.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_StartTextInputWithProperties`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_Capitalization::NONE) | [`SDL_CAPITALIZE_NONE`] | No auto-capitalization will be done |
/// | [`SENTENCES`](SDL_Capitalization::SENTENCES) | [`SDL_CAPITALIZE_SENTENCES`] | The first letter of sentences will be capitalized |
/// | [`WORDS`](SDL_Capitalization::WORDS) | [`SDL_CAPITALIZE_WORDS`] | The first letter of words will be capitalized |
/// | [`LETTERS`](SDL_Capitalization::LETTERS) | [`SDL_CAPITALIZE_LETTERS`] | All letters will be capitalized |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Capitalization(pub ::core::ffi::c_int);

impl From<SDL_Capitalization> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Capitalization) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_Capitalization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_CAPITALIZE_NONE",
            Self::SENTENCES => "SDL_CAPITALIZE_SENTENCES",
            Self::WORDS => "SDL_CAPITALIZE_WORDS",
            Self::LETTERS => "SDL_CAPITALIZE_LETTERS",

            _ => return write!(f, "SDL_Capitalization({})", self.0),
        })
    }
}

impl SDL_Capitalization {
    /// No auto-capitalization will be done
    pub const NONE: Self = Self(0);
    /// The first letter of sentences will be capitalized
    pub const SENTENCES: Self = Self(1);
    /// The first letter of words will be capitalized
    pub const WORDS: Self = Self(2);
    /// All letters will be capitalized
    pub const LETTERS: Self = Self(3);
}

/// No auto-capitalization will be done
pub const SDL_CAPITALIZE_NONE: SDL_Capitalization = SDL_Capitalization::NONE;
/// The first letter of sentences will be capitalized
pub const SDL_CAPITALIZE_SENTENCES: SDL_Capitalization = SDL_Capitalization::SENTENCES;
/// The first letter of words will be capitalized
pub const SDL_CAPITALIZE_WORDS: SDL_Capitalization = SDL_Capitalization::WORDS;
/// All letters will be capitalized
pub const SDL_CAPITALIZE_LETTERS: SDL_Capitalization = SDL_Capitalization::LETTERS;

extern "C" {
    /// Start accepting Unicode text input events in a window, with properties
    /// describing the input.
    ///
    /// This function will enable text input ([`SDL_EVENT_TEXT_INPUT`] and
    /// [`SDL_EVENT_TEXT_EDITING`] events) in the specified window. Please use this
    /// function paired with [`SDL_StopTextInput()`].
    ///
    /// Text input events are not received by default.
    ///
    /// On some platforms using this function shows the screen keyboard and/or
    /// activates an IME, which can prevent some key press events from being passed
    /// through.
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_TEXTINPUT_TYPE_NUMBER`] - an [`SDL_TextInputType`] value that
    ///   describes text being input, defaults to [`SDL_TEXTINPUT_TYPE_TEXT`].
    /// - [`SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER`] - an [`SDL_Capitalization`] value
    ///   that describes how text should be capitalized, defaults to
    ///   [`SDL_CAPITALIZE_SENTENCES`] for normal text entry, [`SDL_CAPITALIZE_WORDS`] for
    ///   [`SDL_TEXTINPUT_TYPE_TEXT_NAME`], and [`SDL_CAPITALIZE_NONE`] for e-mail
    ///   addresses, usernames, and passwords.
    /// - [`SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN`] - true to enable auto completion
    ///   and auto correction, defaults to true.
    /// - [`SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN`] - true if multiple lines of text
    ///   are allowed. This defaults to true if [`SDL_HINT_RETURN_KEY_HIDES_IME`] is
    ///   "0" or is not set, and defaults to false if [`SDL_HINT_RETURN_KEY_HIDES_IME`]
    ///   is "1".
    ///
    /// On Android you can directly specify the input type:
    ///
    /// - [`SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER`] - the text input type to
    ///   use, overriding other properties. This is documented at
    ///   <https://developer.android.com/reference/android/text/InputType>
    ///
    /// ### Parameters
    /// - `window`: the window to enable text input.
    /// - `props`: the properties to use.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetTextInputArea`]
    /// - [`SDL_StartTextInput`]
    /// - [`SDL_StopTextInput`]
    /// - [`SDL_TextInputActive`]
    pub fn SDL_StartTextInputWithProperties(
        window: *mut SDL_Window,
        props: SDL_PropertiesID,
    ) -> ::core::primitive::bool;
}

pub const SDL_PROP_TEXTINPUT_TYPE_NUMBER: *const ::core::ffi::c_char =
    c"SDL.textinput.type".as_ptr();

pub const SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER: *const ::core::ffi::c_char =
    c"SDL.textinput.capitalization".as_ptr();

pub const SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.textinput.autocorrect".as_ptr();

pub const SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.textinput.multiline".as_ptr();

pub const SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER: *const ::core::ffi::c_char =
    c"SDL.textinput.android.inputtype".as_ptr();

extern "C" {
    /// Check whether or not Unicode text input events are enabled for a window.
    ///
    /// ### Parameters
    /// - `window`: the window to check.
    ///
    /// ### Return value
    /// Returns true if text input events are enabled else false.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StartTextInput`]
    pub fn SDL_TextInputActive(window: *mut SDL_Window) -> ::core::primitive::bool;
}

extern "C" {
    /// Stop receiving any text input events in a window.
    ///
    /// If [`SDL_StartTextInput()`] showed the screen keyboard, this function will hide
    /// it.
    ///
    /// ### Parameters
    /// - `window`: the window to disable text input.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StartTextInput`]
    pub fn SDL_StopTextInput(window: *mut SDL_Window) -> ::core::primitive::bool;
}

extern "C" {
    /// Dismiss the composition window/IME without disabling the subsystem.
    ///
    /// ### Parameters
    /// - `window`: the window to affect.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StartTextInput`]
    /// - [`SDL_StopTextInput`]
    pub fn SDL_ClearComposition(window: *mut SDL_Window) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the area used to type Unicode text input.
    ///
    /// Native input methods may place a window with word suggestions near the
    /// cursor, without covering the text being entered.
    ///
    /// ### Parameters
    /// - `window`: the window for which to set the text input area.
    /// - `rect`: the [`SDL_Rect`] representing the text input area, in window
    ///   coordinates, or NULL to clear it.
    /// - `cursor`: the offset of the current cursor location relative to
    ///   `rect->x`, in window coordinates.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetTextInputArea`]
    /// - [`SDL_StartTextInput`]
    pub fn SDL_SetTextInputArea(
        window: *mut SDL_Window,
        rect: *const SDL_Rect,
        cursor: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the area used to type Unicode text input.
    ///
    /// This returns the values previously set by [`SDL_SetTextInputArea()`].
    ///
    /// ### Parameters
    /// - `window`: the window for which to query the text input area.
    /// - `rect`: a pointer to an [`SDL_Rect`] filled in with the text input area,
    ///   may be NULL.
    /// - `cursor`: a pointer to the offset of the current cursor location
    ///   relative to `rect->x`, may be NULL.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetTextInputArea`]
    pub fn SDL_GetTextInputArea(
        window: *mut SDL_Window,
        rect: *mut SDL_Rect,
        cursor: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Check whether the platform has screen keyboard support.
    ///
    /// ### Return value
    /// Returns true if the platform has some screen keyboard support or false if
    ///   not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StartTextInput`]
    /// - [`SDL_ScreenKeyboardShown`]
    pub fn SDL_HasScreenKeyboardSupport() -> ::core::primitive::bool;
}

extern "C" {
    /// Check whether the screen keyboard is shown for given window.
    ///
    /// ### Parameters
    /// - `window`: the window for which screen keyboard should be queried.
    ///
    /// ### Return value
    /// Returns true if screen keyboard is shown or false if not.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasScreenKeyboardSupport`]
    pub fn SDL_ScreenKeyboardShown(window: *mut SDL_Window) -> ::core::primitive::bool;
}

#[cfg(doc)]
use crate::everything::*;
