//! Metadata for items in the `crate::video` module

use super::*;

pub const METADATA_SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER",
    short_name: "GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER",
    value: crate::video::SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER,
    ty: PropertyType::POINTER,
    doc: Some("The pointer to the global `wl_display` object used by the Wayland video\nbackend.\n\nCan be set before the video subsystem is initialized to import an external\n`wl_display` object from an application or toolkit for use in SDL, or read\nafter initialization to export the `wl_display` used by the Wayland video\nbackend. Setting this property after the video subsystem has been\ninitialized has no effect, and reading it when the video subsystem is\nuninitialized will either return the user provided value, if one was set\nprior to initialization, or NULL. See docs/README-wayland.md for more\ninformation.\n"),
    available_since: None,
};
pub const METADATA_SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN",
    short_name: "DISPLAY_HDR_ENABLED_BOOLEAN",
    value: crate::video::SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER",
    short_name: "DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER",
    value: crate::video::SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN",
    short_name: "WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN",
    short_name: "WINDOW_CREATE_BORDERLESS_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN",
    short_name: "WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN",
    short_name: "WINDOW_CREATE_FOCUSABLE_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN",
    short_name: "WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER",
    short_name: "WINDOW_CREATE_FLAGS_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN",
    short_name: "WINDOW_CREATE_FULLSCREEN_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER",
    short_name: "WINDOW_CREATE_HEIGHT_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN",
    short_name: "WINDOW_CREATE_HIDDEN_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN",
    short_name: "WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN",
    short_name: "WINDOW_CREATE_MAXIMIZED_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN",
    short_name: "WINDOW_CREATE_MENU_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN",
    short_name: "WINDOW_CREATE_METAL_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN",
    short_name: "WINDOW_CREATE_MINIMIZED_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN",
    short_name: "WINDOW_CREATE_MODAL_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN",
    short_name: "WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN",
    short_name: "WINDOW_CREATE_OPENGL_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_PARENT_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_PARENT_POINTER",
    short_name: "WINDOW_CREATE_PARENT_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_PARENT_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN",
    short_name: "WINDOW_CREATE_RESIZABLE_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_TITLE_STRING: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_TITLE_STRING",
    short_name: "WINDOW_CREATE_TITLE_STRING",
    value: crate::video::SDL_PROP_WINDOW_CREATE_TITLE_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN",
    short_name: "WINDOW_CREATE_TRANSPARENT_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN",
    short_name: "WINDOW_CREATE_TOOLTIP_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN",
    short_name: "WINDOW_CREATE_UTILITY_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN",
    short_name: "WINDOW_CREATE_VULKAN_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER",
    short_name: "WINDOW_CREATE_WIDTH_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_X_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_X_NUMBER",
    short_name: "WINDOW_CREATE_X_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_X_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_Y_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_Y_NUMBER",
    short_name: "WINDOW_CREATE_Y_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_Y_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER",
    short_name: "WINDOW_CREATE_COCOA_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER",
    short_name: "WINDOW_CREATE_COCOA_VIEW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN: Property =
    Property {
        module: "video",
        name: "SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN",
        short_name: "WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN",
        value: crate::video::SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN,
        ty: PropertyType::BOOLEAN,
        doc: None,
        available_since: None,
    };
pub const METADATA_SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN",
    short_name: "WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER",
    short_name: "WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER",
    short_name: "WINDOW_CREATE_WIN32_HWND_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER",
    short_name: "WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER",
    short_name: "WINDOW_CREATE_X11_WINDOW_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_SHAPE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_SHAPE_POINTER",
    short_name: "WINDOW_SHAPE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_SHAPE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN",
    short_name: "WINDOW_HDR_ENABLED_BOOLEAN",
    value: crate::video::SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT",
    short_name: "WINDOW_SDR_WHITE_LEVEL_FLOAT",
    value: crate::video::SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT",
    short_name: "WINDOW_HDR_HEADROOM_FLOAT",
    value: crate::video::SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER",
    short_name: "WINDOW_ANDROID_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER",
    short_name: "WINDOW_ANDROID_SURFACE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER",
    short_name: "WINDOW_UIKIT_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER",
    short_name: "WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER",
    short_name: "WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER",
    short_name: "WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER",
    short_name: "WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER",
    short_name: "WINDOW_KMSDRM_DEVICE_INDEX_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER",
    short_name: "WINDOW_KMSDRM_DRM_FD_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER",
    short_name: "WINDOW_KMSDRM_GBM_DEVICE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_COCOA_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_COCOA_WINDOW_POINTER",
    short_name: "WINDOW_COCOA_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_COCOA_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER",
    short_name: "WINDOW_COCOA_METAL_VIEW_TAG_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_OPENVR_OVERLAY_ID: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_OPENVR_OVERLAY_ID",
    short_name: "WINDOW_OPENVR_OVERLAY_ID",
    value: crate::video::SDL_PROP_WINDOW_OPENVR_OVERLAY_ID,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER",
    short_name: "WINDOW_VIVANTE_DISPLAY_POINTER",
    value: crate::video::SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER",
    short_name: "WINDOW_VIVANTE_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER",
    short_name: "WINDOW_VIVANTE_SURFACE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WIN32_HWND_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WIN32_HWND_POINTER",
    short_name: "WINDOW_WIN32_HWND_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WIN32_HWND_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WIN32_HDC_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WIN32_HDC_POINTER",
    short_name: "WINDOW_WIN32_HDC_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WIN32_HDC_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER",
    short_name: "WINDOW_WIN32_INSTANCE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER",
    short_name: "WINDOW_WAYLAND_DISPLAY_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER",
    short_name: "WINDOW_WAYLAND_SURFACE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER",
    short_name: "WINDOW_WAYLAND_VIEWPORT_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER",
    short_name: "WINDOW_WAYLAND_EGL_WINDOW_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER",
    short_name: "WINDOW_WAYLAND_XDG_SURFACE_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER",
    short_name: "WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING",
    short_name: "WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER",
    short_name: "WINDOW_WAYLAND_XDG_POPUP_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER",
    short_name: "WINDOW_WAYLAND_XDG_POSITIONER_POINTER",
    value: crate::video::SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_X11_DISPLAY_POINTER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_X11_DISPLAY_POINTER",
    short_name: "WINDOW_X11_DISPLAY_POINTER",
    value: crate::video::SDL_PROP_WINDOW_X11_DISPLAY_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_X11_SCREEN_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_X11_SCREEN_NUMBER",
    short_name: "WINDOW_X11_SCREEN_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_X11_SCREEN_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_WINDOW_X11_WINDOW_NUMBER: Property = Property {
    module: "video",
    name: "SDL_PROP_WINDOW_X11_WINDOW_NUMBER",
    short_name: "WINDOW_X11_WINDOW_NUMBER",
    value: crate::video::SDL_PROP_WINDOW_X11_WINDOW_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_DisplayID: Group = Group {
    module: "video",
    kind: GroupKind::Id,
    name: "SDL_DisplayID",
    short_name: "DisplayID",
    doc: Some("This is a unique ID for a display for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nIf the display is disconnected and reconnected, it will get a new ID.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub const METADATA_SDL_WindowID: Group = Group {
    module: "video",
    kind: GroupKind::Id,
    name: "SDL_WindowID",
    short_name: "WindowID",
    doc: Some("This is a unique ID for a window.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub const METADATA_SDL_SystemTheme: Group = Group {
    module: "video",
    kind: GroupKind::Enum,
    name: "SDL_SystemTheme",
    short_name: "SystemTheme",
    doc: Some("System theme.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SYSTEM_THEME_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("Unknown system theme\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_THEME_LIGHT",
            short_name: "LIGHT",
            doc: Some("Light colored system theme\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SYSTEM_THEME_DARK",
            short_name: "DARK",
            doc: Some("Dark colored system theme\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_DisplayOrientation: Group = Group {
    module: "video",
    kind: GroupKind::Enum,
    name: "SDL_DisplayOrientation",
    short_name: "DisplayOrientation",
    doc: Some("Display orientation values; the way a display is rotated.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_ORIENTATION_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("The display orientation can't be determined\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ORIENTATION_LANDSCAPE",
            short_name: "LANDSCAPE",
            doc: Some("The display is in landscape mode, with the right side up, relative to portrait mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ORIENTATION_LANDSCAPE_FLIPPED",
            short_name: "LANDSCAPE_FLIPPED",
            doc: Some("The display is in landscape mode, with the left side up, relative to portrait mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ORIENTATION_PORTRAIT",
            short_name: "PORTRAIT",
            doc: Some("The display is in portrait mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_ORIENTATION_PORTRAIT_FLIPPED",
            short_name: "PORTRAIT_FLIPPED",
            doc: Some("The display is in portrait mode, upside down\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_WindowFlags: Group = Group {
    module: "video",
    kind: GroupKind::Flags,
    name: "SDL_WindowFlags",
    short_name: "WindowFlags",
    doc: Some("The flags on a window.\n\nThese cover a lot of true/false, or on/off, window state. Some of it is\nimmutable after being set through [`SDL_CreateWindow()`], some of it can be\nchanged on existing windows by the app, and some of it might be altered by\nthe user or system outside of the app's control.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetWindowFlags`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_WINDOW_FULLSCREEN",
            short_name: "FULLSCREEN",
            doc: Some("window is in fullscreen mode\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_OPENGL",
            short_name: "OPENGL",
            doc: Some("window usable with OpenGL context\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_OCCLUDED",
            short_name: "OCCLUDED",
            doc: Some("window is occluded\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_HIDDEN",
            short_name: "HIDDEN",
            doc: Some("window is neither mapped onto the desktop nor shown in the taskbar/dock/window list; [`SDL_ShowWindow()`] is required for it to become visible\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_BORDERLESS",
            short_name: "BORDERLESS",
            doc: Some("no window decoration\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_RESIZABLE",
            short_name: "RESIZABLE",
            doc: Some("window can be resized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MINIMIZED",
            short_name: "MINIMIZED",
            doc: Some("window is minimized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MAXIMIZED",
            short_name: "MAXIMIZED",
            doc: Some("window is maximized\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MOUSE_GRABBED",
            short_name: "MOUSE_GRABBED",
            doc: Some("window has grabbed mouse input\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_INPUT_FOCUS",
            short_name: "INPUT_FOCUS",
            doc: Some("window has input focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MOUSE_FOCUS",
            short_name: "MOUSE_FOCUS",
            doc: Some("window has mouse focus\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_EXTERNAL",
            short_name: "EXTERNAL",
            doc: Some("window not created by SDL\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MODAL",
            short_name: "MODAL",
            doc: Some("window is modal\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_HIGH_PIXEL_DENSITY",
            short_name: "HIGH_PIXEL_DENSITY",
            doc: Some("window uses high pixel density back buffer if possible\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MOUSE_CAPTURE",
            short_name: "MOUSE_CAPTURE",
            doc: Some("window has mouse captured (unrelated to MOUSE_GRABBED)\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_MOUSE_RELATIVE_MODE",
            short_name: "MOUSE_RELATIVE_MODE",
            doc: Some("window has relative mode enabled\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_ALWAYS_ON_TOP",
            short_name: "ALWAYS_ON_TOP",
            doc: Some("window should always be above others\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_UTILITY",
            short_name: "UTILITY",
            doc: Some("window should be treated as a utility window, not showing in the task bar and window list\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_TOOLTIP",
            short_name: "TOOLTIP",
            doc: Some("window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_POPUP_MENU",
            short_name: "POPUP_MENU",
            doc: Some("window should be treated as a popup menu, requires a parent window\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_KEYBOARD_GRABBED",
            short_name: "KEYBOARD_GRABBED",
            doc: Some("window has grabbed keyboard input\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_VULKAN",
            short_name: "VULKAN",
            doc: Some("window usable for Vulkan surface\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_METAL",
            short_name: "METAL",
            doc: Some("window usable for Metal view\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_TRANSPARENT",
            short_name: "TRANSPARENT",
            doc: Some("window with transparent buffer\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_WINDOW_NOT_FOCUSABLE",
            short_name: "NOT_FOCUSABLE",
            doc: Some("window should not be focusable\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_FlashOperation: Group = Group {
    module: "video",
    kind: GroupKind::Enum,
    name: "SDL_FlashOperation",
    short_name: "FlashOperation",
    doc: Some(
        "Window flash operation.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_FLASH_CANCEL",
            short_name: "CANCEL",
            doc: Some("Cancel any window flash state\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FLASH_BRIEFLY",
            short_name: "BRIEFLY",
            doc: Some("Flash the window briefly to get attention\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_FLASH_UNTIL_FOCUSED",
            short_name: "UNTIL_FOCUSED",
            doc: Some("Flash the window until it gets focus\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GLAttr: Group = Group {
    module: "video",
    kind: GroupKind::Enum,
    name: "SDL_GLAttr",
    short_name: "GLAttr",
    doc: Some("An enumeration of OpenGL configuration attributes.\n\nWhile you can set most OpenGL attributes normally, the attributes listed\nabove must be known before SDL creates the window that will be used with\nthe OpenGL context. These attributes are set and read with\n[`SDL_GL_SetAttribute()`] and [`SDL_GL_GetAttribute()`].\n\nIn some cases, these attributes are minimum requests; the GL does not\npromise to give you exactly what you asked for. It's possible to ask for a\n16-bit depth buffer and get a 24-bit one instead, for example, or to ask\nfor no stencil buffer and still have one available. Context creation should\nfail if the GL can't provide your requested attributes at a minimum, but\nyou should check to see exactly what you got.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GL_RED_SIZE",
            short_name: "RED_SIZE",
            doc: Some("the minimum number of bits for the red channel of the color buffer; defaults to 8.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_GREEN_SIZE",
            short_name: "GREEN_SIZE",
            doc: Some("the minimum number of bits for the green channel of the color buffer; defaults to 8.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_BLUE_SIZE",
            short_name: "BLUE_SIZE",
            doc: Some("the minimum number of bits for the blue channel of the color buffer; defaults to 8.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ALPHA_SIZE",
            short_name: "ALPHA_SIZE",
            doc: Some("the minimum number of bits for the alpha channel of the color buffer; defaults to 8.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_BUFFER_SIZE",
            short_name: "BUFFER_SIZE",
            doc: Some("the minimum number of bits for frame buffer size; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_DOUBLEBUFFER",
            short_name: "DOUBLEBUFFER",
            doc: Some("whether the output is single or double buffered; defaults to double buffering on.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_DEPTH_SIZE",
            short_name: "DEPTH_SIZE",
            doc: Some("the minimum number of bits in the depth buffer; defaults to 16.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_STENCIL_SIZE",
            short_name: "STENCIL_SIZE",
            doc: Some("the minimum number of bits in the stencil buffer; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ACCUM_RED_SIZE",
            short_name: "ACCUM_RED_SIZE",
            doc: Some("the minimum number of bits for the red channel of the accumulation buffer; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ACCUM_GREEN_SIZE",
            short_name: "ACCUM_GREEN_SIZE",
            doc: Some("the minimum number of bits for the green channel of the accumulation buffer; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ACCUM_BLUE_SIZE",
            short_name: "ACCUM_BLUE_SIZE",
            doc: Some("the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ACCUM_ALPHA_SIZE",
            short_name: "ACCUM_ALPHA_SIZE",
            doc: Some("the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_STEREO",
            short_name: "STEREO",
            doc: Some("whether the output is stereo 3D; defaults to off.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_MULTISAMPLEBUFFERS",
            short_name: "MULTISAMPLEBUFFERS",
            doc: Some("the number of buffers used for multisample anti-aliasing; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_MULTISAMPLESAMPLES",
            short_name: "MULTISAMPLESAMPLES",
            doc: Some("the number of samples used around the current pixel used for multisample anti-aliasing.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_ACCELERATED_VISUAL",
            short_name: "ACCELERATED_VISUAL",
            doc: Some("set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_RETAINED_BACKING",
            short_name: "RETAINED_BACKING",
            doc: Some("not used (deprecated).\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_MAJOR_VERSION",
            short_name: "CONTEXT_MAJOR_VERSION",
            doc: Some("OpenGL context major version.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_MINOR_VERSION",
            short_name: "CONTEXT_MINOR_VERSION",
            doc: Some("OpenGL context minor version.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_FLAGS",
            short_name: "CONTEXT_FLAGS",
            doc: Some("some combination of 0 or more of elements of the [`SDL_GLContextFlag`] enumeration; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_PROFILE_MASK",
            short_name: "CONTEXT_PROFILE_MASK",
            doc: Some("type of GL context (Core, Compatibility, ES). See [`SDL_GLProfile`]; default value depends on platform.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_SHARE_WITH_CURRENT_CONTEXT",
            short_name: "SHARE_WITH_CURRENT_CONTEXT",
            doc: Some("OpenGL context sharing; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_FRAMEBUFFER_SRGB_CAPABLE",
            short_name: "FRAMEBUFFER_SRGB_CAPABLE",
            doc: Some("requests sRGB capable visual; defaults to 0.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_RELEASE_BEHAVIOR",
            short_name: "CONTEXT_RELEASE_BEHAVIOR",
            doc: Some("sets context the release behavior. See [`SDL_GLContextReleaseFlag`]; defaults to FLUSH.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_RESET_NOTIFICATION",
            short_name: "CONTEXT_RESET_NOTIFICATION",
            doc: Some("set context reset notification. See [`SDL_GLContextResetNotification`]; defaults to NO_NOTIFICATION.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_NO_ERROR",
            short_name: "CONTEXT_NO_ERROR",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_FLOATBUFFERS",
            short_name: "FLOATBUFFERS",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_EGL_PLATFORM",
            short_name: "EGL_PLATFORM",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GLProfile: Group = Group {
    module: "video",
    kind: GroupKind::Flags,
    name: "SDL_GLProfile",
    short_name: "GLProfile",
    doc: Some("Possible values to be set for the [`SDL_GL_CONTEXT_PROFILE_MASK`] attribute.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GL_CONTEXT_PROFILE_CORE",
            short_name: "CORE",
            doc: Some("OpenGL Core Profile context\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_PROFILE_COMPATIBILITY",
            short_name: "COMPATIBILITY",
            doc: Some("OpenGL Compatibility Profile context\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_PROFILE_ES",
            short_name: "ES",
            doc: Some("GLX_CONTEXT_ES2_PROFILE_BIT_EXT\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GLContextFlag: Group = Group {
    module: "video",
    kind: GroupKind::Flags,
    name: "SDL_GLContextFlag",
    short_name: "GLContextFlag",
    doc: Some("Possible flags to be set for the [`SDL_GL_CONTEXT_FLAGS`] attribute.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GL_CONTEXT_DEBUG_FLAG",
            short_name: "DEBUG_FLAG",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG",
            short_name: "FORWARD_COMPATIBLE_FLAG",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG",
            short_name: "ROBUST_ACCESS_FLAG",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_RESET_ISOLATION_FLAG",
            short_name: "RESET_ISOLATION_FLAG",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GLContextReleaseFlag: Group = Group {
    module: "video",
    kind: GroupKind::Flags,
    name: "SDL_GLContextReleaseFlag",
    short_name: "GLContextReleaseFlag",
    doc: Some("Possible values to be set for the [`SDL_GL_CONTEXT_RELEASE_BEHAVIOR`]\nattribute.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE",
            short_name: "NONE",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH",
            short_name: "FLUSH",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_GLContextResetNotification: Group = Group {
    module: "video",
    kind: GroupKind::Flags,
    name: "SDL_GLContextResetNotification",
    short_name: "GLContextResetNotification",
    doc: Some("Possible values to be set [`SDL_GL_CONTEXT_RESET_NOTIFICATION`] attribute.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_GL_CONTEXT_RESET_NO_NOTIFICATION",
            short_name: "NO_NOTIFICATION",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_GL_CONTEXT_RESET_LOSE_CONTEXT",
            short_name: "LOSE_CONTEXT",
            doc: None,
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_HitTestResult: Group = Group {
    module: "video",
    kind: GroupKind::Enum,
    name: "SDL_HitTestResult",
    short_name: "HitTestResult",
    doc: Some("Possible return values from the [`SDL_HitTest`] callback.\n\n## Thread safety\nThis function should only be called on the main thread.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_HitTest`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_HITTEST_NORMAL",
            short_name: "NORMAL",
            doc: Some("Region is normal. No special properties.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_DRAGGABLE",
            short_name: "DRAGGABLE",
            doc: Some("Region can drag entire window.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_TOPLEFT",
            short_name: "RESIZE_TOPLEFT",
            doc: Some("Region is the resizable top-left corner border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_TOP",
            short_name: "RESIZE_TOP",
            doc: Some("Region is the resizable top border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_TOPRIGHT",
            short_name: "RESIZE_TOPRIGHT",
            doc: Some("Region is the resizable top-right corner border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_RIGHT",
            short_name: "RESIZE_RIGHT",
            doc: Some("Region is the resizable right border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_BOTTOMRIGHT",
            short_name: "RESIZE_BOTTOMRIGHT",
            doc: Some("Region is the resizable bottom-right corner border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_BOTTOM",
            short_name: "RESIZE_BOTTOM",
            doc: Some("Region is the resizable bottom border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_BOTTOMLEFT",
            short_name: "RESIZE_BOTTOMLEFT",
            doc: Some("Region is the resizable bottom-left corner border.\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HITTEST_RESIZE_LEFT",
            short_name: "RESIZE_LEFT",
            doc: Some("Region is the resizable left border.\n"),
            available_since: None,
        },
    ],
};
