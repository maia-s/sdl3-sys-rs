//! Header file for SDL 2D rendering functions.
//!
//! This API supports the following features:
//!
//! - single pixel points
//! - single pixel lines
//! - filled rectangles
//! - texture images
//! - 2D polygons
//!
//! The primitives may be drawn in opaque, blended, or additive modes.
//!
//! The texture images may be drawn in opaque, blended, or additive modes. They
//! can have an additional color tint or alpha modulation applied to them, and
//! may also be stretched with linear interpolation.
//!
//! This API is designed to accelerate simple 2D operations. You may want more
//! functionality such as polygons and particle effects and in that case you
//! should use SDL's OpenGL/Direct3D support or one of the many good 3D
//! engines.
//!
//! These functions must be called from the main thread. See this bug for
//! details: https://github.com/libsdl-org/SDL/issues/986

use super::stdinc::*;

use super::blendmode::*;

use super::error::*;

use super::events::*;

use super::pixels::*;

use super::properties::*;

use super::rect::*;

use super::surface::*;

use super::video::*;

/// The name of the software renderer.
///
/// \since This macro is available since SDL 3.0.0.
pub const SDL_SOFTWARE_RENDERER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"software\0") };

/// Vertex structure.
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Vertex {
    /// Vertex position, in SDL_Renderer coordinates
    pub position: SDL_FPoint,
    /// Vertex color
    pub color: SDL_FColor,
    /// Normalized texture coordinates, if needed
    pub tex_coord: SDL_FPoint,
}

/// The access pattern allowed for a texture.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_TEXTUREACCESS_STATIC`], [`SDL_TEXTUREACCESS_STREAMING`], [`SDL_TEXTUREACCESS_TARGET`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_TextureAccess(pub ::core::ffi::c_int);
impl SDL_TextureAccess {
    /// Changes rarely, not lockable
    pub const STATIC: Self = Self(0);
    /// Changes frequently, lockable
    pub const STREAMING: Self = Self(1);
    /// Texture can be used as a render target
    pub const TARGET: Self = Self(2);
}
/// Changes rarely, not lockable
pub const SDL_TEXTUREACCESS_STATIC: SDL_TextureAccess = SDL_TextureAccess::STATIC;
/// Changes frequently, lockable
pub const SDL_TEXTUREACCESS_STREAMING: SDL_TextureAccess = SDL_TextureAccess::STREAMING;
/// Texture can be used as a render target
pub const SDL_TEXTUREACCESS_TARGET: SDL_TextureAccess = SDL_TextureAccess::TARGET;

/// How the logical size is mapped to the output.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_LOGICAL_PRESENTATION_DISABLED`], [`SDL_LOGICAL_PRESENTATION_STRETCH`], [`SDL_LOGICAL_PRESENTATION_LETTERBOX`], [`SDL_LOGICAL_PRESENTATION_OVERSCAN`], [`SDL_LOGICAL_PRESENTATION_INTEGER_SCALE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_RendererLogicalPresentation(pub ::core::ffi::c_int);
impl SDL_RendererLogicalPresentation {
    /// There is no logical size in effect
    pub const DISABLED: Self = Self(0);
    /// The rendered content is stretched to the output resolution
    pub const STRETCH: Self = Self(1);
    /// The rendered content is fit to the largest dimension and the other dimension is letterboxed with black bars
    pub const LETTERBOX: Self = Self(2);
    /// The rendered content is fit to the smallest dimension and the other dimension extends beyond the output bounds
    pub const OVERSCAN: Self = Self(3);
    /// The rendered content is scaled up by integer multiples to fit the output resolution
    pub const INTEGER_SCALE: Self = Self(4);
}
/// There is no logical size in effect
pub const SDL_LOGICAL_PRESENTATION_DISABLED: SDL_RendererLogicalPresentation =
    SDL_RendererLogicalPresentation::DISABLED;
/// The rendered content is stretched to the output resolution
pub const SDL_LOGICAL_PRESENTATION_STRETCH: SDL_RendererLogicalPresentation =
    SDL_RendererLogicalPresentation::STRETCH;
/// The rendered content is fit to the largest dimension and the other dimension is letterboxed with black bars
pub const SDL_LOGICAL_PRESENTATION_LETTERBOX: SDL_RendererLogicalPresentation =
    SDL_RendererLogicalPresentation::LETTERBOX;
/// The rendered content is fit to the smallest dimension and the other dimension extends beyond the output bounds
pub const SDL_LOGICAL_PRESENTATION_OVERSCAN: SDL_RendererLogicalPresentation =
    SDL_RendererLogicalPresentation::OVERSCAN;
/// The rendered content is scaled up by integer multiples to fit the output resolution
pub const SDL_LOGICAL_PRESENTATION_INTEGER_SCALE: SDL_RendererLogicalPresentation =
    SDL_RendererLogicalPresentation::INTEGER_SCALE;

extern "C" {
    /// Get the number of 2D rendering drivers available for the current display.
    ///
    /// A render driver is a set of code that handles rendering and texture
    /// management on a particular display. Normally there is only one, but some
    /// drivers may have several available with different capabilities.
    ///
    /// There may be none if SDL was compiled without render support.
    ///
    /// \returns the number of built in render drivers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRenderer
    /// \sa SDL_GetRenderDriver
    pub fn SDL_GetNumRenderDrivers() -> ::core::ffi::c_int;
}

extern "C" {
    /// Use this function to get the name of a built in 2D rendering driver.
    ///
    /// The list of rendering drivers is given in the order that they are normally
    /// initialized by default; the drivers that seem more reasonable to choose
    /// first (as far as the SDL developers believe) are earlier in the list.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "opengl",
    /// "direct3d12" or "metal". These never have Unicode characters, and are not
    /// meant to be proper names.
    ///
    /// \param index the index of the rendering driver; the value ranges from 0 to
    ///              SDL_GetNumRenderDrivers() - 1.
    /// \returns the name of the rendering driver at the requested index, or NULL
    ///          if an invalid index was specified.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetNumRenderDrivers
    pub fn SDL_GetRenderDriver(index: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Create a window and default renderer.
    ///
    /// \param title the title of the window, in UTF-8 encoding.
    /// \param width the width of the window.
    /// \param height the height of the window.
    /// \param window_flags the flags used to create the window (see
    ///                     SDL_CreateWindow()).
    /// \param window a pointer filled with the window, or NULL on error.
    /// \param renderer a pointer filled with the renderer, or NULL on error.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRenderer
    /// \sa SDL_CreateWindow
    pub fn SDL_CreateWindowAndRenderer(
        title: *const ::core::ffi::c_char,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        window_flags: SDL_WindowFlags,
        window: *mut *mut SDL_Window,
        renderer: *mut *mut SDL_Renderer,
    ) -> SDL_bool;
}

extern "C" {
    /// Create a 2D rendering context for a window.
    ///
    /// If you want a specific renderer, you can specify its name here. A list of
    /// available renderers can be obtained by calling SDL_GetRenderDriver multiple
    /// times, with indices from 0 to SDL_GetNumRenderDrivers()-1. If you don't
    /// need a specific renderer, specify NULL and SDL will attempt to choose the
    /// best option for you, based on what is available on the user's system.
    ///
    /// By default the rendering size matches the window size in pixels, but you
    /// can call SDL_SetRenderLogicalPresentation() to change the content size and
    /// scaling options.
    ///
    /// \param window the window where rendering is displayed.
    /// \param name the name of the rendering driver to initialize, or NULL to
    ///             initialize the first one supporting the requested flags.
    /// \returns a valid rendering context or NULL if there was an error; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRendererWithProperties
    /// \sa SDL_CreateSoftwareRenderer
    /// \sa SDL_DestroyRenderer
    /// \sa SDL_GetNumRenderDrivers
    /// \sa SDL_GetRenderDriver
    /// \sa SDL_GetRendererName
    pub fn SDL_CreateRenderer(
        window: *mut SDL_Window,
        name: *const ::core::ffi::c_char,
    ) -> *mut SDL_Renderer;
}

extern "C" {
    /// Create a 2D rendering context for a window, with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_RENDERER_CREATE_NAME_STRING`: the name of the rendering driver
    ///   to use, if a specific one is desired
    /// - `SDL_PROP_RENDERER_CREATE_WINDOW_POINTER`: the window where rendering is
    ///   displayed, required if this isn't a software renderer using a surface
    /// - `SDL_PROP_RENDERER_CREATE_SURFACE_POINTER`: the surface where rendering
    ///   is displayed, if you want a software renderer without a window
    /// - `SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER`: an SDL_ColorSpace
    ///   value describing the colorspace for output to the display, defaults to
    ///   SDL_COLORSPACE_SRGB. The direct3d11, direct3d12, and metal renderers
    ///   support SDL_COLORSPACE_SRGB_LINEAR, which is a linear color space and
    ///   supports HDR output. If you select SDL_COLORSPACE_SRGB_LINEAR, drawing
    ///   still uses the sRGB colorspace, but values can go beyond 1.0 and float
    ///   (linear) format textures can be used for HDR content.
    /// - `SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER`: non-zero if you want
    ///   present synchronized with the refresh rate. This property can take any
    ///   value that is supported by SDL_SetRenderVSync() for the renderer.
    ///
    /// With the vulkan renderer:
    ///
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER`: the VkInstance to use
    ///   with the renderer, optional.
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER`: the VkSurfaceKHR to use
    ///   with the renderer, optional.
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER`: the
    ///   VkPhysicalDevice to use with the renderer, optional.
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER`: the VkDevice to use
    ///   with the renderer, optional.
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER`: the
    ///   queue family index used for rendering.
    /// - `SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER`: the
    ///   queue family index used for presentation.
    ///
    /// \param props the properties to use.
    /// \returns a valid rendering context or NULL if there was an error; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProperties
    /// \sa SDL_CreateRenderer
    /// \sa SDL_CreateSoftwareRenderer
    /// \sa SDL_DestroyRenderer
    /// \sa SDL_GetRendererName
    pub fn SDL_CreateRendererWithProperties(props: SDL_PropertiesID) -> *mut SDL_Renderer;
}

pub const SDL_PROP_RENDERER_CREATE_NAME_STRING: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.name\0") };

pub const SDL_PROP_RENDERER_CREATE_WINDOW_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.window\0") };

pub const SDL_PROP_RENDERER_CREATE_SURFACE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.surface\0") };

pub const SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.output_colorspace\0")
};

pub const SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.present_vsync\0")
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.vulkan.instance\0")
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.vulkan.surface\0")
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(
        b"SDL.renderer.create.vulkan.physical_device\0",
    )
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.create.vulkan.device\0")
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(
        b"SDL.renderer.create.vulkan.graphics_queue_family_index\0",
    )
};

pub const SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(
        b"SDL.renderer.create.vulkan.present_queue_family_index\0",
    )
};

extern "C" {
    /// Create a 2D software rendering context for a surface.
    ///
    /// Two other API which can be used to create SDL_Renderer:
    /// SDL_CreateRenderer() and SDL_CreateWindowAndRenderer(). These can _also_
    /// create a software renderer, but they are intended to be used with an
    /// SDL_Window as the final destination and not an SDL_Surface.
    ///
    /// \param surface the SDL_Surface structure representing the surface where
    ///                rendering is done.
    /// \returns a valid rendering context or NULL if there was an error; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroyRenderer
    pub fn SDL_CreateSoftwareRenderer(surface: *mut SDL_Surface) -> *mut SDL_Renderer;
}

extern "C" {
    /// Get the renderer associated with a window.
    ///
    /// \param window the window to query.
    /// \returns the rendering context on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRenderer(window: *mut SDL_Window) -> *mut SDL_Renderer;
}

extern "C" {
    /// Get the window associated with a renderer.
    ///
    /// \param renderer the renderer to query.
    /// \returns the window on success or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRenderWindow(renderer: *mut SDL_Renderer) -> *mut SDL_Window;
}

extern "C" {
    /// Get the name of a renderer.
    ///
    /// \param renderer the rendering context.
    /// \returns the name of the selected renderer, or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRenderer
    /// \sa SDL_CreateRendererWithProperties
    pub fn SDL_GetRendererName(renderer: *mut SDL_Renderer) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the properties associated with a renderer.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_RENDERER_NAME_STRING`: the name of the rendering driver
    /// - `SDL_PROP_RENDERER_WINDOW_POINTER`: the window where rendering is
    ///   displayed, if any
    /// - `SDL_PROP_RENDERER_SURFACE_POINTER`: the surface where rendering is
    ///   displayed, if this is a software renderer without a window
    /// - `SDL_PROP_RENDERER_VSYNC_NUMBER`: the current vsync setting
    /// - `SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER`: the maximum texture width
    ///   and height
    /// - `SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER`: a (const SDL_PixelFormat *)
    ///   array of pixel formats, terminated with SDL_PIXELFORMAT_UNKNOWN,
    ///   representing the available texture formats for this renderer.
    /// - `SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER`: an SDL_ColorSpace value
    ///   describing the colorspace for output to the display, defaults to
    ///   SDL_COLORSPACE_SRGB.
    /// - `SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN`: true if the output colorspace is
    ///   SDL_COLORSPACE_SRGB_LINEAR and the renderer is showing on a display with
    ///   HDR enabled. This property can change dynamically when
    ///   SDL_EVENT_DISPLAY_HDR_STATE_CHANGED is sent.
    /// - `SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT`: the value of SDR white in the
    ///   SDL_COLORSPACE_SRGB_LINEAR colorspace. When HDR is enabled, this value is
    ///   automatically multiplied into the color scale. This property can change
    ///   dynamically when SDL_EVENT_DISPLAY_HDR_STATE_CHANGED is sent.
    /// - `SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT`: the additional high dynamic range
    ///   that can be displayed, in terms of the SDR white point. When HDR is not
    ///   enabled, this will be 1.0. This property can change dynamically when
    ///   SDL_EVENT_DISPLAY_HDR_STATE_CHANGED is sent.
    ///
    /// With the direct3d renderer:
    ///
    /// - `SDL_PROP_RENDERER_D3D9_DEVICE_POINTER`: the IDirect3DDevice9 associated
    ///   with the renderer
    ///
    /// With the direct3d11 renderer:
    ///
    /// - `SDL_PROP_RENDERER_D3D11_DEVICE_POINTER`: the ID3D11Device associated
    ///   with the renderer
    /// - `SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER`: the IDXGISwapChain1
    ///   associated with the renderer. This may change when the window is resized.
    ///
    /// With the direct3d12 renderer:
    ///
    /// - `SDL_PROP_RENDERER_D3D12_DEVICE_POINTER`: the ID3D12Device associated
    ///   with the renderer
    /// - `SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER`: the IDXGISwapChain4
    ///   associated with the renderer.
    /// - `SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER`: the ID3D12CommandQueue
    ///   associated with the renderer
    ///
    /// With the vulkan renderer:
    ///
    /// - `SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER`: the VkInstance associated
    ///   with the renderer
    /// - `SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER`: the VkSurfaceKHR associated
    ///   with the renderer
    /// - `SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER`: the VkPhysicalDevice
    ///   associated with the renderer
    /// - `SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER`: the VkDevice associated with
    ///   the renderer
    /// - `SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER`: the queue
    ///   family index used for rendering
    /// - `SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER`: the queue
    ///   family index used for presentation
    /// - `SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER`: the number of
    ///   swapchain images, or potential frames in flight, used by the Vulkan
    ///   renderer
    ///
    /// \param renderer the rendering context.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRendererProperties(renderer: *mut SDL_Renderer) -> SDL_PropertiesID;
}

pub const SDL_PROP_RENDERER_NAME_STRING: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.name\0") };

pub const SDL_PROP_RENDERER_WINDOW_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.window\0") };

pub const SDL_PROP_RENDERER_SURFACE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.surface\0") };

pub const SDL_PROP_RENDERER_VSYNC_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vsync\0") };

pub const SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.max_texture_size\0") };

pub const SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.texture_formats\0") };

pub const SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.output_colorspace\0")
};

pub const SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.HDR_enabled\0") };

pub const SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.SDR_white_point\0") };

pub const SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.HDR_headroom\0") };

pub const SDL_PROP_RENDERER_D3D9_DEVICE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d9.device\0") };

pub const SDL_PROP_RENDERER_D3D11_DEVICE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d11.device\0") };

pub const SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d11.swap_chain\0") };

pub const SDL_PROP_RENDERER_D3D12_DEVICE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d12.device\0") };

pub const SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d12.swap_chain\0") };

pub const SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.d3d12.command_queue\0")
};

pub const SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vulkan.instance\0") };

pub const SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vulkan.surface\0") };

pub const SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vulkan.physical_device\0")
};

pub const SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vulkan.device\0") };

pub const SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(
        b"SDL.renderer.vulkan.graphics_queue_family_index\0",
    )
};

pub const SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(
        b"SDL.renderer.vulkan.present_queue_family_index\0",
    )
};

pub const SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.renderer.vulkan.swapchain_image_count\0")
};

extern "C" {
    /// Get the output size in pixels of a rendering context.
    ///
    /// This returns the true output size in pixels, ignoring any render targets or
    /// logical size and presentation.
    ///
    /// \param renderer the rendering context.
    /// \param w a pointer filled in with the width in pixels.
    /// \param h a pointer filled in with the height in pixels.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetCurrentRenderOutputSize
    pub fn SDL_GetRenderOutputSize(
        renderer: *mut SDL_Renderer,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the current output size in pixels of a rendering context.
    ///
    /// If a rendering target is active, this will return the size of the rendering
    /// target in pixels, otherwise if a logical size is set, it will return the
    /// logical size, otherwise it will return the value of
    /// SDL_GetRenderOutputSize().
    ///
    /// \param renderer the rendering context.
    /// \param w a pointer filled in with the current width.
    /// \param h a pointer filled in with the current height.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderOutputSize
    pub fn SDL_GetCurrentRenderOutputSize(
        renderer: *mut SDL_Renderer,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Create a texture for a rendering context.
    ///
    /// The contents of a texture when first created are not defined.
    ///
    /// \param renderer the rendering context.
    /// \param format one of the enumerated values in SDL_PixelFormat.
    /// \param access one of the enumerated values in SDL_TextureAccess.
    /// \param w the width of the texture in pixels.
    /// \param h the height of the texture in pixels.
    /// \returns a pointer to the created texture or NULL if no rendering context
    ///          was active, the format was unsupported, or the width or height
    ///          were out of range; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateTextureFromSurface
    /// \sa SDL_CreateTextureWithProperties
    /// \sa SDL_DestroyTexture
    /// \sa SDL_GetTextureSize
    /// \sa SDL_UpdateTexture
    pub fn SDL_CreateTexture(
        renderer: *mut SDL_Renderer,
        format: SDL_PixelFormat,
        access: SDL_TextureAccess,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> *mut SDL_Texture;
}

extern "C" {
    /// Create a texture from an existing surface.
    ///
    /// The surface is not modified or freed by this function.
    ///
    /// The SDL_TextureAccess hint for the created texture is
    /// `SDL_TEXTUREACCESS_STATIC`.
    ///
    /// The pixel format of the created texture may be different from the pixel
    /// format of the surface, and can be queried using the
    /// SDL_PROP_TEXTURE_FORMAT_NUMBER property.
    ///
    /// \param renderer the rendering context.
    /// \param surface the SDL_Surface structure containing pixel data used to fill
    ///                the texture.
    /// \returns the created texture or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateTexture
    /// \sa SDL_CreateTextureWithProperties
    /// \sa SDL_DestroyTexture
    pub fn SDL_CreateTextureFromSurface(
        renderer: *mut SDL_Renderer,
        surface: *mut SDL_Surface,
    ) -> *mut SDL_Texture;
}

extern "C" {
    /// Create a texture for a rendering context with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER`: an SDL_ColorSpace value
    ///   describing the texture colorspace, defaults to SDL_COLORSPACE_SRGB_LINEAR
    ///   for floating point textures, SDL_COLORSPACE_HDR10 for 10-bit textures,
    ///   SDL_COLORSPACE_SRGB for other RGB textures and SDL_COLORSPACE_JPEG for
    ///   YUV textures.
    /// - `SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER`: one of the enumerated values in
    ///   SDL_PixelFormat, defaults to the best RGBA format for the renderer
    /// - `SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER`: one of the enumerated values in
    ///   SDL_TextureAccess, defaults to SDL_TEXTUREACCESS_STATIC
    /// - `SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER`: the width of the texture in
    ///   pixels, required
    /// - `SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER`: the height of the texture in
    ///   pixels, required
    /// - `SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT`: for HDR10 and floating
    ///   point textures, this defines the value of 100% diffuse white, with higher
    ///   values being displayed in the High Dynamic Range headroom. This defaults
    ///   to 100 for HDR10 textures and 1.0 for floating point textures.
    /// - `SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT`: for HDR10 and floating
    ///   point textures, this defines the maximum dynamic range used by the
    ///   content, in terms of the SDR white point. This would be equivalent to
    ///   maxCLL / SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT for HDR10 content.
    ///   If this is defined, any values outside the range supported by the display
    ///   will be scaled into the available HDR headroom, otherwise they are
    ///   clipped.
    ///
    /// With the direct3d11 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER`: the ID3D11Texture2D
    ///   associated with the texture, if you want to wrap an existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER`: the ID3D11Texture2D
    ///   associated with the U plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER`: the ID3D11Texture2D
    ///   associated with the V plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    ///
    /// With the direct3d12 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER`: the ID3D12Resource
    ///   associated with the texture, if you want to wrap an existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER`: the ID3D12Resource
    ///   associated with the U plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER`: the ID3D12Resource
    ///   associated with the V plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    ///
    /// With the metal renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER`: the CVPixelBufferRef
    ///   associated with the texture, if you want to create a texture from an
    ///   existing pixel buffer.
    ///
    /// With the opengl renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER`: the GLuint texture
    ///   associated with the texture, if you want to wrap an existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER`: the GLuint texture
    ///   associated with the UV plane of an NV12 texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER`: the GLuint texture
    ///   associated with the U plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER`: the GLuint texture
    ///   associated with the V plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    ///
    /// With the opengles2 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER`: the GLuint texture
    ///   associated with the texture, if you want to wrap an existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER`: the GLuint texture
    ///   associated with the texture, if you want to wrap an existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER`: the GLuint texture
    ///   associated with the UV plane of an NV12 texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER`: the GLuint texture
    ///   associated with the U plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    /// - `SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER`: the GLuint texture
    ///   associated with the V plane of a YUV texture, if you want to wrap an
    ///   existing texture.
    ///
    /// With the vulkan renderer:
    ///
    /// - `SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER`: the VkImage with layout
    ///   VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL associated with the texture, if
    ///   you want to wrap an existing texture.
    ///
    /// \param renderer the rendering context.
    /// \param props the properties to use.
    /// \returns a pointer to the created texture or NULL if no rendering context
    ///          was active, the format was unsupported, or the width or height
    ///          were out of range; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProperties
    /// \sa SDL_CreateTexture
    /// \sa SDL_CreateTextureFromSurface
    /// \sa SDL_DestroyTexture
    /// \sa SDL_GetTextureSize
    /// \sa SDL_UpdateTexture
    pub fn SDL_CreateTextureWithProperties(
        renderer: *mut SDL_Renderer,
        props: SDL_PropertiesID,
    ) -> *mut SDL_Texture;
}

pub const SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.colorspace\0") };

pub const SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.format\0") };

pub const SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.access\0") };

pub const SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.width\0") };

pub const SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.height\0") };

pub const SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.SDR_white_point\0")
};

pub const SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.HDR_headroom\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d11.texture\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d11.texture_u\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d11.texture_v\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d12.texture\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d12.texture_u\0")
};

pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.d3d12.texture_v\0")
};

pub const SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.metal.pixelbuffer\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengl.texture\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengl.texture_uv\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengl.texture_u\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengl.texture_v\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengles2.texture\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengles2.texture_uv\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengles2.texture_u\0")
};

pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.opengles2.texture_v\0")
};

pub const SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.create.vulkan.texture\0")
};

extern "C" {
    /// Get the properties associated with a texture.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_TEXTURE_COLORSPACE_NUMBER`: an SDL_ColorSpace value describing
    ///   the texture colorspace.
    /// - `SDL_PROP_TEXTURE_FORMAT_NUMBER`: one of the enumerated values in
    ///   SDL_PixelFormat.
    /// - `SDL_PROP_TEXTURE_ACCESS_NUMBER`: one of the enumerated values in
    ///   SDL_TextureAccess.
    /// - `SDL_PROP_TEXTURE_WIDTH_NUMBER`: the width of the texture in pixels.
    /// - `SDL_PROP_TEXTURE_HEIGHT_NUMBER`: the height of the texture in pixels.
    /// - `SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT`: for HDR10 and floating point
    ///   textures, this defines the value of 100% diffuse white, with higher
    ///   values being displayed in the High Dynamic Range headroom. This defaults
    ///   to 100 for HDR10 textures and 1.0 for other textures.
    /// - `SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT`: for HDR10 and floating point
    ///   textures, this defines the maximum dynamic range used by the content, in
    ///   terms of the SDR white point. If this is defined, any values outside the
    ///   range supported by the display will be scaled into the available HDR
    ///   headroom, otherwise they are clipped. This defaults to 1.0 for SDR
    ///   textures, 4.0 for HDR10 textures, and no default for floating point
    ///   textures.
    ///
    /// With the direct3d11 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER`: the ID3D11Texture2D associated
    ///   with the texture
    /// - `SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER`: the ID3D11Texture2D
    ///   associated with the U plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER`: the ID3D11Texture2D
    ///   associated with the V plane of a YUV texture
    ///
    /// With the direct3d12 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER`: the ID3D12Resource associated
    ///   with the texture
    /// - `SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER`: the ID3D12Resource associated
    ///   with the U plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER`: the ID3D12Resource associated
    ///   with the V plane of a YUV texture
    ///
    /// With the vulkan renderer:
    ///
    /// - `SDL_PROP_TEXTURE_VULKAN_TEXTURE_POINTER`: the VkImage associated with
    ///   the texture
    /// - `SDL_PROP_TEXTURE_VULKAN_TEXTURE_U_POINTER`: the VkImage associated with
    ///   the U plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_VULKAN_TEXTURE_V_POINTER`: the VkImage associated with
    ///   the V plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_VULKAN_TEXTURE_UV_POINTER`: the VkImage associated with
    ///   the UV plane of a NV12/NV21 texture
    ///
    /// With the opengl renderer:
    ///
    /// - `SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER`: the GLuint texture associated
    ///   with the texture
    /// - `SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER`: the GLuint texture
    ///   associated with the UV plane of an NV12 texture
    /// - `SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER`: the GLuint texture associated
    ///   with the U plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER`: the GLuint texture associated
    ///   with the V plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER`: the GLenum for the
    ///   texture target (`GL_TEXTURE_2D`, `GL_TEXTURE_RECTANGLE_ARB`, etc)
    /// - `SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT`: the texture coordinate width of
    ///   the texture (0.0 - 1.0)
    /// - `SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT`: the texture coordinate height of
    ///   the texture (0.0 - 1.0)
    ///
    /// With the opengles2 renderer:
    ///
    /// - `SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER`: the GLuint texture
    ///   associated with the texture
    /// - `SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER`: the GLuint texture
    ///   associated with the UV plane of an NV12 texture
    /// - `SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER`: the GLuint texture
    ///   associated with the U plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER`: the GLuint texture
    ///   associated with the V plane of a YUV texture
    /// - `SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER`: the GLenum for the
    ///   texture target (`GL_TEXTURE_2D`, `GL_TEXTURE_EXTERNAL_OES`, etc)
    ///
    /// With the vulkan renderer:
    ///
    /// - `SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER`: the VkImage associated with the
    ///   texture
    ///
    /// \param texture the texture to query.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTextureProperties(texture: *mut SDL_Texture) -> SDL_PropertiesID;
}

pub const SDL_PROP_TEXTURE_COLORSPACE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.colorspace\0") };

pub const SDL_PROP_TEXTURE_FORMAT_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.format\0") };

pub const SDL_PROP_TEXTURE_ACCESS_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.access\0") };

pub const SDL_PROP_TEXTURE_WIDTH_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.width\0") };

pub const SDL_PROP_TEXTURE_HEIGHT_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.height\0") };

pub const SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.SDR_white_point\0") };

pub const SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.HDR_headroom\0") };

pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d11.texture\0") };

pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d11.texture_u\0") };

pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d11.texture_v\0") };

pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d12.texture\0") };

pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d12.texture_u\0") };

pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.d3d12.texture_v\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.texture\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.texture_uv\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.texture_u\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.texture_v\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.target\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.tex_w\0") };

pub const SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengl.tex_h\0") };

pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengles2.texture\0") };

pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengles2.texture_uv\0")
};

pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengles2.texture_u\0")
};

pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengles2.texture_v\0")
};

pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.opengles2.target\0") };

pub const SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.texture.vulkan.texture\0") };

extern "C" {
    /// Get the renderer that created an SDL_Texture.
    ///
    /// \param texture the texture to query.
    /// \returns a pointer to the SDL_Renderer that created the texture, or NULL on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRendererFromTexture(texture: *mut SDL_Texture) -> *mut SDL_Renderer;
}

extern "C" {
    /// Get the size of a texture, as floating point values.
    ///
    /// \param texture the texture to query.
    /// \param w a pointer filled in with the width of the texture in pixels. This
    ///          argument can be NULL if you don't need this information.
    /// \param h a pointer filled in with the height of the texture in pixels. This
    ///          argument can be NULL if you don't need this information.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetTextureSize(
        texture: *mut SDL_Texture,
        w: *mut ::core::ffi::c_float,
        h: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set an additional color value multiplied into render copy operations.
    ///
    /// When this texture is rendered, during the copy operation each source color
    /// channel is modulated by the appropriate color value according to the
    /// following formula:
    ///
    /// `srcC = srcC * (color / 255)`
    ///
    /// Color modulation is not always supported by the renderer; it will return
    /// SDL_FALSE if color modulation is not supported.
    ///
    /// \param texture the texture to update.
    /// \param r the red color value multiplied into copy operations.
    /// \param g the green color value multiplied into copy operations.
    /// \param b the blue color value multiplied into copy operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureColorMod
    /// \sa SDL_SetTextureAlphaMod
    /// \sa SDL_SetTextureColorModFloat
    pub fn SDL_SetTextureColorMod(
        texture: *mut SDL_Texture,
        r: Uint8,
        g: Uint8,
        b: Uint8,
    ) -> SDL_bool;
}

extern "C" {
    /// Set an additional color value multiplied into render copy operations.
    ///
    /// When this texture is rendered, during the copy operation each source color
    /// channel is modulated by the appropriate color value according to the
    /// following formula:
    ///
    /// `srcC = srcC * color`
    ///
    /// Color modulation is not always supported by the renderer; it will return
    /// SDL_FALSE if color modulation is not supported.
    ///
    /// \param texture the texture to update.
    /// \param r the red color value multiplied into copy operations.
    /// \param g the green color value multiplied into copy operations.
    /// \param b the blue color value multiplied into copy operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureColorModFloat
    /// \sa SDL_SetTextureAlphaModFloat
    /// \sa SDL_SetTextureColorMod
    pub fn SDL_SetTextureColorModFloat(
        texture: *mut SDL_Texture,
        r: ::core::ffi::c_float,
        g: ::core::ffi::c_float,
        b: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the additional color value multiplied into render copy operations.
    ///
    /// \param texture the texture to query.
    /// \param r a pointer filled in with the current red color value.
    /// \param g a pointer filled in with the current green color value.
    /// \param b a pointer filled in with the current blue color value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaMod
    /// \sa SDL_GetTextureColorModFloat
    /// \sa SDL_SetTextureColorMod
    pub fn SDL_GetTextureColorMod(
        texture: *mut SDL_Texture,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the additional color value multiplied into render copy operations.
    ///
    /// \param texture the texture to query.
    /// \param r a pointer filled in with the current red color value.
    /// \param g a pointer filled in with the current green color value.
    /// \param b a pointer filled in with the current blue color value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaModFloat
    /// \sa SDL_GetTextureColorMod
    /// \sa SDL_SetTextureColorModFloat
    pub fn SDL_GetTextureColorModFloat(
        texture: *mut SDL_Texture,
        r: *mut ::core::ffi::c_float,
        g: *mut ::core::ffi::c_float,
        b: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set an additional alpha value multiplied into render copy operations.
    ///
    /// When this texture is rendered, during the copy operation the source alpha
    /// value is modulated by this alpha value according to the following formula:
    ///
    /// `srcA = srcA * (alpha / 255)`
    ///
    /// Alpha modulation is not always supported by the renderer; it will return
    /// SDL_FALSE if alpha modulation is not supported.
    ///
    /// \param texture the texture to update.
    /// \param alpha the source alpha value multiplied into copy operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaMod
    /// \sa SDL_SetTextureAlphaModFloat
    /// \sa SDL_SetTextureColorMod
    pub fn SDL_SetTextureAlphaMod(texture: *mut SDL_Texture, alpha: Uint8) -> SDL_bool;
}

extern "C" {
    /// Set an additional alpha value multiplied into render copy operations.
    ///
    /// When this texture is rendered, during the copy operation the source alpha
    /// value is modulated by this alpha value according to the following formula:
    ///
    /// `srcA = srcA * alpha`
    ///
    /// Alpha modulation is not always supported by the renderer; it will return
    /// SDL_FALSE if alpha modulation is not supported.
    ///
    /// \param texture the texture to update.
    /// \param alpha the source alpha value multiplied into copy operations.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaModFloat
    /// \sa SDL_SetTextureAlphaMod
    /// \sa SDL_SetTextureColorModFloat
    pub fn SDL_SetTextureAlphaModFloat(
        texture: *mut SDL_Texture,
        alpha: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the additional alpha value multiplied into render copy operations.
    ///
    /// \param texture the texture to query.
    /// \param alpha a pointer filled in with the current alpha value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaModFloat
    /// \sa SDL_GetTextureColorMod
    /// \sa SDL_SetTextureAlphaMod
    pub fn SDL_GetTextureAlphaMod(texture: *mut SDL_Texture, alpha: *mut Uint8) -> SDL_bool;
}

extern "C" {
    /// Get the additional alpha value multiplied into render copy operations.
    ///
    /// \param texture the texture to query.
    /// \param alpha a pointer filled in with the current alpha value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureAlphaMod
    /// \sa SDL_GetTextureColorModFloat
    /// \sa SDL_SetTextureAlphaModFloat
    pub fn SDL_GetTextureAlphaModFloat(
        texture: *mut SDL_Texture,
        alpha: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the blend mode for a texture, used by SDL_RenderTexture().
    ///
    /// If the blend mode is not supported, the closest supported mode is chosen
    /// and this function returns SDL_FALSE.
    ///
    /// \param texture the texture to update.
    /// \param blendMode the SDL_BlendMode to use for texture blending.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureBlendMode
    pub fn SDL_SetTextureBlendMode(texture: *mut SDL_Texture, blendMode: SDL_BlendMode)
        -> SDL_bool;
}

extern "C" {
    /// Get the blend mode used for texture copy operations.
    ///
    /// \param texture the texture to query.
    /// \param blendMode a pointer filled in with the current SDL_BlendMode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTextureBlendMode
    pub fn SDL_GetTextureBlendMode(
        texture: *mut SDL_Texture,
        blendMode: *mut SDL_BlendMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the scale mode used for texture scale operations.
    ///
    /// The default texture scale mode is SDL_SCALEMODE_LINEAR.
    ///
    /// If the scale mode is not supported, the closest supported mode is chosen.
    ///
    /// \param texture the texture to update.
    /// \param scaleMode the SDL_ScaleMode to use for texture scaling.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetTextureScaleMode
    pub fn SDL_SetTextureScaleMode(texture: *mut SDL_Texture, scaleMode: SDL_ScaleMode)
        -> SDL_bool;
}

extern "C" {
    /// Get the scale mode used for texture scale operations.
    ///
    /// \param texture the texture to query.
    /// \param scaleMode a pointer filled in with the current scale mode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetTextureScaleMode
    pub fn SDL_GetTextureScaleMode(
        texture: *mut SDL_Texture,
        scaleMode: *mut SDL_ScaleMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Update the given texture rectangle with new pixel data.
    ///
    /// The pixel data must be in the pixel format of the texture, which can be
    /// queried using the SDL_PROP_TEXTURE_FORMAT_NUMBER property.
    ///
    /// This is a fairly slow function, intended for use with static textures that
    /// do not change often.
    ///
    /// If the texture is intended to be updated often, it is preferred to create
    /// the texture as streaming and use the locking functions referenced below.
    /// While this function will work with streaming textures, for optimization
    /// reasons you may not get the pixels back if you lock the texture afterward.
    ///
    /// \param texture the texture to update.
    /// \param rect an SDL_Rect structure representing the area to update, or NULL
    ///             to update the entire texture.
    /// \param pixels the raw pixel data in the format of the texture.
    /// \param pitch the number of bytes in a row of pixel data, including padding
    ///              between lines.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockTexture
    /// \sa SDL_UnlockTexture
    /// \sa SDL_UpdateNVTexture
    /// \sa SDL_UpdateYUVTexture
    pub fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const ::core::ffi::c_void,
        pitch: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Update a rectangle within a planar YV12 or IYUV texture with new pixel
    /// data.
    ///
    /// You can use SDL_UpdateTexture() as long as your pixel data is a contiguous
    /// block of Y and U/V planes in the proper order, but this function is
    /// available if your pixel data is not contiguous.
    ///
    /// \param texture the texture to update.
    /// \param rect a pointer to the rectangle of pixels to update, or NULL to
    ///             update the entire texture.
    /// \param Yplane the raw pixel data for the Y plane.
    /// \param Ypitch the number of bytes between rows of pixel data for the Y
    ///               plane.
    /// \param Uplane the raw pixel data for the U plane.
    /// \param Upitch the number of bytes between rows of pixel data for the U
    ///               plane.
    /// \param Vplane the raw pixel data for the V plane.
    /// \param Vpitch the number of bytes between rows of pixel data for the V
    ///               plane.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_UpdateNVTexture
    /// \sa SDL_UpdateTexture
    pub fn SDL_UpdateYUVTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        Yplane: *const Uint8,
        Ypitch: ::core::ffi::c_int,
        Uplane: *const Uint8,
        Upitch: ::core::ffi::c_int,
        Vplane: *const Uint8,
        Vpitch: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Update a rectangle within a planar NV12 or NV21 texture with new pixels.
    ///
    /// You can use SDL_UpdateTexture() as long as your pixel data is a contiguous
    /// block of NV12/21 planes in the proper order, but this function is available
    /// if your pixel data is not contiguous.
    ///
    /// \param texture the texture to update.
    /// \param rect a pointer to the rectangle of pixels to update, or NULL to
    ///             update the entire texture.
    /// \param Yplane the raw pixel data for the Y plane.
    /// \param Ypitch the number of bytes between rows of pixel data for the Y
    ///               plane.
    /// \param UVplane the raw pixel data for the UV plane.
    /// \param UVpitch the number of bytes between rows of pixel data for the UV
    ///                plane.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_UpdateTexture
    /// \sa SDL_UpdateYUVTexture
    pub fn SDL_UpdateNVTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        Yplane: *const Uint8,
        Ypitch: ::core::ffi::c_int,
        UVplane: *const Uint8,
        UVpitch: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Lock a portion of the texture for **write-only** pixel access.
    ///
    /// As an optimization, the pixels made available for editing don't necessarily
    /// contain the old texture data. This is a write-only operation, and if you
    /// need to keep a copy of the texture data you should do that at the
    /// application level.
    ///
    /// You must use SDL_UnlockTexture() to unlock the pixels and apply any
    /// changes.
    ///
    /// \param texture the texture to lock for access, which was created with
    ///                `SDL_TEXTUREACCESS_STREAMING`.
    /// \param rect an SDL_Rect structure representing the area to lock for access;
    ///             NULL to lock the entire texture.
    /// \param pixels this is filled in with a pointer to the locked pixels,
    ///               appropriately offset by the locked area.
    /// \param pitch this is filled in with the pitch of the locked pixels; the
    ///              pitch is the length of one row in bytes.
    /// \returns SDL_TRUE on success or SDL_FALSE if the texture is not valid or
    ///          was not created with `SDL_TEXTUREACCESS_STREAMING`; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockTextureToSurface
    /// \sa SDL_UnlockTexture
    pub fn SDL_LockTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *mut *mut ::core::ffi::c_void,
        pitch: *mut ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Lock a portion of the texture for **write-only** pixel access, and expose
    /// it as a SDL surface.
    ///
    /// Besides providing an SDL_Surface instead of raw pixel data, this function
    /// operates like SDL_LockTexture.
    ///
    /// As an optimization, the pixels made available for editing don't necessarily
    /// contain the old texture data. This is a write-only operation, and if you
    /// need to keep a copy of the texture data you should do that at the
    /// application level.
    ///
    /// You must use SDL_UnlockTexture() to unlock the pixels and apply any
    /// changes.
    ///
    /// The returned surface is freed internally after calling SDL_UnlockTexture()
    /// or SDL_DestroyTexture(). The caller should not free it.
    ///
    /// \param texture the texture to lock for access, which must be created with
    ///                `SDL_TEXTUREACCESS_STREAMING`.
    /// \param rect a pointer to the rectangle to lock for access. If the rect is
    ///             NULL, the entire texture will be locked.
    /// \param surface this is filled in with an SDL surface representing the
    ///                locked area.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockTexture
    /// \sa SDL_UnlockTexture
    pub fn SDL_LockTextureToSurface(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        surface: *mut *mut SDL_Surface,
    ) -> SDL_bool;
}

extern "C" {
    /// Unlock a texture, uploading the changes to video memory, if needed.
    ///
    /// **Warning**: Please note that SDL_LockTexture() is intended to be
    /// write-only; it will not guarantee the previous contents of the texture will
    /// be provided. You must fully initialize any area of a texture that you lock
    /// before unlocking it, as the pixels might otherwise be uninitialized memory.
    ///
    /// Which is to say: locking and immediately unlocking a texture can result in
    /// corrupted textures, depending on the renderer in use.
    ///
    /// \param texture a texture locked by SDL_LockTexture().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_LockTexture
    pub fn SDL_UnlockTexture(texture: *mut SDL_Texture);
}

extern "C" {
    /// Set a texture as the current rendering target.
    ///
    /// The default render target is the window for which the renderer was created.
    /// To stop rendering to a texture and render to the window again, call this
    /// function with a NULL `texture`.
    ///
    /// \param renderer the rendering context.
    /// \param texture the targeted texture, which must be created with the
    ///                `SDL_TEXTUREACCESS_TARGET` flag, or NULL to render to the
    ///                window instead of a texture.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderTarget
    pub fn SDL_SetRenderTarget(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture) -> SDL_bool;
}

extern "C" {
    /// Get the current render target.
    ///
    /// The default render target is the window for which the renderer was created,
    /// and is reported a NULL here.
    ///
    /// \param renderer the rendering context.
    /// \returns the current render target or NULL for the default render target.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderTarget
    pub fn SDL_GetRenderTarget(renderer: *mut SDL_Renderer) -> *mut SDL_Texture;
}

extern "C" {
    /// Set a device independent resolution and presentation mode for rendering.
    ///
    /// This function sets the width and height of the logical rendering output. A
    /// render target is created at the specified size and used for rendering and
    /// then copied to the output during presentation.
    ///
    /// You can disable logical coordinates by setting the mode to
    /// SDL_LOGICAL_PRESENTATION_DISABLED, and in that case you get the full pixel
    /// resolution of the output window.
    ///
    /// You can convert coordinates in an event into rendering coordinates using
    /// SDL_ConvertEventToRenderCoordinates().
    ///
    /// \param renderer the rendering context.
    /// \param w the width of the logical resolution.
    /// \param h the height of the logical resolution.
    /// \param mode the presentation mode used.
    /// \param scale_mode the scale mode used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ConvertEventToRenderCoordinates
    /// \sa SDL_GetRenderLogicalPresentation
    /// \sa SDL_GetRenderLogicalPresentationRect
    pub fn SDL_SetRenderLogicalPresentation(
        renderer: *mut SDL_Renderer,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        mode: SDL_RendererLogicalPresentation,
        scale_mode: SDL_ScaleMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Get device independent resolution and presentation mode for rendering.
    ///
    /// This function gets the width and height of the logical rendering output, or
    /// the output size in pixels if a logical resolution is not enabled.
    ///
    /// \param renderer the rendering context.
    /// \param w an int to be filled with the width.
    /// \param h an int to be filled with the height.
    /// \param mode a pointer filled in with the presentation mode.
    /// \param scale_mode a pointer filled in with the scale mode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderLogicalPresentation
    pub fn SDL_GetRenderLogicalPresentation(
        renderer: *mut SDL_Renderer,
        w: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        mode: *mut SDL_RendererLogicalPresentation,
        scale_mode: *mut SDL_ScaleMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the final presentation rectangle for rendering.
    ///
    /// This function returns the calculated rectangle used for logical
    /// presentation, based on the presentation mode and output size. If logical
    /// presentation is disabled, it will fill the rectangle with the output size,
    /// in pixels.
    ///
    /// \param renderer the rendering context.
    /// \param rect a pointer filled in with the final presentation rectangle, may
    ///             be NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderLogicalPresentation
    pub fn SDL_GetRenderLogicalPresentationRect(
        renderer: *mut SDL_Renderer,
        rect: *mut SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Get a point in render coordinates when given a point in window coordinates.
    ///
    /// \param renderer the rendering context.
    /// \param window_x the x coordinate in window coordinates.
    /// \param window_y the y coordinate in window coordinates.
    /// \param x a pointer filled with the x coordinate in render coordinates.
    /// \param y a pointer filled with the y coordinate in render coordinates.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderLogicalPresentation
    /// \sa SDL_SetRenderScale
    pub fn SDL_RenderCoordinatesFromWindow(
        renderer: *mut SDL_Renderer,
        window_x: ::core::ffi::c_float,
        window_y: ::core::ffi::c_float,
        x: *mut ::core::ffi::c_float,
        y: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get a point in window coordinates when given a point in render coordinates.
    ///
    /// \param renderer the rendering context.
    /// \param x the x coordinate in render coordinates.
    /// \param y the y coordinate in render coordinates.
    /// \param window_x a pointer filled with the x coordinate in window
    ///                 coordinates.
    /// \param window_y a pointer filled with the y coordinate in window
    ///                 coordinates.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderLogicalPresentation
    /// \sa SDL_SetRenderScale
    pub fn SDL_RenderCoordinatesToWindow(
        renderer: *mut SDL_Renderer,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
        window_x: *mut ::core::ffi::c_float,
        window_y: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Convert the coordinates in an event to render coordinates.
    ///
    /// Touch coordinates are converted from normalized coordinates in the window
    /// to non-normalized rendering coordinates.
    ///
    /// Once converted, the coordinates may be outside the rendering area.
    ///
    /// \param renderer the rendering context.
    /// \param event the event to modify.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderCoordinatesFromWindow
    pub fn SDL_ConvertEventToRenderCoordinates(
        renderer: *mut SDL_Renderer,
        event: *mut SDL_Event,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the drawing area for rendering on the current target.
    ///
    /// \param renderer the rendering context.
    /// \param rect the SDL_Rect structure representing the drawing area, or NULL
    ///             to set the viewport to the entire target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderViewport
    /// \sa SDL_RenderViewportSet
    pub fn SDL_SetRenderViewport(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Get the drawing area for the current target.
    ///
    /// \param renderer the rendering context.
    /// \param rect an SDL_Rect structure filled in with the current drawing area.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderViewportSet
    /// \sa SDL_SetRenderViewport
    pub fn SDL_GetRenderViewport(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Return whether an explicit rectangle was set as the viewport.
    ///
    /// This is useful if you're saving and restoring the viewport and want to know
    /// whether you should restore a specific rectangle or NULL. Note that the
    /// viewport is always reset when changing rendering targets.
    ///
    /// \param renderer the rendering context.
    /// \returns SDL_TRUE if the viewport was set to a specific rectangle, or
    ///          SDL_FALSE if it was set to NULL (the entire target).
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderViewport
    /// \sa SDL_SetRenderViewport
    pub fn SDL_RenderViewportSet(renderer: *mut SDL_Renderer) -> SDL_bool;
}

extern "C" {
    /// Get the safe area for rendering within the current viewport.
    ///
    /// Some devices have portions of the screen which are partially obscured or
    /// not interactive, possibly due to on-screen controls, curved edges, camera
    /// notches, TV overscan, etc. This function provides the area of the current
    /// viewport which is safe to have interactible content. You should continue
    /// rendering into the rest of the render target, but it should not contain
    /// visually important or interactible content.
    ///
    /// \param renderer the rendering context.
    /// \param rect a pointer filled in with the area that is safe for interactive
    ///             content.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetRenderSafeArea(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Set the clip rectangle for rendering on the specified target.
    ///
    /// \param renderer the rendering context.
    /// \param rect an SDL_Rect structure representing the clip area, relative to
    ///             the viewport, or NULL to disable clipping.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderClipRect
    /// \sa SDL_RenderClipEnabled
    pub fn SDL_SetRenderClipRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Get the clip rectangle for the current target.
    ///
    /// \param renderer the rendering context.
    /// \param rect an SDL_Rect structure filled in with the current clipping area
    ///             or an empty rectangle if clipping is disabled.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderClipEnabled
    /// \sa SDL_SetRenderClipRect
    pub fn SDL_GetRenderClipRect(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect) -> SDL_bool;
}

extern "C" {
    /// Get whether clipping is enabled on the given renderer.
    ///
    /// \param renderer the rendering context.
    /// \returns SDL_TRUE if clipping is enabled or SDL_FALSE if not; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderClipRect
    /// \sa SDL_SetRenderClipRect
    pub fn SDL_RenderClipEnabled(renderer: *mut SDL_Renderer) -> SDL_bool;
}

extern "C" {
    /// Set the drawing scale for rendering on the current target.
    ///
    /// The drawing coordinates are scaled by the x/y scaling factors before they
    /// are used by the renderer. This allows resolution independent drawing with a
    /// single coordinate system.
    ///
    /// If this results in scaling or subpixel drawing by the rendering backend, it
    /// will be handled using the appropriate quality hints. For best results use
    /// integer scaling factors.
    ///
    /// \param renderer the rendering context.
    /// \param scaleX the horizontal scaling factor.
    /// \param scaleY the vertical scaling factor.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderScale
    pub fn SDL_SetRenderScale(
        renderer: *mut SDL_Renderer,
        scaleX: ::core::ffi::c_float,
        scaleY: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the drawing scale for the current target.
    ///
    /// \param renderer the rendering context.
    /// \param scaleX a pointer filled in with the horizontal scaling factor.
    /// \param scaleY a pointer filled in with the vertical scaling factor.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderScale
    pub fn SDL_GetRenderScale(
        renderer: *mut SDL_Renderer,
        scaleX: *mut ::core::ffi::c_float,
        scaleY: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the color used for drawing operations.
    ///
    /// Set the color for drawing or filling rectangles, lines, and points, and for
    /// SDL_RenderClear().
    ///
    /// \param renderer the rendering context.
    /// \param r the red value used to draw on the rendering target.
    /// \param g the green value used to draw on the rendering target.
    /// \param b the blue value used to draw on the rendering target.
    /// \param a the alpha value used to draw on the rendering target; usually
    ///          `SDL_ALPHA_OPAQUE` (255). Use SDL_SetRenderDrawBlendMode to
    ///          specify how the alpha channel is used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderDrawColor
    /// \sa SDL_SetRenderDrawColorFloat
    pub fn SDL_SetRenderDrawColor(
        renderer: *mut SDL_Renderer,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the color used for drawing operations (Rect, Line and Clear).
    ///
    /// Set the color for drawing or filling rectangles, lines, and points, and for
    /// SDL_RenderClear().
    ///
    /// \param renderer the rendering context.
    /// \param r the red value used to draw on the rendering target.
    /// \param g the green value used to draw on the rendering target.
    /// \param b the blue value used to draw on the rendering target.
    /// \param a the alpha value used to draw on the rendering target. Use
    ///          SDL_SetRenderDrawBlendMode to specify how the alpha channel is
    ///          used.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderDrawColorFloat
    /// \sa SDL_SetRenderDrawColor
    pub fn SDL_SetRenderDrawColorFloat(
        renderer: *mut SDL_Renderer,
        r: ::core::ffi::c_float,
        g: ::core::ffi::c_float,
        b: ::core::ffi::c_float,
        a: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the color used for drawing operations (Rect, Line and Clear).
    ///
    /// \param renderer the rendering context.
    /// \param r a pointer filled in with the red value used to draw on the
    ///          rendering target.
    /// \param g a pointer filled in with the green value used to draw on the
    ///          rendering target.
    /// \param b a pointer filled in with the blue value used to draw on the
    ///          rendering target.
    /// \param a a pointer filled in with the alpha value used to draw on the
    ///          rendering target; usually `SDL_ALPHA_OPAQUE` (255).
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderDrawColorFloat
    /// \sa SDL_SetRenderDrawColor
    pub fn SDL_GetRenderDrawColor(
        renderer: *mut SDL_Renderer,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
        a: *mut Uint8,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the color used for drawing operations (Rect, Line and Clear).
    ///
    /// \param renderer the rendering context.
    /// \param r a pointer filled in with the red value used to draw on the
    ///          rendering target.
    /// \param g a pointer filled in with the green value used to draw on the
    ///          rendering target.
    /// \param b a pointer filled in with the blue value used to draw on the
    ///          rendering target.
    /// \param a a pointer filled in with the alpha value used to draw on the
    ///          rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderDrawColorFloat
    /// \sa SDL_GetRenderDrawColor
    pub fn SDL_GetRenderDrawColorFloat(
        renderer: *mut SDL_Renderer,
        r: *mut ::core::ffi::c_float,
        g: *mut ::core::ffi::c_float,
        b: *mut ::core::ffi::c_float,
        a: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the color scale used for render operations.
    ///
    /// The color scale is an additional scale multiplied into the pixel color
    /// value while rendering. This can be used to adjust the brightness of colors
    /// during HDR rendering, or changing HDR video brightness when playing on an
    /// SDR display.
    ///
    /// The color scale does not affect the alpha channel, only the color
    /// brightness.
    ///
    /// \param renderer the rendering context.
    /// \param scale the color scale value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderColorScale
    pub fn SDL_SetRenderColorScale(
        renderer: *mut SDL_Renderer,
        scale: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the color scale used for render operations.
    ///
    /// \param renderer the rendering context.
    /// \param scale a pointer filled in with the current color scale value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderColorScale
    pub fn SDL_GetRenderColorScale(
        renderer: *mut SDL_Renderer,
        scale: *mut ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Set the blend mode used for drawing operations (Fill and Line).
    ///
    /// If the blend mode is not supported, the closest supported mode is chosen.
    ///
    /// \param renderer the rendering context.
    /// \param blendMode the SDL_BlendMode to use for blending.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderDrawBlendMode
    pub fn SDL_SetRenderDrawBlendMode(
        renderer: *mut SDL_Renderer,
        blendMode: SDL_BlendMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Get the blend mode used for drawing operations.
    ///
    /// \param renderer the rendering context.
    /// \param blendMode a pointer filled in with the current SDL_BlendMode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderDrawBlendMode
    pub fn SDL_GetRenderDrawBlendMode(
        renderer: *mut SDL_Renderer,
        blendMode: *mut SDL_BlendMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Clear the current rendering target with the drawing color.
    ///
    /// This function clears the entire rendering target, ignoring the viewport and
    /// the clip rectangle. Note, that clearing will also set/fill all pixels of
    /// the rendering target to current renderer draw color, so make sure to invoke
    /// SDL_SetRenderDrawColor() when needed.
    ///
    /// \param renderer the rendering context.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderDrawColor
    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> SDL_bool;
}

extern "C" {
    /// Draw a point on the current rendering target at subpixel precision.
    ///
    /// \param renderer the renderer which should draw a point.
    /// \param x the x coordinate of the point.
    /// \param y the y coordinate of the point.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderPoints
    pub fn SDL_RenderPoint(
        renderer: *mut SDL_Renderer,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Draw multiple points on the current rendering target at subpixel precision.
    ///
    /// \param renderer the renderer which should draw multiple points.
    /// \param points the points to draw.
    /// \param count the number of points to draw.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderPoint
    pub fn SDL_RenderPoints(
        renderer: *mut SDL_Renderer,
        points: *const SDL_FPoint,
        count: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Draw a line on the current rendering target at subpixel precision.
    ///
    /// \param renderer the renderer which should draw a line.
    /// \param x1 the x coordinate of the start point.
    /// \param y1 the y coordinate of the start point.
    /// \param x2 the x coordinate of the end point.
    /// \param y2 the y coordinate of the end point.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderLines
    pub fn SDL_RenderLine(
        renderer: *mut SDL_Renderer,
        x1: ::core::ffi::c_float,
        y1: ::core::ffi::c_float,
        x2: ::core::ffi::c_float,
        y2: ::core::ffi::c_float,
    ) -> SDL_bool;
}

extern "C" {
    /// Draw a series of connected lines on the current rendering target at
    /// subpixel precision.
    ///
    /// \param renderer the renderer which should draw multiple lines.
    /// \param points the points along the lines.
    /// \param count the number of points, drawing count-1 lines.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderLine
    pub fn SDL_RenderLines(
        renderer: *mut SDL_Renderer,
        points: *const SDL_FPoint,
        count: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Draw a rectangle on the current rendering target at subpixel precision.
    ///
    /// \param renderer the renderer which should draw a rectangle.
    /// \param rect a pointer to the destination rectangle, or NULL to outline the
    ///             entire rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderRects
    pub fn SDL_RenderRect(renderer: *mut SDL_Renderer, rect: *const SDL_FRect) -> SDL_bool;
}

extern "C" {
    /// Draw some number of rectangles on the current rendering target at subpixel
    /// precision.
    ///
    /// \param renderer the renderer which should draw multiple rectangles.
    /// \param rects a pointer to an array of destination rectangles.
    /// \param count the number of rectangles.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderRect
    pub fn SDL_RenderRects(
        renderer: *mut SDL_Renderer,
        rects: *const SDL_FRect,
        count: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Fill a rectangle on the current rendering target with the drawing color at
    /// subpixel precision.
    ///
    /// \param renderer the renderer which should fill a rectangle.
    /// \param rect a pointer to the destination rectangle, or NULL for the entire
    ///             rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderFillRects
    pub fn SDL_RenderFillRect(renderer: *mut SDL_Renderer, rect: *const SDL_FRect) -> SDL_bool;
}

extern "C" {
    /// Fill some number of rectangles on the current rendering target with the
    /// drawing color at subpixel precision.
    ///
    /// \param renderer the renderer which should fill multiple rectangles.
    /// \param rects a pointer to an array of destination rectangles.
    /// \param count the number of rectangles.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderFillRect
    pub fn SDL_RenderFillRects(
        renderer: *mut SDL_Renderer,
        rects: *const SDL_FRect,
        count: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Copy a portion of the texture to the current rendering target at subpixel
    /// precision.
    ///
    /// \param renderer the renderer which should copy parts of a texture.
    /// \param texture the source texture.
    /// \param srcrect a pointer to the source rectangle, or NULL for the entire
    ///                texture.
    /// \param dstrect a pointer to the destination rectangle, or NULL for the
    ///                entire rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderTextureRotated
    /// \sa SDL_RenderTextureTiled
    pub fn SDL_RenderTexture(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_FRect,
        dstrect: *const SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Copy a portion of the source texture to the current rendering target, with
    /// rotation and flipping, at subpixel precision.
    ///
    /// \param renderer the renderer which should copy parts of a texture.
    /// \param texture the source texture.
    /// \param srcrect a pointer to the source rectangle, or NULL for the entire
    ///                texture.
    /// \param dstrect a pointer to the destination rectangle, or NULL for the
    ///                entire rendering target.
    /// \param angle an angle in degrees that indicates the rotation that will be
    ///              applied to dstrect, rotating it in a clockwise direction.
    /// \param center a pointer to a point indicating the point around which
    ///               dstrect will be rotated (if NULL, rotation will be done
    ///               around dstrect.w/2, dstrect.h/2).
    /// \param flip an SDL_FlipMode value stating which flipping actions should be
    ///             performed on the texture.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderTexture
    pub fn SDL_RenderTextureRotated(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_FRect,
        dstrect: *const SDL_FRect,
        angle: ::core::ffi::c_double,
        center: *const SDL_FPoint,
        flip: SDL_FlipMode,
    ) -> SDL_bool;
}

extern "C" {
    /// Tile a portion of the texture to the current rendering target at subpixel
    /// precision.
    ///
    /// The pixels in `srcrect` will be repeated as many times as needed to
    /// completely fill `dstrect`.
    ///
    /// \param renderer the renderer which should copy parts of a texture.
    /// \param texture the source texture.
    /// \param srcrect a pointer to the source rectangle, or NULL for the entire
    ///                texture.
    /// \param scale the scale used to transform srcrect into the destination
    ///              rectangle, e.g. a 32x32 texture with a scale of 2 would fill
    ///              64x64 tiles.
    /// \param dstrect a pointer to the destination rectangle, or NULL for the
    ///                entire rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderTexture
    pub fn SDL_RenderTextureTiled(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_FRect,
        scale: ::core::ffi::c_float,
        dstrect: *const SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Perform a scaled copy using the 9-grid algorithm to the current rendering
    /// target at subpixel precision.
    ///
    /// The pixels in the texture are split into a 3x3 grid, using the different
    /// corner sizes for each corner, and the sides and center making up the
    /// remaining pixels. The corners are then scaled using `scale` and fit into
    /// the corners of the destination rectangle. The sides and center are then
    /// stretched into place to cover the remaining destination rectangle.
    ///
    /// \param renderer the renderer which should copy parts of a texture.
    /// \param texture the source texture.
    /// \param srcrect the SDL_Rect structure representing the rectangle to be used
    ///                for the 9-grid, or NULL to use the entire texture.
    /// \param left_width the width, in pixels, of the left corners in `srcrect`.
    /// \param right_width the width, in pixels, of the right corners in `srcrect`.
    /// \param top_height the height, in pixels, of the top corners in `srcrect`.
    /// \param bottom_height the height, in pixels, of the bottom corners in
    ///                      `srcrect`.
    /// \param scale the scale used to transform the corner of `srcrect` into the
    ///              corner of `dstrect`, or 0.0f for an unscaled copy.
    /// \param dstrect a pointer to the destination rectangle, or NULL for the
    ///                entire rendering target.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderTexture
    pub fn SDL_RenderTexture9Grid(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_FRect,
        left_width: ::core::ffi::c_float,
        right_width: ::core::ffi::c_float,
        top_height: ::core::ffi::c_float,
        bottom_height: ::core::ffi::c_float,
        scale: ::core::ffi::c_float,
        dstrect: *const SDL_FRect,
    ) -> SDL_bool;
}

extern "C" {
    /// Render a list of triangles, optionally using a texture and indices into the
    /// vertex array Color and alpha modulation is done per vertex
    /// (SDL_SetTextureColorMod and SDL_SetTextureAlphaMod are ignored).
    ///
    /// \param renderer the rendering context.
    /// \param texture (optional) The SDL texture to use.
    /// \param vertices vertices.
    /// \param num_vertices number of vertices.
    /// \param indices (optional) An array of integer indices into the 'vertices'
    ///                array, if NULL all vertices will be rendered in sequential
    ///                order.
    /// \param num_indices number of indices.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderGeometryRaw
    pub fn SDL_RenderGeometry(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        vertices: *const SDL_Vertex,
        num_vertices: ::core::ffi::c_int,
        indices: *const ::core::ffi::c_int,
        num_indices: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Render a list of triangles, optionally using a texture and indices into the
    /// vertex arrays Color and alpha modulation is done per vertex
    /// (SDL_SetTextureColorMod and SDL_SetTextureAlphaMod are ignored).
    ///
    /// \param renderer the rendering context.
    /// \param texture (optional) The SDL texture to use.
    /// \param xy vertex positions.
    /// \param xy_stride byte size to move from one element to the next element.
    /// \param color vertex colors (as SDL_FColor).
    /// \param color_stride byte size to move from one element to the next element.
    /// \param uv vertex normalized texture coordinates.
    /// \param uv_stride byte size to move from one element to the next element.
    /// \param num_vertices number of vertices.
    /// \param indices (optional) An array of indices into the 'vertices' arrays,
    ///                if NULL all vertices will be rendered in sequential order.
    /// \param num_indices number of indices.
    /// \param size_indices index size: 1 (byte), 2 (short), 4 (int).
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_RenderGeometry
    pub fn SDL_RenderGeometryRaw(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        xy: *const ::core::ffi::c_float,
        xy_stride: ::core::ffi::c_int,
        color: *const SDL_FColor,
        color_stride: ::core::ffi::c_int,
        uv: *const ::core::ffi::c_float,
        uv_stride: ::core::ffi::c_int,
        num_vertices: ::core::ffi::c_int,
        indices: *const ::core::ffi::c_void,
        num_indices: ::core::ffi::c_int,
        size_indices: ::core::ffi::c_int,
    ) -> SDL_bool;
}

extern "C" {
    /// Read pixels from the current rendering target.
    ///
    /// The returned surface should be freed with SDL_DestroySurface()
    ///
    /// **WARNING**: This is a very slow operation, and should not be used
    /// frequently. If you're using this on the main rendering target, it should be
    /// called after rendering and before SDL_RenderPresent().
    ///
    /// \param renderer the rendering context.
    /// \param rect an SDL_Rect structure representing the area in pixels relative
    ///             to the to current viewport, or NULL for the entire viewport.
    /// \returns a new SDL_Surface on success or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RenderReadPixels(
        renderer: *mut SDL_Renderer,
        rect: *const SDL_Rect,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Update the screen with any rendering performed since the previous call.
    ///
    /// SDL's rendering functions operate on a backbuffer; that is, calling a
    /// rendering function such as SDL_RenderLine() does not directly put a line on
    /// the screen, but rather updates the backbuffer. As such, you compose your
    /// entire scene and *present* the composed backbuffer to the screen as a
    /// complete picture.
    ///
    /// Therefore, when using SDL's rendering API, one does all drawing intended
    /// for the frame, and then calls this function once per frame to present the
    /// final drawing to the user.
    ///
    /// The backbuffer should be considered invalidated after each present; do not
    /// assume that previous contents will exist between frames. You are strongly
    /// encouraged to call SDL_RenderClear() to initialize the backbuffer before
    /// starting each new frame's drawing, even if you plan to overwrite every
    /// pixel.
    ///
    /// Please note, that in case of rendering to a texture - there is **no need**
    /// to call `SDL_RenderPresent` after drawing needed objects to a texture, you
    /// are only required to change back the rendering target to default via
    /// `SDL_SetRenderTarget(renderer, NULL)` afterwards, as textures by themselves
    /// do not have a concept of backbuffers.
    ///
    /// \param renderer the rendering context.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety You may only call this function on the main thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRenderer
    /// \sa SDL_RenderClear
    /// \sa SDL_RenderFillRect
    /// \sa SDL_RenderFillRects
    /// \sa SDL_RenderLine
    /// \sa SDL_RenderLines
    /// \sa SDL_RenderPoint
    /// \sa SDL_RenderPoints
    /// \sa SDL_RenderRect
    /// \sa SDL_RenderRects
    /// \sa SDL_SetRenderDrawBlendMode
    /// \sa SDL_SetRenderDrawColor
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer) -> SDL_bool;
}

extern "C" {
    /// Destroy the specified texture.
    ///
    /// Passing NULL or an otherwise invalid texture will set the SDL error message
    /// to "Invalid texture".
    ///
    /// \param texture the texture to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateTexture
    /// \sa SDL_CreateTextureFromSurface
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
}

extern "C" {
    /// Destroy the rendering context for a window and free all associated
    /// textures.
    ///
    /// This should be called before destroying the associated window.
    ///
    /// \param renderer the rendering context.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateRenderer
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
}

extern "C" {
    /// Force the rendering context to flush any pending commands and state.
    ///
    /// You do not need to (and in fact, shouldn't) call this function unless you
    /// are planning to call into OpenGL/Direct3D/Metal/whatever directly, in
    /// addition to using an SDL_Renderer.
    ///
    /// This is for a very-specific case: if you are using SDL's render API, and
    /// you plan to make OpenGL/D3D/whatever calls in addition to SDL render API
    /// calls. If this applies, you should call this function between calls to
    /// SDL's render API and the low-level API you're using in cooperation.
    ///
    /// In all other cases, you can ignore this function.
    ///
    /// This call makes SDL flush any pending rendering work it was queueing up to
    /// do later in a single batch, and marks any internal cached state as invalid,
    /// so it'll prepare all its state again later, from scratch.
    ///
    /// This means you do not need to save state in your rendering code to protect
    /// the SDL renderer. However, there lots of arbitrary pieces of Direct3D and
    /// OpenGL state that can confuse things; you should use your best judgment and
    /// be prepared to make changes if specific state needs to be protected.
    ///
    /// \param renderer the rendering context.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_FlushRenderer(renderer: *mut SDL_Renderer) -> SDL_bool;
}

extern "C" {
    /// Get the CAMetalLayer associated with the given Metal renderer.
    ///
    /// This function returns `void *`, so SDL doesn't have to include Metal's
    /// headers, but it can be safely cast to a `CAMetalLayer *`.
    ///
    /// \param renderer the renderer to query.
    /// \returns a `CAMetalLayer *` on success, or NULL if the renderer isn't a
    ///          Metal renderer.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderMetalCommandEncoder
    pub fn SDL_GetRenderMetalLayer(renderer: *mut SDL_Renderer) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Get the Metal command encoder for the current frame.
    ///
    /// This function returns `void *`, so SDL doesn't have to include Metal's
    /// headers, but it can be safely cast to an `id<MTLRenderCommandEncoder>`.
    ///
    /// This will return NULL if Metal refuses to give SDL a drawable to render to,
    /// which might happen if the window is hidden/minimized/offscreen. This
    /// doesn't apply to command encoders for render targets, just the window's
    /// backbuffer. Check your return values!
    ///
    /// \param renderer the renderer to query.
    /// \returns an `id<MTLRenderCommandEncoder>` on success, or NULL if the
    ///          renderer isn't a Metal renderer or there was an error.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderMetalLayer
    pub fn SDL_GetRenderMetalCommandEncoder(
        renderer: *mut SDL_Renderer,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Add a set of synchronization semaphores for the current frame.
    ///
    /// The Vulkan renderer will wait for `wait_semaphore` before submitting
    /// rendering commands and signal `signal_semaphore` after rendering commands
    /// are complete for this frame.
    ///
    /// This should be called each frame that you want semaphore synchronization.
    /// The Vulkan renderer may have multiple frames in flight on the GPU, so you
    /// should have multiple semaphores that are used for synchronization. Querying
    /// SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER will give you the
    /// maximum number of semaphores you'll need.
    ///
    /// \param renderer the rendering context.
    /// \param wait_stage_mask the VkPipelineStageFlags for the wait.
    /// \param wait_semaphore a VkSempahore to wait on before rendering the current
    ///                       frame, or 0 if not needed.
    /// \param signal_semaphore a VkSempahore that SDL will signal when rendering
    ///                         for the current frame is complete, or 0 if not
    ///                         needed.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \threadsafety It is **NOT** safe to call this function from two threads at
    ///               once.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_AddVulkanRenderSemaphores(
        renderer: *mut SDL_Renderer,
        wait_stage_mask: Uint32,
        wait_semaphore: Sint64,
        signal_semaphore: Sint64,
    ) -> SDL_bool;
}

extern "C" {
    /// Toggle VSync of the given renderer.
    ///
    /// When a renderer is created, vsync defaults to SDL_RENDERER_VSYNC_DISABLED.
    ///
    /// The `vsync` parameter can be 1 to synchronize present with every vertical
    /// refresh, 2 to synchronize present with every second vertical refresh, etc.,
    /// SDL_WINDOW_SURFACE_VSYNC_ADAPTIVE for late swap tearing (adaptive vsync),
    /// or SDL_WINDOW_SURFACE_VSYNC_DISABLED to disable. Not every value is
    /// supported by every driver, so you should check the return value to see
    /// whether the requested setting is supported.
    ///
    /// \param renderer the renderer to toggle.
    /// \param vsync the vertical refresh sync interval.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderVSync
    pub fn SDL_SetRenderVSync(renderer: *mut SDL_Renderer, vsync: ::core::ffi::c_int) -> SDL_bool;
}

pub const SDL_RENDERER_VSYNC_DISABLED: ::core::primitive::i32 = 0;

pub const SDL_RENDERER_VSYNC_ADAPTIVE: ::core::primitive::i32 = -1_i32;

extern "C" {
    /// Get VSync of the given renderer.
    ///
    /// \param renderer the renderer to toggle.
    /// \param vsync an int filled with the current vertical refresh sync interval.
    ///              See SDL_SetRenderVSync() for the meaning of the value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderVSync
    pub fn SDL_GetRenderVSync(
        renderer: *mut SDL_Renderer,
        vsync: *mut ::core::ffi::c_int,
    ) -> SDL_bool;
}

/// A structure representing rendering state
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Renderer {
    _opaque: [::core::primitive::u8; 0],
}

/// An efficient driver-specific representation of pixel data
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Texture {
    _opaque: [::core::primitive::u8; 0],
}
