//! Functions to creating Metal layers and views on SDL windows.

use super::video::*;

/// A handle to a CAMetalLayer-backed NSView (macOS) or UIView (iOS/tvOS).
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_MetalView = *mut ::core::ffi::c_void;

extern "C" {
    /// Create a CAMetalLayer-backed NSView/UIView and attach it to the specified
    /// window.
    ///
    /// On macOS, this does *not* associate a MTLDevice with the CAMetalLayer on
    /// its own. It is up to user code to do that.
    ///
    /// The returned handle can be casted directly to a NSView or UIView. To access
    /// the backing CAMetalLayer, call [`SDL_Metal_GetLayer()`].
    ///
    /// ### Parameters
    /// - `window`: the window.
    ///
    /// ### Return value
    /// Returns handle NSView or UIView.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_Metal_DestroyView`]
    /// - [`SDL_Metal_GetLayer`]
    pub fn SDL_Metal_CreateView(window: *mut SDL_Window) -> SDL_MetalView;
}

extern "C" {
    /// Destroy an existing [`SDL_MetalView`] object.
    ///
    /// This should be called before [`SDL_DestroyWindow`], if [`SDL_Metal_CreateView`] was
    /// called after [`SDL_CreateWindow`].
    ///
    /// ### Parameters
    /// - `view`: the [`SDL_MetalView`] object.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_Metal_CreateView`]
    pub fn SDL_Metal_DestroyView(view: SDL_MetalView);
}

extern "C" {
    /// Get a pointer to the backing CAMetalLayer for the given view.
    ///
    /// ### Parameters
    /// - `view`: the [`SDL_MetalView`] object.
    ///
    /// ### Return value
    /// Returns a pointer.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_Metal_GetLayer(view: SDL_MetalView) -> *mut ::core::ffi::c_void;
}

#[cfg(doc)]
use crate::everything::*;
