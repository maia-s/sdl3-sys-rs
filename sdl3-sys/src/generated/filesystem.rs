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
    /// ### Return value
    /// Returns an absolute path in UTF-8 encoding to the application data
    ///   directory. NULL will be returned on error or when the platform
    ///   doesn't implement this functionality, call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPrefPath`]
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
    /// to write files (and that [`SDL_GetBasePath()`], while it might be writable, or
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
    /// ### Parameters
    /// - `org`: the name of your organization.
    /// - `app`: the name of your application.
    ///
    /// ### Return value
    /// Returns a UTF-8 string of the user directory in platform-dependent
    ///   notation. NULL if there's a problem (creating directory failed,
    ///   etc.). This should be freed with [`SDL_free()`] when it is no longer
    ///   needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetBasePath`]
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
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetUserFolder`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`HOME`](SDL_Folder::HOME) | [`SDL_FOLDER_HOME`] | The folder which contains all of the current user's data, preferences, and documents. It usually contains most of the other folders. If a requested folder does not exist, the home folder can be considered a safe fallback to store a user's documents. |
/// | [`DESKTOP`](SDL_Folder::DESKTOP) | [`SDL_FOLDER_DESKTOP`] | The folder of files that are displayed on the desktop. Note that the existence of a desktop folder does not guarantee that the system does show icons on its desktop; certain GNU/Linux distros with a graphical environment may not have desktop icons. |
/// | [`DOCUMENTS`](SDL_Folder::DOCUMENTS) | [`SDL_FOLDER_DOCUMENTS`] | User document files, possibly application-specific. This is a good place to save a user's projects. |
/// | [`DOWNLOADS`](SDL_Folder::DOWNLOADS) | [`SDL_FOLDER_DOWNLOADS`] | Standard folder for user files downloaded from the internet. |
/// | [`MUSIC`](SDL_Folder::MUSIC) | [`SDL_FOLDER_MUSIC`] | Music files that can be played using a standard music player (mp3, ogg...). |
/// | [`PICTURES`](SDL_Folder::PICTURES) | [`SDL_FOLDER_PICTURES`] | Image files that can be displayed using a standard viewer (png, jpg...). |
/// | [`PUBLICSHARE`](SDL_Folder::PUBLICSHARE) | [`SDL_FOLDER_PUBLICSHARE`] | Files that are meant to be shared with other users on the same computer. |
/// | [`SAVEDGAMES`](SDL_Folder::SAVEDGAMES) | [`SDL_FOLDER_SAVEDGAMES`] | Save files for games. |
/// | [`SCREENSHOTS`](SDL_Folder::SCREENSHOTS) | [`SDL_FOLDER_SCREENSHOTS`] | Application screenshots. |
/// | [`TEMPLATES`](SDL_Folder::TEMPLATES) | [`SDL_FOLDER_TEMPLATES`] | Template files to be used when the user requests the desktop environment to create a new file in a certain folder, such as "New Text File.txt".  Any file in the Templates folder can be used as a starting point for a new file. |
/// | [`VIDEOS`](SDL_Folder::VIDEOS) | [`SDL_FOLDER_VIDEOS`] | Video files that can be played using a standard video player (mp4, webm...). |
/// | [`COUNT`](SDL_Folder::COUNT) | [`SDL_FOLDER_COUNT`] | Total number of types in this enum, not a folder type by itself. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_Folder(pub ::core::ffi::c_int);

impl From<SDL_Folder> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Folder) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_Folder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::HOME => "SDL_FOLDER_HOME",
            Self::DESKTOP => "SDL_FOLDER_DESKTOP",
            Self::DOCUMENTS => "SDL_FOLDER_DOCUMENTS",
            Self::DOWNLOADS => "SDL_FOLDER_DOWNLOADS",
            Self::MUSIC => "SDL_FOLDER_MUSIC",
            Self::PICTURES => "SDL_FOLDER_PICTURES",
            Self::PUBLICSHARE => "SDL_FOLDER_PUBLICSHARE",
            Self::SAVEDGAMES => "SDL_FOLDER_SAVEDGAMES",
            Self::SCREENSHOTS => "SDL_FOLDER_SCREENSHOTS",
            Self::TEMPLATES => "SDL_FOLDER_TEMPLATES",
            Self::VIDEOS => "SDL_FOLDER_VIDEOS",
            Self::COUNT => "SDL_FOLDER_COUNT",

            _ => return write!(f, "SDL_Folder({})", self.0),
        })
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
    /// data for the application to manage, see [`SDL_GetBasePath()`] and
    /// [`SDL_GetPrefPath()`].
    ///
    /// The returned path is guaranteed to end with a path separator ('\\' on
    /// Windows, '/' on most other platforms).
    ///
    /// If NULL is returned, the error may be obtained with [`SDL_GetError()`].
    ///
    /// ### Parameters
    /// - `folder`: the type of folder to find.
    ///
    /// ### Return value
    /// Returns either a null-terminated C string containing the full path to the
    ///   folder, or NULL if an error happened.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetUserFolder(folder: SDL_Folder) -> *const ::core::ffi::c_char;
}

/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_PathType::NONE) | [`SDL_PATHTYPE_NONE`] | path does not exist |
/// | [`FILE`](SDL_PathType::FILE) | [`SDL_PATHTYPE_FILE`] | a normal file |
/// | [`DIRECTORY`](SDL_PathType::DIRECTORY) | [`SDL_PATHTYPE_DIRECTORY`] | a directory |
/// | [`OTHER`](SDL_PathType::OTHER) | [`SDL_PATHTYPE_OTHER`] | something completely different like a device node (not a symlink, those are always followed) |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PathType(pub ::core::ffi::c_int);

impl From<SDL_PathType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PathType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PathType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_PATHTYPE_NONE",
            Self::FILE => "SDL_PATHTYPE_FILE",
            Self::DIRECTORY => "SDL_PATHTYPE_DIRECTORY",
            Self::OTHER => "SDL_PATHTYPE_OTHER",

            _ => return write!(f, "SDL_PathType({})", self.0),
        })
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
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GlobDirectory`]
/// - [`SDL_GlobStorageDirectory`]
///
/// ### Known values (`sdl3-sys`)
/// | Constant | Description |
/// | -------- | ----------- |
/// | [`SDL_GLOB_CASEINSENSITIVE`] | |
pub type SDL_GlobFlags = Uint32;

pub const SDL_GLOB_CASEINSENSITIVE: SDL_GlobFlags = ((1_u32) as SDL_GlobFlags);

extern "C" {
    /// Create a directory, and any missing parent directories.
    ///
    /// This reports success if `path` already exists as a directory.
    ///
    /// If parent directories are missing, it will also create them. Note that if
    /// this fails, it will not remove any parent directories it already made.
    ///
    /// ### Parameters
    /// - `path`: the path of the directory to create.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_CreateDirectory(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

/// Possible results from an enumeration callback.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_EnumerateDirectoryCallback`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`CONTINUE`](SDL_EnumerationResult::CONTINUE) | [`SDL_ENUM_CONTINUE`] | Value that requests that enumeration continue. |
/// | [`SUCCESS`](SDL_EnumerationResult::SUCCESS) | [`SDL_ENUM_SUCCESS`] | Value that requests that enumeration stop, successfully. |
/// | [`FAILURE`](SDL_EnumerationResult::FAILURE) | [`SDL_ENUM_FAILURE`] | Value that requests that enumeration stop, as a failure. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_EnumerationResult(pub ::core::ffi::c_int);

impl From<SDL_EnumerationResult> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_EnumerationResult) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_EnumerationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::CONTINUE => "SDL_ENUM_CONTINUE",
            Self::SUCCESS => "SDL_ENUM_SUCCESS",
            Self::FAILURE => "SDL_ENUM_FAILURE",

            _ => return write!(f, "SDL_EnumerationResult({})", self.0),
        })
    }
}

impl SDL_EnumerationResult {
    /// Value that requests that enumeration continue.
    pub const CONTINUE: Self = Self(0);
    /// Value that requests that enumeration stop, successfully.
    pub const SUCCESS: Self = Self(1);
    /// Value that requests that enumeration stop, as a failure.
    pub const FAILURE: Self = Self(2);
}

/// Value that requests that enumeration continue.
pub const SDL_ENUM_CONTINUE: SDL_EnumerationResult = SDL_EnumerationResult::CONTINUE;
/// Value that requests that enumeration stop, successfully.
pub const SDL_ENUM_SUCCESS: SDL_EnumerationResult = SDL_EnumerationResult::SUCCESS;
/// Value that requests that enumeration stop, as a failure.
pub const SDL_ENUM_FAILURE: SDL_EnumerationResult = SDL_EnumerationResult::FAILURE;

/// Callback for directory enumeration.
///
/// Enumeration of directory entries will continue until either all entries
/// have been provided to the callback, or the callback has requested a stop
/// through its return value.
///
/// Returning [`SDL_ENUM_CONTINUE`] will let enumeration proceed, calling the
/// callback with further entries. [`SDL_ENUM_SUCCESS`] and [`SDL_ENUM_FAILURE`] will
/// terminate the enumeration early, and dictate the return value of the
/// enumeration function itself.
///
/// ### Parameters
/// - `userdata`: an app-controlled pointer that is passed to the callback.
/// - `dirname`: the directory that is being enumerated.
/// - `fname`: the next entry in the enumeration.
///
/// ### Return value
/// Returns how the enumeration should proceed.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_EnumerateDirectory`]
pub type SDL_EnumerateDirectoryCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        dirname: *const ::core::ffi::c_char,
        fname: *const ::core::ffi::c_char,
    ) -> SDL_EnumerationResult,
>;

extern "C" {
    /// Enumerate a directory through a callback function.
    ///
    /// This function provides every directory entry through an app-provided
    /// callback, called once for each directory entry, until all results have been
    /// provided or the callback returns either [`SDL_ENUM_SUCCESS`] or
    /// [`SDL_ENUM_FAILURE`].
    ///
    /// This will return false if there was a system problem in general, or if a
    /// callback returns [`SDL_ENUM_FAILURE`]. A successful return means a callback
    /// returned [`SDL_ENUM_SUCCESS`] to halt enumeration, or all directory entries
    /// were enumerated.
    ///
    /// ### Parameters
    /// - `path`: the path of the directory to enumerate.
    /// - `callback`: a function that is called for each entry in the directory.
    /// - `userdata`: a pointer that is passed to `callback`.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_EnumerateDirectory(
        path: *const ::core::ffi::c_char,
        callback: SDL_EnumerateDirectoryCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove a file or an empty directory.
    ///
    /// Directories that are not empty will fail; this function will not recursely
    /// delete directory trees.
    ///
    /// ### Parameters
    /// - `path`: the path to remove from the filesystem.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_RemovePath(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Rename a file or directory.
    ///
    /// If the file at `newpath` already exists, it will replaced.
    ///
    /// Note that this will not copy files across filesystems/drives/volumes, as
    /// that is a much more complicated (and possibly time-consuming) operation.
    ///
    /// Which is to say, if this function fails, [`SDL_CopyFile()`] to a temporary file
    /// in the same directory as `newpath`, then [`SDL_RenamePath()`] from the
    /// temporary file to `newpath` and [`SDL_RemovePath()`] on `oldpath` might work
    /// for files. Renaming a non-empty directory across filesystems is
    /// dramatically more complex, however.
    ///
    /// ### Parameters
    /// - `oldpath`: the old path.
    /// - `newpath`: the new path.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_RenamePath(
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Copy a file.
    ///
    /// If the file at `newpath` already exists, it will be overwritten with the
    /// contents of the file at `oldpath`.
    ///
    /// This function will block until the copy is complete, which might be a
    /// significant time for large files on slow disks. On some platforms, the copy
    /// can be handed off to the OS itself, but on others SDL might just open both
    /// paths, and read from one and write to the other.
    ///
    /// Note that this is not an atomic operation! If something tries to read from
    /// `newpath` while the copy is in progress, it will see an incomplete copy of
    /// the data, and if the calling thread terminates (or the power goes out)
    /// during the copy, `newpath`'s previous contents will be gone, replaced with
    /// an incomplete copy of the data. To avoid this risk, it is recommended that
    /// the app copy to a temporary file in the same directory as `newpath`, and if
    /// the copy is successful, use [`SDL_RenamePath()`] to replace `newpath` with the
    /// temporary file. This will ensure that reads of `newpath` will either see a
    /// complete copy of the data, or it will see the pre-copy state of `newpath`.
    ///
    /// This function attempts to synchronize the newly-copied data to disk before
    /// returning, if the platform allows it, so that the renaming trick will not
    /// have a problem in a system crash or power failure, where the file could be
    /// renamed but the contents never made it from the system file cache to the
    /// physical disk.
    ///
    /// If the copy fails for any reason, the state of `newpath` is undefined. It
    /// might be half a copy, it might be the untouched data of what was already
    /// there, or it might be a zero-byte file, etc.
    ///
    /// ### Parameters
    /// - `oldpath`: the old path.
    /// - `newpath`: the new path.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_CopyFile(
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get information about a filesystem path.
    ///
    /// ### Parameters
    /// - `path`: the path to query.
    /// - `info`: a pointer filled in with information about the path, or NULL to
    ///   check for the existence of a file.
    ///
    /// ### Return value
    /// Returns true on success or false if the file doesn't exist, or another
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
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
    /// `flags` may be set to [`SDL_GLOB_CASEINSENSITIVE`] to make the pattern matching
    /// case-insensitive.
    ///
    /// The returned array is always NULL-terminated, for your iterating
    /// convenience, but if `count` is non-NULL, on return it will contain the
    /// number of items in the array, not counting the NULL terminator.
    ///
    /// ### Parameters
    /// - `path`: the path of the directory to enumerate.
    /// - `pattern`: the pattern that files in the directory must match. Can be
    ///   NULL.
    /// - `flags`: `SDL_GLOB_*` bitflags that affect this search.
    /// - `count`: on return, will be set to the number of items in the returned
    ///   array. Can be NULL.
    ///
    /// ### Return value
    /// Returns an array of strings on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information. This is a single allocation
    ///   that should be freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GlobDirectory(
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        flags: SDL_GlobFlags,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
}

#[cfg(doc)]
use crate::everything::*;
