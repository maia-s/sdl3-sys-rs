//! SDL provides its own implementation of some of the most important C runtime
//! functions.
//!
//! Using these functions allows an app to have access to common C
//! functionality without depending on a specific C runtime (or a C runtime at
//! all). More importantly, the SDL implementations work identically across
//! platforms, so apps can avoid surprises like snprintf() behaving differently
//! between Windows and Linux builds, or itoa() only existing on some
//! platforms.
//!
//! For many of the most common functions, like [`SDL_memcpy`], SDL might just call
//! through to the usual C runtime behind the scenes, if it makes sense to do
//! so (if it's faster and always available/reliable on a given platform),
//! reducing library size and offering the most optimized option.
//!
//! SDL also offers other C-runtime-adjacent functionality in this header that
//! either isn't, strictly speaking, part of any C runtime standards, like
//! [`SDL_crc32()`] and [`SDL_reinterpret_cast`], etc. It also offers a few better
//! options, like [`SDL_strlcpy()`], which functions as a safer form of strcpy().

apply_cfg!(#[cfg(doc)] => {
    /// Don't let SDL use "long long" C types.
    ///
    /// SDL will define this if it believes the compiler doesn't understand the
    /// "long long" syntax for C datatypes. This can happen on older compilers.
    ///
    /// If _your_ compiler doesn't support "long long" but SDL doesn't know it, it
    /// is safe to define this yourself to build against the SDL headers.
    ///
    /// If this is defined, it will remove access to some C runtime support
    /// functions, like [`SDL_ulltoa`] and [`SDL_strtoll`] that refer to this datatype
    /// explicitly. The rest of SDL will still be available.
    ///
    /// SDL's own source code cannot be built with a compiler that has this
    /// defined, for various technical reasons.
    pub const SDL_NOLONGLONG: ::core::primitive::i32 = 1;

});

apply_cfg!(#[cfg(not(doc))] => {
});

apply_cfg!(#[cfg(doc)] => {
    /// The largest value that a `size_t` can hold for the target platform.
    ///
    /// `size_t` is generally the same size as a pointer in modern times, but this
    /// can get weird on very old and very esoteric machines. For example, on a
    /// 16-bit Intel 286, you might have a 32-bit "far" pointer (16-bit segment
    /// plus 16-bit offset), but `size_t` is 16 bits, because it can only deal with
    /// the offset into an individual segment.
    ///
    /// In modern times, it's generally expected to cover an entire linear address
    /// space. But be careful!
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_SIZE_MAX: ::core::primitive::usize = ::core::primitive::usize::MAX;

});

apply_cfg!(#[cfg(not(doc))] => {
    pub const SDL_SIZE_MAX: ::core::primitive::usize = ::core::primitive::usize::MAX;

});

apply_cfg!(#[cfg(doc)] => {
});

apply_cfg!(#[cfg(not(doc))] => {
});

apply_cfg!(#[cfg(doc)] => {
});

apply_cfg!(#[cfg(not(doc))] => {
});

apply_cfg!(#[cfg(doc)] => {
});

apply_cfg!(#[cfg(not(doc))] => {
});

/// A signed 8-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Sint8 = ::core::primitive::i8;

pub const SDL_MAX_SINT8: Sint8 = (0x7f as Sint8);

pub const SDL_MIN_SINT8: Sint8 = ((-128_i32) as Sint8);

/// An unsigned 8-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Uint8 = ::core::primitive::u8;

pub const SDL_MAX_UINT8: Uint8 = (0xff as Uint8);

pub const SDL_MIN_UINT8: Uint8 = (0x00 as Uint8);

/// A signed 16-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Sint16 = ::core::primitive::i16;

pub const SDL_MAX_SINT16: Sint16 = (0x7fff as Sint16);

pub const SDL_MIN_SINT16: Sint16 = ((-32768_i32) as Sint16);

/// An unsigned 16-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Uint16 = ::core::primitive::u16;

pub const SDL_MAX_UINT16: Uint16 = (0xffff as Uint16);

pub const SDL_MIN_UINT16: Uint16 = (0x0000 as Uint16);

/// A signed 32-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Sint32 = ::core::primitive::i32;

pub const SDL_MAX_SINT32: Sint32 = (0x7fffffff as Sint32);

pub const SDL_MIN_SINT32: Sint32 = ((-2147483648_i32) as Sint32);

/// An unsigned 32-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
pub type Uint32 = ::core::primitive::u32;

pub const SDL_MAX_UINT32: Uint32 = (0xffffffff as Uint32);

pub const SDL_MIN_UINT32: Uint32 = (0x00000000 as Uint32);

/// A signed 64-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - SDL_SINT64_C
pub type Sint64 = ::core::primitive::i64;

pub const SDL_MAX_SINT64: ::core::primitive::i64 = 9223372036854775807_i64;

pub const SDL_MIN_SINT64: ::core::primitive::i64 = -9223372036854775808_i64;

/// An unsigned 64-bit integer type.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - SDL_UINT64_C
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
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_MAX_SINT64`]
/// - [`SDL_MIN_SINT64`]
pub type SDL_Time = Sint64;

pub const SDL_MAX_TIME: ::core::primitive::i64 = SDL_MAX_SINT64;

pub const SDL_MIN_TIME: ::core::primitive::i64 = SDL_MIN_SINT64;

pub const SDL_FLT_EPSILON: ::core::ffi::c_float = ::core::primitive::f32::EPSILON;

apply_cfg!(#[cfg(doc)] => {
    /// A printf-formatting string for an Sint64 value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIs64 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIs64: *const ::core::ffi::c_char = c"lld".as_ptr();

    /// A printf-formatting string for a Uint64 value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIu64 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIu64: *const ::core::ffi::c_char = c"llu".as_ptr();

    /// A printf-formatting string for a Uint64 value as lower-case hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIx64 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIx64: *const ::core::ffi::c_char = c"llx".as_ptr();

    /// A printf-formatting string for a Uint64 value as upper-case hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIX64 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIX64: *const ::core::ffi::c_char = c"llX".as_ptr();

    /// A printf-formatting string for an Sint32 value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIs32 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIs32: *const ::core::ffi::c_char = c"d".as_ptr();

    /// A printf-formatting string for a Uint32 value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIu32 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIu32: *const ::core::ffi::c_char = c"u".as_ptr();

    /// A printf-formatting string for a Uint32 value as lower-case hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIx32 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIx32: *const ::core::ffi::c_char = c"x".as_ptr();

    /// A printf-formatting string for a Uint32 value as upper-case hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRIX32 " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRIX32: *const ::core::ffi::c_char = c"X".as_ptr();

    /// A printf-formatting string prefix for a `long long` value.
    ///
    /// This is just the prefix! You probably actually want [`SDL_PRILLd`], [`SDL_PRILLu`],
    /// [`SDL_PRILLx`], or [`SDL_PRILLX`] instead.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRILL_PREFIX "d bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRILL_PREFIX: *const ::core::ffi::c_char = c"ll".as_ptr();

    /// A printf-formatting string for a `long long` value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRILLd " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRILLd: *const ::core::ffi::c_char = c"lld".as_ptr();

    /// A printf-formatting string for a `unsigned long long` value.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRILLu " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRILLu: *const ::core::ffi::c_char = c"llu".as_ptr();

    /// A printf-formatting string for an `unsigned long long` value as lower-case
    /// hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRILLx " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRILLx: *const ::core::ffi::c_char = c"llx".as_ptr();

    /// A printf-formatting string for an `unsigned long long` value as upper-case
    /// hexadecimal.
    ///
    /// Use it like this:
    ///
    /// ```c
    /// SDL_Log("There are %" SDL_PRILLX " bottles of beer on the wall.", bottles);
    /// ```
    ///
    /// ## Availability
    /// This macro is available since SDL 3.2.0.
    pub const SDL_PRILLX: *const ::core::ffi::c_char = c"llX".as_ptr();

});

apply_cfg!(#[cfg(any(doc, windows))] => {
    #[cfg(not(doc))]
    pub const SDL_PRIs64: *const ::core::ffi::c_char = c"I64d".as_ptr();

});

apply_cfg!(#[cfg(not(any(doc, windows)))] => {
    apply_cfg!(#[cfg(all(not(any(doc, target_vendor = "apple")), not(target_os = "emscripten"), all(not(windows), target_pointer_width = "64")))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIs64: *const ::core::ffi::c_char = c"ld".as_ptr();

    });

    apply_cfg!(#[cfg(not(all(not(any(doc, target_vendor = "apple")), not(target_os = "emscripten"), all(not(windows), target_pointer_width = "64"))))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIs64: *const ::core::ffi::c_char = c"lld".as_ptr();

    });

});

apply_cfg!(#[cfg(any(doc, windows))] => {
    #[cfg(not(doc))]
    pub const SDL_PRIu64: *const ::core::ffi::c_char = c"I64u".as_ptr();

});

apply_cfg!(#[cfg(not(any(doc, windows)))] => {
    apply_cfg!(#[cfg(all(not(any(doc, target_vendor = "apple")), not(target_os = "emscripten"), all(not(windows), target_pointer_width = "64")))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIu64: *const ::core::ffi::c_char = c"lu".as_ptr();

    });

    apply_cfg!(#[cfg(not(all(not(any(doc, target_vendor = "apple")), not(target_os = "emscripten"), all(not(windows), target_pointer_width = "64"))))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIu64: *const ::core::ffi::c_char = c"llu".as_ptr();

    });

});

apply_cfg!(#[cfg(any(doc, windows))] => {
    #[cfg(not(doc))]
    pub const SDL_PRIx64: *const ::core::ffi::c_char = c"I64x".as_ptr();

});

apply_cfg!(#[cfg(not(any(doc, windows)))] => {
    apply_cfg!(#[cfg(all(not(any(doc, target_vendor = "apple")), all(not(windows), target_pointer_width = "64")))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIx64: *const ::core::ffi::c_char = c"lx".as_ptr();

    });

    apply_cfg!(#[cfg(not(all(not(any(doc, target_vendor = "apple")), all(not(windows), target_pointer_width = "64"))))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIx64: *const ::core::ffi::c_char = c"llx".as_ptr();

    });

});

apply_cfg!(#[cfg(any(doc, windows))] => {
    #[cfg(not(doc))]
    pub const SDL_PRIX64: *const ::core::ffi::c_char = c"I64X".as_ptr();

});

apply_cfg!(#[cfg(not(any(doc, windows)))] => {
    apply_cfg!(#[cfg(all(not(any(doc, target_vendor = "apple")), all(not(windows), target_pointer_width = "64")))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIX64: *const ::core::ffi::c_char = c"lX".as_ptr();

    });

    apply_cfg!(#[cfg(not(all(not(any(doc, target_vendor = "apple")), all(not(windows), target_pointer_width = "64"))))] => {
        #[cfg(not(doc))]
        pub const SDL_PRIX64: *const ::core::ffi::c_char = c"llX".as_ptr();

    });

});

#[cfg(not(doc))]
pub const SDL_PRIs32: *const ::core::ffi::c_char = c"d".as_ptr();

#[cfg(not(doc))]
pub const SDL_PRIu32: *const ::core::ffi::c_char = c"u".as_ptr();

#[cfg(not(doc))]
pub const SDL_PRIx32: *const ::core::ffi::c_char = c"x".as_ptr();

#[cfg(not(doc))]
pub const SDL_PRIX32: *const ::core::ffi::c_char = c"X".as_ptr();

apply_cfg!(#[cfg(any(doc, windows))] => {
    const _: () = ::core::assert!((::core::mem::size_of::<::core::ffi::c_longlong>() == 8_usize));

    #[cfg(not(doc))]
    pub const SDL_PRILL_PREFIX: *const ::core::ffi::c_char = c"I64".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLd: *const ::core::ffi::c_char = c"I64d".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLu: *const ::core::ffi::c_char = c"I64u".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLx: *const ::core::ffi::c_char = c"I64x".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLX: *const ::core::ffi::c_char = c"I64X".as_ptr();

});

apply_cfg!(#[cfg(not(any(doc, windows)))] => {
    #[cfg(not(doc))]
    pub const SDL_PRILL_PREFIX: *const ::core::ffi::c_char = c"ll".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLd: *const ::core::ffi::c_char = c"lld".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLu: *const ::core::ffi::c_char = c"llu".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLx: *const ::core::ffi::c_char = c"llx".as_ptr();

    #[cfg(not(doc))]
    pub const SDL_PRILLX: *const ::core::ffi::c_char = c"llX".as_ptr();

});

apply_cfg!(#[cfg(doc)] => {
});

apply_cfg!(#[cfg(not(doc))] => {
});

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
/// ## Parameters
/// - `A`: the first ASCII character.
/// - `B`: the second ASCII character.
/// - `C`: the third ASCII character.
/// - `D`: the fourth ASCII character.
///
/// ## Return value
/// Returns the four characters converted into a Uint32, one character
///   per-byte.
///
/// ## Thread safety
/// It is safe to call this macro from any thread.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
#[inline(always)]
pub const fn SDL_FOURCC(A: Uint8, B: Uint8, C: Uint8, D: Uint8) -> Uint32 {
    (((((A as Uint32) << 0) | ((B as Uint32) << 8)) | ((C as Uint32) << 16))
        | ((D as Uint32) << 24))
}

#[doc(hidden)]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_alignment_test {
    pub a: Uint8,
    pub b: *mut ::core::ffi::c_void,
}

impl ::core::default::Default for SDL_alignment_test {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

const _: () = ::core::assert!(
    (::core::mem::size_of::<SDL_alignment_test>()
        == (2 * ::core::mem::size_of::<*mut ::core::ffi::c_void>()))
);

const _: () = ::core::assert!((!(0 as ::core::ffi::c_int) == (-1_i32 as ::core::ffi::c_int)));

apply_cfg!(#[cfg(all(not(any(doc, target_os = "horizon")), not(any(doc, target_os = "vita"))))] => {
    #[doc(hidden)]
    /// ## Known values (`sdl3-sys`)
    /// | Associated constant | Global constant | Description |
    /// | ------------------- | --------------- | ----------- |
    /// | [`DUMMY_ENUM_VALUE`](SDL_DUMMY_ENUM::DUMMY_ENUM_VALUE) | [`DUMMY_ENUM_VALUE`] | |
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    #[derive(Default)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SDL_DUMMY_ENUM(pub ::core::ffi::c_int);

    impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_DUMMY_ENUM {
        #[inline(always)]
        fn eq(&self, other: &::core::ffi::c_int) -> bool {
            &self.0 == other
        }
    }

    impl ::core::cmp::PartialEq<SDL_DUMMY_ENUM> for ::core::ffi::c_int {
        #[inline(always)]
        fn eq(&self, other: &SDL_DUMMY_ENUM) -> bool {
            self == &other.0
        }
    }

    impl From<SDL_DUMMY_ENUM> for ::core::ffi::c_int {
        #[inline(always)]
        fn from(value: SDL_DUMMY_ENUM) -> Self {
            value.0
        }
    }

    #[cfg(feature = "debug-impls")]
    impl ::core::fmt::Debug for SDL_DUMMY_ENUM {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            #[allow(unreachable_patterns)]
            f.write_str(match *self {
                Self::DUMMY_ENUM_VALUE => "DUMMY_ENUM_VALUE",

                _ => return write!(f, "SDL_DUMMY_ENUM({})", self.0),
            })
        }
    }

    impl SDL_DUMMY_ENUM {
        pub const DUMMY_ENUM_VALUE: Self = Self((0 as ::core::ffi::c_int));
    }

    #[doc(hidden)]
    pub const DUMMY_ENUM_VALUE: SDL_DUMMY_ENUM = SDL_DUMMY_ENUM::DUMMY_ENUM_VALUE;

    impl SDL_DUMMY_ENUM {
        /// Initialize a `SDL_DUMMY_ENUM` from a raw value.
        #[inline(always)]
        pub const fn new(value: ::core::ffi::c_int) -> Self {
            Self(value)
        }
    }

    impl SDL_DUMMY_ENUM {
        /// Get a copy of the inner raw value.
        #[inline(always)]
        pub const fn value(&self) -> ::core::ffi::c_int {
            self.0
        }
    }

    const _: () = ::core::assert!((::core::mem::size_of::<SDL_DUMMY_ENUM>() == ::core::mem::size_of::<::core::ffi::c_int>()));

});

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
/// ## Thread safety
/// It is safe to call this macro from any thread.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_IOStreamInterface`]
/// - [`SDL_StorageInterface`]
/// - [`SDL_VirtualJoystickDesc`]
///
/// ## Safety (`sdl3-sys`)
/// - `iface` must point to memory that is valid for writing the type `T`.
/// - The type `T` must be a `repr(C)` struct.
/// - The first field of the struct must be of type `u32`. It will be set to
///   the size of the struct in bytes.
/// - The rest of the struct will be initialized as all zero bytes.
#[inline(always)]
pub unsafe fn SDL_INIT_INTERFACE<T>(iface: *mut T) {
    const { ::core::assert!(::core::mem::size_of::<T>() <= ::core::primitive::u32::MAX as usize) };
    unsafe {
        iface.write_bytes(0, 1);
        iface
            .cast::<Uint32>()
            .write(::core::mem::size_of::<T>() as Uint32);
    }
}

apply_cfg!(#[cfg(doc)] => {
});

apply_cfg!(#[cfg(not(doc))] => {
});

unsafe extern "C" {
    /// Allocate uninitialized memory.
    ///
    /// The allocated memory returned by this function must be freed with
    /// [`SDL_free()`].
    ///
    /// If `size` is 0, it will be set to 1.
    ///
    /// If the allocation is successful, the returned pointer is guaranteed to be
    /// aligned to either the *fundamental alignment* (`alignof(max_align_t)` in
    /// C11 and later) or `2 * sizeof(void *)`, whichever is smaller. Use
    /// [`SDL_aligned_alloc()`] if you need to allocate memory aligned to an alignment
    /// greater than this guarantee.
    ///
    /// ## Parameters
    /// - `size`: the size to allocate.
    ///
    /// ## Return value
    /// Returns a pointer to the allocated memory, or NULL if allocation failed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_free`]
    /// - [`SDL_calloc`]
    /// - [`SDL_realloc`]
    /// - [`SDL_aligned_alloc`]
    pub fn SDL_malloc(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}

unsafe extern "C" {
    /// Allocate a zero-initialized array.
    ///
    /// The memory returned by this function must be freed with [`SDL_free()`].
    ///
    /// If either of `nmemb` or `size` is 0, they will both be set to 1.
    ///
    /// If the allocation is successful, the returned pointer is guaranteed to be
    /// aligned to either the *fundamental alignment* (`alignof(max_align_t)` in
    /// C11 and later) or `2 * sizeof(void *)`, whichever is smaller.
    ///
    /// ## Parameters
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of each element of the array.
    ///
    /// ## Return value
    /// Returns a pointer to the allocated array, or NULL if allocation failed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_free`]
    /// - [`SDL_malloc`]
    /// - [`SDL_realloc`]
    pub fn SDL_calloc(
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

unsafe extern "C" {
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
    /// If the allocation is successfully resized, the returned pointer is
    /// guaranteed to be aligned to either the *fundamental alignment*
    /// (`alignof(max_align_t)` in C11 and later) or `2 * sizeof(void *)`,
    /// whichever is smaller.
    ///
    /// ## Parameters
    /// - `mem`: a pointer to allocated memory to reallocate, or NULL.
    /// - `size`: the new size of the memory.
    ///
    /// ## Return value
    /// Returns a pointer to the newly allocated memory, or NULL if allocation
    ///   failed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_free`]
    /// - [`SDL_malloc`]
    /// - [`SDL_calloc`]
    pub fn SDL_realloc(
        mem: *mut ::core::ffi::c_void,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

unsafe extern "C" {
    /// Free allocated memory.
    ///
    /// The pointer is no longer valid after this call and cannot be dereferenced
    /// anymore.
    ///
    /// If `mem` is NULL, this function does nothing.
    ///
    /// ## Parameters
    /// - `mem`: a pointer to allocated memory, or NULL.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_malloc`]
    /// - [`SDL_calloc`]
    /// - [`SDL_realloc`]
    pub fn SDL_free(mem: *mut ::core::ffi::c_void);
}

/// A callback used to implement [`SDL_malloc()`].
///
/// SDL will always ensure that the passed `size` is greater than 0.
///
/// ## Parameters
/// - `size`: the size to allocate.
///
/// ## Return value
/// Returns a pointer to the allocated memory, or NULL if allocation failed.
///
/// ## Thread safety
/// It should be safe to call this callback from any thread.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_malloc`]
/// - [`SDL_GetOriginalMemoryFunctions`]
/// - [`SDL_GetMemoryFunctions`]
/// - [`SDL_SetMemoryFunctions`]
pub type SDL_malloc_func = ::core::option::Option<
    unsafe extern "C" fn(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void,
>;

/// A callback used to implement [`SDL_calloc()`].
///
/// SDL will always ensure that the passed `nmemb` and `size` are both greater
/// than 0.
///
/// ## Parameters
/// - `nmemb`: the number of elements in the array.
/// - `size`: the size of each element of the array.
///
/// ## Return value
/// Returns a pointer to the allocated array, or NULL if allocation failed.
///
/// ## Thread safety
/// It should be safe to call this callback from any thread.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_calloc`]
/// - [`SDL_GetOriginalMemoryFunctions`]
/// - [`SDL_GetMemoryFunctions`]
/// - [`SDL_SetMemoryFunctions`]
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
/// ## Parameters
/// - `mem`: a pointer to allocated memory to reallocate, or NULL.
/// - `size`: the new size of the memory.
///
/// ## Return value
/// Returns a pointer to the newly allocated memory, or NULL if allocation
///   failed.
///
/// ## Thread safety
/// It should be safe to call this callback from any thread.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_realloc`]
/// - [`SDL_GetOriginalMemoryFunctions`]
/// - [`SDL_GetMemoryFunctions`]
/// - [`SDL_SetMemoryFunctions`]
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
/// ## Parameters
/// - `mem`: a pointer to allocated memory.
///
/// ## Thread safety
/// It should be safe to call this callback from any thread.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_free`]
/// - [`SDL_GetOriginalMemoryFunctions`]
/// - [`SDL_GetMemoryFunctions`]
/// - [`SDL_SetMemoryFunctions`]
pub type SDL_free_func =
    ::core::option::Option<unsafe extern "C" fn(mem: *mut ::core::ffi::c_void)>;

unsafe extern "C" {
    /// Get the original set of SDL memory functions.
    ///
    /// This is what [`SDL_malloc`] and friends will use by default, if there has been
    /// no call to [`SDL_SetMemoryFunctions`]. This is not necessarily using the C
    /// runtime's `malloc` functions behind the scenes! Different platforms and
    /// build configurations might do any number of unexpected things.
    ///
    /// ## Parameters
    /// - `malloc_func`: filled with malloc function.
    /// - `calloc_func`: filled with calloc function.
    /// - `realloc_func`: filled with realloc function.
    /// - `free_func`: filled with free function.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetOriginalMemoryFunctions(
        malloc_func: *mut SDL_malloc_func,
        calloc_func: *mut SDL_calloc_func,
        realloc_func: *mut SDL_realloc_func,
        free_func: *mut SDL_free_func,
    );
}

unsafe extern "C" {
    /// Get the current set of SDL memory functions.
    ///
    /// ## Parameters
    /// - `malloc_func`: filled with malloc function.
    /// - `calloc_func`: filled with calloc function.
    /// - `realloc_func`: filled with realloc function.
    /// - `free_func`: filled with free function.
    ///
    /// ## Thread safety
    /// This does not hold a lock, so do not call this in the
    ///   unlikely event of a background thread calling
    ///   [`SDL_SetMemoryFunctions`] simultaneously.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetMemoryFunctions`]
    /// - [`SDL_GetOriginalMemoryFunctions`]
    pub fn SDL_GetMemoryFunctions(
        malloc_func: *mut SDL_malloc_func,
        calloc_func: *mut SDL_calloc_func,
        realloc_func: *mut SDL_realloc_func,
        free_func: *mut SDL_free_func,
    );
}

unsafe extern "C" {
    /// Replace SDL's memory allocation functions with a custom set.
    ///
    /// It is not safe to call this function once any allocations have been made,
    /// as future calls to [`SDL_free`] will use the new allocator, even if they came
    /// from an [`SDL_malloc`] made with the old one!
    ///
    /// If used, usually this needs to be the first call made into the SDL library,
    /// if not the very first thing done at program startup time.
    ///
    /// ## Parameters
    /// - `malloc_func`: custom malloc function.
    /// - `calloc_func`: custom calloc function.
    /// - `realloc_func`: custom realloc function.
    /// - `free_func`: custom free function.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but one
    ///   should not replace the memory functions once any allocations
    ///   are made!
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetMemoryFunctions`]
    /// - [`SDL_GetOriginalMemoryFunctions`]
    pub fn SDL_SetMemoryFunctions(
        malloc_func: SDL_malloc_func,
        calloc_func: SDL_calloc_func,
        realloc_func: SDL_realloc_func,
        free_func: SDL_free_func,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `alignment`: the alignment of the memory.
    /// - `size`: the size to allocate.
    ///
    /// ## Return value
    /// Returns a pointer to the aligned memory, or NULL if allocation failed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_aligned_free`]
    pub fn SDL_aligned_alloc(
        alignment: ::core::primitive::usize,
        size: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

unsafe extern "C" {
    /// Free memory allocated by [`SDL_aligned_alloc()`].
    ///
    /// The pointer is no longer valid after this call and cannot be dereferenced
    /// anymore.
    ///
    /// If `mem` is NULL, this function does nothing.
    ///
    /// ## Parameters
    /// - `mem`: a pointer previously returned by [`SDL_aligned_alloc()`], or NULL.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_aligned_alloc`]
    pub fn SDL_aligned_free(mem: *mut ::core::ffi::c_void);
}

unsafe extern "C" {
    /// Get the number of outstanding (unfreed) allocations.
    ///
    /// ## Return value
    /// Returns the number of allocations or -1 if allocation counting is
    ///   disabled.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_GetNumAllocations() -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Get the process environment.
    ///
    /// This is initialized at application start and is not affected by setenv()
    /// and unsetenv() calls after that point. Use [`SDL_SetEnvironmentVariable()`] and
    /// [`SDL_UnsetEnvironmentVariable()`] if you want to modify this environment, or
    /// [`SDL_setenv_unsafe()`] or [`SDL_unsetenv_unsafe()`] if you want changes to persist
    /// in the C runtime environment after [`SDL_Quit()`].
    ///
    /// ## Return value
    /// Returns a pointer to the environment for the process or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironmentVariable`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_SetEnvironmentVariable`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_GetEnvironment() -> *mut SDL_Environment;
}

unsafe extern "C" {
    /// Create a set of environment variables
    ///
    /// ## Parameters
    /// - `populated`: true to initialize it from the C runtime environment,
    ///   false to create an empty environment.
    ///
    /// ## Return value
    /// Returns a pointer to the new environment or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// If `populated` is false, it is safe to call this function
    ///   from any thread, otherwise it is safe if no other threads are
    ///   calling setenv() or unsetenv()
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironmentVariable`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_SetEnvironmentVariable`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    /// - [`SDL_DestroyEnvironment`]
    pub fn SDL_CreateEnvironment(populated: ::core::primitive::bool) -> *mut SDL_Environment;
}

unsafe extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// ## Parameters
    /// - `env`: the environment to query.
    /// - `name`: the name of the variable to get.
    ///
    /// ## Return value
    /// Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironment`]
    /// - [`SDL_CreateEnvironment`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_SetEnvironmentVariable`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_GetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get all variables in the environment.
    ///
    /// ## Parameters
    /// - `env`: the environment to query.
    ///
    /// ## Return value
    /// Returns a NULL terminated array of pointers to environment variables in
    ///   the form "variable=value" or NULL on failure; call [`SDL_GetError()`]
    ///   for more information. This is a single allocation that should be
    ///   freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironment`]
    /// - [`SDL_CreateEnvironment`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_SetEnvironmentVariable`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_GetEnvironmentVariables(env: *mut SDL_Environment) -> *mut *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Set the value of a variable in the environment.
    ///
    /// ## Parameters
    /// - `env`: the environment to modify.
    /// - `name`: the name of the variable to set.
    /// - `value`: the value of the variable to set.
    /// - `overwrite`: true to overwrite the variable if it exists, false to
    ///   return success without setting the variable if it already
    ///   exists.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironment`]
    /// - [`SDL_CreateEnvironment`]
    /// - [`SDL_GetEnvironmentVariable`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_SetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        overwrite: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Clear a variable from the environment.
    ///
    /// ## Parameters
    /// - `env`: the environment to modify.
    /// - `name`: the name of the variable to unset.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetEnvironment`]
    /// - [`SDL_CreateEnvironment`]
    /// - [`SDL_GetEnvironmentVariable`]
    /// - [`SDL_GetEnvironmentVariables`]
    /// - [`SDL_SetEnvironmentVariable`]
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_UnsetEnvironmentVariable(
        env: *mut SDL_Environment,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Destroy a set of environment variables.
    ///
    /// ## Parameters
    /// - `env`: the environment to destroy.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, as long as
    ///   the environment is no longer in use.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateEnvironment`]
    pub fn SDL_DestroyEnvironment(env: *mut SDL_Environment);
}

unsafe extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// This function uses SDL's cached copy of the environment and is thread-safe.
    ///
    /// ## Parameters
    /// - `name`: the name of the variable to get.
    ///
    /// ## Return value
    /// Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_getenv(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get the value of a variable in the environment.
    ///
    /// This function bypasses SDL's cached copy of the environment and is not
    /// thread-safe.
    ///
    /// ## Parameters
    /// - `name`: the name of the variable to get.
    ///
    /// ## Return value
    /// Returns a pointer to the value of the variable or NULL if it can't be
    ///   found.
    ///
    /// ## Thread safety
    /// This function is not thread safe, consider using [`SDL_getenv()`]
    ///   instead.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_getenv`]
    pub fn SDL_getenv_unsafe(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Set the value of a variable in the environment.
    ///
    /// ## Parameters
    /// - `name`: the name of the variable to set.
    /// - `value`: the value of the variable to set.
    /// - `overwrite`: 1 to overwrite the variable if it exists, 0 to return
    ///   success without setting the variable if it already exists.
    ///
    /// ## Return value
    /// Returns 0 on success, -1 on error.
    ///
    /// ## Thread safety
    /// This function is not thread safe, consider using
    ///   [`SDL_SetEnvironmentVariable()`] instead.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SetEnvironmentVariable`]
    pub fn SDL_setenv_unsafe(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        overwrite: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Clear a variable from the environment.
    ///
    /// ## Parameters
    /// - `name`: the name of the variable to unset.
    ///
    /// ## Return value
    /// Returns 0 on success, -1 on error.
    ///
    /// ## Thread safety
    /// This function is not thread safe, consider using
    ///   [`SDL_UnsetEnvironmentVariable()`] instead.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_UnsetEnvironmentVariable`]
    pub fn SDL_unsetenv_unsafe(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

/// A callback used with SDL sorting and binary search functions.
///
/// ## Parameters
/// - `a`: a pointer to the first element being compared.
/// - `b`: a pointer to the second element being compared.
///
/// ## Return value
/// Returns -1 if `a` should be sorted before `b`, 1 if `b` should be sorted
///   before `a`, 0 if they are equal. If two elements are equal, their
///   order in the sorted array is undefined.
///
/// ## Availability
/// This callback is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_bsearch`]
/// - [`SDL_qsort`]
pub type SDL_CompareCallback = ::core::option::Option<
    unsafe extern "C" fn(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

unsafe extern "C" {
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
    /// ## Parameters
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_bsearch`]
    /// - [`SDL_qsort_r`]
    pub fn SDL_qsort(
        base: *mut ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback,
    );
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `key`: a pointer to a key equal to the element being searched for.
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    ///
    /// ## Return value
    /// Returns a pointer to the matching element in the array, or NULL if not
    ///   found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_bsearch_r`]
    /// - [`SDL_qsort`]
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
/// ## Parameters
/// - `userdata`: the `userdata` pointer passed to the sort function.
/// - `a`: a pointer to the first element being compared.
/// - `b`: a pointer to the second element being compared.
///
/// ## Return value
/// Returns -1 if `a` should be sorted before `b`, 1 if `b` should be sorted
///   before `a`, 0 if they are equal. If two elements are equal, their
///   order in the sorted array is undefined.
///
/// ## Availability
/// This callback is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_qsort_r`]
/// - [`SDL_bsearch_r`]
pub type SDL_CompareCallback_r = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

unsafe extern "C" {
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
    ///     if (A->key < B->key) {
    ///         return (method == sort_increasing) ? -1 : 1;
    ///     } else if (B->key < A->key) {
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
    /// ## Parameters
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    /// - `userdata`: a pointer to pass to the compare function.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_bsearch_r`]
    /// - [`SDL_qsort`]
    pub fn SDL_qsort_r(
        base: *mut ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback_r,
        userdata: *mut ::core::ffi::c_void,
    );
}

unsafe extern "C" {
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
    ///     if (A->key < B->key) {
    ///         return (method == sort_increasing) ? -1 : 1;
    ///     } else if (B->key < A->key) {
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
    /// ## Parameters
    /// - `key`: a pointer to a key equal to the element being searched for.
    /// - `base`: a pointer to the start of the array.
    /// - `nmemb`: the number of elements in the array.
    /// - `size`: the size of the elements in the array.
    /// - `compare`: a function used to compare elements in the array.
    /// - `userdata`: a pointer to pass to the compare function.
    ///
    /// ## Return value
    /// Returns a pointer to the matching element in the array, or NULL if not
    ///   found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_bsearch`]
    /// - [`SDL_qsort_r`]
    pub fn SDL_bsearch_r(
        key: *const ::core::ffi::c_void,
        base: *const ::core::ffi::c_void,
        nmemb: ::core::primitive::usize,
        size: ::core::primitive::usize,
        compare: SDL_CompareCallback_r,
        userdata: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

/// Compute the absolute value of `x`.
///
/// ## Parameters
/// - `x`: an integer value.
///
/// ## Return value
/// Returns the absolute value of x.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
#[inline(always)]
pub const fn SDL_abs(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return x.unsigned_abs() as _;
}

#[inline(always)]
pub fn SDL_min<T: Copy + PartialOrd>(x: T, y: T) -> T {
    if x < y { x } else { y }
}

#[inline(always)]
pub fn SDL_max<T: Copy + PartialOrd>(x: T, y: T) -> T {
    if x > y { x } else { y }
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

unsafe extern "C" {
    /// Query if a character is alphabetic (a letter).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z' and 'A-Z' as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isalpha(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Query if a character is alphabetic (a letter) or a number.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z', 'A-Z', and '0-9' as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isalnum(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is blank (a space or tab).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0x20 (space) or 0x9 (tab) as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isblank(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is a control character.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0 through 0x1F, and 0x7F, as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_iscntrl(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is a numeric digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// '0' (0x30) through '9' (0x39), as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is a hexadecimal digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'F', 'a' through 'f', and '0' through '9', as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isxdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is a punctuation mark.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `((SDL_isgraph(x)) && (!SDL_isalnum(x)))`.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_isgraph`]
    /// - [`SDL_isalnum`]
    pub safe fn SDL_ispunct(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isspace(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is upper case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'Z' as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is lower case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'a' through 'z' as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_islower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is "printable".
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// ' ' (0x20) through '~' (0x7E) as true.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_isprint(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Report if a character is any "printable" except space.
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `(SDL_isprint(x)) && ((x) != ' ')`.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_isprint`]
    pub safe fn SDL_isgraph(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Convert low-ASCII English letters to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'a' through 'z' to uppercase.
    ///
    /// This function returns the uppercase equivalent of `x`. If a character
    /// cannot be converted, or is already uppercase, this function returns `x`.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns capitalized version of x, or x if no conversion available.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_toupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Convert low-ASCII English letters to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function returns the lowercase equivalent of `x`. If a character
    /// cannot be converted, or is already lowercase, this function returns `x`.
    ///
    /// ## Parameters
    /// - `x`: character value to check.
    ///
    /// ## Return value
    /// Returns lowercase version of x, or x if no conversion available.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub safe fn SDL_tolower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Calculate a CRC-16 value.
    ///
    /// <https://en.wikipedia.org/wiki/Cyclic_redundancy_check>
    ///
    /// This function can be called multiple times, to stream data to be
    /// checksummed in blocks. Each call must provide the previous CRC-16 return
    /// value to be updated with the next block. The first call to this function
    /// for a set of blocks should pass in a zero CRC value.
    ///
    /// ## Parameters
    /// - `crc`: the current checksum for this data set, or 0 for a new data set.
    /// - `data`: a new block of data to add to the checksum.
    /// - `len`: the size, in bytes, of the new block of data.
    ///
    /// ## Return value
    /// Returns a CRC-16 checksum value of all blocks in the data set.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_crc16(
        crc: Uint16,
        data: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> Uint16;
}

unsafe extern "C" {
    /// Calculate a CRC-32 value.
    ///
    /// <https://en.wikipedia.org/wiki/Cyclic_redundancy_check>
    ///
    /// This function can be called multiple times, to stream data to be
    /// checksummed in blocks. Each call must provide the previous CRC-32 return
    /// value to be updated with the next block. The first call to this function
    /// for a set of blocks should pass in a zero CRC value.
    ///
    /// ## Parameters
    /// - `crc`: the current checksum for this data set, or 0 for a new data set.
    /// - `data`: a new block of data to add to the checksum.
    /// - `len`: the size, in bytes, of the new block of data.
    ///
    /// ## Return value
    /// Returns a CRC-32 checksum value of all blocks in the data set.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_crc32(
        crc: Uint32,
        data: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> Uint32;
}

unsafe extern "C" {
    /// Calculate a 32-bit MurmurHash3 value for a block of data.
    ///
    /// <https://en.wikipedia.org/wiki/MurmurHash>
    ///
    /// A seed may be specified, which changes the final results consistently, but
    /// this does not work like [`SDL_crc16`] and [`SDL_crc32`]\: you can't feed a previous
    /// result from this function back into itself as the next seed value to
    /// calculate a hash in chunks; it won't produce the same hash as it would if
    /// the same data was provided in a single call.
    ///
    /// If you aren't sure what to provide for a seed, zero is fine. Murmur3 is not
    /// cryptographically secure, so it shouldn't be used for hashing top-secret
    /// data.
    ///
    /// ## Parameters
    /// - `data`: the data to be hashed.
    /// - `len`: the size of data, in bytes.
    /// - `seed`: a value that alters the final hash value.
    ///
    /// ## Return value
    /// Returns a Murmur3 32-bit hash value.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
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
/// ## Parameters
/// - `dst`: The destination memory region. Must not be NULL, and must not
///   overlap with `src`.
/// - `src`: The source memory region. Must not be NULL, and must not overlap
///   with `dst`.
/// - `len`: The length in bytes of both `dst` and `src`.
///
/// ## Return value
/// Returns `dst`.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_memmove`]
#[inline(always)]
pub const unsafe fn SDL_memcpy(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    len: ::core::primitive::usize,
) -> *mut ::core::ffi::c_void {
    unsafe { ::core::ptr::copy_nonoverlapping(src.cast::<Uint8>(), dst.cast::<Uint8>(), len) };
    return dst;
}

/// A macro to copy memory between objects, with basic type checking.
///
/// [`SDL_memcpy`] and [`SDL_memmove`] do not care where you copy memory to and from,
/// which can lead to bugs. This macro aims to avoid most of those bugs by
/// making sure that the source and destination are both pointers to objects
/// that are the same size. It does not check that the objects are the same
/// _type_, just that the copy will not overflow either object.
///
/// The size check happens at compile time, and the compiler will throw an
/// error if the objects are different sizes.
///
/// Generally this is intended to copy a single object, not an array.
///
/// This macro looks like it double-evaluates its parameters, but the extras
/// them are in `sizeof` sections, which generate no code nor side-effects.
///
/// ## Parameters
/// - `dst`: a pointer to the destination object. Must not be NULL.
/// - `src`: a pointer to the source object. Must not be NULL.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
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

/// Copy memory ranges that might overlap.
///
/// It is okay for the memory regions to overlap. If you are confident that the
/// regions never overlap, using [`SDL_memcpy()`] may improve performance.
///
/// ## Parameters
/// - `dst`: The destination memory region. Must not be NULL.
/// - `src`: The source memory region. Must not be NULL.
/// - `len`: The length in bytes of both `dst` and `src`.
///
/// ## Return value
/// Returns `dst`.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_memcpy`]
#[inline(always)]
pub const unsafe fn SDL_memmove(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    len: ::core::primitive::usize,
) -> *mut ::core::ffi::c_void {
    unsafe { ::core::ptr::copy(src.cast::<Uint8>(), dst.cast::<Uint8>(), len) };
    return dst;
}

unsafe extern "C" {
    /// Initialize all bytes of buffer of memory to a specific value.
    ///
    /// This function will set `len` bytes, pointed to by `dst`, to the value
    /// specified in `c`.
    ///
    /// Despite `c` being an `int` instead of a `char`, this only operates on
    /// bytes; `c` must be a value between 0 and 255, inclusive.
    ///
    /// ## Parameters
    /// - `dst`: the destination memory region. Must not be NULL.
    /// - `c`: the byte value to set.
    /// - `len`: the length, in bytes, to set in `dst`.
    ///
    /// ## Return value
    /// Returns `dst`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_memset(
        dst: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        len: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

unsafe extern "C" {
    /// Initialize all 32-bit words of buffer of memory to a specific value.
    ///
    /// This function will set a buffer of `dwords` Uint32 values, pointed to by
    /// `dst`, to the value specified in `val`.
    ///
    /// Unlike [`SDL_memset`], this sets 32-bit values, not bytes, so it's not limited
    /// to a range of 0-255.
    ///
    /// ## Parameters
    /// - `dst`: the destination memory region. Must not be NULL.
    /// - `val`: the Uint32 value to set.
    /// - `dwords`: the number of Uint32 values to set in `dst`.
    ///
    /// ## Return value
    /// Returns `dst`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_memset4(
        dst: *mut ::core::ffi::c_void,
        val: Uint32,
        dwords: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_void;
}

/// Clear an object's memory to zero, using a pointer.
///
/// This is wrapper over [`SDL_memset`] that handles calculating the object size,
/// so there's no chance of copy/paste errors, and the code is cleaner.
///
/// This requires a pointer to an object, not an object itself, nor an array.
///
/// ## Parameters
/// - `x`: a pointer to the object to clear.
///
/// ## Thread safety
/// It is safe to call this macro from any thread.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_zero`]
/// - [`SDL_zeroa`]
///
/// # Safety
/// It must be valid to zero all bytes of `T`, and it must be valid to write a `T` to the memory pointed to by `x`
#[inline(always)]
pub unsafe fn SDL_zerop<T>(x: *mut T) -> *mut T {
    unsafe { x.write_bytes(0, 1) };
    x
}

unsafe extern "C" {
    /// Compare two buffers of memory.
    ///
    /// ## Parameters
    /// - `s1`: the first buffer to compare. NULL is not permitted!
    /// - `s2`: the second buffer to compare. NULL is not permitted!
    /// - `len`: the number of bytes to compare between the buffers.
    ///
    /// ## Return value
    /// Returns less than zero if s1 is "less than" s2, greater than zero if s1 is
    ///   "greater than" s2, and zero if the buffers match exactly for `len`
    ///   bytes.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_memcmp(
        s1: *const ::core::ffi::c_void,
        s2: *const ::core::ffi::c_void,
        len: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like wcslen() but doesn't require access to a C runtime.
    ///
    /// Counts the number of wchar_t values in `wstr`, excluding the null
    /// terminator.
    ///
    /// Like [`SDL_strlen`] only counts bytes and not codepoints in a UTF-8 string,
    /// this counts wchar_t values in a string, even if the string's encoding is of
    /// variable width, like UTF-16.
    ///
    /// Also be aware that wchar_t is different sizes on different platforms (4
    /// bytes on Linux, 2 on Windows, etc).
    ///
    /// ## Parameters
    /// - `wstr`: The null-terminated wide string to read. Must not be NULL.
    ///
    /// ## Return value
    /// Returns the length (in wchar_t values, excluding the null terminator) of
    ///   `wstr`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_wcsnlen`]
    /// - [`SDL_utf8strlen`]
    /// - [`SDL_utf8strnlen`]
    pub fn SDL_wcslen(wstr: *const crate::ffi::c_wchar_t) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// This works exactly like wcsnlen() but doesn't require access to a C
    /// runtime.
    ///
    /// Counts up to a maximum of `maxlen` wchar_t values in `wstr`, excluding the
    /// null terminator.
    ///
    /// Like [`SDL_strnlen`] only counts bytes and not codepoints in a UTF-8 string,
    /// this counts wchar_t values in a string, even if the string's encoding is of
    /// variable width, like UTF-16.
    ///
    /// Also be aware that wchar_t is different sizes on different platforms (4
    /// bytes on Linux, 2 on Windows, etc).
    ///
    /// Also, `maxlen` is a count of wide characters, not bytes!
    ///
    /// ## Parameters
    /// - `wstr`: The null-terminated wide string to read. Must not be NULL.
    /// - `maxlen`: The maximum amount of wide characters to count.
    ///
    /// ## Return value
    /// Returns the length (in wide characters, excluding the null terminator) of
    ///   `wstr` but never more than `maxlen`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_wcslen`]
    /// - [`SDL_utf8strlen`]
    /// - [`SDL_utf8strnlen`]
    pub fn SDL_wcsnlen(
        wstr: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated wide string to copy. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `maxlen`: The length (in wide characters) of the destination buffer.
    ///
    /// ## Return value
    /// Returns the length (in wide characters, excluding the null terminator) of
    ///   `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_wcslcat`]
    pub fn SDL_wcslcpy(
        dst: *mut crate::ffi::c_wchar_t,
        src: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `dst`: The destination buffer already containing the first
    ///   null-terminated wide string. Must not be NULL and must not
    ///   overlap with `src`.
    /// - `src`: The second null-terminated wide string. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `maxlen`: The length (in wide characters) of the destination buffer.
    ///
    /// ## Return value
    /// Returns the length (in wide characters, excluding the null terminator) of
    ///   the string in `dst` plus the length of `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_wcslcpy`]
    pub fn SDL_wcslcat(
        dst: *mut crate::ffi::c_wchar_t,
        src: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// Allocate a copy of a wide string.
    ///
    /// This allocates enough space for a null-terminated copy of `wstr`, using
    /// [`SDL_malloc`], and then makes a copy of the string into this space.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// [`SDL_free`] when no longer needed.
    ///
    /// ## Parameters
    /// - `wstr`: the string to copy.
    ///
    /// ## Return value
    /// Returns a pointer to the newly-allocated wide string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcsdup(wstr: *const crate::ffi::c_wchar_t) -> *mut crate::ffi::c_wchar_t;
}

unsafe extern "C" {
    /// Search a wide string for the first instance of a specific substring.
    ///
    /// The search ends once it finds the requested substring, or a null terminator
    /// byte to end the string.
    ///
    /// Note that this looks for strings of _wide characters_, not _codepoints_, so
    /// it's legal to search for malformed and incomplete UTF-16 sequences.
    ///
    /// ## Parameters
    /// - `haystack`: the wide string to search. Must not be NULL.
    /// - `needle`: the wide string to search for. Must not be NULL.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `needle` in the string, or NULL
    ///   if not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcsstr(
        haystack: *const crate::ffi::c_wchar_t,
        needle: *const crate::ffi::c_wchar_t,
    ) -> *mut crate::ffi::c_wchar_t;
}

unsafe extern "C" {
    /// Search a wide string, up to n wide chars, for the first instance of a
    /// specific substring.
    ///
    /// The search ends once it finds the requested substring, or a null terminator
    /// value to end the string, or `maxlen` wide character have been examined. It
    /// is possible to use this function on a wide string without a null
    /// terminator.
    ///
    /// Note that this looks for strings of _wide characters_, not _codepoints_, so
    /// it's legal to search for malformed and incomplete UTF-16 sequences.
    ///
    /// ## Parameters
    /// - `haystack`: the wide string to search. Must not be NULL.
    /// - `needle`: the wide string to search for. Must not be NULL.
    /// - `maxlen`: the maximum number of wide characters to search in
    ///   `haystack`.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `needle` in the string, or NULL
    ///   if not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcsnstr(
        haystack: *const crate::ffi::c_wchar_t,
        needle: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> *mut crate::ffi::c_wchar_t;
}

unsafe extern "C" {
    /// Compare two null-terminated wide strings.
    ///
    /// This only compares wchar_t values until it hits a null-terminating
    /// character; it does not care if the string is well-formed UTF-16 (or UTF-32,
    /// depending on your platform's wchar_t size), or uses valid Unicode values.
    ///
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcscmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of wchar_t to compare.
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcsncmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcscasecmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of wchar_t values to compare.
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_wcsncasecmp(
        str1: *const crate::ffi::c_wchar_t,
        str2: *const crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Parse a `long` from a wide string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long`, the result is clamped to
    /// the minimum and maximum representable `long` values.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated wide string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid wide character
    ///   (i.e. the next character after the parsed number) will be
    ///   written to this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    ///
    /// ## Return value
    /// Returns the parsed `long`, or 0 if no number could be parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strtol`]
    pub fn SDL_wcstol(
        str: *const crate::ffi::c_wchar_t,
        endp: *mut *mut crate::ffi::c_wchar_t,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}

unsafe extern "C" {
    /// This works exactly like strlen() but doesn't require access to a C runtime.
    ///
    /// Counts the bytes in `str`, excluding the null terminator.
    ///
    /// If you need the length of a UTF-8 string, consider using [`SDL_utf8strlen()`].
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    ///
    /// ## Return value
    /// Returns the length (in bytes, excluding the null terminator) of `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strnlen`]
    /// - [`SDL_utf8strlen`]
    /// - [`SDL_utf8strnlen`]
    pub fn SDL_strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// This works exactly like strnlen() but doesn't require access to a C
    /// runtime.
    ///
    /// Counts up to a maximum of `maxlen` bytes in `str`, excluding the null
    /// terminator.
    ///
    /// If you need the length of a UTF-8 string, consider using [`SDL_utf8strnlen()`].
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `maxlen`: The maximum amount of bytes to count.
    ///
    /// ## Return value
    /// Returns the length (in bytes, excluding the null terminator) of `src` but
    ///   never more than `maxlen`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strlen`]
    /// - [`SDL_utf8strlen`]
    /// - [`SDL_utf8strnlen`]
    pub fn SDL_strnlen(
        str: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated string to copy. Must not be NULL, and must
    ///   not overlap with `dst`.
    /// - `maxlen`: The length (in characters) of the destination buffer.
    ///
    /// ## Return value
    /// Returns the length (in characters, excluding the null terminator) of
    ///   `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strlcat`]
    /// - [`SDL_utf8strlcpy`]
    pub fn SDL_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `dst`: The destination buffer. Must not be NULL, and must not overlap
    ///   with `src`.
    /// - `src`: The null-terminated UTF-8 string to copy. Must not be NULL, and
    ///   must not overlap with `dst`.
    /// - `dst_bytes`: The length (in bytes) of the destination buffer. Must not
    ///   be 0.
    ///
    /// ## Return value
    /// Returns the number of bytes written, excluding the null terminator.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strlcpy`]
    pub fn SDL_utf8strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_bytes: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `dst`: The destination buffer already containing the first
    ///   null-terminated string. Must not be NULL and must not overlap
    ///   with `src`.
    /// - `src`: The second null-terminated string. Must not be NULL, and must
    ///   not overlap with `dst`.
    /// - `maxlen`: The length (in characters) of the destination buffer.
    ///
    /// ## Return value
    /// Returns the length (in characters, excluding the null terminator) of the
    ///   string in `dst` plus the length of `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strlcpy`]
    pub fn SDL_strlcat(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// Allocate a copy of a string.
    ///
    /// This allocates enough space for a null-terminated copy of `str`, using
    /// [`SDL_malloc`], and then makes a copy of the string into this space.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// [`SDL_free`] when no longer needed.
    ///
    /// ## Parameters
    /// - `str`: the string to copy.
    ///
    /// ## Return value
    /// Returns a pointer to the newly-allocated string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Allocate a copy of a string, up to n characters.
    ///
    /// This allocates enough space for a null-terminated copy of `str`, up to
    /// `maxlen` bytes, using [`SDL_malloc`], and then makes a copy of the string into
    /// this space.
    ///
    /// If the string is longer than `maxlen` bytes, the returned string will be
    /// `maxlen` bytes long, plus a null-terminator character that isn't included
    /// in the count.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// [`SDL_free`] when no longer needed.
    ///
    /// ## Parameters
    /// - `str`: the string to copy.
    /// - `maxlen`: the maximum length of the copied string, not counting the
    ///   null-terminator character.
    ///
    /// ## Return value
    /// Returns a pointer to the newly-allocated string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strndup(
        str: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Reverse a string's contents.
    ///
    /// This reverses a null-terminated string in-place. Only the content of the
    /// string is reversed; the null-terminator character remains at the end of the
    /// reversed string.
    ///
    /// **WARNING**: This function reverses the _bytes_ of the string, not the
    /// codepoints. If `str` is a UTF-8 string with Unicode codepoints > 127, this
    /// will ruin the string data. You should only use this function on strings
    /// that are completely comprised of low ASCII characters.
    ///
    /// ## Parameters
    /// - `str`: the string to reverse.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strrev(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert a string to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to uppercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'a' through 'z' to their
    /// uppercase equivalents in-place, returning the original `str` pointer.
    ///
    /// ## Parameters
    /// - `str`: the string to convert in-place. Can not be NULL.
    ///
    /// ## Return value
    /// Returns the `str` pointer passed into this function.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strlwr`]
    pub fn SDL_strupr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert a string to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'A' through 'Z' to their
    /// lowercase equivalents in-place, returning the original `str` pointer.
    ///
    /// ## Parameters
    /// - `str`: the string to convert in-place. Can not be NULL.
    ///
    /// ## Return value
    /// Returns the `str` pointer passed into this function.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_strupr`]
    pub fn SDL_strlwr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Search a string for the first instance of a specific byte.
    ///
    /// The search ends once it finds the requested byte value, or a null
    /// terminator byte to end the string.
    ///
    /// Note that this looks for _bytes_, not _characters_, so you cannot match
    /// against a Unicode codepoint > 255, regardless of character encoding.
    ///
    /// ## Parameters
    /// - `str`: the string to search. Must not be NULL.
    /// - `c`: the byte value to search for.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `c` in the string, or NULL if
    ///   not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strchr(
        str: *const ::core::ffi::c_char,
        c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Search a string for the last instance of a specific byte.
    ///
    /// The search must go until it finds a null terminator byte to end the string.
    ///
    /// Note that this looks for _bytes_, not _characters_, so you cannot match
    /// against a Unicode codepoint > 255, regardless of character encoding.
    ///
    /// ## Parameters
    /// - `str`: the string to search. Must not be NULL.
    /// - `c`: the byte value to search for.
    ///
    /// ## Return value
    /// Returns a pointer to the last instance of `c` in the string, or NULL if
    ///   not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strrchr(
        str: *const ::core::ffi::c_char,
        c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Search a string for the first instance of a specific substring.
    ///
    /// The search ends once it finds the requested substring, or a null terminator
    /// byte to end the string.
    ///
    /// Note that this looks for strings of _bytes_, not _characters_, so it's
    /// legal to search for malformed and incomplete UTF-8 sequences.
    ///
    /// ## Parameters
    /// - `haystack`: the string to search. Must not be NULL.
    /// - `needle`: the string to search for. Must not be NULL.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `needle` in the string, or NULL
    ///   if not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strstr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Search a string, up to n bytes, for the first instance of a specific
    /// substring.
    ///
    /// The search ends once it finds the requested substring, or a null terminator
    /// byte to end the string, or `maxlen` bytes have been examined. It is
    /// possible to use this function on a string without a null terminator.
    ///
    /// Note that this looks for strings of _bytes_, not _characters_, so it's
    /// legal to search for malformed and incomplete UTF-8 sequences.
    ///
    /// ## Parameters
    /// - `haystack`: the string to search. Must not be NULL.
    /// - `needle`: the string to search for. Must not be NULL.
    /// - `maxlen`: the maximum number of bytes to search in `haystack`.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `needle` in the string, or NULL
    ///   if not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strnstr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Search a UTF-8 string for the first instance of a specific substring,
    /// case-insensitively.
    ///
    /// This will work with Unicode strings, using a technique called
    /// "case-folding" to handle the vast majority of case-sensitive human
    /// languages regardless of system locale. It can deal with expanding values: a
    /// German Eszett character can compare against two ASCII 's' chars and be
    /// considered a match, for example. A notable exception: it does not handle
    /// the Turkish 'i' character; human language is complicated!
    ///
    /// Since this handles Unicode, it expects the strings to be well-formed UTF-8
    /// and not a null-terminated string of arbitrary bytes. Bytes that are not
    /// valid UTF-8 are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), which is to say two strings of random bits may turn out to
    /// match if they convert to the same amount of replacement characters.
    ///
    /// ## Parameters
    /// - `haystack`: the string to search. Must not be NULL.
    /// - `needle`: the string to search for. Must not be NULL.
    ///
    /// ## Return value
    /// Returns a pointer to the first instance of `needle` in the string, or NULL
    ///   if not found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strcasestr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// This works exactly like strtok_r() but doesn't require access to a C
    /// runtime.
    ///
    /// Break a string up into a series of tokens.
    ///
    /// To start tokenizing a new string, `str` should be the non-NULL address of
    /// the string to start tokenizing. Future calls to get the next token from the
    /// same string should specify a NULL.
    ///
    /// Note that this function will overwrite pieces of `str` with null chars to
    /// split it into tokens. This function cannot be used with const/read-only
    /// strings!
    ///
    /// `saveptr` just needs to point to a `char *` that can be overwritten; SDL
    /// will use this to save tokenizing state between calls. It is initialized if
    /// `str` is non-NULL, and used to resume tokenizing when `str` is NULL.
    ///
    /// ## Parameters
    /// - `str`: the string to tokenize, or NULL to continue tokenizing.
    /// - `delim`: the delimiter string that separates tokens.
    /// - `saveptr`: pointer to a char *, used for ongoing state.
    ///
    /// ## Return value
    /// Returns A pointer to the next token, or NULL if no tokens remain.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strtok_r(
        str: *mut ::core::ffi::c_char,
        delim: *const ::core::ffi::c_char,
        saveptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Count the number of codepoints in a UTF-8 string.
    ///
    /// Counts the _codepoints_, not _bytes_, in `str`, excluding the null
    /// terminator.
    ///
    /// If you need to count the bytes in a string instead, consider using
    /// [`SDL_strlen()`].
    ///
    /// Since this handles Unicode, it expects the strings to be well-formed UTF-8
    /// and not a null-terminated string of arbitrary bytes. Bytes that are not
    /// valid UTF-8 are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), so a malformed or incomplete UTF-8 sequence might increase the
    /// count by several replacement characters.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated UTF-8 string to read. Must not be NULL.
    ///
    /// ## Return value
    /// Returns The length (in codepoints, excluding the null terminator) of
    ///   `src`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_utf8strnlen`]
    /// - [`SDL_strlen`]
    pub fn SDL_utf8strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// Count the number of codepoints in a UTF-8 string, up to n bytes.
    ///
    /// Counts the _codepoints_, not _bytes_, in `str`, excluding the null
    /// terminator.
    ///
    /// If you need to count the bytes in a string instead, consider using
    /// [`SDL_strnlen()`].
    ///
    /// The counting stops at `bytes` bytes (not codepoints!). This seems
    /// counterintuitive, but makes it easy to express the total size of the
    /// string's buffer.
    ///
    /// Since this handles Unicode, it expects the strings to be well-formed UTF-8
    /// and not a null-terminated string of arbitrary bytes. Bytes that are not
    /// valid UTF-8 are treated as Unicode character U+FFFD (REPLACEMENT
    /// CHARACTER), so a malformed or incomplete UTF-8 sequence might increase the
    /// count by several replacement characters.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated UTF-8 string to read. Must not be NULL.
    /// - `bytes`: The maximum amount of bytes to count.
    ///
    /// ## Return value
    /// Returns The length (in codepoints, excluding the null terminator) of `src`
    ///   but never more than `maxlen`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_utf8strlen`]
    /// - [`SDL_strnlen`]
    pub fn SDL_utf8strnlen(
        str: *const ::core::ffi::c_char,
        bytes: ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

unsafe extern "C" {
    /// Convert an integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget possible negative
    /// signs, null terminator bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_uitoa`]
    /// - [`SDL_ltoa`]
    /// - [`SDL_lltoa`]
    pub fn SDL_itoa(
        value: ::core::ffi::c_int,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert an unsigned integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget null terminator
    /// bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the unsigned integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_itoa`]
    /// - [`SDL_ultoa`]
    /// - [`SDL_ulltoa`]
    pub fn SDL_uitoa(
        value: ::core::ffi::c_uint,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert a long integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget possible negative
    /// signs, null terminator bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the long integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ultoa`]
    /// - [`SDL_itoa`]
    /// - [`SDL_lltoa`]
    pub fn SDL_ltoa(
        value: ::core::ffi::c_long,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert an unsigned long integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget null terminator
    /// bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the unsigned long integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ltoa`]
    /// - [`SDL_uitoa`]
    /// - [`SDL_ulltoa`]
    pub fn SDL_ultoa(
        value: ::core::ffi::c_ulong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert a long long integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget possible negative
    /// signs, null terminator bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the long long integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ulltoa`]
    /// - [`SDL_itoa`]
    /// - [`SDL_ltoa`]
    pub fn SDL_lltoa(
        value: ::core::ffi::c_longlong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Convert an unsigned long long integer into a string.
    ///
    /// This requires a radix to specified for string format. Specifying 10
    /// produces a decimal number, 16 hexadecimal, etc. Must be in the range of 2
    /// to 36.
    ///
    /// Note that this function will overflow a buffer if `str` is not large enough
    /// to hold the output! It may be safer to use [`SDL_snprintf`] to clamp output, or
    /// [`SDL_asprintf`] to allocate a buffer. Otherwise, it doesn't hurt to allocate
    /// much more space than you expect to use (and don't forget null terminator
    /// bytes, etc).
    ///
    /// ## Parameters
    /// - `value`: the unsigned long long integer to convert.
    /// - `str`: the buffer to write the string into.
    /// - `radix`: the radix to use for string generation.
    ///
    /// ## Return value
    /// Returns `str`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_lltoa`]
    /// - [`SDL_uitoa`]
    /// - [`SDL_ultoa`]
    pub fn SDL_ulltoa(
        value: ::core::ffi::c_ulonglong,
        str: *mut ::core::ffi::c_char,
        radix: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Parse an `int` from a string.
    ///
    /// The result of calling `SDL_atoi(str)` is equivalent to
    /// `(int)SDL_strtol(str, NULL, 10)`.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    ///
    /// ## Return value
    /// Returns the parsed `int`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atof`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoull`]
    /// - [`SDL_strtod`]
    /// - [`SDL_itoa`]
    pub fn SDL_atoi(str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Parse a `double` from a string.
    ///
    /// The result of calling `SDL_atof(str)` is equivalent to `SDL_strtod(str,
    /// NULL)`.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    ///
    /// ## Return value
    /// Returns the parsed `double`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoull`]
    /// - [`SDL_strtod`]
    pub fn SDL_atof(str: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Parse a `long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long`, the result is clamped to
    /// the minimum and maximum representable `long` values.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    ///
    /// ## Return value
    /// Returns the parsed `long`, or 0 if no number could be parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_atof`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoull`]
    /// - [`SDL_strtod`]
    /// - [`SDL_ltoa`]
    /// - [`SDL_wcstol`]
    pub fn SDL_strtol(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}

unsafe extern "C" {
    /// Parse an `unsigned long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside an `unsigned long`, the result is
    /// clamped to the maximum representable `unsigned long` value.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    ///
    /// ## Return value
    /// Returns the parsed `unsigned long`, or 0 if no number could be parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_atof`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoull`]
    /// - [`SDL_strtod`]
    /// - [`SDL_ultoa`]
    pub fn SDL_strtoul(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
}

unsafe extern "C" {
    /// Parse a `long long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside a `long long`, the result is
    /// clamped to the minimum and maximum representable `long long` values.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    ///
    /// ## Return value
    /// Returns the parsed `long long`, or 0 if no number could be parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_atof`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtoull`]
    /// - [`SDL_strtod`]
    /// - [`SDL_lltoa`]
    pub fn SDL_strtoll(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
}

unsafe extern "C" {
    /// Parse an `unsigned long long` from a string.
    ///
    /// If `str` starts with whitespace, then those whitespace characters are
    /// skipped before attempting to parse the number.
    ///
    /// If the parsed number does not fit inside an `unsigned long long`, the
    /// result is clamped to the maximum representable `unsigned long long` value.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to read. Must not be NULL.
    /// - `endp`: If not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    /// - `base`: The base of the integer to read. Supported values are 0 and 2
    ///   to 36 inclusive. If 0, the base will be inferred from the
    ///   number's prefix (0x for hexadecimal, 0 for octal, decimal
    ///   otherwise).
    ///
    /// ## Return value
    /// Returns the parsed `unsigned long long`, or 0 if no number could be
    ///   parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_atof`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtod`]
    /// - [`SDL_ulltoa`]
    pub fn SDL_strtoull(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}

unsafe extern "C" {
    /// Parse a `double` from a string.
    ///
    /// This function makes fewer guarantees than the C runtime `strtod`:
    ///
    /// - Only decimal notation is guaranteed to be supported. The handling of
    ///   scientific and hexadecimal notation is unspecified.
    /// - Whether or not INF and NAN can be parsed is unspecified.
    /// - The precision of the result is unspecified.
    ///
    /// ## Parameters
    /// - `str`: the null-terminated string to read. Must not be NULL.
    /// - `endp`: if not NULL, the address of the first invalid character (i.e.
    ///   the next character after the parsed number) will be written to
    ///   this pointer.
    ///
    /// ## Return value
    /// Returns the parsed `double`, or 0 if no number could be parsed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atoi`]
    /// - [`SDL_atof`]
    /// - [`SDL_strtol`]
    /// - [`SDL_strtoll`]
    /// - [`SDL_strtoul`]
    /// - [`SDL_strtoull`]
    pub fn SDL_strtod(
        str: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compare two null-terminated UTF-8 strings.
    ///
    /// Due to the nature of UTF-8 encoding, this will work with Unicode strings,
    /// since effectively this function just compares bytes until it hits a
    /// null-terminating character. Also due to the nature of UTF-8, this can be
    /// used with [`SDL_qsort()`] to put strings in (roughly) alphabetical order.
    ///
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strcmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of _bytes_ to compare.
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strncmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strcasecmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `str1`: the first string to compare. NULL is not permitted!
    /// - `str2`: the second string to compare. NULL is not permitted!
    /// - `maxlen`: the maximum number of bytes to compare.
    ///
    /// ## Return value
    /// Returns less than zero if str1 is "less than" str2, greater than zero if
    ///   str1 is "greater than" str2, and zero if the strings match
    ///   exactly.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strncasecmp(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Searches a string for the first occurrence of any character contained in a
    /// breakset, and returns a pointer from the string to that character.
    ///
    /// ## Parameters
    /// - `str`: The null-terminated string to be searched. Must not be NULL, and
    ///   must not overlap with `breakset`.
    /// - `breakset`: A null-terminated string containing the list of characters
    ///   to look for. Must not be NULL, and must not overlap with
    ///   `str`.
    ///
    /// ## Return value
    /// Returns A pointer to the location, in str, of the first occurrence of a
    ///   character present in the breakset, or NULL if none is found.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_strpbrk(
        str: *const ::core::ffi::c_char,
        breakset: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

/// The Unicode REPLACEMENT CHARACTER codepoint.
///
/// [`SDL_StepUTF8()`] and [`SDL_StepBackUTF8()`] report this codepoint when they
/// encounter a UTF-8 string with encoding errors.
///
/// This tends to render as something like a question mark in most places.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_StepBackUTF8`]
/// - [`SDL_StepUTF8`]
pub const SDL_INVALID_UNICODE_CODEPOINT: ::core::primitive::i32 = 65533;

unsafe extern "C" {
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
    /// ## Parameters
    /// - `pstr`: a pointer to a UTF-8 string pointer to be read and adjusted.
    /// - `pslen`: a pointer to the number of bytes in the string, to be read and
    ///   adjusted. NULL is allowed.
    ///
    /// ## Return value
    /// Returns the first Unicode codepoint in the string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_StepUTF8(
        pstr: *mut *const ::core::ffi::c_char,
        pslen: *mut ::core::primitive::usize,
    ) -> Uint32;
}

unsafe extern "C" {
    /// Decode a UTF-8 string in reverse, one Unicode codepoint at a time.
    ///
    /// This will go to the start of the previous Unicode codepoint in the string,
    /// move `*pstr` to that location and return that codepoint.
    ///
    /// If `*pstr` is already at the start of the string), it will not advance
    /// `*pstr` at all.
    ///
    /// Generally this function is called in a loop until it returns zero,
    /// adjusting its parameter each iteration.
    ///
    /// If an invalid UTF-8 sequence is encountered, this function returns
    /// [`SDL_INVALID_UNICODE_CODEPOINT`].
    ///
    /// Several things can generate invalid UTF-8 sequences, including overlong
    /// encodings, the use of UTF-16 surrogate values, and truncated data. Please
    /// refer to
    /// [RFC3629](https://www.ietf.org/rfc/rfc3629.txt)
    /// for details.
    ///
    /// ## Parameters
    /// - `start`: a pointer to the beginning of the UTF-8 string.
    /// - `pstr`: a pointer to a UTF-8 string pointer to be read and adjusted.
    ///
    /// ## Return value
    /// Returns the previous Unicode codepoint in the string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_StepBackUTF8(
        start: *const ::core::ffi::c_char,
        pstr: *mut *const ::core::ffi::c_char,
    ) -> Uint32;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `codepoint`: a Unicode codepoint to convert to UTF-8.
    /// - `dst`: the location to write the encoded UTF-8. Must point to at least
    ///   4 bytes!
    ///
    /// ## Return value
    /// Returns the first byte past the newly-written UTF-8 sequence.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_UCS4ToUTF8(
        codepoint: Uint32,
        dst: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}

unsafe extern "C" {
    /// This works exactly like sscanf() but doesn't require access to a C runtime.
    ///
    /// Scan a string, matching a format string, converting each '%' item and
    /// storing it to pointers provided through variable arguments.
    ///
    /// ## Parameters
    /// - `text`: the string to scan. Must not be NULL.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `...`: a list of pointers to values to be filled in with scanned items.
    ///
    /// ## Return value
    /// Returns the number of items that matched the format string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_sscanf(
        text: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like vsscanf() but doesn't require access to a C
    /// runtime.
    ///
    /// Functions identically to [`SDL_sscanf()`], except it takes a `va_list` instead
    /// of using `...` variable arguments.
    ///
    /// ## Parameters
    /// - `text`: the string to scan. Must not be NULL.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `ap`: a `va_list` of pointers to values to be filled in with scanned
    ///   items.
    ///
    /// ## Return value
    /// Returns the number of items that matched the format string.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_vsscanf(
        text: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like snprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Format a string of up to `maxlen`-1 bytes, converting each '%' item with
    /// values provided through variable arguments.
    ///
    /// While some C runtimes differ on how to deal with too-large strings, this
    /// function null-terminates the output, by treating the null-terminator as
    /// part of the `maxlen` count. Note that if `maxlen` is zero, however, no
    /// bytes will be written at all.
    ///
    /// This function returns the number of _bytes_ (not _characters_) that should
    /// be written, excluding the null-terminator character. If this returns a
    /// number >= `maxlen`, it means the output string was truncated. A negative
    /// return value means an error occurred.
    ///
    /// Referencing the output string's pointer with a format item is undefined
    /// behavior.
    ///
    /// ## Parameters
    /// - `text`: the buffer to write the string into. Must not be NULL.
    /// - `maxlen`: the maximum bytes to write, including the null-terminator.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `...`: a list of values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of bytes that should be written, not counting the
    ///   null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_snprintf(
        text: *mut ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like swprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Format a wide string of up to `maxlen`-1 wchar_t values, converting each
    /// '%' item with values provided through variable arguments.
    ///
    /// While some C runtimes differ on how to deal with too-large strings, this
    /// function null-terminates the output, by treating the null-terminator as
    /// part of the `maxlen` count. Note that if `maxlen` is zero, however, no wide
    /// characters will be written at all.
    ///
    /// This function returns the number of _wide characters_ (not _codepoints_)
    /// that should be written, excluding the null-terminator character. If this
    /// returns a number >= `maxlen`, it means the output string was truncated. A
    /// negative return value means an error occurred.
    ///
    /// Referencing the output string's pointer with a format item is undefined
    /// behavior.
    ///
    /// ## Parameters
    /// - `text`: the buffer to write the wide string into. Must not be NULL.
    /// - `maxlen`: the maximum wchar_t values to write, including the
    ///   null-terminator.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `...`: a list of values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of wide characters that should be written, not counting
    ///   the null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_swprintf(
        text: *mut crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
        fmt: *const crate::ffi::c_wchar_t,
        ...
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like vsnprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Functions identically to [`SDL_snprintf()`], except it takes a `va_list`
    /// instead of using `...` variable arguments.
    ///
    /// ## Parameters
    /// - `text`: the buffer to write the string into. Must not be NULL.
    /// - `maxlen`: the maximum bytes to write, including the null-terminator.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `ap`: a `va_list` values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of bytes that should be written, not counting the
    ///   null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_vsnprintf(
        text: *mut ::core::ffi::c_char,
        maxlen: ::core::primitive::usize,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like vswprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Functions identically to [`SDL_swprintf()`], except it takes a `va_list`
    /// instead of using `...` variable arguments.
    ///
    /// ## Parameters
    /// - `text`: the buffer to write the string into. Must not be NULL.
    /// - `maxlen`: the maximum wide characters to write, including the
    ///   null-terminator.
    /// - `fmt`: a printf-style format wide string. Must not be NULL.
    /// - `ap`: a `va_list` values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of wide characters that should be written, not counting
    ///   the null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_vswprintf(
        text: *mut crate::ffi::c_wchar_t,
        maxlen: ::core::primitive::usize,
        fmt: *const crate::ffi::c_wchar_t,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like asprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Functions identically to [`SDL_snprintf()`], except it allocates a buffer large
    /// enough to hold the output string on behalf of the caller.
    ///
    /// On success, this function returns the number of bytes (not characters)
    /// comprising the output string, not counting the null-terminator character,
    /// and sets `*strp` to the newly-allocated string.
    ///
    /// On error, this function returns a negative number, and the value of `*strp`
    /// is undefined.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// [`SDL_free`] when no longer needed.
    ///
    /// ## Parameters
    /// - `strp`: on output, is set to the new string. Must not be NULL.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `...`: a list of values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of bytes in the newly-allocated string, not counting
    ///   the null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_asprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This works exactly like vasprintf() but doesn't require access to a C
    /// runtime.
    ///
    /// Functions identically to [`SDL_asprintf()`], except it takes a `va_list`
    /// instead of using `...` variable arguments.
    ///
    /// ## Parameters
    /// - `strp`: on output, is set to the new string. Must not be NULL.
    /// - `fmt`: a printf-style format string. Must not be NULL.
    /// - `ap`: a `va_list` values to be used with the format string.
    ///
    /// ## Return value
    /// Returns the number of bytes in the newly-allocated string, not counting
    ///   the null-terminator char, or a negative value on error.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_vasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ap: crate::ffi::VaList,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Seeds the pseudo-random number generator.
    ///
    /// Reusing the seed number will cause [`SDL_rand()`] to repeat the same stream of
    /// 'random' numbers.
    ///
    /// ## Parameters
    /// - `seed`: the value to use as a random number seed, or 0 to use
    ///   [`SDL_GetPerformanceCounter()`].
    ///
    /// ## Thread safety
    /// This should be called on the same thread that calls
    ///   [`SDL_rand()`]
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_rand`]
    /// - [`SDL_rand_bits`]
    /// - [`SDL_randf`]
    pub fn SDL_srand(seed: Uint64);
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `n`: the number of possible outcomes. n must be positive.
    ///
    /// ## Return value
    /// Returns a random value in the range of \[0 .. n-1\].
    ///
    /// ## Thread safety
    /// All calls should be made from a single thread
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_srand`]
    /// - [`SDL_randf`]
    pub fn SDL_rand(n: Sint32) -> Sint32;
}

unsafe extern "C" {
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
    /// ## Return value
    /// Returns a random value in the range of [0.0, 1.0).
    ///
    /// ## Thread safety
    /// All calls should be made from a single thread
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_srand`]
    /// - [`SDL_rand`]
    pub fn SDL_randf() -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Return value
    /// Returns a random value in the range of \[0-SDL_MAX_UINT32\].
    ///
    /// ## Thread safety
    /// All calls should be made from a single thread
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_rand`]
    /// - [`SDL_randf`]
    /// - [`SDL_srand`]
    pub fn SDL_rand_bits() -> Uint32;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    /// - `n`: the number of possible outcomes. n must be positive.
    ///
    /// ## Return value
    /// Returns a random value in the range of \[0 .. n-1\].
    ///
    /// ## Thread safety
    /// This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_rand`]
    /// - [`SDL_rand_bits_r`]
    /// - [`SDL_randf_r`]
    pub fn SDL_rand_r(state: *mut Uint64, n: Sint32) -> Sint32;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a random value in the range of [0.0, 1.0).
    ///
    /// ## Thread safety
    /// This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_rand_bits_r`]
    /// - [`SDL_rand_r`]
    /// - [`SDL_randf`]
    pub fn SDL_randf_r(state: *mut Uint64) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `state`: a pointer to the current random number state, this may not be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a random value in the range of \[0-SDL_MAX_UINT32\].
    ///
    /// ## Thread safety
    /// This function is thread-safe, as long as the state pointer
    ///   isn't shared between threads.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_rand_r`]
    /// - [`SDL_randf_r`]
    pub fn SDL_rand_bits_r(state: *mut Uint64) -> Uint32;
}

/// The value of Pi, as a double-precision floating point literal.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_PI_F`]
pub const SDL_PI_D: ::core::ffi::c_double = 3.141592653589793_f64;

/// The value of Pi, as a single-precision floating point literal.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_PI_D`]
pub const SDL_PI_F: ::core::ffi::c_float = 3.1415927_f32;

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc cosine of `x`, in radians.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_acosf`]
    /// - [`SDL_asin`]
    /// - [`SDL_cos`]
    pub safe fn SDL_acos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc cosine of `x`, in radians.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_acos`]
    /// - [`SDL_asinf`]
    /// - [`SDL_cosf`]
    pub safe fn SDL_acosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc sine of `x`, in radians.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_asinf`]
    /// - [`SDL_acos`]
    /// - [`SDL_sin`]
    pub safe fn SDL_asin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc sine of `x`, in radians.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_asin`]
    /// - [`SDL_acosf`]
    /// - [`SDL_sinf`]
    pub safe fn SDL_asinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atanf`]
    /// - [`SDL_atan2`]
    /// - [`SDL_tan`]
    pub safe fn SDL_atan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atan`]
    /// - [`SDL_atan2f`]
    /// - [`SDL_tanf`]
    pub safe fn SDL_atanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
    /// Compute the arc tangent of `y / x`, using the signs of x and y to adjust
    /// the result's quadrant.
    ///
    /// The definition of `z = atan2(x, y)` is `y = x tan(z)`, where the quadrant
    /// of z is determined based on the signs of x and y.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-Pi <= y <= Pi`
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
    /// ## Parameters
    /// - `y`: floating point value of the numerator (y coordinate).
    /// - `x`: floating point value of the denominator (x coordinate).
    ///
    /// ## Return value
    /// Returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///   `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atan2f`]
    /// - [`SDL_atan`]
    /// - [`SDL_tan`]
    pub safe fn SDL_atan2(
        y: ::core::ffi::c_double,
        x: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compute the arc tangent of `y / x`, using the signs of x and y to adjust
    /// the result's quadrant.
    ///
    /// The definition of `z = atan2(x, y)` is `y = x tan(z)`, where the quadrant
    /// of z is determined based on the signs of x and y.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-Pi <= y <= Pi`
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
    /// ## Parameters
    /// - `y`: floating point value of the numerator (y coordinate).
    /// - `x`: floating point value of the denominator (x coordinate).
    ///
    /// ## Return value
    /// Returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///   `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_atan2`]
    /// - [`SDL_atan`]
    /// - [`SDL_tan`]
    pub safe fn SDL_atan2f(
        y: ::core::ffi::c_float,
        x: ::core::ffi::c_float,
    ) -> ::core::ffi::c_float;
}

unsafe extern "C" {
    /// Compute the ceiling of `x`.
    ///
    /// The ceiling of `x` is the smallest integer `y` such that `y >= x`, i.e `x`
    /// rounded up to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_ceilf`] for single-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the ceiling of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ceilf`]
    /// - [`SDL_floor`]
    /// - [`SDL_trunc`]
    /// - [`SDL_round`]
    /// - [`SDL_lround`]
    pub safe fn SDL_ceil(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compute the ceiling of `x`.
    ///
    /// The ceiling of `x` is the smallest integer `y` such that `y >= x`, i.e `x`
    /// rounded up to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_ceil`] for double-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the ceiling of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ceil`]
    /// - [`SDL_floorf`]
    /// - [`SDL_truncf`]
    /// - [`SDL_roundf`]
    /// - [`SDL_lroundf`]
    pub safe fn SDL_ceilf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

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
/// ## Parameters
/// - `x`: floating point value to use as the magnitude.
/// - `y`: floating point value to use as the sign.
///
/// ## Return value
/// Returns the floating point value with the sign of y and the magnitude of
///   x.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_copysignf`]
/// - [`SDL_fabs`]
#[inline(always)]
pub const fn SDL_copysign(
    x: ::core::ffi::c_double,
    y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return x.copysign(y);
}

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
/// ## Parameters
/// - `x`: floating point value to use as the magnitude.
/// - `y`: floating point value to use as the sign.
///
/// ## Return value
/// Returns the floating point value with the sign of y and the magnitude of
///   x.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_copysign`]
/// - [`SDL_fabsf`]
#[inline(always)]
pub const fn SDL_copysignf(
    x: ::core::ffi::c_float,
    y: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return x.copysign(y);
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns cosine of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_cosf`]
    /// - [`SDL_acos`]
    /// - [`SDL_sin`]
    pub safe fn SDL_cos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns cosine of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_cos`]
    /// - [`SDL_acosf`]
    /// - [`SDL_sinf`]
    pub safe fn SDL_cosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns value of `e^x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_expf`]
    /// - [`SDL_log`]
    pub safe fn SDL_exp(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns value of `e^x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_exp`]
    /// - [`SDL_logf`]
    pub safe fn SDL_expf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

/// Compute the absolute value of `x`
///
/// Domain: `-INF <= x <= INF`
///
/// Range: `0 <= y <= INF`
///
/// This function operates on double-precision floating point values, use
/// [`SDL_fabsf`] for single-precision floats.
///
/// ## Parameters
/// - `x`: floating point value to use as the magnitude.
///
/// ## Return value
/// Returns the absolute value of `x`.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_fabsf`]
#[inline(always)]
pub const fn SDL_fabs(x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return x.abs();
}

/// Compute the absolute value of `x`
///
/// Domain: `-INF <= x <= INF`
///
/// Range: `0 <= y <= INF`
///
/// This function operates on single-precision floating point values, use
/// [`SDL_fabs`] for double-precision floats.
///
/// ## Parameters
/// - `x`: floating point value to use as the magnitude.
///
/// ## Return value
/// Returns the absolute value of `x`.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_fabs`]
#[inline(always)]
pub const fn SDL_fabsf(x: ::core::ffi::c_float) -> ::core::ffi::c_float {
    return x.abs();
}

unsafe extern "C" {
    /// Compute the floor of `x`.
    ///
    /// The floor of `x` is the largest integer `y` such that `y <= x`, i.e `x`
    /// rounded down to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_floorf`] for single-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the floor of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_floorf`]
    /// - [`SDL_ceil`]
    /// - [`SDL_trunc`]
    /// - [`SDL_round`]
    /// - [`SDL_lround`]
    pub safe fn SDL_floor(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compute the floor of `x`.
    ///
    /// The floor of `x` is the largest integer `y` such that `y <= x`, i.e `x`
    /// rounded down to the nearest integer.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_floor`] for double-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the floor of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_floor`]
    /// - [`SDL_ceilf`]
    /// - [`SDL_truncf`]
    /// - [`SDL_roundf`]
    /// - [`SDL_lroundf`]
    pub safe fn SDL_floorf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns `x` truncated to an integer.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_truncf`]
    /// - [`SDL_fmod`]
    /// - [`SDL_ceil`]
    /// - [`SDL_floor`]
    /// - [`SDL_round`]
    /// - [`SDL_lround`]
    pub safe fn SDL_trunc(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// [`SDL_trunc`] for double-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns `x` truncated to an integer.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_trunc`]
    /// - [`SDL_fmodf`]
    /// - [`SDL_ceilf`]
    /// - [`SDL_floorf`]
    /// - [`SDL_roundf`]
    /// - [`SDL_lroundf`]
    pub safe fn SDL_truncf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: the numerator.
    /// - `y`: the denominator. Must not be 0.
    ///
    /// ## Return value
    /// Returns the remainder of `x / y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_fmodf`]
    /// - [`SDL_modf`]
    /// - [`SDL_trunc`]
    /// - [`SDL_ceil`]
    /// - [`SDL_floor`]
    /// - [`SDL_round`]
    /// - [`SDL_lround`]
    pub safe fn SDL_fmod(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Return the floating-point remainder of `x / y`
    ///
    /// Divides `x` by `y`, and returns the remainder.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`, `y != 0`
    ///
    /// Range: `-y <= z <= y`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_fmod`] for double-precision floats.
    ///
    /// ## Parameters
    /// - `x`: the numerator.
    /// - `y`: the denominator. Must not be 0.
    ///
    /// ## Return value
    /// Returns the remainder of `x / y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_fmod`]
    /// - [`SDL_truncf`]
    /// - [`SDL_modff`]
    /// - [`SDL_ceilf`]
    /// - [`SDL_floorf`]
    /// - [`SDL_roundf`]
    /// - [`SDL_lroundf`]
    pub safe fn SDL_fmodf(x: ::core::ffi::c_float, y: ::core::ffi::c_float)
    -> ::core::ffi::c_float;
}

/// Return whether the value is infinity.
///
/// ## Parameters
/// - `x`: double-precision floating point value.
///
/// ## Return value
/// Returns non-zero if the value is infinity, 0 otherwise.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_isinff`]
#[inline(always)]
pub const fn SDL_isinf(x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    return x.is_infinite() as _;
}

/// Return whether the value is infinity.
///
/// ## Parameters
/// - `x`: floating point value.
///
/// ## Return value
/// Returns non-zero if the value is infinity, 0 otherwise.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_isinf`]
#[inline(always)]
pub const fn SDL_isinff(x: ::core::ffi::c_float) -> ::core::ffi::c_int {
    return x.is_infinite() as _;
}

/// Return whether the value is NaN.
///
/// ## Parameters
/// - `x`: double-precision floating point value.
///
/// ## Return value
/// Returns non-zero if the value is NaN, 0 otherwise.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_isnanf`]
#[inline(always)]
pub const fn SDL_isnan(x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    return x.is_nan() as _;
}

/// Return whether the value is NaN.
///
/// ## Parameters
/// - `x`: floating point value.
///
/// ## Return value
/// Returns non-zero if the value is NaN, 0 otherwise.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_isnan`]
#[inline(always)]
pub const fn SDL_isnanf(x: ::core::ffi::c_float) -> ::core::ffi::c_int {
    return x.is_nan() as _;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than 0.
    ///
    /// ## Return value
    /// Returns the natural logarithm of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_logf`]
    /// - [`SDL_log10`]
    /// - [`SDL_exp`]
    pub safe fn SDL_log(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than 0.
    ///
    /// ## Return value
    /// Returns the natural logarithm of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_log`]
    /// - [`SDL_expf`]
    pub safe fn SDL_logf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than 0.
    ///
    /// ## Return value
    /// Returns the logarithm of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_log10f`]
    /// - [`SDL_log`]
    /// - [`SDL_pow`]
    pub safe fn SDL_log10(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than 0.
    ///
    /// ## Return value
    /// Returns the logarithm of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_log10`]
    /// - [`SDL_logf`]
    /// - [`SDL_powf`]
    pub safe fn SDL_log10f(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on double-precision floating point values, use
    /// [`SDL_modff`] for single-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    /// - `y`: output pointer to store the integer part of `x`.
    ///
    /// ## Return value
    /// Returns the fractional part of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_modff`]
    /// - [`SDL_trunc`]
    /// - [`SDL_fmod`]
    pub fn SDL_modf(
        x: ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_modf`] for double-precision floats.
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    /// - `y`: output pointer to store the integer part of `x`.
    ///
    /// ## Return value
    /// Returns the fractional part of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_modf`]
    /// - [`SDL_truncf`]
    /// - [`SDL_fmodf`]
    pub fn SDL_modff(x: ::core::ffi::c_float, y: *mut ::core::ffi::c_float)
    -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: the base.
    /// - `y`: the exponent.
    ///
    /// ## Return value
    /// Returns `x` raised to the power `y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_powf`]
    /// - [`SDL_exp`]
    /// - [`SDL_log`]
    pub safe fn SDL_pow(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// [`SDL_pow`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// ## Parameters
    /// - `x`: the base.
    /// - `y`: the exponent.
    ///
    /// ## Return value
    /// Returns `x` raised to the power `y`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_pow`]
    /// - [`SDL_expf`]
    /// - [`SDL_logf`]
    pub safe fn SDL_powf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the nearest integer to `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_roundf`]
    /// - [`SDL_lround`]
    /// - [`SDL_floor`]
    /// - [`SDL_ceil`]
    /// - [`SDL_trunc`]
    pub safe fn SDL_round(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Round `x` to the nearest integer.
    ///
    /// Rounds `x` to the nearest integer. Values halfway between integers will be
    /// rounded away from zero.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`, y integer
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_round`] for double-precision floats. To get the result as an integer
    /// type, use [`SDL_lroundf`].
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the nearest integer to `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_round`]
    /// - [`SDL_lroundf`]
    /// - [`SDL_floorf`]
    /// - [`SDL_ceilf`]
    /// - [`SDL_truncf`]
    pub safe fn SDL_roundf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// [`SDL_lroundf`] for single-precision floats. To get the result as a
    /// floating-point type, use [`SDL_round`].
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the nearest integer to `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_lroundf`]
    /// - [`SDL_round`]
    /// - [`SDL_floor`]
    /// - [`SDL_ceil`]
    /// - [`SDL_trunc`]
    pub safe fn SDL_lround(x: ::core::ffi::c_double) -> ::core::ffi::c_long;
}

unsafe extern "C" {
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
    /// [`SDL_lround`] for double-precision floats. To get the result as a
    /// floating-point type, use [`SDL_roundf`].
    ///
    /// ## Parameters
    /// - `x`: floating point value.
    ///
    /// ## Return value
    /// Returns the nearest integer to `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_lround`]
    /// - [`SDL_roundf`]
    /// - [`SDL_floorf`]
    /// - [`SDL_ceilf`]
    /// - [`SDL_truncf`]
    pub safe fn SDL_lroundf(x: ::core::ffi::c_float) -> ::core::ffi::c_long;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value to be scaled.
    /// - `n`: integer exponent.
    ///
    /// ## Return value
    /// Returns `x * 2^n`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_scalbnf`]
    /// - [`SDL_pow`]
    pub safe fn SDL_scalbn(
        x: ::core::ffi::c_double,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value to be scaled.
    /// - `n`: integer exponent.
    ///
    /// ## Return value
    /// Returns `x * 2^n`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_scalbn`]
    /// - [`SDL_powf`]
    pub safe fn SDL_scalbnf(x: ::core::ffi::c_float, n: ::core::ffi::c_int)
    -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns sine of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_sinf`]
    /// - [`SDL_asin`]
    /// - [`SDL_cos`]
    pub safe fn SDL_sin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compute the sine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_sin`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns sine of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_sin`]
    /// - [`SDL_asinf`]
    /// - [`SDL_cosf`]
    pub safe fn SDL_sinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than or equal to 0.
    ///
    /// ## Return value
    /// Returns square root of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_sqrtf`]
    pub safe fn SDL_sqrt(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value. Must be greater than or equal to 0.
    ///
    /// ## Return value
    /// Returns square root of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_sqrt`]
    pub safe fn SDL_sqrtf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns tangent of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_tanf`]
    /// - [`SDL_sin`]
    /// - [`SDL_cos`]
    /// - [`SDL_atan`]
    /// - [`SDL_atan2`]
    pub safe fn SDL_tan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

unsafe extern "C" {
    /// Compute the tangent of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// [`SDL_tan`] for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// ## Parameters
    /// - `x`: floating point value, in radians.
    ///
    /// ## Return value
    /// Returns tangent of `x`.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_tan`]
    /// - [`SDL_sinf`]
    /// - [`SDL_cosf`]
    /// - [`SDL_atanf`]
    /// - [`SDL_atan2f`]
    pub safe fn SDL_tanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}

/// An opaque handle representing string encoding conversion state.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_iconv_open`]
pub type SDL_iconv_t = *mut SDL_iconv_data_t;

unsafe extern "C" {
    /// This function allocates a context for the specified character set
    /// conversion.
    ///
    /// ## Parameters
    /// - `tocode`: The target character encoding, must not be NULL.
    /// - `fromcode`: The source character encoding, must not be NULL.
    ///
    /// ## Return value
    /// Returns a handle that must be freed with [`SDL_iconv_close`], or
    ///   [`SDL_ICONV_ERROR`] on failure.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_iconv`]
    /// - [`SDL_iconv_close`]
    /// - [`SDL_iconv_string`]
    pub fn SDL_iconv_open(
        tocode: *const ::core::ffi::c_char,
        fromcode: *const ::core::ffi::c_char,
    ) -> SDL_iconv_t;
}

unsafe extern "C" {
    /// This function frees a context used for character set conversion.
    ///
    /// ## Parameters
    /// - `cd`: The character set conversion handle.
    ///
    /// ## Return value
    /// Returns 0 on success, or -1 on failure.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_iconv`]
    /// - [`SDL_iconv_open`]
    /// - [`SDL_iconv_string`]
    pub fn SDL_iconv_close(cd: SDL_iconv_t) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// This function converts text between encodings, reading from and writing to
    /// a buffer.
    ///
    /// It returns the number of successful conversions on success. On error,
    /// [`SDL_ICONV_E2BIG`] is returned when the output buffer is too small, or
    /// [`SDL_ICONV_EILSEQ`] is returned when an invalid input sequence is encountered,
    /// or [`SDL_ICONV_EINVAL`] is returned when an incomplete input sequence is
    /// encountered.
    ///
    /// On exit:
    ///
    /// - inbuf will point to the beginning of the next multibyte sequence. On
    ///   error, this is the location of the problematic input sequence. On
    ///   success, this is the end of the input sequence.
    /// - inbytesleft will be set to the number of bytes left to convert, which
    ///   will be 0 on success.
    /// - outbuf will point to the location where to store the next output byte.
    /// - outbytesleft will be set to the number of bytes left in the output
    ///   buffer.
    ///
    /// ## Parameters
    /// - `cd`: The character set conversion context, created in
    ///   [`SDL_iconv_open()`].
    /// - `inbuf`: Address of variable that points to the first character of the
    ///   input sequence.
    /// - `inbytesleft`: The number of bytes in the input buffer.
    /// - `outbuf`: Address of variable that points to the output buffer.
    /// - `outbytesleft`: The number of bytes in the output buffer.
    ///
    /// ## Return value
    /// Returns the number of conversions on success, or a negative error code.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_iconv_open`]
    /// - [`SDL_iconv_close`]
    /// - [`SDL_iconv_string`]
    pub fn SDL_iconv(
        cd: SDL_iconv_t,
        inbuf: *mut *const ::core::ffi::c_char,
        inbytesleft: *mut ::core::primitive::usize,
        outbuf: *mut *mut ::core::ffi::c_char,
        outbytesleft: *mut ::core::primitive::usize,
    ) -> ::core::primitive::usize;
}

/// Generic error. Check [`SDL_GetError()`]?
pub const SDL_ICONV_ERROR: ::core::primitive::usize = (-1_i32 as ::core::primitive::usize);

/// Output buffer was too small.
pub const SDL_ICONV_E2BIG: ::core::primitive::usize = (-2_i32 as ::core::primitive::usize);

/// Invalid input sequence was encountered.
pub const SDL_ICONV_EILSEQ: ::core::primitive::usize = (-3_i32 as ::core::primitive::usize);

/// Incomplete input sequence was encountered.
pub const SDL_ICONV_EINVAL: ::core::primitive::usize = (-4_i32 as ::core::primitive::usize);

unsafe extern "C" {
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
    /// ## Parameters
    /// - `tocode`: the character encoding of the output string. Examples are
    ///   "UTF-8", "UCS-4", etc.
    /// - `fromcode`: the character encoding of data in `inbuf`.
    /// - `inbuf`: the string to convert to a different encoding.
    /// - `inbytesleft`: the size of the input string _in bytes_.
    ///
    /// ## Return value
    /// Returns a new string, converted to the new encoding, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_iconv_open`]
    /// - [`SDL_iconv_close`]
    /// - [`SDL_iconv`]
    pub fn SDL_iconv_string(
        tocode: *const ::core::ffi::c_char,
        fromcode: *const ::core::ffi::c_char,
        inbuf: *const ::core::ffi::c_char,
        inbytesleft: ::core::primitive::usize,
    ) -> *mut ::core::ffi::c_char;
}

/// Convert a UTF-8 string to the current locale's character encoding.
///
/// This is a helper macro that might be more clear than calling
/// [`SDL_iconv_string`] directly. However, it double-evaluates its parameter, so
/// do not use an expression with side-effects here.
///
/// ## Parameters
/// - `S`: the string to convert.
///
/// ## Return value
/// Returns a new string, converted to the new encoding, or NULL on error.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
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

/// Convert a UTF-8 string to UCS-2.
///
/// This is a helper macro that might be more clear than calling
/// [`SDL_iconv_string`] directly. However, it double-evaluates its parameter, so
/// do not use an expression with side-effects here.
///
/// ## Parameters
/// - `S`: the string to convert.
///
/// ## Return value
/// Returns a new string, converted to the new encoding, or NULL on error.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
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

/// Convert a UTF-8 string to UCS-4.
///
/// This is a helper macro that might be more clear than calling
/// [`SDL_iconv_string`] directly. However, it double-evaluates its parameter, so
/// do not use an expression with side-effects here.
///
/// ## Parameters
/// - `S`: the string to convert.
///
/// ## Return value
/// Returns a new string, converted to the new encoding, or NULL on error.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
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

/// Convert a wchar_t string to UTF-8.
///
/// This is a helper macro that might be more clear than calling
/// [`SDL_iconv_string`] directly. However, it double-evaluates its parameter, so
/// do not use an expression with side-effects here.
///
/// ## Parameters
/// - `S`: the string to convert.
///
/// ## Return value
/// Returns a new string, converted to the new encoding, or NULL on error.
///
/// ## Availability
/// This macro is available since SDL 3.2.0.
#[inline(always)]
pub unsafe fn SDL_iconv_wchar_utf8(S: *const crate::ffi::c_wchar_t) -> *mut ::core::ffi::c_char {
    unsafe {
        SDL_iconv_string(
            c"UTF-8".as_ptr(),
            c"WCHAR_T".as_ptr(),
            (S as *const ::core::ffi::c_char),
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
/// ## Parameters
/// - `a`: the multiplicand.
/// - `b`: the multiplier.
/// - `ret`: on non-overflow output, stores the multiplication result, may
///   not be NULL.
///
/// ## Return value
/// Returns false on overflow, true if result is multiplied without overflow.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
#[inline(always)]
pub const unsafe fn SDL_size_mul_check_overflow(
    a: ::core::primitive::usize,
    b: ::core::primitive::usize,
    ret: *mut ::core::primitive::usize,
) -> ::core::primitive::bool {
    if ((a != 0_usize) && (b > (SDL_SIZE_MAX / a))) {
        return false;
    }
    unsafe {
        let (ptr, value) = (ret, (a * b));
        ptr.write(value);
        value
    };
    return true;
}

apply_cfg!(#[cfg(not(doc))] => {
});

/// Add two integers, checking for overflow.
///
/// If `a + b` would overflow, return false.
///
/// Otherwise store `a + b` via ret and return true.
///
/// ## Parameters
/// - `a`: the first addend.
/// - `b`: the second addend.
/// - `ret`: on non-overflow output, stores the addition result, may not be
///   NULL.
///
/// ## Return value
/// Returns false on overflow, true if result is added without overflow.
///
/// ## Thread safety
/// It is safe to call this function from any thread.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
#[inline(always)]
pub const unsafe fn SDL_size_add_check_overflow(
    a: ::core::primitive::usize,
    b: ::core::primitive::usize,
    ret: *mut ::core::primitive::usize,
) -> ::core::primitive::bool {
    if (b > (SDL_SIZE_MAX - a)) {
        return false;
    }
    unsafe {
        let (ptr, value) = (ret, (a + b));
        ptr.write(value);
        value
    };
    return true;
}

apply_cfg!(#[cfg(not(doc))] => {
});

apply_cfg!(#[cfg(doc)] => {
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
    /// ## Availability
    /// This datatype is available since SDL 3.2.0.
    pub type SDL_FunctionPointer = ::core::option::Option<unsafe extern "C" fn()>;

});

apply_cfg!(#[cfg(not(doc))] => {
    pub type SDL_FunctionPointer = ::core::option::Option<unsafe extern "C" fn()>;

});

/// A thread-safe set of environment variables
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetEnvironment`]
/// - [`SDL_CreateEnvironment`]
/// - [`SDL_GetEnvironmentVariable`]
/// - [`SDL_GetEnvironmentVariables`]
/// - [`SDL_SetEnvironmentVariable`]
/// - [`SDL_UnsetEnvironmentVariable`]
/// - [`SDL_DestroyEnvironment`]
#[repr(C)]
pub struct SDL_Environment {
    _opaque: [::core::primitive::u8; 0],
}

#[repr(C)]
pub struct SDL_iconv_data_t {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
