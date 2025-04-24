//! Metadata for items in the `crate::filesystem` module

use super::*;

pub static METADATA_SDL_Folder: Group = Group {
    module: "filesystem",
    kind: GroupKind::Enum,
    name: "SDL_Folder",
    short_name: "Folder",
    doc: Some("The type of the OS-provided default folder for a specific purpose.\n\nNote that the Trash folder isn't included here, because trashing files\nusually involves extra OS-specific functionality to remember the file's\noriginal location.\n\nThe folders supported per platform are:\n\n|             | Windows | macOS/iOS | tvOS | Unix (XDG) | Haiku | Emscripten |\n| ----------- | ------- | --------- | ---- | ---------- | ----- | ---------- |\n| HOME        | X       | X         |      | X          | X     | X          |\n| DESKTOP     | X       | X         |      | X          | X     |            |\n| DOCUMENTS   | X       | X         |      | X          |       |            |\n| DOWNLOADS   | Vista+  | X         |      | X          |       |            |\n| MUSIC       | X       | X         |      | X          |       |            |\n| PICTURES    | X       | X         |      | X          |       |            |\n| PUBLICSHARE |         | X         |      | X          |       |            |\n| SAVEDGAMES  | Vista+  |           |      |            |       |            |\n| SCREENSHOTS | Vista+  |           |      |            |       |            |\n| TEMPLATES   | X       | X         |      | X          |       |            |\n| VIDEOS      | X       | X*        |      | X          |       |            |\n\nNote that on macOS/iOS, the Videos folder is called \"Movies\".\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetUserFolder`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_FOLDER_HOME",
            short_name: "HOME",
            doc: Some("The folder which contains all of the current user's data, preferences, and documents. It usually contains most of the other folders. If a requested folder does not exist, the home folder can be considered a safe fallback to store a user's documents.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_DESKTOP",
            short_name: "DESKTOP",
            doc: Some("The folder of files that are displayed on the desktop. Note that the existence of a desktop folder does not guarantee that the system does show icons on its desktop; certain GNU/Linux distros with a graphical environment may not have desktop icons.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_DOCUMENTS",
            short_name: "DOCUMENTS",
            doc: Some("User document files, possibly application-specific. This is a good place to save a user's projects.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_DOWNLOADS",
            short_name: "DOWNLOADS",
            doc: Some("Standard folder for user files downloaded from the internet.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_MUSIC",
            short_name: "MUSIC",
            doc: Some("Music files that can be played using a standard music player (mp3, ogg...).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_PICTURES",
            short_name: "PICTURES",
            doc: Some("Image files that can be displayed using a standard viewer (png, jpg...).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_PUBLICSHARE",
            short_name: "PUBLICSHARE",
            doc: Some("Files that are meant to be shared with other users on the same computer.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_SAVEDGAMES",
            short_name: "SAVEDGAMES",
            doc: Some("Save files for games.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_SCREENSHOTS",
            short_name: "SCREENSHOTS",
            doc: Some("Application screenshots.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_TEMPLATES",
            short_name: "TEMPLATES",
            doc: Some("Template files to be used when the user requests the desktop environment to create a new file in a certain folder, such as \"New Text File.txt\".  Any file in the Templates folder can be used as a starting point for a new file.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_VIDEOS",
            short_name: "VIDEOS",
            doc: Some("Video files that can be played using a standard video player (mp4, webm...).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FOLDER_COUNT",
            short_name: "COUNT",
            doc: Some("Total number of types in this enum, not a folder type by itself.\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_PathType: Group = Group {
    module: "filesystem",
    kind: GroupKind::Enum,
    name: "SDL_PathType",
    short_name: "PathType",
    doc: Some("Types of filesystem entries.\n\nNote that there may be other sorts of items on a filesystem: devices,\nsymlinks, named pipes, etc. They are currently reported as\n[`SDL_PATHTYPE_OTHER`].\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_PathInfo`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_PATHTYPE_NONE",
            short_name: "NONE",
            doc: Some("path does not exist\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PATHTYPE_FILE",
            short_name: "FILE",
            doc: Some("a normal file\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PATHTYPE_DIRECTORY",
            short_name: "DIRECTORY",
            doc: Some("a directory\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_PATHTYPE_OTHER",
            short_name: "OTHER",
            doc: Some("something completely different like a device node (not a symlink, those are always followed)\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_GlobFlags: Group = Group {
    module: "filesystem",
    kind: GroupKind::Flags,
    name: "SDL_GlobFlags",
    short_name: "GlobFlags",
    doc: Some("Flags for path matching.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GlobDirectory`]\n- [`SDL_GlobStorageDirectory`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GLOB_CASEINSENSITIVE",
            short_name: "CASEINSENSITIVE",
            doc: None,
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_EnumerationResult: Group = Group {
    module: "filesystem",
    kind: GroupKind::Enum,
    name: "SDL_EnumerationResult",
    short_name: "EnumerationResult",
    doc: Some("Possible results from an enumeration callback.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_EnumerateDirectoryCallback`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ENUM_CONTINUE",
            short_name: "CONTINUE",
            doc: Some("Value that requests that enumeration continue.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ENUM_SUCCESS",
            short_name: "SUCCESS",
            doc: Some("Value that requests that enumeration stop, successfully.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ENUM_FAILURE",
            short_name: "FAILURE",
            doc: Some("Value that requests that enumeration stop, as a failure.\n"),
            available_since: None,
        },
    ],
};
