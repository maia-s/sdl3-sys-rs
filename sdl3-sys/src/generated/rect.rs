//! Some helper functions for managing rectangles and 2D points, in both
//! interger and floating point versions.

use super::stdinc::*;

use super::error::*;

/// The structure that defines a point (using integers).
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetRectEnclosingPoints
/// \sa SDL_PointInRect
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Point {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}

/// The structure that defines a point (using floating point values).
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetRectEnclosingPointsFloat
/// \sa SDL_PointInRectFloat
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FPoint {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}

/// A rectangle, with the origin at the upper left (using integers).
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_RectEmpty
/// \sa SDL_RectsEqual
/// \sa SDL_HasRectIntersection
/// \sa SDL_GetRectIntersection
/// \sa SDL_GetRectAndLineIntersection
/// \sa SDL_GetRectUnion
/// \sa SDL_GetRectEnclosingPoints
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
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_RectEmptyFloat
/// \sa SDL_RectsEqualFloat
/// \sa SDL_RectsEqualEpsilon
/// \sa SDL_HasRectIntersectionFloat
/// \sa SDL_GetRectIntersectionFloat
/// \sa SDL_GetRectAndLineIntersectionFloat
/// \sa SDL_GetRectUnionFloat
/// \sa SDL_GetRectEnclosingPointsFloat
/// \sa SDL_PointInRectFloat
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_FRect {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
    pub h: ::core::ffi::c_float,
}

// [sdl3-sys-gen] skipped inline function `SDL_RectToFRect`

// [sdl3-sys-gen] skipped inline function `SDL_PointInRect`

// [sdl3-sys-gen] skipped inline function `SDL_RectEmpty`

// [sdl3-sys-gen] skipped inline function `SDL_RectsEqual`

extern "C" {
    /// Determine whether two rectangles intersect.
    ///
    /// If either pointer is NULL the function will return SDL_FALSE.
    ///
    /// \param A an SDL_Rect structure representing the first rectangle.
    /// \param B an SDL_Rect structure representing the second rectangle.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRectIntersection
    pub fn SDL_HasRectIntersection(A: *const SDL_Rect, B: *const SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Calculate the intersection of two rectangles.
    ///
    /// If `result` is NULL then this function will return SDL_FALSE.
    ///
    /// \param A an SDL_Rect structure representing the first rectangle.
    /// \param B an SDL_Rect structure representing the second rectangle.
    /// \param result an SDL_Rect structure filled in with the intersection of
    ///               rectangles `A` and `B`.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasRectIntersection
    pub fn SDL_GetRectIntersection(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> SDL_bool;
}

extern "C" {
    /// Calculate the union of two rectangles.
    ///
    /// \param A an SDL_Rect structure representing the first rectangle.
    /// \param B an SDL_Rect structure representing the second rectangle.
    /// \param result an SDL_Rect structure filled in with the union of rectangles
    ///               `A` and `B`.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectUnion(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> SDL_bool;
}

extern "C" {
    /// Calculate a minimal rectangle enclosing a set of points.
    ///
    /// If `clip` is not NULL then only points inside of the clipping rectangle are
    /// considered.
    ///
    /// \param points an array of SDL_Point structures representing points to be
    ///               enclosed.
    /// \param count the number of structures in the `points` array.
    /// \param clip an SDL_Rect used for clipping or NULL to enclose all points.
    /// \param result an SDL_Rect structure filled in with the minimal enclosing
    ///               rectangle.
    /// \returns SDL_TRUE if any points were enclosed or SDL_FALSE if all the
    ///          points were outside of the clipping rectangle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectEnclosingPoints(
        points: *const SDL_Point,
        count: ::core::ffi::c_int,
        clip: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> SDL_bool;
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
    /// \param rect an SDL_Rect structure representing the rectangle to intersect.
    /// \param X1 a pointer to the starting X-coordinate of the line.
    /// \param Y1 a pointer to the starting Y-coordinate of the line.
    /// \param X2 a pointer to the ending X-coordinate of the line.
    /// \param Y2 a pointer to the ending Y-coordinate of the line.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectAndLineIntersection(
        rect: *const SDL_Rect,
        X1: *mut ::core::ffi::c_int,
        Y1: *mut ::core::ffi::c_int,
        X2: *mut ::core::ffi::c_int,
        Y2: *mut ::core::ffi::c_int,
    ) -> SDL_bool;
}

// [sdl3-sys-gen] skipped inline function `SDL_PointInRectFloat`

// [sdl3-sys-gen] skipped inline function `SDL_RectEmptyFloat`

// [sdl3-sys-gen] skipped inline function `SDL_RectsEqualEpsilon`

// [sdl3-sys-gen] skipped inline function `SDL_RectsEqualFloat`

extern "C" {
    /// Determine whether two rectangles intersect with float precision.
    ///
    /// If either pointer is NULL the function will return SDL_FALSE.
    ///
    /// \param A an SDL_FRect structure representing the first rectangle.
    /// \param B an SDL_FRect structure representing the second rectangle.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRectIntersection
    pub fn SDL_HasRectIntersectionFloat(A: *const SDL_FRect, B: *const SDL_FRect) -> SDL_bool;
}

extern "C" {
    /// Calculate the intersection of two rectangles with float precision.
    ///
    /// If `result` is NULL then this function will return SDL_FALSE.
    ///
    /// \param A an SDL_FRect structure representing the first rectangle.
    /// \param B an SDL_FRect structure representing the second rectangle.
    /// \param result an SDL_FRect structure filled in with the intersection of
    ///               rectangles `A` and `B`.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HasRectIntersectionFloat
    pub fn SDL_GetRectIntersectionFloat(
        A: *const SDL_FRect,
        B: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Calculate the union of two rectangles with float precision.
    ///
    /// \param A an SDL_FRect structure representing the first rectangle.
    /// \param B an SDL_FRect structure representing the second rectangle.
    /// \param result an SDL_FRect structure filled in with the union of rectangles
    ///               `A` and `B`.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectUnionFloat(
        A: *const SDL_FRect,
        B: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Calculate a minimal rectangle enclosing a set of points with float
    /// precision.
    ///
    /// If `clip` is not NULL then only points inside of the clipping rectangle are
    /// considered.
    ///
    /// \param points an array of SDL_FPoint structures representing points to be
    ///               enclosed.
    /// \param count the number of structures in the `points` array.
    /// \param clip an SDL_FRect used for clipping or NULL to enclose all points.
    /// \param result an SDL_FRect structure filled in with the minimal enclosing
    ///               rectangle.
    /// \returns SDL_TRUE if any points were enclosed or SDL_FALSE if all the
    ///          points were outside of the clipping rectangle.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectEnclosingPointsFloat(
        points: *const SDL_FPoint,
        count: ::core::ffi::c_int,
        clip: *const SDL_FRect,
        result: *mut SDL_FRect,
    ) -> SDL_bool;
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
    /// \param rect an SDL_FRect structure representing the rectangle to intersect.
    /// \param X1 a pointer to the starting X-coordinate of the line.
    /// \param Y1 a pointer to the starting Y-coordinate of the line.
    /// \param X2 a pointer to the ending X-coordinate of the line.
    /// \param Y2 a pointer to the ending Y-coordinate of the line.
    /// \returns SDL_TRUE if there is an intersection, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRectAndLineIntersectionFloat(
        rect: *const SDL_FRect,
        X1: *mut ::core::ffi::c_float,
        Y1: *mut ::core::ffi::c_float,
        X2: *mut ::core::ffi::c_float,
        Y2: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}
