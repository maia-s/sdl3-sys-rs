//! SDL offers a way to add items to the "system tray" (more correctly called
//! the "notification area" on Windows). On platforms that offer this concept,
//! an SDL app can add a tray icon, submenus, checkboxes, and clickable
//! entries, and register a callback that is fired when the user clicks on
//! these pieces.

use super::stdinc::*;

use super::error::*;

use super::surface::*;

use super::video::*;

/// Flags that control the creation of system tray entries.
///
/// Some of these flags are required; exactly one of them must be specified at
/// the time a tray entry is created. Other flags are optional; zero or more of
/// those can be OR'ed together with the required flag.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_InsertTrayEntryAt`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`BUTTON`](SDL_TrayEntryFlags::BUTTON) | [`SDL_TRAYENTRY_BUTTON`] | Make the entry a simple button. Required. |
/// | [`CHECKBOX`](SDL_TrayEntryFlags::CHECKBOX) | [`SDL_TRAYENTRY_CHECKBOX`] | Make the entry a checkbox. Required. |
/// | [`SUBMENU`](SDL_TrayEntryFlags::SUBMENU) | [`SDL_TRAYENTRY_SUBMENU`] | Prepare the entry to have a submenu. Required |
/// | [`DISABLED`](SDL_TrayEntryFlags::DISABLED) | [`SDL_TRAYENTRY_DISABLED`] | Make the entry disabled. Optional. |
/// | [`CHECKED`](SDL_TrayEntryFlags::CHECKED) | [`SDL_TRAYENTRY_CHECKED`] | Make the entry checked. This is valid only for checkboxes. Optional. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_TrayEntryFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_TrayEntryFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_TrayEntryFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_TrayEntryFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_TrayEntryFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_TrayEntryFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_TrayEntryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_TrayEntryFlags(")?;
        let all_bits = all_bits | Self::BUTTON.0;
        if (Self::BUTTON != 0 || self.0 == 0) && *self & Self::BUTTON == Self::BUTTON {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "BUTTON")?;
        }
        let all_bits = all_bits | Self::CHECKBOX.0;
        if (Self::CHECKBOX != 0 || self.0 == 0) && *self & Self::CHECKBOX == Self::CHECKBOX {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "CHECKBOX")?;
        }
        let all_bits = all_bits | Self::SUBMENU.0;
        if (Self::SUBMENU != 0 || self.0 == 0) && *self & Self::SUBMENU == Self::SUBMENU {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "SUBMENU")?;
        }
        let all_bits = all_bits | Self::DISABLED.0;
        if (Self::DISABLED != 0 || self.0 == 0) && *self & Self::DISABLED == Self::DISABLED {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "DISABLED")?;
        }
        let all_bits = all_bits | Self::CHECKED.0;
        if (Self::CHECKED != 0 || self.0 == 0) && *self & Self::CHECKED == Self::CHECKED {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "CHECKED")?;
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

impl ::core::ops::BitAnd for SDL_TrayEntryFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_TrayEntryFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_TrayEntryFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_TrayEntryFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_TrayEntryFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_TrayEntryFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_TrayEntryFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_TrayEntryFlags {
    /// Make the entry a simple button. Required.
    pub const BUTTON: Self = Self((0x00000001 as Uint32));
    /// Make the entry a checkbox. Required.
    pub const CHECKBOX: Self = Self((0x00000002 as Uint32));
    /// Prepare the entry to have a submenu. Required
    pub const SUBMENU: Self = Self((0x00000004 as Uint32));
    /// Make the entry disabled. Optional.
    pub const DISABLED: Self = Self((0x80000000 as Uint32));
    /// Make the entry checked. This is valid only for checkboxes. Optional.
    pub const CHECKED: Self = Self((0x40000000 as Uint32));
}

/// Make the entry a simple button. Required.
pub const SDL_TRAYENTRY_BUTTON: SDL_TrayEntryFlags = SDL_TrayEntryFlags::BUTTON;
/// Make the entry a checkbox. Required.
pub const SDL_TRAYENTRY_CHECKBOX: SDL_TrayEntryFlags = SDL_TrayEntryFlags::CHECKBOX;
/// Prepare the entry to have a submenu. Required
pub const SDL_TRAYENTRY_SUBMENU: SDL_TrayEntryFlags = SDL_TrayEntryFlags::SUBMENU;
/// Make the entry disabled. Optional.
pub const SDL_TRAYENTRY_DISABLED: SDL_TrayEntryFlags = SDL_TrayEntryFlags::DISABLED;
/// Make the entry checked. This is valid only for checkboxes. Optional.
pub const SDL_TRAYENTRY_CHECKED: SDL_TrayEntryFlags = SDL_TrayEntryFlags::CHECKED;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_TrayEntryFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::tray::METADATA_SDL_TrayEntryFlags;
}

/// A callback that is invoked when a tray entry is selected.
///
/// ## Parameters
/// - `userdata`: an optional pointer to pass extra data to the callback when
///   it will be invoked.
/// - `entry`: the tray entry that was selected.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_SetTrayEntryCallback`]
pub type SDL_TrayCallback = ::core::option::Option<
    unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, entry: *mut SDL_TrayEntry),
>;

extern "C" {
    /// Create an icon to be placed in the operating system's tray, or equivalent.
    ///
    /// Many platforms advise not using a system tray unless persistence is a
    /// necessary feature. Avoid needlessly creating a tray icon, as the user may
    /// feel like it clutters their interface.
    ///
    /// Using tray icons require the video subsystem.
    ///
    /// ## Parameters
    /// - `icon`: a surface to be used as icon. May be NULL.
    /// - `tooltip`: a tooltip to be displayed when the mouse hovers the icon in
    ///   UTF-8 encoding. Not supported on all platforms. May be NULL.
    ///
    /// ## Return value
    /// Returns The newly created system tray icon.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTrayMenu`]
    /// - [`SDL_GetTrayMenu`]
    /// - [`SDL_DestroyTray`]
    pub fn SDL_CreateTray(
        icon: *mut SDL_Surface,
        tooltip: *const ::core::ffi::c_char,
    ) -> *mut SDL_Tray;
}

extern "C" {
    /// Updates the system tray icon's icon.
    ///
    /// ## Parameters
    /// - `tray`: the tray icon to be updated.
    /// - `icon`: the new icon. May be NULL.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTray`]
    pub fn SDL_SetTrayIcon(tray: *mut SDL_Tray, icon: *mut SDL_Surface);
}

extern "C" {
    /// Updates the system tray icon's tooltip.
    ///
    /// ## Parameters
    /// - `tray`: the tray icon to be updated.
    /// - `tooltip`: the new tooltip in UTF-8 encoding. May be NULL.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTray`]
    pub fn SDL_SetTrayTooltip(tray: *mut SDL_Tray, tooltip: *const ::core::ffi::c_char);
}

extern "C" {
    /// Create a menu for a system tray.
    ///
    /// This should be called at most once per tray icon.
    ///
    /// This function does the same thing as [`SDL_CreateTraySubmenu()`], except that
    /// it takes a [`SDL_Tray`] instead of a [`SDL_TrayEntry`].
    ///
    /// A menu does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ## Parameters
    /// - `tray`: the tray to bind the menu to.
    ///
    /// ## Return value
    /// Returns the newly created menu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTray`]
    /// - [`SDL_GetTrayMenu`]
    /// - [`SDL_GetTrayMenuParentTray`]
    pub fn SDL_CreateTrayMenu(tray: *mut SDL_Tray) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Create a submenu for a system tray entry.
    ///
    /// This should be called at most once per tray entry.
    ///
    /// This function does the same thing as [`SDL_CreateTrayMenu`], except that it
    /// takes a [`SDL_TrayEntry`] instead of a [`SDL_Tray`].
    ///
    /// A menu does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ## Parameters
    /// - `entry`: the tray entry to bind the menu to.
    ///
    /// ## Return value
    /// Returns the newly created menu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_GetTraySubmenu`]
    /// - [`SDL_GetTrayMenuParentEntry`]
    pub fn SDL_CreateTraySubmenu(entry: *mut SDL_TrayEntry) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Gets a previously created tray menu.
    ///
    /// You should have called [`SDL_CreateTrayMenu()`] on the tray object. This
    /// function allows you to fetch it again later.
    ///
    /// This function does the same thing as [`SDL_GetTraySubmenu()`], except that it
    /// takes a [`SDL_Tray`] instead of a [`SDL_TrayEntry`].
    ///
    /// A menu does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ## Parameters
    /// - `tray`: the tray entry to bind the menu to.
    ///
    /// ## Return value
    /// Returns the newly created menu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTray`]
    /// - [`SDL_CreateTrayMenu`]
    pub fn SDL_GetTrayMenu(tray: *mut SDL_Tray) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Gets a previously created tray entry submenu.
    ///
    /// You should have called [`SDL_CreateTraySubmenu()`] on the entry object. This
    /// function allows you to fetch it again later.
    ///
    /// This function does the same thing as [`SDL_GetTrayMenu()`], except that it
    /// takes a [`SDL_TrayEntry`] instead of a [`SDL_Tray`].
    ///
    /// A menu does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ## Parameters
    /// - `entry`: the tray entry to bind the menu to.
    ///
    /// ## Return value
    /// Returns the newly created menu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_CreateTraySubmenu`]
    pub fn SDL_GetTraySubmenu(entry: *mut SDL_TrayEntry) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Returns a list of entries in the menu, in order.
    ///
    /// ## Parameters
    /// - `menu`: The menu to get entries from.
    /// - `count`: An optional pointer to obtain the number of entries in the
    ///   menu.
    ///
    /// ## Return value
    /// Returns a NULL-terminated list of entries within the given menu. The
    ///   pointer becomes invalid when any function that inserts or deletes
    ///   entries in the menu is called.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_RemoveTrayEntry`]
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_GetTrayEntries(
        menu: *mut SDL_TrayMenu,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *const SDL_TrayEntry;
}

extern "C" {
    /// Removes a tray entry.
    ///
    /// ## Parameters
    /// - `entry`: The entry to be deleted.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_RemoveTrayEntry(entry: *mut SDL_TrayEntry);
}

extern "C" {
    /// Insert a tray entry at a given position.
    ///
    /// If label is NULL, the entry will be a separator. Many functions won't work
    /// for an entry that is a separator.
    ///
    /// An entry does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ## Parameters
    /// - `menu`: the menu to append the entry to.
    /// - `pos`: the desired position for the new entry. Entries at or following
    ///   this place will be moved. If pos is -1, the entry is appended.
    /// - `label`: the text to be displayed on the entry, in UTF-8 encoding, or
    ///   NULL for a separator.
    /// - `flags`: a combination of flags, some of which are mandatory.
    ///
    /// ## Return value
    /// Returns the newly created entry, or NULL if pos is out of bounds.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_TrayEntryFlags`]
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_RemoveTrayEntry`]
    /// - [`SDL_GetTrayEntryParent`]
    pub fn SDL_InsertTrayEntryAt(
        menu: *mut SDL_TrayMenu,
        pos: ::core::ffi::c_int,
        label: *const ::core::ffi::c_char,
        flags: SDL_TrayEntryFlags,
    ) -> *mut SDL_TrayEntry;
}

extern "C" {
    /// Sets the label of an entry.
    ///
    /// An entry cannot change between a separator and an ordinary entry; that is,
    /// it is not possible to set a non-NULL label on an entry that has a NULL
    /// label (separators), or to set a NULL label to an entry that has a non-NULL
    /// label. The function will silently fail if that happens.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be updated.
    /// - `label`: the new label for the entry in UTF-8 encoding.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_GetTrayEntryLabel`]
    pub fn SDL_SetTrayEntryLabel(entry: *mut SDL_TrayEntry, label: *const ::core::ffi::c_char);
}

extern "C" {
    /// Gets the label of an entry.
    ///
    /// If the returned value is NULL, the entry is a separator.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ## Return value
    /// Returns the label of the entry in UTF-8 encoding.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_SetTrayEntryLabel`]
    pub fn SDL_GetTrayEntryLabel(entry: *mut SDL_TrayEntry) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Sets whether or not an entry is checked.
    ///
    /// The entry must have been created with the [`SDL_TRAYENTRY_CHECKBOX`] flag.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be updated.
    /// - `checked`: true if the entry should be checked; false otherwise.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_GetTrayEntryChecked`]
    pub fn SDL_SetTrayEntryChecked(entry: *mut SDL_TrayEntry, checked: ::core::primitive::bool);
}

extern "C" {
    /// Gets whether or not an entry is checked.
    ///
    /// The entry must have been created with the [`SDL_TRAYENTRY_CHECKBOX`] flag.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ## Return value
    /// Returns true if the entry is checked; false otherwise.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_SetTrayEntryChecked`]
    pub fn SDL_GetTrayEntryChecked(entry: *mut SDL_TrayEntry) -> ::core::primitive::bool;
}

extern "C" {
    /// Sets whether or not an entry is enabled.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be updated.
    /// - `enabled`: true if the entry should be enabled; false otherwise.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_GetTrayEntryEnabled`]
    pub fn SDL_SetTrayEntryEnabled(entry: *mut SDL_TrayEntry, enabled: ::core::primitive::bool);
}

extern "C" {
    /// Gets whether or not an entry is enabled.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ## Return value
    /// Returns true if the entry is enabled; false otherwise.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_SetTrayEntryEnabled`]
    pub fn SDL_GetTrayEntryEnabled(entry: *mut SDL_TrayEntry) -> ::core::primitive::bool;
}

extern "C" {
    /// Sets a callback to be invoked when the entry is selected.
    ///
    /// ## Parameters
    /// - `entry`: the entry to be updated.
    /// - `callback`: a callback to be invoked when the entry is selected.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_SetTrayEntryCallback(
        entry: *mut SDL_TrayEntry,
        callback: SDL_TrayCallback,
        userdata: *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Simulate a click on a tray entry.
    ///
    /// ## Parameters
    /// - `entry`: The entry to activate.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ClickTrayEntry(entry: *mut SDL_TrayEntry);
}

extern "C" {
    /// Destroys a tray object.
    ///
    /// This also destroys all associated menus and entries.
    ///
    /// ## Parameters
    /// - `tray`: the tray icon to be destroyed.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTray`]
    pub fn SDL_DestroyTray(tray: *mut SDL_Tray);
}

extern "C" {
    /// Gets the menu containing a certain tray entry.
    ///
    /// ## Parameters
    /// - `entry`: the entry for which to get the parent menu.
    ///
    /// ## Return value
    /// Returns the parent menu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_GetTrayEntryParent(entry: *mut SDL_TrayEntry) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Gets the entry for which the menu is a submenu, if the current menu is a
    /// submenu.
    ///
    /// Either this function or [`SDL_GetTrayMenuParentTray()`] will return non-NULL
    /// for any given menu.
    ///
    /// ## Parameters
    /// - `menu`: the menu for which to get the parent entry.
    ///
    /// ## Return value
    /// Returns the parent entry, or NULL if this menu is not a submenu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTraySubmenu`]
    /// - [`SDL_GetTrayMenuParentTray`]
    pub fn SDL_GetTrayMenuParentEntry(menu: *mut SDL_TrayMenu) -> *mut SDL_TrayEntry;
}

extern "C" {
    /// Gets the tray for which this menu is the first-level menu, if the current
    /// menu isn't a submenu.
    ///
    /// Either this function or [`SDL_GetTrayMenuParentEntry()`] will return non-NULL
    /// for any given menu.
    ///
    /// ## Parameters
    /// - `menu`: the menu for which to get the parent enttrayry.
    ///
    /// ## Return value
    /// Returns the parent tray, or NULL if this menu is a submenu.
    ///
    /// ## Thread safety
    /// This function should be called on the thread that created the
    ///   tray.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateTrayMenu`]
    /// - [`SDL_GetTrayMenuParentEntry`]
    pub fn SDL_GetTrayMenuParentTray(menu: *mut SDL_TrayMenu) -> *mut SDL_Tray;
}

extern "C" {
    /// Update the trays.
    ///
    /// This is called automatically by the event loop and is only needed if you're
    /// using trays but aren't handling SDL events.
    ///
    /// ## Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_UpdateTrays();
}

/// An opaque handle representing a toplevel system tray object.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_Tray {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing an entry on a system tray object.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_TrayEntry {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a menu/submenu on a system tray object.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_TrayMenu {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
