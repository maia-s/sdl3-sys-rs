#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph)]

//! # CategoryCPUInfo
//!
//! CPU feature detection for SDL.
//!
//! These functions are largely concerned with reporting if the system has
//! access to various SIMD instruction sets, but also has other important info
//! to share, such as system RAM size and number of logical CPU cores.

use super::stdinc::*;

/// A guess for the cacheline size used for padding.
///
/// Most x86 processors have a 64 byte cache line. The 64-bit PowerPC
/// processors have a 128 byte cache line. We use the larger value to be
/// generally safe.
///
/// \since This macro is available since SDL 3.0.0.
pub const SDL_CACHELINE_SIZE: ::core::primitive::i32 = 128;

extern "C" {
    /// Get the number of CPU cores available.
    ///
    /// \returns the total number of logical CPU cores. On CPUs that include
    ///          technologies such as hyperthreading, the number of logical cores
    ///          may be more than the number of physical cores.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetCPUCount() -> ::core::ffi::c_int;
}

extern "C" {
    /// Determine the L1 cache line size of the CPU.
    ///
    /// This is useful for determining multi-threaded structure padding or SIMD
    /// prefetch sizes.
    ///
    /// \returns the L1 cache line size of the CPU, in bytes.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetCPUCacheLineSize() -> ::core::ffi::c_int;
}

extern "C" {
    /// Determine whether the CPU has AltiVec features.
    ///
    /// This always returns false on CPUs that aren't using PowerPC instruction
    /// sets.
    ///
    /// \returns SDL_TRUE if the CPU has AltiVec features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_HasAltiVec() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has MMX features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has MMX features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_HasMMX() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has SSE features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has SSE features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasSSE2
    /// \sa SDL_HasSSE3
    /// \sa SDL_HasSSE41
    /// \sa SDL_HasSSE42
    pub fn SDL_HasSSE() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has SSE2 features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has SSE2 features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasSSE
    /// \sa SDL_HasSSE3
    /// \sa SDL_HasSSE41
    /// \sa SDL_HasSSE42
    pub fn SDL_HasSSE2() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has SSE3 features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has SSE3 features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasSSE
    /// \sa SDL_HasSSE2
    /// \sa SDL_HasSSE41
    /// \sa SDL_HasSSE42
    pub fn SDL_HasSSE3() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has SSE4.1 features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has SSE4.1 features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasSSE
    /// \sa SDL_HasSSE2
    /// \sa SDL_HasSSE3
    /// \sa SDL_HasSSE42
    pub fn SDL_HasSSE41() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has SSE4.2 features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has SSE4.2 features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasSSE
    /// \sa SDL_HasSSE2
    /// \sa SDL_HasSSE3
    /// \sa SDL_HasSSE41
    pub fn SDL_HasSSE42() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has AVX features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has AVX features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasAVX2
    /// \sa SDL_HasAVX512F
    pub fn SDL_HasAVX() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has AVX2 features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has AVX2 features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasAVX
    /// \sa SDL_HasAVX512F
    pub fn SDL_HasAVX2() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has AVX-512F (foundation) features.
    ///
    /// This always returns false on CPUs that aren't using Intel instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has AVX-512F features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasAVX
    /// \sa SDL_HasAVX2
    pub fn SDL_HasAVX512F() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has ARM SIMD (ARMv6) features.
    ///
    /// This is different from ARM NEON, which is a different instruction set.
    ///
    /// This always returns false on CPUs that aren't using ARM instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has ARM SIMD features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasNEON
    pub fn SDL_HasARMSIMD() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has NEON (ARM SIMD) features.
    ///
    /// This always returns false on CPUs that aren't using ARM instruction sets.
    ///
    /// \returns SDL_TRUE if the CPU has ARM NEON features or SDL_FALSE if not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_HasNEON() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has LSX (LOONGARCH SIMD) features.
    ///
    /// This always returns false on CPUs that aren't using LOONGARCH instruction
    /// sets.
    ///
    /// \returns SDL_TRUE if the CPU has LOONGARCH LSX features or SDL_FALSE if
    ///          not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_HasLSX() -> SDL_bool;
}

extern "C" {
    /// Determine whether the CPU has LASX (LOONGARCH SIMD) features.
    ///
    /// This always returns false on CPUs that aren't using LOONGARCH instruction
    /// sets.
    ///
    /// \returns SDL_TRUE if the CPU has LOONGARCH LASX features or SDL_FALSE if
    ///          not.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_HasLASX() -> SDL_bool;
}

extern "C" {
    /// Get the amount of RAM configured in the system.
    ///
    /// \returns the amount of RAM configured in the system in MiB.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSystemRAM() -> ::core::ffi::c_int;
}

extern "C" {
    /// Report the alignment this system needs for SIMD allocations.
    ///
    /// This will return the minimum number of bytes to which a pointer must be
    /// aligned to be compatible with SIMD instructions on the current machine. For
    /// example, if the machine supports SSE only, it will return 16, but if it
    /// supports AVX-512F, it'll return 64 (etc). This only reports values for
    /// instruction sets SDL knows about, so if your SDL build doesn't have
    /// SDL_HasAVX512F(), then it might return 16 for the SSE support it sees and
    /// not 64 for the AVX-512 instructions that exist but SDL doesn't know about.
    /// Plan accordingly.
    ///
    /// \returns the alignment in bytes needed for available, known SIMD
    ///          instructions.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_aligned_alloc
    /// \sa SDL_aligned_free
    pub fn SDL_GetSIMDAlignment() -> ::core::primitive::usize;
}

