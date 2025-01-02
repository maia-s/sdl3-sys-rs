//! SDL offers a way to perform I/O asynchronously. This allows an app to read
//! or write files without waiting for data to actually transfer; the functions
//! that request I/O never block while the request is fulfilled.
//!
//! Instead, the data moves in the background and the app can check for results
//! at their leisure.
//!
//! This is more complicated than just reading and writing files in a
//! synchronous way, but it can allow for more efficiency, and never having
//! framerate drops as the hard drive catches up, etc.
//!
//! The general usage pattern for async I/O is:
//!
//! - Create one or more [`SDL_AsyncIOQueue`] objects.
//! - Open files with [`SDL_AsyncIOFromFile`].
//! - Start I/O tasks to the files with [`SDL_ReadAsyncIO`] or [`SDL_WriteAsyncIO`],
//!   putting those tasks into one of the queues.
//! - Later on, use [`SDL_GetAsyncIOResult`] on a queue to see if any task is
//!   finished without blocking. Tasks might finish in any order with success
//!   or failure.
//! - When all your tasks are done, close the file with [`SDL_CloseAsyncIO`]. This
//!   also generates a task, since it might flush data to disk!
//!
//! This all works, without blocking, in a single thread, but one can also wait
//! on a queue in a background thread, sleeping until new results have arrived:
//!
//! - Call [`SDL_WaitAsyncIOResult`] from one or more threads to efficiently block
//!   until new tasks complete.
//! - When shutting down, call [`SDL_SignalAsyncIOQueue`] to unblock any sleeping
//!   threads despite there being no new tasks completed.
//!
//! And, of course, to match the synchronous [`SDL_LoadFile`], we offer
//! [`SDL_LoadFileAsync`] as a convenience function. This will handle allocating a
//! buffer, slurping in the file data, and null-terminating it; you still check
//! for results later.
//!
//! Behind the scenes, SDL will use newer, efficient APIs on platforms that
//! support them: Linux's io_uring and Windows 11's IoRing, for example. If
//! those technologies aren't available, SDL will offload the work to a thread
//! pool that will manage otherwise-synchronous loads without blocking the app.
//!
//! ## Best Practices
//!
//! Simple non-blocking i/o--for an app that just wants to pick up data
//! whenever it's ready without losing framerate waiting on disks to spin--can
//! use whatever pattern works well for the program. In this case, simply call
//! [`SDL_ReadAsyncIO`], or maybe [`SDL_LoadFileAsync`], as needed. Once a frame, call
//! [`SDL_GetAsyncIOResult`] to check for any completed tasks and deal with the
//! data as it arrives.
//!
//! If two separate pieces of the same program need their own i/o, it is legal
//! for each to create their own queue. This will prevent either piece from
//! accidentally consuming the other's completed tasks. Each queue does require
//! some amount of resources, but it is not an overwhelming cost. Do not make a
//! queue for each task, however. It is better to put many tasks into a single
//! queue. They will be reported in order of completion, not in the order they
//! were submitted, so it doesn't generally matter what order tasks are
//! started.
//!
//! One async i/o queue can be shared by multiple threads, or one thread can
//! have more than one queue, but the most efficient way--if ruthless
//! efficiency is the goal--is to have one queue per thread, with multiple
//! threads working in parallel, and attempt to keep each queue loaded with
//! tasks that are both started by and consumed by the same thread. On modern
//! platforms that can use newer interfaces, this can keep data flowing as
//! efficiently as possible all the way from storage hardware to the app, with
//! no contention between threads for access to the same queue.
//!
//! Written data is not guaranteed to make it to physical media by the time a
//! closing task is completed, unless [`SDL_CloseAsyncIO`] is called with its
//! `flush` parameter set to true, which is to say that a successful result
//! here can still result in lost data during an unfortunately-timed power
//! outage if not flushed. However, flushing will take longer and may be
//! unnecessary, depending on the app's needs.

use super::stdinc::*;

/// Types of asynchronous I/O tasks.
///
/// ### Availability
/// This enum is available since SDL 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`READ`](SDL_AsyncIOTaskType::READ) | [`SDL_ASYNCIO_TASK_READ`] | A read operation. |
/// | [`WRITE`](SDL_AsyncIOTaskType::WRITE) | [`SDL_ASYNCIO_TASK_WRITE`] | A write operation. |
/// | [`CLOSE`](SDL_AsyncIOTaskType::CLOSE) | [`SDL_ASYNCIO_TASK_CLOSE`] | A close operation. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_AsyncIOTaskType(pub ::core::ffi::c_int);

impl ::core::default::Default for SDL_AsyncIOTaskType {
    #[inline(always)]
    fn default() -> Self {
        Self::READ
    }
}

impl From<SDL_AsyncIOTaskType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_AsyncIOTaskType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_AsyncIOTaskType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::READ => "SDL_ASYNCIO_TASK_READ",
            Self::WRITE => "SDL_ASYNCIO_TASK_WRITE",
            Self::CLOSE => "SDL_ASYNCIO_TASK_CLOSE",

            _ => return write!(f, "SDL_AsyncIOTaskType({})", self.0),
        })
    }
}

impl SDL_AsyncIOTaskType {
    /// A read operation.
    pub const READ: Self = Self(0);
    /// A write operation.
    pub const WRITE: Self = Self(1);
    /// A close operation.
    pub const CLOSE: Self = Self(2);
}

/// A read operation.
pub const SDL_ASYNCIO_TASK_READ: SDL_AsyncIOTaskType = SDL_AsyncIOTaskType::READ;
/// A write operation.
pub const SDL_ASYNCIO_TASK_WRITE: SDL_AsyncIOTaskType = SDL_AsyncIOTaskType::WRITE;
/// A close operation.
pub const SDL_ASYNCIO_TASK_CLOSE: SDL_AsyncIOTaskType = SDL_AsyncIOTaskType::CLOSE;

/// Possible outcomes of an asynchronous I/O task.
///
/// ### Availability
/// This enum is available since SDL 3.0.0.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`COMPLETE`](SDL_AsyncIOResult::COMPLETE) | [`SDL_ASYNCIO_COMPLETE`] | request was completed without error |
/// | [`FAILURE`](SDL_AsyncIOResult::FAILURE) | [`SDL_ASYNCIO_FAILURE`] | request failed for some reason; check [`SDL_GetError()`]! |
/// | [`CANCELED`](SDL_AsyncIOResult::CANCELED) | [`SDL_ASYNCIO_CANCELED`] | request was canceled before completing. |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_AsyncIOResult(pub ::core::ffi::c_int);

impl ::core::default::Default for SDL_AsyncIOResult {
    #[inline(always)]
    fn default() -> Self {
        Self::COMPLETE
    }
}

impl From<SDL_AsyncIOResult> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_AsyncIOResult) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_AsyncIOResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::COMPLETE => "SDL_ASYNCIO_COMPLETE",
            Self::FAILURE => "SDL_ASYNCIO_FAILURE",
            Self::CANCELED => "SDL_ASYNCIO_CANCELED",

            _ => return write!(f, "SDL_AsyncIOResult({})", self.0),
        })
    }
}

impl SDL_AsyncIOResult {
    /// request was completed without error
    pub const COMPLETE: Self = Self(0);
    /// request failed for some reason; check [`SDL_GetError()`]!
    pub const FAILURE: Self = Self(1);
    /// request was canceled before completing.
    pub const CANCELED: Self = Self(2);
}

/// request was completed without error
pub const SDL_ASYNCIO_COMPLETE: SDL_AsyncIOResult = SDL_AsyncIOResult::COMPLETE;
/// request failed for some reason; check [`SDL_GetError()`]!
pub const SDL_ASYNCIO_FAILURE: SDL_AsyncIOResult = SDL_AsyncIOResult::FAILURE;
/// request was canceled before completing.
pub const SDL_ASYNCIO_CANCELED: SDL_AsyncIOResult = SDL_AsyncIOResult::CANCELED;

/// Information about a completed asynchronous I/O request.
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AsyncIOOutcome {
    /// what generated this task. This pointer will be invalid if it was closed!
    pub asyncio: *mut SDL_AsyncIO,
    /// What sort of task was this? Read, write, etc?
    pub r#type: SDL_AsyncIOTaskType,
    /// the result of the work (success, failure, cancellation).
    pub result: SDL_AsyncIOResult,
    /// buffer where data was read/written.
    pub buffer: *mut ::core::ffi::c_void,
    /// offset in the [`SDL_AsyncIO`] where data was read/written.
    pub offset: Uint64,
    /// number of bytes the task was to read/write.
    pub bytes_requested: Uint64,
    /// actual number of bytes that were read/written.
    pub bytes_transferred: Uint64,
    /// pointer provided by the app when starting the task
    pub userdata: *mut ::core::ffi::c_void,
}

extern "C" {
    /// Use this function to create a new [`SDL_AsyncIO`] object for reading from
    /// and/or writing to a named file.
    ///
    /// The `mode` string understands the following values:
    ///
    /// - "r": Open a file for reading only. It must exist.
    /// - "w": Open a file for writing only. It will create missing files or
    ///   truncate existing ones.
    /// - "r+": Open a file for update both reading and writing. The file must
    ///   exist.
    /// - "w+": Create an empty file for both reading and writing. If a file with
    ///   the same name already exists its content is erased and the file is
    ///   treated as a new empty file.
    ///
    /// There is no "b" mode, as there is only "binary" style I/O, and no "a" mode
    /// for appending, since you specify the position when starting a task.
    ///
    /// This function supports Unicode filenames, but they must be encoded in UTF-8
    /// format, regardless of the underlying operating system.
    ///
    /// This call is _not_ asynchronous; it will open the file before returning,
    /// under the assumption that doing so is generally a fast operation. Future
    /// reads and writes to the opened file will be async, however.
    ///
    /// ### Parameters
    /// - `file`: a UTF-8 string representing the filename to open.
    /// - `mode`: an ASCII string representing the mode to be used for opening
    ///   the file.
    ///
    /// ### Return value
    /// Returns a pointer to the [`SDL_AsyncIO`] structure that is created or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_CloseAsyncIO`]
    /// - [`SDL_ReadAsyncIO`]
    /// - [`SDL_WriteAsyncIO`]
    pub fn SDL_AsyncIOFromFile(
        file: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut SDL_AsyncIO;
}

extern "C" {
    /// Use this function to get the size of the data stream in an [`SDL_AsyncIO`].
    ///
    /// This call is _not_ asynchronous; it assumes that obtaining this info is a
    /// non-blocking operation in most reasonable cases.
    ///
    /// ### Parameters
    /// - `asyncio`: the [`SDL_AsyncIO`] to get the size of the data stream from.
    ///
    /// ### Return value
    /// Returns the size of the data stream in the [`SDL_IOStream`] on success or a
    ///   negative error code on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetAsyncIOSize(asyncio: *mut SDL_AsyncIO) -> Sint64;
}

extern "C" {
    /// Start an async read.
    ///
    /// This function reads up to `size` bytes from `offset` position in the data
    /// source to the area pointed at by `ptr`. This function may read less bytes
    /// than requested.
    ///
    /// This function returns as quickly as possible; it does not wait for the read
    /// to complete. On a successful return, this work will continue in the
    /// background. If the work begins, even failure is asynchronous: a failing
    /// return value from this function only means the work couldn't start at all.
    ///
    /// `ptr` must remain available until the work is done, and may be accessed by
    /// the system at any time until then. Do not allocate it on the stack, as this
    /// might take longer than the life of the calling function to complete!
    ///
    /// An [`SDL_AsyncIOQueue`] must be specified. The newly-created task will be added
    /// to it when it completes its work.
    ///
    /// ### Parameters
    /// - `asyncio`: a pointer to an [`SDL_AsyncIO`] structure.
    /// - `ptr`: a pointer to a buffer to read data into.
    /// - `offset`: the position to start reading in the data source.
    /// - `size`: the number of bytes to read from the data source.
    /// - `queue`: a queue to add the new [`SDL_AsyncIO`] to.
    /// - `userdata`: an app-defined pointer that will be provided with the task
    ///   results.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_WriteAsyncIO`]
    /// - [`SDL_CreateAsyncIOQueue`]
    pub fn SDL_ReadAsyncIO(
        asyncio: *mut SDL_AsyncIO,
        ptr: *mut ::core::ffi::c_void,
        offset: Uint64,
        size: Uint64,
        queue: *mut SDL_AsyncIOQueue,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Start an async write.
    ///
    /// This function writes `size` bytes from `offset` position in the data source
    /// to the area pointed at by `ptr`.
    ///
    /// This function returns as quickly as possible; it does not wait for the
    /// write to complete. On a successful return, this work will continue in the
    /// background. If the work begins, even failure is asynchronous: a failing
    /// return value from this function only means the work couldn't start at all.
    ///
    /// `ptr` must remain available until the work is done, and may be accessed by
    /// the system at any time until then. Do not allocate it on the stack, as this
    /// might take longer than the life of the calling function to complete!
    ///
    /// An [`SDL_AsyncIOQueue`] must be specified. The newly-created task will be added
    /// to it when it completes its work.
    ///
    /// ### Parameters
    /// - `asyncio`: a pointer to an [`SDL_AsyncIO`] structure.
    /// - `ptr`: a pointer to a buffer to write data from.
    /// - `offset`: the position to start writing to the data source.
    /// - `size`: the number of bytes to write to the data source.
    /// - `queue`: a queue to add the new [`SDL_AsyncIO`] to.
    /// - `userdata`: an app-defined pointer that will be provided with the task
    ///   results.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_ReadAsyncIO`]
    /// - [`SDL_CreateAsyncIOQueue`]
    pub fn SDL_WriteAsyncIO(
        asyncio: *mut SDL_AsyncIO,
        ptr: *mut ::core::ffi::c_void,
        offset: Uint64,
        size: Uint64,
        queue: *mut SDL_AsyncIOQueue,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Close and free any allocated resources for an async I/O object.
    ///
    /// Closing a file is _also_ an asynchronous task! If a write failure were to
    /// happen during the closing process, for example, the task results will
    /// report it as usual.
    ///
    /// Closing a file that has been written to does not guarantee the data has
    /// made it to physical media; it may remain in the operating system's file
    /// cache, for later writing to disk. This means that a successfully-closed
    /// file can be lost if the system crashes or loses power in this small window.
    /// To prevent this, call this function with the `flush` parameter set to true.
    /// This will make the operation take longer, and perhaps increase system load
    /// in general, but a successful result guarantees that the data has made it to
    /// physical storage. Don't use this for temporary files, caches, and
    /// unimportant data, and definitely use it for crucial irreplaceable files,
    /// like game saves.
    ///
    /// This function guarantees that the close will happen after any other pending
    /// tasks to `asyncio`, so it's safe to open a file, start several operations,
    /// close the file immediately, then check for all results later. This function
    /// will not block until the tasks have completed.
    ///
    /// Once this function returns true, `asyncio` is no longer valid, regardless
    /// of any future outcomes. Any completed tasks might still contain this
    /// pointer in their [`SDL_AsyncIOOutcome`] data, in case the app was using this
    /// value to track information, but it should not be used again.
    ///
    /// If this function returns false, the close wasn't started at all, and it's
    /// safe to attempt to close again later.
    ///
    /// An [`SDL_AsyncIOQueue`] must be specified. The newly-created task will be added
    /// to it when it completes its work.
    ///
    /// ### Parameters
    /// - `asyncio`: a pointer to an [`SDL_AsyncIO`] structure to close.
    /// - `flush`: true if data should sync to disk before the task completes.
    /// - `queue`: a queue to add the new [`SDL_AsyncIO`] to.
    /// - `userdata`: an app-defined pointer that will be provided with the task
    ///   results.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, but two
    ///   threads should not attempt to close the same object.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_CloseAsyncIO(
        asyncio: *mut SDL_AsyncIO,
        flush: ::core::primitive::bool,
        queue: *mut SDL_AsyncIOQueue,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Create a task queue for tracking multiple I/O operations.
    ///
    /// Async I/O operations are assigned to a queue when started. The queue can be
    /// checked for completed tasks thereafter.
    ///
    /// ### Return value
    /// Returns a new task queue object or NULL if there was an error; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_DestroyAsyncIOQueue`]
    /// - [`SDL_GetAsyncIOResult`]
    /// - [`SDL_WaitAsyncIOResult`]
    pub fn SDL_CreateAsyncIOQueue() -> *mut SDL_AsyncIOQueue;
}

extern "C" {
    /// Destroy a previously-created async I/O task queue.
    ///
    /// If there are still tasks pending for this queue, this call will block until
    /// those tasks are finished. All those tasks will be deallocated. Their
    /// results will be lost to the app.
    ///
    /// Any pending reads from [`SDL_LoadFileAsync()`] that are still in this queue
    /// will have their buffers deallocated by this function, to prevent a memory
    /// leak.
    ///
    /// Once this function is called, the queue is no longer valid and should not
    /// be used, including by other threads that might access it while destruction
    /// is blocking on pending tasks.
    ///
    /// Do not destroy a queue that still has threads waiting on it through
    /// [`SDL_WaitAsyncIOResult()`]. You can call [`SDL_SignalAsyncIOQueue()`] first to
    /// unblock those threads, and take measures (such as [`SDL_WaitThread()`]) to make
    /// sure they have finished their wait and won't wait on the queue again.
    ///
    /// ### Parameters
    /// - `queue`: the task queue to destroy.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, so long as
    ///   no other thread is waiting on the queue with
    ///   [`SDL_WaitAsyncIOResult`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DestroyAsyncIOQueue(queue: *mut SDL_AsyncIOQueue);
}

extern "C" {
    /// Query an async I/O task queue for completed tasks.
    ///
    /// If a task assigned to this queue has finished, this will return true and
    /// fill in `outcome` with the details of the task. If no task in the queue has
    /// finished, this function will return false. This function does not block.
    ///
    /// If a task has completed, this function will free its resources and the task
    /// pointer will no longer be valid. The task will be removed from the queue.
    ///
    /// It is safe for multiple threads to call this function on the same queue at
    /// once; a completed task will only go to one of the threads.
    ///
    /// ### Parameters
    /// - `queue`: the async I/O task queue to query.
    /// - `outcome`: details of a finished task will be written here. May not be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns true if a task has completed, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_WaitAsyncIOResult`]
    pub fn SDL_GetAsyncIOResult(
        queue: *mut SDL_AsyncIOQueue,
        outcome: *mut SDL_AsyncIOOutcome,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Block until an async I/O task queue has a completed task.
    ///
    /// This function puts the calling thread to sleep until there a task assigned
    /// to the queue that has finished.
    ///
    /// If a task assigned to the queue has finished, this will return true and
    /// fill in `outcome` with the details of the task. If no task in the queue has
    /// finished, this function will return false.
    ///
    /// If a task has completed, this function will free its resources and the task
    /// pointer will no longer be valid. The task will be removed from the queue.
    ///
    /// It is safe for multiple threads to call this function on the same queue at
    /// once; a completed task will only go to one of the threads.
    ///
    /// Note that by the nature of various platforms, more than one waiting thread
    /// may wake to handle a single task, but only one will obtain it, so
    /// `timeoutMS` is a _maximum_ wait time, and this function may return false
    /// sooner.
    ///
    /// This function may return false if there was a system error, the OS
    /// inadvertently awoke multiple threads, or if [`SDL_SignalAsyncIOQueue()`] was
    /// called to wake up all waiting threads without a finished task.
    ///
    /// A timeout can be used to specify a maximum wait time, but rather than
    /// polling, it is possible to have a timeout of -1 to wait forever, and use
    /// [`SDL_SignalAsyncIOQueue()`] to wake up the waiting threads later.
    ///
    /// ### Parameters
    /// - `queue`: the async I/O task queue to wait on.
    /// - `outcome`: details of a finished task will be written here. May not be
    ///   NULL.
    /// - `timeoutMS`: the maximum time to wait, in milliseconds, or -1 to wait
    ///   indefinitely.
    ///
    /// ### Return value
    /// Returns true if task has completed, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_SignalAsyncIOQueue`]
    pub fn SDL_WaitAsyncIOResult(
        queue: *mut SDL_AsyncIOQueue,
        outcome: *mut SDL_AsyncIOOutcome,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Wake up any threads that are blocking in [`SDL_WaitAsyncIOResult()`].
    ///
    /// This will unblock any threads that are sleeping in a call to
    /// [`SDL_WaitAsyncIOResult`] for the specified queue, and cause them to return
    /// from that function.
    ///
    /// This can be useful when destroying a queue to make sure nothing is touching
    /// it indefinitely. In this case, once this call completes, the caller should
    /// take measures to make sure any previously-blocked threads have returned
    /// from their wait and will not touch the queue again (perhaps by setting a
    /// flag to tell the threads to terminate and then using [`SDL_WaitThread()`] to
    /// make sure they've done so).
    ///
    /// ### Parameters
    /// - `queue`: the async I/O task queue to signal.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_WaitAsyncIOResult`]
    pub fn SDL_SignalAsyncIOQueue(queue: *mut SDL_AsyncIOQueue);
}

extern "C" {
    /// Load all the data from a file path, asynchronously.
    ///
    /// This function returns as quickly as possible; it does not wait for the read
    /// to complete. On a successful return, this work will continue in the
    /// background. If the work begins, even failure is asynchronous: a failing
    /// return value from this function only means the work couldn't start at all.
    ///
    /// The data is allocated with a zero byte at the end (null terminated) for
    /// convenience. This extra byte is not included in SDL_AsyncIOOutcome's
    /// bytes_transferred value.
    ///
    /// This function will allocate the buffer to contain the file. It must be
    /// deallocated by calling [`SDL_free()`] on SDL_AsyncIOOutcome's buffer field
    /// after completion.
    ///
    /// An [`SDL_AsyncIOQueue`] must be specified. The newly-created task will be added
    /// to it when it completes its work.
    ///
    /// ### Parameters
    /// - `file`: the path to read all available data from.
    /// - `queue`: a queue to add the new [`SDL_AsyncIO`] to.
    /// - `userdata`: an app-defined pointer that will be provided with the task
    ///   results.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`SDL_LoadFile_IO`]
    pub fn SDL_LoadFileAsync(
        file: *const ::core::ffi::c_char,
        queue: *mut SDL_AsyncIOQueue,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

/// The asynchronous I/O operation structure.
///
/// This operates as an opaque handle. One can then request read or write
/// operations on it.
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
///
/// ### See also
/// - [`SDL_AsyncIOFromFile`]
#[repr(C)]
pub struct SDL_AsyncIO {
    _opaque: [::core::primitive::u8; 0],
}

/// A queue of completed asynchronous I/O tasks.
///
/// When starting an asynchronous operation, you specify a queue for the new
/// task. A queue can be asked later if any tasks in it have completed,
/// allowing an app to manage multiple pending tasks in one place, in whatever
/// order they complete.
///
/// ### Availability
/// This struct is available since SDL 3.0.0.
///
/// ### See also
/// - [`SDL_CreateAsyncIOQueue`]
/// - [`SDL_ReadAsyncIO`]
/// - [`SDL_WriteAsyncIO`]
/// - [`SDL_GetAsyncIOResult`]
/// - [`SDL_WaitAsyncIOResult`]
#[repr(C)]
pub struct SDL_AsyncIOQueue {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
