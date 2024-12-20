//! SDL provides an abstract interface for reading and writing data streams. It
//! offers implementations for files, memory, etc, and the app can provideo
//! their own implementations, too.
//!
//! [`SDL_IOStream`] is not related to the standard C++ iostream class, other than
//! both are abstract interfaces to read/write data.

use super::stdinc::*;

use super::error::*;

use super::properties::*;

/// [`SDL_IOStream`] status, set by a read or write operation.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`READY`](SDL_IOStatus::READY) | [`SDL_IO_STATUS_READY`] | Everything is ready (no errors and not EOF). |
/// | [`ERROR`](SDL_IOStatus::ERROR) | [`SDL_IO_STATUS_ERROR`] | Read or write I/O error |
/// | [`EOF`](SDL_IOStatus::EOF) | [`SDL_IO_STATUS_EOF`] | End of file |
/// | [`NOT_READY`](SDL_IOStatus::NOT_READY) | [`SDL_IO_STATUS_NOT_READY`] | Non blocking I/O, not ready |
/// | [`READONLY`](SDL_IOStatus::READONLY) | [`SDL_IO_STATUS_READONLY`] | Tried to write a read-only buffer |
/// | [`WRITEONLY`](SDL_IOStatus::WRITEONLY) | [`SDL_IO_STATUS_WRITEONLY`] | Tried to read a write-only buffer |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_IOStatus(pub ::core::ffi::c_int);

impl From<SDL_IOStatus> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_IOStatus) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_IOStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::READY => "SDL_IO_STATUS_READY",
            Self::ERROR => "SDL_IO_STATUS_ERROR",
            Self::EOF => "SDL_IO_STATUS_EOF",
            Self::NOT_READY => "SDL_IO_STATUS_NOT_READY",
            Self::READONLY => "SDL_IO_STATUS_READONLY",
            Self::WRITEONLY => "SDL_IO_STATUS_WRITEONLY",

            _ => return write!(f, "SDL_IOStatus({})", self.0),
        })
    }
}

impl SDL_IOStatus {
    /// Everything is ready (no errors and not EOF).
    pub const READY: Self = Self(0);
    /// Read or write I/O error
    pub const ERROR: Self = Self(1);
    /// End of file
    pub const EOF: Self = Self(2);
    /// Non blocking I/O, not ready
    pub const NOT_READY: Self = Self(3);
    /// Tried to write a read-only buffer
    pub const READONLY: Self = Self(4);
    /// Tried to read a write-only buffer
    pub const WRITEONLY: Self = Self(5);
}

/// Everything is ready (no errors and not EOF).
pub const SDL_IO_STATUS_READY: SDL_IOStatus = SDL_IOStatus::READY;
/// Read or write I/O error
pub const SDL_IO_STATUS_ERROR: SDL_IOStatus = SDL_IOStatus::ERROR;
/// End of file
pub const SDL_IO_STATUS_EOF: SDL_IOStatus = SDL_IOStatus::EOF;
/// Non blocking I/O, not ready
pub const SDL_IO_STATUS_NOT_READY: SDL_IOStatus = SDL_IOStatus::NOT_READY;
/// Tried to write a read-only buffer
pub const SDL_IO_STATUS_READONLY: SDL_IOStatus = SDL_IOStatus::READONLY;
/// Tried to read a write-only buffer
pub const SDL_IO_STATUS_WRITEONLY: SDL_IOStatus = SDL_IOStatus::WRITEONLY;

/// Possible `whence` values for [`SDL_IOStream`] seeking.
///
/// These map to the same "whence" concept that `fseek` or `lseek` use in the
/// standard C runtime.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`SET`](SDL_IOWhence::SET) | [`SDL_IO_SEEK_SET`] | Seek from the beginning of data |
/// | [`CUR`](SDL_IOWhence::CUR) | [`SDL_IO_SEEK_CUR`] | Seek relative to current read point |
/// | [`END`](SDL_IOWhence::END) | [`SDL_IO_SEEK_END`] | Seek relative to the end of data |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_IOWhence(pub ::core::ffi::c_int);

impl From<SDL_IOWhence> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_IOWhence) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_IOWhence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::SET => "SDL_IO_SEEK_SET",
            Self::CUR => "SDL_IO_SEEK_CUR",
            Self::END => "SDL_IO_SEEK_END",

            _ => return write!(f, "SDL_IOWhence({})", self.0),
        })
    }
}

impl SDL_IOWhence {
    /// Seek from the beginning of data
    pub const SET: Self = Self(0);
    /// Seek relative to current read point
    pub const CUR: Self = Self(1);
    /// Seek relative to the end of data
    pub const END: Self = Self(2);
}

/// Seek from the beginning of data
pub const SDL_IO_SEEK_SET: SDL_IOWhence = SDL_IOWhence::SET;
/// Seek relative to current read point
pub const SDL_IO_SEEK_CUR: SDL_IOWhence = SDL_IOWhence::CUR;
/// Seek relative to the end of data
pub const SDL_IO_SEEK_END: SDL_IOWhence = SDL_IOWhence::END;

/// The function pointers that drive an [`SDL_IOStream`].
///
/// Applications can provide this struct to [`SDL_OpenIO()`] to create their own
/// implementation of [`SDL_IOStream`]. This is not necessarily required, as SDL
/// already offers several common types of I/O streams, via functions like
/// [`SDL_IOFromFile()`] and [`SDL_IOFromMem()`].
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
pub struct SDL_IOStreamInterface {
    pub version: Uint32,
    /// Return the number of bytes in this [`SDL_IOStream`]
    ///
    /// \return the total size of the data stream, or -1 on error.
    pub size: ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void) -> Sint64>,
    /// Seek to `offset` relative to `whence`, one of stdio's whence values:
    /// [`SDL_IO_SEEK_SET`], [`SDL_IO_SEEK_CUR`], [`SDL_IO_SEEK_END`]
    ///
    /// \return the final offset in the data stream, or -1 on error.
    pub seek: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            offset: Sint64,
            whence: SDL_IOWhence,
        ) -> Sint64,
    >,
    /// Read up to `size` bytes from the data stream to the area pointed
    /// at by `ptr`.
    ///
    /// On an incomplete read, you should set `*status` to a value from the
    /// [`SDL_IOStatus`] enum. You do not have to explicitly set this on
    /// a complete, successful read.
    ///
    /// \return the number of bytes read
    pub read: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            ptr: *mut ::core::ffi::c_void,
            size: ::core::primitive::usize,
            status: *mut SDL_IOStatus,
        ) -> ::core::primitive::usize,
    >,
    /// Write exactly `size` bytes from the area pointed at by `ptr`
    /// to data stream.
    ///
    /// On an incomplete write, you should set `*status` to a value from the
    /// [`SDL_IOStatus`] enum. You do not have to explicitly set this on
    /// a complete, successful write.
    ///
    /// \return the number of bytes written
    pub write: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            ptr: *const ::core::ffi::c_void,
            size: ::core::primitive::usize,
            status: *mut SDL_IOStatus,
        ) -> ::core::primitive::usize,
    >,
    /// If the stream is buffering, make sure the data is written out.
    ///
    /// On failure, you should set `*status` to a value from the
    /// [`SDL_IOStatus`] enum. You do not have to explicitly set this on
    /// a successful flush.
    ///
    /// \return true if successful or false on write error when flushing data.
    pub flush: ::core::option::Option<
        extern "C" fn(
            userdata: *mut ::core::ffi::c_void,
            status: *mut SDL_IOStatus,
        ) -> ::core::primitive::bool,
    >,
    /// Close and free any allocated resources.
    ///
    /// This does not guarantee file writes will sync to physical media; they
    /// can be in the system's file cache, waiting to go to disk.
    ///
    /// The [`SDL_IOStream`] is still destroyed even if this fails, so clean up anything
    /// even if flushing buffers, etc, returns an error.
    ///
    /// \return true if successful or false on write error when flushing data.
    pub close: ::core::option::Option<
        extern "C" fn(userdata: *mut ::core::ffi::c_void) -> ::core::primitive::bool,
    >,
}

impl SDL_IOStreamInterface {
    /// Create a new `SDL_IOStreamInterface` initialized with `SDL_INIT_INTERFACE`
    #[inline]
    pub const fn new() -> Self {
        ::core::assert!(::core::mem::size_of::<Self>() <= ::core::primitive::u32::MAX as usize);
        let mut this = unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        this.version = ::core::mem::size_of::<Self>() as ::core::primitive::u32;
        this
    }
}

impl ::core::default::Default for SDL_IOStreamInterface {
    /// Create a new `SDL_IOStreamInterface` initialized with `SDL_INIT_INTERFACE`
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const _: () = ::core::assert!(
    (((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 4_usize)
        && (::core::mem::size_of::<SDL_IOStreamInterface>() == 28_usize))
        || ((::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8_usize)
            && (::core::mem::size_of::<SDL_IOStreamInterface>() == 56_usize)))
);

extern "C" {
    /// Use this function to create a new [`SDL_IOStream`] structure for reading from
    /// and/or writing to a named file.
    ///
    /// The `mode` string is treated roughly the same as in a call to the C
    /// library's fopen(), even if SDL doesn't happen to use fopen() behind the
    /// scenes.
    ///
    /// Available `mode` strings:
    ///
    /// - "r": Open a file for reading. The file must exist.
    /// - "w": Create an empty file for writing. If a file with the same name
    ///   already exists its content is erased and the file is treated as a new
    ///   empty file.
    /// - "a": Append to a file. Writing operations append data at the end of the
    ///   file. The file is created if it does not exist.
    /// - "r+": Open a file for update both reading and writing. The file must
    ///   exist.
    /// - "w+": Create an empty file for both reading and writing. If a file with
    ///   the same name already exists its content is erased and the file is
    ///   treated as a new empty file.
    /// - "a+": Open a file for reading and appending. All writing operations are
    ///   performed at the end of the file, protecting the previous content to be
    ///   overwritten. You can reposition (fseek, rewind) the internal pointer to
    ///   anywhere in the file for reading, but writing operations will move it
    ///   back to the end of file. The file is created if it does not exist.
    ///
    /// **NOTE**: In order to open a file as a binary file, a "b" character has to
    /// be included in the `mode` string. This additional "b" character can either
    /// be appended at the end of the string (thus making the following compound
    /// modes: "rb", "wb", "ab", "r+b", "w+b", "a+b") or be inserted between the
    /// letter and the "+" sign for the mixed modes ("rb+", "wb+", "ab+").
    /// Additional characters may follow the sequence, although they should have no
    /// effect. For example, "t" is sometimes appended to make explicit the file is
    /// a text file.
    ///
    /// This function supports Unicode filenames, but they must be encoded in UTF-8
    /// format, regardless of the underlying operating system.
    ///
    /// In Android, [`SDL_IOFromFile()`] can be used to open content:// URIs. As a
    /// fallback, [`SDL_IOFromFile()`] will transparently open a matching filename in
    /// the app's `assets`.
    ///
    /// Closing the [`SDL_IOStream`] will close SDL's internal file handle.
    ///
    /// The following properties may be set at creation time by SDL:
    ///
    /// - [`SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER`]: a pointer, that can be cast
    ///   to a win32 `HANDLE`, that this [`SDL_IOStream`] is using to access the
    ///   filesystem. If the program isn't running on Windows, or SDL used some
    ///   other method to access the filesystem, this property will not be set.
    /// - [`SDL_PROP_IOSTREAM_STDIO_FILE_POINTER`]: a pointer, that can be cast to a
    ///   stdio `FILE *`, that this [`SDL_IOStream`] is using to access the filesystem.
    ///   If SDL used some other method to access the filesystem, this property
    ///   will not be set. PLEASE NOTE that if SDL is using a different C runtime
    ///   than your app, trying to use this pointer will almost certainly result in
    ///   a crash! This is mostly a problem on Windows; make sure you build SDL and
    ///   your app with the same compiler and settings to avoid it.
    /// - [`SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER`]: a file descriptor that this
    ///   [`SDL_IOStream`] is using to access the filesystem.
    /// - [`SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER`]: a pointer, that can be cast
    ///   to an Android NDK `AAsset *`, that this [`SDL_IOStream`] is using to access
    ///   the filesystem. If SDL used some other method to access the filesystem,
    ///   this property will not be set.
    ///
    /// ### Parameters
    /// - `file`: a UTF-8 string representing the filename to open.
    /// - `mode`: an ASCII string representing the mode to be used for opening
    ///   the file.
    ///
    /// ### Return value
    /// Returns a pointer to the [`SDL_IOStream`] structure that is created or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseIO`]
    /// - [`SDL_FlushIO`]
    /// - [`SDL_ReadIO`]
    /// - [`SDL_SeekIO`]
    /// - [`SDL_TellIO`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_IOFromFile(
        file: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut SDL_IOStream;
}

pub const SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER: *const ::core::ffi::c_char =
    c"SDL.iostream.windows.handle".as_ptr();

pub const SDL_PROP_IOSTREAM_STDIO_FILE_POINTER: *const ::core::ffi::c_char =
    c"SDL.iostream.stdio.file".as_ptr();

pub const SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER: *const ::core::ffi::c_char =
    c"SDL.iostream.file_descriptor".as_ptr();

pub const SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER: *const ::core::ffi::c_char =
    c"SDL.iostream.android.aasset".as_ptr();

extern "C" {
    /// Use this function to prepare a read-write memory buffer for use with
    /// [`SDL_IOStream`].
    ///
    /// This function sets up an [`SDL_IOStream`] struct based on a memory area of a
    /// certain size, for both read and write access.
    ///
    /// This memory buffer is not copied by the [`SDL_IOStream`]; the pointer you
    /// provide must remain valid until you close the stream. Closing the stream
    /// will not free the original buffer.
    ///
    /// If you need to make sure the [`SDL_IOStream`] never writes to the memory
    /// buffer, you should use [`SDL_IOFromConstMem()`] with a read-only buffer of
    /// memory instead.
    ///
    /// The following properties will be set at creation time by SDL:
    ///
    /// - [`SDL_PROP_IOSTREAM_MEMORY_POINTER`]: this will be the `mem` parameter that
    ///   was passed to this function.
    /// - [`SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER`]: this will be the `size` parameter
    ///   that was passed to this function.
    ///
    /// ### Parameters
    /// - `mem`: a pointer to a buffer to feed an [`SDL_IOStream`] stream.
    /// - `size`: the buffer size, in bytes.
    ///
    /// ### Return value
    /// Returns a pointer to a new [`SDL_IOStream`] structure or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_IOFromConstMem`]
    /// - [`SDL_CloseIO`]
    /// - [`SDL_FlushIO`]
    /// - [`SDL_ReadIO`]
    /// - [`SDL_SeekIO`]
    /// - [`SDL_TellIO`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_IOFromMem(
        mem: *mut ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> *mut SDL_IOStream;
}

pub const SDL_PROP_IOSTREAM_MEMORY_POINTER: *const ::core::ffi::c_char =
    c"SDL.iostream.memory.base".as_ptr();

pub const SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER: *const ::core::ffi::c_char =
    c"SDL.iostream.memory.size".as_ptr();

extern "C" {
    /// Use this function to prepare a read-only memory buffer for use with
    /// [`SDL_IOStream`].
    ///
    /// This function sets up an [`SDL_IOStream`] struct based on a memory area of a
    /// certain size. It assumes the memory area is not writable.
    ///
    /// Attempting to write to this [`SDL_IOStream`] stream will report an error
    /// without writing to the memory buffer.
    ///
    /// This memory buffer is not copied by the [`SDL_IOStream`]; the pointer you
    /// provide must remain valid until you close the stream. Closing the stream
    /// will not free the original buffer.
    ///
    /// If you need to write to a memory buffer, you should use [`SDL_IOFromMem()`]
    /// with a writable buffer of memory instead.
    ///
    /// The following properties will be set at creation time by SDL:
    ///
    /// - [`SDL_PROP_IOSTREAM_MEMORY_POINTER`]: this will be the `mem` parameter that
    ///   was passed to this function.
    /// - [`SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER`]: this will be the `size` parameter
    ///   that was passed to this function.
    ///
    /// ### Parameters
    /// - `mem`: a pointer to a read-only buffer to feed an [`SDL_IOStream`] stream.
    /// - `size`: the buffer size, in bytes.
    ///
    /// ### Return value
    /// Returns a pointer to a new [`SDL_IOStream`] structure or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_IOFromMem`]
    /// - [`SDL_CloseIO`]
    /// - [`SDL_ReadIO`]
    /// - [`SDL_SeekIO`]
    /// - [`SDL_TellIO`]
    pub fn SDL_IOFromConstMem(
        mem: *const ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> *mut SDL_IOStream;
}

extern "C" {
    /// Use this function to create an [`SDL_IOStream`] that is backed by dynamically
    /// allocated memory.
    ///
    /// This supports the following properties to provide access to the memory and
    /// control over allocations:
    ///
    /// - [`SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER`]: a pointer to the internal
    ///   memory of the stream. This can be set to NULL to transfer ownership of
    ///   the memory to the application, which should free the memory with
    ///   [`SDL_free()`]. If this is done, the next operation on the stream must be
    ///   [`SDL_CloseIO()`].
    /// - [`SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER`]: memory will be allocated in
    ///   multiples of this size, defaulting to 1024.
    ///
    /// ### Return value
    /// Returns a pointer to a new [`SDL_IOStream`] structure or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseIO`]
    /// - [`SDL_ReadIO`]
    /// - [`SDL_SeekIO`]
    /// - [`SDL_TellIO`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_IOFromDynamicMem() -> *mut SDL_IOStream;
}

pub const SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER: *const ::core::ffi::c_char =
    c"SDL.iostream.dynamic.memory".as_ptr();

pub const SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER: *const ::core::ffi::c_char =
    c"SDL.iostream.dynamic.chunksize".as_ptr();

extern "C" {
    /// Create a custom [`SDL_IOStream`].
    ///
    /// Applications do not need to use this function unless they are providing
    /// their own [`SDL_IOStream`] implementation. If you just need an [`SDL_IOStream`] to
    /// read/write a common data source, you should use the built-in
    /// implementations in SDL, like [`SDL_IOFromFile()`] or [`SDL_IOFromMem()`], etc.
    ///
    /// This function makes a copy of `iface` and the caller does not need to keep
    /// it around after this call.
    ///
    /// ### Parameters
    /// - `iface`: the interface that implements this [`SDL_IOStream`], initialized
    ///   using [`SDL_INIT_INTERFACE()`].
    /// - `userdata`: the pointer that will be passed to the interface functions.
    ///
    /// ### Return value
    /// Returns a pointer to the allocated memory on success or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CloseIO`]
    /// - [`SDL_INIT_INTERFACE`]
    /// - [`SDL_IOFromConstMem`]
    /// - [`SDL_IOFromFile`]
    /// - [`SDL_IOFromMem`]
    pub fn SDL_OpenIO(
        iface: *const SDL_IOStreamInterface,
        userdata: *mut ::core::ffi::c_void,
    ) -> *mut SDL_IOStream;
}

extern "C" {
    /// Close and free an allocated [`SDL_IOStream`] structure.
    ///
    /// [`SDL_CloseIO()`] closes and cleans up the [`SDL_IOStream`] stream. It releases any
    /// resources used by the stream and frees the [`SDL_IOStream`] itself. This
    /// returns true on success, or false if the stream failed to flush to its
    /// output (e.g. to disk).
    ///
    /// Note that if this fails to flush the stream for any reason, this function
    /// reports an error, but the [`SDL_IOStream`] is still invalid once this function
    /// returns.
    ///
    /// This call flushes any buffered writes to the operating system, but there
    /// are no guarantees that those writes have gone to physical media; they might
    /// be in the OS's file cache, waiting to go to disk later. If it's absolutely
    /// crucial that writes go to disk immediately, so they are definitely stored
    /// even if the power fails before the file cache would have caught up, one
    /// should call [`SDL_FlushIO()`] before closing. Note that flushing takes time and
    /// makes the system and your app operate less efficiently, so do so sparingly.
    ///
    /// ### Parameters
    /// - `context`: [`SDL_IOStream`] structure to close.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenIO`]
    pub fn SDL_CloseIO(context: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the properties associated with an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetIOProperties(context: *mut SDL_IOStream) -> SDL_PropertiesID;
}

extern "C" {
    /// Query the stream status of an [`SDL_IOStream`].
    ///
    /// This information can be useful to decide if a short read or write was due
    /// to an error, an EOF, or a non-blocking operation that isn't yet ready to
    /// complete.
    ///
    /// An SDL_IOStream's status is only expected to change after a [`SDL_ReadIO`] or
    /// [`SDL_WriteIO`] call; don't expect it to change if you just call this query
    /// function in a tight loop.
    ///
    /// ### Parameters
    /// - `context`: the [`SDL_IOStream`] to query.
    ///
    /// ### Return value
    /// Returns an [`SDL_IOStatus`] enum with the current state.
    ///
    /// ### Thread safety
    /// This function should not be called at the same time that
    ///   another thread is operating on the same [`SDL_IOStream`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetIOStatus(context: *mut SDL_IOStream) -> SDL_IOStatus;
}

extern "C" {
    /// Use this function to get the size of the data stream in an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `context`: the [`SDL_IOStream`] to get the size of the data stream from.
    ///
    /// ### Return value
    /// Returns the size of the data stream in the [`SDL_IOStream`] on success or a
    ///   negative error code on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetIOSize(context: *mut SDL_IOStream) -> Sint64;
}

extern "C" {
    /// Seek within an [`SDL_IOStream`] data stream.
    ///
    /// This function seeks to byte `offset`, relative to `whence`.
    ///
    /// `whence` may be any of the following values:
    ///
    /// - [`SDL_IO_SEEK_SET`]: seek from the beginning of data
    /// - [`SDL_IO_SEEK_CUR`]: seek relative to current read point
    /// - [`SDL_IO_SEEK_END`]: seek relative to the end of data
    ///
    /// If this stream can not seek, it will return -1.
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    /// - `offset`: an offset in bytes, relative to `whence` location; can be
    ///   negative.
    /// - `whence`: any of [`SDL_IO_SEEK_SET`], [`SDL_IO_SEEK_CUR`],
    ///   [`SDL_IO_SEEK_END`].
    ///
    /// ### Return value
    /// Returns the final offset in the data stream after the seek or -1 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_TellIO`]
    pub fn SDL_SeekIO(context: *mut SDL_IOStream, offset: Sint64, whence: SDL_IOWhence) -> Sint64;
}

extern "C" {
    /// Determine the current read/write offset in an [`SDL_IOStream`] data stream.
    ///
    /// [`SDL_TellIO`] is actually a wrapper function that calls the SDL_IOStream's
    /// `seek` method, with an offset of 0 bytes from [`SDL_IO_SEEK_CUR`], to
    /// simplify application development.
    ///
    /// ### Parameters
    /// - `context`: an [`SDL_IOStream`] data stream object from which to get the
    ///   current offset.
    ///
    /// ### Return value
    /// Returns the current offset in the stream, or -1 if the information can not
    ///   be determined.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SeekIO`]
    pub fn SDL_TellIO(context: *mut SDL_IOStream) -> Sint64;
}

extern "C" {
    /// Read from a data source.
    ///
    /// This function reads up `size` bytes from the data source to the area
    /// pointed at by `ptr`. This function may read less bytes than requested. It
    /// will return zero when the data stream is completely read, and
    /// [`SDL_GetIOStatus()`] will return [`SDL_IO_STATUS_EOF`], or on error, and
    /// [`SDL_GetIOStatus()`] will return [`SDL_IO_STATUS_ERROR`].
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    /// - `ptr`: a pointer to a buffer to read data into.
    /// - `size`: the number of bytes to read from the data source.
    ///
    /// ### Return value
    /// Returns the number of bytes read, or 0 on end of file or other failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_WriteIO`]
    /// - [`SDL_GetIOStatus`]
    pub fn SDL_ReadIO(
        context: *mut SDL_IOStream,
        ptr: *mut ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Write to an [`SDL_IOStream`] data stream.
    ///
    /// This function writes exactly `size` bytes from the area pointed at by `ptr`
    /// to the stream. If this fails for any reason, it'll return less than `size`
    /// to demonstrate how far the write progressed. On success, it returns `size`.
    ///
    /// On error, this function still attempts to write as much as possible, so it
    /// might return a positive value less than the requested write size.
    ///
    /// The caller can use [`SDL_GetIOStatus()`] to determine if the problem is
    /// recoverable, such as a non-blocking write that can simply be retried later,
    /// or a fatal error.
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    /// - `ptr`: a pointer to a buffer containing data to write.
    /// - `size`: the number of bytes to write.
    ///
    /// ### Return value
    /// Returns the number of bytes written, which will be less than `size` on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_IOprintf`]
    /// - [`SDL_ReadIO`]
    /// - [`SDL_SeekIO`]
    /// - [`SDL_FlushIO`]
    /// - [`SDL_GetIOStatus`]
    pub fn SDL_WriteIO(
        context: *mut SDL_IOStream,
        ptr: *const ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Print to an [`SDL_IOStream`] data stream.
    ///
    /// This function does formatted printing to the stream.
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    /// - `fmt`: a printf() style format string.
    /// - `...`: additional parameters matching % tokens in the `fmt` string, if
    ///   any.
    ///
    /// ### Return value
    /// Returns the number of bytes written or 0 on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_IOvprintf`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_IOprintf(
        context: *mut SDL_IOStream,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Print to an [`SDL_IOStream`] data stream.
    ///
    /// This function does formatted printing to the stream.
    ///
    /// ### Parameters
    /// - `context`: a pointer to an [`SDL_IOStream`] structure.
    /// - `fmt`: a printf() style format string.
    /// - `ap`: a variable argument list.
    ///
    /// ### Return value
    /// Returns the number of bytes written or 0 on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_IOprintf`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_IOvprintf(
        context: *mut SDL_IOStream,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Flush any buffered data in the stream.
    ///
    /// This function makes sure that any buffered data is written to the stream.
    /// Normally this isn't necessary but if the stream is a pipe or socket it
    /// guarantees that any pending data is sent.
    ///
    /// ### Parameters
    /// - `context`: [`SDL_IOStream`] structure to flush.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenIO`]
    /// - [`SDL_WriteIO`]
    pub fn SDL_FlushIO(context: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Load all the data from an SDL data stream.
    ///
    /// The data is allocated with a zero byte at the end (null terminated) for
    /// convenience. This extra byte is not included in the value reported via
    /// `datasize`.
    ///
    /// The data should be freed with [`SDL_free()`].
    ///
    /// ### Parameters
    /// - `src`: the [`SDL_IOStream`] to read all available data from.
    /// - `datasize`: a pointer filled in with the number of bytes read, may be
    ///   NULL.
    /// - `closeio`: if true, calls [`SDL_CloseIO()`] on `src` before returning, even
    ///   in the case of an error.
    ///
    /// ### Return value
    /// Returns the data or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LoadFile`]
    pub fn SDL_LoadFile_IO(
        src: *mut SDL_IOStream,
        datasize: *mut ::core::primitive::usize,
        closeio: ::core::primitive::bool,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Load all the data from a file path.
    ///
    /// The data is allocated with a zero byte at the end (null terminated) for
    /// convenience. This extra byte is not included in the value reported via
    /// `datasize`.
    ///
    /// The data should be freed with [`SDL_free()`].
    ///
    /// ### Parameters
    /// - `file`: the path to read all available data from.
    /// - `datasize`: if not NULL, will store the number of bytes read.
    ///
    /// ### Return value
    /// Returns the data or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LoadFile_IO`]
    pub fn SDL_LoadFile(
        file: *const ::core::ffi::c_char,
        datasize: *mut ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Use this function to read a byte from an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `src`: the [`SDL_IOStream`] to read from.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU8(src: *mut SDL_IOStream, value: *mut Uint8) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read a signed byte from an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `src`: the [`SDL_IOStream`] to read from.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS8(src: *mut SDL_IOStream, value: *mut Sint8) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 16 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU16LE(src: *mut SDL_IOStream, value: *mut Uint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 16 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS16LE(src: *mut SDL_IOStream, value: *mut Sint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 16 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU16BE(src: *mut SDL_IOStream, value: *mut Uint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 16 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS16BE(src: *mut SDL_IOStream, value: *mut Sint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 32 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU32LE(src: *mut SDL_IOStream, value: *mut Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 32 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS32LE(src: *mut SDL_IOStream, value: *mut Sint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 32 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU32BE(src: *mut SDL_IOStream, value: *mut Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 32 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS32BE(src: *mut SDL_IOStream, value: *mut Sint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 64 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU64LE(src: *mut SDL_IOStream, value: *mut Uint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 64 bits of little-endian data from an
    /// [`SDL_IOStream`] and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS64LE(src: *mut SDL_IOStream, value: *mut Sint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 64 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadU64BE(src: *mut SDL_IOStream, value: *mut Uint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to read 64 bits of big-endian data from an [`SDL_IOStream`]
    /// and return in native format.
    ///
    /// SDL byteswaps the data only if necessary, so the data returned will be in
    /// the native byte order.
    ///
    /// ### Parameters
    /// - `src`: the stream from which to read data.
    /// - `value`: a pointer filled in with the data read.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_ReadS64BE(src: *mut SDL_IOStream, value: *mut Sint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write a byte to an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `dst`: the [`SDL_IOStream`] to write to.
    /// - `value`: the byte value to write.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU8(dst: *mut SDL_IOStream, value: Uint8) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write a signed byte to an [`SDL_IOStream`].
    ///
    /// ### Parameters
    /// - `dst`: the [`SDL_IOStream`] to write to.
    /// - `value`: the byte value to write.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS8(dst: *mut SDL_IOStream, value: Sint8) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 16 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU16LE(dst: *mut SDL_IOStream, value: Uint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 16 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS16LE(dst: *mut SDL_IOStream, value: Sint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 16 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU16BE(dst: *mut SDL_IOStream, value: Uint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 16 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS16BE(dst: *mut SDL_IOStream, value: Sint16) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 32 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU32LE(dst: *mut SDL_IOStream, value: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 32 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS32LE(dst: *mut SDL_IOStream, value: Sint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 32 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU32BE(dst: *mut SDL_IOStream, value: Uint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 32 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS32BE(dst: *mut SDL_IOStream, value: Sint32) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 64 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU64LE(dst: *mut SDL_IOStream, value: Uint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 64 bits in native format to an [`SDL_IOStream`] as
    /// little-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in little-endian
    /// format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS64LE(dst: *mut SDL_IOStream, value: Sint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 64 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteU64BE(dst: *mut SDL_IOStream, value: Uint64) -> ::core::primitive::bool;
}

extern "C" {
    /// Use this function to write 64 bits in native format to an [`SDL_IOStream`] as
    /// big-endian data.
    ///
    /// SDL byteswaps the data only if necessary, so the application always
    /// specifies native format, and the data written will be in big-endian format.
    ///
    /// ### Parameters
    /// - `dst`: the stream to which data will be written.
    /// - `value`: the data to be written, in native format.
    ///
    /// ### Return value
    /// Returns true on successful write or false on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_WriteS64BE(dst: *mut SDL_IOStream, value: Sint64) -> ::core::primitive::bool;
}

/// The read/write operation structure.
///
/// This operates as an opaque handle. There are several APIs to create various
/// types of I/O streams, or an app can supply an [`SDL_IOStreamInterface`] to
/// [`SDL_OpenIO()`] to provide their own stream implementation behind this
/// struct's abstract interface.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_IOStream {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
