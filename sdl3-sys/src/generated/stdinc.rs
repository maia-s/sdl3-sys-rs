#![allow(non_camel_case_types)]

//! # CategoryStdinc
//!
//! This is a general header that includes C language support. It implements a
//! subset of the C runtime: these should all behave the same way as their C
//! runtime equivalents, but with an SDL_ prefix.

#[cfg(doc)]
emit! {
}

#[cfg(not(doc))]
emit! {
}

/// A boolean type: true or false.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_TRUE
/// \sa SDL_FALSE
pub type SDL_bool = ::core::primitive::bool;

/// A signed 8-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Sint8 = ::core::primitive::i8;

/// An unsigned 8-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Uint8 = ::core::primitive::u8;

/// A signed 16-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Sint16 = ::core::primitive::i16;

/// An unsigned 16-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Uint16 = ::core::primitive::u16;

/// A signed 32-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Sint32 = ::core::primitive::i32;

/// An unsigned 32-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
pub type Uint32 = ::core::primitive::u32;

/// A signed 64-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
///
/// \sa SDL_SINT64_C
pub type Sint64 = ::core::primitive::i64;

/// An unsigned 64-bit integer type.
///
/// \since This macro is available since SDL 3.0.0.
///
/// \sa SDL_UINT64_C
pub type Uint64 = ::core::primitive::u64;

/// SDL times are signed, 64-bit integers representing nanoseconds since the
/// Unix epoch (Jan 1, 1970).
///
/// They can be converted between POSIX time_t values with SDL_NS_TO_SECONDS()
/// and SDL_SECONDS_TO_NS(), and between Windows FILETIME values with
/// SDL_TimeToWindows() and SDL_TimeFromWindows().
///
/// \since This macro is available since SDL 3.0.0.
///
/// \sa SDL_MAX_SINT64
/// \sa SDL_MIN_SINT64
pub type SDL_Time = Sint64;

#[cfg(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows))]
emit! {
}

#[cfg(not(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows)))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
    }

}

#[cfg(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows))]
emit! {
}

#[cfg(not(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows)))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
    }

}

#[cfg(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows))]
emit! {
}

#[cfg(not(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows)))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
    }

}

#[cfg(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows))]
emit! {
}

#[cfg(not(any(any(/* always disabled: SDL_PLATFORM_GDK */), windows)))]
emit! {
    #[cfg(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64")))]
    emit! {
    }

    #[cfg(not(all(not(target_vendor = "apple"), all(not(windows), target_pointer_width = "64"))))]
    emit! {
    }

}

const _: () = ::core::assert!(::core::mem::size_of::<SDL_bool>() == 1);
const _: () = ::core::assert!(::core::mem::size_of::<Uint8>() == 1);
const _: () = ::core::assert!(::core::mem::size_of::<Sint8>() == 1);
const _: () = ::core::assert!(::core::mem::size_of::<Uint16>() == 2);
const _: () = ::core::assert!(::core::mem::size_of::<Sint16>() == 2);
const _: () = ::core::assert!(::core::mem::size_of::<Uint32>() == 4);
const _: () = ::core::assert!(::core::mem::size_of::<Sint32>() == 4);
const _: () = ::core::assert!(::core::mem::size_of::<Uint64>() == 8);
const _: () = ::core::assert!(::core::mem::size_of::<Sint64>() == 8);
#[cfg(all(not(any(/* always disabled: SDL_PLATFORM_3DS */)), not(any(/* always disabled: SDL_PLATFORM_VITA */))))]
emit! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SDL_DUMMY_ENUM(pub ::core::ffi::c_int);
    impl SDL_DUMMY_ENUM {
        pub const DUMMY_ENUM_VALUE: Self = Self(0);
    }
    pub const DUMMY_ENUM_VALUE: SDL_DUMMY_ENUM = SDL_DUMMY_ENUM::DUMMY_ENUM_VALUE;

    const _: () = ::core::assert!(::core::mem::size_of::<SDL_DUMMY_ENUM>() == ::core::mem::size_of::<::core::ffi::c_int>());
}

extern_sdlcall! {{
    pub fn SDL_malloc(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_calloc(nmemb: ::core::primitive::usize, size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_realloc(mem: *mut ::core::ffi::c_void, size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_free(mem: *mut ::core::ffi::c_void);
}}

pub type SDL_malloc_func = ::core::option::Option<extern_sdlcall!(fn(size: ::core::primitive::usize) -> *mut ::core::ffi::c_void)>;

pub type SDL_calloc_func = ::core::option::Option<extern_sdlcall!(fn(nmemb: ::core::primitive::usize, size: ::core::primitive::usize) -> *mut ::core::ffi::c_void)>;

pub type SDL_realloc_func = ::core::option::Option<extern_sdlcall!(fn(mem: *mut ::core::ffi::c_void, size: ::core::primitive::usize) -> *mut ::core::ffi::c_void)>;

pub type SDL_free_func = ::core::option::Option<extern_sdlcall!(fn(mem: *mut ::core::ffi::c_void))>;

extern_sdlcall! {{
    /// Get the original set of SDL memory functions.
    ///
    /// This is what SDL_malloc and friends will use by default, if there has been
    /// no call to SDL_SetMemoryFunctions. This is not necessarily using the C
    /// runtime's `malloc` functions behind the scenes! Different platforms and
    /// build configurations might do any number of unexpected things.
    ///
    /// \param malloc_func filled with malloc function.
    /// \param calloc_func filled with calloc function.
    /// \param realloc_func filled with realloc function.
    /// \param free_func filled with free function.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetOriginalMemoryFunctions(malloc_func: *mut SDL_malloc_func, calloc_func: *mut SDL_calloc_func, realloc_func: *mut SDL_realloc_func, free_func: *mut SDL_free_func);
}}

extern_sdlcall! {{
    /// Get the current set of SDL memory functions.
    ///
    /// \param malloc_func filled with malloc function.
    /// \param calloc_func filled with calloc function.
    /// \param realloc_func filled with realloc function.
    /// \param free_func filled with free function.
    ///
    /// \threadsafety This does not hold a lock, so do not call this in the
    ///               unlikely event of a background thread calling
    ///               SDL_SetMemoryFunctions simultaneously.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetMemoryFunctions
    /// \sa SDL_GetOriginalMemoryFunctions
    pub fn SDL_GetMemoryFunctions(malloc_func: *mut SDL_malloc_func, calloc_func: *mut SDL_calloc_func, realloc_func: *mut SDL_realloc_func, free_func: *mut SDL_free_func);
}}

extern_sdlcall! {{
    /// Replace SDL's memory allocation functions with a custom set.
    ///
    /// It is not safe to call this function once any allocations have been made,
    /// as future calls to SDL_free will use the new allocator, even if they came
    /// from an SDL_malloc made with the old one!
    ///
    /// If used, usually this needs to be the first call made into the SDL library,
    /// if not the very first thing done at program startup time.
    ///
    /// \param malloc_func custom malloc function.
    /// \param calloc_func custom calloc function.
    /// \param realloc_func custom realloc function.
    /// \param free_func custom free function.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread, but one
    ///               should not replace the memory functions once any allocations
    ///               are made!
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetMemoryFunctions
    /// \sa SDL_GetOriginalMemoryFunctions
    pub fn SDL_SetMemoryFunctions(malloc_func: SDL_malloc_func, calloc_func: SDL_calloc_func, realloc_func: SDL_realloc_func, free_func: SDL_free_func) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Allocate memory aligned to a specific value.
    ///
    /// If `alignment` is less than the size of `void *`, then it will be increased
    /// to match that.
    ///
    /// The returned memory address will be a multiple of the alignment value, and
    /// the amount of memory allocated will be a multiple of the alignment value.
    ///
    /// The memory returned by this function must be freed with SDL_aligned_free(),
    /// and _not_ SDL_free.
    ///
    /// \param alignment the alignment requested.
    /// \param size the size to allocate.
    /// \returns a pointer to the aligned memory.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_aligned_free
    pub fn SDL_aligned_alloc(alignment: ::core::primitive::usize, size: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    /// Free memory allocated by SDL_aligned_alloc().
    ///
    /// The pointer is no longer valid after this call and cannot be dereferenced
    /// anymore.
    ///
    /// \param mem a pointer previously returned by SDL_aligned_alloc.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_aligned_alloc
    pub fn SDL_aligned_free(mem: *mut ::core::ffi::c_void);
}}

extern_sdlcall! {{
    /// Get the number of outstanding (unfreed) allocations.
    ///
    /// \returns the number of allocations.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetNumAllocations() -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_getenv(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_setenv(name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char, overwrite: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_unsetenv(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

pub type SDL_CompareCallback = ::core::option::Option<extern_sdlcall!(fn(a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void) -> ::core::ffi::c_int)>;

extern_sdlcall! {{
    pub fn SDL_qsort(base: *mut ::core::ffi::c_void, nmemb: ::core::primitive::usize, size: ::core::primitive::usize, compare: SDL_CompareCallback);
}}

extern_sdlcall! {{
    pub fn SDL_bsearch(key: *const ::core::ffi::c_void, base: *const ::core::ffi::c_void, nmemb: ::core::primitive::usize, size: ::core::primitive::usize, compare: SDL_CompareCallback) -> *mut ::core::ffi::c_void;
}}

pub type SDL_CompareCallback_r = ::core::option::Option<extern_sdlcall!(fn(userdata: *mut ::core::ffi::c_void, a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void) -> ::core::ffi::c_int)>;

extern_sdlcall! {{
    pub fn SDL_qsort_r(base: *mut ::core::ffi::c_void, nmemb: ::core::primitive::usize, size: ::core::primitive::usize, compare: SDL_CompareCallback_r, userdata: *mut ::core::ffi::c_void);
}}

extern_sdlcall! {{
    pub fn SDL_bsearch_r(key: *const ::core::ffi::c_void, base: *const ::core::ffi::c_void, nmemb: ::core::primitive::usize, size: ::core::primitive::usize, compare: SDL_CompareCallback_r, userdata: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_abs(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Query if a character is alphabetic (a letter).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z' and 'A-Z' as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isalpha(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Query if a character is alphabetic (a letter) or a number.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// for English 'a-z', 'A-Z', and '0-9' as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isalnum(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is blank (a space or tab).
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0x20 (space) or 0x9 (tab) as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isblank(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is a control character.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 0 through 0x1F, and 0x7F, as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_iscntrl(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is a numeric digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// '0' (0x30) through '9' (0x39), as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is a hexadecimal digit.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'F', 'a' through 'f', and '0' through '9', as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isxdigit(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is a punctuation mark.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `((SDL_isgraph(x)) && (!SDL_isalnum(x)))`.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isgraph
    /// \sa SDL_isalnum
    pub fn SDL_ispunct(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isspace(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is upper case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'A' through 'Z' as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is lower case.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// 'a' through 'z' as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_islower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is "printable".
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this will only treat ASCII values
    /// ' ' (0x20) through '~' (0x7E) as true.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_isprint(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Report if a character is any "printable" except space.
    ///
    /// Be advised that "printable" has a definition that goes back to text
    /// terminals from the dawn of computing, making this a sort of special case
    /// function that is not suitable for Unicode (or most any) text management.
    ///
    /// **WARNING**: Regardless of system locale, this is equivalent to
    /// `(SDL_isprint(x)) && ((x) != ' ')`.
    ///
    /// \param x character value to check.
    /// \returns non-zero if x falls within the character class, zero otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isprint
    pub fn SDL_isgraph(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Convert low-ASCII English letters to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'a' through 'z' to uppercase.
    ///
    /// This function returns the uppercase equivalent of `x`. If a character
    /// cannot be converted, or is already uppercase, this function returns `x`.
    ///
    /// \param x character value to check.
    /// \returns capitalized version of x, or x if no conversion available.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_toupper(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Convert low-ASCII English letters to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function returns the lowercase equivalent of `x`. If a character
    /// cannot be converted, or is already lowercase, this function returns `x`.
    ///
    /// \param x character value to check.
    /// \returns lowercase version of x, or x if no conversion available.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_tolower(x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_crc16(crc: Uint16, data: *const ::core::ffi::c_void, len: ::core::primitive::usize) -> Uint16;
}}

extern_sdlcall! {{
    pub fn SDL_crc32(crc: Uint32, data: *const ::core::ffi::c_void, len: ::core::primitive::usize) -> Uint32;
}}

extern_sdlcall! {{
    pub fn SDL_memcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, len: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_memmove(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, len: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_memset(dst: *mut ::core::ffi::c_void, c: ::core::ffi::c_int, len: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_memset4(dst: *mut ::core::ffi::c_void, val: Uint32, dwords: ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    pub fn SDL_memcmp(s1: *const ::core::ffi::c_void, s2: *const ::core::ffi::c_void, len: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_wcslen(wstr: *const crate::ffi::c_wchar_t) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_wcsnlen(wstr: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_wcslcpy(dst: *mut crate::ffi::c_wchar_t, src: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_wcslcat(dst: *mut crate::ffi::c_wchar_t, src: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_wcsdup(wstr: *const crate::ffi::c_wchar_t) -> *mut crate::ffi::c_wchar_t;
}}

extern_sdlcall! {{
    pub fn SDL_wcsstr(haystack: *const crate::ffi::c_wchar_t, needle: *const crate::ffi::c_wchar_t) -> *mut crate::ffi::c_wchar_t;
}}

extern_sdlcall! {{
    pub fn SDL_wcsnstr(haystack: *const crate::ffi::c_wchar_t, needle: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> *mut crate::ffi::c_wchar_t;
}}

extern_sdlcall! {{
    /// Compare two null-terminated wide strings.
    ///
    /// This only compares wchar_t values until it hits a null-terminating
    /// character; it does not care if the string is well-formed UTF-16 (or UTF-32,
    /// depending on your platform's wchar_t size), or uses valid Unicode values.
    ///
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_wcscmp(str1: *const crate::ffi::c_wchar_t, str2: *const crate::ffi::c_wchar_t) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \param maxlen the maximum number of wchar_t to compare.
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_wcsncmp(str1: *const crate::ffi::c_wchar_t, str2: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_wcscasecmp(str1: *const crate::ffi::c_wchar_t, str2: *const crate::ffi::c_wchar_t) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \param maxlen the maximum number of wchar_t values to compare.
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_wcsncasecmp(str1: *const crate::ffi::c_wchar_t, str2: *const crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_wcstol(str: *const crate::ffi::c_wchar_t, endp: *mut *mut crate::ffi::c_wchar_t, base: ::core::ffi::c_int) -> ::core::ffi::c_long;
}}

extern_sdlcall! {{
    pub fn SDL_strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_strnlen(str: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_strlcpy(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_utf8strlcpy(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, dst_bytes: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_strlcat(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_strdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strndup(str: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strrev(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Convert a string to uppercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to uppercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'a' through 'z' to their
    /// uppercase equivalents in-place, returning the original `str` pointer.
    ///
    /// \param str the string to convert in-place. Can not be NULL.
    /// \returns the `str` pointer passed into this function.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_strlwr
    pub fn SDL_strupr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Convert a string to lowercase.
    ///
    /// **WARNING**: Regardless of system locale, this will only convert ASCII
    /// values 'A' through 'Z' to lowercase.
    ///
    /// This function operates on a null-terminated string of bytes--even if it is
    /// malformed UTF-8!--and converts ASCII characters 'A' through 'Z' to their
    /// lowercase equivalents in-place, returning the original `str` pointer.
    ///
    /// \param str the string to convert in-place. Can not be NULL.
    /// \returns the `str` pointer passed into this function.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_strupr
    pub fn SDL_strlwr(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strchr(str: *const ::core::ffi::c_char, c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strrchr(str: *const ::core::ffi::c_char, c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strstr(haystack: *const ::core::ffi::c_char, needle: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strnstr(haystack: *const ::core::ffi::c_char, needle: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strcasestr(haystack: *const ::core::ffi::c_char, needle: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_strtok_r(s1: *mut ::core::ffi::c_char, s2: *const ::core::ffi::c_char, saveptr: *mut *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_utf8strlen(str: *const ::core::ffi::c_char) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_utf8strnlen(str: *const ::core::ffi::c_char, bytes: ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    pub fn SDL_itoa(value: ::core::ffi::c_int, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_uitoa(value: ::core::ffi::c_uint, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_ltoa(value: ::core::ffi::c_long, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_ultoa(value: ::core::ffi::c_ulong, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_lltoa(value: Sint64, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_ulltoa(value: Uint64, str: *mut ::core::ffi::c_char, radix: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_atoi(str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_atof(str: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    pub fn SDL_strtol(str: *const ::core::ffi::c_char, endp: *mut *mut ::core::ffi::c_char, base: ::core::ffi::c_int) -> ::core::ffi::c_long;
}}

extern_sdlcall! {{
    pub fn SDL_strtoul(str: *const ::core::ffi::c_char, endp: *mut *mut ::core::ffi::c_char, base: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
}}

extern_sdlcall! {{
    pub fn SDL_strtoll(str: *const ::core::ffi::c_char, endp: *mut *mut ::core::ffi::c_char, base: ::core::ffi::c_int) -> Sint64;
}}

extern_sdlcall! {{
    pub fn SDL_strtoull(str: *const ::core::ffi::c_char, endp: *mut *mut ::core::ffi::c_char, base: ::core::ffi::c_int) -> Uint64;
}}

extern_sdlcall! {{
    pub fn SDL_strtod(str: *const ::core::ffi::c_char, endp: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compare two null-terminated UTF-8 strings.
    ///
    /// Due to the nature of UTF-8 encoding, this will work with Unicode strings,
    /// since effectively this function just compares bytes until it hits a
    /// null-terminating character. Also due to the nature of UTF-8, this can be
    /// used with SDL_qsort() to put strings in (roughly) alphabetical order.
    ///
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_strcmp(str1: *const ::core::ffi::c_char, str2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Compare two UTF-8 strings up to a number of bytes.
    ///
    /// Due to the nature of UTF-8 encoding, this will work with Unicode strings,
    /// since effectively this function just compares bytes until it hits a
    /// null-terminating character. Also due to the nature of UTF-8, this can be
    /// used with SDL_qsort() to put strings in (roughly) alphabetical order.
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \param maxlen the maximum number of _bytes_ to compare.
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_strncmp(str1: *const ::core::ffi::c_char, str2: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_strcasecmp(str1: *const ::core::ffi::c_char, str2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// \param str1 the first string to compare. NULL is not permitted!
    /// \param str2 the second string to compare. NULL is not permitted!
    /// \param maxlen the maximum number of bytes to compare.
    /// \returns less than zero if str1 is "less than" str2, greater than zero if
    ///          str1 is "greater than" str2, and zero if the strings match
    ///          exactly.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_strncasecmp(str1: *const ::core::ffi::c_char, str2: *const ::core::ffi::c_char, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
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
    /// SDL_INVALID_UNICODE_CODEPOINT and advances the string/length by one byte
    /// (which is to say, a multibyte sequence might produce several
    /// SDL_INVALID_UNICODE_CODEPOINT returns before it syncs to the next valid
    /// UTF-8 sequence).
    ///
    /// Several things can generate invalid UTF-8 sequences, including overlong
    /// encodings, the use of UTF-16 surrogate values, and truncated data. Please
    /// refer to
    /// [RFC3629](https://www.ietf.org/rfc/rfc3629.txt)
    /// for details.
    ///
    /// \param pstr a pointer to a UTF-8 string pointer to be read and adjusted.
    /// \param pslen a pointer to the number of bytes in the string, to be read and
    ///              adjusted. NULL is allowed.
    /// \returns the first Unicode codepoint in the string.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_StepUTF8(pstr: *mut *const ::core::ffi::c_char, pslen: *mut ::core::primitive::usize) -> Uint32;
}}

extern_sdlcall! {{
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
    /// \param codepoint a Unicode codepoint to convert to UTF-8.
    /// \param dst the location to write the encoded UTF-8. Must point to at least
    ///            4 bytes!
    /// \returns the first byte past the newly-written UTF-8 sequence.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UCS4ToUTF8(codepoint: Uint32, dst: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}}

extern_sdlcall! {{
    pub fn SDL_sscanf(text: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_vsscanf(text: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ap: crate::ffi::VaList) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_snprintf(text: *mut ::core::ffi::c_char, maxlen: ::core::primitive::usize, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_swprintf(text: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize, fmt: *const crate::ffi::c_wchar_t, ...) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_vsnprintf(text: *mut ::core::ffi::c_char, maxlen: ::core::primitive::usize, fmt: *const ::core::ffi::c_char, ap: crate::ffi::VaList) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_vswprintf(text: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize, fmt: *const crate::ffi::c_wchar_t, ap: crate::ffi::VaList) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_asprintf(strp: *mut *mut ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    pub fn SDL_vasprintf(strp: *mut *mut ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ap: crate::ffi::VaList) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Seeds the pseudo-random number generator.
    ///
    /// Reusing the seed number will cause SDL_rand_*() to repeat the same stream
    /// of 'random' numbers.
    ///
    /// \param seed the value to use as a random number seed, or 0 to use
    ///             SDL_GetPerformanceCounter().
    ///
    /// \threadsafety This should be called on the same thread that calls
    ///               SDL_rand*()
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_rand
    /// \sa SDL_rand_bits
    /// \sa SDL_randf
    pub fn SDL_srand(seed: Uint64);
}}

extern_sdlcall! {{
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
    /// If you want reproducible output, be sure to initialize with SDL_srand()
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// \param n the number of possible outcomes. n must be positive.
    /// \returns a random value in the range of [0 .. n-1].
    ///
    /// \threadsafety All calls should be made from a single thread
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_srand
    /// \sa SDL_randf
    pub fn SDL_rand(n: Sint32) -> Sint32;
}}

extern_sdlcall! {{
    /// Generate a uniform pseudo-random floating point number less than 1.0
    ///
    /// If you want reproducible output, be sure to initialize with SDL_srand()
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// \returns a random value in the range of [0.0, 1.0).
    ///
    /// \threadsafety All calls should be made from a single thread
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_srand
    /// \sa SDL_rand
    pub fn SDL_randf() -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Generate 32 pseudo-random bits.
    ///
    /// You likely want to use SDL_rand() to get a psuedo-random number instead.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// \returns a random value in the range of [0-SDL_MAX_UINT32].
    ///
    /// \threadsafety All calls should be made from a single thread
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_rand
    /// \sa SDL_randf
    /// \sa SDL_srand
    pub fn SDL_rand_bits() -> Uint32;
}}

extern_sdlcall! {{
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
    /// \param state a pointer to the current random number state, this may not be
    ///              NULL.
    /// \param n the number of possible outcomes. n must be positive.
    /// \returns a random value in the range of [0 .. n-1].
    ///
    /// \threadsafety This function is thread-safe, as long as the state pointer
    ///               isn't shared between threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_rand
    /// \sa SDL_rand_bits_r
    /// \sa SDL_randf_r
    pub fn SDL_rand_r(state: *mut Uint64, n: Sint32) -> Sint32;
}}

extern_sdlcall! {{
    /// Generate a uniform pseudo-random floating point number less than 1.0
    ///
    /// If you want reproducible output, be sure to initialize with SDL_srand()
    /// first.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// \param state a pointer to the current random number state, this may not be
    ///              NULL.
    /// \returns a random value in the range of [0.0, 1.0).
    ///
    /// \threadsafety This function is thread-safe, as long as the state pointer
    ///               isn't shared between threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_rand_bits_r
    /// \sa SDL_rand_r
    /// \sa SDL_randf
    pub fn SDL_randf_r(state: *mut Uint64) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Generate 32 pseudo-random bits.
    ///
    /// You likely want to use SDL_rand_r() to get a psuedo-random number instead.
    ///
    /// There are no guarantees as to the quality of the random sequence produced,
    /// and this should not be used for security (cryptography, passwords) or where
    /// money is on the line (loot-boxes, casinos). There are many random number
    /// libraries available with different characteristics and you should pick one
    /// of those to meet any serious needs.
    ///
    /// \param state a pointer to the current random number state, this may not be
    ///              NULL.
    /// \returns a random value in the range of [0-SDL_MAX_UINT32].
    ///
    /// \threadsafety This function is thread-safe, as long as the state pointer
    ///               isn't shared between threads.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_rand_r
    /// \sa SDL_randf_r
    pub fn SDL_rand_bits_r(state: *mut Uint64) -> Uint32;
}}

extern_sdlcall! {{
    /// Compute the arc cosine of `x`.
    ///
    /// The definition of `y = acos(x)` is `x = cos(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `0 <= y <= Pi`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_acosf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc cosine of `x`, in radians.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_acosf
    /// \sa SDL_asin
    /// \sa SDL_cos
    pub fn SDL_acos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the arc cosine of `x`.
    ///
    /// The definition of `y = acos(x)` is `x = cos(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `0 <= y <= Pi`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_acos for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc cosine of `x`, in radians.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_acos
    /// \sa SDL_asinf
    /// \sa SDL_cosf
    pub fn SDL_acosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the arc sine of `x`.
    ///
    /// The definition of `y = asin(x)` is `x = sin(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_asinf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc sine of `x`, in radians.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_asinf
    /// \sa SDL_acos
    /// \sa SDL_sin
    pub fn SDL_asin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the arc sine of `x`.
    ///
    /// The definition of `y = asin(x)` is `x = sin(y)`.
    ///
    /// Domain: `-1 <= x <= 1`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_asin for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc sine of `x`, in radians.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_asin
    /// \sa SDL_acosf
    /// \sa SDL_sinf
    pub fn SDL_asinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the arc tangent of `x`.
    ///
    /// The definition of `y = atan(x)` is `x = tan(y)`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_atanf for single-precision floats.
    ///
    /// To calculate the arc tangent of y / x, use SDL_atan2.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_atanf
    /// \sa SDL_atan2
    /// \sa SDL_tan
    pub fn SDL_atan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the arc tangent of `x`.
    ///
    /// The definition of `y = atan(x)` is `x = tan(y)`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-Pi/2 <= y <= Pi/2`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_atan for dboule-precision floats.
    ///
    /// To calculate the arc tangent of y / x, use SDL_atan2f.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns arc tangent of of `x` in radians, or 0 if `x = 0`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_atan
    /// \sa SDL_atan2f
    /// \sa SDL_tanf
    pub fn SDL_atanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_atan2f for single-precision floats.
    ///
    /// To calculate the arc tangent of a single value, use SDL_atan.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param y floating point value of the numerator (y coordinate).
    /// \param x floating point value of the denominator (x coordinate).
    /// \returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///          `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_atan2f
    /// \sa SDL_atan
    /// \sa SDL_tan
    pub fn SDL_atan2(y: ::core::ffi::c_double, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
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
    /// SDL_atan2 for double-precision floats.
    ///
    /// To calculate the arc tangent of a single value, use SDL_atanf.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param y floating point value of the numerator (y coordinate).
    /// \param x floating point value of the denominator (x coordinate).
    /// \returns arc tangent of of `y / x` in radians, or, if `x = 0`, either
    ///          `-Pi/2`, `0`, or `Pi/2`, depending on the value of `y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_atan2f
    /// \sa SDL_atan
    /// \sa SDL_tan
    pub fn SDL_atan2f(y: ::core::ffi::c_float, x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_ceilf for single-precision floats.
    ///
    /// \param x floating point value.
    /// \returns the ceiling of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ceilf
    /// \sa SDL_floor
    /// \sa SDL_trunc
    /// \sa SDL_round
    /// \sa SDL_lround
    pub fn SDL_ceil(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
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
    /// SDL_ceil for double-precision floats.
    ///
    /// \param x floating point value.
    /// \returns the ceiling of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ceil
    /// \sa SDL_floorf
    /// \sa SDL_truncf
    /// \sa SDL_roundf
    /// \sa SDL_lroundf
    pub fn SDL_ceilf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Copy the sign of one floating-point value to another.
    ///
    /// The definition of copysign is that ``copysign(x, y) = abs(x) * sign(y)``.
    ///
    /// Domain: `-INF <= x <= INF`, ``-INF <= y <= f``
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_copysignf for single-precision floats.
    ///
    /// \param x floating point value to use as the magnitude.
    /// \param y floating point value to use as the sign.
    /// \returns the floating point value with the sign of y and the magnitude of
    ///          x.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_copysignf
    /// \sa SDL_fabs
    pub fn SDL_copysign(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Copy the sign of one floating-point value to another.
    ///
    /// The definition of copysign is that ``copysign(x, y) = abs(x) * sign(y)``.
    ///
    /// Domain: `-INF <= x <= INF`, ``-INF <= y <= f``
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_copysign for double-precision floats.
    ///
    /// \param x floating point value to use as the magnitude.
    /// \param y floating point value to use as the sign.
    /// \returns the floating point value with the sign of y and the magnitude of
    ///          x.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_copysignf
    /// \sa SDL_fabsf
    pub fn SDL_copysignf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the cosine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_cosf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns cosine of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_cosf
    /// \sa SDL_acos
    /// \sa SDL_sin
    pub fn SDL_cos(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the cosine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_cos for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns cosine of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_cos
    /// \sa SDL_acosf
    /// \sa SDL_sinf
    pub fn SDL_cosf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the exponential of `x`.
    ///
    /// The definition of `y = exp(x)` is `y = e^x`, where `e` is the base of the
    /// natural logarithm. The inverse is the natural logarithm, SDL_log.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// The output will overflow if `exp(x)` is too large to be represented.
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_expf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns value of `e^x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_expf
    /// \sa SDL_log
    pub fn SDL_exp(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the exponential of `x`.
    ///
    /// The definition of `y = exp(x)` is `y = e^x`, where `e` is the base of the
    /// natural logarithm. The inverse is the natural logarithm, SDL_logf.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// The output will overflow if `exp(x)` is too large to be represented.
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_exp for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value.
    /// \returns value of `e^x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_exp
    /// \sa SDL_logf
    pub fn SDL_expf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the absolute value of `x`
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_copysignf for single-precision floats.
    ///
    /// \param x floating point value to use as the magnitude.
    /// \returns the absolute value of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_fabsf
    pub fn SDL_fabs(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the absolute value of `x`
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_copysignf for double-precision floats.
    ///
    /// \param x floating point value to use as the magnitude.
    /// \returns the absolute value of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_fabs
    pub fn SDL_fabsf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_floorf for single-precision floats.
    ///
    /// \param x floating point value.
    /// \returns the floor of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_floorf
    /// \sa SDL_ceil
    /// \sa SDL_trunc
    /// \sa SDL_round
    /// \sa SDL_lround
    pub fn SDL_floor(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
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
    /// SDL_floorf for double-precision floats.
    ///
    /// \param x floating point value.
    /// \returns the floor of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_floor
    /// \sa SDL_ceilf
    /// \sa SDL_truncf
    /// \sa SDL_roundf
    /// \sa SDL_lroundf
    pub fn SDL_floorf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_truncf for single-precision floats.
    ///
    /// \param x floating point value.
    /// \returns `x` truncated to an integer.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_truncf
    /// \sa SDL_fmod
    /// \sa SDL_ceil
    /// \sa SDL_floor
    /// \sa SDL_round
    /// \sa SDL_lround
    pub fn SDL_trunc(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
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
    /// SDL_truncf for double-precision floats.
    ///
    /// \param x floating point value.
    /// \returns `x` truncated to an integer.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_trunc
    /// \sa SDL_fmodf
    /// \sa SDL_ceilf
    /// \sa SDL_floorf
    /// \sa SDL_roundf
    /// \sa SDL_lroundf
    pub fn SDL_truncf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Return the floating-point remainder of `x / y`
    ///
    /// Divides `x` by `y`, and returns the remainder.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`, `y != 0`
    ///
    /// Range: `-y <= z <= y`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_fmodf for single-precision floats.
    ///
    /// \param x the numerator.
    /// \param y the denominator. Must not be 0.
    /// \returns the remainder of `x / y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_fmodf
    /// \sa SDL_modf
    /// \sa SDL_trunc
    /// \sa SDL_ceil
    /// \sa SDL_floor
    /// \sa SDL_round
    /// \sa SDL_lround
    pub fn SDL_fmod(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Return the floating-point remainder of `x / y`
    ///
    /// Divides `x` by `y`, and returns the remainder.
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`, `y != 0`
    ///
    /// Range: `-y <= z <= y`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_fmod for single-precision floats.
    ///
    /// \param x the numerator.
    /// \param y the denominator. Must not be 0.
    /// \returns the remainder of `x / y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_fmod
    /// \sa SDL_truncf
    /// \sa SDL_modff
    /// \sa SDL_ceilf
    /// \sa SDL_floorf
    /// \sa SDL_roundf
    /// \sa SDL_lroundf
    pub fn SDL_fmodf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Return whether the value is infinity.
    ///
    /// \param x double-precision floating point value.
    /// \returns non-zero if the value is infinity, 0 otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isinff
    pub fn SDL_isinf(x: ::core::ffi::c_double) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Return whether the value is infinity.
    ///
    /// \param x floating point value.
    /// \returns non-zero if the value is infinity, 0 otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isinf
    pub fn SDL_isinff(x: ::core::ffi::c_float) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Return whether the value is NaN.
    ///
    /// \param x double-precision floating point value.
    /// \returns non-zero if the value is NaN, 0 otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isnanf
    pub fn SDL_isnan(x: ::core::ffi::c_double) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Return whether the value is NaN.
    ///
    /// \param x floating point value.
    /// \returns non-zero if the value is NaN, 0 otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_isnan
    pub fn SDL_isnanf(x: ::core::ffi::c_float) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Compute the natural logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_logf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than 0.
    /// \returns the natural logarithm of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_logf
    /// \sa SDL_log10
    /// \sa SDL_exp
    pub fn SDL_log(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the natural logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_log for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than 0.
    /// \returns the natural logarithm of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_log
    /// \sa SDL_expf
    pub fn SDL_logf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the base-10 logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_log10f for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than 0.
    /// \returns the logarithm of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_log10f
    /// \sa SDL_log
    /// \sa SDL_pow
    pub fn SDL_log10(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the base-10 logarithm of `x`.
    ///
    /// Domain: `0 < x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// It is an error for `x` to be less than or equal to 0.
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_log10 for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than 0.
    /// \returns the logarithm of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_log10
    /// \sa SDL_logf
    /// \sa SDL_powf
    pub fn SDL_log10f(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_modff for single-precision floats.
    ///
    /// \param x floating point value.
    /// \param y output pointer to store the integer part of `x`.
    /// \returns the fractional part of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_modff
    /// \sa SDL_trunc
    /// \sa SDL_fmod
    pub fn SDL_modf(x: ::core::ffi::c_double, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Split `x` into integer and fractional parts
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_modf for double-precision floats.
    ///
    /// \param x floating point value.
    /// \param y output pointer to store the integer part of `x`.
    /// \returns the fractional part of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_modf
    /// \sa SDL_truncf
    /// \sa SDL_fmodf
    pub fn SDL_modff(x: ::core::ffi::c_float, y: *mut ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Raise `x` to the power `y`
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// If `y` is the base of the natural logarithm (e), consider using SDL_exp
    /// instead.
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_powf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x the base.
    /// \param y the exponent.
    /// \returns `x` raised to the power `y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_powf
    /// \sa SDL_exp
    /// \sa SDL_log
    pub fn SDL_pow(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Raise `x` to the power `y`
    ///
    /// Domain: `-INF <= x <= INF`, `-INF <= y <= INF`
    ///
    /// Range: `-INF <= z <= INF`
    ///
    /// If `y` is the base of the natural logarithm (e), consider using SDL_exp
    /// instead.
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_powf for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x the base.
    /// \param y the exponent.
    /// \returns `x` raised to the power `y`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_pow
    /// \sa SDL_expf
    /// \sa SDL_logf
    pub fn SDL_powf(x: ::core::ffi::c_float, y: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_roundf for single-precision floats. To get the result as an integer
    /// type, use SDL_lround.
    ///
    /// \param x floating point value.
    /// \returns the nearest integer to `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_roundf
    /// \sa SDL_lround
    /// \sa SDL_floor
    /// \sa SDL_ceil
    /// \sa SDL_trunc
    pub fn SDL_round(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
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
    /// SDL_roundf for single-precision floats. To get the result as an integer
    /// type, use SDL_lroundf.
    ///
    /// \param x floating point value.
    /// \returns the nearest integer to `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_round
    /// \sa SDL_lroundf
    /// \sa SDL_floorf
    /// \sa SDL_ceilf
    /// \sa SDL_truncf
    pub fn SDL_roundf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
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
    /// SDL_lround for single-precision floats. To get the result as a
    /// floating-point type, use SDL_round.
    ///
    /// \param x floating point value.
    /// \returns the nearest integer to `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_lroundf
    /// \sa SDL_round
    /// \sa SDL_floor
    /// \sa SDL_ceil
    /// \sa SDL_trunc
    pub fn SDL_lround(x: ::core::ffi::c_double) -> ::core::ffi::c_long;
}}

extern_sdlcall! {{
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
    /// SDL_lroundf for double-precision floats. To get the result as a
    /// floating-point type, use SDL_roundf,
    ///
    /// \param x floating point value.
    /// \returns the nearest integer to `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_lround
    /// \sa SDL_roundf
    /// \sa SDL_floorf
    /// \sa SDL_ceilf
    /// \sa SDL_truncf
    pub fn SDL_lroundf(x: ::core::ffi::c_float) -> ::core::ffi::c_long;
}}

extern_sdlcall! {{
    /// Scale `x` by an integer power of two.
    ///
    /// Multiplies `x` by the `n`th power of the floating point radix (always 2).
    ///
    /// Domain: `-INF <= x <= INF`, `n` integer
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_scalbnf for single-precision floats.
    ///
    /// \param x floating point value to be scaled.
    /// \param n integer exponent.
    /// \returns `x * 2^n`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_scalbnf
    /// \sa SDL_pow
    pub fn SDL_scalbn(x: ::core::ffi::c_double, n: ::core::ffi::c_int) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Scale `x` by an integer power of two.
    ///
    /// Multiplies `x` by the `n`th power of the floating point radix (always 2).
    ///
    /// Domain: `-INF <= x <= INF`, `n` integer
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_scalbn for double-precision floats.
    ///
    /// \param x floating point value to be scaled.
    /// \param n integer exponent.
    /// \returns `x * 2^n`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_scalbn
    /// \sa SDL_powf
    pub fn SDL_scalbnf(x: ::core::ffi::c_float, n: ::core::ffi::c_int) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the sine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_sinf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns sine of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_sinf
    /// \sa SDL_asin
    /// \sa SDL_cos
    pub fn SDL_sin(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the sine of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_sinf for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns sine of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_sin
    /// \sa SDL_asinf
    /// \sa SDL_cosf
    pub fn SDL_sinf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the square root of `x`.
    ///
    /// Domain: `0 <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_sqrtf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than or equal to 0.
    /// \returns square root of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_sqrtf
    pub fn SDL_sqrt(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the square root of `x`.
    ///
    /// Domain: `0 <= x <= INF`
    ///
    /// Range: `0 <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_sqrt for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value. Must be greater than or equal to 0.
    /// \returns square root of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_sqrt
    pub fn SDL_sqrtf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Compute the tangent of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on double-precision floating point values, use
    /// SDL_tanf for single-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns tangent of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_tanf
    /// \sa SDL_sin
    /// \sa SDL_cos
    /// \sa SDL_atan
    /// \sa SDL_atan2
    pub fn SDL_tan(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}}

extern_sdlcall! {{
    /// Compute the tangent of `x`.
    ///
    /// Domain: `-INF <= x <= INF`
    ///
    /// Range: `-INF <= y <= INF`
    ///
    /// This function operates on single-precision floating point values, use
    /// SDL_tanf for double-precision floats.
    ///
    /// This function may use a different approximation across different versions,
    /// platforms and configurations. i.e, it can return a different value given
    /// the same input on different machines or operating systems, or if SDL is
    /// updated.
    ///
    /// \param x floating point value, in radians.
    /// \returns tangent of `x`.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_tan
    /// \sa SDL_sinf
    /// \sa SDL_cosf
    /// \sa SDL_atanf
    /// \sa SDL_atan2f
    pub fn SDL_tanf(x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}}

pub type SDL_iconv_t = *mut SDL_iconv_data_t;

extern_sdlcall! {{
    /// This function allocates a context for the specified character set
    /// conversion.
    ///
    /// \param tocode The target character encoding, must not be NULL.
    /// \param fromcode The source character encoding, must not be NULL.
    /// \returns a handle that must be freed with SDL_iconv_close, or
    ///          SDL_ICONV_ERROR on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_iconv
    /// \sa SDL_iconv_close
    /// \sa SDL_iconv_string
    pub fn SDL_iconv_open(tocode: *const ::core::ffi::c_char, fromcode: *const ::core::ffi::c_char) -> SDL_iconv_t;
}}

extern_sdlcall! {{
    /// This function frees a context used for character set conversion.
    ///
    /// \param cd The character set conversion handle.
    /// \returns 0 on success, or -1 on failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_iconv
    /// \sa SDL_iconv_open
    /// \sa SDL_iconv_string
    pub fn SDL_iconv_close(cd: SDL_iconv_t) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// This function converts text between encodings, reading from and writing to
    /// a buffer.
    ///
    /// It returns the number of succesful conversions.
    ///
    /// \param cd The character set conversion context, created in
    ///           SDL_iconv_open().
    /// \param inbuf Address of variable that points to the first character of the
    ///              input sequence.
    /// \param inbytesleft The number of bytes in the input buffer.
    /// \param outbuf Address of variable that points to the output buffer.
    /// \param outbytesleft The number of bytes in the output buffer.
    /// \returns the number of conversions on success, else SDL_ICONV_E2BIG is
    ///          returned when the output buffer is too small, or SDL_ICONV_EILSEQ
    ///          is returned when an invalid input sequence is encountered, or
    ///          SDL_ICONV_EINVAL is returned when an incomplete input sequence is
    ///          encountered.
    ///
    ///          On exit:
    ///
    ///          - inbuf will point to the beginning of the next multibyte
    ///            sequence. On error, this is the location of the problematic
    ///            input sequence. On success, this is the end of the input
    ///            sequence. - inbytesleft will be set to the number of bytes left
    ///            to convert, which will be 0 on success. - outbuf will point to
    ///            the location where to store the next output byte. - outbytesleft
    ///            will be set to the number of bytes left in the output buffer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_iconv_open
    /// \sa SDL_iconv_close
    /// \sa SDL_iconv_string
    pub fn SDL_iconv(cd: SDL_iconv_t, inbuf: *mut *const ::core::ffi::c_char, inbytesleft: *mut ::core::primitive::usize, outbuf: *mut *mut ::core::ffi::c_char, outbytesleft: *mut ::core::primitive::usize) -> ::core::primitive::usize;
}}

extern_sdlcall! {{
    /// Helper function to convert a string's encoding in one call.
    ///
    /// This function converts a buffer or string between encodings in one pass.
    ///
    /// The string does not need to be NULL-terminated; this function operates on
    /// the number of bytes specified in `inbytesleft` whether there is a NULL
    /// character anywhere in the buffer.
    ///
    /// The returned string is owned by the caller, and should be passed to
    /// SDL_free when no longer needed.
    ///
    /// \param tocode the character encoding of the output string. Examples are
    ///               "UTF-8", "UCS-4", etc.
    /// \param fromcode the character encoding of data in `inbuf`.
    /// \param inbuf the string to convert to a different encoding.
    /// \param inbytesleft the size of the input string _in bytes_.
    /// \returns a new string, converted to the new encoding, or NULL on error.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_iconv_open
    /// \sa SDL_iconv_close
    /// \sa SDL_iconv
    pub fn SDL_iconv_string(tocode: *const ::core::ffi::c_char, fromcode: *const ::core::ffi::c_char, inbuf: *const ::core::ffi::c_char, inbytesleft: ::core::primitive::usize) -> *mut ::core::ffi::c_char;
}}

extern "C" {
    /// Multiply two integers, checking for overflow.
    ///
    /// If `a * b` would overflow, return -1.
    ///
    /// Otherwise store `a * b` via ret and return 0.
    ///
    /// \param a the multiplicand.
    /// \param b the multiplier.
    /// \param ret on non-overflow output, stores the multiplication result. May
    ///            not be NULL.
    /// \returns -1 on overflow, 0 if result doesn't overflow.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_size_mul_overflow(a: ::core::primitive::usize, b: ::core::primitive::usize, ret: *mut ::core::primitive::usize) -> ::core::ffi::c_int;
}

#[cfg(not(doc))]
emit! {
    extern "C" {
        pub fn SDL_size_mul_overflow_builtin(a: ::core::primitive::usize, b: ::core::primitive::usize, ret: *mut ::core::primitive::usize) -> ::core::ffi::c_int;
    }

}

extern "C" {
    /// Add two integers, checking for overflow.
    ///
    /// If `a + b` would overflow, return -1.
    ///
    /// Otherwise store `a + b` via ret and return 0.
    ///
    /// \param a the first addend.
    /// \param b the second addend.
    /// \param ret on non-overflow output, stores the addition result. May not be
    ///            NULL.
    /// \returns -1 on overflow, 0 if result doesn't overflow.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_size_add_overflow(a: ::core::primitive::usize, b: ::core::primitive::usize, ret: *mut ::core::primitive::usize) -> ::core::ffi::c_int;
}

#[cfg(not(doc))]
emit! {
    extern "C" {
        pub fn SDL_size_add_overflow_builtin(a: ::core::primitive::usize, b: ::core::primitive::usize, ret: *mut ::core::primitive::usize) -> ::core::ffi::c_int;
    }

}

pub type SDL_FunctionPointer = ::core::option::Option<extern "C" fn()>;

#[repr(C)]
#[non_exhaustive]
pub struct SDL_iconv_data_t { _opaque: [::core::primitive::u8; 0] }

