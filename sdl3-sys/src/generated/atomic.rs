#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]

//! # CategoryAtomic
//!
//! Atomic operations.
//!
//! IMPORTANT: If you are not an expert in concurrent lockless programming, you
//! should not be using any functions in this file. You should be protecting
//! your data structures with full mutexes instead.
//!
//! ***Seriously, here be dragons!***
//!
//! You can find out a little more about lockless programming and the subtle
//! issues that can arise here:
//! https://learn.microsoft.com/en-us/windows/win32/dxtecharts/lockless-programming
//!
//! There's also lots of good information here:
//!
//! - https://www.1024cores.net/home/lock-free-algorithms
//! - https://preshing.com/
//!
//! These operations may or may not actually be implemented using processor
//! specific atomic operations. When possible they are implemented as true
//! processor specific atomic operations. When that is not possible the are
//! implemented using locks that *do* use the available atomic operations.
//!
//! All of the atomic operations that modify memory are full memory barriers.

use super::stdinc::*;

/// An atomic spinlock.
///
/// The atomic locks are efficient spinlocks using CPU instructions, but are
/// vulnerable to starvation and can spin forever if a thread holding a lock
/// has been terminated. For this reason you should minimize the code executed
/// inside an atomic lock and never do expensive things like API or system
/// calls while holding them.
///
/// They are also vulnerable to starvation if the thread holding the lock is
/// lower priority than other threads and doesn't get scheduled. In general you
/// should use mutexes instead, since they have better performance and
/// contention behavior.
///
/// The atomic locks are not safe to lock recursively.
///
/// Porting Note: The spin lock functions and type are required and can not be
/// emulated because they are used in the atomic emulation code.
pub type SDL_SpinLock = ::core::ffi::c_int;

extern "C" {
    /// Try to lock a spin lock by setting it to a non-zero value.
    ///
    /// ***Please note that spinlocks are dangerous if you don't know what you're
    /// doing. Please be careful using any sort of spinlock!***
    ///
    /// \param lock a pointer to a lock variable.
    /// \returns SDL_TRUE if the lock succeeded, SDL_FALSE if the lock is already
    ///          held.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockSpinlock
    /// \sa SDL_UnlockSpinlock
    pub fn SDL_TryLockSpinlock(lock: *mut SDL_SpinLock) -> SDL_bool;
}

extern "C" {
    /// Lock a spin lock by setting it to a non-zero value.
    ///
    /// ***Please note that spinlocks are dangerous if you don't know what you're
    /// doing. Please be careful using any sort of spinlock!***
    ///
    /// \param lock a pointer to a lock variable.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_TryLockSpinlock
    /// \sa SDL_UnlockSpinlock
    pub fn SDL_LockSpinlock(lock: *mut SDL_SpinLock);
}

extern "C" {
    /// Unlock a spin lock by setting it to 0.
    ///
    /// Always returns immediately.
    ///
    /// ***Please note that spinlocks are dangerous if you don't know what you're
    /// doing. Please be careful using any sort of spinlock!***
    ///
    /// \param lock a pointer to a lock variable.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockSpinlock
    /// \sa SDL_TryLockSpinlock
    pub fn SDL_UnlockSpinlock(lock: *mut SDL_SpinLock);
}

#[cfg(doc)]
emit! {
}

#[cfg(not(doc))]
emit! {
    #[cfg(all(not(any(/* always disabled: __clang__ */)), all(windows, target_env = "msvc")))]
    emit! {
        extern "C" {
            pub fn _ReadWriteBarrier();
        }

        // pragma `intrinsic(_ReadWriteBarrier)`
    }

    #[cfg(not(all(not(any(/* always disabled: __clang__ */)), all(windows, target_env = "msvc"))))]
    emit! {
        #[cfg(not(target_os = "emscripten"))]
        emit! {
        }

        #[cfg(target_os = "emscripten")]
        emit! {
        }

    }

}

extern "C" {
    /// Insert a memory release barrier.
    ///
    /// Memory barriers are designed to prevent reads and writes from being
    /// reordered by the compiler and being seen out of order on multi-core CPUs.
    ///
    /// A typical pattern would be for thread A to write some data and a flag, and
    /// for thread B to read the flag and get the data. In this case you would
    /// insert a release barrier between writing the data and the flag,
    /// guaranteeing that the data write completes no later than the flag is
    /// written, and you would insert an acquire barrier between reading the flag
    /// and reading the data, to ensure that all the reads associated with the flag
    /// have completed.
    ///
    /// In this pattern you should always see a release barrier paired with an
    /// acquire barrier and you should gate the data reads/writes with a single
    /// flag variable.
    ///
    /// For more information on these semantics, take a look at the blog post:
    /// http://preshing.com/20120913/acquire-and-release-semantics
    ///
    /// \threadsafety Obviously this macro is safe to use from any thread at any
    ///               time, but if you find yourself needing this, you are probably
    ///               dealing with some very sensitive code; be careful!
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_MemoryBarrierReleaseFunction();
}

extern "C" {
    /// Insert a memory acquire barrier.
    ///
    /// Please refer to SDL_MemoryBarrierReleaseFunction for the details!
    ///
    /// \threadsafety Obviously this function is safe to use from any thread at any
    ///               time, but if you find yourself needing this, you are probably
    ///               dealing with some very sensitive code; be careful!
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MemoryBarrierReleaseFunction
    pub fn SDL_MemoryBarrierAcquireFunction();
}

#[cfg(all(any(any(target_arch = "powerpc", target_arch = "powerpc64"), any(target_arch = "powerpc", target_arch = "powerpc64")), any(/* always disabled: __GNUC__ */)))]
emit! {
}

#[cfg(not(all(any(any(target_arch = "powerpc", target_arch = "powerpc64"), any(target_arch = "powerpc", target_arch = "powerpc64")), any(/* always disabled: __GNUC__ */))))]
emit! {
    #[cfg(all(any(/* always disabled: __GNUC__ */), target_arch = "aarch64"))]
    emit! {
    }

    #[cfg(not(all(any(/* always disabled: __GNUC__ */), target_arch = "aarch64")))]
    emit! {
    }

}

#[cfg(doc)]
emit! {
}

#[cfg(not(doc))]
emit! {
    #[cfg(all(any(any(/* always disabled: __GNUC__ */), any(/* always disabled: __clang__ */)), any(target_arch = "x86", target_arch = "x86_64")))]
    emit! {
    }

    #[cfg(not(all(any(any(/* always disabled: __GNUC__ */), any(/* always disabled: __clang__ */)), any(target_arch = "x86", target_arch = "x86_64"))))]
    emit! {
    }

}

/// A type representing an atomic integer value.
///
/// This can be used to manage a value that is synchronized across multiple
/// CPUs without a race condition; when an app sets a value with SDL_AtomicSet
/// all other threads, regardless of the CPU it is running on, will see that
/// value when retrieved with SDL_AtomicGet, regardless of CPU caches, etc.
///
/// This is also useful for atomic compare-and-swap operations: a thread can
/// change the value as long as its current value matches expectations. When
/// done in a loop, one can guarantee data consistency across threads without a
/// lock (but the usual warnings apply: if you don't know what you're doing, or
/// you don't do it carefully, you can confidently cause any number of
/// disasters with this, so in most cases, you _should_ use a mutex instead of
/// this!).
///
/// This is a struct so people don't accidentally use numeric operations on it
/// directly. You have to use SDL_Atomic* functions.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_AtomicCompareAndSwap
/// \sa SDL_AtomicGet
/// \sa SDL_AtomicSet
/// \sa SDL_AtomicAdd
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AtomicInt {
    pub value: ::core::ffi::c_int,
}

extern "C" {
    /// Set an atomic variable to a new value if it is currently an old value.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to an SDL_AtomicInt variable to be modified.
    /// \param oldval the old value.
    /// \param newval the new value.
    /// \returns SDL_TRUE if the atomic variable was set, SDL_FALSE otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicCompareAndSwapPointer
    pub fn SDL_AtomicCompareAndSwap(a: *mut SDL_AtomicInt, oldval: ::core::ffi::c_int, newval: ::core::ffi::c_int) -> SDL_bool;
}

extern "C" {
    /// Set an atomic variable to a value.
    ///
    /// This function also acts as a full memory barrier.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to an SDL_AtomicInt variable to be modified.
    /// \param v the desired value.
    /// \returns the previous value of the atomic variable.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicGet
    pub fn SDL_AtomicSet(a: *mut SDL_AtomicInt, v: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the value of an atomic variable.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to an SDL_AtomicInt variable.
    /// \returns the current value of an atomic variable.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicSet
    pub fn SDL_AtomicGet(a: *mut SDL_AtomicInt) -> ::core::ffi::c_int;
}

extern "C" {
    /// Add to an atomic variable.
    ///
    /// This function also acts as a full memory barrier.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to an SDL_AtomicInt variable to be modified.
    /// \param v the desired value to add.
    /// \returns the previous value of the atomic variable.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicDecRef
    /// \sa SDL_AtomicIncRef
    pub fn SDL_AtomicAdd(a: *mut SDL_AtomicInt, v: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Set a pointer to a new value if it is currently an old value.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to a pointer.
    /// \param oldval the old pointer value.
    /// \param newval the new pointer value.
    /// \returns SDL_TRUE if the pointer was set, SDL_FALSE otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicCompareAndSwap
    /// \sa SDL_AtomicGetPointer
    /// \sa SDL_AtomicSetPointer
    pub fn SDL_AtomicCompareAndSwapPointer(a: *mut *mut ::core::ffi::c_void, oldval: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> SDL_bool;
}

extern "C" {
    /// Set a pointer to a value atomically.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to a pointer.
    /// \param v the desired pointer value.
    /// \returns the previous value of the pointer.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicCompareAndSwapPointer
    /// \sa SDL_AtomicGetPointer
    pub fn SDL_AtomicSetPointer(a: *mut *mut ::core::ffi::c_void, v: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Get the value of a pointer atomically.
    ///
    /// ***Note: If you don't know what this function is for, you shouldn't use
    /// it!***
    ///
    /// \param a a pointer to a pointer.
    /// \returns the current value of a pointer.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_AtomicCompareAndSwapPointer
    /// \sa SDL_AtomicSetPointer
    pub fn SDL_AtomicGetPointer(a: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}

