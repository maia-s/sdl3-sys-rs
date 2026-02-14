//! Metadata for items in the `crate::render` module

use super::*;

pub const METADATA_SDL_PROP_RENDERER_CREATE_NAME_STRING: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_NAME_STRING",
    short_name: "RENDERER_CREATE_NAME_STRING",
    value: crate::render::SDL_PROP_RENDERER_CREATE_NAME_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_WINDOW_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_WINDOW_POINTER",
    short_name: "RENDERER_CREATE_WINDOW_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_SURFACE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_SURFACE_POINTER",
    short_name: "RENDERER_CREATE_SURFACE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER",
    short_name: "RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER",
    short_name: "RENDERER_CREATE_PRESENT_VSYNC_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_GPU_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_GPU_DEVICE_POINTER",
    short_name: "RENDERER_CREATE_GPU_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_GPU_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN",
    short_name: "RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN",
    value: crate::render::SDL_PROP_RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN",
    short_name: "RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN",
    value: crate::render::SDL_PROP_RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN",
    short_name: "RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN",
    value: crate::render::SDL_PROP_RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_INSTANCE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER",
    short_name: "RENDERER_CREATE_VULKAN_SURFACE_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER",
    short_name: "RENDERER_CREATE_VULKAN_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER,
        ty: PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub const METADATA_SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        value: crate::render::SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER,
        ty: PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub const METADATA_SDL_PROP_RENDERER_NAME_STRING: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_NAME_STRING",
    short_name: "RENDERER_NAME_STRING",
    value: crate::render::SDL_PROP_RENDERER_NAME_STRING,
    ty: PropertyType::STRING,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_WINDOW_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_WINDOW_POINTER",
    short_name: "RENDERER_WINDOW_POINTER",
    value: crate::render::SDL_PROP_RENDERER_WINDOW_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_SURFACE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_SURFACE_POINTER",
    short_name: "RENDERER_SURFACE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_SURFACE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VSYNC_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VSYNC_NUMBER",
    short_name: "RENDERER_VSYNC_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_VSYNC_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER",
    short_name: "RENDERER_MAX_TEXTURE_SIZE_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER",
    short_name: "RENDERER_TEXTURE_FORMATS_POINTER",
    value: crate::render::SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN",
    short_name: "RENDERER_TEXTURE_WRAPPING_BOOLEAN",
    value: crate::render::SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER",
    short_name: "RENDERER_OUTPUT_COLORSPACE_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN",
    short_name: "RENDERER_HDR_ENABLED_BOOLEAN",
    value: crate::render::SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT",
    short_name: "RENDERER_SDR_WHITE_POINT_FLOAT",
    value: crate::render::SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT",
    short_name: "RENDERER_HDR_HEADROOM_FLOAT",
    value: crate::render::SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D9_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D9_DEVICE_POINTER",
    short_name: "RENDERER_D3D9_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D9_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D11_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D11_DEVICE_POINTER",
    short_name: "RENDERER_D3D11_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D11_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER",
    short_name: "RENDERER_D3D11_SWAPCHAIN_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D12_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_DEVICE_POINTER",
    short_name: "RENDERER_D3D12_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D12_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER",
    short_name: "RENDERER_D3D12_SWAPCHAIN_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER",
    short_name: "RENDERER_D3D12_COMMAND_QUEUE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER",
    short_name: "RENDERER_VULKAN_INSTANCE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER",
    short_name: "RENDERER_VULKAN_SURFACE_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER",
    short_name: "RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER",
    short_name: "RENDERER_VULKAN_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER",
        value: crate::render::SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER,
        ty: PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub const METADATA_SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property =
    Property {
        module: "render",
        name: "SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        short_name: "RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER",
        value: crate::render::SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER,
        ty: PropertyType::NUMBER,
        doc: None,
        available_since: None,
    };
pub const METADATA_SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER",
    short_name: "RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER",
    value: crate::render::SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_RENDERER_GPU_DEVICE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_RENDERER_GPU_DEVICE_POINTER",
    short_name: "RENDERER_GPU_DEVICE_POINTER",
    value: crate::render::SDL_PROP_RENDERER_GPU_DEVICE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER",
    short_name: "TEXTURE_CREATE_COLORSPACE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER",
    short_name: "TEXTURE_CREATE_FORMAT_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER",
    short_name: "TEXTURE_CREATE_ACCESS_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER",
    short_name: "TEXTURE_CREATE_WIDTH_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER",
    short_name: "TEXTURE_CREATE_HEIGHT_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_PALETTE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_PALETTE_POINTER",
    short_name: "TEXTURE_CREATE_PALETTE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_PALETTE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT",
    short_name: "TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT",
    short_name: "TEXTURE_CREATE_HDR_HEADROOM_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER",
    short_name: "TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER",
    short_name: "TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER",
    short_name: "TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER",
    short_name: "TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER",
    short_name: "TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_POINTER",
    short_name: "TEXTURE_CREATE_GPU_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER",
    short_name: "TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_U_POINTER",
    short_name: "TEXTURE_CREATE_GPU_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_V_POINTER",
    short_name: "TEXTURE_CREATE_GPU_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_COLORSPACE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_COLORSPACE_NUMBER",
    short_name: "TEXTURE_COLORSPACE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_COLORSPACE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_FORMAT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_FORMAT_NUMBER",
    short_name: "TEXTURE_FORMAT_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_FORMAT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_ACCESS_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_ACCESS_NUMBER",
    short_name: "TEXTURE_ACCESS_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_ACCESS_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_WIDTH_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_WIDTH_NUMBER",
    short_name: "TEXTURE_WIDTH_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_WIDTH_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_HEIGHT_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_HEIGHT_NUMBER",
    short_name: "TEXTURE_HEIGHT_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_HEIGHT_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT",
    short_name: "TEXTURE_SDR_WHITE_POINT_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT",
    short_name: "TEXTURE_HDR_HEADROOM_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER",
    short_name: "TEXTURE_D3D11_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER",
    short_name: "TEXTURE_D3D12_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_UV_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_U_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_V_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER",
    short_name: "TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT",
    short_name: "TEXTURE_OPENGL_TEX_W_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT",
    short_name: "TEXTURE_OPENGL_TEX_H_FLOAT",
    value: crate::render::SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT,
    ty: PropertyType::FLOAT,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_U_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_V_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER",
    short_name: "TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER",
    short_name: "TEXTURE_VULKAN_TEXTURE_NUMBER",
    value: crate::render::SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER,
    ty: PropertyType::NUMBER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER",
    short_name: "TEXTURE_GPU_TEXTURE_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_GPU_TEXTURE_UV_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_GPU_TEXTURE_UV_POINTER",
    short_name: "TEXTURE_GPU_TEXTURE_UV_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_GPU_TEXTURE_UV_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_GPU_TEXTURE_U_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_GPU_TEXTURE_U_POINTER",
    short_name: "TEXTURE_GPU_TEXTURE_U_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_GPU_TEXTURE_U_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_PROP_TEXTURE_GPU_TEXTURE_V_POINTER: Property = Property {
    module: "render",
    name: "SDL_PROP_TEXTURE_GPU_TEXTURE_V_POINTER",
    short_name: "TEXTURE_GPU_TEXTURE_V_POINTER",
    value: crate::render::SDL_PROP_TEXTURE_GPU_TEXTURE_V_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_TextureAccess: Group = Group {
    module: "render",
    kind: GroupKind::Enum,
    name: "SDL_TextureAccess",
    short_name: "TextureAccess",
    doc: Some(
        "The access pattern allowed for a texture.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
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
pub const METADATA_SDL_TextureAddressMode: Group = Group {
    module: "render",
    kind: GroupKind::Enum,
    name: "SDL_TextureAddressMode",
    short_name: "TextureAddressMode",
    doc: Some(
        "The addressing mode for a texture when used in [`SDL_RenderGeometry()`].\n\nThis affects how texture coordinates are interpreted outside of \\[0, 1\\]\n\nTexture wrapping is always supported for power of two texture sizes, and is\nsupported for other texture sizes if\n[`SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN`] is set to true.\n\n## Availability\nThis enum is available since SDL 3.4.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
    values: &[
        GroupValue {
            name: "SDL_TEXTURE_ADDRESS_INVALID",
            short_name: "INVALID",
            doc: None,
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTURE_ADDRESS_AUTO",
            short_name: "AUTO",
            doc: Some(
                "Wrapping is enabled if texture coordinates are outside \\[0, 1\\], this is the default\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTURE_ADDRESS_CLAMP",
            short_name: "CLAMP",
            doc: Some("Texture coordinates are clamped to the \\[0, 1\\] range\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_TEXTURE_ADDRESS_WRAP",
            short_name: "WRAP",
            doc: Some("The texture is repeated (tiled)\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_RendererLogicalPresentation: Group = Group {
    module: "render",
    kind: GroupKind::Enum,
    name: "SDL_RendererLogicalPresentation",
    short_name: "RendererLogicalPresentation",
    doc: Some(
        "How the logical size is mapped to the output.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
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
            doc: Some(
                "The rendered content is fit to the largest dimension and the other dimension is letterboxed with the clear color\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_OVERSCAN",
            short_name: "OVERSCAN",
            doc: Some(
                "The rendered content is fit to the smallest dimension and the other dimension extends beyond the output bounds\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_LOGICAL_PRESENTATION_INTEGER_SCALE",
            short_name: "INTEGER_SCALE",
            doc: Some(
                "The rendered content is scaled up by integer multiples to fit the output resolution\n",
            ),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_Vertex: Struct = Struct {
    module: "render",
    kind: StructKind::Struct,
    name: "SDL_Vertex",
    doc: Some("Vertex structure.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "position",
            doc: Some("Vertex position, in [`SDL_Renderer`] coordinates\n"),
            available_since: None,
            ty: "SDL_FPoint",
        },
        Field {
            name: "color",
            doc: Some("Vertex color\n"),
            available_since: None,
            ty: "SDL_FColor",
        },
        Field {
            name: "tex_coord",
            doc: Some("Normalized texture coordinates, if needed\n"),
            available_since: None,
            ty: "SDL_FPoint",
        },
    ],
};
pub const METADATA_SDL_Texture: Struct = Struct {
    module: "render",
    kind: StructKind::Struct,
    name: "SDL_Texture",
    doc: None,
    available_since: None,
    fields: &[
        Field {
            name: "format",
            doc: Some("The format of the texture, read-only\n"),
            available_since: None,
            ty: "SDL_PixelFormat",
        },
        Field {
            name: "w",
            doc: Some("The width of the texture, read-only.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "h",
            doc: Some("The height of the texture, read-only.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "refcount",
            doc: Some("Application reference count, used when freeing texture\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
pub const METADATA_SDL_GPURenderStateCreateInfo: Struct = Struct {
    module: "render",
    kind: StructKind::Struct,
    name: "SDL_GPURenderStateCreateInfo",
    doc: Some(
        "A structure specifying the parameters of a GPU render state.\n\n## Availability\nThis struct is available since SDL 3.4.0.\n\n## See also\n- [`SDL_CreateGPURenderState`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 4, 0)),
    fields: &[
        Field {
            name: "fragment_shader",
            doc: Some("The fragment shader to use when this render state is active\n"),
            available_since: None,
            ty: "*mut SDL_GPUShader",
        },
        Field {
            name: "num_sampler_bindings",
            doc: Some(
                "The number of additional fragment samplers to bind when this render state is active\n",
            ),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "sampler_bindings",
            doc: Some("Additional fragment samplers to bind when this render state is active\n"),
            available_since: None,
            ty: "*const SDL_GPUTextureSamplerBinding",
        },
        Field {
            name: "num_storage_textures",
            doc: Some("The number of storage textures to bind when this render state is active\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "storage_textures",
            doc: Some("Storage textures to bind when this render state is active\n"),
            available_since: None,
            ty: "*const *mut SDL_GPUTexture",
        },
        Field {
            name: "num_storage_buffers",
            doc: Some("The number of storage buffers to bind when this render state is active\n"),
            available_since: None,
            ty: "Sint32",
        },
        Field {
            name: "storage_buffers",
            doc: Some("Storage buffers to bind when this render state is active\n"),
            available_since: None,
            ty: "*const *mut SDL_GPUBuffer",
        },
        Field {
            name: "props",
            doc: Some("A properties ID for extensions. Should be 0 if no extensions are needed.\n"),
            available_since: None,
            ty: "SDL_PropertiesID",
        },
    ],
};
