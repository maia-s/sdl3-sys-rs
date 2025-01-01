//! System tray menu support.

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
/// ### Availability
/// This datatype is available since SDL 3.2.0.
///
/// ### See also
/// - [`SDL_InsertTrayEntryAt`]
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`SDL_TRAYENTRY_BUTTON`] | Make the entry a simple button. Required. |
/// | [`SDL_TRAYENTRY_CHECKBOX`] | Make the entry a checkbox. Required. |
/// | [`SDL_TRAYENTRY_SUBMENU`] | Prepare the entry to have a submenu. Required |
/// | [`SDL_TRAYENTRY_DISABLED`] | Make the entry disabled. Optional. |
/// | [`SDL_TRAYENTRY_CHECKED`] | Make the entry checked. This is valid only for checkboxes. Optional. |
pub type SDL_TrayEntryFlags = Uint32;

/// Make the entry a simple button. Required.
pub const SDL_TRAYENTRY_BUTTON: SDL_TrayEntryFlags = (0x00000001 as SDL_TrayEntryFlags);

/// Make the entry a checkbox. Required.
pub const SDL_TRAYENTRY_CHECKBOX: SDL_TrayEntryFlags = (0x00000002 as SDL_TrayEntryFlags);

/// Prepare the entry to have a submenu. Required
pub const SDL_TRAYENTRY_SUBMENU: SDL_TrayEntryFlags = (0x00000004 as SDL_TrayEntryFlags);

/// Make the entry disabled. Optional.
pub const SDL_TRAYENTRY_DISABLED: SDL_TrayEntryFlags = (0x80000000 as SDL_TrayEntryFlags);

/// Make the entry checked. This is valid only for checkboxes. Optional.
pub const SDL_TRAYENTRY_CHECKED: SDL_TrayEntryFlags = (0x40000000 as SDL_TrayEntryFlags);

/// A callback that is invoked when a tray entry is selected.
///
/// ### Parameters
/// - `userdata`: an optional pointer to pass extra data to the callback when
///   it will be invoked.
/// - `entry`: the tray entry that was selected.
///
/// ### Availability
/// This datatype is available since SDL 3.2.0.
///
/// ### See also
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
    /// ### Parameters
    /// - `icon`: a surface to be used as icon. May be NULL.
    /// - `tooltip`: a tooltip to be displayed when the mouse hovers the icon in
    ///   UTF-8 encoding. Not supported on all platforms. May be NULL.
    ///
    /// ### Return value
    /// Returns The newly created system tray icon.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `tray`: the tray icon to be updated.
    /// - `icon`: the new icon. May be NULL.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_CreateTray`]
    pub fn SDL_SetTrayIcon(tray: *mut SDL_Tray, icon: *mut SDL_Surface);
}

extern "C" {
    /// Updates the system tray icon's tooltip.
    ///
    /// ### Parameters
    /// - `tray`: the tray icon to be updated.
    /// - `tooltip`: the new tooltip in UTF-8 encoding. May be NULL.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `tray`: the tray to bind the menu to.
    ///
    /// ### Return value
    /// Returns the newly created menu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `entry`: the tray entry to bind the menu to.
    ///
    /// ### Return value
    /// Returns the newly created menu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `tray`: the tray entry to bind the menu to.
    ///
    /// ### Return value
    /// Returns the newly created menu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_CreateTray`]
    /// - [`SDL_CreateTrayMenu`]
    pub fn SDL_GetTrayMenu(tray: *mut SDL_Tray) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Gets a previously created tray entry submenu.
    ///
    /// You should have called [`SDL_CreateTraySubenu()`] on the entry object. This
    /// function allows you to fetch it again later.
    ///
    /// This function does the same thing as [`SDL_GetTrayMenu()`], except that it
    /// takes a [`SDL_TrayEntry`] instead of a [`SDL_Tray`].
    ///
    /// A menu does not need to be destroyed; it will be destroyed with the tray.
    ///
    /// ### Parameters
    /// - `entry`: the tray entry to bind the menu to.
    ///
    /// ### Return value
    /// Returns the newly created menu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_CreateTraySubmenu`]
    pub fn SDL_GetTraySubmenu(entry: *mut SDL_TrayEntry) -> *mut SDL_TrayMenu;
}

extern "C" {
    /// Returns a list of entries in the menu, in order.
    ///
    /// ### Parameters
    /// - `menu`: The menu to get entries from.
    /// - `size`: An optional pointer to obtain the number of entries in the
    ///   menu.
    ///
    /// ### Return value
    /// Returns a NULL-terminated list of entries within the given menu. The
    ///   pointer becomes invalid when any function that inserts or deletes
    ///   entries in the menu is called.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_RemoveTrayEntry`]
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_GetTrayEntries(
        menu: *mut SDL_TrayMenu,
        size: *mut ::core::ffi::c_int,
    ) -> *mut *const SDL_TrayEntry;
}

extern "C" {
    /// Removes a tray entry.
    ///
    /// ### Parameters
    /// - `entry`: The entry to be deleted.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `menu`: the menu to append the entry to.
    /// - `pos`: the desired position for the new entry. Entries at or following
    ///   this place will be moved. If pos is -1, the entry is appended.
    /// - `label`: the text to be displayed on the entry, in UTF-8 encoding, or
    ///   NULL for a separator.
    /// - `flags`: a combination of flags, some of which are mandatory.
    ///
    /// ### Return value
    /// Returns the newly created entry, or NULL if pos is out of bounds.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `entry`: the entry to be updated.
    /// - `label`: the new label for the entry in UTF-8 encoding.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ### Return value
    /// Returns the label of the entry in UTF-8 encoding.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `entry`: the entry to be updated.
    /// - `checked`: [`SDL_TRUE`] if the entry should be checked; [`SDL_FALSE`]
    ///   otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ### Return value
    /// Returns [`SDL_TRUE`] if the entry is checked; [`SDL_FALSE`] otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_SetTrayEntryChecked`]
    pub fn SDL_GetTrayEntryChecked(entry: *mut SDL_TrayEntry) -> ::core::primitive::bool;
}

extern "C" {
    /// Sets whether or not an entry is enabled.
    ///
    /// ### Parameters
    /// - `entry`: the entry to be updated.
    /// - `enabled`: [`SDL_TRUE`] if the entry should be enabled; [`SDL_FALSE`]
    ///   otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_GetTrayEntryEnabled`]
    pub fn SDL_SetTrayEntryEnabled(entry: *mut SDL_TrayEntry, enabled: ::core::primitive::bool);
}

extern "C" {
    /// Gets whether or not an entry is enabled.
    ///
    /// ### Parameters
    /// - `entry`: the entry to be read.
    ///
    /// ### Return value
    /// Returns [`SDL_TRUE`] if the entry is enabled; [`SDL_FALSE`] otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    /// - [`SDL_SetTrayEntryEnabled`]
    pub fn SDL_GetTrayEntryEnabled(entry: *mut SDL_TrayEntry) -> ::core::primitive::bool;
}

extern "C" {
    /// Sets a callback to be invoked when the entry is selected.
    ///
    /// ### Parameters
    /// - `entry`: the entry to be updated.
    /// - `callback`: a callback to be invoked when the entry is selected.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_GetTrayEntries`]
    /// - [`SDL_InsertTrayEntryAt`]
    pub fn SDL_SetTrayEntryCallback(
        entry: *mut SDL_TrayEntry,
        callback: SDL_TrayCallback,
        userdata: *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Destroys a tray object.
    ///
    /// This also destroys all associated menus and entries.
    ///
    /// ### Parameters
    /// - `tray`: the tray icon to be destroyed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_CreateTray`]
    pub fn SDL_DestroyTray(tray: *mut SDL_Tray);
}

extern "C" {
    /// Gets the menu contianing a certain tray entry.
    ///
    /// ### Parameters
    /// - `entry`: the entry for which to get the parent menu.
    ///
    /// ### Return value
    /// Returns the parent menu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `menu`: the menu for which to get the parent entry.
    ///
    /// ### Return value
    /// Returns the parent entry, or NULL if this menu is not a submenu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
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
    /// ### Parameters
    /// - `menu`: the menu for which to get the parent enttrayry.
    ///
    /// ### Return value
    /// Returns the parent tray, or NULL if this menu is a submenu.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_CreateTrayMenu`]
    /// - [`SDL_GetTrayMenuParentEntry`]
    pub fn SDL_GetTrayMenuParentTray(menu: *mut SDL_TrayMenu) -> *mut SDL_Tray;
}

/// An opaque handle representing a toplevel system tray object.
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_Tray {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing an entry on a system tray object.
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_TrayEntry {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a menu/submenu on a system tray object.
///
/// ### Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_TrayMenu {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
