//! Functions to provide thread synchronization primitives.

use super::stdinc::*;

use super::atomic::*;

use super::error::*;

use super::thread::*;

extern "C" {
    /// Create a new mutex.
    ///
    /// All newly-created mutexes begin in the _unlocked_ state.
    ///
    /// Calls to [`SDL_LockMutex()`] will not return while the mutex is locked by
    /// another thread. See [`SDL_TryLockMutex()`] to attempt to lock without blocking.
    ///
    /// SDL mutexes are reentrant.
    ///
    /// ### Return value
    /// Returns the initialized and unlocked mutex or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroyMutex`]
    /// - [`SDL_LockMutex`]
    /// - [`SDL_TryLockMutex`]
    /// - [`SDL_UnlockMutex`]
    pub fn SDL_CreateMutex() -> *mut SDL_Mutex;
}

extern "C" {
    /// Lock the mutex.
    ///
    /// This will block until the mutex is available, which is to say it is in the
    /// unlocked state and the OS has chosen the caller as the next thread to lock
    /// it. Of all threads waiting to lock the mutex, only one may do so at a time.
    ///
    /// It is legal for the owning thread to lock an already-locked mutex. It must
    /// unlock it the same number of times before it is actually made available for
    /// other threads in the system (this is known as a "recursive mutex").
    ///
    /// This function does not fail; if mutex is NULL, it will return immediately
    /// having locked nothing. If the mutex is valid, this function will always
    /// block until it can lock the mutex, and return with it locked.
    ///
    /// ### Parameters
    /// - `mutex`: the mutex to lock.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_TryLockMutex`]
    /// - [`SDL_UnlockMutex`]
    pub fn SDL_LockMutex(mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Try to lock a mutex without blocking.
    ///
    /// This works just like [`SDL_LockMutex()`], but if the mutex is not available,
    /// this function returns false immediately.
    ///
    /// This technique is useful if you need exclusive access to a resource but
    /// don't want to wait for it, and will return to it to try again later.
    ///
    /// This function returns true if passed a NULL mutex.
    ///
    /// ### Parameters
    /// - `mutex`: the mutex to try to lock.
    ///
    /// ### Return value
    /// Returns true on success, false if the mutex would block.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockMutex`]
    /// - [`SDL_UnlockMutex`]
    pub fn SDL_TryLockMutex(mutex: *mut SDL_Mutex) -> ::core::primitive::bool;
}

extern "C" {
    /// Unlock the mutex.
    ///
    /// It is legal for the owning thread to lock an already-locked mutex. It must
    /// unlock it the same number of times before it is actually made available for
    /// other threads in the system (this is known as a "recursive mutex").
    ///
    /// It is illegal to unlock a mutex that has not been locked by the current
    /// thread, and doing so results in undefined behavior.
    ///
    /// ### Parameters
    /// - `mutex`: the mutex to unlock.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockMutex`]
    /// - [`SDL_TryLockMutex`]
    pub fn SDL_UnlockMutex(mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Destroy a mutex created with [`SDL_CreateMutex()`].
    ///
    /// This function must be called on any mutex that is no longer needed. Failure
    /// to destroy a mutex will result in a system memory or resource leak. While
    /// it is safe to destroy a mutex that is _unlocked_, it is not safe to attempt
    /// to destroy a locked mutex, and may result in undefined behavior depending
    /// on the platform.
    ///
    /// ### Parameters
    /// - `mutex`: the mutex to destroy.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateMutex`]
    pub fn SDL_DestroyMutex(mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Create a new read/write lock.
    ///
    /// A read/write lock is useful for situations where you have multiple threads
    /// trying to access a resource that is rarely updated. All threads requesting
    /// a read-only lock will be allowed to run in parallel; if a thread requests a
    /// write lock, it will be provided exclusive access. This makes it safe for
    /// multiple threads to use a resource at the same time if they promise not to
    /// change it, and when it has to be changed, the rwlock will serve as a
    /// gateway to make sure those changes can be made safely.
    ///
    /// In the right situation, a rwlock can be more efficient than a mutex, which
    /// only lets a single thread proceed at a time, even if it won't be modifying
    /// the data.
    ///
    /// All newly-created read/write locks begin in the _unlocked_ state.
    ///
    /// Calls to [`SDL_LockRWLockForReading()`] and [`SDL_LockRWLockForWriting`] will not
    /// return while the rwlock is locked _for writing_ by another thread. See
    /// [`SDL_TryLockRWLockForReading()`] and [`SDL_TryLockRWLockForWriting()`] to attempt
    /// to lock without blocking.
    ///
    /// SDL read/write locks are only recursive for read-only locks! They are not
    /// guaranteed to be fair, or provide access in a FIFO manner! They are not
    /// guaranteed to favor writers. You may not lock a rwlock for both read-only
    /// and write access at the same time from the same thread (so you can't
    /// promote your read-only lock to a write lock without unlocking first).
    ///
    /// ### Return value
    /// Returns the initialized and unlocked read/write lock or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroyRWLock`]
    /// - [`SDL_LockRWLockForReading`]
    /// - [`SDL_LockRWLockForWriting`]
    /// - [`SDL_TryLockRWLockForReading`]
    /// - [`SDL_TryLockRWLockForWriting`]
    /// - [`SDL_UnlockRWLock`]
    pub fn SDL_CreateRWLock() -> *mut SDL_RWLock;
}

extern "C" {
    /// Lock the read/write lock for _read only_ operations.
    ///
    /// This will block until the rwlock is available, which is to say it is not
    /// locked for writing by any other thread. Of all threads waiting to lock the
    /// rwlock, all may do so at the same time as long as they are requesting
    /// read-only access; if a thread wants to lock for writing, only one may do so
    /// at a time, and no other threads, read-only or not, may hold the lock at the
    /// same time.
    ///
    /// It is legal for the owning thread to lock an already-locked rwlock for
    /// reading. It must unlock it the same number of times before it is actually
    /// made available for other threads in the system (this is known as a
    /// "recursive rwlock").
    ///
    /// Note that locking for writing is not recursive (this is only available to
    /// read-only locks).
    ///
    /// It is illegal to request a read-only lock from a thread that already holds
    /// the write lock. Doing so results in undefined behavior. Unlock the write
    /// lock before requesting a read-only lock. (But, of course, if you have the
    /// write lock, you don't need further locks to read in any case.)
    ///
    /// This function does not fail; if rwlock is NULL, it will return immediately
    /// having locked nothing. If the rwlock is valid, this function will always
    /// block until it can lock the mutex, and return with it locked.
    ///
    /// ### Parameters
    /// - `rwlock`: the read/write lock to lock.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockRWLockForWriting`]
    /// - [`SDL_TryLockRWLockForReading`]
    /// - [`SDL_UnlockRWLock`]
    pub fn SDL_LockRWLockForReading(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Lock the read/write lock for _write_ operations.
    ///
    /// This will block until the rwlock is available, which is to say it is not
    /// locked for reading or writing by any other thread. Only one thread may hold
    /// the lock when it requests write access; all other threads, whether they
    /// also want to write or only want read-only access, must wait until the
    /// writer thread has released the lock.
    ///
    /// It is illegal for the owning thread to lock an already-locked rwlock for
    /// writing (read-only may be locked recursively, writing can not). Doing so
    /// results in undefined behavior.
    ///
    /// It is illegal to request a write lock from a thread that already holds a
    /// read-only lock. Doing so results in undefined behavior. Unlock the
    /// read-only lock before requesting a write lock.
    ///
    /// This function does not fail; if rwlock is NULL, it will return immediately
    /// having locked nothing. If the rwlock is valid, this function will always
    /// block until it can lock the mutex, and return with it locked.
    ///
    /// ### Parameters
    /// - `rwlock`: the read/write lock to lock.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockRWLockForReading`]
    /// - [`SDL_TryLockRWLockForWriting`]
    /// - [`SDL_UnlockRWLock`]
    pub fn SDL_LockRWLockForWriting(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Try to lock a read/write lock _for reading_ without blocking.
    ///
    /// This works just like [`SDL_LockRWLockForReading()`], but if the rwlock is not
    /// available, then this function returns false immediately.
    ///
    /// This technique is useful if you need access to a resource but don't want to
    /// wait for it, and will return to it to try again later.
    ///
    /// Trying to lock for read-only access can succeed if other threads are
    /// holding read-only locks, as this won't prevent access.
    ///
    /// This function returns true if passed a NULL rwlock.
    ///
    /// ### Parameters
    /// - `rwlock`: the rwlock to try to lock.
    ///
    /// ### Return value
    /// Returns true on success, false if the lock would block.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockRWLockForReading`]
    /// - [`SDL_TryLockRWLockForWriting`]
    /// - [`SDL_UnlockRWLock`]
    pub fn SDL_TryLockRWLockForReading(rwlock: *mut SDL_RWLock) -> ::core::primitive::bool;
}

extern "C" {
    /// Try to lock a read/write lock _for writing_ without blocking.
    ///
    /// This works just like [`SDL_LockRWLockForWriting()`], but if the rwlock is not
    /// available, then this function returns false immediately.
    ///
    /// This technique is useful if you need exclusive access to a resource but
    /// don't want to wait for it, and will return to it to try again later.
    ///
    /// It is illegal for the owning thread to lock an already-locked rwlock for
    /// writing (read-only may be locked recursively, writing can not). Doing so
    /// results in undefined behavior.
    ///
    /// It is illegal to request a write lock from a thread that already holds a
    /// read-only lock. Doing so results in undefined behavior. Unlock the
    /// read-only lock before requesting a write lock.
    ///
    /// This function returns true if passed a NULL rwlock.
    ///
    /// ### Parameters
    /// - `rwlock`: the rwlock to try to lock.
    ///
    /// ### Return value
    /// Returns true on success, false if the lock would block.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockRWLockForWriting`]
    /// - [`SDL_TryLockRWLockForReading`]
    /// - [`SDL_UnlockRWLock`]
    pub fn SDL_TryLockRWLockForWriting(rwlock: *mut SDL_RWLock) -> ::core::primitive::bool;
}

extern "C" {
    /// Unlock the read/write lock.
    ///
    /// Use this function to unlock the rwlock, whether it was locked for read-only
    /// or write operations.
    ///
    /// It is legal for the owning thread to lock an already-locked read-only lock.
    /// It must unlock it the same number of times before it is actually made
    /// available for other threads in the system (this is known as a "recursive
    /// rwlock").
    ///
    /// It is illegal to unlock a rwlock that has not been locked by the current
    /// thread, and doing so results in undefined behavior.
    ///
    /// ### Parameters
    /// - `rwlock`: the rwlock to unlock.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockRWLockForReading`]
    /// - [`SDL_LockRWLockForWriting`]
    /// - [`SDL_TryLockRWLockForReading`]
    /// - [`SDL_TryLockRWLockForWriting`]
    pub fn SDL_UnlockRWLock(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Destroy a read/write lock created with [`SDL_CreateRWLock()`].
    ///
    /// This function must be called on any read/write lock that is no longer
    /// needed. Failure to destroy a rwlock will result in a system memory or
    /// resource leak. While it is safe to destroy a rwlock that is _unlocked_, it
    /// is not safe to attempt to destroy a locked rwlock, and may result in
    /// undefined behavior depending on the platform.
    ///
    /// ### Parameters
    /// - `rwlock`: the rwlock to destroy.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateRWLock`]
    pub fn SDL_DestroyRWLock(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Create a semaphore.
    ///
    /// This function creates a new semaphore and initializes it with the value
    /// `initial_value`. Each wait operation on the semaphore will atomically
    /// decrement the semaphore value and potentially block if the semaphore value
    /// is 0. Each post operation will atomically increment the semaphore value and
    /// wake waiting threads and allow them to retry the wait operation.
    ///
    /// ### Parameters
    /// - `initial_value`: the starting value of the semaphore.
    ///
    /// ### Return value
    /// Returns a new semaphore or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroySemaphore`]
    /// - [`SDL_SignalSemaphore`]
    /// - [`SDL_TryWaitSemaphore`]
    /// - [`SDL_GetSemaphoreValue`]
    /// - [`SDL_WaitSemaphore`]
    /// - [`SDL_WaitSemaphoreTimeout`]
    pub fn SDL_CreateSemaphore(initial_value: Uint32) -> *mut SDL_Semaphore;
}

extern "C" {
    /// Destroy a semaphore.
    ///
    /// It is not safe to destroy a semaphore if there are threads currently
    /// waiting on it.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore to destroy.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateSemaphore`]
    pub fn SDL_DestroySemaphore(sem: *mut SDL_Semaphore);
}

extern "C" {
    /// Wait until a semaphore has a positive value and then decrements it.
    ///
    /// This function suspends the calling thread until the semaphore pointed to by
    /// `sem` has a positive value, and then atomically decrement the semaphore
    /// value.
    ///
    /// This function is the equivalent of calling [`SDL_WaitSemaphoreTimeout()`] with
    /// a time length of -1.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore wait on.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SignalSemaphore`]
    /// - [`SDL_TryWaitSemaphore`]
    /// - [`SDL_WaitSemaphoreTimeout`]
    pub fn SDL_WaitSemaphore(sem: *mut SDL_Semaphore);
}

extern "C" {
    /// See if a semaphore has a positive value and decrement it if it does.
    ///
    /// This function checks to see if the semaphore pointed to by `sem` has a
    /// positive value and atomically decrements the semaphore value if it does. If
    /// the semaphore doesn't have a positive value, the function immediately
    /// returns false.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore to wait on.
    ///
    /// ### Return value
    /// Returns true if the wait succeeds, false if the wait would block.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SignalSemaphore`]
    /// - [`SDL_WaitSemaphore`]
    /// - [`SDL_WaitSemaphoreTimeout`]
    pub fn SDL_TryWaitSemaphore(sem: *mut SDL_Semaphore) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait until a semaphore has a positive value and then decrements it.
    ///
    /// This function suspends the calling thread until either the semaphore
    /// pointed to by `sem` has a positive value or the specified time has elapsed.
    /// If the call is successful it will atomically decrement the semaphore value.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore to wait on.
    /// - `timeoutMS`: the length of the timeout, in milliseconds, or -1 to wait
    ///   indefinitely.
    ///
    /// ### Return value
    /// Returns true if the wait succeeds or false if the wait times out.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SignalSemaphore`]
    /// - [`SDL_TryWaitSemaphore`]
    /// - [`SDL_WaitSemaphore`]
    pub fn SDL_WaitSemaphoreTimeout(
        sem: *mut SDL_Semaphore,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Atomically increment a semaphore's value and wake waiting threads.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore to increment.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_TryWaitSemaphore`]
    /// - [`SDL_WaitSemaphore`]
    /// - [`SDL_WaitSemaphoreTimeout`]
    pub fn SDL_SignalSemaphore(sem: *mut SDL_Semaphore);
}

extern "C" {
    /// Get the current value of a semaphore.
    ///
    /// ### Parameters
    /// - `sem`: the semaphore to query.
    ///
    /// ### Return value
    /// Returns the current value of the semaphore.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSemaphoreValue(sem: *mut SDL_Semaphore) -> Uint32;
}

extern "C" {
    /// Create a condition variable.
    ///
    /// ### Return value
    /// Returns a new condition variable or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_BroadcastCondition`]
    /// - [`SDL_SignalCondition`]
    /// - [`SDL_WaitCondition`]
    /// - [`SDL_WaitConditionTimeout`]
    /// - [`SDL_DestroyCondition`]
    pub fn SDL_CreateCondition() -> *mut SDL_Condition;
}

extern "C" {
    /// Destroy a condition variable.
    ///
    /// ### Parameters
    /// - `cond`: the condition variable to destroy.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateCondition`]
    pub fn SDL_DestroyCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Restart one of the threads that are waiting on the condition variable.
    ///
    /// ### Parameters
    /// - `cond`: the condition variable to signal.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_BroadcastCondition`]
    /// - [`SDL_WaitCondition`]
    /// - [`SDL_WaitConditionTimeout`]
    pub fn SDL_SignalCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Restart all threads that are waiting on the condition variable.
    ///
    /// ### Parameters
    /// - `cond`: the condition variable to signal.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SignalCondition`]
    /// - [`SDL_WaitCondition`]
    /// - [`SDL_WaitConditionTimeout`]
    pub fn SDL_BroadcastCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Wait until a condition variable is signaled.
    ///
    /// This function unlocks the specified `mutex` and waits for another thread to
    /// call [`SDL_SignalCondition()`] or [`SDL_BroadcastCondition()`] on the condition
    /// variable `cond`. Once the condition variable is signaled, the mutex is
    /// re-locked and the function returns.
    ///
    /// The mutex must be locked before calling this function. Locking the mutex
    /// recursively (more than once) is not supported and leads to undefined
    /// behavior.
    ///
    /// This function is the equivalent of calling [`SDL_WaitConditionTimeout()`] with
    /// a time length of -1.
    ///
    /// ### Parameters
    /// - `cond`: the condition variable to wait on.
    /// - `mutex`: the mutex used to coordinate thread access.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_BroadcastCondition`]
    /// - [`SDL_SignalCondition`]
    /// - [`SDL_WaitConditionTimeout`]
    pub fn SDL_WaitCondition(cond: *mut SDL_Condition, mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Wait until a condition variable is signaled or a certain time has passed.
    ///
    /// This function unlocks the specified `mutex` and waits for another thread to
    /// call [`SDL_SignalCondition()`] or [`SDL_BroadcastCondition()`] on the condition
    /// variable `cond`, or for the specified time to elapse. Once the condition
    /// variable is signaled or the time elapsed, the mutex is re-locked and the
    /// function returns.
    ///
    /// The mutex must be locked before calling this function. Locking the mutex
    /// recursively (more than once) is not supported and leads to undefined
    /// behavior.
    ///
    /// ### Parameters
    /// - `cond`: the condition variable to wait on.
    /// - `mutex`: the mutex used to coordinate thread access.
    /// - `timeoutMS`: the maximum time to wait, in milliseconds, or -1 to wait
    ///   indefinitely.
    ///
    /// ### Return value
    /// Returns true if the condition variable is signaled, false if the condition
    ///   is not signaled in the allotted time.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_BroadcastCondition`]
    /// - [`SDL_SignalCondition`]
    /// - [`SDL_WaitCondition`]
    pub fn SDL_WaitConditionTimeout(
        cond: *mut SDL_Condition,
        mutex: *mut SDL_Mutex,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

/// The current status of an [`SDL_InitState`] structure.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNINITIALIZED`](SDL_InitStatus::UNINITIALIZED) | [`SDL_INIT_STATUS_UNINITIALIZED`] | |
/// | [`INITIALIZING`](SDL_InitStatus::INITIALIZING) | [`SDL_INIT_STATUS_INITIALIZING`] | |
/// | [`INITIALIZED`](SDL_InitStatus::INITIALIZED) | [`SDL_INIT_STATUS_INITIALIZED`] | |
/// | [`UNINITIALIZING`](SDL_InitStatus::UNINITIALIZING) | [`SDL_INIT_STATUS_UNINITIALIZING`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_InitStatus(pub ::core::ffi::c_int);

impl From<SDL_InitStatus> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_InitStatus) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_InitStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNINITIALIZED => "SDL_INIT_STATUS_UNINITIALIZED",
            Self::INITIALIZING => "SDL_INIT_STATUS_INITIALIZING",
            Self::INITIALIZED => "SDL_INIT_STATUS_INITIALIZED",
            Self::UNINITIALIZING => "SDL_INIT_STATUS_UNINITIALIZING",

            _ => return write!(f, "SDL_InitStatus({})", self.0),
        })
    }
}

impl SDL_InitStatus {
    pub const UNINITIALIZED: Self = Self(0);
    pub const INITIALIZING: Self = Self(1);
    pub const INITIALIZED: Self = Self(2);
    pub const UNINITIALIZING: Self = Self(3);
}

pub const SDL_INIT_STATUS_UNINITIALIZED: SDL_InitStatus = SDL_InitStatus::UNINITIALIZED;
pub const SDL_INIT_STATUS_INITIALIZING: SDL_InitStatus = SDL_InitStatus::INITIALIZING;
pub const SDL_INIT_STATUS_INITIALIZED: SDL_InitStatus = SDL_InitStatus::INITIALIZED;
pub const SDL_INIT_STATUS_UNINITIALIZING: SDL_InitStatus = SDL_InitStatus::UNINITIALIZING;

/// A structure used for thread-safe initialization and shutdown.
///
/// Here is an example of using this:
///
/// ```c
///    static SDL_AtomicInitState init;
///
///    bool InitSystem(void)
///    {
///        if (!SDL_ShouldInit(&init)) {
///            // The system is initialized
///            return true;
///        }
///
///        // At this point, you should not leave this function without calling SDL_SetInitialized()
///
///        bool initialized = DoInitTasks();
///        SDL_SetInitialized(&init, initialized);
///        return initialized;
///    }
///
///    bool UseSubsystem(void)
///    {
///        if (SDL_ShouldInit(&init)) {
///            // Error, the subsystem isn't initialized
///            SDL_SetInitialized(&init, false);
///            return false;
///        }
///
///        // Do work using the initialized subsystem
///
///        return true;
///    }
///
///    void QuitSystem(void)
///    {
///        if (!SDL_ShouldQuit(&init)) {
///            // The system is not initialized
///            return true;
///        }
///
///        // At this point, you should not leave this function without calling SDL_SetInitialized()
///
///        DoQuitTasks();
///        SDL_SetInitialized(&init, false);
///    }
/// ```
///
/// Note that this doesn't protect any resources created during initialization,
/// or guarantee that nobody is using those resources during cleanup. You
/// should use other mechanisms to protect those, if that's a concern for your
/// code.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_InitState {
    pub status: SDL_AtomicInt,
    pub thread: SDL_ThreadID,
    pub reserved: *mut ::core::ffi::c_void,
}

extern "C" {
    /// Return whether initialization should be done.
    ///
    /// This function checks the passed in state and if initialization should be
    /// done, sets the status to [`SDL_INIT_STATUS_INITIALIZING`] and returns true.
    /// If another thread is already modifying this state, it will wait until
    /// that's done before returning.
    ///
    /// If this function returns true, the calling code must call
    /// [`SDL_SetInitialized()`] to complete the initialization.
    ///
    /// ### Parameters
    /// - `state`: the initialization state to check.
    ///
    /// ### Return value
    /// Returns true if initialization needs to be done, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetInitialized`]
    /// - [`SDL_ShouldQuit`]
    pub fn SDL_ShouldInit(state: *mut SDL_InitState) -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether cleanup should be done.
    ///
    /// This function checks the passed in state and if cleanup should be done,
    /// sets the status to [`SDL_INIT_STATUS_UNINITIALIZING`] and returns true.
    ///
    /// If this function returns true, the calling code must call
    /// [`SDL_SetInitialized()`] to complete the cleanup.
    ///
    /// ### Parameters
    /// - `state`: the initialization state to check.
    ///
    /// ### Return value
    /// Returns true if cleanup needs to be done, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_SetInitialized`]
    /// - [`SDL_ShouldInit`]
    pub fn SDL_ShouldQuit(state: *mut SDL_InitState) -> ::core::primitive::bool;
}

extern "C" {
    /// Finish an initialization state transition.
    ///
    /// This function sets the status of the passed in state to
    /// [`SDL_INIT_STATUS_INITIALIZED`] or [`SDL_INIT_STATUS_UNINITIALIZED`] and allows
    /// any threads waiting for the status to proceed.
    ///
    /// ### Parameters
    /// - `state`: the initialization state to check.
    /// - `initialized`: the new initialization state.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_ShouldInit`]
    /// - [`SDL_ShouldQuit`]
    pub fn SDL_SetInitialized(state: *mut SDL_InitState, initialized: ::core::primitive::bool);
}

/// A means to block multiple threads until a condition is satisfied.
///
/// Condition variables, paired with an [`SDL_Mutex`], let an app halt multiple
/// threads until a condition has occurred, at which time the app can release
/// one or all waiting threads.
///
/// Wikipedia has a thorough explanation of the concept:
///
/// <https://en.wikipedia.org/wiki/Condition_variable>
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Condition {
    _opaque: [::core::primitive::u8; 0],
}

/// A means to serialize access to a resource between threads.
///
/// Mutexes (short for "mutual exclusion") are a synchronization primitive that
/// allows exactly one thread to proceed at a time.
///
/// Wikipedia has a thorough explanation of the concept:
///
/// <https://en.wikipedia.org/wiki/Mutex>
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Mutex {
    _opaque: [::core::primitive::u8; 0],
}

/// A mutex that allows read-only threads to run in parallel.
///
/// A rwlock is roughly the same concept as [`SDL_Mutex`], but allows threads that
/// request read-only access to all hold the lock at the same time. If a thread
/// requests write access, it will block until all read-only threads have
/// released the lock, and no one else can hold the thread (for reading or
/// writing) at the same time as the writing thread.
///
/// This can be more efficient in cases where several threads need to access
/// data frequently, but changes to that data are rare.
///
/// There are other rules that apply to rwlocks that don't apply to mutexes,
/// about how threads are scheduled and when they can be recursively locked.
/// These are documented in the other rwlock functions.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_RWLock {
    _opaque: [::core::primitive::u8; 0],
}

/// A means to manage access to a resource, by count, between threads.
///
/// Semaphores (specifically, "counting semaphores"), let X number of threads
/// request access at the same time, each thread granted access decrementing a
/// counter. When the counter reaches zero, future requests block until a prior
/// thread releases their request, incrementing the counter again.
///
/// Wikipedia has a thorough explanation of the concept:
///
/// <https://en.wikipedia.org/wiki/Semaphore_(programming)>
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Semaphore {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
