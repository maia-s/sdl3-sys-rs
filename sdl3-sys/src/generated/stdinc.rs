//! This is a general header that includes C language support. It implements a
//! subset of the C runtime APIs, but with an `SDL_` prefix. For most common
//! use cases, these should behave the same way as their C runtime equivalents,
//! but they may differ in how or whether they handle certain edge cases. When
//! in doubt, consult the documentation for details.

pub const SDL_SIZE_MAX: ::core::primitive::usize = ::core::primitive::usize::MAX;

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {}

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {}

/// A signed 8-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Sint8 = ::core::primitive::i8;

pub const SDL_MAX_SINT8: Sint8 = (0x7f as Sint8);

pub const SDL_MIN_SINT8: Sint8 = ((-128_i32) as Sint8);

/// An unsigned 8-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Uint8 = ::core::primitive::u8;

pub const SDL_MAX_UINT8: Uint8 = (0xff as Uint8);

pub const SDL_MIN_UINT8: Uint8 = (0x00 as Uint8);

/// A signed 16-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Sint16 = ::core::primitive::i16;

pub const SDL_MAX_SINT16: Sint16 = (0x7fff as Sint16);

pub const SDL_MIN_SINT16: Sint16 = ((-32768_i32) as Sint16);

/// An unsigned 16-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Uint16 = ::core::primitive::u16;

pub const SDL_MAX_UINT16: Uint16 = (0xffff as Uint16);

pub const SDL_MIN_UINT16: Uint16 = (0x0000 as Uint16);

/// A signed 32-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Sint32 = ::core::primitive::i32;

pub const SDL_MAX_SINT32: Sint32 = (0x7fffffff as Sint32);

pub const SDL_MIN_SINT32: Sint32 = ((-2147483648_i32) as Sint32);

/// An unsigned 32-bit integer type.
///
/// This macro is available since SDL 3.0.0.
pub type Uint32 = ::core::primitive::u32;

pub const SDL_MAX_UINT32: Uint32 = (0xffffffff as Uint32);

pub const SDL_MIN_UINT32: Uint32 = (0x00000000 as Uint32);

/// A signed 64-bit integer type.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_SINT64_C`]<br>
pub type Sint64 = ::core::primitive::i64;

pub const SDL_MAX_SINT64: ::core::primitive::i64 = 9223372036854775807_i64;

pub const SDL_MIN_SINT64: ::core::primitive::i64 = -9223372036854775808_i64;

/// An unsigned 64-bit integer type.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_UINT64_C`]<br>
pub type Uint64 = ::core::primitive::u64;

pub const SDL_MAX_UINT64: ::core::primitive::u64 = 18446744073709551615_u64;

pub const SDL_MIN_UINT64: ::core::primitive::u64 = 0_u64;

/// SDL times are signed, 64-bit integers representing nanoseconds since the
/// Unix epoch (Jan 1, 1970).
///
/// They can be converted between POSIX time_t values with [`SDL_NS_TO_SECONDS()`]
/// and [`SDL_SECONDS_TO_NS()`], and between Windows FILETIME values with
/// [`SDL_TimeToWindows()`] and [`SDL_TimeFromWindows()`].
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_MAX_SINT64`]<br>
/// See also [`SDL_MIN_SINT64`]<br>
pub type SDL_Time = Sint64;

pub const SDL_MAX_TIME: ::core::primitive::i64 = 9223372036854775807_i64;

pub const SDL_MIN_TIME: ::core::primitive::i64 = -9223372036854775808_i64;

pub const SDL_FLT_EPSILON: ::core::ffi::c_float = ::core::primitive::f32::EPSILON;

#[cfg(windows)]
emit! {
    pub const SDL_PRIs64: &::core::ffi::CStr = c"I64d";

}

#[cfg(not(windows))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
        pub const SDL_PRIs64: &::core::ffi::CStr = c"ld";

    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
        pub const SDL_PRIs64: &::core::ffi::CStr = c"lld";

    }

}

#[cfg(windows)]
emit! {
    pub const SDL_PRIu64: &::core::ffi::CStr = c"I64u";

}

#[cfg(not(windows))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
        pub const SDL_PRIu64: &::core::ffi::CStr = c"lu";

    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
        pub const SDL_PRIu64: &::core::ffi::CStr = c"llu";

    }

}

#[cfg(windows)]
emit! {
    pub const SDL_PRIx64: &::core::ffi::CStr = c"I64x";

}

#[cfg(not(windows))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
        pub const SDL_PRIx64: &::core::ffi::CStr = c"lx";

    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
        pub const SDL_PRIx64: &::core::ffi::CStr = c"llx";

    }

}

#[cfg(windows)]
emit! {
    pub const SDL_PRIX64: &::core::ffi::CStr = c"I64X";

}

#[cfg(not(windows))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
        pub const SDL_PRIX64: &::core::ffi::CStr = c"lX";

    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
        pub const SDL_PRIX64: &::core::ffi::CStr = c"llX";

    }

}

pub const SDL_PRIs32: &::core::ffi::CStr = c"d";

pub const SDL_PRIu32: &::core::ffi::CStr = c"u";

pub const SDL_PRIx32: &::core::ffi::CStr = c"x";

pub const SDL_PRIX32: &::core::ffi::CStr = c"X";

#[cfg(windows)]
emit! {
    const _: () = ::core::assert!((::core::mem::size_of::<::core::ffi::c_longlong>() == 8_usize));

    pub const SDL_PRILL_PREFIX: &::core::ffi::CStr = c"I64";

    pub const SDL_PRILLd: &::core::ffi::CStr = c"I64d";

    pub const SDL_PRILLu: &::core::ffi::CStr = c"I64u";

    pub const SDL_PRILLx: &::core::ffi::CStr = c"I64x";

    pub const SDL_PRILLX: &::core::ffi::CStr = c"I64X";

}

#[cfg(not(windows))]
emit! {
    pub const SDL_PRILL_PREFIX: &::core::ffi::CStr = c"ll";

    pub const SDL_PRILLd: &::core::ffi::CStr = c"lld";

    pub const SDL_PRILLu: &::core::ffi::CStr = c"llu";

    pub const SDL_PRILLx: &::core::ffi::CStr = c"llx";

    pub const SDL_PRILLX: &::core::ffi::CStr = c"llX";

}

const _: () = ::core::assert!((::core::mem::size_of::<::core::primitive::bool>() == 1_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Uint8>() == 1_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Sint8>() == 1_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Uint16>() == 2_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Sint16>() == 2_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Uint32>() == 4_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Sint32>() == 4_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Uint64>() == 8_usize));

const _: () = ::core::assert!((::core::mem::size_of::<Sint64>() == 8_usize));

const _: () = ::core::assert!(
    (::core::mem::size_of::<Uint64>() <= ::core::mem::size_of::<::core::ffi::c_ulonglong>())
);

const _: () = ::core::assert!(
    (::core::mem::size_of::<::core::primitive::usize>()
        <= ::core::mem::size_of::<::core::ffi::c_ulonglong>())
);

/// Define a four character code as a Uint32.
///
/// - `A`: the first ASCII character.
/// - `B`: the second ASCII character.
/// - `C`: the third ASCII character.
/// - `D`: the fourth ASCII character.
/// - Returns the four characters converted into a Uint32, one character
///   per-byte.
///
/// Thread safety: It is safe to call this macro from any thread.
///
/// This macro is available since SDL 3.0.0.
#[inline(always)]
pub const fn SDL_FOURCC(A: Uint8, B: Uint8, C: Uint8, D: Uint8) -> Uint32 {
    (((((((A) as Uint8) as Uint32) << 0) | ((((B) as Uint8) as Uint32) << 8))
        | ((((C) as Uint8) as Uint32) << 16))
        | ((((D) as Uint8) as Uint32) << 24))
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_alignment_test {
    pub a: Uint8,
    pub b: *mut ::core::ffi::c_void,
}

const _: () = ::core::assert!(
    (::core::mem::size_of::<SDL_alignment_test>()
        == (2 * ::core::mem::size_of::<*mut ::core::ffi::c_void>()))
);

const _: () = ::core::assert!(
    ((!(0 as ::core::ffi::c_int) as ::core::ffi::c_int) == ((-1_i32) as ::core::ffi::c_int))
);

#[cfg(all(not(any(/* always disabled: SDL_PLATFORM_3DS */)), not(any(/* always disabled: SDL_PLATFORM_VITA */))))]
emit! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "debug-impls", derive(Debug))]
    pub struct SDL_DUMMY_ENUM(pub ::core::ffi::c_int);
    impl From<SDL_DUMMY_ENUM> for ::core::ffi::c_int{
        #[inline(always)]
        fn from(value: SDL_DUMMY_ENUM) -> Self {
            value.0
        }
    }
    impl SDL_DUMMY_ENUM {
        pub const DUMMY_ENUM_VALUE: Self = Self(0);
    }
    pub const DUMMY_ENUM_VALUE: SDL_DUMMY_ENUM = SDL_DUMMY_ENUM::DUMMY_ENUM_VALUE;

    const _: () = ::core::assert!((::core::mem::size_of::<SDL_DUMMY_ENUM>() == ::core::mem::size_of::<::core::ffi::c_int>()));

}

/// A macro to initialize an SDL interface.
///
/// This macro will initialize an SDL interface structure and should be called
/// before you fill out the fields with your implementation.
///
/// You can use it like this:
///
/// ```c
/// SDL_IOStreamInterface iface;
///
/// SDL_INIT_INTERFACE(&iface);
///
/// // Fill in the interface function pointers with your implementation
/// iface.seek = ...
///
/// stream = SDL_OpenIO(&iface, NULL);
/// ```
///
/// If you are using designated initializers, you can use the size of the
/// interface as the version, e.g.
///
/// ```c
/// SDL_IOStreamInterface iface = {
///     .version = sizeof(iface),
///     .seek = ...
/// };
/// stream = SDL_OpenIO(&iface, NULL);
/// ```
///
/// Thread safety: It is safe to call this macro from any thread.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_IOStreamInterface`]<br>
/// See also [`SDL_StorageInterface`]<br>
/// See also [`SDL_VirtualJoystickDesc`]<br>
///
/// # Safety
/// The type `T` must correctly implement [`crate::Interface`], and it must be valid to write a `T` to the memory pointed to by `iface`
#[inline(always)]
pub unsafe fn SDL_INIT_INTERFACE<T: crate::Interface>(iface: *mut T) {
    unsafe {
        iface.write_bytes(0, 1);
        iface
            .cast::<Uint32>()
            .write(::core::mem::size_of::<T>() as Uint32);
    }
}

extern "C" {
    /// Allocate uninitialized memory.
    ///
    /// The allocated memory returned by this function must be freed with
    /// [`SDL_free()`].
    ///
    /// If `size` is 0, it will be set to 1.
    ///
    /// If you want to allocate memory aligned to a specific alignment, consider
    /// using [`SDL_aligned_alloc()`].
    ///
    /// - `size`: the size to allocate.
    /// - Returns a pointer to the allocated memory, or NULL if allocation failed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_free`]<br>
    /// See also [`SDL_calloc`]<br>
    /// See also [`SDL_realloc`]<br>
    /// See also [`SDL_aligned_alloc`]<br>
    pub fn SDL_malloc(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Allocate a zero-initialized array.
    ///
    /// The memory returned by this function must be freed with [`SDL_free()`].
    ///
    /// If either of `nmemb` or `size` is 0, they will both be set to 1.
    ///
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of each element of the array.
    /// - Returns a pointer to the allocated array, or NULL if allocation failed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_free`]<br>
    /// See also [`SDL_malloc`]<br>
    /// See also [`SDL_realloc`]<br>
    pub fn SDL_calloc(
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Change the size of allocated memory.
    ///
    /// The memory returned by this function must be freed with [`SDL_free()`].
    ///
    /// If `size` is 0, it will be set to 1. Note that this is unlike some other C
    /// runtime `realloc` implementations, which may treat `realloc(mem, 0)` the
    /// same way as `free(mem)`.
    ///
    /// If `mem` is NULL, the behavior of this function is equivalent to
    /// [`SDL_malloc()`]. Otherwise, the function can have one of three possible
    /// outcomes:
    ///
    /// - If it returns the same pointer as `mem`, it means that `mem` was resized
    ///   in place without freeing.
    /// - If it returns a different non-NULL pointer, it means that `mem` was freed
    ///   and cannot be dereferenced anymore.
    /// - If it returns NULL (indicating failure), then `mem` will remain valid and
    ///   must still be freed with [`SDL_free()`].
    ///
    /// - `mem`: a pointer to allocated memory to reallocate, or NULL.
    /// - `size`: the new size of the memory.
    /// - Returns a pointer to the newly allocated memory, or NULL if allocation
    ///   failed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_free`]<br>
    /// See also [`SDL_malloc`]<br>
    /// See also [`SDL_calloc`]<br>
    pub fn SDL_realloc(
        mem: *mut ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Free allocated memory.
    ///
    /// The pointer is no longer valid after this call and cannot be dereferenced
    /// anymore.
    ///
    /// If `mem` is NULL, this function does nothing.
    ///
    /// - `mem`: a pointer to allocated memory, or NULL.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_malloc`]<br>
    /// See also [`SDL_calloc`]<br>
    /// See also [`SDL_realloc`]<br>
    pub fn SDL_free(mem: *mut ::core::ffi::c_void);
}

/// A callback used to implement [`SDL_malloc()`].
///
/// SDL will always ensure that the passed `size` is greater than 0.
///
/// - `size`: the size to allocate.
/// - Returns a pointer to the allocated memory, or NULL if allocation failed.
///
/// Thread safety: It should be safe to call this callback from any thread.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_malloc`]<br>
/// See also [`SDL_GetOriginalMemoryFunctions`]<br>
/// See also [`SDL_GetMemoryFunctions`]<br>
/// See also [`SDL_SetMemoryFunctions`]<br>
pub type SDL_malloc_func = ::core::option::Option<
    unsafe extern "C" fn(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void,
>;

/// A callback used to implement [`SDL_calloc()`].
///
/// SDL will always ensure that the passed `nmemb` and `size` are both greater
/// than 0.
///
/// - `nmemb`: the number of elements in the array.
/// - `size`: the size of each element of the array.
/// - Returns a pointer to the allocated array, or NULL if allocation failed.
///
/// Thread safety: It should be safe to call this callback from any thread.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_calloc`]<br>
/// See also [`SDL_GetOriginalMemoryFunctions`]<br>
/// See also [`SDL_GetMemoryFunctions`]<br>
/// See also [`SDL_SetMemoryFunctions`]<br>
pub type SDL_calloc_func = ::core::option::Option<
    unsafe extern "C" fn(
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void,
>;

/// A callback used to implement [`SDL_realloc()`].
///
/// SDL will always ensure that the passed `size` is greater than 0.
///
/// - `mem`: a pointer to allocated memory to reallocate, or NULL.
/// - `size`: the new size of the memory.
/// - Returns a pointer to the newly allocated memory, or NULL if allocation
///   failed.
///
/// Thread safety: It should be safe to call this callback from any thread.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_realloc`]<br>
/// See also [`SDL_GetOriginalMemoryFunctions`]<br>
/// See also [`SDL_GetMemoryFunctions`]<br>
/// See also [`SDL_SetMemoryFunctions`]<br>
pub type SDL_realloc_func = ::core::option::Option<
    unsafe extern "C" fn(
        mem: *mut ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void,
>;

/// A callback used to implement [`SDL_free()`].
///
/// SDL will always ensure that the passed `mem` is a non-NULL pointer.
///
/// - `mem`: a pointer to allocated memory.
///
/// Thread safety: It should be safe to call this callback from any thread.
///
/// This datatype is available since SDL 3.0.0.
///
/// See also [`SDL_free`]<br>
/// See also [`SDL_GetOriginalMemoryFunctions`]<br>
/// See also [`SDL_GetMemoryFunctions`]<br>
/// See also [`SDL_SetMemoryFunctions`]<br>
pub type SDL_free_func =
    ::core::option::Option<unsafe extern "C" fn(mem: *mut ::core::ffi::c_void)>;

extern "C" {
    /// Get the original set of SDL memory functions.
    ///
    /// This is what [`SDL_malloc`] and friends will use by default, if there has been
    /// no call to [`SDL_SetMemoryFunctions`]. This is not necessarily using the C
    /// runtime's `malloc` functions behind the scenes! Different platforms and
    /// build configurations might do any number of unexpected things.
    ///
    /// - `malloc_func`: filled with malloc function.
    /// - `calloc_func`: filled with calloc function.
    /// - `realloc_func`: filled with realloc function.
    /// - `free_func`: filled with free function.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetOriginalMemoryFunctions(
        malloc_func: *mut SDL_malloc_func,
        calloc_func: *mut SDL_calloc_func,
        realloc_func: *mut SDL_realloc_func,
        free_func: *mut SDL_free_func,
    );
}

extern "C" {
    /// Get the current set of SDL memory functions.
    ///
    /// - `malloc_func`: filled with malloc function.
    /// - `calloc_func`: filled with calloc function.
    /// - `realloc_func`: filled with realloc function.
    /// - `free_func`: filled with free function.
    ///
    /// Thread safety: This does not hold a lock, so do not call this in the
    ///   unlikely event of a background thread calling
    ///   [`SDL_SetMemoryFunctions`] simultaneously.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetMemoryFunctions`]<br>
    /// See also [`SDL_GetOriginalMemoryFunctions`]<br>
    pub fn SDL_GetMemoryFunctions(
        malloc_func: *mut SDL_malloc_func,
        calloc_func: *mut SDL_calloc_func,
        realloc_func: *mut SDL_realloc_func,
        free_func: *mut SDL_free_func,
    );
}

extern "C" {
    /// Replace SDL's memory allocation functions with a custom set.
    ///
    /// It is not safe to call this function once any allocations have been made,
    /// as future calls to [`SDL_free`] will use the new allocator, even if they came
    /// from an [`SDL_malloc`] made with the old one!
    ///
    /// If used, usually this needs to be the first call made into the SDL library,
    /// if not the very first thing done at program startup time.
    ///
    /// - `malloc_func`: custom malloc function.
    /// - `calloc_func`: custom calloc function.
    /// - `realloc_func`: custom realloc function.
    /// - `free_func`: custom free function.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// Thread safety: It is safe to call this function from any thread, but one
    ///   should not replace the memory functions once any allocations
    ///   are made!
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetMemoryFunctions`]<br>
    /// See also [`SDL_GetOriginalMemoryFunctions`]<br>
    pub fn SDL_SetMemoryFunctions(
        malloc_func: SDL_malloc_func,
        calloc_func: SDL_calloc_func,
        realloc_func: SDL_realloc_func,
        free_func: SDL_free_func,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Allocate memory aligned to a specific alignment.
    ///
    /// The memory returned by this function must be freed with [`SDL_aligned_free()`],
    /// _not_ [`SDL_free()`].
    ///
    /// If `alignment` is less than the size of `void *`, it will be increased to
    /// match that.
    ///
    /// The returned memory address will be a multiple of the alignment value, and
    /// the size of the memory allocated will be a multiple of the alignment value.
    ///
    /// - `alignment`: the alignment of the memory.
    /// - `size`: the size to allocate.
    /// - Returns a pointer to the aligned memory, or NULL if allocation failed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_aligned_free`]<br>
    pub fn SDL_aligned_alloc(
        alignment: ::core::primitive::usize,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Free memory allocated by [`SDL_aligned_alloc()`].
    ///
    /// The pointer is no longer valid after this call and cannot be dereferenced
    /// anymore.
    ///
    /// If `mem` is NULL, this function does nothing.
    ///
    /// - `mem`: a pointer previously returned by [`SDL_aligned_alloc()`], or NULL.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_aligned_alloc`]<br>
    pub fn SDL_aligned_free(mem: *mut ::core::ffi::c_void);
}

extern "C" {
    /// Get the number of outstanding (unfreed) allocations.
    ///
    /// - Returns the number of allocations.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetNumAllocations() -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the process environment.
    ///
    /// This is initialized at application start and is not affected by setenv()
    /// and unsetenv() calls after that point. Use [`SDL_SetEnvironmentVariable()`] and
    /// [`SDL_UnsetEnvironmentVariable()`] if you want to modify this environment, or
    /// [`SDL_setenv_unsafe()`] or [`SDL_unsetenv_unsafe()`] if you want changes to persist
    /// in the C runtime environment after [`SDL_Quit()`].
    ///
    /// - Returns a pointer to the environment for the process or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironmentVariable`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_GetEnvironment() -> *mut SDL_Environment;
}

extern "C" {
    /// Create a set of environment variables
    ///
    /// - `populated`: true to initialize it from the C runtime environment,
    ///   false to create an empty environment.
    /// - Returns a pointer to the new environment or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// Thread safety: If `populated` is false, it is safe to call this function
    ///   from any thread, otherwise it is safe if no other threads are
    ///   calling setenv() or unsetenv()
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironmentVariable`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    /// See also [`SDL_DestroyEnvironment`]<br>
    pub fn SDL_CreateEnvironment(populated: ::core::primitive::bool) -> *mut SDL_Environment;
}

extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// - `env`: the environment to query.
    /// - `name`: the name of the variable to get.
    /// - Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironment`]<br>
    /// See also [`SDL_CreateEnvironment`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_GetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get all variables in the environment.
    ///
    /// - `env`: the environment to query.
    /// - Returns a NULL terminated array of pointers to environment variables in
    ///   the form "variable=value" or NULL on failure; call [`SDL_GetError()`]
    ///   for more information. This is a single allocation that should be
    ///   freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironment`]<br>
    /// See also [`SDL_CreateEnvironment`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_GetEnvironmentVariables(env: *mut SDL_Environment) -> *mut *mut ::core::ffi::c_char;
}

extern "C" {
    /// Set the value of a variable in the environment.
    ///
    /// - `env`: the environment to modify.
    /// - `name`: the name of the variable to set.
    /// - `value`: the value of the variable to set.
    /// - `overwrite`: true to overwrite the variable if it exists, false to
    ///   return success without setting the variable if it already
    ///   exists.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironment`]<br>
    /// See also [`SDL_CreateEnvironment`]<br>
    /// See also [`SDL_GetEnvironmentVariable`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_SetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        overwrite: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Clear a variable from the environment.
    ///
    /// - `env`: the environment to modify.
    /// - `name`: the name of the variable to unset.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetEnvironment`]<br>
    /// See also [`SDL_CreateEnvironment`]<br>
    /// See also [`SDL_GetEnvironmentVariable`]<br>
    /// See also [`SDL_GetEnvironmentVariables`]<br>
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_UnsetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a set of environment variables.
    ///
    /// - `env`: the environment to destroy.
    ///
    /// Thread safety: It is safe to call this function from any thread, as long as
    ///   the environment is no longer in use.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CreateEnvironment`]<br>
    pub fn SDL_DestroyEnvironment(env: *mut SDL_Environment);
}

extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// This function uses SDL's cached copy of the environment and is thread-safe.
    ///
    /// - `name`: the name of the variable to get.
    /// - Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_getenv(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// This function bypasses SDL's cached copy of the environment and is not
    /// thread-safe.
    ///
    /// - `name`: the name of the variable to get.
    /// - Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// Thread safety: This function is not thread safe, consider using [`SDL_getenv()`]
    ///   instead.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_getenv`]<br>
    pub fn SDL_getenv_unsafe(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Set the value of a variable in the environment.
    ///
    /// - `name`: the name of the variable to set.
    /// - `value`: the value of the variable to set.
    /// - `overwrite`: 1 to overwrite the variable if it exists, 0 to return
    ///   success without setting the variable if it already exists.
    /// - Returns 0 on success, -1 on error.
    ///
    /// Thread safety: This function is not thread safe, consider using
    ///   [`SDL_SetEnvironmentVariable()`] instead.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_SetEnvironmentVariable`]<br>
    pub fn SDL_setenv_unsafe(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        overwrite: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Clear a variable from the environment.
    ///
    /// - `name`: the name of the variable to unset.
    /// - Returns 0 on success, -1 on error.
    ///
    /// Thread safety: This function is not thread safe, consider using
    ///   [`SDL_UnsetEnvironmentVariable()`] instead.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_UnsetEnvironmentVariable`]<br>
    pub fn SDL_unsetenv_unsafe(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

/// A callback used with SDL sorting and binary search functions.
///
/// - `a`: a pointer to the first element being compared.
/// - `b`: a pointer to the second element being compared.
/// - Returns -1 if `a` should be sorted before `b`, 1 if `b` should be sorted
///   before `a`, 0 if they are equal. If two elements are equal, their
///   order in the sorted array is undefined.
///
/// This callback is available since SDL 3.0.0.
///
/// See also [`SDL_bsearch`]<br>
/// See also [`SDL_qsort`]<br>
pub type SDL_CompareCallback = ::core::option::Option<
    unsafe extern "C" fn(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

extern "C" {
    /// Sort an array.
    ///
    /// For example:
    ///
    /// ```c
    /// typedef struct {
    ///     int key;
    ///     const char *string;
    /// } data;
    ///
    /// int SDLCALL compare(const void *a, const void *b)
    /// {
    ///     const data *A = (const data *)a;
    ///     const data *B = (const data *)b;
    ///
    ///     if (A->n < B->n) {
    ///         return -1;
    ///     } else if (B->n < A->n) {
    ///         return 1;
    ///     } else {
    ///         return 0;
    ///     }
    /// }
    ///
    /// data values[] = {
    ///     { 3, "third" }, { 1, "first" }, { 2, "second" }
    /// };
    ///
    /// SDL_qsort(values, SDL_arraysize(values), sizeof(values[0]), compare);
    /// ```
    ///
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_bsearch`]<br>
    /// See also [`SDL_qsort_r`]<br>
    pub fn SDL_qsort(
        base: *mut ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback,
    );
}

extern "C" {
    /// Perform a binary search on a previously sorted array.
    ///
    /// For example:
    ///
    /// ```c
    /// typedef struct {
    ///     int key;
    ///     const char *string;
    /// } data;
    ///
    /// int SDLCALL compare(const void *a, const void *b)
    /// {
    ///     const data *A = (const data *)a;
    ///     const data *B = (const data *)b;
    ///
    ///     if (A->n < B->n) {
    ///         return -1;
    ///     } else if (B->n < A->n) {
    ///         return 1;
    ///     } else {
    ///         return 0;
    ///     }
    /// }
    ///
    /// data values[] = {
    ///     { 1, "first" }, { 2, "second" }, { 3, "third" }
    /// };
    /// data key = { 2, NULL };
    ///
    /// data *result = SDL_bsearch(&key, values, SDL_arraysize(values), sizeof(values[0]), compare);
    /// ```
    ///
    /// - `key`: a pointer to a key equal to the element being searched for.
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    /// - Returns a pointer to the matching element in the array, or NULL if not
    ///   found.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_bsearch_r`]<br>
    /// See also [`SDL_qsort`]<br>
    pub fn SDL_bsearch(
        key: *const ::core::ffi::c_void,
        base: *const ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback,
    ) -> *mut ::core::ffi::c_void;
}

/// A callback used with SDL sorting and binary search functions.
///
/// - `userdata`: the `userdata` pointer passed to the sort function.
/// - `a`: a pointer to the first element being compared.
/// - `b`: a pointer to the second element being compared.
/// - Returns -1 if `a` should be sorted before `b`, 1 if `b` should be sorted
///   before `a`, 0 if they are equal. If two elements are equal, their
///   order in the sorted array is undefined.
///
/// This callback is available since SDL 3.0.0.
///
/// See also [`SDL_qsort_r`]<br>
/// See also [`SDL_bsearch_r`]<br>
pub type SDL_CompareCallback_r = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

extern "C" {
    /// Sort an array, passing a userdata pointer to the compare function.
    ///
    /// For example:
    ///
    /// ```c
    /// typedef enum {
    ///     sort_increasing,
    ///     sort_decreasing,
    /// } sort_method;
    ///
    /// typedef struct {
    ///     int key;
    ///     const char *string;
    /// } data;
    ///
    /// int SDLCALL compare(const void *userdata, const void *a, const void *b)
    /// {
    ///     sort_method method = (sort_method)(uintptr_t)userdata;
    ///     const data *A = (const data *)a;
    ///     const data *B = (const data *)b;
    ///
    ///     if (A->n < B->n) {
    ///         return (method == sort_increasing) ? -1 : 1;
    ///     } else if (B->n < A->n) {
    ///         return (method == sort_increasing) ? 1 : -1;
    ///     } else {
    ///         return 0;
    ///     }
    /// }
    ///
    /// data values[] = {
    ///     { 3, "third" }, { 1, "first" }, { 2, "second" }
    /// };
    ///
    /// SDL_qsort_r(values, SDL_arraysize(values), sizeof(values[0]), compare, (const void *)(uintptr_t)sort_increasing);
    /// ```
    ///
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    /// - `userdata`: a pointer to pass to the compare function.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_bsearch_r`]<br>
    /// See also [`SDL_qsort`]<br>
    pub fn SDL_qsort_r(
        base: *mut ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback_r,
        userdata: *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Perform a binary search on a previously sorted array, passing a userdata
    /// pointer to the compare function.
    ///
    /// For example:
    ///
    /// ```c
    /// typedef enum {
    ///     sort_increasing,
    ///     sort_decreasing,
    /// } sort_method;
    ///
    /// typedef struct {
    ///     int key;
    ///     const char *string;
    /// } data;
    ///
    /// int SDLCALL compare(const void *userdata, const void *a, const void *b)
    /// {
    ///     sort_method method = (sort_method)(uintptr_t)userdata;
    ///     const data *A = (const data *)a;
    ///     const data *B = (const data *)b;
    ///
    ///     if (A->n < B->n) {
    ///         return (method == sort_increasing) ? -1 : 1;
    ///     } else if (B->n < A->n) {
    ///         return (method == sort_increasing) ? 1 : -1;
    ///     } else {
    ///         return 0;
    ///     }
    /// }
    ///
    /// data values[] = {
    ///     { 1, "first" }, { 2, "second" }, { 3, "third" }
    /// };
    /// data key = { 2, NULL };
    ///
    /// data *result = SDL_bsearch_r(&key, values, SDL_arraysize(values), sizeof(values[0]), compare, (const void *)(uintptr_t)sort_increasing);
    /// ```
    ///
    /// - `key`: a pointer to a key equal to the element being searched for.
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    /// - `userdata`: a pointer to pass to the compare function.
    /// - Returns a pointer to the matching element in the array, or NULL if not
    ///   found.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_bsearch`]<br>
    /// See also [`SDL_qsort_r`]<br>
    pub fn SDL_bsearch_r(
        key: *const ::core::ffi::c_void,
        base: *const ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback_r,
        userdata: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    pub fn SDL_abs(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[inline(always)]
pub fn SDL_min<T: Copy + PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}

#[inline(always)]
pub fn SDL_max<T: Copy + PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

#[inline(always)]
pub fn SDL_clamp<T: Copy + PartialOrd>(x: T, a: T, b: T) -> T {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}

extern "C" {
    /// Query if a character is alphabetic (a letter).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z' and 'A-Z' as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isalpha(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Query if a character is alphabetic (a letter) or a number.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z', 'A-Z', and '0-9' as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isalnum(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is blank (a space or tab).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0x20 (space) or 0x9 (tab) as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isblank(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is a control character.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0 through 0x1F, and 0x7F, as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_iscntrl(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is a numeric digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// '0' (0x30) through '9' (0x39), as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is a hexadecimal digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'F', 'a' through 'f', and '0' through '9', as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isxdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is a punctuation mark.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `((SDL_isgraph(x)) && (!SDL_isalnum(x)))`.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isgraph`]<br>
    /// See also [`SDL_isalnum`]<br>
    pub fn SDL_ispunct(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is whitespace.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat the
    /// following ASCII values as true:
    ///
    /// - space (0x20)
    /// - tab (0x09)
    /// - newline (0x0A)
    /// - vertical tab (0x0B)
    /// - form feed (0x0C)
    /// - return (0x0D)
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isspace(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is upper case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'Z' as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is lower case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'a' through 'z' as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_islower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is "printable".
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// ' ' (0x20) through '~' (0x7E) as true.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_isprint(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Report if a character is any "printable" except space.
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `(SDL_isprint(x)) && ((x) != ' ')`.
    ///
    /// - `x`: character value to check.
    /// - Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isprint`]<br>
    pub fn SDL_isgraph(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Convert low-ASCII English letters to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'a' through 'z' to uppercase.
    ///
    /// This function returns the uppercase equivalent of `x`. If a character
    /// cannot be converted, or is already uppercase, this function returns `x`.
    ///
    /// - `x`: character value to check.
    /// - Returns capitalized version of x, or x if no conversion available.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_toupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    /// Convert low-ASCII English letters to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function returns the lowercase equivalent of `x`. If a character
    /// cannot be converted, or is already lowercase, this function returns `x`.
    ///
    /// - `x`: character value to check.
    /// - Returns lowercase version of x, or x if no conversion available.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_tolower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_crc16(
        crc: Uint16,
        data: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> Uint16;
}

extern "C" {
    pub fn SDL_crc32(
        crc: Uint32,
        data: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> Uint32;
}

extern "C" {
    pub fn SDL_murmur3_32(
        data: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
        seed: Uint32,
    ) -> Uint32;
}

/// Copy non-overlapping memory.
///
/// The memory regions must not overlap. If they do, use [`SDL_memmove()`] instead.
///
/// - `dst`: The destination memory region. Must not be NULL, and must not
///   overlap with `src`.
/// - `src`: The source memory region. Must not be NULL, and must not overlap
///   with `dst`.
/// - `len`: The length in bytes of both `dst` and `src`.
/// - Returns `dst`.
///
/// Thread safety: It is safe to call this function from any thread.
///
/// This function is available since SDL 3.0.0.
///
/// See also [`SDL_memmove`]<br>
#[inline(always)]
pub unsafe fn SDL_memcpy(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    len: ::core::primitive::usize,
) -> *mut ::core::ffi::c_void {
    unsafe { ::core::ptr::copy_nonoverlapping(src.cast::<Uint8>(), dst.cast::<Uint8>(), len) };
    return dst;
}

///
/// # Safety
/// It must be valid to write the memory pointed to by `src` to the memory pointed to by `dst`,
/// and the memory pointed to by `src` and `dst` must not overlap
#[inline(always)]
pub unsafe fn SDL_copyp<Dst: Sized, Src: Sized>(dst: *mut Dst, src: *const Src) -> *mut Dst {
    const { assert!(::core::mem::size_of::<Dst>() == ::core::mem::size_of::<Src>()) }
    unsafe {
        ::core::ptr::copy_nonoverlapping(
            src.cast::<Uint8>(),
            dst.cast::<Uint8>(),
            ::core::mem::size_of::<Src>(),
        )
    };
    dst
}

/// Copy memory.
///
/// It is okay for the memory regions to overlap. If you are confident that the
/// regions never overlap, using [`SDL_memcpy()`] may improve performance.
///
/// - `dst`: The destination memory region. Must not be NULL.
/// - `src`: The source memory region. Must not be NULL.
/// - `len`: The length in bytes of both `dst` and `src`.
/// - Returns `dst`.
///
/// Thread safety: It is safe to call this function from any thread.
///
/// This function is available since SDL 3.0.0.
///
/// See also [`SDL_memcpy`]<br>
#[inline(always)]
pub unsafe fn SDL_memmove(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    len: ::core::primitive::usize,
) -> *mut ::core::ffi::c_void {
    unsafe { ::core::ptr::copy(src.cast::<Uint8>(), dst.cast::<Uint8>(), len) };
    return dst;
}

extern "C" {
    pub fn SDL_memset(
        dst: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        len: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    pub fn SDL_memset4(
        dst: *mut ::core::ffi::c_void,
        val: Uint32,
        dwords: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

///
/// # Safety
/// It must be valid to zero all bytes of `T`, and it must be valid to write a `T` to the memory pointed to by `x`
#[inline(always)]
pub unsafe fn SDL_zerop<T>(x: *mut T) -> *mut T {
    unsafe { x.write_bytes(0, 1) };
    x
}

extern "C" {
    pub fn SDL_memcmp(
        s1: *const ::core::ffi::c_void,
        s2: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_wcslen(wstr: *const crate::ffi::c_wchar_t) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_wcsnlen(
        wstr: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Copy a wide string.
    ///
    /// This function copies `maxlen` - 1 wide characters from `src` to `dst`, then
    /// appends a null terminator.
    ///
    /// `src` and `dst` must not overlap.
    ///
    /// If `maxlen` is 0, no wide characters are copied and no null terminator is
    /// written.
    ///
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated wide string to copy. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `maxlen`: The length (in wide characters) of the destination buffer.
    /// - Returns The length (in wide characters, excluding the null terminator) of
    ///   `src`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_wcslcat`]<br>
    pub fn SDL_wcslcpy(
        dst: *mut crate::ffi::c_wchar_t,
        src: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Concatenate wide strings.
    ///
    /// This function appends up to `maxlen` - SDL_wcslen(dst) - 1 wide characters
    /// from `src` to the end of the wide string in `dst`, then appends a null
    /// terminator.
    ///
    /// `src` and `dst` must not overlap.
    ///
    /// If `maxlen` - SDL_wcslen(dst) - 1 is less than or equal to 0, then `dst` is
    /// unmodified.
    ///
    /// - `dst`: The destination buffer already containing the first
    ///   null-terminated wide string. Must not be NULL and must not
    ///   overlap with `src`.
    /// - `src`: The second null-terminated wide string. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `maxlen`: The length (in wide characters) of the destination buffer.
    /// - Returns The length (in wide characters, excluding the null terminator) of
    ///   the string in `dst` plus the length of `src`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_wcslcpy`]<br>
    pub fn SDL_wcslcat(
        dst: *mut crate::ffi::c_wchar_t,
        src: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_wcsdup(wstr: *const crate::ffi::c_wchar_t) -> *mut crate::ffi::c_wchar_t;
}

extern "C" {
    pub fn SDL_wcsstr(
        haystack: *const crate::ffi::c_wchar_t,
        needle: *const crate::ffi::c_wchar_t,
    ) -> *mut crate::ffi::c_wchar_t;
}

extern "C" {
    pub fn SDL_wcsnstr(
        haystack: *const crate::ffi::c_wchar_t,
        needle: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> *mut crate::ffi::c_wchar_t;
}

extern "C" {
    /// Compare two null-terminated wide strings.
    ///
    /// This only compares wchar_t values until it hits a null-terminating
    /// character; it does not care if the string is well-formed UTF-16 (or UTF-32,
    /// depending on your platform's wchar_t size), or uses valid Unicode values.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_wcscmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two wide strings up to a number of wchar_t values.
    ///
    /// This only compares wchar_t values; it does not care if the string is
    /// well-formed UTF-16 (or UTF-32, depending on your platform's wchar_t size),
    /// or uses valid Unicode values.
    ///
    /// Note that while this function is intended to be used with UTF-16 (or
    /// UTF-32, depending on your platform's definition of wchar_t), it is
    /// comparing raw wchar_t values and not Unicode codepoints: `maxlen` specifies
    /// a wchar_t limit! If the limit lands in the middle of a multi-wchar UTF-16
    /// sequence, it will only compare a portion of the final character.
    ///
    /// `maxlen` specifies a maximum number of wchar_t to compare; if the strings
    /// match to this number of wide chars (or both have matched to a
    /// null-terminator character before this count), they will be considered
    /// equal.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of wchar_t to compare.
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_wcsncmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two null-terminated wide strings, case-insensitively.
    ///
    /// This will work with Unicode strings, using a technique called
    /// "case-folding" to handle the vast majority of case-sensitive human
    /// languages regardless of system locale. It can deal with expanding values: a
    /// German Eszett character can compare against two ASCII 's' chars and be
    /// considered a match, for example. A notable exception: it does not handle
    /// the Turkish 'i' character; human language is complicated!
    ///
    /// Depending on your platform, "wchar_t" might be 2 bytes, and expected to be
    /// UTF-16 encoded (like Windows), or 4 bytes in UTF-32 format. Since this
    /// handles Unicode, it expects the string to be well-formed and not a
    /// null-terminated string of arbitrary bytes. Characters that are not valid
    /// UTF-16 (or UTF-32) are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), which is to say two strings of random bits may turn out to
    /// match if they convert to the same amount of replacement characters.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_wcscasecmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two wide strings, case-insensitively, up to a number of wchar_t.
    ///
    /// This will work with Unicode strings, using a technique called
    /// "case-folding" to handle the vast majority of case-sensitive human
    /// languages regardless of system locale. It can deal with expanding values: a
    /// German Eszett character can compare against two ASCII 's' chars and be
    /// considered a match, for example. A notable exception: it does not handle
    /// the Turkish 'i' character; human language is complicated!
    ///
    /// Depending on your platform, "wchar_t" might be 2 bytes, and expected to be
    /// UTF-16 encoded (like Windows), or 4 bytes in UTF-32 format. Since this
    /// handles Unicode, it expects the string to be well-formed and not a
    /// null-terminated string of arbitrary bytes. Characters that are not valid
    /// UTF-16 (or UTF-32) are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), which is to say two strings of random bits may turn out to
    /// match if they convert to the same amount of replacement characters.
    ///
    /// Note that while this function might deal with variable-sized characters,
    /// `maxlen` specifies a _wchar_ limit! If the limit lands in the middle of a
    /// multi-byte UTF-16 sequence, it may convert a portion of the final character
    /// to one or more Unicode character U+FFFD (REPLACEMENT CHARACTER) so as not
    /// to overflow a buffer.
    ///
    /// `maxlen` specifies a maximum number of wchar_t values to compare; if the
    /// strings match to this number of wchar_t (or both have matched to a
    /// null-terminator character before this number of bytes), they will be
    /// considered equal.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of wchar_t values to compare.
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_wcsncasecmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Parse a `long` from a wide string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long`, the result is clamped to
    /// the minimum and maximum representable `long` values.
    ///
    /// - `str`: The null-terminated wide string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid wide character
    ///   (i.e. the next character after the parsed number) will be
    ///   written to this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    /// - Returns The parsed `long`, or 0 if no number could be parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strtol`]<br>
    pub fn SDL_wcstol(
        str: *const crate::ffi::c_wchar_t,
        endp: *mut *mut crate::ffi::c_wchar_t,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}

extern "C" {
    pub fn SDL_strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_strnlen(
        str: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Copy a string.
    ///
    /// This function copies up to `maxlen` - 1 characters from `src` to `dst`,
    /// then appends a null terminator.
    ///
    /// If `maxlen` is 0, no characters are copied and no null terminator is
    /// written.
    ///
    /// If you want to copy an UTF-8 string but need to ensure that multi-byte
    /// sequences are not truncated, consider using [`SDL_utf8strlcpy()`].
    ///
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated string to copy. Must not be NULL, and must
    ///   not overlap with `dst`.
    /// - `maxlen`: The length (in characters) of the destination buffer.
    /// - Returns The length (in characters, excluding the null terminator) of
    ///   `src`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strlcat`]<br>
    /// See also [`SDL_utf8strlcpy`]<br>
    pub fn SDL_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Copy an UTF-8 string.
    ///
    /// This function copies up to `dst_bytes` - 1 bytes from `src` to `dst` while
    /// also ensuring that the string written to `dst` does not end in a truncated
    /// multi-byte sequence. Finally, it appends a null terminator.
    ///
    /// `src` and `dst` must not overlap.
    ///
    /// Note that unlike [`SDL_strlcpy()`], this function returns the number of bytes
    /// written, not the length of `src`.
    ///
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated UTF-8 string to copy. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `dst_bytes`: The length (in bytes) of the destination buffer. Must not
    ///   be 0.
    /// - Returns The number of bytes written, excluding the null terminator.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strlcpy`]<br>
    pub fn SDL_utf8strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_bytes: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Concatenate strings.
    ///
    /// This function appends up to `maxlen` - SDL_strlen(dst) - 1 characters from
    /// `src` to the end of the string in `dst`, then appends a null terminator.
    ///
    /// `src` and `dst` must not overlap.
    ///
    /// If `maxlen` - SDL_strlen(dst) - 1 is less than or equal to 0, then `dst` is
    /// unmodified.
    ///
    /// - `dst`: The destination buffer already containing the first
    ///   null-terminated string. Must not be NULL and must not overlap
    ///   with `src`.
    /// - `src`: The second null-terminated string. Must not be NULL, and must
    ///   not overlap with `dst`.
    /// - `maxlen`: The length (in characters) of the destination buffer.
    /// - Returns The length (in characters, excluding the null terminator) of the
    ///   string in `dst` plus the length of `src`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strlcpy`]<br>
    pub fn SDL_strlcat(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_strdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strndup(
        str: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strrev(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Convert a string to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to uppercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'a' through 'z' to their
    /// uppercase equivalents in-place, returning the original `str` pointer.
    ///
    /// - `str`: the string to convert in-place. Can not be NULL.
    /// - Returns the `str` pointer passed into this function.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strlwr`]<br>
    pub fn SDL_strupr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Convert a string to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'A' through 'Z' to their
    /// lowercase equivalents in-place, returning the original `str` pointer.
    ///
    /// - `str`: the string to convert in-place. Can not be NULL.
    /// - Returns the `str` pointer passed into this function.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_strupr`]<br>
    pub fn SDL_strlwr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strchr(
        str: *const ::core::ffi::c_char,
        c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strrchr(
        str: *const ::core::ffi::c_char,
        c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strstr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strnstr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strcasestr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_strtok_r(
        s1: *mut ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
        saveptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_utf8strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_utf8strnlen(
        str: *const ::core::ffi::c_char,
        bytes: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    pub fn SDL_itoa(
        value: ::core::ffi::c_int,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_uitoa(
        value: ::core::ffi::c_uint,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_ltoa(
        value: ::core::ffi::c_long,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_ultoa(
        value: ::core::ffi::c_ulong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_lltoa(
        value: ::core::ffi::c_longlong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_ulltoa(
        value: ::core::ffi::c_ulonglong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    /// Parse an `int` from a string.
    ///
    /// The result of calling `SDL_atoi(str)` is equivalent to
    /// `(int)SDL_strtol(str, NULL, 10)`.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - Returns The parsed `int`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoull`]<br>
    /// See also [`SDL_strtod`]<br>
    /// See also [`SDL_itoa`]<br>
    pub fn SDL_atoi(str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

extern "C" {
    /// Parse a `double` from a string.
    ///
    /// The result of calling `SDL_atof(str)` is equivalent to `SDL_strtod(str,
    /// NULL)`.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - Returns The parsed `double`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoull`]<br>
    /// See also [`SDL_strtod`]<br>
    pub fn SDL_atof(str: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
}

extern "C" {
    /// Parse a `long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long`, the result is clamped to
    /// the minimum and maximum representable `long` values.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    /// - Returns The parsed `long`, or 0 if no number could be parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoull`]<br>
    /// See also [`SDL_strtod`]<br>
    /// See also [`SDL_ltoa`]<br>
    /// See also [`SDL_wcstol`]<br>
    pub fn SDL_strtol(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}

extern "C" {
    /// Parse an `unsigned long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside an `unsigned long`, the result is
    /// clamped to the maximum representable `unsigned long` value.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    /// - Returns The parsed `unsigned long`, or 0 if no number could be parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoull`]<br>
    /// See also [`SDL_strtod`]<br>
    /// See also [`SDL_ultoa`]<br>
    pub fn SDL_strtoul(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
}

extern "C" {
    /// Parse a `long long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long long`, the result is
    /// clamped to the minimum and maximum representable `long long` values.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    /// - Returns The parsed `long long`, or 0 if no number could be parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtoull`]<br>
    /// See also [`SDL_strtod`]<br>
    /// See also [`SDL_lltoa`]<br>
    pub fn SDL_strtoll(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
}

extern "C" {
    /// Parse an `unsigned long long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside an `unsigned long long`, the
    /// result is clamped to the maximum representable `unsigned long long` value.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    /// - Returns The parsed `unsigned long long`, or 0 if no number could be
    ///   parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtod`]<br>
    /// See also [`SDL_ulltoa`]<br>
    pub fn SDL_strtoull(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}

extern "C" {
    /// Parse a `double` from a string.
    ///
    /// This function makes fewer guarantees than the C runtime `strtod`:
    ///
    /// - Only decimal notation is guaranteed to be supported. The handling of
    ///   scientific and hexadecimal notation is unspecified.
    /// - Whether or not INF and NAN can be parsed is unspecified.
    /// - The precision of the result is unspecified.
    ///
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - Returns The parsed `double`, or 0 if no number could be parsed.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atoi`]<br>
    /// See also [`SDL_atof`]<br>
    /// See also [`SDL_strtol`]<br>
    /// See also [`SDL_strtoll`]<br>
    /// See also [`SDL_strtoul`]<br>
    /// See also [`SDL_strtoull`]<br>
    pub fn SDL_strtod(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compare two null-terminated UTF-8 strings.
    ///
    /// Due to the nature of UTF-8 encoding, this will work with Unicode strings,
    /// since effectively this function just compares bytes until it hits a
    /// null-terminating character. Also due to the nature of UTF-8, this can be
    /// used with [`SDL_qsort()`] to put strings in (roughly) alphabetical order.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_strcmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two UTF-8 strings up to a number of bytes.
    ///
    /// Due to the nature of UTF-8 encoding, this will work with Unicode strings,
    /// since effectively this function just compares bytes until it hits a
    /// null-terminating character. Also due to the nature of UTF-8, this can be
    /// used with [`SDL_qsort()`] to put strings in (roughly) alphabetical order.
    ///
    /// Note that while this function is intended to be used with UTF-8, it is
    /// doing a bytewise comparison, and `maxlen` specifies a _byte_ limit! If the
    /// limit lands in the middle of a multi-byte UTF-8 sequence, it will only
    /// compare a portion of the final character.
    ///
    /// `maxlen` specifies a maximum number of bytes to compare; if the strings
    /// match to this number of bytes (or both have matched to a null-terminator
    /// character before this number of bytes), they will be considered equal.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of _bytes_ to compare.
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_strncmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two null-terminated UTF-8 strings, case-insensitively.
    ///
    /// This will work with Unicode strings, using a technique called
    /// "case-folding" to handle the vast majority of case-sensitive human
    /// languages regardless of system locale. It can deal with expanding values: a
    /// German Eszett character can compare against two ASCII 's' chars and be
    /// considered a match, for example. A notable exception: it does not handle
    /// the Turkish 'i' character; human language is complicated!
    ///
    /// Since this handles Unicode, it expects the string to be well-formed UTF-8
    /// and not a null-terminated string of arbitrary bytes. Bytes that are not
    /// valid UTF-8 are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), which is to say two strings of random bits may turn out to
    /// match if they convert to the same amount of replacement characters.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_strcasecmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compare two UTF-8 strings, case-insensitively, up to a number of bytes.
    ///
    /// This will work with Unicode strings, using a technique called
    /// "case-folding" to handle the vast majority of case-sensitive human
    /// languages regardless of system locale. It can deal with expanding values: a
    /// German Eszett character can compare against two ASCII 's' chars and be
    /// considered a match, for example. A notable exception: it does not handle
    /// the Turkish 'i' character; human language is complicated!
    ///
    /// Since this handles Unicode, it expects the string to be well-formed UTF-8
    /// and not a null-terminated string of arbitrary bytes. Bytes that are not
    /// valid UTF-8 are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), which is to say two strings of random bits may turn out to
    /// match if they convert to the same amount of replacement characters.
    ///
    /// Note that while this function is intended to be used with UTF-8, `maxlen`
    /// specifies a _byte_ limit! If the limit lands in the middle of a multi-byte
    /// UTF-8 sequence, it may convert a portion of the final character to one or
    /// more Unicode character U+FFFD (REPLACEMENT CHARACTER) so as not to overflow
    /// a buffer.
    ///
    /// `maxlen` specifies a maximum number of bytes to compare; if the strings
    /// match to this number of bytes (or both have matched to a null-terminator
    /// character before this number of bytes), they will be considered equal.
    ///
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of bytes to compare.
    /// - Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_strncasecmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Searches a string for the first occurence of any character contained in a
    /// breakset, and returns a pointer from the string to that character.
    ///
    /// - `str`: The null-terminated string to be searched. Must not be NULL, and
    ///   must not overlap with `breakset`.
    /// - `breakset`: A null-terminated string containing the list of characters
    ///   to look for. Must not be NULL, and must not overlap with
    ///   `str`.
    /// - Returns A pointer to the location, in str, of the first occurence of a
    ///   character present in the breakset, or NULL if none is found.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_strpbrk(
        str: *const ::core::ffi::c_char,
        breakset: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

/// The Unicode REPLACEMENT CHARACTER codepoint.
///
/// [`SDL_StepUTF8()`] reports this codepoint when it encounters a UTF-8 string
/// with encoding errors.
///
/// This tends to render as something like a question mark in most places.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_StepUTF8`]<br>
pub const SDL_INVALID_UNICODE_CODEPOINT: ::core::primitive::i32 = 65533;

extern "C" {
    /// Decode a UTF-8 string, one Unicode codepoint at a time.
    ///
    /// This will return the first Unicode codepoint in the UTF-8 encoded string in
    /// `*pstr`, and then advance `*pstr` past any consumed bytes before returning.
    ///
    /// It will not access more than `*pslen` bytes from the string. `*pslen` will
    /// be adjusted, as well, subtracting the number of bytes consumed.
    ///
    /// `pslen` is allowed to be NULL, in which case the string _must_ be
    /// NULL-terminated, as the function will blindly read until it sees the NULL
    /// char.
    ///
    /// if `*pslen` is zero, it assumes the end of string is reached and returns a
    /// zero codepoint regardless of the contents of the string buffer.
    ///
    /// If the resulting codepoint is zero (a NULL terminator), or `*pslen` is
    /// zero, it will not advance `*pstr` or `*pslen` at all.
    ///
    /// Generally this function is called in a loop until it returns zero,
    /// adjusting its parameters each iteration.
    ///
    /// If an invalid UTF-8 sequence is encountered, this function returns
    /// [`SDL_INVALID_UNICODE_CODEPOINT`] and advances the string/length by one byte
    /// (which is to say, a multibyte sequence might produce several
    /// [`SDL_INVALID_UNICODE_CODEPOINT`] returns before it syncs to the next valid
    /// UTF-8 sequence).
    ///
    /// Several things can generate invalid UTF-8 sequences, including overlong
    /// encodings, the use of UTF-16 surrogate values, and truncated data. Please
    /// refer to
    /// [RFC3629](https://www.ietf.org/rfc/rfc3629.txt)
    /// for details.
    ///
    /// - `pstr`: a pointer to a UTF-8 string pointer to be read and adjusted.
    /// - `pslen`: a pointer to the number of bytes in the string, to be read and
    ///   adjusted. NULL is allowed.
    /// - Returns the first Unicode codepoint in the string.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_StepUTF8(
        pstr: *mut *const ::core::ffi::c_char,
        pslen: *mut ::core::primitive::usize,
    ) -> Uint32;
}

extern "C" {
    /// Convert a single Unicode codepoint to UTF-8.
    ///
    /// The buffer pointed to by `dst` must be at least 4 bytes long, as this
    /// function may generate between 1 and 4 bytes of output.
    ///
    /// This function returns the first byte _after_ the newly-written UTF-8
    /// sequence, which is useful for encoding multiple codepoints in a loop, or
    /// knowing where to write a NULL-terminator character to end the string (in
    /// either case, plan to have a buffer of _more_ than 4 bytes!).
    ///
    /// If `codepoint` is an invalid value (outside the Unicode range, or a UTF-16
    /// surrogate value, etc), this will use U+FFFD (REPLACEMENT CHARACTER) for the
    /// codepoint instead, and not set an error.
    ///
    /// If `dst` is NULL, this returns NULL immediately without writing to the
    /// pointer and without setting an error.
    ///
    /// - `codepoint`: a Unicode codepoint to convert to UTF-8.
    /// - `dst`: the location to write the encoded UTF-8. Must point to at least
    ///   4 bytes!
    /// - Returns the first byte past the newly-written UTF-8 sequence.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_UCS4ToUTF8(
        codepoint: Uint32,
        dst: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

extern "C" {
    pub fn SDL_sscanf(
        text: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_vsscanf(
        text: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_snprintf(
        text: *mut ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_swprintf(
        text: *mut crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
        fmt: *const crate::ffi::c_wchar_t,
        ...
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_vsnprintf(
        text: *mut ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_vswprintf(
        text: *mut crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
        fmt: *const crate::ffi::c_wchar_t,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_asprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub fn SDL_vasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Seeds the pseudo-random number generator.
    ///
    /// Reusing the seed number will cause SDL_rand_*() to repeat the same stream
    /// of 'random' numbers.
    ///
    /// - `seed`: the value to use as a random number seed, or 0 to use
    ///   [`SDL_GetPerformanceCounter()`].
    ///
    /// Thread safety: This should be called on the same thread that calls
    ///   SDL_rand*()
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_rand`]<br>
    /// See also [`SDL_rand_bits`]<br>
    /// See also [`SDL_randf`]<br>
    pub fn SDL_srand(seed: Uint64);
}

extern "C" {
    /// Generate a pseudo-random number less than n for positive n
    ///
    /// The method used is faster and of better quality than `rand() % n`. Odds are
    /// roughly 99.9% even for n = 1 million. Evenness is better for smaller n, and
    /// much worse as n gets bigger.
    ///
    /// Example: to simulate a d6 use `SDL_rand(6) + 1` The +1 converts 0..5 to
    /// 1..6
    ///
    /// If you want to generate a pseudo-random number in the full range of Sint32,
    /// you should use: (Sint32)SDL_rand_bits()
    ///
    /// If you want reproducible output, be sure to initialize with [`SDL_srand()`]
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - `n`: the number of possible outcomes. n must be positive.
    /// - Returns a random value in the range of [0 .. n-1].
    ///
    /// Thread safety: All calls should be made from a single thread
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_srand`]<br>
    /// See also [`SDL_randf`]<br>
    pub fn SDL_rand(n: Sint32) -> Sint32;
}

extern "C" {
    /// Generate a uniform pseudo-random floating point number less than 1.0
    ///
    /// If you want reproducible output, be sure to initialize with [`SDL_srand()`]
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - Returns a random value in the range of [0.0, 1.0).
    ///
    /// Thread safety: All calls should be made from a single thread
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_srand`]<br>
    /// See also [`SDL_rand`]<br>
    pub fn SDL_randf() -> ::core::ffi::c_float;
}

extern "C" {
    /// Generate 32 pseudo-random bits.
    ///
    /// You likely want to use [`SDL_rand()`] to get a psuedo-random number instead.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - Returns a random value in the range of [0-SDL_MAX_UINT32].
    ///
    /// Thread safety: All calls should be made from a single thread
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_rand`]<br>
    /// See also [`SDL_randf`]<br>
    /// See also [`SDL_srand`]<br>
    pub fn SDL_rand_bits() -> Uint32;
}

extern "C" {
    /// Generate a pseudo-random number less than n for positive n
    ///
    /// The method used is faster and of better quality than `rand() % n`. Odds are
    /// roughly 99.9% even for n = 1 million. Evenness is better for smaller n, and
    /// much worse as n gets bigger.
    ///
    /// Example: to simulate a d6 use `SDL_rand_r(state, 6) + 1` The +1 converts
    /// 0..5 to 1..6
    ///
    /// If you want to generate a pseudo-random number in the full range of Sint32,
    /// you should use: (Sint32)SDL_rand_bits_r(state)
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    /// - `n`: the number of possible outcomes. n must be positive.
    /// - Returns a random value in the range of [0 .. n-1].
    ///
    /// Thread safety: This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_rand`]<br>
    /// See also [`SDL_rand_bits_r`]<br>
    /// See also [`SDL_randf_r`]<br>
    pub fn SDL_rand_r(state: *mut Uint64, n: Sint32) -> Sint32;
}

extern "C" {
    /// Generate a uniform pseudo-random floating point number less than 1.0
    ///
    /// If you want reproducible output, be sure to initialize with [`SDL_srand()`]
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    /// - Returns a random value in the range of [0.0, 1.0).
    ///
    /// Thread safety: This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_rand_bits_r`]<br>
    /// See also [`SDL_rand_r`]<br>
    /// See also [`SDL_randf`]<br>
    pub fn SDL_randf_r(state: *mut Uint64) -> ::core::ffi::c_float;
}

extern "C" {
    /// Generate 32 pseudo-random bits.
    ///
    /// You likely want to use [`SDL_rand_r()`] to get a psuedo-random number instead.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    /// - Returns a random value in the range of [0-SDL_MAX_UINT32].
    ///
    /// Thread safety: This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_rand_r`]<br>
    /// See also [`SDL_randf_r`]<br>
    pub fn SDL_rand_bits_r(state: *mut Uint64) -> Uint32;
}

/// pi (double)
pub const SDL_PI_D: ::core::ffi::c_double = 3.141592653589793_f64;

/// pi (float)
pub const SDL_PI_F: ::core::ffi::c_float = 3.1415927_f32;

extern "C" {
    /// Compute the arc cosine of `x`.
    ///
    /// The definition of `y = acos(x)` is `x = cos(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `0 <= y <= Pi`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_acosf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc cosine of `x`, in radians.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_acosf`]<br>
    /// See also [`SDL_asin`]<br>
    /// See also [`SDL_cos`]<br>
    pub fn SDL_acos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the arc cosine of `x`.
    ///
    /// The definition of `y = acos(x)` is `x = cos(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `0 <= y <= Pi`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_acos`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc cosine of `x`, in radians.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_acos`]<br>
    /// See also [`SDL_asinf`]<br>
    /// See also [`SDL_cosf`]<br>
    pub fn SDL_acosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the arc sine of `x`.
    ///
    /// The definition of `y = asin(x)` is `x = sin(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_asinf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc sine of `x`, in radians.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_asinf`]<br>
    /// See also [`SDL_acos`]<br>
    /// See also [`SDL_sin`]<br>
    pub fn SDL_asin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the arc sine of `x`.
    ///
    /// The definition of `y = asin(x)` is `x = sin(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_asin`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc sine of `x`, in radians.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_asin`]<br>
    /// See also [`SDL_acosf`]<br>
    /// See also [`SDL_sinf`]<br>
    pub fn SDL_asinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the arc tangent of `x`.
    ///
    /// The definition of `y = atan(x)` is `x = tan(y)`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_atanf`] for single-precision floats.
    ///
    /// To calculate the arc tangent of y / x, use [`SDL_atan2`].
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atanf`]<br>
    /// See also [`SDL_atan2`]<br>
    /// See also [`SDL_tan`]<br>
    pub fn SDL_atan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the arc tangent of `x`.
    ///
    /// The definition of `y = atan(x)` is `x = tan(y)`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_atan`] for dboule-precision floats.
    ///
    /// To calculate the arc tangent of y / x, use [`SDL_atan2f`].
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atan`]<br>
    /// See also [`SDL_atan2f`]<br>
    /// See also [`SDL_tanf`]<br>
    pub fn SDL_atanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the arc tangent of `y / x`, using the signs of x and y to adjust
    /// the result's quadrant.
    ///
    /// The definition of `z = atan2(x, y)` is `y = x tan(z)`, where the quadrant
    /// of z is determined based on the signs of x and y.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_atan2f`] for single-precision floats.
    ///
    /// To calculate the arc tangent of a single value, use [`SDL_atan`].
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `y`: floating point value of the numerator (y coordinate).
    /// - `x`: floating point value of the denominator (x coordinate).
    /// - Returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///   `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atan2f`]<br>
    /// See also [`SDL_atan`]<br>
    /// See also [`SDL_tan`]<br>
    pub fn SDL_atan2(y: ::core::ffi::c_double, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the arc tangent of `y / x`, using the signs of x and y to adjust
    /// the result's quadrant.
    ///
    /// The definition of `z = atan2(x, y)` is `y = x tan(z)`, where the quadrant
    /// of z is determined based on the signs of x and y.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_atan2`] for double-precision floats.
    ///
    /// To calculate the arc tangent of a single value, use [`SDL_atanf`].
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `y`: floating point value of the numerator (y coordinate).
    /// - `x`: floating point value of the denominator (x coordinate).
    /// - Returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///   `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_atan2f`]<br>
    /// See also [`SDL_atan`]<br>
    /// See also [`SDL_tan`]<br>
    pub fn SDL_atan2f(y: ::core::ffi::c_float, x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the ceiling of `x`.
    ///
    /// The ceiling of `x` is the smallest integer `y` such that `y > x`, i.e `x`
    /// rounded up to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_ceilf`] for single-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns the ceiling of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_trunc`]<br>
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_lround`]<br>
    pub fn SDL_ceil(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the ceiling of `x`.
    ///
    /// The ceiling of `x` is the smallest integer `y` such that `y > x`, i.e `x`
    /// rounded up to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_ceil`] for double-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns the ceiling of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_truncf`]<br>
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_lroundf`]<br>
    pub fn SDL_ceilf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Copy the sign of one floating-point value to another.
    ///
    /// The definition of copysign is that ``copysign(x, y) = abs(x) * sign(y)``.
    ///
    /// Domain: `-INF <= x <= INF`, ``-INF <= y <= f``
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_copysignf`] for single-precision floats.
    ///
    /// - `x`: floating point value to use as the magnitude.
    /// - `y`: floating point value to use as the sign.
    /// - Returns the floating point value with the sign of y and the magnitude of
    ///   x.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_copysignf`]<br>
    /// See also [`SDL_fabs`]<br>
    pub fn SDL_copysign(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

extern "C" {
    /// Copy the sign of one floating-point value to another.
    ///
    /// The definition of copysign is that ``copysign(x, y) = abs(x) * sign(y)``.
    ///
    /// Domain: `-INF <= x <= INF`, ``-INF <= y <= f``
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_copysign`] for double-precision floats.
    ///
    /// - `x`: floating point value to use as the magnitude.
    /// - `y`: floating point value to use as the sign.
    /// - Returns the floating point value with the sign of y and the magnitude of
    ///   x.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_copysignf`]<br>
    /// See also [`SDL_fabsf`]<br>
    pub fn SDL_copysignf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the cosine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_cosf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns cosine of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_cosf`]<br>
    /// See also [`SDL_acos`]<br>
    /// See also [`SDL_sin`]<br>
    pub fn SDL_cos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the cosine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_cos`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns cosine of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_cos`]<br>
    /// See also [`SDL_acosf`]<br>
    /// See also [`SDL_sinf`]<br>
    pub fn SDL_cosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the exponential of `x`.
    ///
    /// The definition of `y = exp(x)` is `y = e^x`, where `e` is the base of the
    /// natural logarithm. The inverse is the natural logarithm, [`SDL_log`].
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// The output will overflow if `exp(x)` is too large to be represented.
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_expf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns value of `e^x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_expf`]<br>
    /// See also [`SDL_log`]<br>
    pub fn SDL_exp(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the exponential of `x`.
    ///
    /// The definition of `y = exp(x)` is `y = e^x`, where `e` is the base of the
    /// natural logarithm. The inverse is the natural logarithm, [`SDL_logf`].
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// The output will overflow if `exp(x)` is too large to be represented.
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_exp`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value.
    /// - Returns value of `e^x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_exp`]<br>
    /// See also [`SDL_logf`]<br>
    pub fn SDL_expf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the absolute value of `x`
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_copysignf`] for single-precision floats.
    ///
    /// - `x`: floating point value to use as the magnitude.
    /// - Returns the absolute value of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_fabsf`]<br>
    pub fn SDL_fabs(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the absolute value of `x`
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_copysignf`] for double-precision floats.
    ///
    /// - `x`: floating point value to use as the magnitude.
    /// - Returns the absolute value of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_fabs`]<br>
    pub fn SDL_fabsf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the floor of `x`.
    ///
    /// The floor of `x` is the largest integer `y` such that `y > x`, i.e `x`
    /// rounded down to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_floorf`] for single-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns the floor of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_trunc`]<br>
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_lround`]<br>
    pub fn SDL_floor(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the floor of `x`.
    ///
    /// The floor of `x` is the largest integer `y` such that `y > x`, i.e `x`
    /// rounded down to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_floorf`] for double-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns the floor of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_truncf`]<br>
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_lroundf`]<br>
    pub fn SDL_floorf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Truncate `x` to an integer.
    ///
    /// Rounds `x` to the next closest integer to 0. This is equivalent to removing
    /// the fractional part of `x`, leaving only the integer part.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_truncf`] for single-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns `x` truncated to an integer.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_truncf`]<br>
    /// See also [`SDL_fmod`]<br>
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_lround`]<br>
    pub fn SDL_trunc(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Truncate `x` to an integer.
    ///
    /// Rounds `x` to the next closest integer to 0. This is equivalent to removing
    /// the fractional part of `x`, leaving only the integer part.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_truncf`] for double-precision floats.
    ///
    /// - `x`: floating point value.
    /// - Returns `x` truncated to an integer.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_trunc`]<br>
    /// See also [`SDL_fmodf`]<br>
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_lroundf`]<br>
    pub fn SDL_truncf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Return the floating-point remainder of `x / y`
    ///
    /// Divides `x` by `y`, and returns the remainder.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`, `y != 0`
    ///
    /// Range: `-y <= z <= y`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_fmodf`] for single-precision floats.
    ///
    /// - `x`: the numerator.
    /// - `y`: the denominator. Must not be 0.
    /// - Returns the remainder of `x / y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_fmodf`]<br>
    /// See also [`SDL_modf`]<br>
    /// See also [`SDL_trunc`]<br>
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_lround`]<br>
    pub fn SDL_fmod(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Return the floating-point remainder of `x / y`
    ///
    /// Divides `x` by `y`, and returns the remainder.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`, `y != 0`
    ///
    /// Range: `-y <= z <= y`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_fmod`] for single-precision floats.
    ///
    /// - `x`: the numerator.
    /// - `y`: the denominator. Must not be 0.
    /// - Returns the remainder of `x / y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_fmod`]<br>
    /// See also [`SDL_truncf`]<br>
    /// See also [`SDL_modff`]<br>
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_lroundf`]<br>
    pub fn SDL_fmodf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Return whether the value is infinity.
    ///
    /// - `x`: double-precision floating point value.
    /// - Returns non-zero if the value is infinity, 0 otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isinff`]<br>
    pub fn SDL_isinf(x: ::core::ffi::c_double) -> ::core::ffi::c_int;
}

extern "C" {
    /// Return whether the value is infinity.
    ///
    /// - `x`: floating point value.
    /// - Returns non-zero if the value is infinity, 0 otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isinf`]<br>
    pub fn SDL_isinff(x: ::core::ffi::c_float) -> ::core::ffi::c_int;
}

extern "C" {
    /// Return whether the value is NaN.
    ///
    /// - `x`: double-precision floating point value.
    /// - Returns non-zero if the value is NaN, 0 otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isnanf`]<br>
    pub fn SDL_isnan(x: ::core::ffi::c_double) -> ::core::ffi::c_int;
}

extern "C" {
    /// Return whether the value is NaN.
    ///
    /// - `x`: floating point value.
    /// - Returns non-zero if the value is NaN, 0 otherwise.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_isnan`]<br>
    pub fn SDL_isnanf(x: ::core::ffi::c_float) -> ::core::ffi::c_int;
}

extern "C" {
    /// Compute the natural logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_logf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than 0.
    /// - Returns the natural logarithm of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_logf`]<br>
    /// See also [`SDL_log10`]<br>
    /// See also [`SDL_exp`]<br>
    pub fn SDL_log(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the natural logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_log`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than 0.
    /// - Returns the natural logarithm of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_log`]<br>
    /// See also [`SDL_expf`]<br>
    pub fn SDL_logf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the base-10 logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_log10f`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than 0.
    /// - Returns the logarithm of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_log10f`]<br>
    /// See also [`SDL_log`]<br>
    /// See also [`SDL_pow`]<br>
    pub fn SDL_log10(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the base-10 logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_log10`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than 0.
    /// - Returns the logarithm of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_log10`]<br>
    /// See also [`SDL_logf`]<br>
    /// See also [`SDL_powf`]<br>
    pub fn SDL_log10f(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_modff`] for single-precision floats.
    ///
    /// - `x`: floating point value.
    /// - `y`: output pointer to store the integer part of `x`.
    /// - Returns the fractional part of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_modff`]<br>
    /// See also [`SDL_trunc`]<br>
    /// See also [`SDL_fmod`]<br>
    pub fn SDL_modf(
        x: ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

extern "C" {
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_modf`] for double-precision floats.
    ///
    /// - `x`: floating point value.
    /// - `y`: output pointer to store the integer part of `x`.
    /// - Returns the fractional part of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_modf`]<br>
    /// See also [`SDL_truncf`]<br>
    /// See also [`SDL_fmodf`]<br>
    pub fn SDL_modff(x: ::core::ffi::c_float, y: *mut ::core::ffi::c_float)
        -> ::core::ffi::c_float;
}

extern "C" {
    /// Raise `x` to the power `y`
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// If `y` is the base of the natural logarithm (e), consider using [`SDL_exp`]
    /// instead.
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_powf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: the base.
    /// - `y`: the exponent.
    /// - Returns `x` raised to the power `y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_powf`]<br>
    /// See also [`SDL_exp`]<br>
    /// See also [`SDL_log`]<br>
    pub fn SDL_pow(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Raise `x` to the power `y`
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// If `y` is the base of the natural logarithm (e), consider using [`SDL_exp`]
    /// instead.
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_powf`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: the base.
    /// - `y`: the exponent.
    /// - Returns `x` raised to the power `y`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_pow`]<br>
    /// See also [`SDL_expf`]<br>
    /// See also [`SDL_logf`]<br>
    pub fn SDL_powf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Round `x` to the nearest integer.
    ///
    /// Rounds `x` to the nearest integer. Values halfway between integers will be
    /// rounded away from zero.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_roundf`] for single-precision floats. To get the result as an integer
    /// type, use [`SDL_lround`].
    ///
    /// - `x`: floating point value.
    /// - Returns the nearest integer to `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_lround`]<br>
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_trunc`]<br>
    pub fn SDL_round(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Round `x` to the nearest integer.
    ///
    /// Rounds `x` to the nearest integer. Values halfway between integers will be
    /// rounded away from zero.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_roundf`] for single-precision floats. To get the result as an integer
    /// type, use [`SDL_lroundf`].
    ///
    /// - `x`: floating point value.
    /// - Returns the nearest integer to `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_lroundf`]<br>
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_truncf`]<br>
    pub fn SDL_roundf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Round `x` to the nearest integer representable as a long
    ///
    /// Rounds `x` to the nearest integer. Values halfway between integers will be
    /// rounded away from zero.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `MIN_LONG <= y <= MAX_LONG`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_lround`] for single-precision floats. To get the result as a
    /// floating-point type, use [`SDL_round`].
    ///
    /// - `x`: floating point value.
    /// - Returns the nearest integer to `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_lroundf`]<br>
    /// See also [`SDL_round`]<br>
    /// See also [`SDL_floor`]<br>
    /// See also [`SDL_ceil`]<br>
    /// See also [`SDL_trunc`]<br>
    pub fn SDL_lround(x: ::core::ffi::c_double) -> ::core::ffi::c_long;
}

extern "C" {
    /// Round `x` to the nearest integer representable as a long
    ///
    /// Rounds `x` to the nearest integer. Values halfway between integers will be
    /// rounded away from zero.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `MIN_LONG <= y <= MAX_LONG`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_lroundf`] for double-precision floats. To get the result as a
    /// floating-point type, use [`SDL_roundf`],
    ///
    /// - `x`: floating point value.
    /// - Returns the nearest integer to `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_lround`]<br>
    /// See also [`SDL_roundf`]<br>
    /// See also [`SDL_floorf`]<br>
    /// See also [`SDL_ceilf`]<br>
    /// See also [`SDL_truncf`]<br>
    pub fn SDL_lroundf(x: ::core::ffi::c_float) -> ::core::ffi::c_long;
}

extern "C" {
    /// Scale `x` by an integer power of two.
    ///
    /// Multiplies `x` by the `n`th power of the floating point radix (always 2).
    ///
    /// Domain: `-INF <= x <= INF`, `n` integer
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_scalbnf`] for single-precision floats.
    ///
    /// - `x`: floating point value to be scaled.
    /// - `n`: integer exponent.
    /// - Returns `x * 2^n`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_scalbnf`]<br>
    /// See also [`SDL_pow`]<br>
    pub fn SDL_scalbn(x: ::core::ffi::c_double, n: ::core::ffi::c_int) -> ::core::ffi::c_double;
}

extern "C" {
    /// Scale `x` by an integer power of two.
    ///
    /// Multiplies `x` by the `n`th power of the floating point radix (always 2).
    ///
    /// Domain: `-INF <= x <= INF`, `n` integer
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_scalbn`] for double-precision floats.
    ///
    /// - `x`: floating point value to be scaled.
    /// - `n`: integer exponent.
    /// - Returns `x * 2^n`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_scalbn`]<br>
    /// See also [`SDL_powf`]<br>
    pub fn SDL_scalbnf(x: ::core::ffi::c_float, n: ::core::ffi::c_int) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the sine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_sinf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns sine of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_sinf`]<br>
    /// See also [`SDL_asin`]<br>
    /// See also [`SDL_cos`]<br>
    pub fn SDL_sin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the sine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_sinf`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns sine of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_sin`]<br>
    /// See also [`SDL_asinf`]<br>
    /// See also [`SDL_cosf`]<br>
    pub fn SDL_sinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the square root of `x`.
    ///
    /// Domain: `0 <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_sqrtf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than or equal to 0.
    /// - Returns square root of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_sqrtf`]<br>
    pub fn SDL_sqrt(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the square root of `x`.
    ///
    /// Domain: `0 <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_sqrt`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value. Must be greater than or equal to 0.
    /// - Returns square root of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_sqrt`]<br>
    pub fn SDL_sqrtf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

extern "C" {
    /// Compute the tangent of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_tanf`] for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns tangent of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_tanf`]<br>
    /// See also [`SDL_sin`]<br>
    /// See also [`SDL_cos`]<br>
    /// See also [`SDL_atan`]<br>
    /// See also [`SDL_atan2`]<br>
    pub fn SDL_tan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

extern "C" {
    /// Compute the tangent of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_tanf`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// - `x`: floating point value, in radians.
    /// - Returns tangent of `x`.
    ///
    /// Thread safety: It is safe to call this function from any thread.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_tan`]<br>
    /// See also [`SDL_sinf`]<br>
    /// See also [`SDL_cosf`]<br>
    /// See also [`SDL_atanf`]<br>
    /// See also [`SDL_atan2f`]<br>
    pub fn SDL_tanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

pub const SDL_ICONV_ERROR: ::core::primitive::usize = (-1_i32 as ::core::primitive::usize);

pub const SDL_ICONV_E2BIG: ::core::primitive::usize = (-2_i32 as ::core::primitive::usize);

pub const SDL_ICONV_EILSEQ: ::core::primitive::usize = (-3_i32 as ::core::primitive::usize);

pub const SDL_ICONV_EINVAL: ::core::primitive::usize = (-4_i32 as ::core::primitive::usize);

pub type SDL_iconv_t = *mut SDL_iconv_data_t;

extern "C" {
    /// This function allocates a context for the specified character set
    /// conversion.
    ///
    /// - `tocode`: The target character encoding, must not be NULL.
    /// - `fromcode`: The source character encoding, must not be NULL.
    /// - Returns a handle that must be freed with [`SDL_iconv_close`], or
    ///   [`SDL_ICONV_ERROR`] on failure.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_iconv`]<br>
    /// See also [`SDL_iconv_close`]<br>
    /// See also [`SDL_iconv_string`]<br>
    pub fn SDL_iconv_open(
        tocode: *const ::core::ffi::c_char,
        fromcode: *const ::core::ffi::c_char,
    ) -> SDL_iconv_t;
}

extern "C" {
    /// This function frees a context used for character set conversion.
    ///
    /// - `cd`: The character set conversion handle.
    /// - Returns 0 on success, or -1 on failure.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_iconv`]<br>
    /// See also [`SDL_iconv_open`]<br>
    /// See also [`SDL_iconv_string`]<br>
    pub fn SDL_iconv_close(cd: SDL_iconv_t) -> ::core::ffi::c_int;
}

extern "C" {
    /// This function converts text between encodings, reading from and writing to
    /// a buffer.
    ///
    /// It returns the number of succesful conversions.
    ///
    /// - `cd`: The character set conversion context, created in
    ///   [`SDL_iconv_open()`].
    /// - `inbuf`: Address of variable that points to the first character of the
    ///   input sequence.
    /// - `inbytesleft`: The number of bytes in the input buffer.
    /// - `outbuf`: Address of variable that points to the output buffer.
    /// - `outbytesleft`: The number of bytes in the output buffer.
    /// - Returns the number of conversions on success, else [`SDL_ICONV_E2BIG`] is
    ///   returned when the output buffer is too small, or [`SDL_ICONV_EILSEQ`]
    ///   is returned when an invalid input sequence is encountered, or
    ///   [`SDL_ICONV_EINVAL`] is returned when an incomplete input sequence is
    ///   encountered.
    ///
    /// ```text
    ///      On exit:
    ///
    ///      - inbuf will point to the beginning of the next multibyte
    ///        sequence. On error, this is the location of the problematic
    ///        input sequence. On success, this is the end of the input
    ///        sequence. - inbytesleft will be set to the number of bytes left
    ///        to convert, which will be 0 on success. - outbuf will point to
    ///        the location where to store the next output byte. - outbytesleft
    ///        will be set to the number of bytes left in the output buffer.
    /// ```
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_iconv_open`]<br>
    /// See also [`SDL_iconv_close`]<br>
    /// See also [`SDL_iconv_string`]<br>
    pub fn SDL_iconv(
        cd: SDL_iconv_t,
        inbuf: *mut *const ::core::ffi::c_char,
        inbytesleft: *mut ::core::primitive::usize,
        outbuf: *mut *mut ::core::ffi::c_char,
        outbytesleft: *mut ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

extern "C" {
    /// Helper function to convert a string's encoding in one call.
    ///
    /// This function converts a buffer or string between encodings in one pass.
    ///
    /// The string does not need to be NULL-terminated; this function operates on
    /// the number of bytes specified in `inbytesleft` whether there is a NULL
    /// character anywhere in the buffer.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// [`SDL_free`] when no longer needed.
    ///
    /// - `tocode`: the character encoding of the output string. Examples are
    ///   "UTF-8", "UCS-4", etc.
    /// - `fromcode`: the character encoding of data in `inbuf`.
    /// - `inbuf`: the string to convert to a different encoding.
    /// - `inbytesleft`: the size of the input string _in bytes_.
    /// - Returns a new string, converted to the new encoding, or NULL on error.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_iconv_open`]<br>
    /// See also [`SDL_iconv_close`]<br>
    /// See also [`SDL_iconv`]<br>
    pub fn SDL_iconv_string(
        tocode: *const ::core::ffi::c_char,
        fromcode: *const ::core::ffi::c_char,
        inbuf: *const ::core::ffi::c_char,
        inbytesleft: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

#[inline(always)]
pub unsafe fn SDL_iconv_utf8_locale(S: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        SDL_iconv_string(
            c"".as_ptr(),
            c"UTF-8".as_ptr(),
            S,
            (unsafe { SDL_strlen(S) } + 1_usize),
        )
    }
}

#[inline(always)]
pub unsafe fn SDL_iconv_utf8_ucs2(S: *const ::core::ffi::c_char) -> *mut Uint16 {
    (unsafe {
        SDL_iconv_string(
            c"UCS-2".as_ptr(),
            c"UTF-8".as_ptr(),
            S,
            (unsafe { SDL_strlen(S) } + 1_usize),
        )
    } as *mut Uint16)
}

#[inline(always)]
pub unsafe fn SDL_iconv_utf8_ucs4(S: *const ::core::ffi::c_char) -> *mut Uint32 {
    (unsafe {
        SDL_iconv_string(
            c"UCS-4".as_ptr(),
            c"UTF-8".as_ptr(),
            S,
            (unsafe { SDL_strlen(S) } + 1_usize),
        )
    } as *mut Uint32)
}

#[inline(always)]
pub unsafe fn SDL_iconv_wchar_utf8(S: *const crate::ffi::c_wchar_t) -> *mut ::core::ffi::c_char {
    unsafe {
        SDL_iconv_string(
            c"UTF-8".as_ptr(),
            c"WCHAR_T".as_ptr(),
            (S as *mut ::core::ffi::c_char),
            ((unsafe { SDL_wcslen(S) } + 1_usize)
                * ::core::mem::size_of::<crate::ffi::c_wchar_t>()),
        )
    }
}

/// Multiply two integers, checking for overflow.
///
/// If `a * b` would overflow, return false.
///
/// Otherwise store `a * b` via ret and return true.
///
/// - `a`: the multiplicand.
/// - `b`: the multiplier.
/// - `ret`: on non-overflow output, stores the multiplication result, may
///   not be NULL.
/// - Returns false on overflow, true if result is multiplied without overflow.
///
/// Thread safety: It is safe to call this function from any thread.
///
/// This function is available since SDL 3.0.0.
#[inline(always)]
pub unsafe fn SDL_size_mul_check_overflow(
    a: ::core::primitive::usize,
    b: ::core::primitive::usize,
    ret: *mut ::core::primitive::usize,
) -> ::core::primitive::bool {
    if ((a != 0_usize) && (b > (::core::primitive::usize::MAX / a))) {
        return false;
    }
    unsafe {
        let (ptr, value) = (ret, (a * b));
        ptr.write(value);
        value
    };
    return true;
}

#[cfg(not(doc))]
emit! {}

/// Add two integers, checking for overflow.
///
/// If `a + b` would overflow, return -1.
///
/// Otherwise store `a + b` via ret and return 0.
///
/// - `a`: the first addend.
/// - `b`: the second addend.
/// - `ret`: on non-overflow output, stores the addition result, may not be
///   NULL.
/// - Returns false on overflow, true if result is added without overflow.
///
/// Thread safety: It is safe to call this function from any thread.
///
/// This function is available since SDL 3.0.0.
#[inline(always)]
pub unsafe fn SDL_size_add_check_overflow(
    a: ::core::primitive::usize,
    b: ::core::primitive::usize,
    ret: *mut ::core::primitive::usize,
) -> ::core::primitive::bool {
    if (b > (::core::primitive::usize::MAX - a)) {
        return false;
    }
    unsafe {
        let (ptr, value) = (ret, (a + b));
        ptr.write(value);
        value
    };
    return true;
}

#[cfg(not(doc))]
emit! {}

#[cfg(doc)]
emit! {
    /// A generic function pointer.
    ///
    /// In theory, generic function pointers should use this, instead of `void *`,
    /// since some platforms could treat code addresses differently than data
    /// addresses. Although in current times no popular platforms make this
    /// distinction, it is more correct and portable to use the correct type for a
    /// generic pointer.
    ///
    /// If for some reason you need to force this typedef to be an actual `void *`,
    /// perhaps to work around a compiler or existing code, you can define
    /// `SDL_FUNCTION_POINTER_IS_VOID_POINTER` before including any SDL headers.
    ///
    /// This datatype is available since SDL 3.0.0.
    pub type SDL_FunctionPointer = ::core::option::Option<unsafe extern "C" fn()>;

}

#[cfg(not(doc))]
emit! {
    pub type SDL_FunctionPointer = ::core::option::Option<unsafe extern "C" fn()>;

}

/// A thread-safe set of environment variables
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetEnvironment
/// \sa SDL_CreateEnvironment
/// \sa SDL_GetEnvironmentVariable
/// \sa SDL_GetEnvironmentVariables
/// \sa SDL_SetEnvironmentVariable
/// \sa SDL_UnsetEnvironmentVariable
/// \sa SDL_DestroyEnvironment
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Environment {
    _opaque: [::core::primitive::u8; 0],
}

#[repr(C)]
#[non_exhaustive]
pub struct SDL_iconv_data_t {
    _opaque: [::core::primitive::u8; 0],
}
