//! Metadata for items in the `crate::render` module

use super::*;

pub static METADATA_SDL_PROP_RENDERER_CREATE_NAME_STRING: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_NAME_STRING",
    short_name: "RENDERER_CREATE_NAME",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_NAME_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_WINDOW_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_WINDOW_POINTER",
    short_name: "RENDERER_CREATE_WINDOW",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_WINDOW_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_SURFACE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_SURFACE_POINTER",
    short_name: "RENDERER_CREATE_SURFACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_SURFACE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER",
    short_name: "RENDERER_CREATE_OUTPUT_COLORSPACE",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER",
    short_name: "RENDERER_CREATE_PRESENT_VSYNC",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_INSTANCE",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER",
    short_name: "RENDERER_CREATE_VULKAN_SURFACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX",
        value: unsafe {
            CStr::from_ptr(
                crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER,
            )
        },
        ty: SDL_PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub static METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX",
        value: unsafe {
            CStr::from_ptr(
                crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER,
            )
        },
        ty: SDL_PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub static METADATA_SDL_PROP_RENDERER_NAME_STRING: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_NAME_STRING",
    short_name: "RENDERER_NAME",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_NAME_STRING) },
    ty: SDL_PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_WINDOW_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_WINDOW_POINTER",
    short_name: "RENDERER_WINDOW",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_WINDOW_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_SURFACE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_SURFACE_POINTER",
    short_name: "RENDERER_SURFACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_SURFACE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VSYNC_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VSYNC_NUMBER",
    short_name: "RENDERER_VSYNC",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VSYNC_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER",
    short_name: "RENDERER_MAX_TEXTURE_SIZE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER",
    short_name: "RENDERER_TEXTURE_FORMATS",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER",
    short_name: "RENDERER_OUTPUT_COLORSPACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN",
    short_name: "RENDERER_HDR_ENABLED",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN) },
    ty: SDL_PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT",
    short_name: "RENDERER_SDR_WHITE_POINT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT",
    short_name: "RENDERER_HDR_HEADROOM",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D9_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D9_DEVICE_POINTER",
    short_name: "RENDERER_D3D9_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D9_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D11_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D11_DEVICE_POINTER",
    short_name: "RENDERER_D3D11_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D11_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER",
    short_name: "RENDERER_D3D11_SWAPCHAIN",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D12_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_DEVICE_POINTER",
    short_name: "RENDERER_D3D12_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D12_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER",
    short_name: "RENDERER_D3D12_SWAPCHAIN",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER",
    short_name: "RENDERER_D3D12_COMMAND_QUEUE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER",
    short_name: "RENDERER_VULKAN_INSTANCE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER",
    short_name: "RENDERER_VULKAN_SURFACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER",
    short_name: "RENDERER_VULKAN_PHYSICAL_DEVICE",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER",
    short_name: "RENDERER_VULKAN_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX",
        value: unsafe {
            CStr::from_ptr(
                crate::render::SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER,
            )
        },
        ty: SDL_PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub static METADATA_SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX",
        value: unsafe {
            CStr::from_ptr(
                crate::render::SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER,
            )
        },
        ty: SDL_PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub static METADATA_SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER",
    short_name: "RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_RENDERER_GPU_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_GPU_DEVICE_POINTER",
    short_name: "RENDERER_GPU_DEVICE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_RENDERER_GPU_DEVICE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER",
    short_name: "TEXTURE_CREATE_COLORSPACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER",
    short_name: "TEXTURE_CREATE_FORMAT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER",
    short_name: "TEXTURE_CREATE_ACCESS",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER",
    short_name: "TEXTURE_CREATE_WIDTH",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER",
    short_name: "TEXTURE_CREATE_HEIGHT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT",
    short_name: "TEXTURE_CREATE_SDR_WHITE_POINT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT",
    short_name: "TEXTURE_CREATE_HDR_HEADROOM",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE_U",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE_V",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE_U",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE_V",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER",
    short_name: "TEXTURE_CREATE_METAL_PIXELBUFFER",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER)
    },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_UV",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_U",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_V",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_UV",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_U",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_V",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_VULKAN_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_COLORSPACE_NUMBER",
    short_name: "TEXTURE_COLORSPACE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_COLORSPACE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_FORMAT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_FORMAT_NUMBER",
    short_name: "TEXTURE_FORMAT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_FORMAT_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_ACCESS_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_ACCESS_NUMBER",
    short_name: "TEXTURE_ACCESS",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_ACCESS_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_WIDTH_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_WIDTH_NUMBER",
    short_name: "TEXTURE_WIDTH",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_WIDTH_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_HEIGHT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_HEIGHT_NUMBER",
    short_name: "TEXTURE_HEIGHT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_HEIGHT_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT",
    short_name: "TEXTURE_SDR_WHITE_POINT",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT",
    short_name: "TEXTURE_HDR_HEADROOM",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE_U",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE_V",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE_U",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE_V",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER) },
    ty: SDL_PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_UV",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_U",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_V",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_TARGET",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT",
    short_name: "TEXTURE_OPENGL_TEX_W",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT",
    short_name: "TEXTURE_OPENGL_TEX_H",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT) },
    ty: SDL_PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_UV",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_U",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_V",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_TARGET",
    value: unsafe {
        CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER)
    },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER",
    short_name: "TEXTURE_VULKAN_TEXTURE",
    value: unsafe { CStr::from_ptr(crate::render::SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER) },
    ty: SDL_PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub static METADATA_SDL_TextureAccess: Group = Group {
    module: "render",
    kind: GroupKind::Enum,
    name: "SDL_TextureAccess",
    short_name: "TextureAccess",
    doc: Some("The access pattern allowed for a texture.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_TEXTUREACCESS_STATIC",
            short_name: "STATIC",
            doc: Some("Changes rarely, not lockable\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTUREACCESS_STREAMING",
            short_name: "STREAMING",
            doc: Some("Changes frequently, lockable\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTUREACCESS_TARGET",
            short_name: "TARGET",
            doc: Some("Texture can be used as a render target\n"),
            available_since: None,
        },
    ],
};
pub static METADATA_SDL_RendererLogicalPresentation: Group = Group {
    module: "render",
    kind: GroupKind::Enum,
    name: "SDL_RendererLogicalPresentation",
    short_name: "RendererLogicalPresentation",
    doc: Some("How the logical size is mapped to the output.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_DISABLED",
            short_name: "DISABLED",
            doc: Some("There is no logical size in effect\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_STRETCH",
            short_name: "STRETCH",
            doc: Some("The rendered content is stretched to the output resolution\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_LETTERBOX",
            short_name: "LETTERBOX",
            doc: Some("The rendered content is fit to the largest dimension and the other dimension is letterboxed with black bars\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_OVERSCAN",
            short_name: "OVERSCAN",
            doc: Some("The rendered content is fit to the smallest dimension and the other dimension extends beyond the output bounds\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_INTEGER_SCALE",
            short_name: "INTEGER_SCALE",
            doc: Some("The rendered content is scaled up by integer multiples to fit the output resolution\n"),
            available_since: None,
        },
    ],
};
