#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryMetal
//!
//! Functions to creating Metal layers and views on SDL windows.

use super::video::*;

/// A handle to a CAMetalLayer-backed NSView (macOS) or UIView (iOS/tvOS).
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_MetalView = *mut ::core::ffi::c_void;

extern_sdlcall! {{
    /// Create a CAMetalLayer-backed NSView/UIView and attach it to the specified
    /// window.
    ///
    /// On macOS, this does *not* associate a MTLDevice with the CAMetalLayer on
    /// its own. It is up to user code to do that.
    ///
    /// The returned handle can be casted directly to a NSView or UIView. To access
    /// the backing CAMetalLayer, call SDL_Metal_GetLayer().
    ///
    /// \param window the window.
    /// \returns handle NSView or UIView.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Metal_DestroyView
    /// \sa SDL_Metal_GetLayer
    pub fn SDL_Metal_CreateView(window: *mut SDL_Window) -> SDL_MetalView;
}}

extern_sdlcall! {{
    /// Destroy an existing SDL_MetalView object.
    ///
    /// This should be called before SDL_DestroyWindow, if SDL_Metal_CreateView was
    /// called after SDL_CreateWindow.
    ///
    /// \param view the SDL_MetalView object.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Metal_CreateView
    pub fn SDL_Metal_DestroyView(view: SDL_MetalView);
}}

extern_sdlcall! {{
    /// Get a pointer to the backing CAMetalLayer for the given view.
    ///
    /// \param view the SDL_MetalView object.
    /// \returns a pointer.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_Metal_GetLayer(view: SDL_MetalView) -> *mut ::core::ffi::c_void;
}}

