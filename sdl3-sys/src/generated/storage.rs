//! SDL storage container management.

use super::stdinc::*;

use super::error::*;

use super::filesystem::*;

use super::properties::*;

/// Function interface for [`SDL_Storage`].
///
/// Apps that want to supply a custom implementation of [`SDL_Storage`] will fill
/// in all the functions in this struct, and then pass it to [`SDL_OpenStorage`] to
/// create a custom [`SDL_Storage`] object.
///
/// It is not usually necessary to do this; SDL provides standard
/// implementations for many things you might expect to do with an [`SDL_Storage`].
///
/// This structure should be initialized using [`SDL_INIT_INTERFACE()`]
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_INIT_INTERFACE`]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_StorageInterface {
    pub version: Uint32,
    pub close: ::core::option::Option<
        extern "C" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool,
    >,
    pub ready: ::core::option::Option<
        extern "C" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool,
    >,
    pub enumerate: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
            callback: SDL_EnumerateDirectoryCallback,
            callback_userdata: *mut ::core::ffi::c_void,
        ) -> ::core::primitive::bool,
    >,
    pub info: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
            info: *mut SDL_PathInfo,
        ) -> ::core::primitive::bool,
    >,
    pub read_file: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
            destination: *mut ::core::ffi::c_void,
            length: Uint64,
        ) -> ::core::primitive::bool,
    >,
    pub write_file: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
            source: *const ::core::ffi::c_void,
            length: Uint64,
        ) -> ::core::primitive::bool,
    >,
    pub mkdir: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
        ) -> ::core::primitive::bool,
    >,
    pub remove: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            path: *const ::core::ffi::c_char,
        ) -> ::core::primitive::bool,
    >,
    pub rename: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            oldpath: *const ::core::ffi::c_char,
            newpath: *const ::core::ffi::c_char,
        ) -> ::core::primitive::bool,
    >,
    pub copy: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            oldpath: *const ::core::ffi::c_char,
            newpath: *const ::core::ffi::c_char,
        ) -> ::core::primitive::bool,
    >,
    pub space_remaining:
        ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void) -> Uint64>,
}

impl SDL_StorageInterface {
    /// Create a new `SDL_StorageInterface` initialized with `SDL_INIT_INTERFACE`
    #[inline]
    pub const fn new() -> Self {
        ::core::assert!(::core::mem::size_of::<Self>() <= ::core::primitive::u32::MAX as usize);
        let mut this = unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        this.version = ::core::mem::size_of::<Self>() as ::core::primitive::u32;
        this
    }
}

impl ::core::default::Default for SDL_StorageInterface {
    /// Create a new `SDL_StorageInterface` initialized with `SDL_INIT_INTERFACE`
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const _: () = ::core::assert!(
    (((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 4_usize)
        && (::core::mem::size_of::<SDL_StorageInterface>() == 48_usize))
        || ((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8_usize)
            && (::core::mem::size_of::<SDL_StorageInterface>() == 96_usize)))
);

extern "C" {
    /// Opens up a read-only container for the application's filesystem.
    ///
    /// ### Parameters
    /// - `override`: a path to override the backend's default title root.
    /// - `props`: a property list that may contain backend-specific information.
    ///
    /// ### Return value
    /// Returns a title storage container on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseStorage`]
    /// - [`SDL_GetStorageFileSize`]
    /// - [`SDL_OpenUserStorage`]
    /// - [`SDL_ReadStorageFile`]
    pub fn SDL_OpenTitleStorage(
        r#override: *const ::core::ffi::c_char,
        props: SDL_PropertiesID,
    ) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container for a user's unique read/write filesystem.
    ///
    /// While title storage can generally be kept open throughout runtime, user
    /// storage should only be opened when the client is ready to read/write files.
    /// This allows the backend to properly batch file operations and flush them
    /// when the container has been closed; ensuring safe and optimal save I/O.
    ///
    /// ### Parameters
    /// - `org`: the name of your organization.
    /// - `app`: the name of your application.
    /// - `props`: a property list that may contain backend-specific information.
    ///
    /// ### Return value
    /// Returns a user storage container on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseStorage`]
    /// - [`SDL_GetStorageFileSize`]
    /// - [`SDL_GetStorageSpaceRemaining`]
    /// - [`SDL_OpenTitleStorage`]
    /// - [`SDL_ReadStorageFile`]
    /// - [`SDL_StorageReady`]
    /// - [`SDL_WriteStorageFile`]
    pub fn SDL_OpenUserStorage(
        org: *const ::core::ffi::c_char,
        app: *const ::core::ffi::c_char,
        props: SDL_PropertiesID,
    ) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container for local filesystem storage.
    ///
    /// This is provided for development and tools. Portable applications should
    /// use [`SDL_OpenTitleStorage()`] for access to game data and
    /// [`SDL_OpenUserStorage()`] for access to user data.
    ///
    /// ### Parameters
    /// - `path`: the base path prepended to all storage paths, or NULL for no
    ///   base path.
    ///
    /// ### Return value
    /// Returns a filesystem storage container on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseStorage`]
    /// - [`SDL_GetStorageFileSize`]
    /// - [`SDL_GetStorageSpaceRemaining`]
    /// - [`SDL_OpenTitleStorage`]
    /// - [`SDL_OpenUserStorage`]
    /// - [`SDL_ReadStorageFile`]
    /// - [`SDL_WriteStorageFile`]
    pub fn SDL_OpenFileStorage(path: *const ::core::ffi::c_char) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container using a client-provided storage interface.
    ///
    /// Applications do not need to use this function unless they are providing
    /// their own [`SDL_Storage`] implementation. If you just need an [`SDL_Storage`], you
    /// should use the built-in implementations in SDL, like [`SDL_OpenTitleStorage()`]
    /// or [`SDL_OpenUserStorage()`].
    ///
    /// This function makes a copy of `iface` and the caller does not need to keep
    /// it around after this call.
    ///
    /// ### Parameters
    /// - `iface`: the interface that implements this storage, initialized using
    ///   [`SDL_INIT_INTERFACE()`].
    /// - `userdata`: the pointer that will be passed to the interface functions.
    ///
    /// ### Return value
    /// Returns a storage container on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseStorage`]
    /// - [`SDL_GetStorageFileSize`]
    /// - [`SDL_GetStorageSpaceRemaining`]
    /// - [`SDL_INIT_INTERFACE`]
    /// - [`SDL_ReadStorageFile`]
    /// - [`SDL_StorageReady`]
    /// - [`SDL_WriteStorageFile`]
    pub fn SDL_OpenStorage(
        iface: *const SDL_StorageInterface,
        userdata: *mut ::core::ffi::c_void,
    ) -> *mut SDL_Storage;
}

extern "C" {
    /// Closes and frees a storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to close.
    ///
    /// ### Return value
    /// Returns true if the container was freed with no errors, false otherwise;
    ///   call [`SDL_GetError()`] for more information. Even if the function
    ///   returns an error, the container data will be freed; the error is
    ///   only for informational purposes.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenFileStorage`]
    /// - [`SDL_OpenStorage`]
    /// - [`SDL_OpenTitleStorage`]
    /// - [`SDL_OpenUserStorage`]
    pub fn SDL_CloseStorage(storage: *mut SDL_Storage) -> ::core::primitive::bool;
}

extern "C" {
    /// Checks if the storage container is ready to use.
    ///
    /// This function should be called in regular intervals until it returns true -
    /// however, it is not recommended to spinwait on this call, as the backend may
    /// depend on a synchronous message loop.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to query.
    ///
    /// ### Return value
    /// Returns true if the container is ready, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_StorageReady(storage: *mut SDL_Storage) -> ::core::primitive::bool;
}

extern "C" {
    /// Query the size of a file within a storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to query.
    /// - `path`: the relative path of the file to query.
    /// - `length`: a pointer to be filled with the file's length.
    ///
    /// ### Return value
    /// Returns true if the file could be queried or false on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_ReadStorageFile`]
    /// - [`SDL_StorageReady`]
    pub fn SDL_GetStorageFileSize(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        length: *mut Uint64,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Synchronously read a file from a storage container into a client-provided
    /// buffer.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to read from.
    /// - `path`: the relative path of the file to read.
    /// - `destination`: a client-provided buffer to read the file into.
    /// - `length`: the length of the destination buffer.
    ///
    /// ### Return value
    /// Returns true if the file was read or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetStorageFileSize`]
    /// - [`SDL_StorageReady`]
    /// - [`SDL_WriteStorageFile`]
    pub fn SDL_ReadStorageFile(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        destination: *mut ::core::ffi::c_void,
        length: Uint64,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Synchronously write a file from client memory into a storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to write to.
    /// - `path`: the relative path of the file to write.
    /// - `source`: a client-provided buffer to write from.
    /// - `length`: the length of the source buffer.
    ///
    /// ### Return value
    /// Returns true if the file was written or false on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetStorageSpaceRemaining`]
    /// - [`SDL_ReadStorageFile`]
    /// - [`SDL_StorageReady`]
    pub fn SDL_WriteStorageFile(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        source: *const ::core::ffi::c_void,
        length: Uint64,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Create a directory in a writable storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container.
    /// - `path`: the path of the directory to create.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_CreateStorageDirectory(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Enumerate a directory in a storage container through a callback function.
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
    /// - `storage`: a storage container.
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
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_EnumerateStorageDirectory(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        callback: SDL_EnumerateDirectoryCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Remove a file or an empty directory in a writable storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container.
    /// - `path`: the path of the directory to enumerate.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_RemoveStoragePath(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Rename a file or directory in a writable storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container.
    /// - `oldpath`: the old path.
    /// - `newpath`: the new path.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_RenameStoragePath(
        storage: *mut SDL_Storage,
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Copy a file in a writable storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container.
    /// - `oldpath`: the old path.
    /// - `newpath`: the new path.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_CopyStorageFile(
        storage: *mut SDL_Storage,
        oldpath: *const ::core::ffi::c_char,
        newpath: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get information about a filesystem path in a storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container.
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
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    pub fn SDL_GetStoragePathInfo(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        info: *mut SDL_PathInfo,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Queries the remaining space in a storage container.
    ///
    /// ### Parameters
    /// - `storage`: a storage container to query.
    ///
    /// ### Return value
    /// Returns the amount of remaining space, in bytes.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_StorageReady`]
    /// - [`SDL_WriteStorageFile`]
    pub fn SDL_GetStorageSpaceRemaining(storage: *mut SDL_Storage) -> Uint64;
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
    /// - `storage`: a storage container.
    /// - `path`: the path of the directory to enumerate.
    /// - `pattern`: the pattern that files in the directory must match. Can be
    ///   NULL.
    /// - `flags`: `SDL_GLOB_*` bitflags that affect this search.
    /// - `count`: on return, will be set to the number of items in the returned
    ///   array. Can be NULL.
    ///
    /// ### Return value
    /// Returns an array of strings on success or NULL on failure; call
    ///   [`SDL_GetError()`] for more information. The caller should pass the
    ///   returned pointer to [`SDL_free`] when done with it. This is a single
    ///   allocation that should be freed with [`SDL_free()`] when it is no
    ///   longer needed.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, assuming
    ///   the `storage` object is thread-safe.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GlobStorageDirectory(
        storage: *mut SDL_Storage,
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        flags: SDL_GlobFlags,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
}

/// An abstract interface for filesystem access.
///
/// This is an opaque datatype. One can create this object using standard SDL
/// functions like [`SDL_OpenTitleStorage`] or [`SDL_OpenUserStorage`], etc, or create
/// an object with a custom implementation using [`SDL_OpenStorage`].
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Storage {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
