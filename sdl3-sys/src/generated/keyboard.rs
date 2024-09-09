#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]

//! # CategoryKeyboard
//!
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
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_KeyboardID = Uint32;

extern "C" {
    /// Return whether a keyboard is currently connected.
    ///
    /// \returns SDL_TRUE if a keyboard is connected, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyboards
    pub fn SDL_HasKeyboard() -> SDL_bool;
}

extern "C" {
    /// Get a list of currently connected keyboards.
    ///
    /// Note that this will include any device or virtual driver that includes
    /// keyboard functionality, including some mice, KVM switches, motherboard
    /// power buttons, etc. You should wait for input from a device before you
    /// consider it actively in use.
    ///
    /// \param count a pointer filled in with the number of keyboards returned, may
    ///              be NULL.
    /// \returns a 0 terminated array of keyboards instance IDs or NULL on failure;
    ///          call SDL_GetError() for more information. This should be freed
    ///          with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyboardNameForID
    /// \sa SDL_HasKeyboard
    pub fn SDL_GetKeyboards(count: *mut ::core::ffi::c_int) -> *mut SDL_KeyboardID;
}

extern "C" {
    /// Get the name of a keyboard.
    ///
    /// This function returns "" if the keyboard doesn't have a name.
    ///
    /// \param instance_id the keyboard instance ID.
    /// \returns the name of the selected keyboard or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyboards
    pub fn SDL_GetKeyboardNameForID(instance_id: SDL_KeyboardID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query the window which currently has keyboard focus.
    ///
    /// \returns the window with keyboard focus.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetKeyboardFocus() -> *mut SDL_Window;
}

extern "C" {
    /// Get a snapshot of the current state of the keyboard.
    ///
    /// The pointer returned is a pointer to an internal SDL array. It will be
    /// valid for the whole lifetime of the application and should not be freed by
    /// the caller.
    ///
    /// A array element with a value of 1 means that the key is pressed and a value
    /// of 0 means that it is not. Indexes into this array are obtained by using
    /// SDL_Scancode values.
    ///
    /// Use SDL_PumpEvents() to update the state array.
    ///
    /// This function gives you the current state after all events have been
    /// processed, so if a key or button has been pressed and released before you
    /// process events, then the pressed state will never show up in the
    /// SDL_GetKeyboardState() calls.
    ///
    /// Note: This function doesn't take into account whether shift has been
    /// pressed or not.
    ///
    /// \param numkeys if non-NULL, receives the length of the returned array.
    /// \returns a pointer to an array of key states.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_PumpEvents
    /// \sa SDL_ResetKeyboard
    pub fn SDL_GetKeyboardState(numkeys: *mut ::core::ffi::c_int) -> *const Uint8;
}

extern "C" {
    /// Clear the state of the keyboard.
    ///
    /// This function will generate key up events for all pressed keys.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyboardState
    pub fn SDL_ResetKeyboard();
}

extern "C" {
    /// Get the current key modifier state for the keyboard.
    ///
    /// \returns an OR'd combination of the modifier keys for the keyboard. See
    ///          SDL_Keymod for details.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyboardState
    /// \sa SDL_SetModState
    pub fn SDL_GetModState() -> SDL_Keymod;
}

extern "C" {
    /// Set the current key modifier state for the keyboard.
    ///
    /// The inverse of SDL_GetModState(), SDL_SetModState() allows you to impose
    /// modifier key states on your application. Simply pass your desired modifier
    /// states into `modstate`. This value may be a bitwise, OR'd combination of
    /// SDL_Keymod values.
    ///
    /// This does not change the keyboard state, only the key modifier flags that
    /// SDL reports.
    ///
    /// \param modstate the desired SDL_Keymod for the keyboard.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetModState
    pub fn SDL_SetModState(modstate: SDL_Keymod);
}

extern "C" {
    /// Get the key code corresponding to the given scancode according to the
    /// current keyboard layout.
    ///
    /// If you want to get the keycode as it would be delivered in key events,
    /// including options specified in SDL_HINT_KEYCODE_OPTIONS, then you should
    /// pass `key_event` as SDL_TRUE. Otherwise this function simply translates the
    /// scancode based on the given modifier state.
    ///
    /// \param scancode the desired SDL_Scancode to query.
    /// \param modstate the modifier state to use when translating the scancode to
    ///                 a keycode.
    /// \param key_event SDL_TRUE if the keycode will be used in key events.
    /// \returns the SDL_Keycode that corresponds to the given SDL_Scancode.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyName
    /// \sa SDL_GetScancodeFromKey
    pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode, modstate: SDL_Keymod, key_event: SDL_bool) -> SDL_Keycode;
}

extern "C" {
    /// Get the scancode corresponding to the given key code according to the
    /// current keyboard layout.
    ///
    /// Note that there may be multiple scancode+modifier states that can generate
    /// this keycode, this will just return the first one found.
    ///
    /// \param key the desired SDL_Keycode to query.
    /// \param modstate a pointer to the modifier state that would be used when the
    ///                 scancode generates this key, may be NULL.
    /// \returns the SDL_Scancode that corresponds to the given SDL_Keycode.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyFromScancode
    /// \sa SDL_GetScancodeName
    pub fn SDL_GetScancodeFromKey(key: SDL_Keycode, modstate: *mut SDL_Keymod) -> SDL_Scancode;
}

extern "C" {
    /// Set a human-readable name for a scancode.
    ///
    /// \param scancode the desired SDL_Scancode.
    /// \param name the name to use for the scancode, encoded as UTF-8. The string
    ///             is not copied, so the pointer given to this function must stay
    ///             valid while SDL is being used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetScancodeName
    pub fn SDL_SetScancodeName(scancode: SDL_Scancode, name: *const ::core::ffi::c_char) -> SDL_bool;
}

extern "C" {
    /// Get a human-readable name for a scancode.
    ///
    /// **Warning**: The returned name is by design not stable across platforms,
    /// e.g. the name for `SDL_SCANCODE_LGUI` is "Left GUI" under Linux but "Left
    /// Windows" under Microsoft Windows, and some scancodes like
    /// `SDL_SCANCODE_NONUSBACKSLASH` don't have any name at all. There are even
    /// scancodes that share names, e.g. `SDL_SCANCODE_RETURN` and
    /// `SDL_SCANCODE_RETURN2` (both called "Return"). This function is therefore
    /// unsuitable for creating a stable cross-platform two-way mapping between
    /// strings and scancodes.
    ///
    /// \param scancode the desired SDL_Scancode to query.
    /// \returns a pointer to the name for the scancode. If the scancode doesn't
    ///          have a name this function returns an empty string ("").
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetScancodeFromKey
    /// \sa SDL_GetScancodeFromName
    /// \sa SDL_SetScancodeName
    pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a scancode from a human-readable name.
    ///
    /// \param name the human-readable scancode name.
    /// \returns the SDL_Scancode, or `SDL_SCANCODE_UNKNOWN` if the name wasn't
    ///          recognized; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyFromName
    /// \sa SDL_GetScancodeFromKey
    /// \sa SDL_GetScancodeName
    pub fn SDL_GetScancodeFromName(name: *const ::core::ffi::c_char) -> SDL_Scancode;
}

extern "C" {
    /// Get a human-readable name for a key.
    ///
    /// If the key doesn't have a name, this function returns an empty string ("").
    ///
    /// \param key the desired SDL_Keycode to query.
    /// \returns a UTF-8 encoded string of the key name.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyFromName
    /// \sa SDL_GetKeyFromScancode
    /// \sa SDL_GetScancodeFromKey
    pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a key code from a human-readable name.
    ///
    /// \param name the human-readable key name.
    /// \returns key code, or `SDLK_UNKNOWN` if the name wasn't recognized; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetKeyFromScancode
    /// \sa SDL_GetKeyName
    /// \sa SDL_GetScancodeFromName
    pub fn SDL_GetKeyFromName(name: *const ::core::ffi::c_char) -> SDL_Keycode;
}

extern "C" {
    /// Start accepting Unicode text input events in a window.
    ///
    /// This function will enable text input (SDL_EVENT_TEXT_INPUT and
    /// SDL_EVENT_TEXT_EDITING events) in the specified window. Please use this
    /// function paired with SDL_StopTextInput().
    ///
    /// Text input events are not received by default.
    ///
    /// On some platforms using this function shows the screen keyboard.
    ///
    /// \param window the window to enable text input.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTextInputArea
    /// \sa SDL_StartTextInputWithProperties
    /// \sa SDL_StopTextInput
    /// \sa SDL_TextInputActive
    pub fn SDL_StartTextInput(window: *mut SDL_Window) -> SDL_bool;
}

/// Text input type.
///
/// These are the valid values for SDL_PROP_TEXTINPUT_TYPE_NUMBER. Not every
/// value is valid on every platform, but where a value isn't supported, a
/// reasonable fallback will be used.
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_StartTextInputWithProperties
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_TEXTINPUT_TYPE_TEXT`], [`SDL_TEXTINPUT_TYPE_TEXT_NAME`], [`SDL_TEXTINPUT_TYPE_TEXT_EMAIL`], [`SDL_TEXTINPUT_TYPE_TEXT_USERNAME`], [`SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN`], [`SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE`], [`SDL_TEXTINPUT_TYPE_NUMBER`], [`SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN`], [`SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TextInputType(pub ::core::ffi::c_int);
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
pub const SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_HIDDEN: SDL_TextInputType = SDL_TextInputType::TEXT_PASSWORD_HIDDEN;
/// The input is a secure password that is visible
pub const SDL_TEXTINPUT_TYPE_TEXT_PASSWORD_VISIBLE: SDL_TextInputType = SDL_TextInputType::TEXT_PASSWORD_VISIBLE;
/// The input is a number
pub const SDL_TEXTINPUT_TYPE_NUMBER: SDL_TextInputType = SDL_TextInputType::NUMBER;
/// The input is a secure PIN that is hidden
pub const SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_HIDDEN: SDL_TextInputType = SDL_TextInputType::NUMBER_PASSWORD_HIDDEN;
/// The input is a secure PIN that is visible
pub const SDL_TEXTINPUT_TYPE_NUMBER_PASSWORD_VISIBLE: SDL_TextInputType = SDL_TextInputType::NUMBER_PASSWORD_VISIBLE;

/// Auto capitalization type.
///
/// These are the valid values for
/// SDL_PROP_TEXTINPUT_AUTOCAPITALIZATION_NUMBER. Not every value is valid on
/// every platform, but where a value isn't supported, a reasonable fallback
/// will be used.
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_StartTextInputWithProperties
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_CAPITALIZE_NONE`], [`SDL_CAPITALIZE_SENTENCES`], [`SDL_CAPITALIZE_WORDS`], [`SDL_CAPITALIZE_LETTERS`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Capitalization(pub ::core::ffi::c_int);
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
    /// This function will enable text input (SDL_EVENT_TEXT_INPUT and
    /// SDL_EVENT_TEXT_EDITING events) in the specified window. Please use this
    /// function paired with SDL_StopTextInput().
    ///
    /// Text input events are not received by default.
    ///
    /// On some platforms using this function shows the screen keyboard.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_TEXTINPUT_TYPE_NUMBER` - an SDL_TextInputType value that
    ///   describes text being input, defaults to SDL_TEXTINPUT_TYPE_TEXT.
    /// - `SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER` - an SDL_Capitalization value
    ///   that describes how text should be capitalized, defaults to
    ///   SDL_CAPITALIZE_SENTENCES for normal text entry, SDL_CAPITALIZE_WORDS for
    ///   SDL_TEXTINPUT_TYPE_TEXT_NAME, and SDL_CAPITALIZE_NONE for e-mail
    ///   addresses, usernames, and passwords.
    /// - `SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN` - true to enable auto completion
    ///   and auto correction, defaults to SDL_TRUE.
    /// - `SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN` - true if multiple lines of text
    ///   are allowed. This defaults to SDL_TRUE if SDL_HINT_RETURN_KEY_HIDES_IME
    ///   is "0" or is not set, and defaults to SDL_FALSE if
    ///   SDL_HINT_RETURN_KEY_HIDES_IME is "1".
    ///
    /// On Android you can directly specify the input type:
    ///
    /// - `SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER` - the text input type to
    ///   use, overriding other properties. This is documented at
    ///   https://developer.android.com/reference/android/text/InputType
    ///
    /// \param window the window to enable text input.
    /// \param props the properties to use.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTextInputArea
    /// \sa SDL_StartTextInput
    /// \sa SDL_StopTextInput
    /// \sa SDL_TextInputActive
    pub fn SDL_StartTextInputWithProperties(window: *mut SDL_Window, props: SDL_PropertiesID) -> SDL_bool;
}

pub const SDL_PROP_TEXTINPUT_TYPE_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.textinput.type\0") };

pub const SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.textinput.capitalization\0") };

pub const SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.textinput.autocorrect\0") };

pub const SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.textinput.multiline\0") };

pub const SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.textinput.android.inputtype\0") };

extern "C" {
    /// Check whether or not Unicode text input events are enabled for a window.
    ///
    /// \param window the window to check.
    /// \returns SDL_TRUE if text input events are enabled else SDL_FALSE.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StartTextInput
    pub fn SDL_TextInputActive(window: *mut SDL_Window) -> SDL_bool;
}

extern "C" {
    /// Stop receiving any text input events in a window.
    ///
    /// If SDL_StartTextInput() showed the screen keyboard, this function will hide
    /// it.
    ///
    /// \param window the window to disable text input.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StartTextInput
    pub fn SDL_StopTextInput(window: *mut SDL_Window) -> SDL_bool;
}

extern "C" {
    /// Dismiss the composition window/IME without disabling the subsystem.
    ///
    /// \param window the window to affect.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StartTextInput
    /// \sa SDL_StopTextInput
    pub fn SDL_ClearComposition(window: *mut SDL_Window) -> SDL_bool;
}

extern "C" {
    /// Set the area used to type Unicode text input.
    ///
    /// Native input methods may place a window with word suggestions near the
    /// cursor, without covering the text being entered.
    ///
    /// \param window the window for which to set the text input area.
    /// \param rect the SDL_Rect representing the text input area, in window
    ///             coordinates, or NULL to clear it.
    /// \param cursor the offset of the current cursor location relative to
    ///               `rect->x`, in window coordinates.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextInputArea
    /// \sa SDL_StartTextInput
    pub fn SDL_SetTextInputArea(window: *mut SDL_Window, rect: *const SDL_Rect, cursor: ::core::ffi::c_int) -> SDL_bool;
}

extern "C" {
    /// Get the area used to type Unicode text input.
    ///
    /// This returns the values previously set by SDL_SetTextInputArea().
    ///
    /// \param window the window for which to query the text input area.
    /// \param rect a pointer to an SDL_Rect filled in with the text input area,
    ///             may be NULL.
    /// \param cursor a pointer to the offset of the current cursor location
    ///               relative to `rect->x`, may be NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTextInputArea
    pub fn SDL_GetTextInputArea(window: *mut SDL_Window, rect: *mut SDL_Rect, cursor: *mut ::core::ffi::c_int) -> SDL_bool;
}

extern "C" {
    /// Check whether the platform has screen keyboard support.
    ///
    /// \returns SDL_TRUE if the platform has some screen keyboard support or
    ///          SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StartTextInput
    /// \sa SDL_ScreenKeyboardShown
    pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
}

extern "C" {
    /// Check whether the screen keyboard is shown for given window.
    ///
    /// \param window the window for which screen keyboard should be queried.
    /// \returns SDL_TRUE if screen keyboard is shown or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasScreenKeyboardSupport
    pub fn SDL_ScreenKeyboardShown(window: *mut SDL_Window) -> SDL_bool;
}

