#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph)]

//! # CategoryScancode
//!
//! Defines keyboard scancodes.

use super::stdinc::*;

/// The SDL keyboard scancode representation.
///
/// An SDL scancode is the physical representation of a key on the keyboard,
/// independent of language and keyboard mapping.
///
/// Values of this type are used to represent keyboard keys, among other places
/// in the `scancode` field of the SDL_KeyboardEvent structure.
///
/// The values in this enumeration are based on the USB usage page standard:
/// https://usb.org/sites/default/files/hut1_5.pdf
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_SCANCODE_UNKNOWN`], [`SDL_SCANCODE_A`], [`SDL_SCANCODE_B`], [`SDL_SCANCODE_C`], [`SDL_SCANCODE_D`], [`SDL_SCANCODE_E`], [`SDL_SCANCODE_F`], [`SDL_SCANCODE_G`], [`SDL_SCANCODE_H`], [`SDL_SCANCODE_I`], [`SDL_SCANCODE_J`], [`SDL_SCANCODE_K`], [`SDL_SCANCODE_L`], [`SDL_SCANCODE_M`], [`SDL_SCANCODE_N`], [`SDL_SCANCODE_O`], [`SDL_SCANCODE_P`], [`SDL_SCANCODE_Q`], [`SDL_SCANCODE_R`], [`SDL_SCANCODE_S`], [`SDL_SCANCODE_T`], [`SDL_SCANCODE_U`], [`SDL_SCANCODE_V`], [`SDL_SCANCODE_W`], [`SDL_SCANCODE_X`], [`SDL_SCANCODE_Y`], [`SDL_SCANCODE_Z`], [`SDL_SCANCODE_1`], [`SDL_SCANCODE_2`], [`SDL_SCANCODE_3`], [`SDL_SCANCODE_4`], [`SDL_SCANCODE_5`], [`SDL_SCANCODE_6`], [`SDL_SCANCODE_7`], [`SDL_SCANCODE_8`], [`SDL_SCANCODE_9`], [`SDL_SCANCODE_0`], [`SDL_SCANCODE_RETURN`], [`SDL_SCANCODE_ESCAPE`], [`SDL_SCANCODE_BACKSPACE`], [`SDL_SCANCODE_TAB`], [`SDL_SCANCODE_SPACE`], [`SDL_SCANCODE_MINUS`], [`SDL_SCANCODE_EQUALS`], [`SDL_SCANCODE_LEFTBRACKET`], [`SDL_SCANCODE_RIGHTBRACKET`], [`SDL_SCANCODE_BACKSLASH`], [`SDL_SCANCODE_NONUSHASH`], [`SDL_SCANCODE_SEMICOLON`], [`SDL_SCANCODE_APOSTROPHE`], [`SDL_SCANCODE_GRAVE`], [`SDL_SCANCODE_COMMA`], [`SDL_SCANCODE_PERIOD`], [`SDL_SCANCODE_SLASH`], [`SDL_SCANCODE_CAPSLOCK`], [`SDL_SCANCODE_F1`], [`SDL_SCANCODE_F2`], [`SDL_SCANCODE_F3`], [`SDL_SCANCODE_F4`], [`SDL_SCANCODE_F5`], [`SDL_SCANCODE_F6`], [`SDL_SCANCODE_F7`], [`SDL_SCANCODE_F8`], [`SDL_SCANCODE_F9`], [`SDL_SCANCODE_F10`], [`SDL_SCANCODE_F11`], [`SDL_SCANCODE_F12`], [`SDL_SCANCODE_PRINTSCREEN`], [`SDL_SCANCODE_SCROLLLOCK`], [`SDL_SCANCODE_PAUSE`], [`SDL_SCANCODE_INSERT`], [`SDL_SCANCODE_HOME`], [`SDL_SCANCODE_PAGEUP`], [`SDL_SCANCODE_DELETE`], [`SDL_SCANCODE_END`], [`SDL_SCANCODE_PAGEDOWN`], [`SDL_SCANCODE_RIGHT`], [`SDL_SCANCODE_LEFT`], [`SDL_SCANCODE_DOWN`], [`SDL_SCANCODE_UP`], [`SDL_SCANCODE_NUMLOCKCLEAR`], [`SDL_SCANCODE_KP_DIVIDE`], [`SDL_SCANCODE_KP_MULTIPLY`], [`SDL_SCANCODE_KP_MINUS`], [`SDL_SCANCODE_KP_PLUS`], [`SDL_SCANCODE_KP_ENTER`], [`SDL_SCANCODE_KP_1`], [`SDL_SCANCODE_KP_2`], [`SDL_SCANCODE_KP_3`], [`SDL_SCANCODE_KP_4`], [`SDL_SCANCODE_KP_5`], [`SDL_SCANCODE_KP_6`], [`SDL_SCANCODE_KP_7`], [`SDL_SCANCODE_KP_8`], [`SDL_SCANCODE_KP_9`], [`SDL_SCANCODE_KP_0`], [`SDL_SCANCODE_KP_PERIOD`], [`SDL_SCANCODE_NONUSBACKSLASH`], [`SDL_SCANCODE_APPLICATION`], [`SDL_SCANCODE_POWER`], [`SDL_SCANCODE_KP_EQUALS`], [`SDL_SCANCODE_F13`], [`SDL_SCANCODE_F14`], [`SDL_SCANCODE_F15`], [`SDL_SCANCODE_F16`], [`SDL_SCANCODE_F17`], [`SDL_SCANCODE_F18`], [`SDL_SCANCODE_F19`], [`SDL_SCANCODE_F20`], [`SDL_SCANCODE_F21`], [`SDL_SCANCODE_F22`], [`SDL_SCANCODE_F23`], [`SDL_SCANCODE_F24`], [`SDL_SCANCODE_EXECUTE`], [`SDL_SCANCODE_HELP`], [`SDL_SCANCODE_MENU`], [`SDL_SCANCODE_SELECT`], [`SDL_SCANCODE_STOP`], [`SDL_SCANCODE_AGAIN`], [`SDL_SCANCODE_UNDO`], [`SDL_SCANCODE_CUT`], [`SDL_SCANCODE_COPY`], [`SDL_SCANCODE_PASTE`], [`SDL_SCANCODE_FIND`], [`SDL_SCANCODE_MUTE`], [`SDL_SCANCODE_VOLUMEUP`], [`SDL_SCANCODE_VOLUMEDOWN`], [`SDL_SCANCODE_KP_COMMA`], [`SDL_SCANCODE_KP_EQUALSAS400`], [`SDL_SCANCODE_INTERNATIONAL1`], [`SDL_SCANCODE_INTERNATIONAL2`], [`SDL_SCANCODE_INTERNATIONAL3`], [`SDL_SCANCODE_INTERNATIONAL4`], [`SDL_SCANCODE_INTERNATIONAL5`], [`SDL_SCANCODE_INTERNATIONAL6`], [`SDL_SCANCODE_INTERNATIONAL7`], [`SDL_SCANCODE_INTERNATIONAL8`], [`SDL_SCANCODE_INTERNATIONAL9`], [`SDL_SCANCODE_LANG1`], [`SDL_SCANCODE_LANG2`], [`SDL_SCANCODE_LANG3`], [`SDL_SCANCODE_LANG4`], [`SDL_SCANCODE_LANG5`], [`SDL_SCANCODE_LANG6`], [`SDL_SCANCODE_LANG7`], [`SDL_SCANCODE_LANG8`], [`SDL_SCANCODE_LANG9`], [`SDL_SCANCODE_ALTERASE`], [`SDL_SCANCODE_SYSREQ`], [`SDL_SCANCODE_CANCEL`], [`SDL_SCANCODE_CLEAR`], [`SDL_SCANCODE_PRIOR`], [`SDL_SCANCODE_RETURN2`], [`SDL_SCANCODE_SEPARATOR`], [`SDL_SCANCODE_OUT`], [`SDL_SCANCODE_OPER`], [`SDL_SCANCODE_CLEARAGAIN`], [`SDL_SCANCODE_CRSEL`], [`SDL_SCANCODE_EXSEL`], [`SDL_SCANCODE_KP_00`], [`SDL_SCANCODE_KP_000`], [`SDL_SCANCODE_THOUSANDSSEPARATOR`], [`SDL_SCANCODE_DECIMALSEPARATOR`], [`SDL_SCANCODE_CURRENCYUNIT`], [`SDL_SCANCODE_CURRENCYSUBUNIT`], [`SDL_SCANCODE_KP_LEFTPAREN`], [`SDL_SCANCODE_KP_RIGHTPAREN`], [`SDL_SCANCODE_KP_LEFTBRACE`], [`SDL_SCANCODE_KP_RIGHTBRACE`], [`SDL_SCANCODE_KP_TAB`], [`SDL_SCANCODE_KP_BACKSPACE`], [`SDL_SCANCODE_KP_A`], [`SDL_SCANCODE_KP_B`], [`SDL_SCANCODE_KP_C`], [`SDL_SCANCODE_KP_D`], [`SDL_SCANCODE_KP_E`], [`SDL_SCANCODE_KP_F`], [`SDL_SCANCODE_KP_XOR`], [`SDL_SCANCODE_KP_POWER`], [`SDL_SCANCODE_KP_PERCENT`], [`SDL_SCANCODE_KP_LESS`], [`SDL_SCANCODE_KP_GREATER`], [`SDL_SCANCODE_KP_AMPERSAND`], [`SDL_SCANCODE_KP_DBLAMPERSAND`], [`SDL_SCANCODE_KP_VERTICALBAR`], [`SDL_SCANCODE_KP_DBLVERTICALBAR`], [`SDL_SCANCODE_KP_COLON`], [`SDL_SCANCODE_KP_HASH`], [`SDL_SCANCODE_KP_SPACE`], [`SDL_SCANCODE_KP_AT`], [`SDL_SCANCODE_KP_EXCLAM`], [`SDL_SCANCODE_KP_MEMSTORE`], [`SDL_SCANCODE_KP_MEMRECALL`], [`SDL_SCANCODE_KP_MEMCLEAR`], [`SDL_SCANCODE_KP_MEMADD`], [`SDL_SCANCODE_KP_MEMSUBTRACT`], [`SDL_SCANCODE_KP_MEMMULTIPLY`], [`SDL_SCANCODE_KP_MEMDIVIDE`], [`SDL_SCANCODE_KP_PLUSMINUS`], [`SDL_SCANCODE_KP_CLEAR`], [`SDL_SCANCODE_KP_CLEARENTRY`], [`SDL_SCANCODE_KP_BINARY`], [`SDL_SCANCODE_KP_OCTAL`], [`SDL_SCANCODE_KP_DECIMAL`], [`SDL_SCANCODE_KP_HEXADECIMAL`], [`SDL_SCANCODE_LCTRL`], [`SDL_SCANCODE_LSHIFT`], [`SDL_SCANCODE_LALT`], [`SDL_SCANCODE_LGUI`], [`SDL_SCANCODE_RCTRL`], [`SDL_SCANCODE_RSHIFT`], [`SDL_SCANCODE_RALT`], [`SDL_SCANCODE_RGUI`], [`SDL_SCANCODE_MODE`], [`SDL_SCANCODE_SLEEP`], [`SDL_SCANCODE_WAKE`], [`SDL_SCANCODE_CHANNEL_INCREMENT`], [`SDL_SCANCODE_CHANNEL_DECREMENT`], [`SDL_SCANCODE_MEDIA_PLAY`], [`SDL_SCANCODE_MEDIA_PAUSE`], [`SDL_SCANCODE_MEDIA_RECORD`], [`SDL_SCANCODE_MEDIA_FAST_FORWARD`], [`SDL_SCANCODE_MEDIA_REWIND`], [`SDL_SCANCODE_MEDIA_NEXT_TRACK`], [`SDL_SCANCODE_MEDIA_PREVIOUS_TRACK`], [`SDL_SCANCODE_MEDIA_STOP`], [`SDL_SCANCODE_MEDIA_EJECT`], [`SDL_SCANCODE_MEDIA_PLAY_PAUSE`], [`SDL_SCANCODE_MEDIA_SELECT`], [`SDL_SCANCODE_AC_NEW`], [`SDL_SCANCODE_AC_OPEN`], [`SDL_SCANCODE_AC_CLOSE`], [`SDL_SCANCODE_AC_EXIT`], [`SDL_SCANCODE_AC_SAVE`], [`SDL_SCANCODE_AC_PRINT`], [`SDL_SCANCODE_AC_PROPERTIES`], [`SDL_SCANCODE_AC_SEARCH`], [`SDL_SCANCODE_AC_HOME`], [`SDL_SCANCODE_AC_BACK`], [`SDL_SCANCODE_AC_FORWARD`], [`SDL_SCANCODE_AC_STOP`], [`SDL_SCANCODE_AC_REFRESH`], [`SDL_SCANCODE_AC_BOOKMARKS`], [`SDL_SCANCODE_SOFTLEFT`], [`SDL_SCANCODE_SOFTRIGHT`], [`SDL_SCANCODE_CALL`], [`SDL_SCANCODE_ENDCALL`], [`SDL_SCANCODE_RESERVED`], [`SDL_NUM_SCANCODES`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Scancode(pub ::core::ffi::c_int);
impl SDL_Scancode {
    pub const SCANCODE_UNKNOWN: Self = Self(0);
    pub const SCANCODE_A: Self = Self(4);
    pub const SCANCODE_B: Self = Self(5);
    pub const SCANCODE_C: Self = Self(6);
    pub const SCANCODE_D: Self = Self(7);
    pub const SCANCODE_E: Self = Self(8);
    pub const SCANCODE_F: Self = Self(9);
    pub const SCANCODE_G: Self = Self(10);
    pub const SCANCODE_H: Self = Self(11);
    pub const SCANCODE_I: Self = Self(12);
    pub const SCANCODE_J: Self = Self(13);
    pub const SCANCODE_K: Self = Self(14);
    pub const SCANCODE_L: Self = Self(15);
    pub const SCANCODE_M: Self = Self(16);
    pub const SCANCODE_N: Self = Self(17);
    pub const SCANCODE_O: Self = Self(18);
    pub const SCANCODE_P: Self = Self(19);
    pub const SCANCODE_Q: Self = Self(20);
    pub const SCANCODE_R: Self = Self(21);
    pub const SCANCODE_S: Self = Self(22);
    pub const SCANCODE_T: Self = Self(23);
    pub const SCANCODE_U: Self = Self(24);
    pub const SCANCODE_V: Self = Self(25);
    pub const SCANCODE_W: Self = Self(26);
    pub const SCANCODE_X: Self = Self(27);
    pub const SCANCODE_Y: Self = Self(28);
    pub const SCANCODE_Z: Self = Self(29);
    pub const SCANCODE_1: Self = Self(30);
    pub const SCANCODE_2: Self = Self(31);
    pub const SCANCODE_3: Self = Self(32);
    pub const SCANCODE_4: Self = Self(33);
    pub const SCANCODE_5: Self = Self(34);
    pub const SCANCODE_6: Self = Self(35);
    pub const SCANCODE_7: Self = Self(36);
    pub const SCANCODE_8: Self = Self(37);
    pub const SCANCODE_9: Self = Self(38);
    pub const SCANCODE_0: Self = Self(39);
    pub const SCANCODE_RETURN: Self = Self(40);
    pub const SCANCODE_ESCAPE: Self = Self(41);
    pub const SCANCODE_BACKSPACE: Self = Self(42);
    pub const SCANCODE_TAB: Self = Self(43);
    pub const SCANCODE_SPACE: Self = Self(44);
    pub const SCANCODE_MINUS: Self = Self(45);
    pub const SCANCODE_EQUALS: Self = Self(46);
    pub const SCANCODE_LEFTBRACKET: Self = Self(47);
    pub const SCANCODE_RIGHTBRACKET: Self = Self(48);
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
    pub const SCANCODE_BACKSLASH: Self = Self(49);
    /// ISO USB keyboards actually use this code
    /// instead of 49 for the same key, but all
    /// OSes I've seen treat the two codes
    /// identically. So, as an implementor, unless
    /// your keyboard generates both of those
    /// codes and your OS treats them differently,
    /// you should generate SDL_SCANCODE_BACKSLASH
    /// instead of this code. As a user, you
    /// should not rely on this code because SDL
    /// will never generate it with most (all?)
    /// keyboards.
    pub const SCANCODE_NONUSHASH: Self = Self(50);
    pub const SCANCODE_SEMICOLON: Self = Self(51);
    pub const SCANCODE_APOSTROPHE: Self = Self(52);
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
    pub const SCANCODE_GRAVE: Self = Self(53);
    pub const SCANCODE_COMMA: Self = Self(54);
    pub const SCANCODE_PERIOD: Self = Self(55);
    pub const SCANCODE_SLASH: Self = Self(56);
    pub const SCANCODE_CAPSLOCK: Self = Self(57);
    pub const SCANCODE_F1: Self = Self(58);
    pub const SCANCODE_F2: Self = Self(59);
    pub const SCANCODE_F3: Self = Self(60);
    pub const SCANCODE_F4: Self = Self(61);
    pub const SCANCODE_F5: Self = Self(62);
    pub const SCANCODE_F6: Self = Self(63);
    pub const SCANCODE_F7: Self = Self(64);
    pub const SCANCODE_F8: Self = Self(65);
    pub const SCANCODE_F9: Self = Self(66);
    pub const SCANCODE_F10: Self = Self(67);
    pub const SCANCODE_F11: Self = Self(68);
    pub const SCANCODE_F12: Self = Self(69);
    pub const SCANCODE_PRINTSCREEN: Self = Self(70);
    pub const SCANCODE_SCROLLLOCK: Self = Self(71);
    pub const SCANCODE_PAUSE: Self = Self(72);
    /// insert on PC, help on some Mac keyboards (but
    /// does send code 73, not 117)
    pub const SCANCODE_INSERT: Self = Self(73);
    pub const SCANCODE_HOME: Self = Self(74);
    pub const SCANCODE_PAGEUP: Self = Self(75);
    pub const SCANCODE_DELETE: Self = Self(76);
    pub const SCANCODE_END: Self = Self(77);
    pub const SCANCODE_PAGEDOWN: Self = Self(78);
    pub const SCANCODE_RIGHT: Self = Self(79);
    pub const SCANCODE_LEFT: Self = Self(80);
    pub const SCANCODE_DOWN: Self = Self(81);
    pub const SCANCODE_UP: Self = Self(82);
    /// num lock on PC, clear on Mac keyboards
    pub const SCANCODE_NUMLOCKCLEAR: Self = Self(83);
    pub const SCANCODE_KP_DIVIDE: Self = Self(84);
    pub const SCANCODE_KP_MULTIPLY: Self = Self(85);
    pub const SCANCODE_KP_MINUS: Self = Self(86);
    pub const SCANCODE_KP_PLUS: Self = Self(87);
    pub const SCANCODE_KP_ENTER: Self = Self(88);
    pub const SCANCODE_KP_1: Self = Self(89);
    pub const SCANCODE_KP_2: Self = Self(90);
    pub const SCANCODE_KP_3: Self = Self(91);
    pub const SCANCODE_KP_4: Self = Self(92);
    pub const SCANCODE_KP_5: Self = Self(93);
    pub const SCANCODE_KP_6: Self = Self(94);
    pub const SCANCODE_KP_7: Self = Self(95);
    pub const SCANCODE_KP_8: Self = Self(96);
    pub const SCANCODE_KP_9: Self = Self(97);
    pub const SCANCODE_KP_0: Self = Self(98);
    pub const SCANCODE_KP_PERIOD: Self = Self(99);
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
    pub const SCANCODE_NONUSBACKSLASH: Self = Self(100);
    /// windows contextual menu, compose
    pub const SCANCODE_APPLICATION: Self = Self(101);
    /// The USB document says this is a status flag,
    /// not a physical key - but some Mac keyboards
    /// do have a power key.
    pub const SCANCODE_POWER: Self = Self(102);
    pub const SCANCODE_KP_EQUALS: Self = Self(103);
    pub const SCANCODE_F13: Self = Self(104);
    pub const SCANCODE_F14: Self = Self(105);
    pub const SCANCODE_F15: Self = Self(106);
    pub const SCANCODE_F16: Self = Self(107);
    pub const SCANCODE_F17: Self = Self(108);
    pub const SCANCODE_F18: Self = Self(109);
    pub const SCANCODE_F19: Self = Self(110);
    pub const SCANCODE_F20: Self = Self(111);
    pub const SCANCODE_F21: Self = Self(112);
    pub const SCANCODE_F22: Self = Self(113);
    pub const SCANCODE_F23: Self = Self(114);
    pub const SCANCODE_F24: Self = Self(115);
    pub const SCANCODE_EXECUTE: Self = Self(116);
    /// AL Integrated Help Center
    pub const SCANCODE_HELP: Self = Self(117);
    /// Menu (show menu)
    pub const SCANCODE_MENU: Self = Self(118);
    pub const SCANCODE_SELECT: Self = Self(119);
    /// AC Stop
    pub const SCANCODE_STOP: Self = Self(120);
    /// AC Redo/Repeat
    pub const SCANCODE_AGAIN: Self = Self(121);
    /// AC Undo
    pub const SCANCODE_UNDO: Self = Self(122);
    /// AC Cut
    pub const SCANCODE_CUT: Self = Self(123);
    /// AC Copy
    pub const SCANCODE_COPY: Self = Self(124);
    /// AC Paste
    pub const SCANCODE_PASTE: Self = Self(125);
    /// AC Find
    pub const SCANCODE_FIND: Self = Self(126);
    pub const SCANCODE_MUTE: Self = Self(127);
    pub const SCANCODE_VOLUMEUP: Self = Self(128);
    pub const SCANCODE_VOLUMEDOWN: Self = Self(129);
    pub const SCANCODE_KP_COMMA: Self = Self(133);
    pub const SCANCODE_KP_EQUALSAS400: Self = Self(134);
    /// used on Asian keyboards, see
    /// footnotes in USB doc
    pub const SCANCODE_INTERNATIONAL1: Self = Self(135);
    pub const SCANCODE_INTERNATIONAL2: Self = Self(136);
    /// Yen
    pub const SCANCODE_INTERNATIONAL3: Self = Self(137);
    pub const SCANCODE_INTERNATIONAL4: Self = Self(138);
    pub const SCANCODE_INTERNATIONAL5: Self = Self(139);
    pub const SCANCODE_INTERNATIONAL6: Self = Self(140);
    pub const SCANCODE_INTERNATIONAL7: Self = Self(141);
    pub const SCANCODE_INTERNATIONAL8: Self = Self(142);
    pub const SCANCODE_INTERNATIONAL9: Self = Self(143);
    /// Hangul/English toggle
    pub const SCANCODE_LANG1: Self = Self(144);
    /// Hanja conversion
    pub const SCANCODE_LANG2: Self = Self(145);
    /// Katakana
    pub const SCANCODE_LANG3: Self = Self(146);
    /// Hiragana
    pub const SCANCODE_LANG4: Self = Self(147);
    /// Zenkaku/Hankaku
    pub const SCANCODE_LANG5: Self = Self(148);
    /// reserved
    pub const SCANCODE_LANG6: Self = Self(149);
    /// reserved
    pub const SCANCODE_LANG7: Self = Self(150);
    /// reserved
    pub const SCANCODE_LANG8: Self = Self(151);
    /// reserved
    pub const SCANCODE_LANG9: Self = Self(152);
    /// Erase-Eaze
    pub const SCANCODE_ALTERASE: Self = Self(153);
    pub const SCANCODE_SYSREQ: Self = Self(154);
    /// AC Cancel
    pub const SCANCODE_CANCEL: Self = Self(155);
    pub const SCANCODE_CLEAR: Self = Self(156);
    pub const SCANCODE_PRIOR: Self = Self(157);
    pub const SCANCODE_RETURN2: Self = Self(158);
    pub const SCANCODE_SEPARATOR: Self = Self(159);
    pub const SCANCODE_OUT: Self = Self(160);
    pub const SCANCODE_OPER: Self = Self(161);
    pub const SCANCODE_CLEARAGAIN: Self = Self(162);
    pub const SCANCODE_CRSEL: Self = Self(163);
    pub const SCANCODE_EXSEL: Self = Self(164);
    pub const SCANCODE_KP_00: Self = Self(176);
    pub const SCANCODE_KP_000: Self = Self(177);
    pub const SCANCODE_THOUSANDSSEPARATOR: Self = Self(178);
    pub const SCANCODE_DECIMALSEPARATOR: Self = Self(179);
    pub const SCANCODE_CURRENCYUNIT: Self = Self(180);
    pub const SCANCODE_CURRENCYSUBUNIT: Self = Self(181);
    pub const SCANCODE_KP_LEFTPAREN: Self = Self(182);
    pub const SCANCODE_KP_RIGHTPAREN: Self = Self(183);
    pub const SCANCODE_KP_LEFTBRACE: Self = Self(184);
    pub const SCANCODE_KP_RIGHTBRACE: Self = Self(185);
    pub const SCANCODE_KP_TAB: Self = Self(186);
    pub const SCANCODE_KP_BACKSPACE: Self = Self(187);
    pub const SCANCODE_KP_A: Self = Self(188);
    pub const SCANCODE_KP_B: Self = Self(189);
    pub const SCANCODE_KP_C: Self = Self(190);
    pub const SCANCODE_KP_D: Self = Self(191);
    pub const SCANCODE_KP_E: Self = Self(192);
    pub const SCANCODE_KP_F: Self = Self(193);
    pub const SCANCODE_KP_XOR: Self = Self(194);
    pub const SCANCODE_KP_POWER: Self = Self(195);
    pub const SCANCODE_KP_PERCENT: Self = Self(196);
    pub const SCANCODE_KP_LESS: Self = Self(197);
    pub const SCANCODE_KP_GREATER: Self = Self(198);
    pub const SCANCODE_KP_AMPERSAND: Self = Self(199);
    pub const SCANCODE_KP_DBLAMPERSAND: Self = Self(200);
    pub const SCANCODE_KP_VERTICALBAR: Self = Self(201);
    pub const SCANCODE_KP_DBLVERTICALBAR: Self = Self(202);
    pub const SCANCODE_KP_COLON: Self = Self(203);
    pub const SCANCODE_KP_HASH: Self = Self(204);
    pub const SCANCODE_KP_SPACE: Self = Self(205);
    pub const SCANCODE_KP_AT: Self = Self(206);
    pub const SCANCODE_KP_EXCLAM: Self = Self(207);
    pub const SCANCODE_KP_MEMSTORE: Self = Self(208);
    pub const SCANCODE_KP_MEMRECALL: Self = Self(209);
    pub const SCANCODE_KP_MEMCLEAR: Self = Self(210);
    pub const SCANCODE_KP_MEMADD: Self = Self(211);
    pub const SCANCODE_KP_MEMSUBTRACT: Self = Self(212);
    pub const SCANCODE_KP_MEMMULTIPLY: Self = Self(213);
    pub const SCANCODE_KP_MEMDIVIDE: Self = Self(214);
    pub const SCANCODE_KP_PLUSMINUS: Self = Self(215);
    pub const SCANCODE_KP_CLEAR: Self = Self(216);
    pub const SCANCODE_KP_CLEARENTRY: Self = Self(217);
    pub const SCANCODE_KP_BINARY: Self = Self(218);
    pub const SCANCODE_KP_OCTAL: Self = Self(219);
    pub const SCANCODE_KP_DECIMAL: Self = Self(220);
    pub const SCANCODE_KP_HEXADECIMAL: Self = Self(221);
    pub const SCANCODE_LCTRL: Self = Self(224);
    pub const SCANCODE_LSHIFT: Self = Self(225);
    /// alt, option
    pub const SCANCODE_LALT: Self = Self(226);
    /// windows, command (apple), meta
    pub const SCANCODE_LGUI: Self = Self(227);
    pub const SCANCODE_RCTRL: Self = Self(228);
    pub const SCANCODE_RSHIFT: Self = Self(229);
    /// alt gr, option
    pub const SCANCODE_RALT: Self = Self(230);
    /// windows, command (apple), meta
    pub const SCANCODE_RGUI: Self = Self(231);
    /// I'm not sure if this is really not covered
    /// by any of the above, but since there's a
    /// special SDL_KMOD_MODE for it I'm adding it here
    pub const SCANCODE_MODE: Self = Self(257);
    /// Sleep
    pub const SCANCODE_SLEEP: Self = Self(258);
    /// Wake
    pub const SCANCODE_WAKE: Self = Self(259);
    /// Channel Increment
    pub const SCANCODE_CHANNEL_INCREMENT: Self = Self(260);
    /// Channel Decrement
    pub const SCANCODE_CHANNEL_DECREMENT: Self = Self(261);
    /// Play
    pub const SCANCODE_MEDIA_PLAY: Self = Self(262);
    /// Pause
    pub const SCANCODE_MEDIA_PAUSE: Self = Self(263);
    /// Record
    pub const SCANCODE_MEDIA_RECORD: Self = Self(264);
    /// Fast Forward
    pub const SCANCODE_MEDIA_FAST_FORWARD: Self = Self(265);
    /// Rewind
    pub const SCANCODE_MEDIA_REWIND: Self = Self(266);
    /// Next Track
    pub const SCANCODE_MEDIA_NEXT_TRACK: Self = Self(267);
    /// Previous Track
    pub const SCANCODE_MEDIA_PREVIOUS_TRACK: Self = Self(268);
    /// Stop
    pub const SCANCODE_MEDIA_STOP: Self = Self(269);
    /// Eject
    pub const SCANCODE_MEDIA_EJECT: Self = Self(270);
    /// Play / Pause
    pub const SCANCODE_MEDIA_PLAY_PAUSE: Self = Self(271);
    pub const SCANCODE_MEDIA_SELECT: Self = Self(272);
    /// AC New
    pub const SCANCODE_AC_NEW: Self = Self(273);
    /// AC Open
    pub const SCANCODE_AC_OPEN: Self = Self(274);
    /// AC Close
    pub const SCANCODE_AC_CLOSE: Self = Self(275);
    /// AC Exit
    pub const SCANCODE_AC_EXIT: Self = Self(276);
    /// AC Save
    pub const SCANCODE_AC_SAVE: Self = Self(277);
    /// AC Print
    pub const SCANCODE_AC_PRINT: Self = Self(278);
    /// AC Properties
    pub const SCANCODE_AC_PROPERTIES: Self = Self(279);
    /// AC Search
    pub const SCANCODE_AC_SEARCH: Self = Self(280);
    /// AC Home
    pub const SCANCODE_AC_HOME: Self = Self(281);
    /// AC Back
    pub const SCANCODE_AC_BACK: Self = Self(282);
    /// AC Forward
    pub const SCANCODE_AC_FORWARD: Self = Self(283);
    /// AC Stop
    pub const SCANCODE_AC_STOP: Self = Self(284);
    /// AC Refresh
    pub const SCANCODE_AC_REFRESH: Self = Self(285);
    /// AC Bookmarks
    pub const SCANCODE_AC_BOOKMARKS: Self = Self(286);
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom left
    /// of the display.
    pub const SCANCODE_SOFTLEFT: Self = Self(287);
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom right
    /// of the display.
    pub const SCANCODE_SOFTRIGHT: Self = Self(288);
    /// Used for accepting phone calls.
    pub const SCANCODE_CALL: Self = Self(289);
    /// Used for rejecting phone calls.
    pub const SCANCODE_ENDCALL: Self = Self(290);
    /// 400-500 reserved for dynamic keycodes
    pub const SCANCODE_RESERVED: Self = Self(400);
    /// not a key, just marks the number of scancodes
    /// for array bounds
    pub const NUM_SCANCODES: Self = Self(512);
}
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = SDL_Scancode::SCANCODE_UNKNOWN;
pub const SDL_SCANCODE_A: SDL_Scancode = SDL_Scancode::SCANCODE_A;
pub const SDL_SCANCODE_B: SDL_Scancode = SDL_Scancode::SCANCODE_B;
pub const SDL_SCANCODE_C: SDL_Scancode = SDL_Scancode::SCANCODE_C;
pub const SDL_SCANCODE_D: SDL_Scancode = SDL_Scancode::SCANCODE_D;
pub const SDL_SCANCODE_E: SDL_Scancode = SDL_Scancode::SCANCODE_E;
pub const SDL_SCANCODE_F: SDL_Scancode = SDL_Scancode::SCANCODE_F;
pub const SDL_SCANCODE_G: SDL_Scancode = SDL_Scancode::SCANCODE_G;
pub const SDL_SCANCODE_H: SDL_Scancode = SDL_Scancode::SCANCODE_H;
pub const SDL_SCANCODE_I: SDL_Scancode = SDL_Scancode::SCANCODE_I;
pub const SDL_SCANCODE_J: SDL_Scancode = SDL_Scancode::SCANCODE_J;
pub const SDL_SCANCODE_K: SDL_Scancode = SDL_Scancode::SCANCODE_K;
pub const SDL_SCANCODE_L: SDL_Scancode = SDL_Scancode::SCANCODE_L;
pub const SDL_SCANCODE_M: SDL_Scancode = SDL_Scancode::SCANCODE_M;
pub const SDL_SCANCODE_N: SDL_Scancode = SDL_Scancode::SCANCODE_N;
pub const SDL_SCANCODE_O: SDL_Scancode = SDL_Scancode::SCANCODE_O;
pub const SDL_SCANCODE_P: SDL_Scancode = SDL_Scancode::SCANCODE_P;
pub const SDL_SCANCODE_Q: SDL_Scancode = SDL_Scancode::SCANCODE_Q;
pub const SDL_SCANCODE_R: SDL_Scancode = SDL_Scancode::SCANCODE_R;
pub const SDL_SCANCODE_S: SDL_Scancode = SDL_Scancode::SCANCODE_S;
pub const SDL_SCANCODE_T: SDL_Scancode = SDL_Scancode::SCANCODE_T;
pub const SDL_SCANCODE_U: SDL_Scancode = SDL_Scancode::SCANCODE_U;
pub const SDL_SCANCODE_V: SDL_Scancode = SDL_Scancode::SCANCODE_V;
pub const SDL_SCANCODE_W: SDL_Scancode = SDL_Scancode::SCANCODE_W;
pub const SDL_SCANCODE_X: SDL_Scancode = SDL_Scancode::SCANCODE_X;
pub const SDL_SCANCODE_Y: SDL_Scancode = SDL_Scancode::SCANCODE_Y;
pub const SDL_SCANCODE_Z: SDL_Scancode = SDL_Scancode::SCANCODE_Z;
pub const SDL_SCANCODE_1: SDL_Scancode = SDL_Scancode::SCANCODE_1;
pub const SDL_SCANCODE_2: SDL_Scancode = SDL_Scancode::SCANCODE_2;
pub const SDL_SCANCODE_3: SDL_Scancode = SDL_Scancode::SCANCODE_3;
pub const SDL_SCANCODE_4: SDL_Scancode = SDL_Scancode::SCANCODE_4;
pub const SDL_SCANCODE_5: SDL_Scancode = SDL_Scancode::SCANCODE_5;
pub const SDL_SCANCODE_6: SDL_Scancode = SDL_Scancode::SCANCODE_6;
pub const SDL_SCANCODE_7: SDL_Scancode = SDL_Scancode::SCANCODE_7;
pub const SDL_SCANCODE_8: SDL_Scancode = SDL_Scancode::SCANCODE_8;
pub const SDL_SCANCODE_9: SDL_Scancode = SDL_Scancode::SCANCODE_9;
pub const SDL_SCANCODE_0: SDL_Scancode = SDL_Scancode::SCANCODE_0;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = SDL_Scancode::SCANCODE_RETURN;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = SDL_Scancode::SCANCODE_ESCAPE;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = SDL_Scancode::SCANCODE_BACKSPACE;
pub const SDL_SCANCODE_TAB: SDL_Scancode = SDL_Scancode::SCANCODE_TAB;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = SDL_Scancode::SCANCODE_SPACE;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = SDL_Scancode::SCANCODE_MINUS;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = SDL_Scancode::SCANCODE_EQUALS;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = SDL_Scancode::SCANCODE_LEFTBRACKET;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = SDL_Scancode::SCANCODE_RIGHTBRACKET;
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
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = SDL_Scancode::SCANCODE_BACKSLASH;
/// ISO USB keyboards actually use this code
/// instead of 49 for the same key, but all
/// OSes I've seen treat the two codes
/// identically. So, as an implementor, unless
/// your keyboard generates both of those
/// codes and your OS treats them differently,
/// you should generate SDL_SCANCODE_BACKSLASH
/// instead of this code. As a user, you
/// should not rely on this code because SDL
/// will never generate it with most (all?)
/// keyboards.
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = SDL_Scancode::SCANCODE_NONUSHASH;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = SDL_Scancode::SCANCODE_SEMICOLON;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = SDL_Scancode::SCANCODE_APOSTROPHE;
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
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = SDL_Scancode::SCANCODE_GRAVE;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = SDL_Scancode::SCANCODE_COMMA;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = SDL_Scancode::SCANCODE_PERIOD;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = SDL_Scancode::SCANCODE_SLASH;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = SDL_Scancode::SCANCODE_CAPSLOCK;
pub const SDL_SCANCODE_F1: SDL_Scancode = SDL_Scancode::SCANCODE_F1;
pub const SDL_SCANCODE_F2: SDL_Scancode = SDL_Scancode::SCANCODE_F2;
pub const SDL_SCANCODE_F3: SDL_Scancode = SDL_Scancode::SCANCODE_F3;
pub const SDL_SCANCODE_F4: SDL_Scancode = SDL_Scancode::SCANCODE_F4;
pub const SDL_SCANCODE_F5: SDL_Scancode = SDL_Scancode::SCANCODE_F5;
pub const SDL_SCANCODE_F6: SDL_Scancode = SDL_Scancode::SCANCODE_F6;
pub const SDL_SCANCODE_F7: SDL_Scancode = SDL_Scancode::SCANCODE_F7;
pub const SDL_SCANCODE_F8: SDL_Scancode = SDL_Scancode::SCANCODE_F8;
pub const SDL_SCANCODE_F9: SDL_Scancode = SDL_Scancode::SCANCODE_F9;
pub const SDL_SCANCODE_F10: SDL_Scancode = SDL_Scancode::SCANCODE_F10;
pub const SDL_SCANCODE_F11: SDL_Scancode = SDL_Scancode::SCANCODE_F11;
pub const SDL_SCANCODE_F12: SDL_Scancode = SDL_Scancode::SCANCODE_F12;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = SDL_Scancode::SCANCODE_PRINTSCREEN;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = SDL_Scancode::SCANCODE_SCROLLLOCK;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = SDL_Scancode::SCANCODE_PAUSE;
/// insert on PC, help on some Mac keyboards (but
/// does send code 73, not 117)
pub const SDL_SCANCODE_INSERT: SDL_Scancode = SDL_Scancode::SCANCODE_INSERT;
pub const SDL_SCANCODE_HOME: SDL_Scancode = SDL_Scancode::SCANCODE_HOME;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = SDL_Scancode::SCANCODE_PAGEUP;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = SDL_Scancode::SCANCODE_DELETE;
pub const SDL_SCANCODE_END: SDL_Scancode = SDL_Scancode::SCANCODE_END;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = SDL_Scancode::SCANCODE_PAGEDOWN;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = SDL_Scancode::SCANCODE_RIGHT;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = SDL_Scancode::SCANCODE_LEFT;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = SDL_Scancode::SCANCODE_DOWN;
pub const SDL_SCANCODE_UP: SDL_Scancode = SDL_Scancode::SCANCODE_UP;
/// num lock on PC, clear on Mac keyboards
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = SDL_Scancode::SCANCODE_NUMLOCKCLEAR;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_DIVIDE;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MULTIPLY;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MINUS;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = SDL_Scancode::SCANCODE_KP_PLUS;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = SDL_Scancode::SCANCODE_KP_ENTER;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = SDL_Scancode::SCANCODE_KP_1;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = SDL_Scancode::SCANCODE_KP_2;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = SDL_Scancode::SCANCODE_KP_3;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = SDL_Scancode::SCANCODE_KP_4;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = SDL_Scancode::SCANCODE_KP_5;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = SDL_Scancode::SCANCODE_KP_6;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = SDL_Scancode::SCANCODE_KP_7;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = SDL_Scancode::SCANCODE_KP_8;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = SDL_Scancode::SCANCODE_KP_9;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = SDL_Scancode::SCANCODE_KP_0;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = SDL_Scancode::SCANCODE_KP_PERIOD;
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
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = SDL_Scancode::SCANCODE_NONUSBACKSLASH;
/// windows contextual menu, compose
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = SDL_Scancode::SCANCODE_APPLICATION;
/// The USB document says this is a status flag,
/// not a physical key - but some Mac keyboards
/// do have a power key.
pub const SDL_SCANCODE_POWER: SDL_Scancode = SDL_Scancode::SCANCODE_POWER;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = SDL_Scancode::SCANCODE_KP_EQUALS;
pub const SDL_SCANCODE_F13: SDL_Scancode = SDL_Scancode::SCANCODE_F13;
pub const SDL_SCANCODE_F14: SDL_Scancode = SDL_Scancode::SCANCODE_F14;
pub const SDL_SCANCODE_F15: SDL_Scancode = SDL_Scancode::SCANCODE_F15;
pub const SDL_SCANCODE_F16: SDL_Scancode = SDL_Scancode::SCANCODE_F16;
pub const SDL_SCANCODE_F17: SDL_Scancode = SDL_Scancode::SCANCODE_F17;
pub const SDL_SCANCODE_F18: SDL_Scancode = SDL_Scancode::SCANCODE_F18;
pub const SDL_SCANCODE_F19: SDL_Scancode = SDL_Scancode::SCANCODE_F19;
pub const SDL_SCANCODE_F20: SDL_Scancode = SDL_Scancode::SCANCODE_F20;
pub const SDL_SCANCODE_F21: SDL_Scancode = SDL_Scancode::SCANCODE_F21;
pub const SDL_SCANCODE_F22: SDL_Scancode = SDL_Scancode::SCANCODE_F22;
pub const SDL_SCANCODE_F23: SDL_Scancode = SDL_Scancode::SCANCODE_F23;
pub const SDL_SCANCODE_F24: SDL_Scancode = SDL_Scancode::SCANCODE_F24;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = SDL_Scancode::SCANCODE_EXECUTE;
/// AL Integrated Help Center
pub const SDL_SCANCODE_HELP: SDL_Scancode = SDL_Scancode::SCANCODE_HELP;
/// Menu (show menu)
pub const SDL_SCANCODE_MENU: SDL_Scancode = SDL_Scancode::SCANCODE_MENU;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = SDL_Scancode::SCANCODE_SELECT;
/// AC Stop
pub const SDL_SCANCODE_STOP: SDL_Scancode = SDL_Scancode::SCANCODE_STOP;
/// AC Redo/Repeat
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = SDL_Scancode::SCANCODE_AGAIN;
/// AC Undo
pub const SDL_SCANCODE_UNDO: SDL_Scancode = SDL_Scancode::SCANCODE_UNDO;
/// AC Cut
pub const SDL_SCANCODE_CUT: SDL_Scancode = SDL_Scancode::SCANCODE_CUT;
/// AC Copy
pub const SDL_SCANCODE_COPY: SDL_Scancode = SDL_Scancode::SCANCODE_COPY;
/// AC Paste
pub const SDL_SCANCODE_PASTE: SDL_Scancode = SDL_Scancode::SCANCODE_PASTE;
/// AC Find
pub const SDL_SCANCODE_FIND: SDL_Scancode = SDL_Scancode::SCANCODE_FIND;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = SDL_Scancode::SCANCODE_MUTE;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = SDL_Scancode::SCANCODE_VOLUMEUP;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = SDL_Scancode::SCANCODE_VOLUMEDOWN;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = SDL_Scancode::SCANCODE_KP_COMMA;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = SDL_Scancode::SCANCODE_KP_EQUALSAS400;
/// used on Asian keyboards, see
/// footnotes in USB doc
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL1;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL2;
/// Yen
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL3;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL4;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL5;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL6;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL7;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL8;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = SDL_Scancode::SCANCODE_INTERNATIONAL9;
/// Hangul/English toggle
pub const SDL_SCANCODE_LANG1: SDL_Scancode = SDL_Scancode::SCANCODE_LANG1;
/// Hanja conversion
pub const SDL_SCANCODE_LANG2: SDL_Scancode = SDL_Scancode::SCANCODE_LANG2;
/// Katakana
pub const SDL_SCANCODE_LANG3: SDL_Scancode = SDL_Scancode::SCANCODE_LANG3;
/// Hiragana
pub const SDL_SCANCODE_LANG4: SDL_Scancode = SDL_Scancode::SCANCODE_LANG4;
/// Zenkaku/Hankaku
pub const SDL_SCANCODE_LANG5: SDL_Scancode = SDL_Scancode::SCANCODE_LANG5;
/// reserved
pub const SDL_SCANCODE_LANG6: SDL_Scancode = SDL_Scancode::SCANCODE_LANG6;
/// reserved
pub const SDL_SCANCODE_LANG7: SDL_Scancode = SDL_Scancode::SCANCODE_LANG7;
/// reserved
pub const SDL_SCANCODE_LANG8: SDL_Scancode = SDL_Scancode::SCANCODE_LANG8;
/// reserved
pub const SDL_SCANCODE_LANG9: SDL_Scancode = SDL_Scancode::SCANCODE_LANG9;
/// Erase-Eaze
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = SDL_Scancode::SCANCODE_ALTERASE;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = SDL_Scancode::SCANCODE_SYSREQ;
/// AC Cancel
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = SDL_Scancode::SCANCODE_CANCEL;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = SDL_Scancode::SCANCODE_CLEAR;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = SDL_Scancode::SCANCODE_PRIOR;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = SDL_Scancode::SCANCODE_RETURN2;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = SDL_Scancode::SCANCODE_SEPARATOR;
pub const SDL_SCANCODE_OUT: SDL_Scancode = SDL_Scancode::SCANCODE_OUT;
pub const SDL_SCANCODE_OPER: SDL_Scancode = SDL_Scancode::SCANCODE_OPER;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = SDL_Scancode::SCANCODE_CLEARAGAIN;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = SDL_Scancode::SCANCODE_CRSEL;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = SDL_Scancode::SCANCODE_EXSEL;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = SDL_Scancode::SCANCODE_KP_00;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = SDL_Scancode::SCANCODE_KP_000;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = SDL_Scancode::SCANCODE_THOUSANDSSEPARATOR;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = SDL_Scancode::SCANCODE_DECIMALSEPARATOR;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = SDL_Scancode::SCANCODE_CURRENCYUNIT;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = SDL_Scancode::SCANCODE_CURRENCYSUBUNIT;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = SDL_Scancode::SCANCODE_KP_LEFTPAREN;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = SDL_Scancode::SCANCODE_KP_RIGHTPAREN;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_LEFTBRACE;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_RIGHTBRACE;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = SDL_Scancode::SCANCODE_KP_TAB;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_BACKSPACE;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = SDL_Scancode::SCANCODE_KP_A;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = SDL_Scancode::SCANCODE_KP_B;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = SDL_Scancode::SCANCODE_KP_C;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = SDL_Scancode::SCANCODE_KP_D;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = SDL_Scancode::SCANCODE_KP_E;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = SDL_Scancode::SCANCODE_KP_F;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = SDL_Scancode::SCANCODE_KP_XOR;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = SDL_Scancode::SCANCODE_KP_POWER;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = SDL_Scancode::SCANCODE_KP_PERCENT;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = SDL_Scancode::SCANCODE_KP_LESS;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = SDL_Scancode::SCANCODE_KP_GREATER;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = SDL_Scancode::SCANCODE_KP_AMPERSAND;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = SDL_Scancode::SCANCODE_KP_DBLAMPERSAND;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = SDL_Scancode::SCANCODE_KP_VERTICALBAR;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = SDL_Scancode::SCANCODE_KP_DBLVERTICALBAR;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = SDL_Scancode::SCANCODE_KP_COLON;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = SDL_Scancode::SCANCODE_KP_HASH;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_SPACE;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = SDL_Scancode::SCANCODE_KP_AT;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = SDL_Scancode::SCANCODE_KP_EXCLAM;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMSTORE;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMRECALL;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMCLEAR;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMADD;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMSUBTRACT;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMMULTIPLY;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = SDL_Scancode::SCANCODE_KP_MEMDIVIDE;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = SDL_Scancode::SCANCODE_KP_PLUSMINUS;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = SDL_Scancode::SCANCODE_KP_CLEAR;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = SDL_Scancode::SCANCODE_KP_CLEARENTRY;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = SDL_Scancode::SCANCODE_KP_BINARY;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = SDL_Scancode::SCANCODE_KP_OCTAL;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = SDL_Scancode::SCANCODE_KP_DECIMAL;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = SDL_Scancode::SCANCODE_KP_HEXADECIMAL;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = SDL_Scancode::SCANCODE_LCTRL;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = SDL_Scancode::SCANCODE_LSHIFT;
/// alt, option
pub const SDL_SCANCODE_LALT: SDL_Scancode = SDL_Scancode::SCANCODE_LALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_LGUI: SDL_Scancode = SDL_Scancode::SCANCODE_LGUI;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = SDL_Scancode::SCANCODE_RCTRL;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = SDL_Scancode::SCANCODE_RSHIFT;
/// alt gr, option
pub const SDL_SCANCODE_RALT: SDL_Scancode = SDL_Scancode::SCANCODE_RALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_RGUI: SDL_Scancode = SDL_Scancode::SCANCODE_RGUI;
/// I'm not sure if this is really not covered
/// by any of the above, but since there's a
/// special SDL_KMOD_MODE for it I'm adding it here
pub const SDL_SCANCODE_MODE: SDL_Scancode = SDL_Scancode::SCANCODE_MODE;
/// Sleep
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = SDL_Scancode::SCANCODE_SLEEP;
/// Wake
pub const SDL_SCANCODE_WAKE: SDL_Scancode = SDL_Scancode::SCANCODE_WAKE;
/// Channel Increment
pub const SDL_SCANCODE_CHANNEL_INCREMENT: SDL_Scancode = SDL_Scancode::SCANCODE_CHANNEL_INCREMENT;
/// Channel Decrement
pub const SDL_SCANCODE_CHANNEL_DECREMENT: SDL_Scancode = SDL_Scancode::SCANCODE_CHANNEL_DECREMENT;
/// Play
pub const SDL_SCANCODE_MEDIA_PLAY: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_PLAY;
/// Pause
pub const SDL_SCANCODE_MEDIA_PAUSE: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_PAUSE;
/// Record
pub const SDL_SCANCODE_MEDIA_RECORD: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_RECORD;
/// Fast Forward
pub const SDL_SCANCODE_MEDIA_FAST_FORWARD: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_FAST_FORWARD;
/// Rewind
pub const SDL_SCANCODE_MEDIA_REWIND: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_REWIND;
/// Next Track
pub const SDL_SCANCODE_MEDIA_NEXT_TRACK: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_NEXT_TRACK;
/// Previous Track
pub const SDL_SCANCODE_MEDIA_PREVIOUS_TRACK: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_PREVIOUS_TRACK;
/// Stop
pub const SDL_SCANCODE_MEDIA_STOP: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_STOP;
/// Eject
pub const SDL_SCANCODE_MEDIA_EJECT: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_EJECT;
/// Play / Pause
pub const SDL_SCANCODE_MEDIA_PLAY_PAUSE: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_PLAY_PAUSE;
pub const SDL_SCANCODE_MEDIA_SELECT: SDL_Scancode = SDL_Scancode::SCANCODE_MEDIA_SELECT;
/// AC New
pub const SDL_SCANCODE_AC_NEW: SDL_Scancode = SDL_Scancode::SCANCODE_AC_NEW;
/// AC Open
pub const SDL_SCANCODE_AC_OPEN: SDL_Scancode = SDL_Scancode::SCANCODE_AC_OPEN;
/// AC Close
pub const SDL_SCANCODE_AC_CLOSE: SDL_Scancode = SDL_Scancode::SCANCODE_AC_CLOSE;
/// AC Exit
pub const SDL_SCANCODE_AC_EXIT: SDL_Scancode = SDL_Scancode::SCANCODE_AC_EXIT;
/// AC Save
pub const SDL_SCANCODE_AC_SAVE: SDL_Scancode = SDL_Scancode::SCANCODE_AC_SAVE;
/// AC Print
pub const SDL_SCANCODE_AC_PRINT: SDL_Scancode = SDL_Scancode::SCANCODE_AC_PRINT;
/// AC Properties
pub const SDL_SCANCODE_AC_PROPERTIES: SDL_Scancode = SDL_Scancode::SCANCODE_AC_PROPERTIES;
/// AC Search
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = SDL_Scancode::SCANCODE_AC_SEARCH;
/// AC Home
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = SDL_Scancode::SCANCODE_AC_HOME;
/// AC Back
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = SDL_Scancode::SCANCODE_AC_BACK;
/// AC Forward
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = SDL_Scancode::SCANCODE_AC_FORWARD;
/// AC Stop
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = SDL_Scancode::SCANCODE_AC_STOP;
/// AC Refresh
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = SDL_Scancode::SCANCODE_AC_REFRESH;
/// AC Bookmarks
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = SDL_Scancode::SCANCODE_AC_BOOKMARKS;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom left
/// of the display.
pub const SDL_SCANCODE_SOFTLEFT: SDL_Scancode = SDL_Scancode::SCANCODE_SOFTLEFT;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom right
/// of the display.
pub const SDL_SCANCODE_SOFTRIGHT: SDL_Scancode = SDL_Scancode::SCANCODE_SOFTRIGHT;
/// Used for accepting phone calls.
pub const SDL_SCANCODE_CALL: SDL_Scancode = SDL_Scancode::SCANCODE_CALL;
/// Used for rejecting phone calls.
pub const SDL_SCANCODE_ENDCALL: SDL_Scancode = SDL_Scancode::SCANCODE_ENDCALL;
/// 400-500 reserved for dynamic keycodes
pub const SDL_SCANCODE_RESERVED: SDL_Scancode = SDL_Scancode::SCANCODE_RESERVED;
/// not a key, just marks the number of scancodes
/// for array bounds
pub const SDL_NUM_SCANCODES: SDL_Scancode = SDL_Scancode::NUM_SCANCODES;

