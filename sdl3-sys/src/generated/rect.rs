//! Some helper functions for managing rectangles and 2D points, in both
//! integer and floating point versions.

use super::stdinc::*;

use super::error::*;

/// The structure that defines a point (using integers).
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetRectEnclosingPoints`]
/// - [`SDL_PointInRect`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Point {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}

/// The structure that defines a point (using floating point values).
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetRectEnclosingPointsFloat`]
/// - [`SDL_PointInRectFloat`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FPoint {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}

/// A rectangle, with the origin at the upper left (using integers).
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_RectEmpty`]
/// - [`SDL_RectsEqual`]
/// - [`SDL_HasRectIntersection`]
/// - [`SDL_GetRectIntersection`]
/// - [`SDL_GetRectAndLineIntersection`]
/// - [`SDL_GetRectUnion`]
/// - [`SDL_GetRectEnclosingPoints`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Rect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}

/// A rectangle, with the origin at the upper left (using floating point
/// values).
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_RectEmptyFloat`]
/// - [`SDL_RectsEqualFloat`]
/// - [`SDL_RectsEqualEpsilon`]
/// - [`SDL_HasRectIntersectionFloat`]
/// - [`SDL_GetRectIntersectionFloat`]
/// - [`SDL_GetRectAndLineIntersectionFloat`]
/// - [`SDL_GetRectUnionFloat`]
/// - [`SDL_GetRectEnclosingPointsFloat`]
/// - [`SDL_PointInRectFloat`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FRect {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
    pub h: ::core::ffi::c_float,
}

/// Convert an [`SDL_Rect`] to [`SDL_FRect`]
///
/// ### Parameters
/// - `rect`: a pointer to an [`SDL_Rect`].
/// - `frect`: a pointer filled in with the floating point representation of
///   `rect`.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_RectToFRect(rect: *const SDL_Rect, frect: *mut SDL_FRect) {
    {
        let (ptr, value) = (
            unsafe { ::core::ptr::addr_of_mut!((*frect).x) },
            (unsafe { ::core::ptr::addr_of!((*rect).x).read() } as ::core::ffi::c_float),
        );
        unsafe { ptr.write(value) };
        value
    };
    {
        let (ptr, value) = (
            unsafe { ::core::ptr::addr_of_mut!((*frect).y) },
            (unsafe { ::core::ptr::addr_of!((*rect).y).read() } as ::core::ffi::c_float),
        );
        unsafe { ptr.write(value) };
        value
    };
    {
        let (ptr, value) = (
            unsafe { ::core::ptr::addr_of_mut!((*frect).w) },
            (unsafe { ::core::ptr::addr_of!((*rect).w).read() } as ::core::ffi::c_float),
        );
        unsafe { ptr.write(value) };
        value
    };
    {
        let (ptr, value) = (
            unsafe { ::core::ptr::addr_of_mut!((*frect).h) },
            (unsafe { ::core::ptr::addr_of!((*rect).h).read() } as ::core::ffi::c_float),
        );
        unsafe { ptr.write(value) };
        value
    };
}

/// Determine whether a point resides inside a rectangle.
///
/// A point is considered part of a rectangle if both `p` and `r` are not NULL,
/// and `p`'s x and y coordinates are >= to the rectangle's top left corner,
/// and < the rectangle's x+w and y+h. So a 1x1 rectangle considers point (0,0)
/// as "inside" and (0,1) as not.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `p`: the point to test.
/// - `r`: the rectangle to test.
///
/// ### Return value
/// Returns true if `p` is contained by `r`, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_PointInRect(p: *const SDL_Point, r: *const SDL_Rect) -> ::core::primitive::bool {
    return if (((((!p.is_null() && !r.is_null())
        && (unsafe { ::core::ptr::addr_of!((*p).x).read() }
            >= unsafe { ::core::ptr::addr_of!((*r).x).read() }))
        && (unsafe { ::core::ptr::addr_of!((*p).x).read() }
            < (unsafe { ::core::ptr::addr_of!((*r).x).read() }
                + unsafe { ::core::ptr::addr_of!((*r).w).read() })))
        && (unsafe { ::core::ptr::addr_of!((*p).y).read() }
            >= unsafe { ::core::ptr::addr_of!((*r).y).read() }))
        && (unsafe { ::core::ptr::addr_of!((*p).y).read() }
            < (unsafe { ::core::ptr::addr_of!((*r).y).read() }
                + unsafe { ::core::ptr::addr_of!((*r).h).read() })))
    {
        true
    } else {
        false
    };
}

/// Determine whether a rectangle has no area.
///
/// A rectangle is considered "empty" for this function if `r` is NULL, or if
/// `r`'s width and/or height are <= 0.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `r`: the rectangle to test.
///
/// ### Return value
/// Returns true if the rectangle is "empty", false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_RectEmpty(r: *const SDL_Rect) -> ::core::primitive::bool {
    return if ((!(!r.is_null()) || (unsafe { ::core::ptr::addr_of!((*r).w).read() } <= 0_i32))
        || (unsafe { ::core::ptr::addr_of!((*r).h).read() } <= 0_i32))
    {
        true
    } else {
        false
    };
}

/// Determine whether two rectangles are equal.
///
/// Rectangles are considered equal if both are not NULL and each of their x,
/// y, width and height match.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `a`: the first rectangle to test.
/// - `b`: the second rectangle to test.
///
/// ### Return value
/// Returns true if the rectangles are equal, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_RectsEqual(a: *const SDL_Rect, b: *const SDL_Rect) -> ::core::primitive::bool {
    return if (((((!a.is_null() && !b.is_null())
        && (unsafe { ::core::ptr::addr_of!((*a).x).read() }
            == unsafe { ::core::ptr::addr_of!((*b).x).read() }))
        && (unsafe { ::core::ptr::addr_of!((*a).y).read() }
            == unsafe { ::core::ptr::addr_of!((*b).y).read() }))
        && (unsafe { ::core::ptr::addr_of!((*a).w).read() }
            == unsafe { ::core::ptr::addr_of!((*b).w).read() }))
        && (unsafe { ::core::ptr::addr_of!((*a).h).read() }
            == unsafe { ::core::ptr::addr_of!((*b).h).read() }))
    {
        true
    } else {
        false
    };
}

extern "C" {
    /// Determine whether two rectangles intersect.
    ///
    /// If either pointer is NULL the function will return false.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_Rect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_Rect`] structure representing the second rectangle.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetRectIntersection`]
    pub fn SDL_HasRectIntersection(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the intersection of two rectangles.
    ///
    /// If `result` is NULL then this function will return false.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_Rect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_Rect`] structure representing the second rectangle.
    /// - `result`: an [`SDL_Rect`] structure filled in with the intersection of
    ///   rectangles `A` and `B`.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasRectIntersection`]
    pub fn SDL_GetRectIntersection(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the union of two rectangles.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_Rect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_Rect`] structure representing the second rectangle.
    /// - `result`: an [`SDL_Rect`] structure filled in with the union of rectangles
    ///   `A` and `B`.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectUnion(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate a minimal rectangle enclosing a set of points.
    ///
    /// If `clip` is not NULL then only points inside of the clipping rectangle are
    /// considered.
    ///
    /// ### Parameters
    /// - `points`: an array of [`SDL_Point`] structures representing points to be
    ///   enclosed.
    /// - `count`: the number of structures in the `points` array.
    /// - `clip`: an [`SDL_Rect`] used for clipping or NULL to enclose all points.
    /// - `result`: an [`SDL_Rect`] structure filled in with the minimal enclosing
    ///   rectangle.
    ///
    /// ### Return value
    /// Returns true if any points were enclosed or false if all the points were
    ///   outside of the clipping rectangle.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectEnclosingPoints(
        points: *const SDL_Point,
        count: ::core::ffi::c_int,
        clip: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the intersection of a rectangle and line segment.
    ///
    /// This function is used to clip a line segment to a rectangle. A line segment
    /// contained entirely within the rectangle or that does not intersect will
    /// remain unchanged. A line segment that crosses the rectangle at either or
    /// both ends will be clipped to the boundary of the rectangle and the new
    /// coordinates saved in `X1`, `Y1`, `X2`, and/or `Y2` as necessary.
    ///
    /// ### Parameters
    /// - `rect`: an [`SDL_Rect`] structure representing the rectangle to intersect.
    /// - `X1`: a pointer to the starting X-coordinate of the line.
    /// - `Y1`: a pointer to the starting Y-coordinate of the line.
    /// - `X2`: a pointer to the ending X-coordinate of the line.
    /// - `Y2`: a pointer to the ending Y-coordinate of the line.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectAndLineIntersection(
        rect: *const SDL_Rect,
        X1: *mut ::core::ffi::c_int,
        Y1: *mut ::core::ffi::c_int,
        X2: *mut ::core::ffi::c_int,
        Y2: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

/// Determine whether a point resides inside a floating point rectangle.
///
/// A point is considered part of a rectangle if both `p` and `r` are not NULL,
/// and `p`'s x and y coordinates are >= to the rectangle's top left corner,
/// and <= the rectangle's x+w and y+h. So a 1x1 rectangle considers point
/// (0,0) and (0,1) as "inside" and (0,2) as not.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `p`: the point to test.
/// - `r`: the rectangle to test.
///
/// ### Return value
/// Returns true if `p` is contained by `r`, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_PointInRectFloat(
    p: *const SDL_FPoint,
    r: *const SDL_FRect,
) -> ::core::primitive::bool {
    return if (((((!p.is_null() && !r.is_null())
        && (unsafe { ::core::ptr::addr_of!((*p).x).read() }
            >= unsafe { ::core::ptr::addr_of!((*r).x).read() }))
        && (unsafe { ::core::ptr::addr_of!((*p).x).read() }
            <= (unsafe { ::core::ptr::addr_of!((*r).x).read() }
                + unsafe { ::core::ptr::addr_of!((*r).w).read() })))
        && (unsafe { ::core::ptr::addr_of!((*p).y).read() }
            >= unsafe { ::core::ptr::addr_of!((*r).y).read() }))
        && (unsafe { ::core::ptr::addr_of!((*p).y).read() }
            <= (unsafe { ::core::ptr::addr_of!((*r).y).read() }
                + unsafe { ::core::ptr::addr_of!((*r).h).read() })))
    {
        true
    } else {
        false
    };
}

/// Determine whether a floating point rectangle can contain any point.
///
/// A rectangle is considered "empty" for this function if `r` is NULL, or if
/// `r`'s width and/or height are < 0.0f.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `r`: the rectangle to test.
///
/// ### Return value
/// Returns true if the rectangle is "empty", false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
#[inline(always)]
pub unsafe fn SDL_RectEmptyFloat(r: *const SDL_FRect) -> ::core::primitive::bool {
    return if ((!(!r.is_null()) || (unsafe { ::core::ptr::addr_of!((*r).w).read() } < 0.0_f32))
        || (unsafe { ::core::ptr::addr_of!((*r).h).read() } < 0.0_f32))
    {
        true
    } else {
        false
    };
}

/// Determine whether two floating point rectangles are equal, within some
/// given epsilon.
///
/// Rectangles are considered equal if both are not NULL and each of their x,
/// y, width and height are within `epsilon` of each other. If you don't know
/// what value to use for `epsilon`, you should call the [`SDL_RectsEqualFloat`]
/// function instead.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `a`: the first rectangle to test.
/// - `b`: the second rectangle to test.
/// - `epsilon`: the epsilon value for comparison.
///
/// ### Return value
/// Returns true if the rectangles are equal, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_RectsEqualFloat`]
#[inline(always)]
pub unsafe fn SDL_RectsEqualEpsilon(
    a: *const SDL_FRect,
    b: *const SDL_FRect,
    epsilon: ::core::ffi::c_float,
) -> ::core::primitive::bool {
    return if ((!a.is_null() && !b.is_null())
        && ((a == b)
            || ((((unsafe {
                SDL_fabsf(
                    (unsafe { ::core::ptr::addr_of!((*a).x).read() }
                        - unsafe { ::core::ptr::addr_of!((*b).x).read() }),
                )
            } <= epsilon)
                && (unsafe {
                    SDL_fabsf(
                        (unsafe { ::core::ptr::addr_of!((*a).y).read() }
                            - unsafe { ::core::ptr::addr_of!((*b).y).read() }),
                    )
                } <= epsilon))
                && (unsafe {
                    SDL_fabsf(
                        (unsafe { ::core::ptr::addr_of!((*a).w).read() }
                            - unsafe { ::core::ptr::addr_of!((*b).w).read() }),
                    )
                } <= epsilon))
                && (unsafe {
                    SDL_fabsf(
                        (unsafe { ::core::ptr::addr_of!((*a).h).read() }
                            - unsafe { ::core::ptr::addr_of!((*b).h).read() }),
                    )
                } <= epsilon))))
    {
        true
    } else {
        false
    };
}

/// Determine whether two floating point rectangles are equal, within a default
/// epsilon.
///
/// Rectangles are considered equal if both are not NULL and each of their x,
/// y, width and height are within [`SDL_FLT_EPSILON`] of each other. This is often
/// a reasonable way to compare two floating point rectangles and deal with the
/// slight precision variations in floating point calculations that tend to pop
/// up.
///
/// Note that this is a forced-inline function in a header, and not a public
/// API function available in the SDL library (which is to say, the code is
/// embedded in the calling program and the linker and dynamic loader will not
/// be able to find this function inside SDL itself).
///
/// ### Parameters
/// - `a`: the first rectangle to test.
/// - `b`: the second rectangle to test.
///
/// ### Return value
/// Returns true if the rectangles are equal, false otherwise.
///
/// ### Thread safety
/// It is safe to call this function from any thread.
///
/// ### Availability
/// This function is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_RectsEqualEpsilon`]
#[inline(always)]
pub unsafe fn SDL_RectsEqualFloat(
    a: *const SDL_FRect,
    b: *const SDL_FRect,
) -> ::core::primitive::bool {
    return unsafe { SDL_RectsEqualEpsilon(a, b, SDL_FLT_EPSILON) };
}

extern "C" {
    /// Determine whether two rectangles intersect with float precision.
    ///
    /// If either pointer is NULL the function will return false.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_FRect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_FRect`] structure representing the second rectangle.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetRectIntersection`]
    pub fn SDL_HasRectIntersectionFloat(
        A: *const SDL_FRect,
        B: *const SDL_FRect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the intersection of two rectangles with float precision.
    ///
    /// If `result` is NULL then this function will return false.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_FRect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_FRect`] structure representing the second rectangle.
    /// - `result`: an [`SDL_FRect`] structure filled in with the intersection of
    ///   rectangles `A` and `B`.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasRectIntersectionFloat`]
    pub fn SDL_GetRectIntersectionFloat(
        A: *const SDL_FRect,
        B: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the union of two rectangles with float precision.
    ///
    /// ### Parameters
    /// - `A`: an [`SDL_FRect`] structure representing the first rectangle.
    /// - `B`: an [`SDL_FRect`] structure representing the second rectangle.
    /// - `result`: an [`SDL_FRect`] structure filled in with the union of rectangles
    ///   `A` and `B`.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectUnionFloat(
        A: *const SDL_FRect,
        B: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate a minimal rectangle enclosing a set of points with float
    /// precision.
    ///
    /// If `clip` is not NULL then only points inside of the clipping rectangle are
    /// considered.
    ///
    /// ### Parameters
    /// - `points`: an array of [`SDL_FPoint`] structures representing points to be
    ///   enclosed.
    /// - `count`: the number of structures in the `points` array.
    /// - `clip`: an [`SDL_FRect`] used for clipping or NULL to enclose all points.
    /// - `result`: an [`SDL_FRect`] structure filled in with the minimal enclosing
    ///   rectangle.
    ///
    /// ### Return value
    /// Returns true if any points were enclosed or false if all the points were
    ///   outside of the clipping rectangle.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectEnclosingPointsFloat(
        points: *const SDL_FPoint,
        count: ::core::ffi::c_int,
        clip: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the intersection of a rectangle and line segment with float
    /// precision.
    ///
    /// This function is used to clip a line segment to a rectangle. A line segment
    /// contained entirely within the rectangle or that does not intersect will
    /// remain unchanged. A line segment that crosses the rectangle at either or
    /// both ends will be clipped to the boundary of the rectangle and the new
    /// coordinates saved in `X1`, `Y1`, `X2`, and/or `Y2` as necessary.
    ///
    /// ### Parameters
    /// - `rect`: an [`SDL_FRect`] structure representing the rectangle to intersect.
    /// - `X1`: a pointer to the starting X-coordinate of the line.
    /// - `Y1`: a pointer to the starting Y-coordinate of the line.
    /// - `X2`: a pointer to the ending X-coordinate of the line.
    /// - `Y2`: a pointer to the ending Y-coordinate of the line.
    ///
    /// ### Return value
    /// Returns true if there is an intersection, false otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetRectAndLineIntersectionFloat(
        rect: *const SDL_FRect,
        X1: *mut ::core::ffi::c_float,
        Y1: *mut ::core::ffi::c_float,
        X2: *mut ::core::ffi::c_float,
        Y2: *mut ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

#[cfg(doc)]
use crate::everything::*;
