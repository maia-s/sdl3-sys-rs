//! SDL Filesystem API.

use super::stdinc::*;

use super::error::*;

extern "C" {
    /// Get the directory where the application was run from.
    ///
    /// SDL caches the result of this call internally, but the first call to this
    /// function is not necessarily fast, so plan accordingly.
    ///
    /// **macOS and iOS Specific Functionality**: If the application is in a ".app"
    /// bundle, this function returns the Resource directory (e.g.
    /// MyApp.app/Contents/Resources/). This behaviour can be overridden by adding
    /// a property to the Info.plist file. Adding a string key with the name
    /// SDL_FILESYSTEM_BASE_DIR_TYPE with a supported value will change the
    /// behaviour.
    ///
    /// Supported values for the SDL_FILESYSTEM_BASE_DIR_TYPE property (Given an
    /// application in /Applications/SDLApp/MyApp.app):
    ///
    /// - `resource`: bundle resource directory (the default). For example:
    ///   `/Applications/SDLApp/MyApp.app/Contents/Resources`
    /// - `bundle`: the Bundle directory. For example:
    ///   `/Applications/SDLApp/MyApp.app/`
    /// - `parent`: the containing directory of the bundle. For example:
    ///   `/Applications/SDLApp/`
    ///
    /// **Nintendo 3DS Specific Functionality**: This function returns "romfs"
    /// directory of the application as it is uncommon to store resources outside
    /// the executable. As such it is not a writable directory.
    ///
    /// The returned path is guaranteed to end with a path separator ('\\' on
    /// Windows, '/' on most other platforms).
    ///
    /// \returns an absolute path in UTF-8 encoding to the application data
    ///          directory. NULL will be returned on error or when the platform
    ///          doesn't implement this functionality, call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetPrefPath
    pub fn SDL_GetBasePath() -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the user-and-app-specific path where files can be written.
    ///
    /// Get the "pref dir". This is meant to be where users can write personal
    /// files (preferences and save games, etc) that are specific to your
    /// application. This directory is unique per user, per application.
    ///
    /// This function will decide the appropriate location in the native
    /// filesystem, create the directory if necessary, and return a string of the
    /// absolute path to the directory in UTF-8 encoding.
    ///
    /// On Windows, the string might look like:
    ///
    /// `C:\\Users\\bob\\AppData\\Roaming\\My Company\\My Program Name\\`
    ///
    /// On Linux, the string might look like:
    ///
    /// `/home/bob/.local/share/My Program Name/`
    ///
    /// On macOS, the string might look like:
    ///
    /// `/Users/bob/Library/Application Support/My Program Name/`
    ///
    /// You should assume the path returned by this function is the only safe place
    /// to write files (and that SDL_GetBasePath(), while it might be writable, or
    /// even the parent of the returned path, isn't where you should be writing
    /// things).
    ///
    /// Both the org and app strings may become part of a directory name, so please
    /// follow these rules:
    ///
    /// - Try to use the same org string (_including case-sensitivity_) for all
    ///   your applications that use this function.
    /// - Always use a unique app string for each one, and make sure it never
    ///   changes for an app once you've decided on it.
    /// - Unicode characters are legal, as long as they are UTF-8 encoded, but...
    /// - ...only use letters, numbers, and spaces. Avoid punctuation like "Game
    ///   Name 2: Bad Guy's Revenge!" ... "Game Name 2" is sufficient.
    ///
    /// The returned path is guaranteed to end with a path separator ('\\' on
    /// Windows, '/' on most other platforms).
    ///
    /// \param org the name of your organization.
    /// \param app the name of your application.
    /// \returns a UTF-8 string of the user directory in platform-dependent
    ///          notation. NULL if there's a problem (creating directory failed,
    ///          etc.). This should be freed with SDL_free() when it is no longer
    ///          needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetBasePath
    pub fn SDL_GetPrefPath(
        org: *const ::core::ffi::c_char,
        app: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

/// The type of the OS-provided default folder for a specific purpose.
///
/// Note that the Trash folder isn't included here, because trashing files
/// usually involves extra OS-specific functionality to remember the file's
/// original location.
///
/// The folders supported per platform are:
///
/// |             | Windows | macOS/iOS | tvOS | Unix (XDG) | Haiku | Emscripten |
/// | ----------- | ------- | --------- | ---- | ---------- | ----- | ---------- |
/// | HOME        | X       | X         |      | X          | X     | X          |
/// | DESKTOP     | X       | X         |      | X          | X     |            |
/// | DOCUMENTS   | X       | X         |      | X          |       |            |
/// | DOWNLOADS   | Vista+  | X         |      | X          |       |            |
/// | MUSIC       | X       | X         |      | X          |       |            |
/// | PICTURES    | X       | X         |      | X          |       |            |
/// | PUBLICSHARE |         | X         |      | X          |       |            |
/// | SAVEDGAMES  | Vista+  |           |      |            |       |            |
/// | SCREENSHOTS | Vista+  |           |      |            |       |            |
/// | TEMPLATES   | X       | X         |      | X          |       |            |
/// | VIDEOS      | X       | X*        |      | X          |       |            |
///
/// Note that on macOS/iOS, the Videos folder is called "Movies".
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_GetUserFolder
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_FOLDER_HOME`], [`SDL_FOLDER_DESKTOP`], [`SDL_FOLDER_DOCUMENTS`], [`SDL_FOLDER_DOWNLOADS`], [`SDL_FOLDER_MUSIC`], [`SDL_FOLDER_PICTURES`], [`SDL_FOLDER_PUBLICSHARE`], [`SDL_FOLDER_SAVEDGAMES`], [`SDL_FOLDER_SCREENSHOTS`], [`SDL_FOLDER_TEMPLATES`], [`SDL_FOLDER_VIDEOS`], [`SDL_FOLDER_COUNT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Folder(pub ::core::ffi::c_int);
impl From<SDL_Folder> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Folder) -> Self {
        value.0
    }
}
impl SDL_Folder {
    /// The folder which contains all of the current user's data, preferences, and documents. It usually contains most of the other folders. If a requested folder does not exist, the home folder can be considered a safe fallback to store a user's documents.
    pub const HOME: Self = Self(0);
    /// The folder of files that are displayed on the desktop. Note that the existence of a desktop folder does not guarantee that the system does show icons on its desktop; certain GNU/Linux distros with a graphical environment may not have desktop icons.
    pub const DESKTOP: Self = Self(1);
    /// User document files, possibly application-specific. This is a good place to save a user's projects.
    pub const DOCUMENTS: Self = Self(2);
    /// Standard folder for user files downloaded from the internet.
    pub const DOWNLOADS: Self = Self(3);
    /// Music files that can be played using a standard music player (mp3, ogg...).
    pub const MUSIC: Self = Self(4);
    /// Image files that can be displayed using a standard viewer (png, jpg...).
    pub const PICTURES: Self = Self(5);
    /// Files that are meant to be shared with other users on the same computer.
    pub const PUBLICSHARE: Self = Self(6);
    /// Save files for games.
    pub const SAVEDGAMES: Self = Self(7);
    /// Application screenshots.
    pub const SCREENSHOTS: Self = Self(8);
    /// Template files to be used when the user requests the desktop environment to create a new file in a certain folder, such as "New Text File.txt".  Any file in the Templates folder can be used as a starting point for a new file.
    pub const TEMPLATES: Self = Self(9);
    /// Video files that can be played using a standard video player (mp4, webm...).
    pub const VIDEOS: Self = Self(10);
    /// Total number of types in this enum, not a folder type by itself.
    pub const COUNT: Self = Self(11);
}
/// The folder which contains all of the current user's data, preferences, and documents. It usually contains most of the other folders. If a requested folder does not exist, the home folder can be considered a safe fallback to store a user's documents.
pub const SDL_FOLDER_HOME: SDL_Folder = SDL_Folder::HOME;
/// The folder of files that are displayed on the desktop. Note that the existence of a desktop folder does not guarantee that the system does show icons on its desktop; certain GNU/Linux distros with a graphical environment may not have desktop icons.
pub const SDL_FOLDER_DESKTOP: SDL_Folder = SDL_Folder::DESKTOP;
/// User document files, possibly application-specific. This is a good place to save a user's projects.
pub const SDL_FOLDER_DOCUMENTS: SDL_Folder = SDL_Folder::DOCUMENTS;
/// Standard folder for user files downloaded from the internet.
pub const SDL_FOLDER_DOWNLOADS: SDL_Folder = SDL_Folder::DOWNLOADS;
/// Music files that can be played using a standard music player (mp3, ogg...).
pub const SDL_FOLDER_MUSIC: SDL_Folder = SDL_Folder::MUSIC;
/// Image files that can be displayed using a standard viewer (png, jpg...).
pub const SDL_FOLDER_PICTURES: SDL_Folder = SDL_Folder::PICTURES;
/// Files that are meant to be shared with other users on the same computer.
pub const SDL_FOLDER_PUBLICSHARE: SDL_Folder = SDL_Folder::PUBLICSHARE;
/// Save files for games.
pub const SDL_FOLDER_SAVEDGAMES: SDL_Folder = SDL_Folder::SAVEDGAMES;
/// Application screenshots.
pub const SDL_FOLDER_SCREENSHOTS: SDL_Folder = SDL_Folder::SCREENSHOTS;
/// Template files to be used when the user requests the desktop environment to create a new file in a certain folder, such as "New Text File.txt".  Any file in the Templates folder can be used as a starting point for a new file.
pub const SDL_FOLDER_TEMPLATES: SDL_Folder = SDL_Folder::TEMPLATES;
/// Video files that can be played using a standard video player (mp4, webm...).
pub const SDL_FOLDER_VIDEOS: SDL_Folder = SDL_Folder::VIDEOS;
/// Total number of types in this enum, not a folder type by itself.
pub const SDL_FOLDER_COUNT: SDL_Folder = SDL_Folder::COUNT;

extern "C" {
    /// Finds the most suitable user folder for a specific purpose.
    ///
    /// Many OSes provide certain standard folders for certain purposes, such as
    /// storing pictures, music or videos for a certain user. This function gives
    /// the path for many of those special locations.
    ///
    /// This function is specifically for _user_ folders, which are meant for the
    /// user to access and manage. For application-specific folders, meant to hold
    /// data for the application to manage, see SDL_GetBasePath() and
    /// SDL_GetPrefPath().
    ///
    /// The returned path is guaranteed to end with a path separator ('\\' on
    /// Windows, '/' on most other platforms).
    ///
    /// If NULL is returned, the error may be obtained with SDL_GetError().
    ///
    /// \param folder the type of folder to find.
    /// \returns either a null-terminated C string containing the full path to the
    ///          folder, or NULL if an error happened.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetUserFolder(folder: SDL_Folder) -> *const ::core::ffi::c_char;
}

/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PATHTYPE_NONE`], [`SDL_PATHTYPE_FILE`], [`SDL_PATHTYPE_DIRECTORY`], [`SDL_PATHTYPE_OTHER`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PathType(pub ::core::ffi::c_int);
impl From<SDL_PathType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PathType) -> Self {
        value.0
    }
}
impl SDL_PathType {
    /// path does not exist
    pub const NONE: Self = Self(0);
    /// a normal file
    pub const FILE: Self = Self(1);
    /// a directory
    pub const DIRECTORY: Self = Self(2);
    /// something completely different like a device node (not a symlink, those are always followed)
    pub const OTHER: Self = Self(3);
}
/// path does not exist
pub const SDL_PATHTYPE_NONE: SDL_PathType = SDL_PathType::NONE;
/// a normal file
pub const SDL_PATHTYPE_FILE: SDL_PathType = SDL_PathType::FILE;
/// a directory
pub const SDL_PATHTYPE_DIRECTORY: SDL_PathType = SDL_PathType::DIRECTORY;
/// something completely different like a device node (not a symlink, those are always followed)
pub const SDL_PATHTYPE_OTHER: SDL_PathType = SDL_PathType::OTHER;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_PathInfo {
    /// the path type
    pub r#type: SDL_PathType,
    /// the file size in bytes
    pub size: Uint64,
    /// the time when the path was created
    pub create_time: SDL_Time,
    /// the last time the path was modified
    pub modify_time: SDL_Time,
    /// the last time the path was read
    pub access_time: SDL_Time,
}

/// Flags for path matching
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_GlobDirectory
/// \sa SDL_GlobStorageDirectory
pub type SDL_GlobFlags = Uint32;

pub const SDL_GLOB_CASEINSENSITIVE: ::core::primitive::u32 = 1_u32;

extern "C" {
    /// Create a directory.
    ///
    /// \param path the path of the directory to create.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CreateDirectory(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

pub type SDL_EnumerateDirectoryCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        dirname: *const ::core::ffi::c_char,
        fname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

extern "C" {
    /// Enumerate a directory through a callback function.
    ///
    /// This function provides every directory entry through an app-provided
    /// callback, called once for each directory entry, until all results have been
    /// provided or the callback returns <= 0.
    ///
    /// \param path the path of the directory to enumerate.
    /// \param callback a function that is called for each entry in the directory.
    /// \param userdata a pointer that is passed to `callback`.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EnumerateDirectory(
        path: *const ::core::ffi::c_char,
        callback: SDL_EnumerateDirectoryCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove a file or an empty directory.
    ///
    /// \param path the path of the directory to enumerate.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RemovePath(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Rename a file or directory.
    ///
    /// \param oldpath the old path.
    /// \param newpath the new path.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RenamePath(
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Copy a file.
    ///
    /// \param oldpath the old path.
    /// \param newpath the new path.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CopyFile(
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get information about a filesystem path.
    ///
    /// \param path the path to query.
    /// \param info a pointer filled in with information about the path, or NULL to
    ///             check for the existence of a file.
    /// \returns true on success or false if the file doesn't exist, or another
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetPathInfo(
        path: *const ::core::ffi::c_char,
        info: *mut SDL_PathInfo,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Enumerate a directory tree, filtered by pattern, and return a list.
    ///
    /// Files are filtered out if they don't match the string in `pattern`, which
    /// may contain wildcard characters '*' (match everything) and '?' (match one
    /// character). If pattern is NULL, no filtering is done and all results are
    /// returned. Subdirectories are permitted, and are specified with a path
    /// separator of '/'. Wildcard characters '*' and '?' never match a path
    /// separator.
    ///
    /// `flags` may be set to SDL_GLOB_CASEINSENSITIVE to make the pattern matching
    /// case-insensitive.
    ///
    /// The returned array is always NULL-terminated, for your iterating
    /// convenience, but if `count` is non-NULL, on return it will contain the
    /// number of items in the array, not counting the NULL terminator.
    ///
    /// \param path the path of the directory to enumerate.
    /// \param pattern the pattern that files in the directory must match. Can be
    ///                NULL.
    /// \param flags `SDL_GLOB_*` bitflags that affect this search.
    /// \param count on return, will be set to the number of items in the returned
    ///              array. Can be NULL.
    /// \returns an array of strings on success or NULL on failure; call
    ///          SDL_GetError() for more information. This is a single allocation
    ///          that should be freed with SDL_free() when it is no longer needed.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GlobDirectory(
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        flags: SDL_GlobFlags,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
}
