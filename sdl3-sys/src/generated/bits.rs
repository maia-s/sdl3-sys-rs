//! Functions for fiddling with bits and bitmasks.

use super::stdinc::*;

/// Get the index of the most significant (set) bit in a 32-bit number.
///
/// Result is undefined when called with 0. This operation can also be stated
/// as "count leading zeroes" and "log base 2".
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `x`: the 32-bit value to examine.
///
/// ### Return value
/// Returns the index of the most significant bit, or -1 if the value is 0.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_MostSignificantBitIndex32(x: Uint32) -> ::core::ffi::c_int {
    31 - (x.leading_zeros() as ::core::ffi::c_int)
}

/// Determine if a unsigned 32-bit value has exactly one bit set.
///
/// If there are no bits set (`x` is zero), or more than one bit set, this
/// returns false. If any one bit is exclusively set, this returns true.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `x`: the 32-bit value to examine.
///
/// ### Return value
/// Returns true if exactly one bit is set in `x`, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub const fn SDL_HasExactlyOneBitSet32(x: Uint32) -> ::core::primitive::bool {
    if ((x != 0) && !((x & (x - 1_u32)) != 0)) {
        return true;
    }
    return false;
}

#[cfg(doc)]
use crate::everything::*;
