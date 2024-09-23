//! Functions to provide thread synchronization primitives.

use super::stdinc::*;

use super::error::*;

extern "C" {
    /// Create a new mutex.
    ///
    /// All newly-created mutexes begin in the _unlocked_ state.
    ///
    /// Calls to SDL_LockMutex() will not return while the mutex is locked by
    /// another thread. See SDL_TryLockMutex() to attempt to lock without blocking.
    ///
    /// SDL mutexes are reentrant.
    ///
    /// \returns the initialized and unlocked mutex or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroyMutex
    /// \sa SDL_LockMutex
    /// \sa SDL_TryLockMutex
    /// \sa SDL_UnlockMutex
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
    /// \param mutex the mutex to lock.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_TryLockMutex
    /// \sa SDL_UnlockMutex
    pub fn SDL_LockMutex(mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Try to lock a mutex without blocking.
    ///
    /// This works just like SDL_LockMutex(), but if the mutex is not available,
    /// this function returns false immediately.
    ///
    /// This technique is useful if you need exclusive access to a resource but
    /// don't want to wait for it, and will return to it to try again later.
    ///
    /// This function returns true if passed a NULL mutex.
    ///
    /// \param mutex the mutex to try to lock.
    /// \returns true on success, false if the mutex would block.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockMutex
    /// \sa SDL_UnlockMutex
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
    /// \param mutex the mutex to unlock.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockMutex
    /// \sa SDL_TryLockMutex
    pub fn SDL_UnlockMutex(mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Destroy a mutex created with SDL_CreateMutex().
    ///
    /// This function must be called on any mutex that is no longer needed. Failure
    /// to destroy a mutex will result in a system memory or resource leak. While
    /// it is safe to destroy a mutex that is _unlocked_, it is not safe to attempt
    /// to destroy a locked mutex, and may result in undefined behavior depending
    /// on the platform.
    ///
    /// \param mutex the mutex to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateMutex
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
    /// Calls to SDL_LockRWLockForReading() and SDL_LockRWLockForWriting will not
    /// return while the rwlock is locked _for writing_ by another thread. See
    /// SDL_TryLockRWLockForReading() and SDL_TryLockRWLockForWriting() to attempt
    /// to lock without blocking.
    ///
    /// SDL read/write locks are only recursive for read-only locks! They are not
    /// guaranteed to be fair, or provide access in a FIFO manner! They are not
    /// guaranteed to favor writers. You may not lock a rwlock for both read-only
    /// and write access at the same time from the same thread (so you can't
    /// promote your read-only lock to a write lock without unlocking first).
    ///
    /// \returns the initialized and unlocked read/write lock or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroyRWLock
    /// \sa SDL_LockRWLockForReading
    /// \sa SDL_LockRWLockForWriting
    /// \sa SDL_TryLockRWLockForReading
    /// \sa SDL_TryLockRWLockForWriting
    /// \sa SDL_UnlockRWLock
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
    /// \param rwlock the read/write lock to lock.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockRWLockForWriting
    /// \sa SDL_TryLockRWLockForReading
    /// \sa SDL_UnlockRWLock
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
    /// \param rwlock the read/write lock to lock.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockRWLockForReading
    /// \sa SDL_TryLockRWLockForWriting
    /// \sa SDL_UnlockRWLock
    pub fn SDL_LockRWLockForWriting(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Try to lock a read/write lock _for reading_ without blocking.
    ///
    /// This works just like SDL_LockRWLockForReading(), but if the rwlock is not
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
    /// \param rwlock the rwlock to try to lock.
    /// \returns true on success, false if the lock would block.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockRWLockForReading
    /// \sa SDL_TryLockRWLockForWriting
    /// \sa SDL_UnlockRWLock
    pub fn SDL_TryLockRWLockForReading(rwlock: *mut SDL_RWLock) -> ::core::primitive::bool;
}

extern "C" {
    /// Try to lock a read/write lock _for writing_ without blocking.
    ///
    /// This works just like SDL_LockRWLockForWriting(), but if the rwlock is not
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
    /// \param rwlock the rwlock to try to lock.
    /// \returns true on success, false if the lock would block.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockRWLockForWriting
    /// \sa SDL_TryLockRWLockForReading
    /// \sa SDL_UnlockRWLock
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
    /// \param rwlock the rwlock to unlock.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockRWLockForReading
    /// \sa SDL_LockRWLockForWriting
    /// \sa SDL_TryLockRWLockForReading
    /// \sa SDL_TryLockRWLockForWriting
    pub fn SDL_UnlockRWLock(rwlock: *mut SDL_RWLock);
}

extern "C" {
    /// Destroy a read/write lock created with SDL_CreateRWLock().
    ///
    /// This function must be called on any read/write lock that is no longer
    /// needed. Failure to destroy a rwlock will result in a system memory or
    /// resource leak. While it is safe to destroy a rwlock that is _unlocked_, it
    /// is not safe to attempt to destroy a locked rwlock, and may result in
    /// undefined behavior depending on the platform.
    ///
    /// \param rwlock the rwlock to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRWLock
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
    /// \param initial_value the starting value of the semaphore.
    /// \returns a new semaphore or NULL on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroySemaphore
    /// \sa SDL_SignalSemaphore
    /// \sa SDL_TryWaitSemaphore
    /// \sa SDL_GetSemaphoreValue
    /// \sa SDL_WaitSemaphore
    /// \sa SDL_WaitSemaphoreTimeout
    pub fn SDL_CreateSemaphore(initial_value: Uint32) -> *mut SDL_Semaphore;
}

extern "C" {
    /// Destroy a semaphore.
    ///
    /// It is not safe to destroy a semaphore if there are threads currently
    /// waiting on it.
    ///
    /// \param sem the semaphore to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateSemaphore
    pub fn SDL_DestroySemaphore(sem: *mut SDL_Semaphore);
}

extern "C" {
    /// Wait until a semaphore has a positive value and then decrements it.
    ///
    /// This function suspends the calling thread until the semaphore pointed to by
    /// `sem` has a positive value, and then atomically decrement the semaphore
    /// value.
    ///
    /// This function is the equivalent of calling SDL_WaitSemaphoreTimeout() with
    /// a time length of -1.
    ///
    /// \param sem the semaphore wait on.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SignalSemaphore
    /// \sa SDL_TryWaitSemaphore
    /// \sa SDL_WaitSemaphoreTimeout
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
    /// \param sem the semaphore to wait on.
    /// \returns true if the wait succeeds, false if the wait would block.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SignalSemaphore
    /// \sa SDL_WaitSemaphore
    /// \sa SDL_WaitSemaphoreTimeout
    pub fn SDL_TryWaitSemaphore(sem: *mut SDL_Semaphore) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait until a semaphore has a positive value and then decrements it.
    ///
    /// This function suspends the calling thread until either the semaphore
    /// pointed to by `sem` has a positive value or the specified time has elapsed.
    /// If the call is successful it will atomically decrement the semaphore value.
    ///
    /// \param sem the semaphore to wait on.
    /// \param timeoutMS the length of the timeout, in milliseconds, or -1 to wait
    ///                  indefinitely.
    /// \returns true if the wait succeeds or false if the wait times out.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SignalSemaphore
    /// \sa SDL_TryWaitSemaphore
    /// \sa SDL_WaitSemaphore
    pub fn SDL_WaitSemaphoreTimeout(
        sem: *mut SDL_Semaphore,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Atomically increment a semaphore's value and wake waiting threads.
    ///
    /// \param sem the semaphore to increment.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_TryWaitSemaphore
    /// \sa SDL_WaitSemaphore
    /// \sa SDL_WaitSemaphoreTimeout
    pub fn SDL_SignalSemaphore(sem: *mut SDL_Semaphore);
}

extern "C" {
    /// Get the current value of a semaphore.
    ///
    /// \param sem the semaphore to query.
    /// \returns the current value of the semaphore.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSemaphoreValue(sem: *mut SDL_Semaphore) -> Uint32;
}

extern "C" {
    /// Create a condition variable.
    ///
    /// \returns a new condition variable or NULL on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BroadcastCondition
    /// \sa SDL_SignalCondition
    /// \sa SDL_WaitCondition
    /// \sa SDL_WaitConditionTimeout
    /// \sa SDL_DestroyCondition
    pub fn SDL_CreateCondition() -> *mut SDL_Condition;
}

extern "C" {
    /// Destroy a condition variable.
    ///
    /// \param cond the condition variable to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateCondition
    pub fn SDL_DestroyCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Restart one of the threads that are waiting on the condition variable.
    ///
    /// \param cond the condition variable to signal.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BroadcastCondition
    /// \sa SDL_WaitCondition
    /// \sa SDL_WaitConditionTimeout
    pub fn SDL_SignalCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Restart all threads that are waiting on the condition variable.
    ///
    /// \param cond the condition variable to signal.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SignalCondition
    /// \sa SDL_WaitCondition
    /// \sa SDL_WaitConditionTimeout
    pub fn SDL_BroadcastCondition(cond: *mut SDL_Condition);
}

extern "C" {
    /// Wait until a condition variable is signaled.
    ///
    /// This function unlocks the specified `mutex` and waits for another thread to
    /// call SDL_SignalCondition() or SDL_BroadcastCondition() on the condition
    /// variable `cond`. Once the condition variable is signaled, the mutex is
    /// re-locked and the function returns.
    ///
    /// The mutex must be locked before calling this function. Locking the mutex
    /// recursively (more than once) is not supported and leads to undefined
    /// behavior.
    ///
    /// This function is the equivalent of calling SDL_WaitConditionTimeout() with
    /// a time length of -1.
    ///
    /// \param cond the condition variable to wait on.
    /// \param mutex the mutex used to coordinate thread access.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BroadcastCondition
    /// \sa SDL_SignalCondition
    /// \sa SDL_WaitConditionTimeout
    pub fn SDL_WaitCondition(cond: *mut SDL_Condition, mutex: *mut SDL_Mutex);
}

extern "C" {
    /// Wait until a condition variable is signaled or a certain time has passed.
    ///
    /// This function unlocks the specified `mutex` and waits for another thread to
    /// call SDL_SignalCondition() or SDL_BroadcastCondition() on the condition
    /// variable `cond`, or for the specified time to elapse. Once the condition
    /// variable is signaled or the time elapsed, the mutex is re-locked and the
    /// function returns.
    ///
    /// The mutex must be locked before calling this function. Locking the mutex
    /// recursively (more than once) is not supported and leads to undefined
    /// behavior.
    ///
    /// \param cond the condition variable to wait on.
    /// \param mutex the mutex used to coordinate thread access.
    /// \param timeoutMS the maximum time to wait, in milliseconds, or -1 to wait
    ///                  indefinitely.
    /// \returns true if the condition variable is signaled, false if the condition
    ///          is not signaled in the allotted time.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_BroadcastCondition
    /// \sa SDL_SignalCondition
    /// \sa SDL_WaitCondition
    pub fn SDL_WaitConditionTimeout(
        cond: *mut SDL_Condition,
        mutex: *mut SDL_Mutex,
        timeoutMS: Sint32,
    ) -> ::core::primitive::bool;
}

/// A means to block multiple threads until a condition is satisfied.
///
/// Condition variables, paired with an SDL_Mutex, let an app halt multiple
/// threads until a condition has occurred, at which time the app can release
/// one or all waiting threads.
///
/// Wikipedia has a thorough explanation of the concept:
///
/// https://en.wikipedia.org/wiki/Condition_variable
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
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
/// https://en.wikipedia.org/wiki/Mutex
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Mutex {
    _opaque: [::core::primitive::u8; 0],
}

/// A mutex that allows read-only threads to run in parallel.
///
/// A rwlock is roughly the same concept as SDL_Mutex, but allows threads that
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
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
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
/// https://en.wikipedia.org/wiki/Semaphore_(programming)
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Semaphore {
    _opaque: [::core::primitive::u8; 0],
}
