#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]

//! # CategoryStorage
//!
//! SDL storage container management.

use super::stdinc::*;

use super::error::*;

use super::filesystem::*;

use super::properties::*;

/// Function interface for SDL_Storage.
///
/// Apps that want to supply a custom implementation of SDL_Storage will fill
/// in all the functions in this struct, and then pass it to SDL_OpenStorage to
/// create a custom SDL_Storage object.
///
/// It is not usually necessary to do this; SDL provides standard
/// implementations for many things you might expect to do with an SDL_Storage.
///
/// This structure should be initialized using SDL_INIT_INTERFACE()
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_INIT_INTERFACE
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_StorageInterface {
    pub version: Uint32,
    pub close: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void) -> SDL_bool>,
    pub ready: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void) -> SDL_bool>,
    pub enumerate: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, callback: SDL_EnumerateDirectoryCallback, callback_userdata: *mut ::core::ffi::c_void) -> SDL_bool>,
    pub info: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, info: *mut SDL_PathInfo) -> SDL_bool>,
    pub read_file: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, destination: *mut ::core::ffi::c_void, length: Uint64) -> SDL_bool>,
    pub write_file: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char, source: *const ::core::ffi::c_void, length: Uint64) -> SDL_bool>,
    pub mkdir: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char) -> SDL_bool>,
    pub remove: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, path: *const ::core::ffi::c_char) -> SDL_bool>,
    pub rename: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> SDL_bool>,
    pub copy: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> SDL_bool>,
    pub space_remaining: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void) -> Uint64>,
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

impl crate::sealed_interface::Sealed for SDL_StorageInterface {}
unsafe impl crate::Interface for SDL_StorageInterface {}

const _: () = ::core::assert!(((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 4 && ::core::mem::size_of::<SDL_StorageInterface>() == 48) || (::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8 && ::core::mem::size_of::<SDL_StorageInterface>() == 96)));

extern "C" {
    /// Opens up a read-only container for the application's filesystem.
    ///
    /// \param override a path to override the backend's default title root.
    /// \param props a property list that may contain backend-specific information.
    /// \returns a title storage container on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CloseStorage
    /// \sa SDL_GetStorageFileSize
    /// \sa SDL_OpenUserStorage
    /// \sa SDL_ReadStorageFile
    pub fn SDL_OpenTitleStorage(r#override: *const ::core::ffi::c_char, props: SDL_PropertiesID) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container for a user's unique read/write filesystem.
    ///
    /// While title storage can generally be kept open throughout runtime, user
    /// storage should only be opened when the client is ready to read/write files.
    /// This allows the backend to properly batch file operations and flush them
    /// when the container has been closed; ensuring safe and optimal save I/O.
    ///
    /// \param org the name of your organization.
    /// \param app the name of your application.
    /// \param props a property list that may contain backend-specific information.
    /// \returns a user storage container on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CloseStorage
    /// \sa SDL_GetStorageFileSize
    /// \sa SDL_GetStorageSpaceRemaining
    /// \sa SDL_OpenTitleStorage
    /// \sa SDL_ReadStorageFile
    /// \sa SDL_StorageReady
    /// \sa SDL_WriteStorageFile
    pub fn SDL_OpenUserStorage(org: *const ::core::ffi::c_char, app: *const ::core::ffi::c_char, props: SDL_PropertiesID) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container for local filesystem storage.
    ///
    /// This is provided for development and tools. Portable applications should
    /// use SDL_OpenTitleStorage() for access to game data and
    /// SDL_OpenUserStorage() for access to user data.
    ///
    /// \param path the base path prepended to all storage paths, or NULL for no
    ///             base path.
    /// \returns a filesystem storage container on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CloseStorage
    /// \sa SDL_GetStorageFileSize
    /// \sa SDL_GetStorageSpaceRemaining
    /// \sa SDL_OpenTitleStorage
    /// \sa SDL_OpenUserStorage
    /// \sa SDL_ReadStorageFile
    /// \sa SDL_WriteStorageFile
    pub fn SDL_OpenFileStorage(path: *const ::core::ffi::c_char) -> *mut SDL_Storage;
}

extern "C" {
    /// Opens up a container using a client-provided storage interface.
    ///
    /// Applications do not need to use this function unless they are providing
    /// their own SDL_Storage implementation. If you just need an SDL_Storage, you
    /// should use the built-in implementations in SDL, like SDL_OpenTitleStorage()
    /// or SDL_OpenUserStorage().
    ///
    /// This function makes a copy of `iface` and the caller does not need to keep
    /// it around after this call.
    ///
    /// \param iface the interface that implements this storage, initialized using
    ///              SDL_INIT_INTERFACE().
    /// \param userdata the pointer that will be passed to the interface functions.
    /// \returns a storage container on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CloseStorage
    /// \sa SDL_GetStorageFileSize
    /// \sa SDL_GetStorageSpaceRemaining
    /// \sa SDL_INIT_INTERFACE
    /// \sa SDL_ReadStorageFile
    /// \sa SDL_StorageReady
    /// \sa SDL_WriteStorageFile
    pub fn SDL_OpenStorage(iface: *const SDL_StorageInterface, userdata: *mut ::core::ffi::c_void) -> *mut SDL_Storage;
}

extern "C" {
    /// Closes and frees a storage container.
    ///
    /// \param storage a storage container to close.
    /// \returns SDL_TRUE if the container was freed with no errors, SDL_FALSE
    ///          otherwise; call SDL_GetError() for more information. Even if the
    ///          function returns an error, the container data will be freed; the
    ///          error is only for informational purposes.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_OpenFileStorage
    /// \sa SDL_OpenStorage
    /// \sa SDL_OpenTitleStorage
    /// \sa SDL_OpenUserStorage
    pub fn SDL_CloseStorage(storage: *mut SDL_Storage) -> SDL_bool;
}

extern "C" {
    /// Checks if the storage container is ready to use.
    ///
    /// This function should be called in regular intervals until it returns
    /// SDL_TRUE - however, it is not recommended to spinwait on this call, as the
    /// backend may depend on a synchronous message loop.
    ///
    /// \param storage a storage container to query.
    /// \returns SDL_TRUE if the container is ready, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_StorageReady(storage: *mut SDL_Storage) -> SDL_bool;
}

extern "C" {
    /// Query the size of a file within a storage container.
    ///
    /// \param storage a storage container to query.
    /// \param path the relative path of the file to query.
    /// \param length a pointer to be filled with the file's length.
    /// \returns SDL_TRUE if the file could be queried or SDL_FALSE on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ReadStorageFile
    /// \sa SDL_StorageReady
    pub fn SDL_GetStorageFileSize(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, length: *mut Uint64) -> SDL_bool;
}

extern "C" {
    /// Synchronously read a file from a storage container into a client-provided
    /// buffer.
    ///
    /// \param storage a storage container to read from.
    /// \param path the relative path of the file to read.
    /// \param destination a client-provided buffer to read the file into.
    /// \param length the length of the destination buffer.
    /// \returns SDL_TRUE if the file was read or SDL_FALSE on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetStorageFileSize
    /// \sa SDL_StorageReady
    /// \sa SDL_WriteStorageFile
    pub fn SDL_ReadStorageFile(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, destination: *mut ::core::ffi::c_void, length: Uint64) -> SDL_bool;
}

extern "C" {
    /// Synchronously write a file from client memory into a storage container.
    ///
    /// \param storage a storage container to write to.
    /// \param path the relative path of the file to write.
    /// \param source a client-provided buffer to write from.
    /// \param length the length of the source buffer.
    /// \returns SDL_TRUE if the file was written or SDL_FALSE on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetStorageSpaceRemaining
    /// \sa SDL_ReadStorageFile
    /// \sa SDL_StorageReady
    pub fn SDL_WriteStorageFile(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, source: *const ::core::ffi::c_void, length: Uint64) -> SDL_bool;
}

extern "C" {
    /// Create a directory in a writable storage container.
    ///
    /// \param storage a storage container.
    /// \param path the path of the directory to create.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_CreateStorageDirectory(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char) -> SDL_bool;
}

extern "C" {
    /// Enumerate a directory in a storage container through a callback function.
    ///
    /// This function provides every directory entry through an app-provided
    /// callback, called once for each directory entry, until all results have been
    /// provided or the callback returns <= 0.
    ///
    /// \param storage a storage container.
    /// \param path the path of the directory to enumerate.
    /// \param callback a function that is called for each entry in the directory.
    /// \param userdata a pointer that is passed to `callback`.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_EnumerateStorageDirectory(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, callback: SDL_EnumerateDirectoryCallback, userdata: *mut ::core::ffi::c_void) -> SDL_bool;
}

extern "C" {
    /// Remove a file or an empty directory in a writable storage container.
    ///
    /// \param storage a storage container.
    /// \param path the path of the directory to enumerate.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_RemoveStoragePath(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char) -> SDL_bool;
}

extern "C" {
    /// Rename a file or directory in a writable storage container.
    ///
    /// \param storage a storage container.
    /// \param oldpath the old path.
    /// \param newpath the new path.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_RenameStoragePath(storage: *mut SDL_Storage, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> SDL_bool;
}

extern "C" {
    /// Copy a file in a writable storage container.
    ///
    /// \param storage a storage container.
    /// \param oldpath the old path.
    /// \param newpath the new path.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_CopyStorageFile(storage: *mut SDL_Storage, oldpath: *const ::core::ffi::c_char, newpath: *const ::core::ffi::c_char) -> SDL_bool;
}

extern "C" {
    /// Get information about a filesystem path in a storage container.
    ///
    /// \param storage a storage container.
    /// \param path the path to query.
    /// \param info a pointer filled in with information about the path, or NULL to
    ///             check for the existence of a file.
    /// \returns SDL_TRUE on success or SDL_FALSE if the file doesn't exist, or
    ///          another failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    pub fn SDL_GetStoragePathInfo(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, info: *mut SDL_PathInfo) -> SDL_bool;
}

extern "C" {
    /// Queries the remaining space in a storage container.
    ///
    /// \param storage a storage container to query.
    /// \returns the amount of remaining space, in bytes.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_StorageReady
    /// \sa SDL_WriteStorageFile
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
    /// `flags` may be set to SDL_GLOB_CASEINSENSITIVE to make the pattern matching
    /// case-insensitive.
    ///
    /// The returned array is always NULL-terminated, for your iterating
    /// convenience, but if `count` is non-NULL, on return it will contain the
    /// number of items in the array, not counting the NULL terminator.
    ///
    /// \param storage a storage container.
    /// \param path the path of the directory to enumerate.
    /// \param pattern the pattern that files in the directory must match. Can be
    ///                NULL.
    /// \param flags `SDL_GLOB_*` bitflags that affect this search.
    /// \param count on return, will be set to the number of items in the returned
    ///              array. Can be NULL.
    /// \returns an array of strings on success or NULL on failure; call
    ///          SDL_GetError() for more information. The caller should pass the
    ///          returned pointer to SDL_free when done with it. This is a single
    ///          allocation that should be freed with SDL_free() when it is no
    ///          longer needed.
    ///
    /// \threadsafety It is safe to call this function from any thread, assuming
    ///               the `storage` object is thread-safe.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GlobStorageDirectory(storage: *mut SDL_Storage, path: *const ::core::ffi::c_char, pattern: *const ::core::ffi::c_char, flags: SDL_GlobFlags, count: *mut ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
}

/// An abstract interface for filesystem access.
///
/// This is an opaque datatype. One can create this object using standard SDL
/// functions like SDL_OpenTitleStorage or SDL_OpenUserStorage, etc, or create
/// an object with a custom implementation using SDL_OpenStorage.
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Storage { _opaque: [::core::primitive::u8; 0] }

