//! Defines keyboard scancodes.
//!
//! Please refer to the Best Keyboard Practices document for details on what
//! this information means and how best to use it.
//!
//! <https://wiki.libsdl.org/SDL3/BestKeyboardPractices>

use super::stdinc::*;

/// The SDL keyboard scancode representation.
///
/// An SDL scancode is the physical representation of a key on the keyboard,
/// independent of language and keyboard mapping.
///
/// Values of this type are used to represent keyboard keys, among other places
/// in the `scancode` field of the [`SDL_KeyboardEvent`] structure.
///
/// The values in this enumeration are based on the USB usage page standard:
/// <https://usb.org/sites/default/files/hut1_5.pdf>
///
/// ### Availability
/// This enum is available since SDL 3.2.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_Scancode::UNKNOWN) | [`SDL_SCANCODE_UNKNOWN`] | |
/// | [`A`](SDL_Scancode::A) | [`SDL_SCANCODE_A`] | |
/// | [`B`](SDL_Scancode::B) | [`SDL_SCANCODE_B`] | |
/// | [`C`](SDL_Scancode::C) | [`SDL_SCANCODE_C`] | |
/// | [`D`](SDL_Scancode::D) | [`SDL_SCANCODE_D`] | |
/// | [`E`](SDL_Scancode::E) | [`SDL_SCANCODE_E`] | |
/// | [`F`](SDL_Scancode::F) | [`SDL_SCANCODE_F`] | |
/// | [`G`](SDL_Scancode::G) | [`SDL_SCANCODE_G`] | |
/// | [`H`](SDL_Scancode::H) | [`SDL_SCANCODE_H`] | |
/// | [`I`](SDL_Scancode::I) | [`SDL_SCANCODE_I`] | |
/// | [`J`](SDL_Scancode::J) | [`SDL_SCANCODE_J`] | |
/// | [`K`](SDL_Scancode::K) | [`SDL_SCANCODE_K`] | |
/// | [`L`](SDL_Scancode::L) | [`SDL_SCANCODE_L`] | |
/// | [`M`](SDL_Scancode::M) | [`SDL_SCANCODE_M`] | |
/// | [`N`](SDL_Scancode::N) | [`SDL_SCANCODE_N`] | |
/// | [`O`](SDL_Scancode::O) | [`SDL_SCANCODE_O`] | |
/// | [`P`](SDL_Scancode::P) | [`SDL_SCANCODE_P`] | |
/// | [`Q`](SDL_Scancode::Q) | [`SDL_SCANCODE_Q`] | |
/// | [`R`](SDL_Scancode::R) | [`SDL_SCANCODE_R`] | |
/// | [`S`](SDL_Scancode::S) | [`SDL_SCANCODE_S`] | |
/// | [`T`](SDL_Scancode::T) | [`SDL_SCANCODE_T`] | |
/// | [`U`](SDL_Scancode::U) | [`SDL_SCANCODE_U`] | |
/// | [`V`](SDL_Scancode::V) | [`SDL_SCANCODE_V`] | |
/// | [`W`](SDL_Scancode::W) | [`SDL_SCANCODE_W`] | |
/// | [`X`](SDL_Scancode::X) | [`SDL_SCANCODE_X`] | |
/// | [`Y`](SDL_Scancode::Y) | [`SDL_SCANCODE_Y`] | |
/// | [`Z`](SDL_Scancode::Z) | [`SDL_SCANCODE_Z`] | |
/// | [`_1`](SDL_Scancode::_1) | [`SDL_SCANCODE_1`] | |
/// | [`_2`](SDL_Scancode::_2) | [`SDL_SCANCODE_2`] | |
/// | [`_3`](SDL_Scancode::_3) | [`SDL_SCANCODE_3`] | |
/// | [`_4`](SDL_Scancode::_4) | [`SDL_SCANCODE_4`] | |
/// | [`_5`](SDL_Scancode::_5) | [`SDL_SCANCODE_5`] | |
/// | [`_6`](SDL_Scancode::_6) | [`SDL_SCANCODE_6`] | |
/// | [`_7`](SDL_Scancode::_7) | [`SDL_SCANCODE_7`] | |
/// | [`_8`](SDL_Scancode::_8) | [`SDL_SCANCODE_8`] | |
/// | [`_9`](SDL_Scancode::_9) | [`SDL_SCANCODE_9`] | |
/// | [`_0`](SDL_Scancode::_0) | [`SDL_SCANCODE_0`] | |
/// | [`RETURN`](SDL_Scancode::RETURN) | [`SDL_SCANCODE_RETURN`] | |
/// | [`ESCAPE`](SDL_Scancode::ESCAPE) | [`SDL_SCANCODE_ESCAPE`] | |
/// | [`BACKSPACE`](SDL_Scancode::BACKSPACE) | [`SDL_SCANCODE_BACKSPACE`] | |
/// | [`TAB`](SDL_Scancode::TAB) | [`SDL_SCANCODE_TAB`] | |
/// | [`SPACE`](SDL_Scancode::SPACE) | [`SDL_SCANCODE_SPACE`] | |
/// | [`MINUS`](SDL_Scancode::MINUS) | [`SDL_SCANCODE_MINUS`] | |
/// | [`EQUALS`](SDL_Scancode::EQUALS) | [`SDL_SCANCODE_EQUALS`] | |
/// | [`LEFTBRACKET`](SDL_Scancode::LEFTBRACKET) | [`SDL_SCANCODE_LEFTBRACKET`] | |
/// | [`RIGHTBRACKET`](SDL_Scancode::RIGHTBRACKET) | [`SDL_SCANCODE_RIGHTBRACKET`] | |
/// | [`BACKSLASH`](SDL_Scancode::BACKSLASH) | [`SDL_SCANCODE_BACKSLASH`] | Located at the lower left of the return key on ISO keyboards and at the right end of the QWERTY row on ANSI keyboards. Produces REVERSE SOLIDUS (backslash) and VERTICAL LINE in a US layout, REVERSE SOLIDUS and VERTICAL LINE in a UK Mac layout, NUMBER SIGN and TILDE in a UK Windows layout, DOLLAR SIGN and POUND SIGN in a Swiss German layout, NUMBER SIGN and APOSTROPHE in a German layout, GRAVE ACCENT and POUND SIGN in a French Mac layout, and ASTERISK and MICRO SIGN in a French Windows layout. |
/// | [`NONUSHASH`](SDL_Scancode::NONUSHASH) | [`SDL_SCANCODE_NONUSHASH`] | ISO USB keyboards actually use this code instead of 49 for the same key, but all OSes I've seen treat the two codes identically. So, as an implementor, unless your keyboard generates both of those codes and your OS treats them differently, you should generate [`SDL_SCANCODE_BACKSLASH`] instead of this code. As a user, you should not rely on this code because SDL will never generate it with most (all?) keyboards. |
/// | [`SEMICOLON`](SDL_Scancode::SEMICOLON) | [`SDL_SCANCODE_SEMICOLON`] | |
/// | [`APOSTROPHE`](SDL_Scancode::APOSTROPHE) | [`SDL_SCANCODE_APOSTROPHE`] | |
/// | [`GRAVE`](SDL_Scancode::GRAVE) | [`SDL_SCANCODE_GRAVE`] | Located in the top left corner (on both ANSI and ISO keyboards). Produces GRAVE ACCENT and TILDE in a US Windows layout and in US and UK Mac layouts on ANSI keyboards, GRAVE ACCENT and NOT SIGN in a UK Windows layout, SECTION SIGN and PLUS-MINUS SIGN in US and UK Mac layouts on ISO keyboards, SECTION SIGN and DEGREE SIGN in a Swiss German layout (Mac: only on ISO keyboards), CIRCUMFLEX ACCENT and DEGREE SIGN in a German layout (Mac: only on ISO keyboards), SUPERSCRIPT TWO and TILDE in a French Windows layout, COMMERCIAL AT and NUMBER SIGN in a French Mac layout on ISO keyboards, and LESS-THAN SIGN and GREATER-THAN SIGN in a Swiss German, German, or French Mac layout on ANSI keyboards. |
/// | [`COMMA`](SDL_Scancode::COMMA) | [`SDL_SCANCODE_COMMA`] | |
/// | [`PERIOD`](SDL_Scancode::PERIOD) | [`SDL_SCANCODE_PERIOD`] | |
/// | [`SLASH`](SDL_Scancode::SLASH) | [`SDL_SCANCODE_SLASH`] | |
/// | [`CAPSLOCK`](SDL_Scancode::CAPSLOCK) | [`SDL_SCANCODE_CAPSLOCK`] | |
/// | [`F1`](SDL_Scancode::F1) | [`SDL_SCANCODE_F1`] | |
/// | [`F2`](SDL_Scancode::F2) | [`SDL_SCANCODE_F2`] | |
/// | [`F3`](SDL_Scancode::F3) | [`SDL_SCANCODE_F3`] | |
/// | [`F4`](SDL_Scancode::F4) | [`SDL_SCANCODE_F4`] | |
/// | [`F5`](SDL_Scancode::F5) | [`SDL_SCANCODE_F5`] | |
/// | [`F6`](SDL_Scancode::F6) | [`SDL_SCANCODE_F6`] | |
/// | [`F7`](SDL_Scancode::F7) | [`SDL_SCANCODE_F7`] | |
/// | [`F8`](SDL_Scancode::F8) | [`SDL_SCANCODE_F8`] | |
/// | [`F9`](SDL_Scancode::F9) | [`SDL_SCANCODE_F9`] | |
/// | [`F10`](SDL_Scancode::F10) | [`SDL_SCANCODE_F10`] | |
/// | [`F11`](SDL_Scancode::F11) | [`SDL_SCANCODE_F11`] | |
/// | [`F12`](SDL_Scancode::F12) | [`SDL_SCANCODE_F12`] | |
/// | [`PRINTSCREEN`](SDL_Scancode::PRINTSCREEN) | [`SDL_SCANCODE_PRINTSCREEN`] | |
/// | [`SCROLLLOCK`](SDL_Scancode::SCROLLLOCK) | [`SDL_SCANCODE_SCROLLLOCK`] | |
/// | [`PAUSE`](SDL_Scancode::PAUSE) | [`SDL_SCANCODE_PAUSE`] | |
/// | [`INSERT`](SDL_Scancode::INSERT) | [`SDL_SCANCODE_INSERT`] | insert on PC, help on some Mac keyboards (but does send code 73, not 117) |
/// | [`HOME`](SDL_Scancode::HOME) | [`SDL_SCANCODE_HOME`] | |
/// | [`PAGEUP`](SDL_Scancode::PAGEUP) | [`SDL_SCANCODE_PAGEUP`] | |
/// | [`DELETE`](SDL_Scancode::DELETE) | [`SDL_SCANCODE_DELETE`] | |
/// | [`END`](SDL_Scancode::END) | [`SDL_SCANCODE_END`] | |
/// | [`PAGEDOWN`](SDL_Scancode::PAGEDOWN) | [`SDL_SCANCODE_PAGEDOWN`] | |
/// | [`RIGHT`](SDL_Scancode::RIGHT) | [`SDL_SCANCODE_RIGHT`] | |
/// | [`LEFT`](SDL_Scancode::LEFT) | [`SDL_SCANCODE_LEFT`] | |
/// | [`DOWN`](SDL_Scancode::DOWN) | [`SDL_SCANCODE_DOWN`] | |
/// | [`UP`](SDL_Scancode::UP) | [`SDL_SCANCODE_UP`] | |
/// | [`NUMLOCKCLEAR`](SDL_Scancode::NUMLOCKCLEAR) | [`SDL_SCANCODE_NUMLOCKCLEAR`] | num lock on PC, clear on Mac keyboards |
/// | [`KP_DIVIDE`](SDL_Scancode::KP_DIVIDE) | [`SDL_SCANCODE_KP_DIVIDE`] | |
/// | [`KP_MULTIPLY`](SDL_Scancode::KP_MULTIPLY) | [`SDL_SCANCODE_KP_MULTIPLY`] | |
/// | [`KP_MINUS`](SDL_Scancode::KP_MINUS) | [`SDL_SCANCODE_KP_MINUS`] | |
/// | [`KP_PLUS`](SDL_Scancode::KP_PLUS) | [`SDL_SCANCODE_KP_PLUS`] | |
/// | [`KP_ENTER`](SDL_Scancode::KP_ENTER) | [`SDL_SCANCODE_KP_ENTER`] | |
/// | [`KP_1`](SDL_Scancode::KP_1) | [`SDL_SCANCODE_KP_1`] | |
/// | [`KP_2`](SDL_Scancode::KP_2) | [`SDL_SCANCODE_KP_2`] | |
/// | [`KP_3`](SDL_Scancode::KP_3) | [`SDL_SCANCODE_KP_3`] | |
/// | [`KP_4`](SDL_Scancode::KP_4) | [`SDL_SCANCODE_KP_4`] | |
/// | [`KP_5`](SDL_Scancode::KP_5) | [`SDL_SCANCODE_KP_5`] | |
/// | [`KP_6`](SDL_Scancode::KP_6) | [`SDL_SCANCODE_KP_6`] | |
/// | [`KP_7`](SDL_Scancode::KP_7) | [`SDL_SCANCODE_KP_7`] | |
/// | [`KP_8`](SDL_Scancode::KP_8) | [`SDL_SCANCODE_KP_8`] | |
/// | [`KP_9`](SDL_Scancode::KP_9) | [`SDL_SCANCODE_KP_9`] | |
/// | [`KP_0`](SDL_Scancode::KP_0) | [`SDL_SCANCODE_KP_0`] | |
/// | [`KP_PERIOD`](SDL_Scancode::KP_PERIOD) | [`SDL_SCANCODE_KP_PERIOD`] | |
/// | [`NONUSBACKSLASH`](SDL_Scancode::NONUSBACKSLASH) | [`SDL_SCANCODE_NONUSBACKSLASH`] | This is the additional key that ISO keyboards have over ANSI ones, located between left shift and Y. Produces GRAVE ACCENT and TILDE in a US or UK Mac layout, REVERSE SOLIDUS (backslash) and VERTICAL LINE in a US or UK Windows layout, and LESS-THAN SIGN and GREATER-THAN SIGN in a Swiss German, German, or French layout. |
/// | [`APPLICATION`](SDL_Scancode::APPLICATION) | [`SDL_SCANCODE_APPLICATION`] | windows contextual menu, compose |
/// | [`POWER`](SDL_Scancode::POWER) | [`SDL_SCANCODE_POWER`] | The USB document says this is a status flag, not a physical key - but some Mac keyboards do have a power key. |
/// | [`KP_EQUALS`](SDL_Scancode::KP_EQUALS) | [`SDL_SCANCODE_KP_EQUALS`] | |
/// | [`F13`](SDL_Scancode::F13) | [`SDL_SCANCODE_F13`] | |
/// | [`F14`](SDL_Scancode::F14) | [`SDL_SCANCODE_F14`] | |
/// | [`F15`](SDL_Scancode::F15) | [`SDL_SCANCODE_F15`] | |
/// | [`F16`](SDL_Scancode::F16) | [`SDL_SCANCODE_F16`] | |
/// | [`F17`](SDL_Scancode::F17) | [`SDL_SCANCODE_F17`] | |
/// | [`F18`](SDL_Scancode::F18) | [`SDL_SCANCODE_F18`] | |
/// | [`F19`](SDL_Scancode::F19) | [`SDL_SCANCODE_F19`] | |
/// | [`F20`](SDL_Scancode::F20) | [`SDL_SCANCODE_F20`] | |
/// | [`F21`](SDL_Scancode::F21) | [`SDL_SCANCODE_F21`] | |
/// | [`F22`](SDL_Scancode::F22) | [`SDL_SCANCODE_F22`] | |
/// | [`F23`](SDL_Scancode::F23) | [`SDL_SCANCODE_F23`] | |
/// | [`F24`](SDL_Scancode::F24) | [`SDL_SCANCODE_F24`] | |
/// | [`EXECUTE`](SDL_Scancode::EXECUTE) | [`SDL_SCANCODE_EXECUTE`] | |
/// | [`HELP`](SDL_Scancode::HELP) | [`SDL_SCANCODE_HELP`] | AL Integrated Help Center |
/// | [`MENU`](SDL_Scancode::MENU) | [`SDL_SCANCODE_MENU`] | Menu (show menu) |
/// | [`SELECT`](SDL_Scancode::SELECT) | [`SDL_SCANCODE_SELECT`] | |
/// | [`STOP`](SDL_Scancode::STOP) | [`SDL_SCANCODE_STOP`] | AC Stop |
/// | [`AGAIN`](SDL_Scancode::AGAIN) | [`SDL_SCANCODE_AGAIN`] | AC Redo/Repeat |
/// | [`UNDO`](SDL_Scancode::UNDO) | [`SDL_SCANCODE_UNDO`] | AC Undo |
/// | [`CUT`](SDL_Scancode::CUT) | [`SDL_SCANCODE_CUT`] | AC Cut |
/// | [`COPY`](SDL_Scancode::COPY) | [`SDL_SCANCODE_COPY`] | AC Copy |
/// | [`PASTE`](SDL_Scancode::PASTE) | [`SDL_SCANCODE_PASTE`] | AC Paste |
/// | [`FIND`](SDL_Scancode::FIND) | [`SDL_SCANCODE_FIND`] | AC Find |
/// | [`MUTE`](SDL_Scancode::MUTE) | [`SDL_SCANCODE_MUTE`] | |
/// | [`VOLUMEUP`](SDL_Scancode::VOLUMEUP) | [`SDL_SCANCODE_VOLUMEUP`] | |
/// | [`VOLUMEDOWN`](SDL_Scancode::VOLUMEDOWN) | [`SDL_SCANCODE_VOLUMEDOWN`] | |
/// | [`KP_COMMA`](SDL_Scancode::KP_COMMA) | [`SDL_SCANCODE_KP_COMMA`] | |
/// | [`KP_EQUALSAS400`](SDL_Scancode::KP_EQUALSAS400) | [`SDL_SCANCODE_KP_EQUALSAS400`] | |
/// | [`INTERNATIONAL1`](SDL_Scancode::INTERNATIONAL1) | [`SDL_SCANCODE_INTERNATIONAL1`] | used on Asian keyboards, see footnotes in USB doc |
/// | [`INTERNATIONAL2`](SDL_Scancode::INTERNATIONAL2) | [`SDL_SCANCODE_INTERNATIONAL2`] | |
/// | [`INTERNATIONAL3`](SDL_Scancode::INTERNATIONAL3) | [`SDL_SCANCODE_INTERNATIONAL3`] | Yen |
/// | [`INTERNATIONAL4`](SDL_Scancode::INTERNATIONAL4) | [`SDL_SCANCODE_INTERNATIONAL4`] | |
/// | [`INTERNATIONAL5`](SDL_Scancode::INTERNATIONAL5) | [`SDL_SCANCODE_INTERNATIONAL5`] | |
/// | [`INTERNATIONAL6`](SDL_Scancode::INTERNATIONAL6) | [`SDL_SCANCODE_INTERNATIONAL6`] | |
/// | [`INTERNATIONAL7`](SDL_Scancode::INTERNATIONAL7) | [`SDL_SCANCODE_INTERNATIONAL7`] | |
/// | [`INTERNATIONAL8`](SDL_Scancode::INTERNATIONAL8) | [`SDL_SCANCODE_INTERNATIONAL8`] | |
/// | [`INTERNATIONAL9`](SDL_Scancode::INTERNATIONAL9) | [`SDL_SCANCODE_INTERNATIONAL9`] | |
/// | [`LANG1`](SDL_Scancode::LANG1) | [`SDL_SCANCODE_LANG1`] | Hangul/English toggle |
/// | [`LANG2`](SDL_Scancode::LANG2) | [`SDL_SCANCODE_LANG2`] | Hanja conversion |
/// | [`LANG3`](SDL_Scancode::LANG3) | [`SDL_SCANCODE_LANG3`] | Katakana |
/// | [`LANG4`](SDL_Scancode::LANG4) | [`SDL_SCANCODE_LANG4`] | Hiragana |
/// | [`LANG5`](SDL_Scancode::LANG5) | [`SDL_SCANCODE_LANG5`] | Zenkaku/Hankaku |
/// | [`LANG6`](SDL_Scancode::LANG6) | [`SDL_SCANCODE_LANG6`] | reserved |
/// | [`LANG7`](SDL_Scancode::LANG7) | [`SDL_SCANCODE_LANG7`] | reserved |
/// | [`LANG8`](SDL_Scancode::LANG8) | [`SDL_SCANCODE_LANG8`] | reserved |
/// | [`LANG9`](SDL_Scancode::LANG9) | [`SDL_SCANCODE_LANG9`] | reserved |
/// | [`ALTERASE`](SDL_Scancode::ALTERASE) | [`SDL_SCANCODE_ALTERASE`] | Erase-Eaze |
/// | [`SYSREQ`](SDL_Scancode::SYSREQ) | [`SDL_SCANCODE_SYSREQ`] | |
/// | [`CANCEL`](SDL_Scancode::CANCEL) | [`SDL_SCANCODE_CANCEL`] | AC Cancel |
/// | [`CLEAR`](SDL_Scancode::CLEAR) | [`SDL_SCANCODE_CLEAR`] | |
/// | [`PRIOR`](SDL_Scancode::PRIOR) | [`SDL_SCANCODE_PRIOR`] | |
/// | [`RETURN2`](SDL_Scancode::RETURN2) | [`SDL_SCANCODE_RETURN2`] | |
/// | [`SEPARATOR`](SDL_Scancode::SEPARATOR) | [`SDL_SCANCODE_SEPARATOR`] | |
/// | [`OUT`](SDL_Scancode::OUT) | [`SDL_SCANCODE_OUT`] | |
/// | [`OPER`](SDL_Scancode::OPER) | [`SDL_SCANCODE_OPER`] | |
/// | [`CLEARAGAIN`](SDL_Scancode::CLEARAGAIN) | [`SDL_SCANCODE_CLEARAGAIN`] | |
/// | [`CRSEL`](SDL_Scancode::CRSEL) | [`SDL_SCANCODE_CRSEL`] | |
/// | [`EXSEL`](SDL_Scancode::EXSEL) | [`SDL_SCANCODE_EXSEL`] | |
/// | [`KP_00`](SDL_Scancode::KP_00) | [`SDL_SCANCODE_KP_00`] | |
/// | [`KP_000`](SDL_Scancode::KP_000) | [`SDL_SCANCODE_KP_000`] | |
/// | [`THOUSANDSSEPARATOR`](SDL_Scancode::THOUSANDSSEPARATOR) | [`SDL_SCANCODE_THOUSANDSSEPARATOR`] | |
/// | [`DECIMALSEPARATOR`](SDL_Scancode::DECIMALSEPARATOR) | [`SDL_SCANCODE_DECIMALSEPARATOR`] | |
/// | [`CURRENCYUNIT`](SDL_Scancode::CURRENCYUNIT) | [`SDL_SCANCODE_CURRENCYUNIT`] | |
/// | [`CURRENCYSUBUNIT`](SDL_Scancode::CURRENCYSUBUNIT) | [`SDL_SCANCODE_CURRENCYSUBUNIT`] | |
/// | [`KP_LEFTPAREN`](SDL_Scancode::KP_LEFTPAREN) | [`SDL_SCANCODE_KP_LEFTPAREN`] | |
/// | [`KP_RIGHTPAREN`](SDL_Scancode::KP_RIGHTPAREN) | [`SDL_SCANCODE_KP_RIGHTPAREN`] | |
/// | [`KP_LEFTBRACE`](SDL_Scancode::KP_LEFTBRACE) | [`SDL_SCANCODE_KP_LEFTBRACE`] | |
/// | [`KP_RIGHTBRACE`](SDL_Scancode::KP_RIGHTBRACE) | [`SDL_SCANCODE_KP_RIGHTBRACE`] | |
/// | [`KP_TAB`](SDL_Scancode::KP_TAB) | [`SDL_SCANCODE_KP_TAB`] | |
/// | [`KP_BACKSPACE`](SDL_Scancode::KP_BACKSPACE) | [`SDL_SCANCODE_KP_BACKSPACE`] | |
/// | [`KP_A`](SDL_Scancode::KP_A) | [`SDL_SCANCODE_KP_A`] | |
/// | [`KP_B`](SDL_Scancode::KP_B) | [`SDL_SCANCODE_KP_B`] | |
/// | [`KP_C`](SDL_Scancode::KP_C) | [`SDL_SCANCODE_KP_C`] | |
/// | [`KP_D`](SDL_Scancode::KP_D) | [`SDL_SCANCODE_KP_D`] | |
/// | [`KP_E`](SDL_Scancode::KP_E) | [`SDL_SCANCODE_KP_E`] | |
/// | [`KP_F`](SDL_Scancode::KP_F) | [`SDL_SCANCODE_KP_F`] | |
/// | [`KP_XOR`](SDL_Scancode::KP_XOR) | [`SDL_SCANCODE_KP_XOR`] | |
/// | [`KP_POWER`](SDL_Scancode::KP_POWER) | [`SDL_SCANCODE_KP_POWER`] | |
/// | [`KP_PERCENT`](SDL_Scancode::KP_PERCENT) | [`SDL_SCANCODE_KP_PERCENT`] | |
/// | [`KP_LESS`](SDL_Scancode::KP_LESS) | [`SDL_SCANCODE_KP_LESS`] | |
/// | [`KP_GREATER`](SDL_Scancode::KP_GREATER) | [`SDL_SCANCODE_KP_GREATER`] | |
/// | [`KP_AMPERSAND`](SDL_Scancode::KP_AMPERSAND) | [`SDL_SCANCODE_KP_AMPERSAND`] | |
/// | [`KP_DBLAMPERSAND`](SDL_Scancode::KP_DBLAMPERSAND) | [`SDL_SCANCODE_KP_DBLAMPERSAND`] | |
/// | [`KP_VERTICALBAR`](SDL_Scancode::KP_VERTICALBAR) | [`SDL_SCANCODE_KP_VERTICALBAR`] | |
/// | [`KP_DBLVERTICALBAR`](SDL_Scancode::KP_DBLVERTICALBAR) | [`SDL_SCANCODE_KP_DBLVERTICALBAR`] | |
/// | [`KP_COLON`](SDL_Scancode::KP_COLON) | [`SDL_SCANCODE_KP_COLON`] | |
/// | [`KP_HASH`](SDL_Scancode::KP_HASH) | [`SDL_SCANCODE_KP_HASH`] | |
/// | [`KP_SPACE`](SDL_Scancode::KP_SPACE) | [`SDL_SCANCODE_KP_SPACE`] | |
/// | [`KP_AT`](SDL_Scancode::KP_AT) | [`SDL_SCANCODE_KP_AT`] | |
/// | [`KP_EXCLAM`](SDL_Scancode::KP_EXCLAM) | [`SDL_SCANCODE_KP_EXCLAM`] | |
/// | [`KP_MEMSTORE`](SDL_Scancode::KP_MEMSTORE) | [`SDL_SCANCODE_KP_MEMSTORE`] | |
/// | [`KP_MEMRECALL`](SDL_Scancode::KP_MEMRECALL) | [`SDL_SCANCODE_KP_MEMRECALL`] | |
/// | [`KP_MEMCLEAR`](SDL_Scancode::KP_MEMCLEAR) | [`SDL_SCANCODE_KP_MEMCLEAR`] | |
/// | [`KP_MEMADD`](SDL_Scancode::KP_MEMADD) | [`SDL_SCANCODE_KP_MEMADD`] | |
/// | [`KP_MEMSUBTRACT`](SDL_Scancode::KP_MEMSUBTRACT) | [`SDL_SCANCODE_KP_MEMSUBTRACT`] | |
/// | [`KP_MEMMULTIPLY`](SDL_Scancode::KP_MEMMULTIPLY) | [`SDL_SCANCODE_KP_MEMMULTIPLY`] | |
/// | [`KP_MEMDIVIDE`](SDL_Scancode::KP_MEMDIVIDE) | [`SDL_SCANCODE_KP_MEMDIVIDE`] | |
/// | [`KP_PLUSMINUS`](SDL_Scancode::KP_PLUSMINUS) | [`SDL_SCANCODE_KP_PLUSMINUS`] | |
/// | [`KP_CLEAR`](SDL_Scancode::KP_CLEAR) | [`SDL_SCANCODE_KP_CLEAR`] | |
/// | [`KP_CLEARENTRY`](SDL_Scancode::KP_CLEARENTRY) | [`SDL_SCANCODE_KP_CLEARENTRY`] | |
/// | [`KP_BINARY`](SDL_Scancode::KP_BINARY) | [`SDL_SCANCODE_KP_BINARY`] | |
/// | [`KP_OCTAL`](SDL_Scancode::KP_OCTAL) | [`SDL_SCANCODE_KP_OCTAL`] | |
/// | [`KP_DECIMAL`](SDL_Scancode::KP_DECIMAL) | [`SDL_SCANCODE_KP_DECIMAL`] | |
/// | [`KP_HEXADECIMAL`](SDL_Scancode::KP_HEXADECIMAL) | [`SDL_SCANCODE_KP_HEXADECIMAL`] | |
/// | [`LCTRL`](SDL_Scancode::LCTRL) | [`SDL_SCANCODE_LCTRL`] | |
/// | [`LSHIFT`](SDL_Scancode::LSHIFT) | [`SDL_SCANCODE_LSHIFT`] | |
/// | [`LALT`](SDL_Scancode::LALT) | [`SDL_SCANCODE_LALT`] | alt, option |
/// | [`LGUI`](SDL_Scancode::LGUI) | [`SDL_SCANCODE_LGUI`] | windows, command (apple), meta |
/// | [`RCTRL`](SDL_Scancode::RCTRL) | [`SDL_SCANCODE_RCTRL`] | |
/// | [`RSHIFT`](SDL_Scancode::RSHIFT) | [`SDL_SCANCODE_RSHIFT`] | |
/// | [`RALT`](SDL_Scancode::RALT) | [`SDL_SCANCODE_RALT`] | alt gr, option |
/// | [`RGUI`](SDL_Scancode::RGUI) | [`SDL_SCANCODE_RGUI`] | windows, command (apple), meta |
/// | [`MODE`](SDL_Scancode::MODE) | [`SDL_SCANCODE_MODE`] | I'm not sure if this is really not covered by any of the above, but since there's a special [`SDL_KMOD_MODE`] for it I'm adding it here |
/// | [`SLEEP`](SDL_Scancode::SLEEP) | [`SDL_SCANCODE_SLEEP`] | Sleep |
/// | [`WAKE`](SDL_Scancode::WAKE) | [`SDL_SCANCODE_WAKE`] | Wake |
/// | [`CHANNEL_INCREMENT`](SDL_Scancode::CHANNEL_INCREMENT) | [`SDL_SCANCODE_CHANNEL_INCREMENT`] | Channel Increment |
/// | [`CHANNEL_DECREMENT`](SDL_Scancode::CHANNEL_DECREMENT) | [`SDL_SCANCODE_CHANNEL_DECREMENT`] | Channel Decrement |
/// | [`MEDIA_PLAY`](SDL_Scancode::MEDIA_PLAY) | [`SDL_SCANCODE_MEDIA_PLAY`] | Play |
/// | [`MEDIA_PAUSE`](SDL_Scancode::MEDIA_PAUSE) | [`SDL_SCANCODE_MEDIA_PAUSE`] | Pause |
/// | [`MEDIA_RECORD`](SDL_Scancode::MEDIA_RECORD) | [`SDL_SCANCODE_MEDIA_RECORD`] | Record |
/// | [`MEDIA_FAST_FORWARD`](SDL_Scancode::MEDIA_FAST_FORWARD) | [`SDL_SCANCODE_MEDIA_FAST_FORWARD`] | Fast Forward |
/// | [`MEDIA_REWIND`](SDL_Scancode::MEDIA_REWIND) | [`SDL_SCANCODE_MEDIA_REWIND`] | Rewind |
/// | [`MEDIA_NEXT_TRACK`](SDL_Scancode::MEDIA_NEXT_TRACK) | [`SDL_SCANCODE_MEDIA_NEXT_TRACK`] | Next Track |
/// | [`MEDIA_PREVIOUS_TRACK`](SDL_Scancode::MEDIA_PREVIOUS_TRACK) | [`SDL_SCANCODE_MEDIA_PREVIOUS_TRACK`] | Previous Track |
/// | [`MEDIA_STOP`](SDL_Scancode::MEDIA_STOP) | [`SDL_SCANCODE_MEDIA_STOP`] | Stop |
/// | [`MEDIA_EJECT`](SDL_Scancode::MEDIA_EJECT) | [`SDL_SCANCODE_MEDIA_EJECT`] | Eject |
/// | [`MEDIA_PLAY_PAUSE`](SDL_Scancode::MEDIA_PLAY_PAUSE) | [`SDL_SCANCODE_MEDIA_PLAY_PAUSE`] | Play / Pause |
/// | [`MEDIA_SELECT`](SDL_Scancode::MEDIA_SELECT) | [`SDL_SCANCODE_MEDIA_SELECT`] | |
/// | [`AC_NEW`](SDL_Scancode::AC_NEW) | [`SDL_SCANCODE_AC_NEW`] | AC New |
/// | [`AC_OPEN`](SDL_Scancode::AC_OPEN) | [`SDL_SCANCODE_AC_OPEN`] | AC Open |
/// | [`AC_CLOSE`](SDL_Scancode::AC_CLOSE) | [`SDL_SCANCODE_AC_CLOSE`] | AC Close |
/// | [`AC_EXIT`](SDL_Scancode::AC_EXIT) | [`SDL_SCANCODE_AC_EXIT`] | AC Exit |
/// | [`AC_SAVE`](SDL_Scancode::AC_SAVE) | [`SDL_SCANCODE_AC_SAVE`] | AC Save |
/// | [`AC_PRINT`](SDL_Scancode::AC_PRINT) | [`SDL_SCANCODE_AC_PRINT`] | AC Print |
/// | [`AC_PROPERTIES`](SDL_Scancode::AC_PROPERTIES) | [`SDL_SCANCODE_AC_PROPERTIES`] | AC Properties |
/// | [`AC_SEARCH`](SDL_Scancode::AC_SEARCH) | [`SDL_SCANCODE_AC_SEARCH`] | AC Search |
/// | [`AC_HOME`](SDL_Scancode::AC_HOME) | [`SDL_SCANCODE_AC_HOME`] | AC Home |
/// | [`AC_BACK`](SDL_Scancode::AC_BACK) | [`SDL_SCANCODE_AC_BACK`] | AC Back |
/// | [`AC_FORWARD`](SDL_Scancode::AC_FORWARD) | [`SDL_SCANCODE_AC_FORWARD`] | AC Forward |
/// | [`AC_STOP`](SDL_Scancode::AC_STOP) | [`SDL_SCANCODE_AC_STOP`] | AC Stop |
/// | [`AC_REFRESH`](SDL_Scancode::AC_REFRESH) | [`SDL_SCANCODE_AC_REFRESH`] | AC Refresh |
/// | [`AC_BOOKMARKS`](SDL_Scancode::AC_BOOKMARKS) | [`SDL_SCANCODE_AC_BOOKMARKS`] | AC Bookmarks |
/// | [`SOFTLEFT`](SDL_Scancode::SOFTLEFT) | [`SDL_SCANCODE_SOFTLEFT`] | Usually situated below the display on phones and used as a multi-function feature key for selecting a software defined function shown on the bottom left of the display. |
/// | [`SOFTRIGHT`](SDL_Scancode::SOFTRIGHT) | [`SDL_SCANCODE_SOFTRIGHT`] | Usually situated below the display on phones and used as a multi-function feature key for selecting a software defined function shown on the bottom right of the display. |
/// | [`CALL`](SDL_Scancode::CALL) | [`SDL_SCANCODE_CALL`] | Used for accepting phone calls. |
/// | [`ENDCALL`](SDL_Scancode::ENDCALL) | [`SDL_SCANCODE_ENDCALL`] | Used for rejecting phone calls. |
/// | [`RESERVED`](SDL_Scancode::RESERVED) | [`SDL_SCANCODE_RESERVED`] | 400-500 reserved for dynamic keycodes |
/// | [`COUNT`](SDL_Scancode::COUNT) | [`SDL_SCANCODE_COUNT`] | not a key, just marks the number of scancodes for array bounds |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Scancode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_Scancode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_Scancode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_Scancode) -> bool {
        self == &other.0
    }
}

impl From<SDL_Scancode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Scancode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_Scancode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_SCANCODE_UNKNOWN",
            Self::A => "SDL_SCANCODE_A",
            Self::B => "SDL_SCANCODE_B",
            Self::C => "SDL_SCANCODE_C",
            Self::D => "SDL_SCANCODE_D",
            Self::E => "SDL_SCANCODE_E",
            Self::F => "SDL_SCANCODE_F",
            Self::G => "SDL_SCANCODE_G",
            Self::H => "SDL_SCANCODE_H",
            Self::I => "SDL_SCANCODE_I",
            Self::J => "SDL_SCANCODE_J",
            Self::K => "SDL_SCANCODE_K",
            Self::L => "SDL_SCANCODE_L",
            Self::M => "SDL_SCANCODE_M",
            Self::N => "SDL_SCANCODE_N",
            Self::O => "SDL_SCANCODE_O",
            Self::P => "SDL_SCANCODE_P",
            Self::Q => "SDL_SCANCODE_Q",
            Self::R => "SDL_SCANCODE_R",
            Self::S => "SDL_SCANCODE_S",
            Self::T => "SDL_SCANCODE_T",
            Self::U => "SDL_SCANCODE_U",
            Self::V => "SDL_SCANCODE_V",
            Self::W => "SDL_SCANCODE_W",
            Self::X => "SDL_SCANCODE_X",
            Self::Y => "SDL_SCANCODE_Y",
            Self::Z => "SDL_SCANCODE_Z",
            Self::_1 => "SDL_SCANCODE_1",
            Self::_2 => "SDL_SCANCODE_2",
            Self::_3 => "SDL_SCANCODE_3",
            Self::_4 => "SDL_SCANCODE_4",
            Self::_5 => "SDL_SCANCODE_5",
            Self::_6 => "SDL_SCANCODE_6",
            Self::_7 => "SDL_SCANCODE_7",
            Self::_8 => "SDL_SCANCODE_8",
            Self::_9 => "SDL_SCANCODE_9",
            Self::_0 => "SDL_SCANCODE_0",
            Self::RETURN => "SDL_SCANCODE_RETURN",
            Self::ESCAPE => "SDL_SCANCODE_ESCAPE",
            Self::BACKSPACE => "SDL_SCANCODE_BACKSPACE",
            Self::TAB => "SDL_SCANCODE_TAB",
            Self::SPACE => "SDL_SCANCODE_SPACE",
            Self::MINUS => "SDL_SCANCODE_MINUS",
            Self::EQUALS => "SDL_SCANCODE_EQUALS",
            Self::LEFTBRACKET => "SDL_SCANCODE_LEFTBRACKET",
            Self::RIGHTBRACKET => "SDL_SCANCODE_RIGHTBRACKET",
            Self::BACKSLASH => "SDL_SCANCODE_BACKSLASH",
            Self::NONUSHASH => "SDL_SCANCODE_NONUSHASH",
            Self::SEMICOLON => "SDL_SCANCODE_SEMICOLON",
            Self::APOSTROPHE => "SDL_SCANCODE_APOSTROPHE",
            Self::GRAVE => "SDL_SCANCODE_GRAVE",
            Self::COMMA => "SDL_SCANCODE_COMMA",
            Self::PERIOD => "SDL_SCANCODE_PERIOD",
            Self::SLASH => "SDL_SCANCODE_SLASH",
            Self::CAPSLOCK => "SDL_SCANCODE_CAPSLOCK",
            Self::F1 => "SDL_SCANCODE_F1",
            Self::F2 => "SDL_SCANCODE_F2",
            Self::F3 => "SDL_SCANCODE_F3",
            Self::F4 => "SDL_SCANCODE_F4",
            Self::F5 => "SDL_SCANCODE_F5",
            Self::F6 => "SDL_SCANCODE_F6",
            Self::F7 => "SDL_SCANCODE_F7",
            Self::F8 => "SDL_SCANCODE_F8",
            Self::F9 => "SDL_SCANCODE_F9",
            Self::F10 => "SDL_SCANCODE_F10",
            Self::F11 => "SDL_SCANCODE_F11",
            Self::F12 => "SDL_SCANCODE_F12",
            Self::PRINTSCREEN => "SDL_SCANCODE_PRINTSCREEN",
            Self::SCROLLLOCK => "SDL_SCANCODE_SCROLLLOCK",
            Self::PAUSE => "SDL_SCANCODE_PAUSE",
            Self::INSERT => "SDL_SCANCODE_INSERT",
            Self::HOME => "SDL_SCANCODE_HOME",
            Self::PAGEUP => "SDL_SCANCODE_PAGEUP",
            Self::DELETE => "SDL_SCANCODE_DELETE",
            Self::END => "SDL_SCANCODE_END",
            Self::PAGEDOWN => "SDL_SCANCODE_PAGEDOWN",
            Self::RIGHT => "SDL_SCANCODE_RIGHT",
            Self::LEFT => "SDL_SCANCODE_LEFT",
            Self::DOWN => "SDL_SCANCODE_DOWN",
            Self::UP => "SDL_SCANCODE_UP",
            Self::NUMLOCKCLEAR => "SDL_SCANCODE_NUMLOCKCLEAR",
            Self::KP_DIVIDE => "SDL_SCANCODE_KP_DIVIDE",
            Self::KP_MULTIPLY => "SDL_SCANCODE_KP_MULTIPLY",
            Self::KP_MINUS => "SDL_SCANCODE_KP_MINUS",
            Self::KP_PLUS => "SDL_SCANCODE_KP_PLUS",
            Self::KP_ENTER => "SDL_SCANCODE_KP_ENTER",
            Self::KP_1 => "SDL_SCANCODE_KP_1",
            Self::KP_2 => "SDL_SCANCODE_KP_2",
            Self::KP_3 => "SDL_SCANCODE_KP_3",
            Self::KP_4 => "SDL_SCANCODE_KP_4",
            Self::KP_5 => "SDL_SCANCODE_KP_5",
            Self::KP_6 => "SDL_SCANCODE_KP_6",
            Self::KP_7 => "SDL_SCANCODE_KP_7",
            Self::KP_8 => "SDL_SCANCODE_KP_8",
            Self::KP_9 => "SDL_SCANCODE_KP_9",
            Self::KP_0 => "SDL_SCANCODE_KP_0",
            Self::KP_PERIOD => "SDL_SCANCODE_KP_PERIOD",
            Self::NONUSBACKSLASH => "SDL_SCANCODE_NONUSBACKSLASH",
            Self::APPLICATION => "SDL_SCANCODE_APPLICATION",
            Self::POWER => "SDL_SCANCODE_POWER",
            Self::KP_EQUALS => "SDL_SCANCODE_KP_EQUALS",
            Self::F13 => "SDL_SCANCODE_F13",
            Self::F14 => "SDL_SCANCODE_F14",
            Self::F15 => "SDL_SCANCODE_F15",
            Self::F16 => "SDL_SCANCODE_F16",
            Self::F17 => "SDL_SCANCODE_F17",
            Self::F18 => "SDL_SCANCODE_F18",
            Self::F19 => "SDL_SCANCODE_F19",
            Self::F20 => "SDL_SCANCODE_F20",
            Self::F21 => "SDL_SCANCODE_F21",
            Self::F22 => "SDL_SCANCODE_F22",
            Self::F23 => "SDL_SCANCODE_F23",
            Self::F24 => "SDL_SCANCODE_F24",
            Self::EXECUTE => "SDL_SCANCODE_EXECUTE",
            Self::HELP => "SDL_SCANCODE_HELP",
            Self::MENU => "SDL_SCANCODE_MENU",
            Self::SELECT => "SDL_SCANCODE_SELECT",
            Self::STOP => "SDL_SCANCODE_STOP",
            Self::AGAIN => "SDL_SCANCODE_AGAIN",
            Self::UNDO => "SDL_SCANCODE_UNDO",
            Self::CUT => "SDL_SCANCODE_CUT",
            Self::COPY => "SDL_SCANCODE_COPY",
            Self::PASTE => "SDL_SCANCODE_PASTE",
            Self::FIND => "SDL_SCANCODE_FIND",
            Self::MUTE => "SDL_SCANCODE_MUTE",
            Self::VOLUMEUP => "SDL_SCANCODE_VOLUMEUP",
            Self::VOLUMEDOWN => "SDL_SCANCODE_VOLUMEDOWN",
            Self::KP_COMMA => "SDL_SCANCODE_KP_COMMA",
            Self::KP_EQUALSAS400 => "SDL_SCANCODE_KP_EQUALSAS400",
            Self::INTERNATIONAL1 => "SDL_SCANCODE_INTERNATIONAL1",
            Self::INTERNATIONAL2 => "SDL_SCANCODE_INTERNATIONAL2",
            Self::INTERNATIONAL3 => "SDL_SCANCODE_INTERNATIONAL3",
            Self::INTERNATIONAL4 => "SDL_SCANCODE_INTERNATIONAL4",
            Self::INTERNATIONAL5 => "SDL_SCANCODE_INTERNATIONAL5",
            Self::INTERNATIONAL6 => "SDL_SCANCODE_INTERNATIONAL6",
            Self::INTERNATIONAL7 => "SDL_SCANCODE_INTERNATIONAL7",
            Self::INTERNATIONAL8 => "SDL_SCANCODE_INTERNATIONAL8",
            Self::INTERNATIONAL9 => "SDL_SCANCODE_INTERNATIONAL9",
            Self::LANG1 => "SDL_SCANCODE_LANG1",
            Self::LANG2 => "SDL_SCANCODE_LANG2",
            Self::LANG3 => "SDL_SCANCODE_LANG3",
            Self::LANG4 => "SDL_SCANCODE_LANG4",
            Self::LANG5 => "SDL_SCANCODE_LANG5",
            Self::LANG6 => "SDL_SCANCODE_LANG6",
            Self::LANG7 => "SDL_SCANCODE_LANG7",
            Self::LANG8 => "SDL_SCANCODE_LANG8",
            Self::LANG9 => "SDL_SCANCODE_LANG9",
            Self::ALTERASE => "SDL_SCANCODE_ALTERASE",
            Self::SYSREQ => "SDL_SCANCODE_SYSREQ",
            Self::CANCEL => "SDL_SCANCODE_CANCEL",
            Self::CLEAR => "SDL_SCANCODE_CLEAR",
            Self::PRIOR => "SDL_SCANCODE_PRIOR",
            Self::RETURN2 => "SDL_SCANCODE_RETURN2",
            Self::SEPARATOR => "SDL_SCANCODE_SEPARATOR",
            Self::OUT => "SDL_SCANCODE_OUT",
            Self::OPER => "SDL_SCANCODE_OPER",
            Self::CLEARAGAIN => "SDL_SCANCODE_CLEARAGAIN",
            Self::CRSEL => "SDL_SCANCODE_CRSEL",
            Self::EXSEL => "SDL_SCANCODE_EXSEL",
            Self::KP_00 => "SDL_SCANCODE_KP_00",
            Self::KP_000 => "SDL_SCANCODE_KP_000",
            Self::THOUSANDSSEPARATOR => "SDL_SCANCODE_THOUSANDSSEPARATOR",
            Self::DECIMALSEPARATOR => "SDL_SCANCODE_DECIMALSEPARATOR",
            Self::CURRENCYUNIT => "SDL_SCANCODE_CURRENCYUNIT",
            Self::CURRENCYSUBUNIT => "SDL_SCANCODE_CURRENCYSUBUNIT",
            Self::KP_LEFTPAREN => "SDL_SCANCODE_KP_LEFTPAREN",
            Self::KP_RIGHTPAREN => "SDL_SCANCODE_KP_RIGHTPAREN",
            Self::KP_LEFTBRACE => "SDL_SCANCODE_KP_LEFTBRACE",
            Self::KP_RIGHTBRACE => "SDL_SCANCODE_KP_RIGHTBRACE",
            Self::KP_TAB => "SDL_SCANCODE_KP_TAB",
            Self::KP_BACKSPACE => "SDL_SCANCODE_KP_BACKSPACE",
            Self::KP_A => "SDL_SCANCODE_KP_A",
            Self::KP_B => "SDL_SCANCODE_KP_B",
            Self::KP_C => "SDL_SCANCODE_KP_C",
            Self::KP_D => "SDL_SCANCODE_KP_D",
            Self::KP_E => "SDL_SCANCODE_KP_E",
            Self::KP_F => "SDL_SCANCODE_KP_F",
            Self::KP_XOR => "SDL_SCANCODE_KP_XOR",
            Self::KP_POWER => "SDL_SCANCODE_KP_POWER",
            Self::KP_PERCENT => "SDL_SCANCODE_KP_PERCENT",
            Self::KP_LESS => "SDL_SCANCODE_KP_LESS",
            Self::KP_GREATER => "SDL_SCANCODE_KP_GREATER",
            Self::KP_AMPERSAND => "SDL_SCANCODE_KP_AMPERSAND",
            Self::KP_DBLAMPERSAND => "SDL_SCANCODE_KP_DBLAMPERSAND",
            Self::KP_VERTICALBAR => "SDL_SCANCODE_KP_VERTICALBAR",
            Self::KP_DBLVERTICALBAR => "SDL_SCANCODE_KP_DBLVERTICALBAR",
            Self::KP_COLON => "SDL_SCANCODE_KP_COLON",
            Self::KP_HASH => "SDL_SCANCODE_KP_HASH",
            Self::KP_SPACE => "SDL_SCANCODE_KP_SPACE",
            Self::KP_AT => "SDL_SCANCODE_KP_AT",
            Self::KP_EXCLAM => "SDL_SCANCODE_KP_EXCLAM",
            Self::KP_MEMSTORE => "SDL_SCANCODE_KP_MEMSTORE",
            Self::KP_MEMRECALL => "SDL_SCANCODE_KP_MEMRECALL",
            Self::KP_MEMCLEAR => "SDL_SCANCODE_KP_MEMCLEAR",
            Self::KP_MEMADD => "SDL_SCANCODE_KP_MEMADD",
            Self::KP_MEMSUBTRACT => "SDL_SCANCODE_KP_MEMSUBTRACT",
            Self::KP_MEMMULTIPLY => "SDL_SCANCODE_KP_MEMMULTIPLY",
            Self::KP_MEMDIVIDE => "SDL_SCANCODE_KP_MEMDIVIDE",
            Self::KP_PLUSMINUS => "SDL_SCANCODE_KP_PLUSMINUS",
            Self::KP_CLEAR => "SDL_SCANCODE_KP_CLEAR",
            Self::KP_CLEARENTRY => "SDL_SCANCODE_KP_CLEARENTRY",
            Self::KP_BINARY => "SDL_SCANCODE_KP_BINARY",
            Self::KP_OCTAL => "SDL_SCANCODE_KP_OCTAL",
            Self::KP_DECIMAL => "SDL_SCANCODE_KP_DECIMAL",
            Self::KP_HEXADECIMAL => "SDL_SCANCODE_KP_HEXADECIMAL",
            Self::LCTRL => "SDL_SCANCODE_LCTRL",
            Self::LSHIFT => "SDL_SCANCODE_LSHIFT",
            Self::LALT => "SDL_SCANCODE_LALT",
            Self::LGUI => "SDL_SCANCODE_LGUI",
            Self::RCTRL => "SDL_SCANCODE_RCTRL",
            Self::RSHIFT => "SDL_SCANCODE_RSHIFT",
            Self::RALT => "SDL_SCANCODE_RALT",
            Self::RGUI => "SDL_SCANCODE_RGUI",
            Self::MODE => "SDL_SCANCODE_MODE",
            Self::SLEEP => "SDL_SCANCODE_SLEEP",
            Self::WAKE => "SDL_SCANCODE_WAKE",
            Self::CHANNEL_INCREMENT => "SDL_SCANCODE_CHANNEL_INCREMENT",
            Self::CHANNEL_DECREMENT => "SDL_SCANCODE_CHANNEL_DECREMENT",
            Self::MEDIA_PLAY => "SDL_SCANCODE_MEDIA_PLAY",
            Self::MEDIA_PAUSE => "SDL_SCANCODE_MEDIA_PAUSE",
            Self::MEDIA_RECORD => "SDL_SCANCODE_MEDIA_RECORD",
            Self::MEDIA_FAST_FORWARD => "SDL_SCANCODE_MEDIA_FAST_FORWARD",
            Self::MEDIA_REWIND => "SDL_SCANCODE_MEDIA_REWIND",
            Self::MEDIA_NEXT_TRACK => "SDL_SCANCODE_MEDIA_NEXT_TRACK",
            Self::MEDIA_PREVIOUS_TRACK => "SDL_SCANCODE_MEDIA_PREVIOUS_TRACK",
            Self::MEDIA_STOP => "SDL_SCANCODE_MEDIA_STOP",
            Self::MEDIA_EJECT => "SDL_SCANCODE_MEDIA_EJECT",
            Self::MEDIA_PLAY_PAUSE => "SDL_SCANCODE_MEDIA_PLAY_PAUSE",
            Self::MEDIA_SELECT => "SDL_SCANCODE_MEDIA_SELECT",
            Self::AC_NEW => "SDL_SCANCODE_AC_NEW",
            Self::AC_OPEN => "SDL_SCANCODE_AC_OPEN",
            Self::AC_CLOSE => "SDL_SCANCODE_AC_CLOSE",
            Self::AC_EXIT => "SDL_SCANCODE_AC_EXIT",
            Self::AC_SAVE => "SDL_SCANCODE_AC_SAVE",
            Self::AC_PRINT => "SDL_SCANCODE_AC_PRINT",
            Self::AC_PROPERTIES => "SDL_SCANCODE_AC_PROPERTIES",
            Self::AC_SEARCH => "SDL_SCANCODE_AC_SEARCH",
            Self::AC_HOME => "SDL_SCANCODE_AC_HOME",
            Self::AC_BACK => "SDL_SCANCODE_AC_BACK",
            Self::AC_FORWARD => "SDL_SCANCODE_AC_FORWARD",
            Self::AC_STOP => "SDL_SCANCODE_AC_STOP",
            Self::AC_REFRESH => "SDL_SCANCODE_AC_REFRESH",
            Self::AC_BOOKMARKS => "SDL_SCANCODE_AC_BOOKMARKS",
            Self::SOFTLEFT => "SDL_SCANCODE_SOFTLEFT",
            Self::SOFTRIGHT => "SDL_SCANCODE_SOFTRIGHT",
            Self::CALL => "SDL_SCANCODE_CALL",
            Self::ENDCALL => "SDL_SCANCODE_ENDCALL",
            Self::RESERVED => "SDL_SCANCODE_RESERVED",
            Self::COUNT => "SDL_SCANCODE_COUNT",

            _ => return write!(f, "SDL_Scancode({})", self.0),
        })
    }
}

impl SDL_Scancode {
    pub const UNKNOWN: Self = Self((0 as ::core::ffi::c_int));
    pub const A: Self = Self((4 as ::core::ffi::c_int));
    pub const B: Self = Self((5 as ::core::ffi::c_int));
    pub const C: Self = Self((6 as ::core::ffi::c_int));
    pub const D: Self = Self((7 as ::core::ffi::c_int));
    pub const E: Self = Self((8 as ::core::ffi::c_int));
    pub const F: Self = Self((9 as ::core::ffi::c_int));
    pub const G: Self = Self((10 as ::core::ffi::c_int));
    pub const H: Self = Self((11 as ::core::ffi::c_int));
    pub const I: Self = Self((12 as ::core::ffi::c_int));
    pub const J: Self = Self((13 as ::core::ffi::c_int));
    pub const K: Self = Self((14 as ::core::ffi::c_int));
    pub const L: Self = Self((15 as ::core::ffi::c_int));
    pub const M: Self = Self((16 as ::core::ffi::c_int));
    pub const N: Self = Self((17 as ::core::ffi::c_int));
    pub const O: Self = Self((18 as ::core::ffi::c_int));
    pub const P: Self = Self((19 as ::core::ffi::c_int));
    pub const Q: Self = Self((20 as ::core::ffi::c_int));
    pub const R: Self = Self((21 as ::core::ffi::c_int));
    pub const S: Self = Self((22 as ::core::ffi::c_int));
    pub const T: Self = Self((23 as ::core::ffi::c_int));
    pub const U: Self = Self((24 as ::core::ffi::c_int));
    pub const V: Self = Self((25 as ::core::ffi::c_int));
    pub const W: Self = Self((26 as ::core::ffi::c_int));
    pub const X: Self = Self((27 as ::core::ffi::c_int));
    pub const Y: Self = Self((28 as ::core::ffi::c_int));
    pub const Z: Self = Self((29 as ::core::ffi::c_int));
    pub const _1: Self = Self((30 as ::core::ffi::c_int));
    pub const _2: Self = Self((31 as ::core::ffi::c_int));
    pub const _3: Self = Self((32 as ::core::ffi::c_int));
    pub const _4: Self = Self((33 as ::core::ffi::c_int));
    pub const _5: Self = Self((34 as ::core::ffi::c_int));
    pub const _6: Self = Self((35 as ::core::ffi::c_int));
    pub const _7: Self = Self((36 as ::core::ffi::c_int));
    pub const _8: Self = Self((37 as ::core::ffi::c_int));
    pub const _9: Self = Self((38 as ::core::ffi::c_int));
    pub const _0: Self = Self((39 as ::core::ffi::c_int));
    pub const RETURN: Self = Self((40 as ::core::ffi::c_int));
    pub const ESCAPE: Self = Self((41 as ::core::ffi::c_int));
    pub const BACKSPACE: Self = Self((42 as ::core::ffi::c_int));
    pub const TAB: Self = Self((43 as ::core::ffi::c_int));
    pub const SPACE: Self = Self((44 as ::core::ffi::c_int));
    pub const MINUS: Self = Self((45 as ::core::ffi::c_int));
    pub const EQUALS: Self = Self((46 as ::core::ffi::c_int));
    pub const LEFTBRACKET: Self = Self((47 as ::core::ffi::c_int));
    pub const RIGHTBRACKET: Self = Self((48 as ::core::ffi::c_int));
    /// Located at the lower left of the return
    /// key on ISO keyboards and at the right end
    /// of the QWERTY row on ANSI keyboards.
    /// Produces REVERSE SOLIDUS (backslash) and
    /// VERTICAL LINE in a US layout, REVERSE
    /// SOLIDUS and VERTICAL LINE in a UK Mac
    /// layout, NUMBER SIGN and TILDE in a UK
    /// Windows layout, DOLLAR SIGN and POUND SIGN
    /// in a Swiss German layout, NUMBER SIGN and
    /// APOSTROPHE in a German layout, GRAVE
    /// ACCENT and POUND SIGN in a French Mac
    /// layout, and ASTERISK and MICRO SIGN in a
    /// French Windows layout.
    pub const BACKSLASH: Self = Self((49 as ::core::ffi::c_int));
    /// ISO USB keyboards actually use this code
    /// instead of 49 for the same key, but all
    /// OSes I've seen treat the two codes
    /// identically. So, as an implementor, unless
    /// your keyboard generates both of those
    /// codes and your OS treats them differently,
    /// you should generate [`SDL_SCANCODE_BACKSLASH`]
    /// instead of this code. As a user, you
    /// should not rely on this code because SDL
    /// will never generate it with most (all?)
    /// keyboards.
    pub const NONUSHASH: Self = Self((50 as ::core::ffi::c_int));
    pub const SEMICOLON: Self = Self((51 as ::core::ffi::c_int));
    pub const APOSTROPHE: Self = Self((52 as ::core::ffi::c_int));
    /// Located in the top left corner (on both ANSI
    /// and ISO keyboards). Produces GRAVE ACCENT and
    /// TILDE in a US Windows layout and in US and UK
    /// Mac layouts on ANSI keyboards, GRAVE ACCENT
    /// and NOT SIGN in a UK Windows layout, SECTION
    /// SIGN and PLUS-MINUS SIGN in US and UK Mac
    /// layouts on ISO keyboards, SECTION SIGN and
    /// DEGREE SIGN in a Swiss German layout (Mac:
    /// only on ISO keyboards), CIRCUMFLEX ACCENT and
    /// DEGREE SIGN in a German layout (Mac: only on
    /// ISO keyboards), SUPERSCRIPT TWO and TILDE in a
    /// French Windows layout, COMMERCIAL AT and
    /// NUMBER SIGN in a French Mac layout on ISO
    /// keyboards, and LESS-THAN SIGN and GREATER-THAN
    /// SIGN in a Swiss German, German, or French Mac
    /// layout on ANSI keyboards.
    pub const GRAVE: Self = Self((53 as ::core::ffi::c_int));
    pub const COMMA: Self = Self((54 as ::core::ffi::c_int));
    pub const PERIOD: Self = Self((55 as ::core::ffi::c_int));
    pub const SLASH: Self = Self((56 as ::core::ffi::c_int));
    pub const CAPSLOCK: Self = Self((57 as ::core::ffi::c_int));
    pub const F1: Self = Self((58 as ::core::ffi::c_int));
    pub const F2: Self = Self((59 as ::core::ffi::c_int));
    pub const F3: Self = Self((60 as ::core::ffi::c_int));
    pub const F4: Self = Self((61 as ::core::ffi::c_int));
    pub const F5: Self = Self((62 as ::core::ffi::c_int));
    pub const F6: Self = Self((63 as ::core::ffi::c_int));
    pub const F7: Self = Self((64 as ::core::ffi::c_int));
    pub const F8: Self = Self((65 as ::core::ffi::c_int));
    pub const F9: Self = Self((66 as ::core::ffi::c_int));
    pub const F10: Self = Self((67 as ::core::ffi::c_int));
    pub const F11: Self = Self((68 as ::core::ffi::c_int));
    pub const F12: Self = Self((69 as ::core::ffi::c_int));
    pub const PRINTSCREEN: Self = Self((70 as ::core::ffi::c_int));
    pub const SCROLLLOCK: Self = Self((71 as ::core::ffi::c_int));
    pub const PAUSE: Self = Self((72 as ::core::ffi::c_int));
    /// insert on PC, help on some Mac keyboards (but
    /// does send code 73, not 117)
    pub const INSERT: Self = Self((73 as ::core::ffi::c_int));
    pub const HOME: Self = Self((74 as ::core::ffi::c_int));
    pub const PAGEUP: Self = Self((75 as ::core::ffi::c_int));
    pub const DELETE: Self = Self((76 as ::core::ffi::c_int));
    pub const END: Self = Self((77 as ::core::ffi::c_int));
    pub const PAGEDOWN: Self = Self((78 as ::core::ffi::c_int));
    pub const RIGHT: Self = Self((79 as ::core::ffi::c_int));
    pub const LEFT: Self = Self((80 as ::core::ffi::c_int));
    pub const DOWN: Self = Self((81 as ::core::ffi::c_int));
    pub const UP: Self = Self((82 as ::core::ffi::c_int));
    /// num lock on PC, clear on Mac keyboards
    pub const NUMLOCKCLEAR: Self = Self((83 as ::core::ffi::c_int));
    pub const KP_DIVIDE: Self = Self((84 as ::core::ffi::c_int));
    pub const KP_MULTIPLY: Self = Self((85 as ::core::ffi::c_int));
    pub const KP_MINUS: Self = Self((86 as ::core::ffi::c_int));
    pub const KP_PLUS: Self = Self((87 as ::core::ffi::c_int));
    pub const KP_ENTER: Self = Self((88 as ::core::ffi::c_int));
    pub const KP_1: Self = Self((89 as ::core::ffi::c_int));
    pub const KP_2: Self = Self((90 as ::core::ffi::c_int));
    pub const KP_3: Self = Self((91 as ::core::ffi::c_int));
    pub const KP_4: Self = Self((92 as ::core::ffi::c_int));
    pub const KP_5: Self = Self((93 as ::core::ffi::c_int));
    pub const KP_6: Self = Self((94 as ::core::ffi::c_int));
    pub const KP_7: Self = Self((95 as ::core::ffi::c_int));
    pub const KP_8: Self = Self((96 as ::core::ffi::c_int));
    pub const KP_9: Self = Self((97 as ::core::ffi::c_int));
    pub const KP_0: Self = Self((98 as ::core::ffi::c_int));
    pub const KP_PERIOD: Self = Self((99 as ::core::ffi::c_int));
    /// This is the additional key that ISO
    /// keyboards have over ANSI ones,
    /// located between left shift and Y.
    /// Produces GRAVE ACCENT and TILDE in a
    /// US or UK Mac layout, REVERSE SOLIDUS
    /// (backslash) and VERTICAL LINE in a
    /// US or UK Windows layout, and
    /// LESS-THAN SIGN and GREATER-THAN SIGN
    /// in a Swiss German, German, or French
    /// layout.
    pub const NONUSBACKSLASH: Self = Self((100 as ::core::ffi::c_int));
    /// windows contextual menu, compose
    pub const APPLICATION: Self = Self((101 as ::core::ffi::c_int));
    /// The USB document says this is a status flag,
    /// not a physical key - but some Mac keyboards
    /// do have a power key.
    pub const POWER: Self = Self((102 as ::core::ffi::c_int));
    pub const KP_EQUALS: Self = Self((103 as ::core::ffi::c_int));
    pub const F13: Self = Self((104 as ::core::ffi::c_int));
    pub const F14: Self = Self((105 as ::core::ffi::c_int));
    pub const F15: Self = Self((106 as ::core::ffi::c_int));
    pub const F16: Self = Self((107 as ::core::ffi::c_int));
    pub const F17: Self = Self((108 as ::core::ffi::c_int));
    pub const F18: Self = Self((109 as ::core::ffi::c_int));
    pub const F19: Self = Self((110 as ::core::ffi::c_int));
    pub const F20: Self = Self((111 as ::core::ffi::c_int));
    pub const F21: Self = Self((112 as ::core::ffi::c_int));
    pub const F22: Self = Self((113 as ::core::ffi::c_int));
    pub const F23: Self = Self((114 as ::core::ffi::c_int));
    pub const F24: Self = Self((115 as ::core::ffi::c_int));
    pub const EXECUTE: Self = Self((116 as ::core::ffi::c_int));
    /// AL Integrated Help Center
    pub const HELP: Self = Self((117 as ::core::ffi::c_int));
    /// Menu (show menu)
    pub const MENU: Self = Self((118 as ::core::ffi::c_int));
    pub const SELECT: Self = Self((119 as ::core::ffi::c_int));
    /// AC Stop
    pub const STOP: Self = Self((120 as ::core::ffi::c_int));
    /// AC Redo/Repeat
    pub const AGAIN: Self = Self((121 as ::core::ffi::c_int));
    /// AC Undo
    pub const UNDO: Self = Self((122 as ::core::ffi::c_int));
    /// AC Cut
    pub const CUT: Self = Self((123 as ::core::ffi::c_int));
    /// AC Copy
    pub const COPY: Self = Self((124 as ::core::ffi::c_int));
    /// AC Paste
    pub const PASTE: Self = Self((125 as ::core::ffi::c_int));
    /// AC Find
    pub const FIND: Self = Self((126 as ::core::ffi::c_int));
    pub const MUTE: Self = Self((127 as ::core::ffi::c_int));
    pub const VOLUMEUP: Self = Self((128 as ::core::ffi::c_int));
    pub const VOLUMEDOWN: Self = Self((129 as ::core::ffi::c_int));
    pub const KP_COMMA: Self = Self((133 as ::core::ffi::c_int));
    pub const KP_EQUALSAS400: Self = Self((134 as ::core::ffi::c_int));
    /// used on Asian keyboards, see
    /// footnotes in USB doc
    pub const INTERNATIONAL1: Self = Self((135 as ::core::ffi::c_int));
    pub const INTERNATIONAL2: Self = Self((136 as ::core::ffi::c_int));
    /// Yen
    pub const INTERNATIONAL3: Self = Self((137 as ::core::ffi::c_int));
    pub const INTERNATIONAL4: Self = Self((138 as ::core::ffi::c_int));
    pub const INTERNATIONAL5: Self = Self((139 as ::core::ffi::c_int));
    pub const INTERNATIONAL6: Self = Self((140 as ::core::ffi::c_int));
    pub const INTERNATIONAL7: Self = Self((141 as ::core::ffi::c_int));
    pub const INTERNATIONAL8: Self = Self((142 as ::core::ffi::c_int));
    pub const INTERNATIONAL9: Self = Self((143 as ::core::ffi::c_int));
    /// Hangul/English toggle
    pub const LANG1: Self = Self((144 as ::core::ffi::c_int));
    /// Hanja conversion
    pub const LANG2: Self = Self((145 as ::core::ffi::c_int));
    /// Katakana
    pub const LANG3: Self = Self((146 as ::core::ffi::c_int));
    /// Hiragana
    pub const LANG4: Self = Self((147 as ::core::ffi::c_int));
    /// Zenkaku/Hankaku
    pub const LANG5: Self = Self((148 as ::core::ffi::c_int));
    /// reserved
    pub const LANG6: Self = Self((149 as ::core::ffi::c_int));
    /// reserved
    pub const LANG7: Self = Self((150 as ::core::ffi::c_int));
    /// reserved
    pub const LANG8: Self = Self((151 as ::core::ffi::c_int));
    /// reserved
    pub const LANG9: Self = Self((152 as ::core::ffi::c_int));
    /// Erase-Eaze
    pub const ALTERASE: Self = Self((153 as ::core::ffi::c_int));
    pub const SYSREQ: Self = Self((154 as ::core::ffi::c_int));
    /// AC Cancel
    pub const CANCEL: Self = Self((155 as ::core::ffi::c_int));
    pub const CLEAR: Self = Self((156 as ::core::ffi::c_int));
    pub const PRIOR: Self = Self((157 as ::core::ffi::c_int));
    pub const RETURN2: Self = Self((158 as ::core::ffi::c_int));
    pub const SEPARATOR: Self = Self((159 as ::core::ffi::c_int));
    pub const OUT: Self = Self((160 as ::core::ffi::c_int));
    pub const OPER: Self = Self((161 as ::core::ffi::c_int));
    pub const CLEARAGAIN: Self = Self((162 as ::core::ffi::c_int));
    pub const CRSEL: Self = Self((163 as ::core::ffi::c_int));
    pub const EXSEL: Self = Self((164 as ::core::ffi::c_int));
    pub const KP_00: Self = Self((176 as ::core::ffi::c_int));
    pub const KP_000: Self = Self((177 as ::core::ffi::c_int));
    pub const THOUSANDSSEPARATOR: Self = Self((178 as ::core::ffi::c_int));
    pub const DECIMALSEPARATOR: Self = Self((179 as ::core::ffi::c_int));
    pub const CURRENCYUNIT: Self = Self((180 as ::core::ffi::c_int));
    pub const CURRENCYSUBUNIT: Self = Self((181 as ::core::ffi::c_int));
    pub const KP_LEFTPAREN: Self = Self((182 as ::core::ffi::c_int));
    pub const KP_RIGHTPAREN: Self = Self((183 as ::core::ffi::c_int));
    pub const KP_LEFTBRACE: Self = Self((184 as ::core::ffi::c_int));
    pub const KP_RIGHTBRACE: Self = Self((185 as ::core::ffi::c_int));
    pub const KP_TAB: Self = Self((186 as ::core::ffi::c_int));
    pub const KP_BACKSPACE: Self = Self((187 as ::core::ffi::c_int));
    pub const KP_A: Self = Self((188 as ::core::ffi::c_int));
    pub const KP_B: Self = Self((189 as ::core::ffi::c_int));
    pub const KP_C: Self = Self((190 as ::core::ffi::c_int));
    pub const KP_D: Self = Self((191 as ::core::ffi::c_int));
    pub const KP_E: Self = Self((192 as ::core::ffi::c_int));
    pub const KP_F: Self = Self((193 as ::core::ffi::c_int));
    pub const KP_XOR: Self = Self((194 as ::core::ffi::c_int));
    pub const KP_POWER: Self = Self((195 as ::core::ffi::c_int));
    pub const KP_PERCENT: Self = Self((196 as ::core::ffi::c_int));
    pub const KP_LESS: Self = Self((197 as ::core::ffi::c_int));
    pub const KP_GREATER: Self = Self((198 as ::core::ffi::c_int));
    pub const KP_AMPERSAND: Self = Self((199 as ::core::ffi::c_int));
    pub const KP_DBLAMPERSAND: Self = Self((200 as ::core::ffi::c_int));
    pub const KP_VERTICALBAR: Self = Self((201 as ::core::ffi::c_int));
    pub const KP_DBLVERTICALBAR: Self = Self((202 as ::core::ffi::c_int));
    pub const KP_COLON: Self = Self((203 as ::core::ffi::c_int));
    pub const KP_HASH: Self = Self((204 as ::core::ffi::c_int));
    pub const KP_SPACE: Self = Self((205 as ::core::ffi::c_int));
    pub const KP_AT: Self = Self((206 as ::core::ffi::c_int));
    pub const KP_EXCLAM: Self = Self((207 as ::core::ffi::c_int));
    pub const KP_MEMSTORE: Self = Self((208 as ::core::ffi::c_int));
    pub const KP_MEMRECALL: Self = Self((209 as ::core::ffi::c_int));
    pub const KP_MEMCLEAR: Self = Self((210 as ::core::ffi::c_int));
    pub const KP_MEMADD: Self = Self((211 as ::core::ffi::c_int));
    pub const KP_MEMSUBTRACT: Self = Self((212 as ::core::ffi::c_int));
    pub const KP_MEMMULTIPLY: Self = Self((213 as ::core::ffi::c_int));
    pub const KP_MEMDIVIDE: Self = Self((214 as ::core::ffi::c_int));
    pub const KP_PLUSMINUS: Self = Self((215 as ::core::ffi::c_int));
    pub const KP_CLEAR: Self = Self((216 as ::core::ffi::c_int));
    pub const KP_CLEARENTRY: Self = Self((217 as ::core::ffi::c_int));
    pub const KP_BINARY: Self = Self((218 as ::core::ffi::c_int));
    pub const KP_OCTAL: Self = Self((219 as ::core::ffi::c_int));
    pub const KP_DECIMAL: Self = Self((220 as ::core::ffi::c_int));
    pub const KP_HEXADECIMAL: Self = Self((221 as ::core::ffi::c_int));
    pub const LCTRL: Self = Self((224 as ::core::ffi::c_int));
    pub const LSHIFT: Self = Self((225 as ::core::ffi::c_int));
    /// alt, option
    pub const LALT: Self = Self((226 as ::core::ffi::c_int));
    /// windows, command (apple), meta
    pub const LGUI: Self = Self((227 as ::core::ffi::c_int));
    pub const RCTRL: Self = Self((228 as ::core::ffi::c_int));
    pub const RSHIFT: Self = Self((229 as ::core::ffi::c_int));
    /// alt gr, option
    pub const RALT: Self = Self((230 as ::core::ffi::c_int));
    /// windows, command (apple), meta
    pub const RGUI: Self = Self((231 as ::core::ffi::c_int));
    /// I'm not sure if this is really not covered
    /// by any of the above, but since there's a
    /// special [`SDL_KMOD_MODE`] for it I'm adding it here
    pub const MODE: Self = Self((257 as ::core::ffi::c_int));
    /// Sleep
    pub const SLEEP: Self = Self((258 as ::core::ffi::c_int));
    /// Wake
    pub const WAKE: Self = Self((259 as ::core::ffi::c_int));
    /// Channel Increment
    pub const CHANNEL_INCREMENT: Self = Self((260 as ::core::ffi::c_int));
    /// Channel Decrement
    pub const CHANNEL_DECREMENT: Self = Self((261 as ::core::ffi::c_int));
    /// Play
    pub const MEDIA_PLAY: Self = Self((262 as ::core::ffi::c_int));
    /// Pause
    pub const MEDIA_PAUSE: Self = Self((263 as ::core::ffi::c_int));
    /// Record
    pub const MEDIA_RECORD: Self = Self((264 as ::core::ffi::c_int));
    /// Fast Forward
    pub const MEDIA_FAST_FORWARD: Self = Self((265 as ::core::ffi::c_int));
    /// Rewind
    pub const MEDIA_REWIND: Self = Self((266 as ::core::ffi::c_int));
    /// Next Track
    pub const MEDIA_NEXT_TRACK: Self = Self((267 as ::core::ffi::c_int));
    /// Previous Track
    pub const MEDIA_PREVIOUS_TRACK: Self = Self((268 as ::core::ffi::c_int));
    /// Stop
    pub const MEDIA_STOP: Self = Self((269 as ::core::ffi::c_int));
    /// Eject
    pub const MEDIA_EJECT: Self = Self((270 as ::core::ffi::c_int));
    /// Play / Pause
    pub const MEDIA_PLAY_PAUSE: Self = Self((271 as ::core::ffi::c_int));
    pub const MEDIA_SELECT: Self = Self((272 as ::core::ffi::c_int));
    /// AC New
    pub const AC_NEW: Self = Self((273 as ::core::ffi::c_int));
    /// AC Open
    pub const AC_OPEN: Self = Self((274 as ::core::ffi::c_int));
    /// AC Close
    pub const AC_CLOSE: Self = Self((275 as ::core::ffi::c_int));
    /// AC Exit
    pub const AC_EXIT: Self = Self((276 as ::core::ffi::c_int));
    /// AC Save
    pub const AC_SAVE: Self = Self((277 as ::core::ffi::c_int));
    /// AC Print
    pub const AC_PRINT: Self = Self((278 as ::core::ffi::c_int));
    /// AC Properties
    pub const AC_PROPERTIES: Self = Self((279 as ::core::ffi::c_int));
    /// AC Search
    pub const AC_SEARCH: Self = Self((280 as ::core::ffi::c_int));
    /// AC Home
    pub const AC_HOME: Self = Self((281 as ::core::ffi::c_int));
    /// AC Back
    pub const AC_BACK: Self = Self((282 as ::core::ffi::c_int));
    /// AC Forward
    pub const AC_FORWARD: Self = Self((283 as ::core::ffi::c_int));
    /// AC Stop
    pub const AC_STOP: Self = Self((284 as ::core::ffi::c_int));
    /// AC Refresh
    pub const AC_REFRESH: Self = Self((285 as ::core::ffi::c_int));
    /// AC Bookmarks
    pub const AC_BOOKMARKS: Self = Self((286 as ::core::ffi::c_int));
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom left
    /// of the display.
    pub const SOFTLEFT: Self = Self((287 as ::core::ffi::c_int));
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom right
    /// of the display.
    pub const SOFTRIGHT: Self = Self((288 as ::core::ffi::c_int));
    /// Used for accepting phone calls.
    pub const CALL: Self = Self((289 as ::core::ffi::c_int));
    /// Used for rejecting phone calls.
    pub const ENDCALL: Self = Self((290 as ::core::ffi::c_int));
    /// 400-500 reserved for dynamic keycodes
    pub const RESERVED: Self = Self((400 as ::core::ffi::c_int));
    /// not a key, just marks the number of scancodes for array bounds
    pub const COUNT: Self = Self((512 as ::core::ffi::c_int));
}

pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = SDL_Scancode::UNKNOWN;
pub const SDL_SCANCODE_A: SDL_Scancode = SDL_Scancode::A;
pub const SDL_SCANCODE_B: SDL_Scancode = SDL_Scancode::B;
pub const SDL_SCANCODE_C: SDL_Scancode = SDL_Scancode::C;
pub const SDL_SCANCODE_D: SDL_Scancode = SDL_Scancode::D;
pub const SDL_SCANCODE_E: SDL_Scancode = SDL_Scancode::E;
pub const SDL_SCANCODE_F: SDL_Scancode = SDL_Scancode::F;
pub const SDL_SCANCODE_G: SDL_Scancode = SDL_Scancode::G;
pub const SDL_SCANCODE_H: SDL_Scancode = SDL_Scancode::H;
pub const SDL_SCANCODE_I: SDL_Scancode = SDL_Scancode::I;
pub const SDL_SCANCODE_J: SDL_Scancode = SDL_Scancode::J;
pub const SDL_SCANCODE_K: SDL_Scancode = SDL_Scancode::K;
pub const SDL_SCANCODE_L: SDL_Scancode = SDL_Scancode::L;
pub const SDL_SCANCODE_M: SDL_Scancode = SDL_Scancode::M;
pub const SDL_SCANCODE_N: SDL_Scancode = SDL_Scancode::N;
pub const SDL_SCANCODE_O: SDL_Scancode = SDL_Scancode::O;
pub const SDL_SCANCODE_P: SDL_Scancode = SDL_Scancode::P;
pub const SDL_SCANCODE_Q: SDL_Scancode = SDL_Scancode::Q;
pub const SDL_SCANCODE_R: SDL_Scancode = SDL_Scancode::R;
pub const SDL_SCANCODE_S: SDL_Scancode = SDL_Scancode::S;
pub const SDL_SCANCODE_T: SDL_Scancode = SDL_Scancode::T;
pub const SDL_SCANCODE_U: SDL_Scancode = SDL_Scancode::U;
pub const SDL_SCANCODE_V: SDL_Scancode = SDL_Scancode::V;
pub const SDL_SCANCODE_W: SDL_Scancode = SDL_Scancode::W;
pub const SDL_SCANCODE_X: SDL_Scancode = SDL_Scancode::X;
pub const SDL_SCANCODE_Y: SDL_Scancode = SDL_Scancode::Y;
pub const SDL_SCANCODE_Z: SDL_Scancode = SDL_Scancode::Z;
pub const SDL_SCANCODE_1: SDL_Scancode = SDL_Scancode::_1;
pub const SDL_SCANCODE_2: SDL_Scancode = SDL_Scancode::_2;
pub const SDL_SCANCODE_3: SDL_Scancode = SDL_Scancode::_3;
pub const SDL_SCANCODE_4: SDL_Scancode = SDL_Scancode::_4;
pub const SDL_SCANCODE_5: SDL_Scancode = SDL_Scancode::_5;
pub const SDL_SCANCODE_6: SDL_Scancode = SDL_Scancode::_6;
pub const SDL_SCANCODE_7: SDL_Scancode = SDL_Scancode::_7;
pub const SDL_SCANCODE_8: SDL_Scancode = SDL_Scancode::_8;
pub const SDL_SCANCODE_9: SDL_Scancode = SDL_Scancode::_9;
pub const SDL_SCANCODE_0: SDL_Scancode = SDL_Scancode::_0;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = SDL_Scancode::RETURN;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = SDL_Scancode::ESCAPE;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = SDL_Scancode::BACKSPACE;
pub const SDL_SCANCODE_TAB: SDL_Scancode = SDL_Scancode::TAB;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = SDL_Scancode::SPACE;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = SDL_Scancode::MINUS;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = SDL_Scancode::EQUALS;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = SDL_Scancode::LEFTBRACKET;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = SDL_Scancode::RIGHTBRACKET;
/// Located at the lower left of the return
/// key on ISO keyboards and at the right end
/// of the QWERTY row on ANSI keyboards.
/// Produces REVERSE SOLIDUS (backslash) and
/// VERTICAL LINE in a US layout, REVERSE
/// SOLIDUS and VERTICAL LINE in a UK Mac
/// layout, NUMBER SIGN and TILDE in a UK
/// Windows layout, DOLLAR SIGN and POUND SIGN
/// in a Swiss German layout, NUMBER SIGN and
/// APOSTROPHE in a German layout, GRAVE
/// ACCENT and POUND SIGN in a French Mac
/// layout, and ASTERISK and MICRO SIGN in a
/// French Windows layout.
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = SDL_Scancode::BACKSLASH;
/// ISO USB keyboards actually use this code
/// instead of 49 for the same key, but all
/// OSes I've seen treat the two codes
/// identically. So, as an implementor, unless
/// your keyboard generates both of those
/// codes and your OS treats them differently,
/// you should generate [`SDL_SCANCODE_BACKSLASH`]
/// instead of this code. As a user, you
/// should not rely on this code because SDL
/// will never generate it with most (all?)
/// keyboards.
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = SDL_Scancode::NONUSHASH;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = SDL_Scancode::SEMICOLON;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = SDL_Scancode::APOSTROPHE;
/// Located in the top left corner (on both ANSI
/// and ISO keyboards). Produces GRAVE ACCENT and
/// TILDE in a US Windows layout and in US and UK
/// Mac layouts on ANSI keyboards, GRAVE ACCENT
/// and NOT SIGN in a UK Windows layout, SECTION
/// SIGN and PLUS-MINUS SIGN in US and UK Mac
/// layouts on ISO keyboards, SECTION SIGN and
/// DEGREE SIGN in a Swiss German layout (Mac:
/// only on ISO keyboards), CIRCUMFLEX ACCENT and
/// DEGREE SIGN in a German layout (Mac: only on
/// ISO keyboards), SUPERSCRIPT TWO and TILDE in a
/// French Windows layout, COMMERCIAL AT and
/// NUMBER SIGN in a French Mac layout on ISO
/// keyboards, and LESS-THAN SIGN and GREATER-THAN
/// SIGN in a Swiss German, German, or French Mac
/// layout on ANSI keyboards.
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = SDL_Scancode::GRAVE;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = SDL_Scancode::COMMA;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = SDL_Scancode::PERIOD;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = SDL_Scancode::SLASH;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = SDL_Scancode::CAPSLOCK;
pub const SDL_SCANCODE_F1: SDL_Scancode = SDL_Scancode::F1;
pub const SDL_SCANCODE_F2: SDL_Scancode = SDL_Scancode::F2;
pub const SDL_SCANCODE_F3: SDL_Scancode = SDL_Scancode::F3;
pub const SDL_SCANCODE_F4: SDL_Scancode = SDL_Scancode::F4;
pub const SDL_SCANCODE_F5: SDL_Scancode = SDL_Scancode::F5;
pub const SDL_SCANCODE_F6: SDL_Scancode = SDL_Scancode::F6;
pub const SDL_SCANCODE_F7: SDL_Scancode = SDL_Scancode::F7;
pub const SDL_SCANCODE_F8: SDL_Scancode = SDL_Scancode::F8;
pub const SDL_SCANCODE_F9: SDL_Scancode = SDL_Scancode::F9;
pub const SDL_SCANCODE_F10: SDL_Scancode = SDL_Scancode::F10;
pub const SDL_SCANCODE_F11: SDL_Scancode = SDL_Scancode::F11;
pub const SDL_SCANCODE_F12: SDL_Scancode = SDL_Scancode::F12;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = SDL_Scancode::PRINTSCREEN;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = SDL_Scancode::SCROLLLOCK;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = SDL_Scancode::PAUSE;
/// insert on PC, help on some Mac keyboards (but
/// does send code 73, not 117)
pub const SDL_SCANCODE_INSERT: SDL_Scancode = SDL_Scancode::INSERT;
pub const SDL_SCANCODE_HOME: SDL_Scancode = SDL_Scancode::HOME;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = SDL_Scancode::PAGEUP;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = SDL_Scancode::DELETE;
pub const SDL_SCANCODE_END: SDL_Scancode = SDL_Scancode::END;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = SDL_Scancode::PAGEDOWN;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = SDL_Scancode::RIGHT;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = SDL_Scancode::LEFT;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = SDL_Scancode::DOWN;
pub const SDL_SCANCODE_UP: SDL_Scancode = SDL_Scancode::UP;
/// num lock on PC, clear on Mac keyboards
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = SDL_Scancode::NUMLOCKCLEAR;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = SDL_Scancode::KP_DIVIDE;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = SDL_Scancode::KP_MULTIPLY;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = SDL_Scancode::KP_MINUS;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = SDL_Scancode::KP_PLUS;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = SDL_Scancode::KP_ENTER;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = SDL_Scancode::KP_1;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = SDL_Scancode::KP_2;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = SDL_Scancode::KP_3;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = SDL_Scancode::KP_4;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = SDL_Scancode::KP_5;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = SDL_Scancode::KP_6;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = SDL_Scancode::KP_7;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = SDL_Scancode::KP_8;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = SDL_Scancode::KP_9;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = SDL_Scancode::KP_0;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = SDL_Scancode::KP_PERIOD;
/// This is the additional key that ISO
/// keyboards have over ANSI ones,
/// located between left shift and Y.
/// Produces GRAVE ACCENT and TILDE in a
/// US or UK Mac layout, REVERSE SOLIDUS
/// (backslash) and VERTICAL LINE in a
/// US or UK Windows layout, and
/// LESS-THAN SIGN and GREATER-THAN SIGN
/// in a Swiss German, German, or French
/// layout.
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = SDL_Scancode::NONUSBACKSLASH;
/// windows contextual menu, compose
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = SDL_Scancode::APPLICATION;
/// The USB document says this is a status flag,
/// not a physical key - but some Mac keyboards
/// do have a power key.
pub const SDL_SCANCODE_POWER: SDL_Scancode = SDL_Scancode::POWER;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = SDL_Scancode::KP_EQUALS;
pub const SDL_SCANCODE_F13: SDL_Scancode = SDL_Scancode::F13;
pub const SDL_SCANCODE_F14: SDL_Scancode = SDL_Scancode::F14;
pub const SDL_SCANCODE_F15: SDL_Scancode = SDL_Scancode::F15;
pub const SDL_SCANCODE_F16: SDL_Scancode = SDL_Scancode::F16;
pub const SDL_SCANCODE_F17: SDL_Scancode = SDL_Scancode::F17;
pub const SDL_SCANCODE_F18: SDL_Scancode = SDL_Scancode::F18;
pub const SDL_SCANCODE_F19: SDL_Scancode = SDL_Scancode::F19;
pub const SDL_SCANCODE_F20: SDL_Scancode = SDL_Scancode::F20;
pub const SDL_SCANCODE_F21: SDL_Scancode = SDL_Scancode::F21;
pub const SDL_SCANCODE_F22: SDL_Scancode = SDL_Scancode::F22;
pub const SDL_SCANCODE_F23: SDL_Scancode = SDL_Scancode::F23;
pub const SDL_SCANCODE_F24: SDL_Scancode = SDL_Scancode::F24;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = SDL_Scancode::EXECUTE;
/// AL Integrated Help Center
pub const SDL_SCANCODE_HELP: SDL_Scancode = SDL_Scancode::HELP;
/// Menu (show menu)
pub const SDL_SCANCODE_MENU: SDL_Scancode = SDL_Scancode::MENU;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = SDL_Scancode::SELECT;
/// AC Stop
pub const SDL_SCANCODE_STOP: SDL_Scancode = SDL_Scancode::STOP;
/// AC Redo/Repeat
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = SDL_Scancode::AGAIN;
/// AC Undo
pub const SDL_SCANCODE_UNDO: SDL_Scancode = SDL_Scancode::UNDO;
/// AC Cut
pub const SDL_SCANCODE_CUT: SDL_Scancode = SDL_Scancode::CUT;
/// AC Copy
pub const SDL_SCANCODE_COPY: SDL_Scancode = SDL_Scancode::COPY;
/// AC Paste
pub const SDL_SCANCODE_PASTE: SDL_Scancode = SDL_Scancode::PASTE;
/// AC Find
pub const SDL_SCANCODE_FIND: SDL_Scancode = SDL_Scancode::FIND;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = SDL_Scancode::MUTE;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = SDL_Scancode::VOLUMEUP;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = SDL_Scancode::VOLUMEDOWN;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = SDL_Scancode::KP_COMMA;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = SDL_Scancode::KP_EQUALSAS400;
/// used on Asian keyboards, see
/// footnotes in USB doc
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = SDL_Scancode::INTERNATIONAL1;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = SDL_Scancode::INTERNATIONAL2;
/// Yen
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = SDL_Scancode::INTERNATIONAL3;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = SDL_Scancode::INTERNATIONAL4;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = SDL_Scancode::INTERNATIONAL5;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = SDL_Scancode::INTERNATIONAL6;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = SDL_Scancode::INTERNATIONAL7;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = SDL_Scancode::INTERNATIONAL8;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = SDL_Scancode::INTERNATIONAL9;
/// Hangul/English toggle
pub const SDL_SCANCODE_LANG1: SDL_Scancode = SDL_Scancode::LANG1;
/// Hanja conversion
pub const SDL_SCANCODE_LANG2: SDL_Scancode = SDL_Scancode::LANG2;
/// Katakana
pub const SDL_SCANCODE_LANG3: SDL_Scancode = SDL_Scancode::LANG3;
/// Hiragana
pub const SDL_SCANCODE_LANG4: SDL_Scancode = SDL_Scancode::LANG4;
/// Zenkaku/Hankaku
pub const SDL_SCANCODE_LANG5: SDL_Scancode = SDL_Scancode::LANG5;
/// reserved
pub const SDL_SCANCODE_LANG6: SDL_Scancode = SDL_Scancode::LANG6;
/// reserved
pub const SDL_SCANCODE_LANG7: SDL_Scancode = SDL_Scancode::LANG7;
/// reserved
pub const SDL_SCANCODE_LANG8: SDL_Scancode = SDL_Scancode::LANG8;
/// reserved
pub const SDL_SCANCODE_LANG9: SDL_Scancode = SDL_Scancode::LANG9;
/// Erase-Eaze
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = SDL_Scancode::ALTERASE;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = SDL_Scancode::SYSREQ;
/// AC Cancel
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = SDL_Scancode::CANCEL;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = SDL_Scancode::CLEAR;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = SDL_Scancode::PRIOR;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = SDL_Scancode::RETURN2;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = SDL_Scancode::SEPARATOR;
pub const SDL_SCANCODE_OUT: SDL_Scancode = SDL_Scancode::OUT;
pub const SDL_SCANCODE_OPER: SDL_Scancode = SDL_Scancode::OPER;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = SDL_Scancode::CLEARAGAIN;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = SDL_Scancode::CRSEL;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = SDL_Scancode::EXSEL;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = SDL_Scancode::KP_00;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = SDL_Scancode::KP_000;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = SDL_Scancode::THOUSANDSSEPARATOR;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = SDL_Scancode::DECIMALSEPARATOR;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = SDL_Scancode::CURRENCYUNIT;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = SDL_Scancode::CURRENCYSUBUNIT;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = SDL_Scancode::KP_LEFTPAREN;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = SDL_Scancode::KP_RIGHTPAREN;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = SDL_Scancode::KP_LEFTBRACE;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = SDL_Scancode::KP_RIGHTBRACE;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = SDL_Scancode::KP_TAB;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = SDL_Scancode::KP_BACKSPACE;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = SDL_Scancode::KP_A;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = SDL_Scancode::KP_B;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = SDL_Scancode::KP_C;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = SDL_Scancode::KP_D;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = SDL_Scancode::KP_E;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = SDL_Scancode::KP_F;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = SDL_Scancode::KP_XOR;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = SDL_Scancode::KP_POWER;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = SDL_Scancode::KP_PERCENT;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = SDL_Scancode::KP_LESS;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = SDL_Scancode::KP_GREATER;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = SDL_Scancode::KP_AMPERSAND;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = SDL_Scancode::KP_DBLAMPERSAND;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = SDL_Scancode::KP_VERTICALBAR;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = SDL_Scancode::KP_DBLVERTICALBAR;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = SDL_Scancode::KP_COLON;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = SDL_Scancode::KP_HASH;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = SDL_Scancode::KP_SPACE;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = SDL_Scancode::KP_AT;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = SDL_Scancode::KP_EXCLAM;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = SDL_Scancode::KP_MEMSTORE;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = SDL_Scancode::KP_MEMRECALL;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = SDL_Scancode::KP_MEMCLEAR;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = SDL_Scancode::KP_MEMADD;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = SDL_Scancode::KP_MEMSUBTRACT;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = SDL_Scancode::KP_MEMMULTIPLY;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = SDL_Scancode::KP_MEMDIVIDE;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = SDL_Scancode::KP_PLUSMINUS;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = SDL_Scancode::KP_CLEAR;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = SDL_Scancode::KP_CLEARENTRY;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = SDL_Scancode::KP_BINARY;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = SDL_Scancode::KP_OCTAL;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = SDL_Scancode::KP_DECIMAL;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = SDL_Scancode::KP_HEXADECIMAL;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = SDL_Scancode::LCTRL;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = SDL_Scancode::LSHIFT;
/// alt, option
pub const SDL_SCANCODE_LALT: SDL_Scancode = SDL_Scancode::LALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_LGUI: SDL_Scancode = SDL_Scancode::LGUI;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = SDL_Scancode::RCTRL;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = SDL_Scancode::RSHIFT;
/// alt gr, option
pub const SDL_SCANCODE_RALT: SDL_Scancode = SDL_Scancode::RALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_RGUI: SDL_Scancode = SDL_Scancode::RGUI;
/// I'm not sure if this is really not covered
/// by any of the above, but since there's a
/// special [`SDL_KMOD_MODE`] for it I'm adding it here
pub const SDL_SCANCODE_MODE: SDL_Scancode = SDL_Scancode::MODE;
/// Sleep
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = SDL_Scancode::SLEEP;
/// Wake
pub const SDL_SCANCODE_WAKE: SDL_Scancode = SDL_Scancode::WAKE;
/// Channel Increment
pub const SDL_SCANCODE_CHANNEL_INCREMENT: SDL_Scancode = SDL_Scancode::CHANNEL_INCREMENT;
/// Channel Decrement
pub const SDL_SCANCODE_CHANNEL_DECREMENT: SDL_Scancode = SDL_Scancode::CHANNEL_DECREMENT;
/// Play
pub const SDL_SCANCODE_MEDIA_PLAY: SDL_Scancode = SDL_Scancode::MEDIA_PLAY;
/// Pause
pub const SDL_SCANCODE_MEDIA_PAUSE: SDL_Scancode = SDL_Scancode::MEDIA_PAUSE;
/// Record
pub const SDL_SCANCODE_MEDIA_RECORD: SDL_Scancode = SDL_Scancode::MEDIA_RECORD;
/// Fast Forward
pub const SDL_SCANCODE_MEDIA_FAST_FORWARD: SDL_Scancode = SDL_Scancode::MEDIA_FAST_FORWARD;
/// Rewind
pub const SDL_SCANCODE_MEDIA_REWIND: SDL_Scancode = SDL_Scancode::MEDIA_REWIND;
/// Next Track
pub const SDL_SCANCODE_MEDIA_NEXT_TRACK: SDL_Scancode = SDL_Scancode::MEDIA_NEXT_TRACK;
/// Previous Track
pub const SDL_SCANCODE_MEDIA_PREVIOUS_TRACK: SDL_Scancode = SDL_Scancode::MEDIA_PREVIOUS_TRACK;
/// Stop
pub const SDL_SCANCODE_MEDIA_STOP: SDL_Scancode = SDL_Scancode::MEDIA_STOP;
/// Eject
pub const SDL_SCANCODE_MEDIA_EJECT: SDL_Scancode = SDL_Scancode::MEDIA_EJECT;
/// Play / Pause
pub const SDL_SCANCODE_MEDIA_PLAY_PAUSE: SDL_Scancode = SDL_Scancode::MEDIA_PLAY_PAUSE;
pub const SDL_SCANCODE_MEDIA_SELECT: SDL_Scancode = SDL_Scancode::MEDIA_SELECT;
/// AC New
pub const SDL_SCANCODE_AC_NEW: SDL_Scancode = SDL_Scancode::AC_NEW;
/// AC Open
pub const SDL_SCANCODE_AC_OPEN: SDL_Scancode = SDL_Scancode::AC_OPEN;
/// AC Close
pub const SDL_SCANCODE_AC_CLOSE: SDL_Scancode = SDL_Scancode::AC_CLOSE;
/// AC Exit
pub const SDL_SCANCODE_AC_EXIT: SDL_Scancode = SDL_Scancode::AC_EXIT;
/// AC Save
pub const SDL_SCANCODE_AC_SAVE: SDL_Scancode = SDL_Scancode::AC_SAVE;
/// AC Print
pub const SDL_SCANCODE_AC_PRINT: SDL_Scancode = SDL_Scancode::AC_PRINT;
/// AC Properties
pub const SDL_SCANCODE_AC_PROPERTIES: SDL_Scancode = SDL_Scancode::AC_PROPERTIES;
/// AC Search
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = SDL_Scancode::AC_SEARCH;
/// AC Home
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = SDL_Scancode::AC_HOME;
/// AC Back
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = SDL_Scancode::AC_BACK;
/// AC Forward
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = SDL_Scancode::AC_FORWARD;
/// AC Stop
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = SDL_Scancode::AC_STOP;
/// AC Refresh
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = SDL_Scancode::AC_REFRESH;
/// AC Bookmarks
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = SDL_Scancode::AC_BOOKMARKS;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom left
/// of the display.
pub const SDL_SCANCODE_SOFTLEFT: SDL_Scancode = SDL_Scancode::SOFTLEFT;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom right
/// of the display.
pub const SDL_SCANCODE_SOFTRIGHT: SDL_Scancode = SDL_Scancode::SOFTRIGHT;
/// Used for accepting phone calls.
pub const SDL_SCANCODE_CALL: SDL_Scancode = SDL_Scancode::CALL;
/// Used for rejecting phone calls.
pub const SDL_SCANCODE_ENDCALL: SDL_Scancode = SDL_Scancode::ENDCALL;
/// 400-500 reserved for dynamic keycodes
pub const SDL_SCANCODE_RESERVED: SDL_Scancode = SDL_Scancode::RESERVED;
/// not a key, just marks the number of scancodes for array bounds
pub const SDL_SCANCODE_COUNT: SDL_Scancode = SDL_Scancode::COUNT;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_Scancode {
    const GROUP_METADATA: &sdl3_sys::metadata::Group = &sdl3_sys::metadata::Group {
        kind: sdl3_sys::metadata::GroupKind::Enum,
        module: "scancode",
        name: "SDL_Scancode",
        short_name: "Scancode",
        doc: "The SDL keyboard scancode representation.\n\nAn SDL scancode is the physical representation of a key on the keyboard,\nindependent of language and keyboard mapping.\n\nValues of this type are used to represent keyboard keys, among other places\nin the `scancode` field of the [`SDL_KeyboardEvent`] structure.\n\nThe values in this enumeration are based on the USB usage page standard:\n<https://usb.org/sites/default/files/hut1_5.pdf>\n\n### Availability\nThis enum is available since SDL 3.2.0.\n",
        values: &[
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_UNKNOWN",
                short_name: "UNKNOWN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_A",
                short_name: "A",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_B",
                short_name: "B",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_C",
                short_name: "C",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_D",
                short_name: "D",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_E",
                short_name: "E",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F",
                short_name: "F",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_G",
                short_name: "G",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_H",
                short_name: "H",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_I",
                short_name: "I",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_J",
                short_name: "J",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_K",
                short_name: "K",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_L",
                short_name: "L",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_M",
                short_name: "M",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_N",
                short_name: "N",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_O",
                short_name: "O",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_P",
                short_name: "P",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_Q",
                short_name: "Q",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_R",
                short_name: "R",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_S",
                short_name: "S",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_T",
                short_name: "T",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_U",
                short_name: "U",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_V",
                short_name: "V",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_W",
                short_name: "W",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_X",
                short_name: "X",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_Y",
                short_name: "Y",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_Z",
                short_name: "Z",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_1",
                short_name: "_1",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_2",
                short_name: "_2",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_3",
                short_name: "_3",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_4",
                short_name: "_4",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_5",
                short_name: "_5",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_6",
                short_name: "_6",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_7",
                short_name: "_7",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_8",
                short_name: "_8",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_9",
                short_name: "_9",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_0",
                short_name: "_0",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RETURN",
                short_name: "RETURN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_ESCAPE",
                short_name: "ESCAPE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_BACKSPACE",
                short_name: "BACKSPACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_TAB",
                short_name: "TAB",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SPACE",
                short_name: "SPACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MINUS",
                short_name: "MINUS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_EQUALS",
                short_name: "EQUALS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LEFTBRACKET",
                short_name: "LEFTBRACKET",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RIGHTBRACKET",
                short_name: "RIGHTBRACKET",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_BACKSLASH",
                short_name: "BACKSLASH",
                doc: "Located at the lower left of the return\nkey on ISO keyboards and at the right end\nof the QWERTY row on ANSI keyboards.\nProduces REVERSE SOLIDUS (backslash) and\nVERTICAL LINE in a US layout, REVERSE\nSOLIDUS and VERTICAL LINE in a UK Mac\nlayout, NUMBER SIGN and TILDE in a UK\nWindows layout, DOLLAR SIGN and POUND SIGN\nin a Swiss German layout, NUMBER SIGN and\nAPOSTROPHE in a German layout, GRAVE\nACCENT and POUND SIGN in a French Mac\nlayout, and ASTERISK and MICRO SIGN in a\nFrench Windows layout.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_NONUSHASH",
                short_name: "NONUSHASH",
                doc: "ISO USB keyboards actually use this code\ninstead of 49 for the same key, but all\nOSes I've seen treat the two codes\nidentically. So, as an implementor, unless\nyour keyboard generates both of those\ncodes and your OS treats them differently,\nyou should generate [`SDL_SCANCODE_BACKSLASH`]\ninstead of this code. As a user, you\nshould not rely on this code because SDL\nwill never generate it with most (all?)\nkeyboards.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SEMICOLON",
                short_name: "SEMICOLON",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_APOSTROPHE",
                short_name: "APOSTROPHE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_GRAVE",
                short_name: "GRAVE",
                doc: "Located in the top left corner (on both ANSI\nand ISO keyboards). Produces GRAVE ACCENT and\nTILDE in a US Windows layout and in US and UK\nMac layouts on ANSI keyboards, GRAVE ACCENT\nand NOT SIGN in a UK Windows layout, SECTION\nSIGN and PLUS-MINUS SIGN in US and UK Mac\nlayouts on ISO keyboards, SECTION SIGN and\nDEGREE SIGN in a Swiss German layout (Mac:\nonly on ISO keyboards), CIRCUMFLEX ACCENT and\nDEGREE SIGN in a German layout (Mac: only on\nISO keyboards), SUPERSCRIPT TWO and TILDE in a\nFrench Windows layout, COMMERCIAL AT and\nNUMBER SIGN in a French Mac layout on ISO\nkeyboards, and LESS-THAN SIGN and GREATER-THAN\nSIGN in a Swiss German, German, or French Mac\nlayout on ANSI keyboards.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_COMMA",
                short_name: "COMMA",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PERIOD",
                short_name: "PERIOD",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SLASH",
                short_name: "SLASH",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CAPSLOCK",
                short_name: "CAPSLOCK",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F1",
                short_name: "F1",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F2",
                short_name: "F2",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F3",
                short_name: "F3",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F4",
                short_name: "F4",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F5",
                short_name: "F5",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F6",
                short_name: "F6",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F7",
                short_name: "F7",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F8",
                short_name: "F8",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F9",
                short_name: "F9",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F10",
                short_name: "F10",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F11",
                short_name: "F11",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F12",
                short_name: "F12",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PRINTSCREEN",
                short_name: "PRINTSCREEN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SCROLLLOCK",
                short_name: "SCROLLLOCK",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PAUSE",
                short_name: "PAUSE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INSERT",
                short_name: "INSERT",
                doc: "insert on PC, help on some Mac keyboards (but\ndoes send code 73, not 117)\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_HOME",
                short_name: "HOME",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PAGEUP",
                short_name: "PAGEUP",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_DELETE",
                short_name: "DELETE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_END",
                short_name: "END",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PAGEDOWN",
                short_name: "PAGEDOWN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RIGHT",
                short_name: "RIGHT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LEFT",
                short_name: "LEFT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_DOWN",
                short_name: "DOWN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_UP",
                short_name: "UP",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_NUMLOCKCLEAR",
                short_name: "NUMLOCKCLEAR",
                doc: "num lock on PC, clear on Mac keyboards\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_DIVIDE",
                short_name: "KP_DIVIDE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MULTIPLY",
                short_name: "KP_MULTIPLY",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MINUS",
                short_name: "KP_MINUS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_PLUS",
                short_name: "KP_PLUS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_ENTER",
                short_name: "KP_ENTER",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_1",
                short_name: "KP_1",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_2",
                short_name: "KP_2",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_3",
                short_name: "KP_3",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_4",
                short_name: "KP_4",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_5",
                short_name: "KP_5",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_6",
                short_name: "KP_6",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_7",
                short_name: "KP_7",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_8",
                short_name: "KP_8",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_9",
                short_name: "KP_9",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_0",
                short_name: "KP_0",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_PERIOD",
                short_name: "KP_PERIOD",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_NONUSBACKSLASH",
                short_name: "NONUSBACKSLASH",
                doc: "This is the additional key that ISO\nkeyboards have over ANSI ones,\nlocated between left shift and Y.\nProduces GRAVE ACCENT and TILDE in a\nUS or UK Mac layout, REVERSE SOLIDUS\n(backslash) and VERTICAL LINE in a\nUS or UK Windows layout, and\nLESS-THAN SIGN and GREATER-THAN SIGN\nin a Swiss German, German, or French\nlayout.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_APPLICATION",
                short_name: "APPLICATION",
                doc: "windows contextual menu, compose\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_POWER",
                short_name: "POWER",
                doc: "The USB document says this is a status flag,\nnot a physical key - but some Mac keyboards\ndo have a power key.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_EQUALS",
                short_name: "KP_EQUALS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F13",
                short_name: "F13",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F14",
                short_name: "F14",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F15",
                short_name: "F15",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F16",
                short_name: "F16",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F17",
                short_name: "F17",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F18",
                short_name: "F18",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F19",
                short_name: "F19",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F20",
                short_name: "F20",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F21",
                short_name: "F21",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F22",
                short_name: "F22",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F23",
                short_name: "F23",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_F24",
                short_name: "F24",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_EXECUTE",
                short_name: "EXECUTE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_HELP",
                short_name: "HELP",
                doc: "AL Integrated Help Center\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MENU",
                short_name: "MENU",
                doc: "Menu (show menu)\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SELECT",
                short_name: "SELECT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_STOP",
                short_name: "STOP",
                doc: "AC Stop\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AGAIN",
                short_name: "AGAIN",
                doc: "AC Redo/Repeat\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_UNDO",
                short_name: "UNDO",
                doc: "AC Undo\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CUT",
                short_name: "CUT",
                doc: "AC Cut\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_COPY",
                short_name: "COPY",
                doc: "AC Copy\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PASTE",
                short_name: "PASTE",
                doc: "AC Paste\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_FIND",
                short_name: "FIND",
                doc: "AC Find\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MUTE",
                short_name: "MUTE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_VOLUMEUP",
                short_name: "VOLUMEUP",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_VOLUMEDOWN",
                short_name: "VOLUMEDOWN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_COMMA",
                short_name: "KP_COMMA",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_EQUALSAS400",
                short_name: "KP_EQUALSAS400",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL1",
                short_name: "INTERNATIONAL1",
                doc: "used on Asian keyboards, see\nfootnotes in USB doc\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL2",
                short_name: "INTERNATIONAL2",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL3",
                short_name: "INTERNATIONAL3",
                doc: "Yen\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL4",
                short_name: "INTERNATIONAL4",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL5",
                short_name: "INTERNATIONAL5",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL6",
                short_name: "INTERNATIONAL6",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL7",
                short_name: "INTERNATIONAL7",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL8",
                short_name: "INTERNATIONAL8",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_INTERNATIONAL9",
                short_name: "INTERNATIONAL9",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG1",
                short_name: "LANG1",
                doc: "Hangul/English toggle\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG2",
                short_name: "LANG2",
                doc: "Hanja conversion\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG3",
                short_name: "LANG3",
                doc: "Katakana\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG4",
                short_name: "LANG4",
                doc: "Hiragana\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG5",
                short_name: "LANG5",
                doc: "Zenkaku/Hankaku\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG6",
                short_name: "LANG6",
                doc: "reserved\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG7",
                short_name: "LANG7",
                doc: "reserved\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG8",
                short_name: "LANG8",
                doc: "reserved\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LANG9",
                short_name: "LANG9",
                doc: "reserved\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_ALTERASE",
                short_name: "ALTERASE",
                doc: "Erase-Eaze\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SYSREQ",
                short_name: "SYSREQ",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CANCEL",
                short_name: "CANCEL",
                doc: "AC Cancel\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CLEAR",
                short_name: "CLEAR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_PRIOR",
                short_name: "PRIOR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RETURN2",
                short_name: "RETURN2",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SEPARATOR",
                short_name: "SEPARATOR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_OUT",
                short_name: "OUT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_OPER",
                short_name: "OPER",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CLEARAGAIN",
                short_name: "CLEARAGAIN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CRSEL",
                short_name: "CRSEL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_EXSEL",
                short_name: "EXSEL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_00",
                short_name: "KP_00",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_000",
                short_name: "KP_000",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_THOUSANDSSEPARATOR",
                short_name: "THOUSANDSSEPARATOR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_DECIMALSEPARATOR",
                short_name: "DECIMALSEPARATOR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CURRENCYUNIT",
                short_name: "CURRENCYUNIT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CURRENCYSUBUNIT",
                short_name: "CURRENCYSUBUNIT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_LEFTPAREN",
                short_name: "KP_LEFTPAREN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_RIGHTPAREN",
                short_name: "KP_RIGHTPAREN",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_LEFTBRACE",
                short_name: "KP_LEFTBRACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_RIGHTBRACE",
                short_name: "KP_RIGHTBRACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_TAB",
                short_name: "KP_TAB",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_BACKSPACE",
                short_name: "KP_BACKSPACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_A",
                short_name: "KP_A",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_B",
                short_name: "KP_B",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_C",
                short_name: "KP_C",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_D",
                short_name: "KP_D",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_E",
                short_name: "KP_E",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_F",
                short_name: "KP_F",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_XOR",
                short_name: "KP_XOR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_POWER",
                short_name: "KP_POWER",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_PERCENT",
                short_name: "KP_PERCENT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_LESS",
                short_name: "KP_LESS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_GREATER",
                short_name: "KP_GREATER",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_AMPERSAND",
                short_name: "KP_AMPERSAND",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_DBLAMPERSAND",
                short_name: "KP_DBLAMPERSAND",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_VERTICALBAR",
                short_name: "KP_VERTICALBAR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_DBLVERTICALBAR",
                short_name: "KP_DBLVERTICALBAR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_COLON",
                short_name: "KP_COLON",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_HASH",
                short_name: "KP_HASH",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_SPACE",
                short_name: "KP_SPACE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_AT",
                short_name: "KP_AT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_EXCLAM",
                short_name: "KP_EXCLAM",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMSTORE",
                short_name: "KP_MEMSTORE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMRECALL",
                short_name: "KP_MEMRECALL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMCLEAR",
                short_name: "KP_MEMCLEAR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMADD",
                short_name: "KP_MEMADD",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMSUBTRACT",
                short_name: "KP_MEMSUBTRACT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMMULTIPLY",
                short_name: "KP_MEMMULTIPLY",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_MEMDIVIDE",
                short_name: "KP_MEMDIVIDE",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_PLUSMINUS",
                short_name: "KP_PLUSMINUS",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_CLEAR",
                short_name: "KP_CLEAR",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_CLEARENTRY",
                short_name: "KP_CLEARENTRY",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_BINARY",
                short_name: "KP_BINARY",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_OCTAL",
                short_name: "KP_OCTAL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_DECIMAL",
                short_name: "KP_DECIMAL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_KP_HEXADECIMAL",
                short_name: "KP_HEXADECIMAL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LCTRL",
                short_name: "LCTRL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LSHIFT",
                short_name: "LSHIFT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LALT",
                short_name: "LALT",
                doc: "alt, option\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_LGUI",
                short_name: "LGUI",
                doc: "windows, command (apple), meta\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RCTRL",
                short_name: "RCTRL",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RSHIFT",
                short_name: "RSHIFT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RALT",
                short_name: "RALT",
                doc: "alt gr, option\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RGUI",
                short_name: "RGUI",
                doc: "windows, command (apple), meta\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MODE",
                short_name: "MODE",
                doc: "I'm not sure if this is really not covered\nby any of the above, but since there's a\nspecial [`SDL_KMOD_MODE`] for it I'm adding it here\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SLEEP",
                short_name: "SLEEP",
                doc: "Sleep\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_WAKE",
                short_name: "WAKE",
                doc: "Wake\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CHANNEL_INCREMENT",
                short_name: "CHANNEL_INCREMENT",
                doc: "Channel Increment\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CHANNEL_DECREMENT",
                short_name: "CHANNEL_DECREMENT",
                doc: "Channel Decrement\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_PLAY",
                short_name: "MEDIA_PLAY",
                doc: "Play\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_PAUSE",
                short_name: "MEDIA_PAUSE",
                doc: "Pause\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_RECORD",
                short_name: "MEDIA_RECORD",
                doc: "Record\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_FAST_FORWARD",
                short_name: "MEDIA_FAST_FORWARD",
                doc: "Fast Forward\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_REWIND",
                short_name: "MEDIA_REWIND",
                doc: "Rewind\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_NEXT_TRACK",
                short_name: "MEDIA_NEXT_TRACK",
                doc: "Next Track\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_PREVIOUS_TRACK",
                short_name: "MEDIA_PREVIOUS_TRACK",
                doc: "Previous Track\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_STOP",
                short_name: "MEDIA_STOP",
                doc: "Stop\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_EJECT",
                short_name: "MEDIA_EJECT",
                doc: "Eject\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_PLAY_PAUSE",
                short_name: "MEDIA_PLAY_PAUSE",
                doc: "Play / Pause\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_MEDIA_SELECT",
                short_name: "MEDIA_SELECT",
                doc: "",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_NEW",
                short_name: "AC_NEW",
                doc: "AC New\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_OPEN",
                short_name: "AC_OPEN",
                doc: "AC Open\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_CLOSE",
                short_name: "AC_CLOSE",
                doc: "AC Close\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_EXIT",
                short_name: "AC_EXIT",
                doc: "AC Exit\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_SAVE",
                short_name: "AC_SAVE",
                doc: "AC Save\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_PRINT",
                short_name: "AC_PRINT",
                doc: "AC Print\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_PROPERTIES",
                short_name: "AC_PROPERTIES",
                doc: "AC Properties\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_SEARCH",
                short_name: "AC_SEARCH",
                doc: "AC Search\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_HOME",
                short_name: "AC_HOME",
                doc: "AC Home\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_BACK",
                short_name: "AC_BACK",
                doc: "AC Back\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_FORWARD",
                short_name: "AC_FORWARD",
                doc: "AC Forward\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_STOP",
                short_name: "AC_STOP",
                doc: "AC Stop\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_REFRESH",
                short_name: "AC_REFRESH",
                doc: "AC Refresh\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_AC_BOOKMARKS",
                short_name: "AC_BOOKMARKS",
                doc: "AC Bookmarks\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SOFTLEFT",
                short_name: "SOFTLEFT",
                doc: "Usually situated below the display on phones and\nused as a multi-function feature key for selecting\na software defined function shown on the bottom left\nof the display.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_SOFTRIGHT",
                short_name: "SOFTRIGHT",
                doc: "Usually situated below the display on phones and\nused as a multi-function feature key for selecting\na software defined function shown on the bottom right\nof the display.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_CALL",
                short_name: "CALL",
                doc: "Used for accepting phone calls.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_ENDCALL",
                short_name: "ENDCALL",
                doc: "Used for rejecting phone calls.\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_RESERVED",
                short_name: "RESERVED",
                doc: "400-500 reserved for dynamic keycodes\n",
            },
            sdl3_sys::metadata::GroupValue {
                name: "SDL_SCANCODE_COUNT",
                short_name: "COUNT",
                doc: "not a key, just marks the number of scancodes for array bounds\n",
            },
        ],
    };
}

#[cfg(doc)]
use crate::everything::*;
