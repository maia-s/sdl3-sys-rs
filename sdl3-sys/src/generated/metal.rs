//! Functions to creating Metal layers and views on SDL windows.

use super::video::*;

/// A handle to a CAMetalLayer-backed NSView (macOS) or UIView (iOS/tvOS).
///
/// This datatype is available since SDL 3.0.0.
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
    /// - `window`: the window.
    /// - Returns handle NSView or UIView.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_Metal_DestroyView`]<br>
    /// See also [`SDL_Metal_GetLayer`]<br>
    pub fn SDL_Metal_CreateView(window: *mut SDL_Window) -> SDL_MetalView;
}

extern "C" {
    /// Destroy an existing [`SDL_MetalView`] object.
    ///
    /// This should be called before [`SDL_DestroyWindow`], if [`SDL_Metal_CreateView`] was
    /// called after [`SDL_CreateWindow`].
    ///
    /// - `view`: the [`SDL_MetalView`] object.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_Metal_CreateView`]<br>
    pub fn SDL_Metal_DestroyView(view: SDL_MetalView);
}

extern "C" {
    /// Get a pointer to the backing CAMetalLayer for the given view.
    ///
    /// - `view`: the [`SDL_MetalView`] object.
    /// - Returns a pointer.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_Metal_GetLayer(view: SDL_MetalView) -> *mut ::core::ffi::c_void;
}
