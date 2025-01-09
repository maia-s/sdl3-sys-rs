//! File dialog support.
//!
//! SDL offers file dialogs, to let users select files with native GUI
//! interfaces. There are "open" dialogs, "save" dialogs, and folder selection
//! dialogs. The app can control some details, such as filtering to specific
//! files, or whether multiple files can be selected by the user.
//!
//! Note that launching a file dialog is a non-blocking operation; control
//! returns to the app immediately, and a callback is called later (possibly in
//! another thread) when the user makes a choice.

use super::stdinc::*;

use super::error::*;

use super::properties::*;

use super::video::*;

/// An entry for filters for file dialogs.
///
/// `name` is a user-readable label for the filter (for example, "Office
/// document").
///
/// `pattern` is a semicolon-separated list of file extensions (for example,
/// "doc;docx"). File extensions may only contain alphanumeric characters,
/// hyphens, underscores and periods. Alternatively, the whole string can be a
/// single asterisk ("*"), which serves as an "All files" filter.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_DialogFileCallback`]
/// - [`SDL_ShowOpenFileDialog`]
/// - [`SDL_ShowSaveFileDialog`]
/// - [`SDL_ShowOpenFolderDialog`]
/// - [`SDL_ShowFileDialogWithProperties`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_DialogFileFilter {
    pub name: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
}

impl ::core::default::Default for SDL_DialogFileFilter {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// Callback used by file dialog functions.
///
/// The specific usage is described in each function.
///
/// If `filelist` is:
///
/// - NULL, an error occurred. Details can be obtained with [`SDL_GetError()`].
/// - A pointer to NULL, the user either didn't choose any file or canceled the
///   dialog.
/// - A pointer to non-`NULL`, the user chose one or more files. The argument
///   is a null-terminated list of pointers to C strings, each containing a
///   path.
///
/// The filelist argument should not be freed; it will automatically be freed
/// when the callback returns.
///
/// The filter argument is the index of the filter that was selected, or -1 if
/// no filter was selected or if the platform or method doesn't support
/// fetching the selected filter.
///
/// In Android, the `filelist` are `content://` URIs. They should be opened
/// using [`SDL_IOFromFile()`] with appropriate modes. This applies both to open
/// and save file dialog.
///
/// ### Parameters
/// - `userdata`: an app-provided pointer, for the callback's use.
/// - `filelist`: the file(s) chosen by the user.
/// - `filter`: index of the selected filter.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_DialogFileFilter`]
/// - [`SDL_ShowOpenFileDialog`]
/// - [`SDL_ShowSaveFileDialog`]
/// - [`SDL_ShowOpenFolderDialog`]
/// - [`SDL_ShowFileDialogWithProperties`]
pub type SDL_DialogFileCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        filelist: *const *const ::core::ffi::c_char,
        filter: ::core::ffi::c_int,
    ),
>;

extern "C" {
    /// Displays a dialog that lets the user select a file on their filesystem.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// Depending on the platform, the user may be allowed to input paths that
    /// don't yet exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to [`SDL_PumpEvents`] in their main loop.
    ///
    /// ### Parameters
    /// - `callback`: a function pointer to be invoked when the user selects a
    ///   file and accepts, or cancels the dialog, or an error
    ///   occurs.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    /// - `window`: the window that the dialog should be modal for, may be NULL.
    ///   Not all platforms support this option.
    /// - `filters`: a list of filters, may be NULL. Not all platforms support
    ///   this option, and platforms that do support it may allow the
    ///   user to ignore the filters. If non-NULL, it must remain
    ///   valid at least until the callback is invoked.
    /// - `nfilters`: the number of filters. Ignored if filters is NULL.
    /// - `default_location`: the default folder or file to start the dialog at,
    ///   may be NULL. Not all platforms support this option.
    /// - `allow_many`: if non-zero, the user will be allowed to select multiple
    ///   entries. Not all platforms support this option.
    ///
    /// ### Thread safety
    /// This function should be called only from the main thread. The
    ///   callback may be invoked from the same thread or from a
    ///   different one, depending on the OS's constraints.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DialogFileCallback`]
    /// - [`SDL_DialogFileFilter`]
    /// - [`SDL_ShowSaveFileDialog`]
    /// - [`SDL_ShowOpenFolderDialog`]
    /// - [`SDL_ShowFileDialogWithProperties`]
    pub fn SDL_ShowOpenFileDialog(
        callback: SDL_DialogFileCallback,
        userdata: *mut ::core::ffi::c_void,
        window: *mut SDL_Window,
        filters: *const SDL_DialogFileFilter,
        nfilters: ::core::ffi::c_int,
        default_location: *const ::core::ffi::c_char,
        allow_many: ::core::primitive::bool,
    );
}

extern "C" {
    /// Displays a dialog that lets the user choose a new or existing file on their
    /// filesystem.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// The chosen file may or may not already exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to [`SDL_PumpEvents`] in their main loop.
    ///
    /// ### Parameters
    /// - `callback`: a function pointer to be invoked when the user selects a
    ///   file and accepts, or cancels the dialog, or an error
    ///   occurs.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    /// - `window`: the window that the dialog should be modal for, may be NULL.
    ///   Not all platforms support this option.
    /// - `filters`: a list of filters, may be NULL. Not all platforms support
    ///   this option, and platforms that do support it may allow the
    ///   user to ignore the filters. If non-NULL, it must remain
    ///   valid at least until the callback is invoked.
    /// - `nfilters`: the number of filters. Ignored if filters is NULL.
    /// - `default_location`: the default folder or file to start the dialog at,
    ///   may be NULL. Not all platforms support this option.
    ///
    /// ### Thread safety
    /// This function should be called only from the main thread. The
    ///   callback may be invoked from the same thread or from a
    ///   different one, depending on the OS's constraints.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DialogFileCallback`]
    /// - [`SDL_DialogFileFilter`]
    /// - [`SDL_ShowOpenFileDialog`]
    /// - [`SDL_ShowOpenFolderDialog`]
    /// - [`SDL_ShowFileDialogWithProperties`]
    pub fn SDL_ShowSaveFileDialog(
        callback: SDL_DialogFileCallback,
        userdata: *mut ::core::ffi::c_void,
        window: *mut SDL_Window,
        filters: *const SDL_DialogFileFilter,
        nfilters: ::core::ffi::c_int,
        default_location: *const ::core::ffi::c_char,
    );
}

extern "C" {
    /// Displays a dialog that lets the user select a folder on their filesystem.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// Depending on the platform, the user may be allowed to input paths that
    /// don't yet exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to [`SDL_PumpEvents`] in their main loop.
    ///
    /// ### Parameters
    /// - `callback`: a function pointer to be invoked when the user selects a
    ///   file and accepts, or cancels the dialog, or an error
    ///   occurs.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    /// - `window`: the window that the dialog should be modal for, may be NULL.
    ///   Not all platforms support this option.
    /// - `default_location`: the default folder or file to start the dialog at,
    ///   may be NULL. Not all platforms support this option.
    /// - `allow_many`: if non-zero, the user will be allowed to select multiple
    ///   entries. Not all platforms support this option.
    ///
    /// ### Thread safety
    /// This function should be called only from the main thread. The
    ///   callback may be invoked from the same thread or from a
    ///   different one, depending on the OS's constraints.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DialogFileCallback`]
    /// - [`SDL_ShowOpenFileDialog`]
    /// - [`SDL_ShowSaveFileDialog`]
    /// - [`SDL_ShowFileDialogWithProperties`]
    pub fn SDL_ShowOpenFolderDialog(
        callback: SDL_DialogFileCallback,
        userdata: *mut ::core::ffi::c_void,
        window: *mut SDL_Window,
        default_location: *const ::core::ffi::c_char,
        allow_many: ::core::primitive::bool,
    );
}

/// Various types of file dialogs.
///
/// This is used by [`SDL_ShowFileDialogWithProperties()`] to decide what kind of
/// dialog to present to the user.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_ShowFileDialogWithProperties`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`OPENFILE`](SDL_FileDialogType::OPENFILE) | [`SDL_FILEDIALOG_OPENFILE`] | |
/// | [`SAVEFILE`](SDL_FileDialogType::SAVEFILE) | [`SDL_FILEDIALOG_SAVEFILE`] | |
/// | [`OPENFOLDER`](SDL_FileDialogType::OPENFOLDER) | [`SDL_FILEDIALOG_OPENFOLDER`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_FileDialogType(pub ::core::ffi::c_int);

impl From<SDL_FileDialogType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_FileDialogType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_FileDialogType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::OPENFILE => "SDL_FILEDIALOG_OPENFILE",
            Self::SAVEFILE => "SDL_FILEDIALOG_SAVEFILE",
            Self::OPENFOLDER => "SDL_FILEDIALOG_OPENFOLDER",

            _ => return write!(f, "SDL_FileDialogType({})", self.0),
        })
    }
}

impl SDL_FileDialogType {
    pub const OPENFILE: Self = Self(0);
    pub const SAVEFILE: Self = Self(1);
    pub const OPENFOLDER: Self = Self(2);
}

pub const SDL_FILEDIALOG_OPENFILE: SDL_FileDialogType = SDL_FileDialogType::OPENFILE;
pub const SDL_FILEDIALOG_SAVEFILE: SDL_FileDialogType = SDL_FileDialogType::SAVEFILE;
pub const SDL_FILEDIALOG_OPENFOLDER: SDL_FileDialogType = SDL_FileDialogType::OPENFOLDER;

extern "C" {
    /// Create and launch a file dialog with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_FILE_DIALOG_FILTERS_POINTER`]: a pointer to a list of
    ///   [`SDL_DialogFileFilter`] structs, which will be used as filters for
    ///   file-based selections. Ignored if the dialog is an "Open Folder" dialog.
    ///   If non-NULL, the array of filters must remain valid at least until the
    ///   callback is invoked.
    /// - [`SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER`]: the number of filters in the
    ///   array of filters, if it exists.
    /// - [`SDL_PROP_FILE_DIALOG_WINDOW_POINTER`]: the window that the dialog should
    ///   be modal for.
    /// - [`SDL_PROP_FILE_DIALOG_LOCATION_STRING`]: the default folder or file to
    ///   start the dialog at.
    /// - [`SDL_PROP_FILE_DIALOG_MANY_BOOLEAN`]: true to allow the user to select
    ///   more than one entry.
    /// - [`SDL_PROP_FILE_DIALOG_TITLE_STRING`]: the title for the dialog.
    /// - [`SDL_PROP_FILE_DIALOG_ACCEPT_STRING`]: the label that the accept button
    ///   should have.
    /// - [`SDL_PROP_FILE_DIALOG_CANCEL_STRING`]: the label that the cancel button
    ///   should have.
    ///
    /// Note that each platform may or may not support any of the properties.
    ///
    /// ### Parameters
    /// - `type`: the type of file dialog.
    /// - `callback`: a function pointer to be invoked when the user selects a
    ///   file and accepts, or cancels the dialog, or an error
    ///   occurs.
    /// - `userdata`: an optional pointer to pass extra data to the callback when
    ///   it will be invoked.
    /// - `props`: the properties to use.
    ///
    /// ### Thread safety
    /// This function should be called only from the main thread. The
    ///   callback may be invoked from the same thread or from a
    ///   different one, depending on the OS's constraints.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.8.
    ///
    /// ### See also
    /// - [`SDL_FileDialogType`]
    /// - [`SDL_DialogFileCallback`]
    /// - [`SDL_DialogFileFilter`]
    /// - [`SDL_ShowOpenFileDialog`]
    /// - [`SDL_ShowSaveFileDialog`]
    /// - [`SDL_ShowOpenFolderDialog`]
    pub fn SDL_ShowFileDialogWithProperties(
        r#type: SDL_FileDialogType,
        callback: SDL_DialogFileCallback,
        userdata: *mut ::core::ffi::c_void,
        props: SDL_PropertiesID,
    );
}

pub const SDL_PROP_FILE_DIALOG_FILTERS_POINTER: *const ::core::ffi::c_char =
    c"SDL.filedialog.filters".as_ptr();

pub const SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER: *const ::core::ffi::c_char =
    c"SDL.filedialog.nfilters".as_ptr();

pub const SDL_PROP_FILE_DIALOG_WINDOW_POINTER: *const ::core::ffi::c_char =
    c"SDL.filedialog.window".as_ptr();

pub const SDL_PROP_FILE_DIALOG_LOCATION_STRING: *const ::core::ffi::c_char =
    c"SDL.filedialog.location".as_ptr();

pub const SDL_PROP_FILE_DIALOG_MANY_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.filedialog.many".as_ptr();

pub const SDL_PROP_FILE_DIALOG_TITLE_STRING: *const ::core::ffi::c_char =
    c"SDL.filedialog.title".as_ptr();

pub const SDL_PROP_FILE_DIALOG_ACCEPT_STRING: *const ::core::ffi::c_char =
    c"SDL.filedialog.accept".as_ptr();

pub const SDL_PROP_FILE_DIALOG_CANCEL_STRING: *const ::core::ffi::c_char =
    c"SDL.filedialog.cancel".as_ptr();

#[cfg(doc)]
use crate::everything::*;
