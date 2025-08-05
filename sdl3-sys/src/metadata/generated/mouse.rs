//! Metadata for items in the `crate::mouse` module

use super::*;

pub const METADATA_SDL_MouseID: Group = Group {
    module: "mouse",
    kind: GroupKind::Id,
    name: "SDL_MouseID",
    short_name: "MouseID",
    doc: Some(
        "This is a unique ID for a mouse for the time it is connected to the system,\nand is never reused for the lifetime of the application.\n\nIf the mouse is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[],
};
pub const METADATA_SDL_SystemCursor: Group = Group {
    module: "mouse",
    kind: GroupKind::Enum,
    name: "SDL_SystemCursor",
    short_name: "SystemCursor",
    doc: Some(
        "Cursor types for [`SDL_CreateSystemCursor()`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_DEFAULT",
            short_name: "DEFAULT",
            doc: Some("Default cursor. Usually an arrow.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_TEXT",
            short_name: "TEXT",
            doc: Some("Text selection. Usually an I-beam.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_WAIT",
            short_name: "WAIT",
            doc: Some("Wait. Usually an hourglass or watch or spinning ball.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_CROSSHAIR",
            short_name: "CROSSHAIR",
            doc: Some("Crosshair.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_PROGRESS",
            short_name: "PROGRESS",
            doc: Some("Program is busy but still interactive. Usually it's WAIT with an arrow.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NWSE_RESIZE",
            short_name: "NWSE_RESIZE",
            doc: Some("Double arrow pointing northwest and southeast.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NESW_RESIZE",
            short_name: "NESW_RESIZE",
            doc: Some("Double arrow pointing northeast and southwest.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_EW_RESIZE",
            short_name: "EW_RESIZE",
            doc: Some("Double arrow pointing west and east.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NS_RESIZE",
            short_name: "NS_RESIZE",
            doc: Some("Double arrow pointing north and south.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_MOVE",
            short_name: "MOVE",
            doc: Some("Four pointed arrow pointing north, south, east, and west.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NOT_ALLOWED",
            short_name: "NOT_ALLOWED",
            doc: Some("Not permitted. Usually a slashed circle or crossbones.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_POINTER",
            short_name: "POINTER",
            doc: Some("Pointer that indicates a link. Usually a pointing hand.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NW_RESIZE",
            short_name: "NW_RESIZE",
            doc: Some(
                "Window resize top-left. This may be a single arrow or a double arrow like NWSE_RESIZE.\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_N_RESIZE",
            short_name: "N_RESIZE",
            doc: Some("Window resize top. May be NS_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_NE_RESIZE",
            short_name: "NE_RESIZE",
            doc: Some("Window resize top-right. May be NESW_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_E_RESIZE",
            short_name: "E_RESIZE",
            doc: Some("Window resize right. May be EW_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_SE_RESIZE",
            short_name: "SE_RESIZE",
            doc: Some("Window resize bottom-right. May be NWSE_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_S_RESIZE",
            short_name: "S_RESIZE",
            doc: Some("Window resize bottom. May be NS_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_SW_RESIZE",
            short_name: "SW_RESIZE",
            doc: Some("Window resize bottom-left. May be NESW_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_W_RESIZE",
            short_name: "W_RESIZE",
            doc: Some("Window resize left. May be EW_RESIZE.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_CURSOR_COUNT",
            short_name: "COUNT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_MouseWheelDirection: Group = Group {
    module: "mouse",
    kind: GroupKind::Enum,
    name: "SDL_MouseWheelDirection",
    short_name: "MouseWheelDirection",
    doc: Some(
        "Scroll direction types for the Scroll event\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_MOUSEWHEEL_NORMAL",
            short_name: "NORMAL",
            doc: Some("The scroll direction is normal\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_MOUSEWHEEL_FLIPPED",
            short_name: "FLIPPED",
            doc: Some("The scroll direction is flipped / natural\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_MouseButtonFlags: Group = Group {
    module: "mouse",
    kind: GroupKind::Flags,
    name: "SDL_MouseButtonFlags",
    short_name: "MouseButtonFlags",
    doc: Some(
        "A bitmask of pressed mouse buttons, as reported by [`SDL_GetMouseState`], etc.\n\n- Button 1: Left mouse button\n- Button 2: Middle mouse button\n- Button 3: Right mouse button\n- Button 4: Side mouse button 1\n- Button 5: Side mouse button 2\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetMouseState`]\n- [`SDL_GetGlobalMouseState`]\n- [`SDL_GetRelativeMouseState`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_BUTTON_LMASK",
            short_name: "LMASK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_BUTTON_MMASK",
            short_name: "MMASK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_BUTTON_RMASK",
            short_name: "RMASK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_BUTTON_X1MASK",
            short_name: "X1MASK",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_BUTTON_X2MASK",
            short_name: "X2MASK",
            doc: None,
            available_since: None,
        },
    ],
};
