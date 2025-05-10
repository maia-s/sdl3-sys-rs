//! The GPU API offers a cross-platform way for apps to talk to modern graphics
//! hardware. It offers both 3D graphics and compute support, in the style of
//! Metal, Vulkan, and Direct3D 12.
//!
//! A basic workflow might be something like this:
//!
//! The app creates a GPU device with [`SDL_CreateGPUDevice()`], and assigns it to
//! a window with [`SDL_ClaimWindowForGPUDevice()`]--although strictly speaking you
//! can render offscreen entirely, perhaps for image processing, and not use a
//! window at all.
//!
//! Next, the app prepares static data (things that are created once and used
//! over and over). For example:
//!
//! - Shaders (programs that run on the GPU): use [`SDL_CreateGPUShader()`].
//! - Vertex buffers (arrays of geometry data) and other rendering data: use
//!   [`SDL_CreateGPUBuffer()`] and [`SDL_UploadToGPUBuffer()`].
//! - Textures (images): use [`SDL_CreateGPUTexture()`] and
//!   [`SDL_UploadToGPUTexture()`].
//! - Samplers (how textures should be read from): use [`SDL_CreateGPUSampler()`].
//! - Render pipelines (precalculated rendering state): use
//!   [`SDL_CreateGPUGraphicsPipeline()`]
//!
//! To render, the app creates one or more command buffers, with
//! [`SDL_AcquireGPUCommandBuffer()`]. Command buffers collect rendering
//! instructions that will be submitted to the GPU in batch. Complex scenes can
//! use multiple command buffers, maybe configured across multiple threads in
//! parallel, as long as they are submitted in the correct order, but many apps
//! will just need one command buffer per frame.
//!
//! Rendering can happen to a texture (what other APIs call a "render target")
//! or it can happen to the swapchain texture (which is just a special texture
//! that represents a window's contents). The app can use
//! [`SDL_WaitAndAcquireGPUSwapchainTexture()`] to render to the window.
//!
//! Rendering actually happens in a Render Pass, which is encoded into a
//! command buffer. One can encode multiple render passes (or alternate between
//! render and compute passes) in a single command buffer, but many apps might
//! simply need a single render pass in a single command buffer. Render Passes
//! can render to up to four color textures and one depth texture
//! simultaneously. If the set of textures being rendered to needs to change,
//! the Render Pass must be ended and a new one must be begun.
//!
//! The app calls [`SDL_BeginGPURenderPass()`]. Then it sets states it needs for
//! each draw:
//!
//! - [`SDL_BindGPUGraphicsPipeline()`]
//! - [`SDL_SetGPUViewport()`]
//! - [`SDL_BindGPUVertexBuffers()`]
//! - [`SDL_BindGPUVertexSamplers()`]
//! - etc
//!
//! Then, make the actual draw commands with these states:
//!
//! - [`SDL_DrawGPUPrimitives()`]
//! - [`SDL_DrawGPUPrimitivesIndirect()`]
//! - [`SDL_DrawGPUIndexedPrimitivesIndirect()`]
//! - etc
//!
//! After all the drawing commands for a pass are complete, the app should call
//! [`SDL_EndGPURenderPass()`]. Once a render pass ends all render-related state is
//! reset.
//!
//! The app can begin new Render Passes and make new draws in the same command
//! buffer until the entire scene is rendered.
//!
//! Once all of the render commands for the scene are complete, the app calls
//! [`SDL_SubmitGPUCommandBuffer()`] to send it to the GPU for processing.
//!
//! If the app needs to read back data from texture or buffers, the API has an
//! efficient way of doing this, provided that the app is willing to tolerate
//! some latency. When the app uses [`SDL_DownloadFromGPUTexture()`] or
//! [`SDL_DownloadFromGPUBuffer()`], submitting the command buffer with
//! [`SDL_SubmitGPUCommandBufferAndAcquireFence()`] will return a fence handle that
//! the app can poll or wait on in a thread. Once the fence indicates that the
//! command buffer is done processing, it is safe to read the downloaded data.
//! Make sure to call [`SDL_ReleaseGPUFence()`] when done with the fence.
//!
//! The API also has "compute" support. The app calls [`SDL_BeginGPUComputePass()`]
//! with compute-writeable textures and/or buffers, which can be written to in
//! a compute shader. Then it sets states it needs for the compute dispatches:
//!
//! - [`SDL_BindGPUComputePipeline()`]
//! - [`SDL_BindGPUComputeStorageBuffers()`]
//! - [`SDL_BindGPUComputeStorageTextures()`]
//!
//! Then, dispatch compute work:
//!
//! - [`SDL_DispatchGPUCompute()`]
//!
//! For advanced users, this opens up powerful GPU-driven workflows.
//!
//! Graphics and compute pipelines require the use of shaders, which as
//! mentioned above are small programs executed on the GPU. Each backend
//! (Vulkan, Metal, D3D12) requires a different shader format. When the app
//! creates the GPU device, the app lets the device know which shader formats
//! the app can provide. It will then select the appropriate backend depending
//! on the available shader formats and the backends available on the platform.
//! When creating shaders, the app must provide the correct shader format for
//! the selected backend. If you would like to learn more about why the API
//! works this way, there is a detailed
//! [blog post](https://moonside.games/posts/layers-all-the-way-down/)
//! explaining this situation.
//!
//! It is optimal for apps to pre-compile the shader formats they might use,
//! but for ease of use SDL provides a separate project,
//! [SDL_shadercross](https://github.com/libsdl-org/SDL_shadercross)
//! , for performing runtime shader cross-compilation. It also has a CLI
//! interface for offline precompilation as well.
//!
//! This is an extremely quick overview that leaves out several important
//! details. Already, though, one can see that GPU programming can be quite
//! complex! If you just need simple 2D graphics, the
//! [Render API](https://wiki.libsdl.org/SDL3/CategoryRender)
//! is much easier to use but still hardware-accelerated. That said, even for
//! 2D applications the performance benefits and expressiveness of the GPU API
//! are significant.
//!
//! The GPU API targets a feature set with a wide range of hardware support and
//! ease of portability. It is designed so that the app won't have to branch
//! itself by querying feature support. If you need cutting-edge features with
//! limited hardware support, this API is probably not for you.
//!
//! Examples demonstrating proper usage of this API can be found
//! [here](https://github.com/TheSpydog/SDL_gpu_examples)
//! .
//!
//! ## Performance considerations
//!
//! Here are some basic tips for maximizing your rendering performance.
//!
//! - Beginning a new render pass is relatively expensive. Use as few render
//!   passes as you can.
//! - Minimize the amount of state changes. For example, binding a pipeline is
//!   relatively cheap, but doing it hundreds of times when you don't need to
//!   will slow the performance significantly.
//! - Perform your data uploads as early as possible in the frame.
//! - Don't churn resources. Creating and releasing resources is expensive.
//!   It's better to create what you need up front and cache it.
//! - Don't use uniform buffers for large amounts of data (more than a matrix
//!   or so). Use a storage buffer instead.
//! - Use cycling correctly. There is a detailed explanation of cycling further
//!   below.
//! - Use culling techniques to minimize pixel writes. The less writing the GPU
//!   has to do the better. Culling can be a very advanced topic but even
//!   simple culling techniques can boost performance significantly.
//!
//! In general try to remember the golden rule of performance: doing things is
//! more expensive than not doing things. Don't Touch The Driver!
//!
//! ## FAQ
//!
//! **Question: When are you adding more advanced features, like ray tracing or
//! mesh shaders?**
//!
//! Answer: We don't have immediate plans to add more bleeding-edge features,
//! but we certainly might in the future, when these features prove worthwhile,
//! and reasonable to implement across several platforms and underlying APIs.
//! So while these things are not in the "never" category, they are definitely
//! not "near future" items either.
//!
//! **Question: Why is my shader not working?**
//!
//! Answer: A common oversight when using shaders is not properly laying out
//! the shader resources/registers correctly. The GPU API is very strict with
//! how it wants resources to be laid out and it's difficult for the API to
//! automatically validate shaders to see if they have a compatible layout. See
//! the documentation for [`SDL_CreateGPUShader()`] and
//! [`SDL_CreateGPUComputePipeline()`] for information on the expected layout.
//!
//! Another common issue is not setting the correct number of samplers,
//! textures, and buffers in [`SDL_GPUShaderCreateInfo`]. If possible use shader
//! reflection to extract the required information from the shader
//! automatically instead of manually filling in the struct's values.
//!
//! **Question: My application isn't performing very well. Is this the GPU
//! API's fault?**
//!
//! Answer: No. Long answer: The GPU API is a relatively thin layer over the
//! underlying graphics API. While it's possible that we have done something
//! inefficiently, it's very unlikely especially if you are relatively
//! inexperienced with GPU rendering. Please see the performance tips above and
//! make sure you are following them. Additionally, tools like RenderDoc can be
//! very helpful for diagnosing incorrect behavior and performance issues.
//!
//! ## System Requirements
//!
//! **Vulkan:** Supported on Windows, Linux, Nintendo Switch, and certain
//! Android devices. Requires Vulkan 1.0 with the following extensions and
//! device features:
//!
//! - `VK_KHR_swapchain`
//! - `VK_KHR_maintenance1`
//! - `independentBlend`
//! - `imageCubeArray`
//! - `depthClamp`
//! - `shaderClipDistance`
//! - `drawIndirectFirstInstance`
//!
//! **D3D12:** Supported on Windows 10 or newer, Xbox One (GDK), and Xbox
//! Series X|S (GDK). Requires a GPU that supports DirectX 12 Feature Level
//! 11_1.
//!
//! **Metal:** Supported on macOS 10.14+ and iOS/tvOS 13.0+. Hardware
//! requirements vary by operating system:
//!
//! - macOS requires an Apple Silicon or
//!   [Intel Mac2 family](https://developer.apple.com/documentation/metal/mtlfeatureset/mtlfeatureset_macos_gpufamily2_v1?language=objc)
//!   GPU
//! - iOS/tvOS requires an A9 GPU or newer
//! - iOS Simulator and tvOS Simulator are unsupported
//!
//! ## Uniform Data
//!
//! Uniforms are for passing data to shaders. The uniform data will be constant
//! across all executions of the shader.
//!
//! There are 4 available uniform slots per shader stage (where the stages are
//! vertex, fragment, and compute). Uniform data pushed to a slot on a stage
//! keeps its value throughout the command buffer until you call the relevant
//! Push function on that slot again.
//!
//! For example, you could write your vertex shaders to read a camera matrix
//! from uniform binding slot 0, push the camera matrix at the start of the
//! command buffer, and that data will be used for every subsequent draw call.
//!
//! It is valid to push uniform data during a render or compute pass.
//!
//! Uniforms are best for pushing small amounts of data. If you are pushing
//! more than a matrix or two per call you should consider using a storage
//! buffer instead.
//!
//! ## A Note On Cycling
//!
//! When using a command buffer, operations do not occur immediately - they
//! occur some time after the command buffer is submitted.
//!
//! When a resource is used in a pending or active command buffer, it is
//! considered to be "bound". When a resource is no longer used in any pending
//! or active command buffers, it is considered to be "unbound".
//!
//! If data resources are bound, it is unspecified when that data will be
//! unbound unless you acquire a fence when submitting the command buffer and
//! wait on it. However, this doesn't mean you need to track resource usage
//! manually.
//!
//! All of the functions and structs that involve writing to a resource have a
//! "cycle" bool. [`SDL_GPUTransferBuffer`], [`SDL_GPUBuffer`], and [`SDL_GPUTexture`] all
//! effectively function as ring buffers on internal resources. When cycle is
//! true, if the resource is bound, the cycle rotates to the next unbound
//! internal resource, or if none are available, a new one is created. This
//! means you don't have to worry about complex state tracking and
//! synchronization as long as cycling is correctly employed.
//!
//! For example: you can call [`SDL_MapGPUTransferBuffer()`], write texture data,
//! [`SDL_UnmapGPUTransferBuffer()`], and then [`SDL_UploadToGPUTexture()`]. The next
//! time you write texture data to the transfer buffer, if you set the cycle
//! param to true, you don't have to worry about overwriting any data that is
//! not yet uploaded.
//!
//! Another example: If you are using a texture in a render pass every frame,
//! this can cause a data dependency between frames. If you set cycle to true
//! in the [`SDL_GPUColorTargetInfo`] struct, you can prevent this data dependency.
//!
//! Cycling will never undefine already bound data. When cycling, all data in
//! the resource is considered to be undefined for subsequent commands until
//! that data is written again. You must take care not to read undefined data.
//!
//! Note that when cycling a texture, the entire texture will be cycled, even
//! if only part of the texture is used in the call, so you must consider the
//! entire texture to contain undefined data after cycling.
//!
//! You must also take care not to overwrite a section of data that has been
//! referenced in a command without cycling first. It is OK to overwrite
//! unreferenced data in a bound resource without cycling, but overwriting a
//! section of data that has already been referenced will produce unexpected
//! results.

use super::stdinc::*;

use super::pixels::*;

use super::properties::*;

use super::rect::*;

use super::surface::*;

use super::video::*;

/// Specifies the primitive topology of a graphics pipeline.
///
/// If you are using POINTLIST you must include a point size output in the
/// vertex shader.
///
/// - For HLSL compiling to SPIRV you must decorate a float output with
///   \[\[vk::builtin("PointSize")\]\].
/// - For GLSL you must set the gl_PointSize builtin.
/// - For MSL you must include a float output with the \[\[point_size\]\]
///   decorator.
///
/// Note that sized point topology is totally unsupported on D3D12. Any size
/// other than 1 will be ignored. In general, you should avoid using point
/// topology for both compatibility and performance reasons. You WILL regret
/// using it.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`TRIANGLELIST`](SDL_GPUPrimitiveType::TRIANGLELIST) | [`SDL_GPU_PRIMITIVETYPE_TRIANGLELIST`] | A series of separate triangles. |
/// | [`TRIANGLESTRIP`](SDL_GPUPrimitiveType::TRIANGLESTRIP) | [`SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP`] | A series of connected triangles. |
/// | [`LINELIST`](SDL_GPUPrimitiveType::LINELIST) | [`SDL_GPU_PRIMITIVETYPE_LINELIST`] | A series of separate lines. |
/// | [`LINESTRIP`](SDL_GPUPrimitiveType::LINESTRIP) | [`SDL_GPU_PRIMITIVETYPE_LINESTRIP`] | A series of connected lines. |
/// | [`POINTLIST`](SDL_GPUPrimitiveType::POINTLIST) | [`SDL_GPU_PRIMITIVETYPE_POINTLIST`] | A series of separate points. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUPrimitiveType(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUPrimitiveType {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUPrimitiveType> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUPrimitiveType) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUPrimitiveType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUPrimitiveType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUPrimitiveType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::TRIANGLELIST => "SDL_GPU_PRIMITIVETYPE_TRIANGLELIST",
            Self::TRIANGLESTRIP => "SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP",
            Self::LINELIST => "SDL_GPU_PRIMITIVETYPE_LINELIST",
            Self::LINESTRIP => "SDL_GPU_PRIMITIVETYPE_LINESTRIP",
            Self::POINTLIST => "SDL_GPU_PRIMITIVETYPE_POINTLIST",

            _ => return write!(f, "SDL_GPUPrimitiveType({})", self.0),
        })
    }
}

impl SDL_GPUPrimitiveType {
    /// A series of separate triangles.
    pub const TRIANGLELIST: Self = Self((0 as ::core::ffi::c_int));
    /// A series of connected triangles.
    pub const TRIANGLESTRIP: Self = Self((1 as ::core::ffi::c_int));
    /// A series of separate lines.
    pub const LINELIST: Self = Self((2 as ::core::ffi::c_int));
    /// A series of connected lines.
    pub const LINESTRIP: Self = Self((3 as ::core::ffi::c_int));
    /// A series of separate points.
    pub const POINTLIST: Self = Self((4 as ::core::ffi::c_int));
}

/// A series of separate triangles.
pub const SDL_GPU_PRIMITIVETYPE_TRIANGLELIST: SDL_GPUPrimitiveType =
    SDL_GPUPrimitiveType::TRIANGLELIST;
/// A series of connected triangles.
pub const SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP: SDL_GPUPrimitiveType =
    SDL_GPUPrimitiveType::TRIANGLESTRIP;
/// A series of separate lines.
pub const SDL_GPU_PRIMITIVETYPE_LINELIST: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::LINELIST;
/// A series of connected lines.
pub const SDL_GPU_PRIMITIVETYPE_LINESTRIP: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::LINESTRIP;
/// A series of separate points.
pub const SDL_GPU_PRIMITIVETYPE_POINTLIST: SDL_GPUPrimitiveType = SDL_GPUPrimitiveType::POINTLIST;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUPrimitiveType {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUPrimitiveType;
}

/// Specifies how the contents of a texture attached to a render pass are
/// treated at the beginning of the render pass.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPURenderPass`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`LOAD`](SDL_GPULoadOp::LOAD) | [`SDL_GPU_LOADOP_LOAD`] | The previous contents of the texture will be preserved. |
/// | [`CLEAR`](SDL_GPULoadOp::CLEAR) | [`SDL_GPU_LOADOP_CLEAR`] | The contents of the texture will be cleared to a color. |
/// | [`DONT_CARE`](SDL_GPULoadOp::DONT_CARE) | [`SDL_GPU_LOADOP_DONT_CARE`] | The previous contents of the texture need not be preserved. The contents will be undefined. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPULoadOp(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPULoadOp {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPULoadOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPULoadOp) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPULoadOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPULoadOp) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPULoadOp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::LOAD => "SDL_GPU_LOADOP_LOAD",
            Self::CLEAR => "SDL_GPU_LOADOP_CLEAR",
            Self::DONT_CARE => "SDL_GPU_LOADOP_DONT_CARE",

            _ => return write!(f, "SDL_GPULoadOp({})", self.0),
        })
    }
}

impl SDL_GPULoadOp {
    /// The previous contents of the texture will be preserved.
    pub const LOAD: Self = Self((0 as ::core::ffi::c_int));
    /// The contents of the texture will be cleared to a color.
    pub const CLEAR: Self = Self((1 as ::core::ffi::c_int));
    /// The previous contents of the texture need not be preserved. The contents will be undefined.
    pub const DONT_CARE: Self = Self((2 as ::core::ffi::c_int));
}

/// The previous contents of the texture will be preserved.
pub const SDL_GPU_LOADOP_LOAD: SDL_GPULoadOp = SDL_GPULoadOp::LOAD;
/// The contents of the texture will be cleared to a color.
pub const SDL_GPU_LOADOP_CLEAR: SDL_GPULoadOp = SDL_GPULoadOp::CLEAR;
/// The previous contents of the texture need not be preserved. The contents will be undefined.
pub const SDL_GPU_LOADOP_DONT_CARE: SDL_GPULoadOp = SDL_GPULoadOp::DONT_CARE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPULoadOp {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPULoadOp;
}

/// Specifies how the contents of a texture attached to a render pass are
/// treated at the end of the render pass.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPURenderPass`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`STORE`](SDL_GPUStoreOp::STORE) | [`SDL_GPU_STOREOP_STORE`] | The contents generated during the render pass will be written to memory. |
/// | [`DONT_CARE`](SDL_GPUStoreOp::DONT_CARE) | [`SDL_GPU_STOREOP_DONT_CARE`] | The contents generated during the render pass are not needed and may be discarded. The contents will be undefined. |
/// | [`RESOLVE`](SDL_GPUStoreOp::RESOLVE) | [`SDL_GPU_STOREOP_RESOLVE`] | The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture may then be discarded and will be undefined. |
/// | [`RESOLVE_AND_STORE`](SDL_GPUStoreOp::RESOLVE_AND_STORE) | [`SDL_GPU_STOREOP_RESOLVE_AND_STORE`] | The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture will be written to memory. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUStoreOp(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUStoreOp {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUStoreOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUStoreOp) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUStoreOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUStoreOp) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUStoreOp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::STORE => "SDL_GPU_STOREOP_STORE",
            Self::DONT_CARE => "SDL_GPU_STOREOP_DONT_CARE",
            Self::RESOLVE => "SDL_GPU_STOREOP_RESOLVE",
            Self::RESOLVE_AND_STORE => "SDL_GPU_STOREOP_RESOLVE_AND_STORE",

            _ => return write!(f, "SDL_GPUStoreOp({})", self.0),
        })
    }
}

impl SDL_GPUStoreOp {
    /// The contents generated during the render pass will be written to memory.
    pub const STORE: Self = Self((0 as ::core::ffi::c_int));
    /// The contents generated during the render pass are not needed and may be discarded. The contents will be undefined.
    pub const DONT_CARE: Self = Self((1 as ::core::ffi::c_int));
    /// The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture may then be discarded and will be undefined.
    pub const RESOLVE: Self = Self((2 as ::core::ffi::c_int));
    /// The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture will be written to memory.
    pub const RESOLVE_AND_STORE: Self = Self((3 as ::core::ffi::c_int));
}

/// The contents generated during the render pass will be written to memory.
pub const SDL_GPU_STOREOP_STORE: SDL_GPUStoreOp = SDL_GPUStoreOp::STORE;
/// The contents generated during the render pass are not needed and may be discarded. The contents will be undefined.
pub const SDL_GPU_STOREOP_DONT_CARE: SDL_GPUStoreOp = SDL_GPUStoreOp::DONT_CARE;
/// The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture may then be discarded and will be undefined.
pub const SDL_GPU_STOREOP_RESOLVE: SDL_GPUStoreOp = SDL_GPUStoreOp::RESOLVE;
/// The multisample contents generated during the render pass will be resolved to a non-multisample texture. The contents in the multisample texture will be written to memory.
pub const SDL_GPU_STOREOP_RESOLVE_AND_STORE: SDL_GPUStoreOp = SDL_GPUStoreOp::RESOLVE_AND_STORE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUStoreOp {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUStoreOp;
}

/// Specifies the size of elements in an index buffer.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`_16BIT`](SDL_GPUIndexElementSize::_16BIT) | [`SDL_GPU_INDEXELEMENTSIZE_16BIT`] | The index elements are 16-bit. |
/// | [`_32BIT`](SDL_GPUIndexElementSize::_32BIT) | [`SDL_GPU_INDEXELEMENTSIZE_32BIT`] | The index elements are 32-bit. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUIndexElementSize(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUIndexElementSize {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUIndexElementSize> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUIndexElementSize) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUIndexElementSize> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUIndexElementSize) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUIndexElementSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::_16BIT => "SDL_GPU_INDEXELEMENTSIZE_16BIT",
            Self::_32BIT => "SDL_GPU_INDEXELEMENTSIZE_32BIT",

            _ => return write!(f, "SDL_GPUIndexElementSize({})", self.0),
        })
    }
}

impl SDL_GPUIndexElementSize {
    /// The index elements are 16-bit.
    pub const _16BIT: Self = Self((0 as ::core::ffi::c_int));
    /// The index elements are 32-bit.
    pub const _32BIT: Self = Self((1 as ::core::ffi::c_int));
}

/// The index elements are 16-bit.
pub const SDL_GPU_INDEXELEMENTSIZE_16BIT: SDL_GPUIndexElementSize = SDL_GPUIndexElementSize::_16BIT;
/// The index elements are 32-bit.
pub const SDL_GPU_INDEXELEMENTSIZE_32BIT: SDL_GPUIndexElementSize = SDL_GPUIndexElementSize::_32BIT;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUIndexElementSize {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUIndexElementSize;
}

/// Specifies the pixel format of a texture.
///
/// Texture format support varies depending on driver, hardware, and usage
/// flags. In general, you should use [`SDL_GPUTextureSupportsFormat`] to query if
/// a format is supported before using it. However, there are a few guaranteed
/// formats.
///
/// FIXME: Check universal support for 32-bit component formats FIXME: Check
/// universal support for SIMULTANEOUS_READ_WRITE
///
/// For SAMPLER usage, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - B8G8R8A8_UNORM
/// - R8_UNORM
/// - R8_SNORM
/// - R8G8_UNORM
/// - R8G8_SNORM
/// - R8G8B8A8_SNORM
/// - R16_FLOAT
/// - R16G16_FLOAT
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R11G11B10_UFLOAT
/// - R8G8B8A8_UNORM_SRGB
/// - B8G8R8A8_UNORM_SRGB
/// - D16_UNORM
///
/// For COLOR_TARGET usage, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - B8G8R8A8_UNORM
/// - R8_UNORM
/// - R16_FLOAT
/// - R16G16_FLOAT
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R8_UINT
/// - R8G8_UINT
/// - R8G8B8A8_UINT
/// - R16_UINT
/// - R16G16_UINT
/// - R16G16B16A16_UINT
/// - R8_INT
/// - R8G8_INT
/// - R8G8B8A8_INT
/// - R16_INT
/// - R16G16_INT
/// - R16G16B16A16_INT
/// - R8G8B8A8_UNORM_SRGB
/// - B8G8R8A8_UNORM_SRGB
///
/// For STORAGE usages, the following formats are universally supported:
///
/// - R8G8B8A8_UNORM
/// - R8G8B8A8_SNORM
/// - R16G16B16A16_FLOAT
/// - R32_FLOAT
/// - R32G32_FLOAT
/// - R32G32B32A32_FLOAT
/// - R8G8B8A8_UINT
/// - R16G16B16A16_UINT
/// - R8G8B8A8_INT
/// - R16G16B16A16_INT
///
/// For DEPTH_STENCIL_TARGET usage, the following formats are universally
/// supported:
///
/// - D16_UNORM
/// - Either (but not necessarily both!) D24_UNORM or D32_FLOAT
/// - Either (but not necessarily both!) D24_UNORM_S8_UINT or D32_FLOAT_S8_UINT
///
/// Unless D16_UNORM is sufficient for your purposes, always check which of
/// D24/D32 is supported before creating a depth-stencil texture!
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
/// - [`SDL_GPUTextureSupportsFormat`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUTextureFormat::INVALID) | [`SDL_GPU_TEXTUREFORMAT_INVALID`] | |
/// | [`A8_UNORM`](SDL_GPUTextureFormat::A8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_A8_UNORM`] | |
/// | [`R8_UNORM`](SDL_GPUTextureFormat::R8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R8_UNORM`] | |
/// | [`R8G8_UNORM`](SDL_GPUTextureFormat::R8G8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R8G8_UNORM`] | |
/// | [`R8G8B8A8_UNORM`](SDL_GPUTextureFormat::R8G8B8A8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM`] | |
/// | [`R16_UNORM`](SDL_GPUTextureFormat::R16_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R16_UNORM`] | |
/// | [`R16G16_UNORM`](SDL_GPUTextureFormat::R16G16_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R16G16_UNORM`] | |
/// | [`R16G16B16A16_UNORM`](SDL_GPUTextureFormat::R16G16B16A16_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM`] | |
/// | [`R10G10B10A2_UNORM`](SDL_GPUTextureFormat::R10G10B10A2_UNORM) | [`SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM`] | |
/// | [`B5G6R5_UNORM`](SDL_GPUTextureFormat::B5G6R5_UNORM) | [`SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM`] | |
/// | [`B5G5R5A1_UNORM`](SDL_GPUTextureFormat::B5G5R5A1_UNORM) | [`SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM`] | |
/// | [`B4G4R4A4_UNORM`](SDL_GPUTextureFormat::B4G4R4A4_UNORM) | [`SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM`] | |
/// | [`B8G8R8A8_UNORM`](SDL_GPUTextureFormat::B8G8R8A8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM`] | |
/// | [`BC1_RGBA_UNORM`](SDL_GPUTextureFormat::BC1_RGBA_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM`] | |
/// | [`BC2_RGBA_UNORM`](SDL_GPUTextureFormat::BC2_RGBA_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM`] | |
/// | [`BC3_RGBA_UNORM`](SDL_GPUTextureFormat::BC3_RGBA_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM`] | |
/// | [`BC4_R_UNORM`](SDL_GPUTextureFormat::BC4_R_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM`] | |
/// | [`BC5_RG_UNORM`](SDL_GPUTextureFormat::BC5_RG_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM`] | |
/// | [`BC7_RGBA_UNORM`](SDL_GPUTextureFormat::BC7_RGBA_UNORM) | [`SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM`] | |
/// | [`BC6H_RGB_FLOAT`](SDL_GPUTextureFormat::BC6H_RGB_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT`] | |
/// | [`BC6H_RGB_UFLOAT`](SDL_GPUTextureFormat::BC6H_RGB_UFLOAT) | [`SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT`] | |
/// | [`R8_SNORM`](SDL_GPUTextureFormat::R8_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R8_SNORM`] | |
/// | [`R8G8_SNORM`](SDL_GPUTextureFormat::R8G8_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R8G8_SNORM`] | |
/// | [`R8G8B8A8_SNORM`](SDL_GPUTextureFormat::R8G8B8A8_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM`] | |
/// | [`R16_SNORM`](SDL_GPUTextureFormat::R16_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R16_SNORM`] | |
/// | [`R16G16_SNORM`](SDL_GPUTextureFormat::R16G16_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R16G16_SNORM`] | |
/// | [`R16G16B16A16_SNORM`](SDL_GPUTextureFormat::R16G16B16A16_SNORM) | [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM`] | |
/// | [`R16_FLOAT`](SDL_GPUTextureFormat::R16_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R16_FLOAT`] | |
/// | [`R16G16_FLOAT`](SDL_GPUTextureFormat::R16G16_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT`] | |
/// | [`R16G16B16A16_FLOAT`](SDL_GPUTextureFormat::R16G16B16A16_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT`] | |
/// | [`R32_FLOAT`](SDL_GPUTextureFormat::R32_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R32_FLOAT`] | |
/// | [`R32G32_FLOAT`](SDL_GPUTextureFormat::R32G32_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT`] | |
/// | [`R32G32B32A32_FLOAT`](SDL_GPUTextureFormat::R32G32B32A32_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT`] | |
/// | [`R11G11B10_UFLOAT`](SDL_GPUTextureFormat::R11G11B10_UFLOAT) | [`SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT`] | |
/// | [`R8_UINT`](SDL_GPUTextureFormat::R8_UINT) | [`SDL_GPU_TEXTUREFORMAT_R8_UINT`] | |
/// | [`R8G8_UINT`](SDL_GPUTextureFormat::R8G8_UINT) | [`SDL_GPU_TEXTUREFORMAT_R8G8_UINT`] | |
/// | [`R8G8B8A8_UINT`](SDL_GPUTextureFormat::R8G8B8A8_UINT) | [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT`] | |
/// | [`R16_UINT`](SDL_GPUTextureFormat::R16_UINT) | [`SDL_GPU_TEXTUREFORMAT_R16_UINT`] | |
/// | [`R16G16_UINT`](SDL_GPUTextureFormat::R16G16_UINT) | [`SDL_GPU_TEXTUREFORMAT_R16G16_UINT`] | |
/// | [`R16G16B16A16_UINT`](SDL_GPUTextureFormat::R16G16B16A16_UINT) | [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT`] | |
/// | [`R32_UINT`](SDL_GPUTextureFormat::R32_UINT) | [`SDL_GPU_TEXTUREFORMAT_R32_UINT`] | |
/// | [`R32G32_UINT`](SDL_GPUTextureFormat::R32G32_UINT) | [`SDL_GPU_TEXTUREFORMAT_R32G32_UINT`] | |
/// | [`R32G32B32A32_UINT`](SDL_GPUTextureFormat::R32G32B32A32_UINT) | [`SDL_GPU_TEXTUREFORMAT_R32G32B32A32_UINT`] | |
/// | [`R8_INT`](SDL_GPUTextureFormat::R8_INT) | [`SDL_GPU_TEXTUREFORMAT_R8_INT`] | |
/// | [`R8G8_INT`](SDL_GPUTextureFormat::R8G8_INT) | [`SDL_GPU_TEXTUREFORMAT_R8G8_INT`] | |
/// | [`R8G8B8A8_INT`](SDL_GPUTextureFormat::R8G8B8A8_INT) | [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT`] | |
/// | [`R16_INT`](SDL_GPUTextureFormat::R16_INT) | [`SDL_GPU_TEXTUREFORMAT_R16_INT`] | |
/// | [`R16G16_INT`](SDL_GPUTextureFormat::R16G16_INT) | [`SDL_GPU_TEXTUREFORMAT_R16G16_INT`] | |
/// | [`R16G16B16A16_INT`](SDL_GPUTextureFormat::R16G16B16A16_INT) | [`SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT`] | |
/// | [`R32_INT`](SDL_GPUTextureFormat::R32_INT) | [`SDL_GPU_TEXTUREFORMAT_R32_INT`] | |
/// | [`R32G32_INT`](SDL_GPUTextureFormat::R32G32_INT) | [`SDL_GPU_TEXTUREFORMAT_R32G32_INT`] | |
/// | [`R32G32B32A32_INT`](SDL_GPUTextureFormat::R32G32B32A32_INT) | [`SDL_GPU_TEXTUREFORMAT_R32G32B32A32_INT`] | |
/// | [`R8G8B8A8_UNORM_SRGB`](SDL_GPUTextureFormat::R8G8B8A8_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB`] | |
/// | [`B8G8R8A8_UNORM_SRGB`](SDL_GPUTextureFormat::B8G8R8A8_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB`] | |
/// | [`BC1_RGBA_UNORM_SRGB`](SDL_GPUTextureFormat::BC1_RGBA_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB`] | |
/// | [`BC2_RGBA_UNORM_SRGB`](SDL_GPUTextureFormat::BC2_RGBA_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB`] | |
/// | [`BC3_RGBA_UNORM_SRGB`](SDL_GPUTextureFormat::BC3_RGBA_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB`] | |
/// | [`BC7_RGBA_UNORM_SRGB`](SDL_GPUTextureFormat::BC7_RGBA_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB`] | |
/// | [`D16_UNORM`](SDL_GPUTextureFormat::D16_UNORM) | [`SDL_GPU_TEXTUREFORMAT_D16_UNORM`] | |
/// | [`D24_UNORM`](SDL_GPUTextureFormat::D24_UNORM) | [`SDL_GPU_TEXTUREFORMAT_D24_UNORM`] | |
/// | [`D32_FLOAT`](SDL_GPUTextureFormat::D32_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_D32_FLOAT`] | |
/// | [`D24_UNORM_S8_UINT`](SDL_GPUTextureFormat::D24_UNORM_S8_UINT) | [`SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT`] | |
/// | [`D32_FLOAT_S8_UINT`](SDL_GPUTextureFormat::D32_FLOAT_S8_UINT) | [`SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT`] | |
/// | [`ASTC_4x4_UNORM`](SDL_GPUTextureFormat::ASTC_4x4_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM`] | |
/// | [`ASTC_5x4_UNORM`](SDL_GPUTextureFormat::ASTC_5x4_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM`] | |
/// | [`ASTC_5x5_UNORM`](SDL_GPUTextureFormat::ASTC_5x5_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM`] | |
/// | [`ASTC_6x5_UNORM`](SDL_GPUTextureFormat::ASTC_6x5_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM`] | |
/// | [`ASTC_6x6_UNORM`](SDL_GPUTextureFormat::ASTC_6x6_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM`] | |
/// | [`ASTC_8x5_UNORM`](SDL_GPUTextureFormat::ASTC_8x5_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM`] | |
/// | [`ASTC_8x6_UNORM`](SDL_GPUTextureFormat::ASTC_8x6_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM`] | |
/// | [`ASTC_8x8_UNORM`](SDL_GPUTextureFormat::ASTC_8x8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM`] | |
/// | [`ASTC_10x5_UNORM`](SDL_GPUTextureFormat::ASTC_10x5_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM`] | |
/// | [`ASTC_10x6_UNORM`](SDL_GPUTextureFormat::ASTC_10x6_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM`] | |
/// | [`ASTC_10x8_UNORM`](SDL_GPUTextureFormat::ASTC_10x8_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM`] | |
/// | [`ASTC_10x10_UNORM`](SDL_GPUTextureFormat::ASTC_10x10_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM`] | |
/// | [`ASTC_12x10_UNORM`](SDL_GPUTextureFormat::ASTC_12x10_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM`] | |
/// | [`ASTC_12x12_UNORM`](SDL_GPUTextureFormat::ASTC_12x12_UNORM) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM`] | |
/// | [`ASTC_4x4_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_4x4_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM_SRGB`] | |
/// | [`ASTC_5x4_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_5x4_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM_SRGB`] | |
/// | [`ASTC_5x5_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_5x5_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM_SRGB`] | |
/// | [`ASTC_6x5_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_6x5_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM_SRGB`] | |
/// | [`ASTC_6x6_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_6x6_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM_SRGB`] | |
/// | [`ASTC_8x5_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_8x5_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM_SRGB`] | |
/// | [`ASTC_8x6_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_8x6_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM_SRGB`] | |
/// | [`ASTC_8x8_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_8x8_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM_SRGB`] | |
/// | [`ASTC_10x5_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_10x5_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM_SRGB`] | |
/// | [`ASTC_10x6_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_10x6_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM_SRGB`] | |
/// | [`ASTC_10x8_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_10x8_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM_SRGB`] | |
/// | [`ASTC_10x10_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_10x10_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM_SRGB`] | |
/// | [`ASTC_12x10_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_12x10_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM_SRGB`] | |
/// | [`ASTC_12x12_UNORM_SRGB`](SDL_GPUTextureFormat::ASTC_12x12_UNORM_SRGB) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM_SRGB`] | |
/// | [`ASTC_4x4_FLOAT`](SDL_GPUTextureFormat::ASTC_4x4_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_4x4_FLOAT`] | |
/// | [`ASTC_5x4_FLOAT`](SDL_GPUTextureFormat::ASTC_5x4_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x4_FLOAT`] | |
/// | [`ASTC_5x5_FLOAT`](SDL_GPUTextureFormat::ASTC_5x5_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_5x5_FLOAT`] | |
/// | [`ASTC_6x5_FLOAT`](SDL_GPUTextureFormat::ASTC_6x5_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x5_FLOAT`] | |
/// | [`ASTC_6x6_FLOAT`](SDL_GPUTextureFormat::ASTC_6x6_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_6x6_FLOAT`] | |
/// | [`ASTC_8x5_FLOAT`](SDL_GPUTextureFormat::ASTC_8x5_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x5_FLOAT`] | |
/// | [`ASTC_8x6_FLOAT`](SDL_GPUTextureFormat::ASTC_8x6_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x6_FLOAT`] | |
/// | [`ASTC_8x8_FLOAT`](SDL_GPUTextureFormat::ASTC_8x8_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_8x8_FLOAT`] | |
/// | [`ASTC_10x5_FLOAT`](SDL_GPUTextureFormat::ASTC_10x5_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x5_FLOAT`] | |
/// | [`ASTC_10x6_FLOAT`](SDL_GPUTextureFormat::ASTC_10x6_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x6_FLOAT`] | |
/// | [`ASTC_10x8_FLOAT`](SDL_GPUTextureFormat::ASTC_10x8_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x8_FLOAT`] | |
/// | [`ASTC_10x10_FLOAT`](SDL_GPUTextureFormat::ASTC_10x10_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_10x10_FLOAT`] | |
/// | [`ASTC_12x10_FLOAT`](SDL_GPUTextureFormat::ASTC_12x10_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x10_FLOAT`] | |
/// | [`ASTC_12x12_FLOAT`](SDL_GPUTextureFormat::ASTC_12x12_FLOAT) | [`SDL_GPU_TEXTUREFORMAT_ASTC_12x12_FLOAT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUTextureFormat(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUTextureFormat {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUTextureFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUTextureFormat) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUTextureFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUTextureFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUTextureFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_TEXTUREFORMAT_INVALID",
            Self::A8_UNORM => "SDL_GPU_TEXTUREFORMAT_A8_UNORM",
            Self::R8_UNORM => "SDL_GPU_TEXTUREFORMAT_R8_UNORM",
            Self::R8G8_UNORM => "SDL_GPU_TEXTUREFORMAT_R8G8_UNORM",
            Self::R8G8B8A8_UNORM => "SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM",
            Self::R16_UNORM => "SDL_GPU_TEXTUREFORMAT_R16_UNORM",
            Self::R16G16_UNORM => "SDL_GPU_TEXTUREFORMAT_R16G16_UNORM",
            Self::R16G16B16A16_UNORM => "SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM",
            Self::R10G10B10A2_UNORM => "SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM",
            Self::B5G6R5_UNORM => "SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM",
            Self::B5G5R5A1_UNORM => "SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM",
            Self::B4G4R4A4_UNORM => "SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM",
            Self::B8G8R8A8_UNORM => "SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM",
            Self::BC1_RGBA_UNORM => "SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM",
            Self::BC2_RGBA_UNORM => "SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM",
            Self::BC3_RGBA_UNORM => "SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM",
            Self::BC4_R_UNORM => "SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM",
            Self::BC5_RG_UNORM => "SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM",
            Self::BC7_RGBA_UNORM => "SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM",
            Self::BC6H_RGB_FLOAT => "SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT",
            Self::BC6H_RGB_UFLOAT => "SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT",
            Self::R8_SNORM => "SDL_GPU_TEXTUREFORMAT_R8_SNORM",
            Self::R8G8_SNORM => "SDL_GPU_TEXTUREFORMAT_R8G8_SNORM",
            Self::R8G8B8A8_SNORM => "SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM",
            Self::R16_SNORM => "SDL_GPU_TEXTUREFORMAT_R16_SNORM",
            Self::R16G16_SNORM => "SDL_GPU_TEXTUREFORMAT_R16G16_SNORM",
            Self::R16G16B16A16_SNORM => "SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM",
            Self::R16_FLOAT => "SDL_GPU_TEXTUREFORMAT_R16_FLOAT",
            Self::R16G16_FLOAT => "SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT",
            Self::R16G16B16A16_FLOAT => "SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT",
            Self::R32_FLOAT => "SDL_GPU_TEXTUREFORMAT_R32_FLOAT",
            Self::R32G32_FLOAT => "SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT",
            Self::R32G32B32A32_FLOAT => "SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT",
            Self::R11G11B10_UFLOAT => "SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT",
            Self::R8_UINT => "SDL_GPU_TEXTUREFORMAT_R8_UINT",
            Self::R8G8_UINT => "SDL_GPU_TEXTUREFORMAT_R8G8_UINT",
            Self::R8G8B8A8_UINT => "SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT",
            Self::R16_UINT => "SDL_GPU_TEXTUREFORMAT_R16_UINT",
            Self::R16G16_UINT => "SDL_GPU_TEXTUREFORMAT_R16G16_UINT",
            Self::R16G16B16A16_UINT => "SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT",
            Self::R32_UINT => "SDL_GPU_TEXTUREFORMAT_R32_UINT",
            Self::R32G32_UINT => "SDL_GPU_TEXTUREFORMAT_R32G32_UINT",
            Self::R32G32B32A32_UINT => "SDL_GPU_TEXTUREFORMAT_R32G32B32A32_UINT",
            Self::R8_INT => "SDL_GPU_TEXTUREFORMAT_R8_INT",
            Self::R8G8_INT => "SDL_GPU_TEXTUREFORMAT_R8G8_INT",
            Self::R8G8B8A8_INT => "SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT",
            Self::R16_INT => "SDL_GPU_TEXTUREFORMAT_R16_INT",
            Self::R16G16_INT => "SDL_GPU_TEXTUREFORMAT_R16G16_INT",
            Self::R16G16B16A16_INT => "SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT",
            Self::R32_INT => "SDL_GPU_TEXTUREFORMAT_R32_INT",
            Self::R32G32_INT => "SDL_GPU_TEXTUREFORMAT_R32G32_INT",
            Self::R32G32B32A32_INT => "SDL_GPU_TEXTUREFORMAT_R32G32B32A32_INT",
            Self::R8G8B8A8_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB",
            Self::B8G8R8A8_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB",
            Self::BC1_RGBA_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB",
            Self::BC2_RGBA_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB",
            Self::BC3_RGBA_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB",
            Self::BC7_RGBA_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB",
            Self::D16_UNORM => "SDL_GPU_TEXTUREFORMAT_D16_UNORM",
            Self::D24_UNORM => "SDL_GPU_TEXTUREFORMAT_D24_UNORM",
            Self::D32_FLOAT => "SDL_GPU_TEXTUREFORMAT_D32_FLOAT",
            Self::D24_UNORM_S8_UINT => "SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT",
            Self::D32_FLOAT_S8_UINT => "SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT",
            Self::ASTC_4x4_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM",
            Self::ASTC_5x4_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM",
            Self::ASTC_5x5_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM",
            Self::ASTC_6x5_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM",
            Self::ASTC_6x6_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM",
            Self::ASTC_8x5_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM",
            Self::ASTC_8x6_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM",
            Self::ASTC_8x8_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM",
            Self::ASTC_10x5_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM",
            Self::ASTC_10x6_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM",
            Self::ASTC_10x8_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM",
            Self::ASTC_10x10_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM",
            Self::ASTC_12x10_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM",
            Self::ASTC_12x12_UNORM => "SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM",
            Self::ASTC_4x4_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM_SRGB",
            Self::ASTC_5x4_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM_SRGB",
            Self::ASTC_5x5_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM_SRGB",
            Self::ASTC_6x5_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM_SRGB",
            Self::ASTC_6x6_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM_SRGB",
            Self::ASTC_8x5_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM_SRGB",
            Self::ASTC_8x6_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM_SRGB",
            Self::ASTC_8x8_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM_SRGB",
            Self::ASTC_10x5_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM_SRGB",
            Self::ASTC_10x6_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM_SRGB",
            Self::ASTC_10x8_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM_SRGB",
            Self::ASTC_10x10_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM_SRGB",
            Self::ASTC_12x10_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM_SRGB",
            Self::ASTC_12x12_UNORM_SRGB => "SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM_SRGB",
            Self::ASTC_4x4_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_4x4_FLOAT",
            Self::ASTC_5x4_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_5x4_FLOAT",
            Self::ASTC_5x5_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_5x5_FLOAT",
            Self::ASTC_6x5_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_6x5_FLOAT",
            Self::ASTC_6x6_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_6x6_FLOAT",
            Self::ASTC_8x5_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_8x5_FLOAT",
            Self::ASTC_8x6_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_8x6_FLOAT",
            Self::ASTC_8x8_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_8x8_FLOAT",
            Self::ASTC_10x5_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_10x5_FLOAT",
            Self::ASTC_10x6_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_10x6_FLOAT",
            Self::ASTC_10x8_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_10x8_FLOAT",
            Self::ASTC_10x10_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_10x10_FLOAT",
            Self::ASTC_12x10_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_12x10_FLOAT",
            Self::ASTC_12x12_FLOAT => "SDL_GPU_TEXTUREFORMAT_ASTC_12x12_FLOAT",

            _ => return write!(f, "SDL_GPUTextureFormat({})", self.0),
        })
    }
}

impl SDL_GPUTextureFormat {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    pub const A8_UNORM: Self = Self((1 as ::core::ffi::c_int));
    pub const R8_UNORM: Self = Self((2 as ::core::ffi::c_int));
    pub const R8G8_UNORM: Self = Self((3 as ::core::ffi::c_int));
    pub const R8G8B8A8_UNORM: Self = Self((4 as ::core::ffi::c_int));
    pub const R16_UNORM: Self = Self((5 as ::core::ffi::c_int));
    pub const R16G16_UNORM: Self = Self((6 as ::core::ffi::c_int));
    pub const R16G16B16A16_UNORM: Self = Self((7 as ::core::ffi::c_int));
    pub const R10G10B10A2_UNORM: Self = Self((8 as ::core::ffi::c_int));
    pub const B5G6R5_UNORM: Self = Self((9 as ::core::ffi::c_int));
    pub const B5G5R5A1_UNORM: Self = Self((10 as ::core::ffi::c_int));
    pub const B4G4R4A4_UNORM: Self = Self((11 as ::core::ffi::c_int));
    pub const B8G8R8A8_UNORM: Self = Self((12 as ::core::ffi::c_int));
    pub const BC1_RGBA_UNORM: Self = Self((13 as ::core::ffi::c_int));
    pub const BC2_RGBA_UNORM: Self = Self((14 as ::core::ffi::c_int));
    pub const BC3_RGBA_UNORM: Self = Self((15 as ::core::ffi::c_int));
    pub const BC4_R_UNORM: Self = Self((16 as ::core::ffi::c_int));
    pub const BC5_RG_UNORM: Self = Self((17 as ::core::ffi::c_int));
    pub const BC7_RGBA_UNORM: Self = Self((18 as ::core::ffi::c_int));
    pub const BC6H_RGB_FLOAT: Self = Self((19 as ::core::ffi::c_int));
    pub const BC6H_RGB_UFLOAT: Self = Self((20 as ::core::ffi::c_int));
    pub const R8_SNORM: Self = Self((21 as ::core::ffi::c_int));
    pub const R8G8_SNORM: Self = Self((22 as ::core::ffi::c_int));
    pub const R8G8B8A8_SNORM: Self = Self((23 as ::core::ffi::c_int));
    pub const R16_SNORM: Self = Self((24 as ::core::ffi::c_int));
    pub const R16G16_SNORM: Self = Self((25 as ::core::ffi::c_int));
    pub const R16G16B16A16_SNORM: Self = Self((26 as ::core::ffi::c_int));
    pub const R16_FLOAT: Self = Self((27 as ::core::ffi::c_int));
    pub const R16G16_FLOAT: Self = Self((28 as ::core::ffi::c_int));
    pub const R16G16B16A16_FLOAT: Self = Self((29 as ::core::ffi::c_int));
    pub const R32_FLOAT: Self = Self((30 as ::core::ffi::c_int));
    pub const R32G32_FLOAT: Self = Self((31 as ::core::ffi::c_int));
    pub const R32G32B32A32_FLOAT: Self = Self((32 as ::core::ffi::c_int));
    pub const R11G11B10_UFLOAT: Self = Self((33 as ::core::ffi::c_int));
    pub const R8_UINT: Self = Self((34 as ::core::ffi::c_int));
    pub const R8G8_UINT: Self = Self((35 as ::core::ffi::c_int));
    pub const R8G8B8A8_UINT: Self = Self((36 as ::core::ffi::c_int));
    pub const R16_UINT: Self = Self((37 as ::core::ffi::c_int));
    pub const R16G16_UINT: Self = Self((38 as ::core::ffi::c_int));
    pub const R16G16B16A16_UINT: Self = Self((39 as ::core::ffi::c_int));
    pub const R32_UINT: Self = Self((40 as ::core::ffi::c_int));
    pub const R32G32_UINT: Self = Self((41 as ::core::ffi::c_int));
    pub const R32G32B32A32_UINT: Self = Self((42 as ::core::ffi::c_int));
    pub const R8_INT: Self = Self((43 as ::core::ffi::c_int));
    pub const R8G8_INT: Self = Self((44 as ::core::ffi::c_int));
    pub const R8G8B8A8_INT: Self = Self((45 as ::core::ffi::c_int));
    pub const R16_INT: Self = Self((46 as ::core::ffi::c_int));
    pub const R16G16_INT: Self = Self((47 as ::core::ffi::c_int));
    pub const R16G16B16A16_INT: Self = Self((48 as ::core::ffi::c_int));
    pub const R32_INT: Self = Self((49 as ::core::ffi::c_int));
    pub const R32G32_INT: Self = Self((50 as ::core::ffi::c_int));
    pub const R32G32B32A32_INT: Self = Self((51 as ::core::ffi::c_int));
    pub const R8G8B8A8_UNORM_SRGB: Self = Self((52 as ::core::ffi::c_int));
    pub const B8G8R8A8_UNORM_SRGB: Self = Self((53 as ::core::ffi::c_int));
    pub const BC1_RGBA_UNORM_SRGB: Self = Self((54 as ::core::ffi::c_int));
    pub const BC2_RGBA_UNORM_SRGB: Self = Self((55 as ::core::ffi::c_int));
    pub const BC3_RGBA_UNORM_SRGB: Self = Self((56 as ::core::ffi::c_int));
    pub const BC7_RGBA_UNORM_SRGB: Self = Self((57 as ::core::ffi::c_int));
    pub const D16_UNORM: Self = Self((58 as ::core::ffi::c_int));
    pub const D24_UNORM: Self = Self((59 as ::core::ffi::c_int));
    pub const D32_FLOAT: Self = Self((60 as ::core::ffi::c_int));
    pub const D24_UNORM_S8_UINT: Self = Self((61 as ::core::ffi::c_int));
    pub const D32_FLOAT_S8_UINT: Self = Self((62 as ::core::ffi::c_int));
    pub const ASTC_4x4_UNORM: Self = Self((63 as ::core::ffi::c_int));
    pub const ASTC_5x4_UNORM: Self = Self((64 as ::core::ffi::c_int));
    pub const ASTC_5x5_UNORM: Self = Self((65 as ::core::ffi::c_int));
    pub const ASTC_6x5_UNORM: Self = Self((66 as ::core::ffi::c_int));
    pub const ASTC_6x6_UNORM: Self = Self((67 as ::core::ffi::c_int));
    pub const ASTC_8x5_UNORM: Self = Self((68 as ::core::ffi::c_int));
    pub const ASTC_8x6_UNORM: Self = Self((69 as ::core::ffi::c_int));
    pub const ASTC_8x8_UNORM: Self = Self((70 as ::core::ffi::c_int));
    pub const ASTC_10x5_UNORM: Self = Self((71 as ::core::ffi::c_int));
    pub const ASTC_10x6_UNORM: Self = Self((72 as ::core::ffi::c_int));
    pub const ASTC_10x8_UNORM: Self = Self((73 as ::core::ffi::c_int));
    pub const ASTC_10x10_UNORM: Self = Self((74 as ::core::ffi::c_int));
    pub const ASTC_12x10_UNORM: Self = Self((75 as ::core::ffi::c_int));
    pub const ASTC_12x12_UNORM: Self = Self((76 as ::core::ffi::c_int));
    pub const ASTC_4x4_UNORM_SRGB: Self = Self((77 as ::core::ffi::c_int));
    pub const ASTC_5x4_UNORM_SRGB: Self = Self((78 as ::core::ffi::c_int));
    pub const ASTC_5x5_UNORM_SRGB: Self = Self((79 as ::core::ffi::c_int));
    pub const ASTC_6x5_UNORM_SRGB: Self = Self((80 as ::core::ffi::c_int));
    pub const ASTC_6x6_UNORM_SRGB: Self = Self((81 as ::core::ffi::c_int));
    pub const ASTC_8x5_UNORM_SRGB: Self = Self((82 as ::core::ffi::c_int));
    pub const ASTC_8x6_UNORM_SRGB: Self = Self((83 as ::core::ffi::c_int));
    pub const ASTC_8x8_UNORM_SRGB: Self = Self((84 as ::core::ffi::c_int));
    pub const ASTC_10x5_UNORM_SRGB: Self = Self((85 as ::core::ffi::c_int));
    pub const ASTC_10x6_UNORM_SRGB: Self = Self((86 as ::core::ffi::c_int));
    pub const ASTC_10x8_UNORM_SRGB: Self = Self((87 as ::core::ffi::c_int));
    pub const ASTC_10x10_UNORM_SRGB: Self = Self((88 as ::core::ffi::c_int));
    pub const ASTC_12x10_UNORM_SRGB: Self = Self((89 as ::core::ffi::c_int));
    pub const ASTC_12x12_UNORM_SRGB: Self = Self((90 as ::core::ffi::c_int));
    pub const ASTC_4x4_FLOAT: Self = Self((91 as ::core::ffi::c_int));
    pub const ASTC_5x4_FLOAT: Self = Self((92 as ::core::ffi::c_int));
    pub const ASTC_5x5_FLOAT: Self = Self((93 as ::core::ffi::c_int));
    pub const ASTC_6x5_FLOAT: Self = Self((94 as ::core::ffi::c_int));
    pub const ASTC_6x6_FLOAT: Self = Self((95 as ::core::ffi::c_int));
    pub const ASTC_8x5_FLOAT: Self = Self((96 as ::core::ffi::c_int));
    pub const ASTC_8x6_FLOAT: Self = Self((97 as ::core::ffi::c_int));
    pub const ASTC_8x8_FLOAT: Self = Self((98 as ::core::ffi::c_int));
    pub const ASTC_10x5_FLOAT: Self = Self((99 as ::core::ffi::c_int));
    pub const ASTC_10x6_FLOAT: Self = Self((100 as ::core::ffi::c_int));
    pub const ASTC_10x8_FLOAT: Self = Self((101 as ::core::ffi::c_int));
    pub const ASTC_10x10_FLOAT: Self = Self((102 as ::core::ffi::c_int));
    pub const ASTC_12x10_FLOAT: Self = Self((103 as ::core::ffi::c_int));
    pub const ASTC_12x12_FLOAT: Self = Self((104 as ::core::ffi::c_int));
}

pub const SDL_GPU_TEXTUREFORMAT_INVALID: SDL_GPUTextureFormat = SDL_GPUTextureFormat::INVALID;
pub const SDL_GPU_TEXTUREFORMAT_A8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R8G8B8A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16B16A16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R10G10B10A2_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::B5G6R5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::B5G5R5A1_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::B4G4R4A4_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::B8G8R8A8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC1_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC2_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC3_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC4_R_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC5_RG_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC7_RGBA_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC6H_RGB_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC6H_RGB_UFLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R8_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R8G8B8A8_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_SNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_SNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16B16A16_SNORM;
pub const SDL_GPU_TEXTUREFORMAT_R16_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16B16A16_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R32G32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R32G32B32A32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R11G11B10_UFLOAT;
pub const SDL_GPU_TEXTUREFORMAT_R8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R8G8B8A8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16B16A16_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R32_UINT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R32G32_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32B32A32_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R32G32B32A32_UINT;
pub const SDL_GPU_TEXTUREFORMAT_R8_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R8G8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R8G8B8A8_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R16G16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R16G16B16A16_INT;
pub const SDL_GPU_TEXTUREFORMAT_R32_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32_INT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32_INT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::R32G32_INT;
pub const SDL_GPU_TEXTUREFORMAT_R32G32B32A32_INT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R32G32B32A32_INT;
pub const SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::R8G8B8A8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::B8G8R8A8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC1_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC2_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC3_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::BC7_RGBA_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_D16_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D16_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_D24_UNORM: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D24_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_D32_FLOAT: SDL_GPUTextureFormat = SDL_GPUTextureFormat::D32_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::D24_UNORM_S8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::D32_FLOAT_S8_UINT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_4x4_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x4_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x6_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x6_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x5_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x6_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x8_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x10_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x10_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x12_UNORM;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_4x4_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x4_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x5_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x5_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x6_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x5_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x6_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x5_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x6_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x8_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x10_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x10_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM_SRGB: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x12_UNORM_SRGB;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_4x4_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_4x4_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x4_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x4_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_5x5_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_5x5_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x5_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x5_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_6x6_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_6x6_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x5_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x5_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x6_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x6_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_8x8_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_8x8_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x5_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x5_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x6_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x6_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x8_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x8_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_10x10_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_10x10_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x10_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x10_FLOAT;
pub const SDL_GPU_TEXTUREFORMAT_ASTC_12x12_FLOAT: SDL_GPUTextureFormat =
    SDL_GPUTextureFormat::ASTC_12x12_FLOAT;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUTextureFormat {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUTextureFormat;
}

/// Specifies how a texture is intended to be used by the client.
///
/// A texture must have at least one usage flag. Note that some usage flag
/// combinations are invalid.
///
/// With regards to compute storage usage, READ | WRITE means that you can have
/// shader A that only writes into the texture and shader B that only reads
/// from the texture and bind the same texture to either shader respectively.
/// SIMULTANEOUS means that you can do reads and writes within the same shader
/// or compute pass. It also implies that atomic ops can be used, since those
/// are read-modify-write operations. If you use SIMULTANEOUS, you are
/// responsible for avoiding data races, as there is no data synchronization
/// within a compute pass. Note that SIMULTANEOUS usage is only supported by a
/// limited number of texture formats.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`SAMPLER`](SDL_GPUTextureUsageFlags::SAMPLER) | [`SDL_GPU_TEXTUREUSAGE_SAMPLER`] | Texture supports sampling. |
/// | [`COLOR_TARGET`](SDL_GPUTextureUsageFlags::COLOR_TARGET) | [`SDL_GPU_TEXTUREUSAGE_COLOR_TARGET`] | Texture is a color render target. |
/// | [`DEPTH_STENCIL_TARGET`](SDL_GPUTextureUsageFlags::DEPTH_STENCIL_TARGET) | [`SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET`] | Texture is a depth stencil target. |
/// | [`GRAPHICS_STORAGE_READ`](SDL_GPUTextureUsageFlags::GRAPHICS_STORAGE_READ) | [`SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ`] | Texture supports storage reads in graphics stages. |
/// | [`COMPUTE_STORAGE_READ`](SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_READ) | [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ`] | Texture supports storage reads in the compute stage. |
/// | [`COMPUTE_STORAGE_WRITE`](SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_WRITE) | [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_WRITE`] | Texture supports storage writes in the compute stage. |
/// | [`COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE`](SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE) | [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE`] | Texture supports reads and writes in the same compute shader. This is NOT equivalent to READ | WRITE. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_GPUTextureUsageFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_GPUTextureUsageFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUTextureUsageFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUTextureUsageFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUTextureUsageFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_GPUTextureUsageFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUTextureUsageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_GPUTextureUsageFlags(")?;
        let all_bits = all_bits | Self::SAMPLER.0;
        if (Self::SAMPLER != 0 || self.0 == 0) && *self & Self::SAMPLER == Self::SAMPLER {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "SAMPLER")?;
        }
        let all_bits = all_bits | Self::COLOR_TARGET.0;
        if (Self::COLOR_TARGET != 0 || self.0 == 0)
            && *self & Self::COLOR_TARGET == Self::COLOR_TARGET
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COLOR_TARGET")?;
        }
        let all_bits = all_bits | Self::DEPTH_STENCIL_TARGET.0;
        if (Self::DEPTH_STENCIL_TARGET != 0 || self.0 == 0)
            && *self & Self::DEPTH_STENCIL_TARGET == Self::DEPTH_STENCIL_TARGET
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "DEPTH_STENCIL_TARGET")?;
        }
        let all_bits = all_bits | Self::GRAPHICS_STORAGE_READ.0;
        if (Self::GRAPHICS_STORAGE_READ != 0 || self.0 == 0)
            && *self & Self::GRAPHICS_STORAGE_READ == Self::GRAPHICS_STORAGE_READ
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "GRAPHICS_STORAGE_READ")?;
        }
        let all_bits = all_bits | Self::COMPUTE_STORAGE_READ.0;
        if (Self::COMPUTE_STORAGE_READ != 0 || self.0 == 0)
            && *self & Self::COMPUTE_STORAGE_READ == Self::COMPUTE_STORAGE_READ
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COMPUTE_STORAGE_READ")?;
        }
        let all_bits = all_bits | Self::COMPUTE_STORAGE_WRITE.0;
        if (Self::COMPUTE_STORAGE_WRITE != 0 || self.0 == 0)
            && *self & Self::COMPUTE_STORAGE_WRITE == Self::COMPUTE_STORAGE_WRITE
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COMPUTE_STORAGE_WRITE")?;
        }
        let all_bits = all_bits | Self::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE.0;
        if (Self::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE != 0 || self.0 == 0)
            && *self & Self::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE
                == Self::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_GPUTextureUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_GPUTextureUsageFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_GPUTextureUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_GPUTextureUsageFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_GPUTextureUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_GPUTextureUsageFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_GPUTextureUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_GPUTextureUsageFlags {
    /// Texture supports sampling.
    pub const SAMPLER: Self = Self((1_u32 as Uint32));
    /// Texture is a color render target.
    pub const COLOR_TARGET: Self = Self((2_u32 as Uint32));
    /// Texture is a depth stencil target.
    pub const DEPTH_STENCIL_TARGET: Self = Self((4_u32 as Uint32));
    /// Texture supports storage reads in graphics stages.
    pub const GRAPHICS_STORAGE_READ: Self = Self((8_u32 as Uint32));
    /// Texture supports storage reads in the compute stage.
    pub const COMPUTE_STORAGE_READ: Self = Self((16_u32 as Uint32));
    /// Texture supports storage writes in the compute stage.
    pub const COMPUTE_STORAGE_WRITE: Self = Self((32_u32 as Uint32));
    /// Texture supports reads and writes in the same compute shader. This is NOT equivalent to READ | WRITE.
    pub const COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE: Self = Self((64_u32 as Uint32));
}

/// Texture supports sampling.
pub const SDL_GPU_TEXTUREUSAGE_SAMPLER: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::SAMPLER;
/// Texture is a color render target.
pub const SDL_GPU_TEXTUREUSAGE_COLOR_TARGET: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::COLOR_TARGET;
/// Texture is a depth stencil target.
pub const SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::DEPTH_STENCIL_TARGET;
/// Texture supports storage reads in graphics stages.
pub const SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::GRAPHICS_STORAGE_READ;
/// Texture supports storage reads in the compute stage.
pub const SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_READ;
/// Texture supports storage writes in the compute stage.
pub const SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_WRITE: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_WRITE;
/// Texture supports reads and writes in the same compute shader. This is NOT equivalent to READ | WRITE.
pub const SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE: SDL_GPUTextureUsageFlags =
    SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUTextureUsageFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUTextureUsageFlags;
}

/// Specifies the type of a texture.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`_2D`](SDL_GPUTextureType::_2D) | [`SDL_GPU_TEXTURETYPE_2D`] | The texture is a 2-dimensional image. |
/// | [`_2D_ARRAY`](SDL_GPUTextureType::_2D_ARRAY) | [`SDL_GPU_TEXTURETYPE_2D_ARRAY`] | The texture is a 2-dimensional array image. |
/// | [`_3D`](SDL_GPUTextureType::_3D) | [`SDL_GPU_TEXTURETYPE_3D`] | The texture is a 3-dimensional image. |
/// | [`CUBE`](SDL_GPUTextureType::CUBE) | [`SDL_GPU_TEXTURETYPE_CUBE`] | The texture is a cube image. |
/// | [`CUBE_ARRAY`](SDL_GPUTextureType::CUBE_ARRAY) | [`SDL_GPU_TEXTURETYPE_CUBE_ARRAY`] | The texture is a cube array image. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUTextureType(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUTextureType {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUTextureType> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUTextureType) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUTextureType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUTextureType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUTextureType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::_2D => "SDL_GPU_TEXTURETYPE_2D",
            Self::_2D_ARRAY => "SDL_GPU_TEXTURETYPE_2D_ARRAY",
            Self::_3D => "SDL_GPU_TEXTURETYPE_3D",
            Self::CUBE => "SDL_GPU_TEXTURETYPE_CUBE",
            Self::CUBE_ARRAY => "SDL_GPU_TEXTURETYPE_CUBE_ARRAY",

            _ => return write!(f, "SDL_GPUTextureType({})", self.0),
        })
    }
}

impl SDL_GPUTextureType {
    /// The texture is a 2-dimensional image.
    pub const _2D: Self = Self((0 as ::core::ffi::c_int));
    /// The texture is a 2-dimensional array image.
    pub const _2D_ARRAY: Self = Self((1 as ::core::ffi::c_int));
    /// The texture is a 3-dimensional image.
    pub const _3D: Self = Self((2 as ::core::ffi::c_int));
    /// The texture is a cube image.
    pub const CUBE: Self = Self((3 as ::core::ffi::c_int));
    /// The texture is a cube array image.
    pub const CUBE_ARRAY: Self = Self((4 as ::core::ffi::c_int));
}

/// The texture is a 2-dimensional image.
pub const SDL_GPU_TEXTURETYPE_2D: SDL_GPUTextureType = SDL_GPUTextureType::_2D;
/// The texture is a 2-dimensional array image.
pub const SDL_GPU_TEXTURETYPE_2D_ARRAY: SDL_GPUTextureType = SDL_GPUTextureType::_2D_ARRAY;
/// The texture is a 3-dimensional image.
pub const SDL_GPU_TEXTURETYPE_3D: SDL_GPUTextureType = SDL_GPUTextureType::_3D;
/// The texture is a cube image.
pub const SDL_GPU_TEXTURETYPE_CUBE: SDL_GPUTextureType = SDL_GPUTextureType::CUBE;
/// The texture is a cube array image.
pub const SDL_GPU_TEXTURETYPE_CUBE_ARRAY: SDL_GPUTextureType = SDL_GPUTextureType::CUBE_ARRAY;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUTextureType {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUTextureType;
}

/// Specifies the sample count of a texture.
///
/// Used in multisampling. Note that this value only applies when the texture
/// is used as a render target.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
/// - [`SDL_GPUTextureSupportsSampleCount`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`_1`](SDL_GPUSampleCount::_1) | [`SDL_GPU_SAMPLECOUNT_1`] | No multisampling. |
/// | [`_2`](SDL_GPUSampleCount::_2) | [`SDL_GPU_SAMPLECOUNT_2`] | MSAA 2x |
/// | [`_4`](SDL_GPUSampleCount::_4) | [`SDL_GPU_SAMPLECOUNT_4`] | MSAA 4x |
/// | [`_8`](SDL_GPUSampleCount::_8) | [`SDL_GPU_SAMPLECOUNT_8`] | MSAA 8x |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUSampleCount(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUSampleCount {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUSampleCount> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUSampleCount) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUSampleCount> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUSampleCount) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUSampleCount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::_1 => "SDL_GPU_SAMPLECOUNT_1",
            Self::_2 => "SDL_GPU_SAMPLECOUNT_2",
            Self::_4 => "SDL_GPU_SAMPLECOUNT_4",
            Self::_8 => "SDL_GPU_SAMPLECOUNT_8",

            _ => return write!(f, "SDL_GPUSampleCount({})", self.0),
        })
    }
}

impl SDL_GPUSampleCount {
    /// No multisampling.
    pub const _1: Self = Self((0 as ::core::ffi::c_int));
    /// MSAA 2x
    pub const _2: Self = Self((1 as ::core::ffi::c_int));
    /// MSAA 4x
    pub const _4: Self = Self((2 as ::core::ffi::c_int));
    /// MSAA 8x
    pub const _8: Self = Self((3 as ::core::ffi::c_int));
}

/// No multisampling.
pub const SDL_GPU_SAMPLECOUNT_1: SDL_GPUSampleCount = SDL_GPUSampleCount::_1;
/// MSAA 2x
pub const SDL_GPU_SAMPLECOUNT_2: SDL_GPUSampleCount = SDL_GPUSampleCount::_2;
/// MSAA 4x
pub const SDL_GPU_SAMPLECOUNT_4: SDL_GPUSampleCount = SDL_GPUSampleCount::_4;
/// MSAA 8x
pub const SDL_GPU_SAMPLECOUNT_8: SDL_GPUSampleCount = SDL_GPUSampleCount::_8;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUSampleCount {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUSampleCount;
}

/// Specifies the face of a cube map.
///
/// Can be passed in as the layer field in texture-related structs.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`POSITIVEX`](SDL_GPUCubeMapFace::POSITIVEX) | [`SDL_GPU_CUBEMAPFACE_POSITIVEX`] | |
/// | [`NEGATIVEX`](SDL_GPUCubeMapFace::NEGATIVEX) | [`SDL_GPU_CUBEMAPFACE_NEGATIVEX`] | |
/// | [`POSITIVEY`](SDL_GPUCubeMapFace::POSITIVEY) | [`SDL_GPU_CUBEMAPFACE_POSITIVEY`] | |
/// | [`NEGATIVEY`](SDL_GPUCubeMapFace::NEGATIVEY) | [`SDL_GPU_CUBEMAPFACE_NEGATIVEY`] | |
/// | [`POSITIVEZ`](SDL_GPUCubeMapFace::POSITIVEZ) | [`SDL_GPU_CUBEMAPFACE_POSITIVEZ`] | |
/// | [`NEGATIVEZ`](SDL_GPUCubeMapFace::NEGATIVEZ) | [`SDL_GPU_CUBEMAPFACE_NEGATIVEZ`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUCubeMapFace(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUCubeMapFace {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUCubeMapFace> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUCubeMapFace) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUCubeMapFace> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUCubeMapFace) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUCubeMapFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::POSITIVEX => "SDL_GPU_CUBEMAPFACE_POSITIVEX",
            Self::NEGATIVEX => "SDL_GPU_CUBEMAPFACE_NEGATIVEX",
            Self::POSITIVEY => "SDL_GPU_CUBEMAPFACE_POSITIVEY",
            Self::NEGATIVEY => "SDL_GPU_CUBEMAPFACE_NEGATIVEY",
            Self::POSITIVEZ => "SDL_GPU_CUBEMAPFACE_POSITIVEZ",
            Self::NEGATIVEZ => "SDL_GPU_CUBEMAPFACE_NEGATIVEZ",

            _ => return write!(f, "SDL_GPUCubeMapFace({})", self.0),
        })
    }
}

impl SDL_GPUCubeMapFace {
    pub const POSITIVEX: Self = Self((0 as ::core::ffi::c_int));
    pub const NEGATIVEX: Self = Self((1 as ::core::ffi::c_int));
    pub const POSITIVEY: Self = Self((2 as ::core::ffi::c_int));
    pub const NEGATIVEY: Self = Self((3 as ::core::ffi::c_int));
    pub const POSITIVEZ: Self = Self((4 as ::core::ffi::c_int));
    pub const NEGATIVEZ: Self = Self((5 as ::core::ffi::c_int));
}

pub const SDL_GPU_CUBEMAPFACE_POSITIVEX: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEX;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEX: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEX;
pub const SDL_GPU_CUBEMAPFACE_POSITIVEY: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEY;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEY: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEY;
pub const SDL_GPU_CUBEMAPFACE_POSITIVEZ: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::POSITIVEZ;
pub const SDL_GPU_CUBEMAPFACE_NEGATIVEZ: SDL_GPUCubeMapFace = SDL_GPUCubeMapFace::NEGATIVEZ;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUCubeMapFace {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUCubeMapFace;
}

/// Specifies how a buffer is intended to be used by the client.
///
/// A buffer must have at least one usage flag. Note that some usage flag
/// combinations are invalid.
///
/// Unlike textures, READ | WRITE can be used for simultaneous read-write
/// usage. The same data synchronization concerns as textures apply.
///
/// If you use a STORAGE flag, the data in the buffer must respect std140
/// layout conventions. In practical terms this means you must ensure that vec3
/// and vec4 fields are 16-byte aligned.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUBuffer`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`VERTEX`](SDL_GPUBufferUsageFlags::VERTEX) | [`SDL_GPU_BUFFERUSAGE_VERTEX`] | Buffer is a vertex buffer. |
/// | [`INDEX`](SDL_GPUBufferUsageFlags::INDEX) | [`SDL_GPU_BUFFERUSAGE_INDEX`] | Buffer is an index buffer. |
/// | [`INDIRECT`](SDL_GPUBufferUsageFlags::INDIRECT) | [`SDL_GPU_BUFFERUSAGE_INDIRECT`] | Buffer is an indirect buffer. |
/// | [`GRAPHICS_STORAGE_READ`](SDL_GPUBufferUsageFlags::GRAPHICS_STORAGE_READ) | [`SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ`] | Buffer supports storage reads in graphics stages. |
/// | [`COMPUTE_STORAGE_READ`](SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_READ) | [`SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_READ`] | Buffer supports storage reads in the compute stage. |
/// | [`COMPUTE_STORAGE_WRITE`](SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE) | [`SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_WRITE`] | Buffer supports storage writes in the compute stage. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_GPUBufferUsageFlags(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_GPUBufferUsageFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUBufferUsageFlags> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUBufferUsageFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUBufferUsageFlags> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_GPUBufferUsageFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUBufferUsageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_GPUBufferUsageFlags(")?;
        let all_bits = all_bits | Self::VERTEX.0;
        if (Self::VERTEX != 0 || self.0 == 0) && *self & Self::VERTEX == Self::VERTEX {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "VERTEX")?;
        }
        let all_bits = all_bits | Self::INDEX.0;
        if (Self::INDEX != 0 || self.0 == 0) && *self & Self::INDEX == Self::INDEX {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "INDEX")?;
        }
        let all_bits = all_bits | Self::INDIRECT.0;
        if (Self::INDIRECT != 0 || self.0 == 0) && *self & Self::INDIRECT == Self::INDIRECT {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "INDIRECT")?;
        }
        let all_bits = all_bits | Self::GRAPHICS_STORAGE_READ.0;
        if (Self::GRAPHICS_STORAGE_READ != 0 || self.0 == 0)
            && *self & Self::GRAPHICS_STORAGE_READ == Self::GRAPHICS_STORAGE_READ
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "GRAPHICS_STORAGE_READ")?;
        }
        let all_bits = all_bits | Self::COMPUTE_STORAGE_READ.0;
        if (Self::COMPUTE_STORAGE_READ != 0 || self.0 == 0)
            && *self & Self::COMPUTE_STORAGE_READ == Self::COMPUTE_STORAGE_READ
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COMPUTE_STORAGE_READ")?;
        }
        let all_bits = all_bits | Self::COMPUTE_STORAGE_WRITE.0;
        if (Self::COMPUTE_STORAGE_WRITE != 0 || self.0 == 0)
            && *self & Self::COMPUTE_STORAGE_WRITE == Self::COMPUTE_STORAGE_WRITE
        {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "COMPUTE_STORAGE_WRITE")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_GPUBufferUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_GPUBufferUsageFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_GPUBufferUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_GPUBufferUsageFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_GPUBufferUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_GPUBufferUsageFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_GPUBufferUsageFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_GPUBufferUsageFlags {
    /// Buffer is a vertex buffer.
    pub const VERTEX: Self = Self((1_u32 as Uint32));
    /// Buffer is an index buffer.
    pub const INDEX: Self = Self((2_u32 as Uint32));
    /// Buffer is an indirect buffer.
    pub const INDIRECT: Self = Self((4_u32 as Uint32));
    /// Buffer supports storage reads in graphics stages.
    pub const GRAPHICS_STORAGE_READ: Self = Self((8_u32 as Uint32));
    /// Buffer supports storage reads in the compute stage.
    pub const COMPUTE_STORAGE_READ: Self = Self((16_u32 as Uint32));
    /// Buffer supports storage writes in the compute stage.
    pub const COMPUTE_STORAGE_WRITE: Self = Self((32_u32 as Uint32));
}

/// Buffer is a vertex buffer.
pub const SDL_GPU_BUFFERUSAGE_VERTEX: SDL_GPUBufferUsageFlags = SDL_GPUBufferUsageFlags::VERTEX;
/// Buffer is an index buffer.
pub const SDL_GPU_BUFFERUSAGE_INDEX: SDL_GPUBufferUsageFlags = SDL_GPUBufferUsageFlags::INDEX;
/// Buffer is an indirect buffer.
pub const SDL_GPU_BUFFERUSAGE_INDIRECT: SDL_GPUBufferUsageFlags = SDL_GPUBufferUsageFlags::INDIRECT;
/// Buffer supports storage reads in graphics stages.
pub const SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ: SDL_GPUBufferUsageFlags =
    SDL_GPUBufferUsageFlags::GRAPHICS_STORAGE_READ;
/// Buffer supports storage reads in the compute stage.
pub const SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_READ: SDL_GPUBufferUsageFlags =
    SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_READ;
/// Buffer supports storage writes in the compute stage.
pub const SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_WRITE: SDL_GPUBufferUsageFlags =
    SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUBufferUsageFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUBufferUsageFlags;
}

/// Specifies how a transfer buffer is intended to be used by the client.
///
/// Note that mapping and copying FROM an upload transfer buffer or TO a
/// download transfer buffer is undefined behavior.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTransferBuffer`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UPLOAD`](SDL_GPUTransferBufferUsage::UPLOAD) | [`SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD`] | |
/// | [`DOWNLOAD`](SDL_GPUTransferBufferUsage::DOWNLOAD) | [`SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUTransferBufferUsage(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUTransferBufferUsage {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUTransferBufferUsage> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUTransferBufferUsage) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUTransferBufferUsage> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUTransferBufferUsage) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUTransferBufferUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UPLOAD => "SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD",
            Self::DOWNLOAD => "SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD",

            _ => return write!(f, "SDL_GPUTransferBufferUsage({})", self.0),
        })
    }
}

impl SDL_GPUTransferBufferUsage {
    pub const UPLOAD: Self = Self((0 as ::core::ffi::c_int));
    pub const DOWNLOAD: Self = Self((1 as ::core::ffi::c_int));
}

pub const SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD: SDL_GPUTransferBufferUsage =
    SDL_GPUTransferBufferUsage::UPLOAD;
pub const SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD: SDL_GPUTransferBufferUsage =
    SDL_GPUTransferBufferUsage::DOWNLOAD;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUTransferBufferUsage {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUTransferBufferUsage;
}

/// Specifies which stage a shader program corresponds to.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUShader`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`VERTEX`](SDL_GPUShaderStage::VERTEX) | [`SDL_GPU_SHADERSTAGE_VERTEX`] | |
/// | [`FRAGMENT`](SDL_GPUShaderStage::FRAGMENT) | [`SDL_GPU_SHADERSTAGE_FRAGMENT`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUShaderStage(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUShaderStage {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUShaderStage> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUShaderStage) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUShaderStage> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUShaderStage) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUShaderStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::VERTEX => "SDL_GPU_SHADERSTAGE_VERTEX",
            Self::FRAGMENT => "SDL_GPU_SHADERSTAGE_FRAGMENT",

            _ => return write!(f, "SDL_GPUShaderStage({})", self.0),
        })
    }
}

impl SDL_GPUShaderStage {
    pub const VERTEX: Self = Self((0 as ::core::ffi::c_int));
    pub const FRAGMENT: Self = Self((1 as ::core::ffi::c_int));
}

pub const SDL_GPU_SHADERSTAGE_VERTEX: SDL_GPUShaderStage = SDL_GPUShaderStage::VERTEX;
pub const SDL_GPU_SHADERSTAGE_FRAGMENT: SDL_GPUShaderStage = SDL_GPUShaderStage::FRAGMENT;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUShaderStage {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUShaderStage;
}

/// Specifies the format of shader code.
///
/// Each format corresponds to a specific backend that accepts it.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUShader`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUShaderFormat::INVALID) | [`SDL_GPU_SHADERFORMAT_INVALID`] | |
/// | [`PRIVATE`](SDL_GPUShaderFormat::PRIVATE) | [`SDL_GPU_SHADERFORMAT_PRIVATE`] | Shaders for NDA'd platforms. |
/// | [`SPIRV`](SDL_GPUShaderFormat::SPIRV) | [`SDL_GPU_SHADERFORMAT_SPIRV`] | SPIR-V shaders for Vulkan. |
/// | [`DXBC`](SDL_GPUShaderFormat::DXBC) | [`SDL_GPU_SHADERFORMAT_DXBC`] | DXBC SM5_1 shaders for D3D12. |
/// | [`DXIL`](SDL_GPUShaderFormat::DXIL) | [`SDL_GPU_SHADERFORMAT_DXIL`] | DXIL SM6_0 shaders for D3D12. |
/// | [`MSL`](SDL_GPUShaderFormat::MSL) | [`SDL_GPU_SHADERFORMAT_MSL`] | MSL shaders for Metal. |
/// | [`METALLIB`](SDL_GPUShaderFormat::METALLIB) | [`SDL_GPU_SHADERFORMAT_METALLIB`] | Precompiled metallib shaders for Metal. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_GPUShaderFormat(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_GPUShaderFormat {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUShaderFormat> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUShaderFormat) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUShaderFormat> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_GPUShaderFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUShaderFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_GPUShaderFormat(")?;
        let all_bits = all_bits | Self::INVALID.0;
        if (Self::INVALID != 0 || self.0 == 0) && *self & Self::INVALID == Self::INVALID {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "INVALID")?;
        }
        let all_bits = all_bits | Self::PRIVATE.0;
        if (Self::PRIVATE != 0 || self.0 == 0) && *self & Self::PRIVATE == Self::PRIVATE {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "PRIVATE")?;
        }
        let all_bits = all_bits | Self::SPIRV.0;
        if (Self::SPIRV != 0 || self.0 == 0) && *self & Self::SPIRV == Self::SPIRV {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "SPIRV")?;
        }
        let all_bits = all_bits | Self::DXBC.0;
        if (Self::DXBC != 0 || self.0 == 0) && *self & Self::DXBC == Self::DXBC {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "DXBC")?;
        }
        let all_bits = all_bits | Self::DXIL.0;
        if (Self::DXIL != 0 || self.0 == 0) && *self & Self::DXIL == Self::DXIL {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "DXIL")?;
        }
        let all_bits = all_bits | Self::MSL.0;
        if (Self::MSL != 0 || self.0 == 0) && *self & Self::MSL == Self::MSL {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "MSL")?;
        }
        let all_bits = all_bits | Self::METALLIB.0;
        if (Self::METALLIB != 0 || self.0 == 0) && *self & Self::METALLIB == Self::METALLIB {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "METALLIB")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_GPUShaderFormat {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_GPUShaderFormat {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_GPUShaderFormat {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_GPUShaderFormat {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_GPUShaderFormat {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_GPUShaderFormat {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_GPUShaderFormat {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_GPUShaderFormat {
    pub const INVALID: Self = Self((0 as Uint32));
    /// Shaders for NDA'd platforms.
    pub const PRIVATE: Self = Self((1_u32 as Uint32));
    /// SPIR-V shaders for Vulkan.
    pub const SPIRV: Self = Self((2_u32 as Uint32));
    /// DXBC SM5_1 shaders for D3D12.
    pub const DXBC: Self = Self((4_u32 as Uint32));
    /// DXIL SM6_0 shaders for D3D12.
    pub const DXIL: Self = Self((8_u32 as Uint32));
    /// MSL shaders for Metal.
    pub const MSL: Self = Self((16_u32 as Uint32));
    /// Precompiled metallib shaders for Metal.
    pub const METALLIB: Self = Self((32_u32 as Uint32));
}

pub const SDL_GPU_SHADERFORMAT_INVALID: SDL_GPUShaderFormat = SDL_GPUShaderFormat::INVALID;
/// Shaders for NDA'd platforms.
pub const SDL_GPU_SHADERFORMAT_PRIVATE: SDL_GPUShaderFormat = SDL_GPUShaderFormat::PRIVATE;
/// SPIR-V shaders for Vulkan.
pub const SDL_GPU_SHADERFORMAT_SPIRV: SDL_GPUShaderFormat = SDL_GPUShaderFormat::SPIRV;
/// DXBC SM5_1 shaders for D3D12.
pub const SDL_GPU_SHADERFORMAT_DXBC: SDL_GPUShaderFormat = SDL_GPUShaderFormat::DXBC;
/// DXIL SM6_0 shaders for D3D12.
pub const SDL_GPU_SHADERFORMAT_DXIL: SDL_GPUShaderFormat = SDL_GPUShaderFormat::DXIL;
/// MSL shaders for Metal.
pub const SDL_GPU_SHADERFORMAT_MSL: SDL_GPUShaderFormat = SDL_GPUShaderFormat::MSL;
/// Precompiled metallib shaders for Metal.
pub const SDL_GPU_SHADERFORMAT_METALLIB: SDL_GPUShaderFormat = SDL_GPUShaderFormat::METALLIB;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUShaderFormat {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUShaderFormat;
}

/// Specifies the format of a vertex attribute.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUVertexElementFormat::INVALID) | [`SDL_GPU_VERTEXELEMENTFORMAT_INVALID`] | |
/// | [`INT`](SDL_GPUVertexElementFormat::INT) | [`SDL_GPU_VERTEXELEMENTFORMAT_INT`] | |
/// | [`INT2`](SDL_GPUVertexElementFormat::INT2) | [`SDL_GPU_VERTEXELEMENTFORMAT_INT2`] | |
/// | [`INT3`](SDL_GPUVertexElementFormat::INT3) | [`SDL_GPU_VERTEXELEMENTFORMAT_INT3`] | |
/// | [`INT4`](SDL_GPUVertexElementFormat::INT4) | [`SDL_GPU_VERTEXELEMENTFORMAT_INT4`] | |
/// | [`UINT`](SDL_GPUVertexElementFormat::UINT) | [`SDL_GPU_VERTEXELEMENTFORMAT_UINT`] | |
/// | [`UINT2`](SDL_GPUVertexElementFormat::UINT2) | [`SDL_GPU_VERTEXELEMENTFORMAT_UINT2`] | |
/// | [`UINT3`](SDL_GPUVertexElementFormat::UINT3) | [`SDL_GPU_VERTEXELEMENTFORMAT_UINT3`] | |
/// | [`UINT4`](SDL_GPUVertexElementFormat::UINT4) | [`SDL_GPU_VERTEXELEMENTFORMAT_UINT4`] | |
/// | [`FLOAT`](SDL_GPUVertexElementFormat::FLOAT) | [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT`] | |
/// | [`FLOAT2`](SDL_GPUVertexElementFormat::FLOAT2) | [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2`] | |
/// | [`FLOAT3`](SDL_GPUVertexElementFormat::FLOAT3) | [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3`] | |
/// | [`FLOAT4`](SDL_GPUVertexElementFormat::FLOAT4) | [`SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4`] | |
/// | [`BYTE2`](SDL_GPUVertexElementFormat::BYTE2) | [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE2`] | |
/// | [`BYTE4`](SDL_GPUVertexElementFormat::BYTE4) | [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE4`] | |
/// | [`UBYTE2`](SDL_GPUVertexElementFormat::UBYTE2) | [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2`] | |
/// | [`UBYTE4`](SDL_GPUVertexElementFormat::UBYTE4) | [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4`] | |
/// | [`BYTE2_NORM`](SDL_GPUVertexElementFormat::BYTE2_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM`] | |
/// | [`BYTE4_NORM`](SDL_GPUVertexElementFormat::BYTE4_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM`] | |
/// | [`UBYTE2_NORM`](SDL_GPUVertexElementFormat::UBYTE2_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM`] | |
/// | [`UBYTE4_NORM`](SDL_GPUVertexElementFormat::UBYTE4_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM`] | |
/// | [`SHORT2`](SDL_GPUVertexElementFormat::SHORT2) | [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT2`] | |
/// | [`SHORT4`](SDL_GPUVertexElementFormat::SHORT4) | [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT4`] | |
/// | [`USHORT2`](SDL_GPUVertexElementFormat::USHORT2) | [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT2`] | |
/// | [`USHORT4`](SDL_GPUVertexElementFormat::USHORT4) | [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT4`] | |
/// | [`SHORT2_NORM`](SDL_GPUVertexElementFormat::SHORT2_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM`] | |
/// | [`SHORT4_NORM`](SDL_GPUVertexElementFormat::SHORT4_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM`] | |
/// | [`USHORT2_NORM`](SDL_GPUVertexElementFormat::USHORT2_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM`] | |
/// | [`USHORT4_NORM`](SDL_GPUVertexElementFormat::USHORT4_NORM) | [`SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM`] | |
/// | [`HALF2`](SDL_GPUVertexElementFormat::HALF2) | [`SDL_GPU_VERTEXELEMENTFORMAT_HALF2`] | |
/// | [`HALF4`](SDL_GPUVertexElementFormat::HALF4) | [`SDL_GPU_VERTEXELEMENTFORMAT_HALF4`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUVertexElementFormat(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUVertexElementFormat {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUVertexElementFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUVertexElementFormat) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUVertexElementFormat> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUVertexElementFormat) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUVertexElementFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_VERTEXELEMENTFORMAT_INVALID",
            Self::INT => "SDL_GPU_VERTEXELEMENTFORMAT_INT",
            Self::INT2 => "SDL_GPU_VERTEXELEMENTFORMAT_INT2",
            Self::INT3 => "SDL_GPU_VERTEXELEMENTFORMAT_INT3",
            Self::INT4 => "SDL_GPU_VERTEXELEMENTFORMAT_INT4",
            Self::UINT => "SDL_GPU_VERTEXELEMENTFORMAT_UINT",
            Self::UINT2 => "SDL_GPU_VERTEXELEMENTFORMAT_UINT2",
            Self::UINT3 => "SDL_GPU_VERTEXELEMENTFORMAT_UINT3",
            Self::UINT4 => "SDL_GPU_VERTEXELEMENTFORMAT_UINT4",
            Self::FLOAT => "SDL_GPU_VERTEXELEMENTFORMAT_FLOAT",
            Self::FLOAT2 => "SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2",
            Self::FLOAT3 => "SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3",
            Self::FLOAT4 => "SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4",
            Self::BYTE2 => "SDL_GPU_VERTEXELEMENTFORMAT_BYTE2",
            Self::BYTE4 => "SDL_GPU_VERTEXELEMENTFORMAT_BYTE4",
            Self::UBYTE2 => "SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2",
            Self::UBYTE4 => "SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4",
            Self::BYTE2_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM",
            Self::BYTE4_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM",
            Self::UBYTE2_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM",
            Self::UBYTE4_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM",
            Self::SHORT2 => "SDL_GPU_VERTEXELEMENTFORMAT_SHORT2",
            Self::SHORT4 => "SDL_GPU_VERTEXELEMENTFORMAT_SHORT4",
            Self::USHORT2 => "SDL_GPU_VERTEXELEMENTFORMAT_USHORT2",
            Self::USHORT4 => "SDL_GPU_VERTEXELEMENTFORMAT_USHORT4",
            Self::SHORT2_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM",
            Self::SHORT4_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM",
            Self::USHORT2_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM",
            Self::USHORT4_NORM => "SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM",
            Self::HALF2 => "SDL_GPU_VERTEXELEMENTFORMAT_HALF2",
            Self::HALF4 => "SDL_GPU_VERTEXELEMENTFORMAT_HALF4",

            _ => return write!(f, "SDL_GPUVertexElementFormat({})", self.0),
        })
    }
}

impl SDL_GPUVertexElementFormat {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    pub const INT: Self = Self((1 as ::core::ffi::c_int));
    pub const INT2: Self = Self((2 as ::core::ffi::c_int));
    pub const INT3: Self = Self((3 as ::core::ffi::c_int));
    pub const INT4: Self = Self((4 as ::core::ffi::c_int));
    pub const UINT: Self = Self((5 as ::core::ffi::c_int));
    pub const UINT2: Self = Self((6 as ::core::ffi::c_int));
    pub const UINT3: Self = Self((7 as ::core::ffi::c_int));
    pub const UINT4: Self = Self((8 as ::core::ffi::c_int));
    pub const FLOAT: Self = Self((9 as ::core::ffi::c_int));
    pub const FLOAT2: Self = Self((10 as ::core::ffi::c_int));
    pub const FLOAT3: Self = Self((11 as ::core::ffi::c_int));
    pub const FLOAT4: Self = Self((12 as ::core::ffi::c_int));
    pub const BYTE2: Self = Self((13 as ::core::ffi::c_int));
    pub const BYTE4: Self = Self((14 as ::core::ffi::c_int));
    pub const UBYTE2: Self = Self((15 as ::core::ffi::c_int));
    pub const UBYTE4: Self = Self((16 as ::core::ffi::c_int));
    pub const BYTE2_NORM: Self = Self((17 as ::core::ffi::c_int));
    pub const BYTE4_NORM: Self = Self((18 as ::core::ffi::c_int));
    pub const UBYTE2_NORM: Self = Self((19 as ::core::ffi::c_int));
    pub const UBYTE4_NORM: Self = Self((20 as ::core::ffi::c_int));
    pub const SHORT2: Self = Self((21 as ::core::ffi::c_int));
    pub const SHORT4: Self = Self((22 as ::core::ffi::c_int));
    pub const USHORT2: Self = Self((23 as ::core::ffi::c_int));
    pub const USHORT4: Self = Self((24 as ::core::ffi::c_int));
    pub const SHORT2_NORM: Self = Self((25 as ::core::ffi::c_int));
    pub const SHORT4_NORM: Self = Self((26 as ::core::ffi::c_int));
    pub const USHORT2_NORM: Self = Self((27 as ::core::ffi::c_int));
    pub const USHORT4_NORM: Self = Self((28 as ::core::ffi::c_int));
    pub const HALF2: Self = Self((29 as ::core::ffi::c_int));
    pub const HALF4: Self = Self((30 as ::core::ffi::c_int));
}

pub const SDL_GPU_VERTEXELEMENTFORMAT_INVALID: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::INVALID;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::INT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::INT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT3: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::INT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_INT4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::INT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UINT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UINT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT3: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UINT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UINT4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UINT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::FLOAT;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::FLOAT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::FLOAT3;
pub const SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::FLOAT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::BYTE2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::BYTE4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UBYTE2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UBYTE4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::BYTE2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::BYTE4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UBYTE2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::UBYTE4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::SHORT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::SHORT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::USHORT2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::USHORT4;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::SHORT2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::SHORT4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::USHORT2_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::USHORT4_NORM;
pub const SDL_GPU_VERTEXELEMENTFORMAT_HALF2: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::HALF2;
pub const SDL_GPU_VERTEXELEMENTFORMAT_HALF4: SDL_GPUVertexElementFormat =
    SDL_GPUVertexElementFormat::HALF4;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUVertexElementFormat {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUVertexElementFormat;
}

/// Specifies the rate at which vertex attributes are pulled from buffers.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`VERTEX`](SDL_GPUVertexInputRate::VERTEX) | [`SDL_GPU_VERTEXINPUTRATE_VERTEX`] | Attribute addressing is a function of the vertex index. |
/// | [`INSTANCE`](SDL_GPUVertexInputRate::INSTANCE) | [`SDL_GPU_VERTEXINPUTRATE_INSTANCE`] | Attribute addressing is a function of the instance index. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUVertexInputRate(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUVertexInputRate {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUVertexInputRate> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUVertexInputRate) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUVertexInputRate> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUVertexInputRate) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUVertexInputRate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::VERTEX => "SDL_GPU_VERTEXINPUTRATE_VERTEX",
            Self::INSTANCE => "SDL_GPU_VERTEXINPUTRATE_INSTANCE",

            _ => return write!(f, "SDL_GPUVertexInputRate({})", self.0),
        })
    }
}

impl SDL_GPUVertexInputRate {
    /// Attribute addressing is a function of the vertex index.
    pub const VERTEX: Self = Self((0 as ::core::ffi::c_int));
    /// Attribute addressing is a function of the instance index.
    pub const INSTANCE: Self = Self((1 as ::core::ffi::c_int));
}

/// Attribute addressing is a function of the vertex index.
pub const SDL_GPU_VERTEXINPUTRATE_VERTEX: SDL_GPUVertexInputRate = SDL_GPUVertexInputRate::VERTEX;
/// Attribute addressing is a function of the instance index.
pub const SDL_GPU_VERTEXINPUTRATE_INSTANCE: SDL_GPUVertexInputRate =
    SDL_GPUVertexInputRate::INSTANCE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUVertexInputRate {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUVertexInputRate;
}

/// Specifies the fill mode of the graphics pipeline.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`FILL`](SDL_GPUFillMode::FILL) | [`SDL_GPU_FILLMODE_FILL`] | Polygons will be rendered via rasterization. |
/// | [`LINE`](SDL_GPUFillMode::LINE) | [`SDL_GPU_FILLMODE_LINE`] | Polygon edges will be drawn as line segments. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUFillMode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUFillMode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUFillMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUFillMode) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUFillMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUFillMode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUFillMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::FILL => "SDL_GPU_FILLMODE_FILL",
            Self::LINE => "SDL_GPU_FILLMODE_LINE",

            _ => return write!(f, "SDL_GPUFillMode({})", self.0),
        })
    }
}

impl SDL_GPUFillMode {
    /// Polygons will be rendered via rasterization.
    pub const FILL: Self = Self((0 as ::core::ffi::c_int));
    /// Polygon edges will be drawn as line segments.
    pub const LINE: Self = Self((1 as ::core::ffi::c_int));
}

/// Polygons will be rendered via rasterization.
pub const SDL_GPU_FILLMODE_FILL: SDL_GPUFillMode = SDL_GPUFillMode::FILL;
/// Polygon edges will be drawn as line segments.
pub const SDL_GPU_FILLMODE_LINE: SDL_GPUFillMode = SDL_GPUFillMode::LINE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUFillMode {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUFillMode;
}

/// Specifies the facing direction in which triangle faces will be culled.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](SDL_GPUCullMode::NONE) | [`SDL_GPU_CULLMODE_NONE`] | No triangles are culled. |
/// | [`FRONT`](SDL_GPUCullMode::FRONT) | [`SDL_GPU_CULLMODE_FRONT`] | Front-facing triangles are culled. |
/// | [`BACK`](SDL_GPUCullMode::BACK) | [`SDL_GPU_CULLMODE_BACK`] | Back-facing triangles are culled. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUCullMode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUCullMode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUCullMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUCullMode) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUCullMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUCullMode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUCullMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SDL_GPU_CULLMODE_NONE",
            Self::FRONT => "SDL_GPU_CULLMODE_FRONT",
            Self::BACK => "SDL_GPU_CULLMODE_BACK",

            _ => return write!(f, "SDL_GPUCullMode({})", self.0),
        })
    }
}

impl SDL_GPUCullMode {
    /// No triangles are culled.
    pub const NONE: Self = Self((0 as ::core::ffi::c_int));
    /// Front-facing triangles are culled.
    pub const FRONT: Self = Self((1 as ::core::ffi::c_int));
    /// Back-facing triangles are culled.
    pub const BACK: Self = Self((2 as ::core::ffi::c_int));
}

/// No triangles are culled.
pub const SDL_GPU_CULLMODE_NONE: SDL_GPUCullMode = SDL_GPUCullMode::NONE;
/// Front-facing triangles are culled.
pub const SDL_GPU_CULLMODE_FRONT: SDL_GPUCullMode = SDL_GPUCullMode::FRONT;
/// Back-facing triangles are culled.
pub const SDL_GPU_CULLMODE_BACK: SDL_GPUCullMode = SDL_GPUCullMode::BACK;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUCullMode {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUCullMode;
}

/// Specifies the vertex winding that will cause a triangle to be determined to
/// be front-facing.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`COUNTER_CLOCKWISE`](SDL_GPUFrontFace::COUNTER_CLOCKWISE) | [`SDL_GPU_FRONTFACE_COUNTER_CLOCKWISE`] | A triangle with counter-clockwise vertex winding will be considered front-facing. |
/// | [`CLOCKWISE`](SDL_GPUFrontFace::CLOCKWISE) | [`SDL_GPU_FRONTFACE_CLOCKWISE`] | A triangle with clockwise vertex winding will be considered front-facing. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUFrontFace(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUFrontFace {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUFrontFace> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUFrontFace) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUFrontFace> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUFrontFace) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUFrontFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::COUNTER_CLOCKWISE => "SDL_GPU_FRONTFACE_COUNTER_CLOCKWISE",
            Self::CLOCKWISE => "SDL_GPU_FRONTFACE_CLOCKWISE",

            _ => return write!(f, "SDL_GPUFrontFace({})", self.0),
        })
    }
}

impl SDL_GPUFrontFace {
    /// A triangle with counter-clockwise vertex winding will be considered front-facing.
    pub const COUNTER_CLOCKWISE: Self = Self((0 as ::core::ffi::c_int));
    /// A triangle with clockwise vertex winding will be considered front-facing.
    pub const CLOCKWISE: Self = Self((1 as ::core::ffi::c_int));
}

/// A triangle with counter-clockwise vertex winding will be considered front-facing.
pub const SDL_GPU_FRONTFACE_COUNTER_CLOCKWISE: SDL_GPUFrontFace =
    SDL_GPUFrontFace::COUNTER_CLOCKWISE;
/// A triangle with clockwise vertex winding will be considered front-facing.
pub const SDL_GPU_FRONTFACE_CLOCKWISE: SDL_GPUFrontFace = SDL_GPUFrontFace::CLOCKWISE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUFrontFace {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUFrontFace;
}

/// Specifies a comparison operator for depth, stencil and sampler operations.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUCompareOp::INVALID) | [`SDL_GPU_COMPAREOP_INVALID`] | |
/// | [`NEVER`](SDL_GPUCompareOp::NEVER) | [`SDL_GPU_COMPAREOP_NEVER`] | The comparison always evaluates false. |
/// | [`LESS`](SDL_GPUCompareOp::LESS) | [`SDL_GPU_COMPAREOP_LESS`] | The comparison evaluates reference < test. |
/// | [`EQUAL`](SDL_GPUCompareOp::EQUAL) | [`SDL_GPU_COMPAREOP_EQUAL`] | The comparison evaluates reference == test. |
/// | [`LESS_OR_EQUAL`](SDL_GPUCompareOp::LESS_OR_EQUAL) | [`SDL_GPU_COMPAREOP_LESS_OR_EQUAL`] | The comparison evaluates reference <= test. |
/// | [`GREATER`](SDL_GPUCompareOp::GREATER) | [`SDL_GPU_COMPAREOP_GREATER`] | The comparison evaluates reference > test. |
/// | [`NOT_EQUAL`](SDL_GPUCompareOp::NOT_EQUAL) | [`SDL_GPU_COMPAREOP_NOT_EQUAL`] | The comparison evaluates reference != test. |
/// | [`GREATER_OR_EQUAL`](SDL_GPUCompareOp::GREATER_OR_EQUAL) | [`SDL_GPU_COMPAREOP_GREATER_OR_EQUAL`] | The comparison evalutes reference >= test. |
/// | [`ALWAYS`](SDL_GPUCompareOp::ALWAYS) | [`SDL_GPU_COMPAREOP_ALWAYS`] | The comparison always evaluates true. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUCompareOp(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUCompareOp {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUCompareOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUCompareOp) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUCompareOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUCompareOp) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUCompareOp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_COMPAREOP_INVALID",
            Self::NEVER => "SDL_GPU_COMPAREOP_NEVER",
            Self::LESS => "SDL_GPU_COMPAREOP_LESS",
            Self::EQUAL => "SDL_GPU_COMPAREOP_EQUAL",
            Self::LESS_OR_EQUAL => "SDL_GPU_COMPAREOP_LESS_OR_EQUAL",
            Self::GREATER => "SDL_GPU_COMPAREOP_GREATER",
            Self::NOT_EQUAL => "SDL_GPU_COMPAREOP_NOT_EQUAL",
            Self::GREATER_OR_EQUAL => "SDL_GPU_COMPAREOP_GREATER_OR_EQUAL",
            Self::ALWAYS => "SDL_GPU_COMPAREOP_ALWAYS",

            _ => return write!(f, "SDL_GPUCompareOp({})", self.0),
        })
    }
}

impl SDL_GPUCompareOp {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    /// The comparison always evaluates false.
    pub const NEVER: Self = Self((1 as ::core::ffi::c_int));
    /// The comparison evaluates reference < test.
    pub const LESS: Self = Self((2 as ::core::ffi::c_int));
    /// The comparison evaluates reference == test.
    pub const EQUAL: Self = Self((3 as ::core::ffi::c_int));
    /// The comparison evaluates reference <= test.
    pub const LESS_OR_EQUAL: Self = Self((4 as ::core::ffi::c_int));
    /// The comparison evaluates reference > test.
    pub const GREATER: Self = Self((5 as ::core::ffi::c_int));
    /// The comparison evaluates reference != test.
    pub const NOT_EQUAL: Self = Self((6 as ::core::ffi::c_int));
    /// The comparison evalutes reference >= test.
    pub const GREATER_OR_EQUAL: Self = Self((7 as ::core::ffi::c_int));
    /// The comparison always evaluates true.
    pub const ALWAYS: Self = Self((8 as ::core::ffi::c_int));
}

pub const SDL_GPU_COMPAREOP_INVALID: SDL_GPUCompareOp = SDL_GPUCompareOp::INVALID;
/// The comparison always evaluates false.
pub const SDL_GPU_COMPAREOP_NEVER: SDL_GPUCompareOp = SDL_GPUCompareOp::NEVER;
/// The comparison evaluates reference < test.
pub const SDL_GPU_COMPAREOP_LESS: SDL_GPUCompareOp = SDL_GPUCompareOp::LESS;
/// The comparison evaluates reference == test.
pub const SDL_GPU_COMPAREOP_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::EQUAL;
/// The comparison evaluates reference <= test.
pub const SDL_GPU_COMPAREOP_LESS_OR_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::LESS_OR_EQUAL;
/// The comparison evaluates reference > test.
pub const SDL_GPU_COMPAREOP_GREATER: SDL_GPUCompareOp = SDL_GPUCompareOp::GREATER;
/// The comparison evaluates reference != test.
pub const SDL_GPU_COMPAREOP_NOT_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::NOT_EQUAL;
/// The comparison evalutes reference >= test.
pub const SDL_GPU_COMPAREOP_GREATER_OR_EQUAL: SDL_GPUCompareOp = SDL_GPUCompareOp::GREATER_OR_EQUAL;
/// The comparison always evaluates true.
pub const SDL_GPU_COMPAREOP_ALWAYS: SDL_GPUCompareOp = SDL_GPUCompareOp::ALWAYS;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUCompareOp {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUCompareOp;
}

/// Specifies what happens to a stored stencil value if stencil tests fail or
/// pass.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUStencilOp::INVALID) | [`SDL_GPU_STENCILOP_INVALID`] | |
/// | [`KEEP`](SDL_GPUStencilOp::KEEP) | [`SDL_GPU_STENCILOP_KEEP`] | Keeps the current value. |
/// | [`ZERO`](SDL_GPUStencilOp::ZERO) | [`SDL_GPU_STENCILOP_ZERO`] | Sets the value to 0. |
/// | [`REPLACE`](SDL_GPUStencilOp::REPLACE) | [`SDL_GPU_STENCILOP_REPLACE`] | Sets the value to reference. |
/// | [`INCREMENT_AND_CLAMP`](SDL_GPUStencilOp::INCREMENT_AND_CLAMP) | [`SDL_GPU_STENCILOP_INCREMENT_AND_CLAMP`] | Increments the current value and clamps to the maximum value. |
/// | [`DECREMENT_AND_CLAMP`](SDL_GPUStencilOp::DECREMENT_AND_CLAMP) | [`SDL_GPU_STENCILOP_DECREMENT_AND_CLAMP`] | Decrements the current value and clamps to 0. |
/// | [`INVERT`](SDL_GPUStencilOp::INVERT) | [`SDL_GPU_STENCILOP_INVERT`] | Bitwise-inverts the current value. |
/// | [`INCREMENT_AND_WRAP`](SDL_GPUStencilOp::INCREMENT_AND_WRAP) | [`SDL_GPU_STENCILOP_INCREMENT_AND_WRAP`] | Increments the current value and wraps back to 0. |
/// | [`DECREMENT_AND_WRAP`](SDL_GPUStencilOp::DECREMENT_AND_WRAP) | [`SDL_GPU_STENCILOP_DECREMENT_AND_WRAP`] | Decrements the current value and wraps to the maximum value. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUStencilOp(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUStencilOp {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUStencilOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUStencilOp) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUStencilOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUStencilOp) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUStencilOp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_STENCILOP_INVALID",
            Self::KEEP => "SDL_GPU_STENCILOP_KEEP",
            Self::ZERO => "SDL_GPU_STENCILOP_ZERO",
            Self::REPLACE => "SDL_GPU_STENCILOP_REPLACE",
            Self::INCREMENT_AND_CLAMP => "SDL_GPU_STENCILOP_INCREMENT_AND_CLAMP",
            Self::DECREMENT_AND_CLAMP => "SDL_GPU_STENCILOP_DECREMENT_AND_CLAMP",
            Self::INVERT => "SDL_GPU_STENCILOP_INVERT",
            Self::INCREMENT_AND_WRAP => "SDL_GPU_STENCILOP_INCREMENT_AND_WRAP",
            Self::DECREMENT_AND_WRAP => "SDL_GPU_STENCILOP_DECREMENT_AND_WRAP",

            _ => return write!(f, "SDL_GPUStencilOp({})", self.0),
        })
    }
}

impl SDL_GPUStencilOp {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    /// Keeps the current value.
    pub const KEEP: Self = Self((1 as ::core::ffi::c_int));
    /// Sets the value to 0.
    pub const ZERO: Self = Self((2 as ::core::ffi::c_int));
    /// Sets the value to reference.
    pub const REPLACE: Self = Self((3 as ::core::ffi::c_int));
    /// Increments the current value and clamps to the maximum value.
    pub const INCREMENT_AND_CLAMP: Self = Self((4 as ::core::ffi::c_int));
    /// Decrements the current value and clamps to 0.
    pub const DECREMENT_AND_CLAMP: Self = Self((5 as ::core::ffi::c_int));
    /// Bitwise-inverts the current value.
    pub const INVERT: Self = Self((6 as ::core::ffi::c_int));
    /// Increments the current value and wraps back to 0.
    pub const INCREMENT_AND_WRAP: Self = Self((7 as ::core::ffi::c_int));
    /// Decrements the current value and wraps to the maximum value.
    pub const DECREMENT_AND_WRAP: Self = Self((8 as ::core::ffi::c_int));
}

pub const SDL_GPU_STENCILOP_INVALID: SDL_GPUStencilOp = SDL_GPUStencilOp::INVALID;
/// Keeps the current value.
pub const SDL_GPU_STENCILOP_KEEP: SDL_GPUStencilOp = SDL_GPUStencilOp::KEEP;
/// Sets the value to 0.
pub const SDL_GPU_STENCILOP_ZERO: SDL_GPUStencilOp = SDL_GPUStencilOp::ZERO;
/// Sets the value to reference.
pub const SDL_GPU_STENCILOP_REPLACE: SDL_GPUStencilOp = SDL_GPUStencilOp::REPLACE;
/// Increments the current value and clamps to the maximum value.
pub const SDL_GPU_STENCILOP_INCREMENT_AND_CLAMP: SDL_GPUStencilOp =
    SDL_GPUStencilOp::INCREMENT_AND_CLAMP;
/// Decrements the current value and clamps to 0.
pub const SDL_GPU_STENCILOP_DECREMENT_AND_CLAMP: SDL_GPUStencilOp =
    SDL_GPUStencilOp::DECREMENT_AND_CLAMP;
/// Bitwise-inverts the current value.
pub const SDL_GPU_STENCILOP_INVERT: SDL_GPUStencilOp = SDL_GPUStencilOp::INVERT;
/// Increments the current value and wraps back to 0.
pub const SDL_GPU_STENCILOP_INCREMENT_AND_WRAP: SDL_GPUStencilOp =
    SDL_GPUStencilOp::INCREMENT_AND_WRAP;
/// Decrements the current value and wraps to the maximum value.
pub const SDL_GPU_STENCILOP_DECREMENT_AND_WRAP: SDL_GPUStencilOp =
    SDL_GPUStencilOp::DECREMENT_AND_WRAP;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUStencilOp {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUStencilOp;
}

/// Specifies the operator to be used when pixels in a render target are
/// blended with existing pixels in the texture.
///
/// The source color is the value written by the fragment shader. The
/// destination color is the value currently existing in the texture.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUBlendOp::INVALID) | [`SDL_GPU_BLENDOP_INVALID`] | |
/// | [`ADD`](SDL_GPUBlendOp::ADD) | [`SDL_GPU_BLENDOP_ADD`] | (source * source_factor) + (destination * destination_factor) |
/// | [`SUBTRACT`](SDL_GPUBlendOp::SUBTRACT) | [`SDL_GPU_BLENDOP_SUBTRACT`] | (source * source_factor) - (destination * destination_factor) |
/// | [`REVERSE_SUBTRACT`](SDL_GPUBlendOp::REVERSE_SUBTRACT) | [`SDL_GPU_BLENDOP_REVERSE_SUBTRACT`] | (destination * destination_factor) - (source * source_factor) |
/// | [`MIN`](SDL_GPUBlendOp::MIN) | [`SDL_GPU_BLENDOP_MIN`] | min(source, destination) |
/// | [`MAX`](SDL_GPUBlendOp::MAX) | [`SDL_GPU_BLENDOP_MAX`] | max(source, destination) |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUBlendOp(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUBlendOp {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUBlendOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUBlendOp) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUBlendOp> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUBlendOp) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUBlendOp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_BLENDOP_INVALID",
            Self::ADD => "SDL_GPU_BLENDOP_ADD",
            Self::SUBTRACT => "SDL_GPU_BLENDOP_SUBTRACT",
            Self::REVERSE_SUBTRACT => "SDL_GPU_BLENDOP_REVERSE_SUBTRACT",
            Self::MIN => "SDL_GPU_BLENDOP_MIN",
            Self::MAX => "SDL_GPU_BLENDOP_MAX",

            _ => return write!(f, "SDL_GPUBlendOp({})", self.0),
        })
    }
}

impl SDL_GPUBlendOp {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    /// (source * source_factor) + (destination * destination_factor)
    pub const ADD: Self = Self((1 as ::core::ffi::c_int));
    /// (source * source_factor) - (destination * destination_factor)
    pub const SUBTRACT: Self = Self((2 as ::core::ffi::c_int));
    /// (destination * destination_factor) - (source * source_factor)
    pub const REVERSE_SUBTRACT: Self = Self((3 as ::core::ffi::c_int));
    /// min(source, destination)
    pub const MIN: Self = Self((4 as ::core::ffi::c_int));
    /// max(source, destination)
    pub const MAX: Self = Self((5 as ::core::ffi::c_int));
}

pub const SDL_GPU_BLENDOP_INVALID: SDL_GPUBlendOp = SDL_GPUBlendOp::INVALID;
/// (source * source_factor) + (destination * destination_factor)
pub const SDL_GPU_BLENDOP_ADD: SDL_GPUBlendOp = SDL_GPUBlendOp::ADD;
/// (source * source_factor) - (destination * destination_factor)
pub const SDL_GPU_BLENDOP_SUBTRACT: SDL_GPUBlendOp = SDL_GPUBlendOp::SUBTRACT;
/// (destination * destination_factor) - (source * source_factor)
pub const SDL_GPU_BLENDOP_REVERSE_SUBTRACT: SDL_GPUBlendOp = SDL_GPUBlendOp::REVERSE_SUBTRACT;
/// min(source, destination)
pub const SDL_GPU_BLENDOP_MIN: SDL_GPUBlendOp = SDL_GPUBlendOp::MIN;
/// max(source, destination)
pub const SDL_GPU_BLENDOP_MAX: SDL_GPUBlendOp = SDL_GPUBlendOp::MAX;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUBlendOp {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUBlendOp;
}

/// Specifies a blending factor to be used when pixels in a render target are
/// blended with existing pixels in the texture.
///
/// The source color is the value written by the fragment shader. The
/// destination color is the value currently existing in the texture.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_GPUBlendFactor::INVALID) | [`SDL_GPU_BLENDFACTOR_INVALID`] | |
/// | [`ZERO`](SDL_GPUBlendFactor::ZERO) | [`SDL_GPU_BLENDFACTOR_ZERO`] | 0 |
/// | [`ONE`](SDL_GPUBlendFactor::ONE) | [`SDL_GPU_BLENDFACTOR_ONE`] | 1 |
/// | [`SRC_COLOR`](SDL_GPUBlendFactor::SRC_COLOR) | [`SDL_GPU_BLENDFACTOR_SRC_COLOR`] | source color |
/// | [`ONE_MINUS_SRC_COLOR`](SDL_GPUBlendFactor::ONE_MINUS_SRC_COLOR) | [`SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR`] | 1 - source color |
/// | [`DST_COLOR`](SDL_GPUBlendFactor::DST_COLOR) | [`SDL_GPU_BLENDFACTOR_DST_COLOR`] | destination color |
/// | [`ONE_MINUS_DST_COLOR`](SDL_GPUBlendFactor::ONE_MINUS_DST_COLOR) | [`SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR`] | 1 - destination color |
/// | [`SRC_ALPHA`](SDL_GPUBlendFactor::SRC_ALPHA) | [`SDL_GPU_BLENDFACTOR_SRC_ALPHA`] | source alpha |
/// | [`ONE_MINUS_SRC_ALPHA`](SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA) | [`SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA`] | 1 - source alpha |
/// | [`DST_ALPHA`](SDL_GPUBlendFactor::DST_ALPHA) | [`SDL_GPU_BLENDFACTOR_DST_ALPHA`] | destination alpha |
/// | [`ONE_MINUS_DST_ALPHA`](SDL_GPUBlendFactor::ONE_MINUS_DST_ALPHA) | [`SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA`] | 1 - destination alpha |
/// | [`CONSTANT_COLOR`](SDL_GPUBlendFactor::CONSTANT_COLOR) | [`SDL_GPU_BLENDFACTOR_CONSTANT_COLOR`] | blend constant |
/// | [`ONE_MINUS_CONSTANT_COLOR`](SDL_GPUBlendFactor::ONE_MINUS_CONSTANT_COLOR) | [`SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR`] | 1 - blend constant |
/// | [`SRC_ALPHA_SATURATE`](SDL_GPUBlendFactor::SRC_ALPHA_SATURATE) | [`SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE`] | min(source alpha, 1 - destination alpha) |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUBlendFactor(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUBlendFactor {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUBlendFactor> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUBlendFactor) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUBlendFactor> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUBlendFactor) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUBlendFactor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_GPU_BLENDFACTOR_INVALID",
            Self::ZERO => "SDL_GPU_BLENDFACTOR_ZERO",
            Self::ONE => "SDL_GPU_BLENDFACTOR_ONE",
            Self::SRC_COLOR => "SDL_GPU_BLENDFACTOR_SRC_COLOR",
            Self::ONE_MINUS_SRC_COLOR => "SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR",
            Self::DST_COLOR => "SDL_GPU_BLENDFACTOR_DST_COLOR",
            Self::ONE_MINUS_DST_COLOR => "SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR",
            Self::SRC_ALPHA => "SDL_GPU_BLENDFACTOR_SRC_ALPHA",
            Self::ONE_MINUS_SRC_ALPHA => "SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA",
            Self::DST_ALPHA => "SDL_GPU_BLENDFACTOR_DST_ALPHA",
            Self::ONE_MINUS_DST_ALPHA => "SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA",
            Self::CONSTANT_COLOR => "SDL_GPU_BLENDFACTOR_CONSTANT_COLOR",
            Self::ONE_MINUS_CONSTANT_COLOR => "SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR",
            Self::SRC_ALPHA_SATURATE => "SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE",

            _ => return write!(f, "SDL_GPUBlendFactor({})", self.0),
        })
    }
}

impl SDL_GPUBlendFactor {
    pub const INVALID: Self = Self((0 as ::core::ffi::c_int));
    /// 0
    pub const ZERO: Self = Self((1 as ::core::ffi::c_int));
    /// 1
    pub const ONE: Self = Self((2 as ::core::ffi::c_int));
    /// source color
    pub const SRC_COLOR: Self = Self((3 as ::core::ffi::c_int));
    /// 1 - source color
    pub const ONE_MINUS_SRC_COLOR: Self = Self((4 as ::core::ffi::c_int));
    /// destination color
    pub const DST_COLOR: Self = Self((5 as ::core::ffi::c_int));
    /// 1 - destination color
    pub const ONE_MINUS_DST_COLOR: Self = Self((6 as ::core::ffi::c_int));
    /// source alpha
    pub const SRC_ALPHA: Self = Self((7 as ::core::ffi::c_int));
    /// 1 - source alpha
    pub const ONE_MINUS_SRC_ALPHA: Self = Self((8 as ::core::ffi::c_int));
    /// destination alpha
    pub const DST_ALPHA: Self = Self((9 as ::core::ffi::c_int));
    /// 1 - destination alpha
    pub const ONE_MINUS_DST_ALPHA: Self = Self((10 as ::core::ffi::c_int));
    /// blend constant
    pub const CONSTANT_COLOR: Self = Self((11 as ::core::ffi::c_int));
    /// 1 - blend constant
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self((12 as ::core::ffi::c_int));
    /// min(source alpha, 1 - destination alpha)
    pub const SRC_ALPHA_SATURATE: Self = Self((13 as ::core::ffi::c_int));
}

pub const SDL_GPU_BLENDFACTOR_INVALID: SDL_GPUBlendFactor = SDL_GPUBlendFactor::INVALID;
/// 0
pub const SDL_GPU_BLENDFACTOR_ZERO: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ZERO;
/// 1
pub const SDL_GPU_BLENDFACTOR_ONE: SDL_GPUBlendFactor = SDL_GPUBlendFactor::ONE;
/// source color
pub const SDL_GPU_BLENDFACTOR_SRC_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::SRC_COLOR;
/// 1 - source color
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::ONE_MINUS_SRC_COLOR;
/// destination color
pub const SDL_GPU_BLENDFACTOR_DST_COLOR: SDL_GPUBlendFactor = SDL_GPUBlendFactor::DST_COLOR;
/// 1 - destination color
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::ONE_MINUS_DST_COLOR;
/// source alpha
pub const SDL_GPU_BLENDFACTOR_SRC_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::SRC_ALPHA;
/// 1 - source alpha
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA;
/// destination alpha
pub const SDL_GPU_BLENDFACTOR_DST_ALPHA: SDL_GPUBlendFactor = SDL_GPUBlendFactor::DST_ALPHA;
/// 1 - destination alpha
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::ONE_MINUS_DST_ALPHA;
/// blend constant
pub const SDL_GPU_BLENDFACTOR_CONSTANT_COLOR: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::CONSTANT_COLOR;
/// 1 - blend constant
pub const SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::ONE_MINUS_CONSTANT_COLOR;
/// min(source alpha, 1 - destination alpha)
pub const SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE: SDL_GPUBlendFactor =
    SDL_GPUBlendFactor::SRC_ALPHA_SATURATE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUBlendFactor {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUBlendFactor;
}

/// Specifies which color components are written in a graphics pipeline.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`R`](SDL_GPUColorComponentFlags::R) | [`SDL_GPU_COLORCOMPONENT_R`] | the red component |
/// | [`G`](SDL_GPUColorComponentFlags::G) | [`SDL_GPU_COLORCOMPONENT_G`] | the green component |
/// | [`B`](SDL_GPUColorComponentFlags::B) | [`SDL_GPU_COLORCOMPONENT_B`] | the blue component |
/// | [`A`](SDL_GPUColorComponentFlags::A) | [`SDL_GPU_COLORCOMPONENT_A`] | the alpha component |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SDL_GPUColorComponentFlags(pub Uint8);

impl ::core::cmp::PartialEq<Uint8> for SDL_GPUColorComponentFlags {
    #[inline(always)]
    fn eq(&self, other: &Uint8) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUColorComponentFlags> for Uint8 {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUColorComponentFlags) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUColorComponentFlags> for Uint8 {
    #[inline(always)]
    fn from(value: SDL_GPUColorComponentFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUColorComponentFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut first = true;
        let all_bits = 0;
        write!(f, "SDL_GPUColorComponentFlags(")?;
        let all_bits = all_bits | Self::R.0;
        if (Self::R != 0 || self.0 == 0) && *self & Self::R == Self::R {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "R")?;
        }
        let all_bits = all_bits | Self::G.0;
        if (Self::G != 0 || self.0 == 0) && *self & Self::G == Self::G {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "G")?;
        }
        let all_bits = all_bits | Self::B.0;
        if (Self::B != 0 || self.0 == 0) && *self & Self::B == Self::B {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "B")?;
        }
        let all_bits = all_bits | Self::A.0;
        if (Self::A != 0 || self.0 == 0) && *self & Self::A == Self::A {
            if !first {
                write!(f, " | ")?;
            }
            first = false;
            write!(f, "A")?;
        }

        if self.0 & !all_bits != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{:#x}", self.0)?;
        } else if first {
            write!(f, "0")?;
        }
        write!(f, ")")
    }
}

impl ::core::ops::BitAnd for SDL_GPUColorComponentFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ::core::ops::BitAndAssign for SDL_GPUColorComponentFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ::core::ops::BitOr for SDL_GPUColorComponentFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ::core::ops::BitOrAssign for SDL_GPUColorComponentFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ::core::ops::BitXor for SDL_GPUColorComponentFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ::core::ops::BitXorAssign for SDL_GPUColorComponentFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ::core::ops::Not for SDL_GPUColorComponentFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl SDL_GPUColorComponentFlags {
    /// the red component
    pub const R: Self = Self((1_u32 as Uint8));
    /// the green component
    pub const G: Self = Self((2_u32 as Uint8));
    /// the blue component
    pub const B: Self = Self((4_u32 as Uint8));
    /// the alpha component
    pub const A: Self = Self((8_u32 as Uint8));
}

/// the red component
pub const SDL_GPU_COLORCOMPONENT_R: SDL_GPUColorComponentFlags = SDL_GPUColorComponentFlags::R;
/// the green component
pub const SDL_GPU_COLORCOMPONENT_G: SDL_GPUColorComponentFlags = SDL_GPUColorComponentFlags::G;
/// the blue component
pub const SDL_GPU_COLORCOMPONENT_B: SDL_GPUColorComponentFlags = SDL_GPUColorComponentFlags::B;
/// the alpha component
pub const SDL_GPU_COLORCOMPONENT_A: SDL_GPUColorComponentFlags = SDL_GPUColorComponentFlags::A;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUColorComponentFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUColorComponentFlags;
}

/// Specifies a filter operation used by a sampler.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUSampler`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NEAREST`](SDL_GPUFilter::NEAREST) | [`SDL_GPU_FILTER_NEAREST`] | Point filtering. |
/// | [`LINEAR`](SDL_GPUFilter::LINEAR) | [`SDL_GPU_FILTER_LINEAR`] | Linear filtering. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUFilter(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUFilter {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUFilter> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUFilter) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUFilter> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUFilter) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NEAREST => "SDL_GPU_FILTER_NEAREST",
            Self::LINEAR => "SDL_GPU_FILTER_LINEAR",

            _ => return write!(f, "SDL_GPUFilter({})", self.0),
        })
    }
}

impl SDL_GPUFilter {
    /// Point filtering.
    pub const NEAREST: Self = Self((0 as ::core::ffi::c_int));
    /// Linear filtering.
    pub const LINEAR: Self = Self((1 as ::core::ffi::c_int));
}

/// Point filtering.
pub const SDL_GPU_FILTER_NEAREST: SDL_GPUFilter = SDL_GPUFilter::NEAREST;
/// Linear filtering.
pub const SDL_GPU_FILTER_LINEAR: SDL_GPUFilter = SDL_GPUFilter::LINEAR;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUFilter {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUFilter;
}

/// Specifies a mipmap mode used by a sampler.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUSampler`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NEAREST`](SDL_GPUSamplerMipmapMode::NEAREST) | [`SDL_GPU_SAMPLERMIPMAPMODE_NEAREST`] | Point filtering. |
/// | [`LINEAR`](SDL_GPUSamplerMipmapMode::LINEAR) | [`SDL_GPU_SAMPLERMIPMAPMODE_LINEAR`] | Linear filtering. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUSamplerMipmapMode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUSamplerMipmapMode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUSamplerMipmapMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUSamplerMipmapMode) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUSamplerMipmapMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUSamplerMipmapMode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUSamplerMipmapMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NEAREST => "SDL_GPU_SAMPLERMIPMAPMODE_NEAREST",
            Self::LINEAR => "SDL_GPU_SAMPLERMIPMAPMODE_LINEAR",

            _ => return write!(f, "SDL_GPUSamplerMipmapMode({})", self.0),
        })
    }
}

impl SDL_GPUSamplerMipmapMode {
    /// Point filtering.
    pub const NEAREST: Self = Self((0 as ::core::ffi::c_int));
    /// Linear filtering.
    pub const LINEAR: Self = Self((1 as ::core::ffi::c_int));
}

/// Point filtering.
pub const SDL_GPU_SAMPLERMIPMAPMODE_NEAREST: SDL_GPUSamplerMipmapMode =
    SDL_GPUSamplerMipmapMode::NEAREST;
/// Linear filtering.
pub const SDL_GPU_SAMPLERMIPMAPMODE_LINEAR: SDL_GPUSamplerMipmapMode =
    SDL_GPUSamplerMipmapMode::LINEAR;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUSamplerMipmapMode {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUSamplerMipmapMode;
}

/// Specifies behavior of texture sampling when the coordinates exceed the 0-1
/// range.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUSampler`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`REPEAT`](SDL_GPUSamplerAddressMode::REPEAT) | [`SDL_GPU_SAMPLERADDRESSMODE_REPEAT`] | Specifies that the coordinates will wrap around. |
/// | [`MIRRORED_REPEAT`](SDL_GPUSamplerAddressMode::MIRRORED_REPEAT) | [`SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT`] | Specifies that the coordinates will wrap around mirrored. |
/// | [`CLAMP_TO_EDGE`](SDL_GPUSamplerAddressMode::CLAMP_TO_EDGE) | [`SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE`] | Specifies that the coordinates will clamp to the 0-1 range. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUSamplerAddressMode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUSamplerAddressMode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUSamplerAddressMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUSamplerAddressMode) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUSamplerAddressMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUSamplerAddressMode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUSamplerAddressMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::REPEAT => "SDL_GPU_SAMPLERADDRESSMODE_REPEAT",
            Self::MIRRORED_REPEAT => "SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT",
            Self::CLAMP_TO_EDGE => "SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE",

            _ => return write!(f, "SDL_GPUSamplerAddressMode({})", self.0),
        })
    }
}

impl SDL_GPUSamplerAddressMode {
    /// Specifies that the coordinates will wrap around.
    pub const REPEAT: Self = Self((0 as ::core::ffi::c_int));
    /// Specifies that the coordinates will wrap around mirrored.
    pub const MIRRORED_REPEAT: Self = Self((1 as ::core::ffi::c_int));
    /// Specifies that the coordinates will clamp to the 0-1 range.
    pub const CLAMP_TO_EDGE: Self = Self((2 as ::core::ffi::c_int));
}

/// Specifies that the coordinates will wrap around.
pub const SDL_GPU_SAMPLERADDRESSMODE_REPEAT: SDL_GPUSamplerAddressMode =
    SDL_GPUSamplerAddressMode::REPEAT;
/// Specifies that the coordinates will wrap around mirrored.
pub const SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT: SDL_GPUSamplerAddressMode =
    SDL_GPUSamplerAddressMode::MIRRORED_REPEAT;
/// Specifies that the coordinates will clamp to the 0-1 range.
pub const SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE: SDL_GPUSamplerAddressMode =
    SDL_GPUSamplerAddressMode::CLAMP_TO_EDGE;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUSamplerAddressMode {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUSamplerAddressMode;
}

/// Specifies the timing that will be used to present swapchain textures to the
/// OS.
///
/// VSYNC mode will always be supported. IMMEDIATE and MAILBOX modes may not be
/// supported on certain systems.
///
/// It is recommended to query [`SDL_WindowSupportsGPUPresentMode`] after claiming
/// the window if you wish to change the present mode to IMMEDIATE or MAILBOX.
///
/// - VSYNC: Waits for vblank before presenting. No tearing is possible. If
///   there is a pending image to present, the new image is enqueued for
///   presentation. Disallows tearing at the cost of visual latency.
/// - IMMEDIATE: Immediately presents. Lowest latency option, but tearing may
///   occur.
/// - MAILBOX: Waits for vblank before presenting. No tearing is possible. If
///   there is a pending image to present, the pending image is replaced by the
///   new image. Similar to VSYNC, but with reduced visual latency.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_SetGPUSwapchainParameters`]
/// - [`SDL_WindowSupportsGPUPresentMode`]
/// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`VSYNC`](SDL_GPUPresentMode::VSYNC) | [`SDL_GPU_PRESENTMODE_VSYNC`] | |
/// | [`IMMEDIATE`](SDL_GPUPresentMode::IMMEDIATE) | [`SDL_GPU_PRESENTMODE_IMMEDIATE`] | |
/// | [`MAILBOX`](SDL_GPUPresentMode::MAILBOX) | [`SDL_GPU_PRESENTMODE_MAILBOX`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUPresentMode(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUPresentMode {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUPresentMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUPresentMode) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUPresentMode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUPresentMode) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUPresentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::VSYNC => "SDL_GPU_PRESENTMODE_VSYNC",
            Self::IMMEDIATE => "SDL_GPU_PRESENTMODE_IMMEDIATE",
            Self::MAILBOX => "SDL_GPU_PRESENTMODE_MAILBOX",

            _ => return write!(f, "SDL_GPUPresentMode({})", self.0),
        })
    }
}

impl SDL_GPUPresentMode {
    pub const VSYNC: Self = Self((0 as ::core::ffi::c_int));
    pub const IMMEDIATE: Self = Self((1 as ::core::ffi::c_int));
    pub const MAILBOX: Self = Self((2 as ::core::ffi::c_int));
}

pub const SDL_GPU_PRESENTMODE_VSYNC: SDL_GPUPresentMode = SDL_GPUPresentMode::VSYNC;
pub const SDL_GPU_PRESENTMODE_IMMEDIATE: SDL_GPUPresentMode = SDL_GPUPresentMode::IMMEDIATE;
pub const SDL_GPU_PRESENTMODE_MAILBOX: SDL_GPUPresentMode = SDL_GPUPresentMode::MAILBOX;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUPresentMode {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUPresentMode;
}

/// Specifies the texture format and colorspace of the swapchain textures.
///
/// SDR will always be supported. Other compositions may not be supported on
/// certain systems.
///
/// It is recommended to query [`SDL_WindowSupportsGPUSwapchainComposition`] after
/// claiming the window if you wish to change the swapchain composition from
/// SDR.
///
/// - SDR: B8G8R8A8 or R8G8B8A8 swapchain. Pixel values are in sRGB encoding.
/// - SDR_LINEAR: B8G8R8A8_SRGB or R8G8B8A8_SRGB swapchain. Pixel values are
///   stored in memory in sRGB encoding but accessed in shaders in "linear
///   sRGB" encoding which is sRGB but with a linear transfer function.
/// - HDR_EXTENDED_LINEAR: R16G16B16A16_FLOAT swapchain. Pixel values are in
///   extended linear sRGB encoding and permits values outside of the \[0, 1\]
///   range.
/// - HDR10_ST2084: A2R10G10B10 or A2B10G10R10 swapchain. Pixel values are in
///   BT.2020 ST2084 (PQ) encoding.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_SetGPUSwapchainParameters`]
/// - [`SDL_WindowSupportsGPUSwapchainComposition`]
/// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`SDR`](SDL_GPUSwapchainComposition::SDR) | [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR`] | |
/// | [`SDR_LINEAR`](SDL_GPUSwapchainComposition::SDR_LINEAR) | [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR_LINEAR`] | |
/// | [`HDR_EXTENDED_LINEAR`](SDL_GPUSwapchainComposition::HDR_EXTENDED_LINEAR) | [`SDL_GPU_SWAPCHAINCOMPOSITION_HDR_EXTENDED_LINEAR`] | |
/// | [`HDR10_ST2084`](SDL_GPUSwapchainComposition::HDR10_ST2084) | [`SDL_GPU_SWAPCHAINCOMPOSITION_HDR10_ST2084`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GPUSwapchainComposition(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_GPUSwapchainComposition {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_GPUSwapchainComposition> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_GPUSwapchainComposition) -> bool {
        self == &other.0
    }
}

impl From<SDL_GPUSwapchainComposition> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_GPUSwapchainComposition) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_GPUSwapchainComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::SDR => "SDL_GPU_SWAPCHAINCOMPOSITION_SDR",
            Self::SDR_LINEAR => "SDL_GPU_SWAPCHAINCOMPOSITION_SDR_LINEAR",
            Self::HDR_EXTENDED_LINEAR => "SDL_GPU_SWAPCHAINCOMPOSITION_HDR_EXTENDED_LINEAR",
            Self::HDR10_ST2084 => "SDL_GPU_SWAPCHAINCOMPOSITION_HDR10_ST2084",

            _ => return write!(f, "SDL_GPUSwapchainComposition({})", self.0),
        })
    }
}

impl SDL_GPUSwapchainComposition {
    pub const SDR: Self = Self((0 as ::core::ffi::c_int));
    pub const SDR_LINEAR: Self = Self((1 as ::core::ffi::c_int));
    pub const HDR_EXTENDED_LINEAR: Self = Self((2 as ::core::ffi::c_int));
    pub const HDR10_ST2084: Self = Self((3 as ::core::ffi::c_int));
}

pub const SDL_GPU_SWAPCHAINCOMPOSITION_SDR: SDL_GPUSwapchainComposition =
    SDL_GPUSwapchainComposition::SDR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_SDR_LINEAR: SDL_GPUSwapchainComposition =
    SDL_GPUSwapchainComposition::SDR_LINEAR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_HDR_EXTENDED_LINEAR: SDL_GPUSwapchainComposition =
    SDL_GPUSwapchainComposition::HDR_EXTENDED_LINEAR;
pub const SDL_GPU_SWAPCHAINCOMPOSITION_HDR10_ST2084: SDL_GPUSwapchainComposition =
    SDL_GPUSwapchainComposition::HDR10_ST2084;

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::HasGroupMetadata for SDL_GPUSwapchainComposition {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::gpu::METADATA_SDL_GPUSwapchainComposition;
}

/// A structure specifying a viewport.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_SetGPUViewport`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUViewport {
    /// The left offset of the viewport.
    pub x: ::core::ffi::c_float,
    /// The top offset of the viewport.
    pub y: ::core::ffi::c_float,
    /// The width of the viewport.
    pub w: ::core::ffi::c_float,
    /// The height of the viewport.
    pub h: ::core::ffi::c_float,
    /// The minimum depth of the viewport.
    pub min_depth: ::core::ffi::c_float,
    /// The maximum depth of the viewport.
    pub max_depth: ::core::ffi::c_float,
}

/// A structure specifying parameters related to transferring data to or from a
/// texture.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_UploadToGPUTexture`]
/// - [`SDL_DownloadFromGPUTexture`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureTransferInfo {
    /// The transfer buffer used in the transfer operation.
    pub transfer_buffer: *mut SDL_GPUTransferBuffer,
    /// The starting byte of the image data in the transfer buffer.
    pub offset: Uint32,
    /// The number of pixels from one row to the next.
    pub pixels_per_row: Uint32,
    /// The number of rows from one layer/depth-slice to the next.
    pub rows_per_layer: Uint32,
}

impl ::core::default::Default for SDL_GPUTextureTransferInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a location in a transfer buffer.
///
/// Used when transferring buffer data to or from a transfer buffer.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_UploadToGPUBuffer`]
/// - [`SDL_DownloadFromGPUBuffer`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTransferBufferLocation {
    /// The transfer buffer used in the transfer operation.
    pub transfer_buffer: *mut SDL_GPUTransferBuffer,
    /// The starting byte of the buffer data in the transfer buffer.
    pub offset: Uint32,
}

impl ::core::default::Default for SDL_GPUTransferBufferLocation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a location in a texture.
///
/// Used when copying data from one texture to another.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CopyGPUTextureToTexture`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureLocation {
    /// The texture used in the copy operation.
    pub texture: *mut SDL_GPUTexture,
    /// The mip level index of the location.
    pub mip_level: Uint32,
    /// The layer index of the location.
    pub layer: Uint32,
    /// The left offset of the location.
    pub x: Uint32,
    /// The top offset of the location.
    pub y: Uint32,
    /// The front offset of the location.
    pub z: Uint32,
}

impl ::core::default::Default for SDL_GPUTextureLocation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a region of a texture.
///
/// Used when transferring data to or from a texture.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_UploadToGPUTexture`]
/// - [`SDL_DownloadFromGPUTexture`]
/// - [`SDL_CreateGPUTexture`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureRegion {
    /// The texture used in the copy operation.
    pub texture: *mut SDL_GPUTexture,
    /// The mip level index to transfer.
    pub mip_level: Uint32,
    /// The layer index to transfer.
    pub layer: Uint32,
    /// The left offset of the region.
    pub x: Uint32,
    /// The top offset of the region.
    pub y: Uint32,
    /// The front offset of the region.
    pub z: Uint32,
    /// The width of the region.
    pub w: Uint32,
    /// The height of the region.
    pub h: Uint32,
    /// The depth of the region.
    pub d: Uint32,
}

impl ::core::default::Default for SDL_GPUTextureRegion {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a region of a texture used in the blit operation.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BlitGPUTexture`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBlitRegion {
    /// The texture.
    pub texture: *mut SDL_GPUTexture,
    /// The mip level index of the region.
    pub mip_level: Uint32,
    /// The layer index or depth plane of the region. This value is treated as a layer index on 2D array and cube textures, and as a depth plane on 3D textures.
    pub layer_or_depth_plane: Uint32,
    /// The left offset of the region.
    pub x: Uint32,
    /// The top offset of the region.
    pub y: Uint32,
    /// The width of the region.
    pub w: Uint32,
    /// The height of the region.
    pub h: Uint32,
}

impl ::core::default::Default for SDL_GPUBlitRegion {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a location in a buffer.
///
/// Used when copying data between buffers.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CopyGPUBufferToBuffer`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferLocation {
    /// The buffer.
    pub buffer: *mut SDL_GPUBuffer,
    /// The starting byte within the buffer.
    pub offset: Uint32,
}

impl ::core::default::Default for SDL_GPUBufferLocation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying a region of a buffer.
///
/// Used when transferring data to or from buffers.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_UploadToGPUBuffer`]
/// - [`SDL_DownloadFromGPUBuffer`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferRegion {
    /// The buffer.
    pub buffer: *mut SDL_GPUBuffer,
    /// The starting byte within the buffer.
    pub offset: Uint32,
    /// The size in bytes of the region.
    pub size: Uint32,
}

impl ::core::default::Default for SDL_GPUBufferRegion {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of an indirect draw command.
///
/// Note that the `first_vertex` and `first_instance` parameters are NOT
/// compatible with built-in vertex/instance ID variables in shaders (for
/// example, SV_VertexID); GPU APIs and shader languages do not define these
/// built-in variables consistently, so if your shader depends on them, the
/// only way to keep behavior consistent and portable is to always pass 0 for
/// the correlating parameter in the draw calls.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_DrawGPUPrimitivesIndirect`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndirectDrawCommand {
    /// The number of vertices to draw.
    pub num_vertices: Uint32,
    /// The number of instances to draw.
    pub num_instances: Uint32,
    /// The index of the first vertex to draw.
    pub first_vertex: Uint32,
    /// The ID of the first instance to draw.
    pub first_instance: Uint32,
}

/// A structure specifying the parameters of an indexed indirect draw command.
///
/// Note that the `first_vertex` and `first_instance` parameters are NOT
/// compatible with built-in vertex/instance ID variables in shaders (for
/// example, SV_VertexID); GPU APIs and shader languages do not define these
/// built-in variables consistently, so if your shader depends on them, the
/// only way to keep behavior consistent and portable is to always pass 0 for
/// the correlating parameter in the draw calls.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_DrawGPUIndexedPrimitivesIndirect`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndexedIndirectDrawCommand {
    /// The number of indices to draw per instance.
    pub num_indices: Uint32,
    /// The number of instances to draw.
    pub num_instances: Uint32,
    /// The base index within the index buffer.
    pub first_index: Uint32,
    /// The value added to the vertex index before indexing into the vertex buffer.
    pub vertex_offset: Sint32,
    /// The ID of the first instance to draw.
    pub first_instance: Uint32,
}

/// A structure specifying the parameters of an indexed dispatch command.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_DispatchGPUComputeIndirect`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUIndirectDispatchCommand {
    /// The number of local workgroups to dispatch in the X dimension.
    pub groupcount_x: Uint32,
    /// The number of local workgroups to dispatch in the Y dimension.
    pub groupcount_y: Uint32,
    /// The number of local workgroups to dispatch in the Z dimension.
    pub groupcount_z: Uint32,
}

/// A structure specifying the parameters of a sampler.
///
/// Note that mip_lod_bias is a no-op for the Metal driver. For Metal, LOD bias
/// must be applied via shader instead.
///
/// ## Availability
/// This function is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUSampler`]
/// - [`SDL_GPUFilter`]
/// - [`SDL_GPUSamplerMipmapMode`]
/// - [`SDL_GPUSamplerAddressMode`]
/// - [`SDL_GPUCompareOp`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUSamplerCreateInfo {
    /// The minification filter to apply to lookups.
    pub min_filter: SDL_GPUFilter,
    /// The magnification filter to apply to lookups.
    pub mag_filter: SDL_GPUFilter,
    /// The mipmap filter to apply to lookups.
    pub mipmap_mode: SDL_GPUSamplerMipmapMode,
    /// The addressing mode for U coordinates outside [0, 1).
    pub address_mode_u: SDL_GPUSamplerAddressMode,
    /// The addressing mode for V coordinates outside [0, 1).
    pub address_mode_v: SDL_GPUSamplerAddressMode,
    /// The addressing mode for W coordinates outside [0, 1).
    pub address_mode_w: SDL_GPUSamplerAddressMode,
    /// The bias to be added to mipmap LOD calculation.
    pub mip_lod_bias: ::core::ffi::c_float,
    /// The anisotropy value clamp used by the sampler. If enable_anisotropy is false, this is ignored.
    pub max_anisotropy: ::core::ffi::c_float,
    /// The comparison operator to apply to fetched data before filtering.
    pub compare_op: SDL_GPUCompareOp,
    /// Clamps the minimum of the computed LOD value.
    pub min_lod: ::core::ffi::c_float,
    /// Clamps the maximum of the computed LOD value.
    pub max_lod: ::core::ffi::c_float,
    /// true to enable anisotropic filtering.
    pub enable_anisotropy: ::core::primitive::bool,
    /// true to enable comparison against a reference value during lookups.
    pub enable_compare: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

/// A structure specifying the parameters of vertex buffers used in a graphics
/// pipeline.
///
/// When you call [`SDL_BindGPUVertexBuffers`], you specify the binding slots of
/// the vertex buffers. For example if you called [`SDL_BindGPUVertexBuffers`] with
/// a first_slot of 2 and num_bindings of 3, the binding slots 2, 3, 4 would be
/// used by the vertex buffers you pass in.
///
/// Vertex attributes are linked to buffers via the buffer_slot field of
/// [`SDL_GPUVertexAttribute`]. For example, if an attribute has a buffer_slot of
/// 0, then that attribute belongs to the vertex buffer bound at slot 0.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUVertexAttribute`]
/// - [`SDL_GPUVertexInputRate`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexBufferDescription {
    /// The binding slot of the vertex buffer.
    pub slot: Uint32,
    /// The byte pitch between consecutive elements of the vertex buffer.
    pub pitch: Uint32,
    /// Whether attribute addressing is a function of the vertex index or instance index.
    pub input_rate: SDL_GPUVertexInputRate,
    /// Reserved for future use. Must be set to 0.
    pub instance_step_rate: Uint32,
}

/// A structure specifying a vertex attribute.
///
/// All vertex attribute locations provided to an [`SDL_GPUVertexInputState`] must
/// be unique.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUVertexBufferDescription`]
/// - [`SDL_GPUVertexInputState`]
/// - [`SDL_GPUVertexElementFormat`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexAttribute {
    /// The shader input location index.
    pub location: Uint32,
    /// The binding slot of the associated vertex buffer.
    pub buffer_slot: Uint32,
    /// The size and type of the attribute data.
    pub format: SDL_GPUVertexElementFormat,
    /// The byte offset of this attribute relative to the start of the vertex element.
    pub offset: Uint32,
}

/// A structure specifying the parameters of a graphics pipeline vertex input
/// state.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineCreateInfo`]
/// - [`SDL_GPUVertexBufferDescription`]
/// - [`SDL_GPUVertexAttribute`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUVertexInputState {
    /// A pointer to an array of vertex buffer descriptions.
    pub vertex_buffer_descriptions: *const SDL_GPUVertexBufferDescription,
    /// The number of vertex buffer descriptions in the above array.
    pub num_vertex_buffers: Uint32,
    /// A pointer to an array of vertex attribute descriptions.
    pub vertex_attributes: *const SDL_GPUVertexAttribute,
    /// The number of vertex attribute descriptions in the above array.
    pub num_vertex_attributes: Uint32,
}

impl ::core::default::Default for SDL_GPUVertexInputState {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the stencil operation state of a graphics pipeline.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUDepthStencilState`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStencilOpState {
    /// The action performed on samples that fail the stencil test.
    pub fail_op: SDL_GPUStencilOp,
    /// The action performed on samples that pass the depth and stencil tests.
    pub pass_op: SDL_GPUStencilOp,
    /// The action performed on samples that pass the stencil test and fail the depth test.
    pub depth_fail_op: SDL_GPUStencilOp,
    /// The comparison operator used in the stencil test.
    pub compare_op: SDL_GPUCompareOp,
}

/// A structure specifying the blend state of a color target.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUColorTargetDescription`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetBlendState {
    /// The value to be multiplied by the source RGB value.
    pub src_color_blendfactor: SDL_GPUBlendFactor,
    /// The value to be multiplied by the destination RGB value.
    pub dst_color_blendfactor: SDL_GPUBlendFactor,
    /// The blend operation for the RGB components.
    pub color_blend_op: SDL_GPUBlendOp,
    /// The value to be multiplied by the source alpha.
    pub src_alpha_blendfactor: SDL_GPUBlendFactor,
    /// The value to be multiplied by the destination alpha.
    pub dst_alpha_blendfactor: SDL_GPUBlendFactor,
    /// The blend operation for the alpha component.
    pub alpha_blend_op: SDL_GPUBlendOp,
    /// A bitmask specifying which of the RGBA components are enabled for writing. Writes to all channels if enable_color_write_mask is false.
    pub color_write_mask: SDL_GPUColorComponentFlags,
    /// Whether blending is enabled for the color target.
    pub enable_blend: ::core::primitive::bool,
    /// Whether the color write mask is enabled.
    pub enable_color_write_mask: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
}

/// A structure specifying code and metadata for creating a shader object.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUShader`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUShaderCreateInfo {
    /// The size in bytes of the code pointed to.
    pub code_size: ::core::primitive::usize,
    /// A pointer to shader code.
    pub code: *const Uint8,
    /// A pointer to a null-terminated UTF-8 string specifying the entry point function name for the shader.
    pub entrypoint: *const ::core::ffi::c_char,
    /// The format of the shader code.
    pub format: SDL_GPUShaderFormat,
    /// The stage the shader program corresponds to.
    pub stage: SDL_GPUShaderStage,
    /// The number of samplers defined in the shader.
    pub num_samplers: Uint32,
    /// The number of storage textures defined in the shader.
    pub num_storage_textures: Uint32,
    /// The number of storage buffers defined in the shader.
    pub num_storage_buffers: Uint32,
    /// The number of uniform buffers defined in the shader.
    pub num_uniform_buffers: Uint32,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

impl ::core::default::Default for SDL_GPUShaderCreateInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of a texture.
///
/// Usage flags can be bitwise OR'd together for combinations of usages. Note
/// that certain usage combinations are invalid, for example SAMPLER and
/// GRAPHICS_STORAGE.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
/// - [`SDL_GPUTextureType`]
/// - [`SDL_GPUTextureFormat`]
/// - [`SDL_GPUTextureUsageFlags`]
/// - [`SDL_GPUSampleCount`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureCreateInfo {
    /// The base dimensionality of the texture.
    pub r#type: SDL_GPUTextureType,
    /// The pixel format of the texture.
    pub format: SDL_GPUTextureFormat,
    /// How the texture is intended to be used by the client.
    pub usage: SDL_GPUTextureUsageFlags,
    /// The width of the texture.
    pub width: Uint32,
    /// The height of the texture.
    pub height: Uint32,
    /// The layer count or depth of the texture. This value is treated as a layer count on 2D array textures, and as a depth value on 3D textures.
    pub layer_count_or_depth: Uint32,
    /// The number of mip levels in the texture.
    pub num_levels: Uint32,
    /// The number of samples per texel. Only applies if the texture is used as a render target.
    pub sample_count: SDL_GPUSampleCount,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

/// A structure specifying the parameters of a buffer.
///
/// Usage flags can be bitwise OR'd together for combinations of usages. Note
/// that certain combinations are invalid, for example VERTEX and INDEX.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUBuffer`]
/// - [`SDL_GPUBufferUsageFlags`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferCreateInfo {
    /// How the buffer is intended to be used by the client.
    pub usage: SDL_GPUBufferUsageFlags,
    /// The size in bytes of the buffer.
    pub size: Uint32,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

/// A structure specifying the parameters of a transfer buffer.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTransferBuffer`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTransferBufferCreateInfo {
    /// How the transfer buffer is intended to be used by the client.
    pub usage: SDL_GPUTransferBufferUsage,
    /// The size in bytes of the transfer buffer.
    pub size: Uint32,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

/// A structure specifying the parameters of the graphics pipeline rasterizer
/// state.
///
/// Note that [`SDL_GPU_FILLMODE_LINE`] is not supported on many Android devices.
/// For those devices, the fill mode will automatically fall back to FILL.
///
/// Also note that the D3D12 driver will enable depth clamping even if
/// enable_depth_clip is true. If you need this clamp+clip behavior, consider
/// enabling depth clip and then manually clamping depth in your fragment
/// shaders on Metal and Vulkan.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineCreateInfo`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPURasterizerState {
    /// Whether polygons will be filled in or drawn as lines.
    pub fill_mode: SDL_GPUFillMode,
    /// The facing direction in which triangles will be culled.
    pub cull_mode: SDL_GPUCullMode,
    /// The vertex winding that will cause a triangle to be determined as front-facing.
    pub front_face: SDL_GPUFrontFace,
    /// A scalar factor controlling the depth value added to each fragment.
    pub depth_bias_constant_factor: ::core::ffi::c_float,
    /// The maximum depth bias of a fragment.
    pub depth_bias_clamp: ::core::ffi::c_float,
    /// A scalar factor applied to a fragment's slope in depth calculations.
    pub depth_bias_slope_factor: ::core::ffi::c_float,
    /// true to bias fragment depth values.
    pub enable_depth_bias: ::core::primitive::bool,
    /// true to enable depth clip, false to enable depth clamp.
    pub enable_depth_clip: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
}

/// A structure specifying the parameters of the graphics pipeline multisample
/// state.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineCreateInfo`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUMultisampleState {
    /// The number of samples to be used in rasterization.
    pub sample_count: SDL_GPUSampleCount,
    /// Reserved for future use. Must be set to 0.
    pub sample_mask: Uint32,
    /// Reserved for future use. Must be set to false.
    pub enable_mask: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

/// A structure specifying the parameters of the graphics pipeline depth
/// stencil state.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineCreateInfo`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDepthStencilState {
    /// The comparison operator used for depth testing.
    pub compare_op: SDL_GPUCompareOp,
    /// The stencil op state for back-facing triangles.
    pub back_stencil_state: SDL_GPUStencilOpState,
    /// The stencil op state for front-facing triangles.
    pub front_stencil_state: SDL_GPUStencilOpState,
    /// Selects the bits of the stencil values participating in the stencil test.
    pub compare_mask: Uint8,
    /// Selects the bits of the stencil values updated by the stencil test.
    pub write_mask: Uint8,
    /// true enables the depth test.
    pub enable_depth_test: ::core::primitive::bool,
    /// true enables depth writes. Depth writes are always disabled when enable_depth_test is false.
    pub enable_depth_write: ::core::primitive::bool,
    /// true enables the stencil test.
    pub enable_stencil_test: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

/// A structure specifying the parameters of color targets used in a graphics
/// pipeline.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineTargetInfo`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetDescription {
    /// The pixel format of the texture to be used as a color target.
    pub format: SDL_GPUTextureFormat,
    /// The blend state to be used for the color target.
    pub blend_state: SDL_GPUColorTargetBlendState,
}

/// A structure specifying the descriptions of render targets used in a
/// graphics pipeline.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GPUGraphicsPipelineCreateInfo`]
/// - [`SDL_GPUColorTargetDescription`]
/// - [`SDL_GPUTextureFormat`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUGraphicsPipelineTargetInfo {
    /// A pointer to an array of color target descriptions.
    pub color_target_descriptions: *const SDL_GPUColorTargetDescription,
    /// The number of color target descriptions in the above array.
    pub num_color_targets: Uint32,
    /// The pixel format of the depth-stencil target. Ignored if has_depth_stencil_target is false.
    pub depth_stencil_format: SDL_GPUTextureFormat,
    /// true specifies that the pipeline uses a depth-stencil target.
    pub has_depth_stencil_target: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

impl ::core::default::Default for SDL_GPUGraphicsPipelineTargetInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of a graphics pipeline state.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
/// - [`SDL_GPUShader`]
/// - [`SDL_GPUVertexInputState`]
/// - [`SDL_GPUPrimitiveType`]
/// - [`SDL_GPURasterizerState`]
/// - [`SDL_GPUMultisampleState`]
/// - [`SDL_GPUDepthStencilState`]
/// - [`SDL_GPUGraphicsPipelineTargetInfo`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUGraphicsPipelineCreateInfo {
    /// The vertex shader used by the graphics pipeline.
    pub vertex_shader: *mut SDL_GPUShader,
    /// The fragment shader used by the graphics pipeline.
    pub fragment_shader: *mut SDL_GPUShader,
    /// The vertex layout of the graphics pipeline.
    pub vertex_input_state: SDL_GPUVertexInputState,
    /// The primitive topology of the graphics pipeline.
    pub primitive_type: SDL_GPUPrimitiveType,
    /// The rasterizer state of the graphics pipeline.
    pub rasterizer_state: SDL_GPURasterizerState,
    /// The multisample state of the graphics pipeline.
    pub multisample_state: SDL_GPUMultisampleState,
    /// The depth-stencil state of the graphics pipeline.
    pub depth_stencil_state: SDL_GPUDepthStencilState,
    /// Formats and blend modes for the render targets of the graphics pipeline.
    pub target_info: SDL_GPUGraphicsPipelineTargetInfo,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

impl ::core::default::Default for SDL_GPUGraphicsPipelineCreateInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of a compute pipeline state.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUComputePipeline`]
/// - [`SDL_GPUShaderFormat`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUComputePipelineCreateInfo {
    /// The size in bytes of the compute shader code pointed to.
    pub code_size: ::core::primitive::usize,
    /// A pointer to compute shader code.
    pub code: *const Uint8,
    /// A pointer to a null-terminated UTF-8 string specifying the entry point function name for the shader.
    pub entrypoint: *const ::core::ffi::c_char,
    /// The format of the compute shader code.
    pub format: SDL_GPUShaderFormat,
    /// The number of samplers defined in the shader.
    pub num_samplers: Uint32,
    /// The number of readonly storage textures defined in the shader.
    pub num_readonly_storage_textures: Uint32,
    /// The number of readonly storage buffers defined in the shader.
    pub num_readonly_storage_buffers: Uint32,
    /// The number of read-write storage textures defined in the shader.
    pub num_readwrite_storage_textures: Uint32,
    /// The number of read-write storage buffers defined in the shader.
    pub num_readwrite_storage_buffers: Uint32,
    /// The number of uniform buffers defined in the shader.
    pub num_uniform_buffers: Uint32,
    /// The number of threads in the X dimension. This should match the value in the shader.
    pub threadcount_x: Uint32,
    /// The number of threads in the Y dimension. This should match the value in the shader.
    pub threadcount_y: Uint32,
    /// The number of threads in the Z dimension. This should match the value in the shader.
    pub threadcount_z: Uint32,
    /// A properties ID for extensions. Should be 0 if no extensions are needed.
    pub props: SDL_PropertiesID,
}

impl ::core::default::Default for SDL_GPUComputePipelineCreateInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of a color target used by a render
/// pass.
///
/// The load_op field determines what is done with the texture at the beginning
/// of the render pass.
///
/// - LOAD: Loads the data currently in the texture. Not recommended for
///   multisample textures as it requires significant memory bandwidth.
/// - CLEAR: Clears the texture to a single color.
/// - DONT_CARE: The driver will do whatever it wants with the texture memory.
///   This is a good option if you know that every single pixel will be touched
///   in the render pass.
///
/// The store_op field determines what is done with the color results of the
/// render pass.
///
/// - STORE: Stores the results of the render pass in the texture. Not
///   recommended for multisample textures as it requires significant memory
///   bandwidth.
/// - DONT_CARE: The driver will do whatever it wants with the texture memory.
///   This is often a good option for depth/stencil textures.
/// - RESOLVE: Resolves a multisample texture into resolve_texture, which must
///   have a sample count of 1. Then the driver may discard the multisample
///   texture memory. This is the most performant method of resolving a
///   multisample target.
/// - RESOLVE_AND_STORE: Resolves a multisample texture into the
///   resolve_texture, which must have a sample count of 1. Then the driver
///   stores the multisample texture's contents. Not recommended as it requires
///   significant memory bandwidth.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPURenderPass`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUColorTargetInfo {
    /// The texture that will be used as a color target by a render pass.
    pub texture: *mut SDL_GPUTexture,
    /// The mip level to use as a color target.
    pub mip_level: Uint32,
    /// The layer index or depth plane to use as a color target. This value is treated as a layer index on 2D array and cube textures, and as a depth plane on 3D textures.
    pub layer_or_depth_plane: Uint32,
    /// The color to clear the color target to at the start of the render pass. Ignored if [`SDL_GPU_LOADOP_CLEAR`] is not used.
    pub clear_color: SDL_FColor,
    /// What is done with the contents of the color target at the beginning of the render pass.
    pub load_op: SDL_GPULoadOp,
    /// What is done with the results of the render pass.
    pub store_op: SDL_GPUStoreOp,
    /// The texture that will receive the results of a multisample resolve operation. Ignored if a RESOLVE* store_op is not used.
    pub resolve_texture: *mut SDL_GPUTexture,
    /// The mip level of the resolve texture to use for the resolve operation. Ignored if a RESOLVE* store_op is not used.
    pub resolve_mip_level: Uint32,
    /// The layer index of the resolve texture to use for the resolve operation. Ignored if a RESOLVE* store_op is not used.
    pub resolve_layer: Uint32,
    /// true cycles the texture if the texture is bound and load_op is not LOAD
    pub cycle: ::core::primitive::bool,
    /// true cycles the resolve texture if the resolve texture is bound. Ignored if a RESOLVE* store_op is not used.
    pub cycle_resolve_texture: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
}

impl ::core::default::Default for SDL_GPUColorTargetInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying the parameters of a depth-stencil target used by a
/// render pass.
///
/// The load_op field determines what is done with the depth contents of the
/// texture at the beginning of the render pass.
///
/// - LOAD: Loads the depth values currently in the texture.
/// - CLEAR: Clears the texture to a single depth.
/// - DONT_CARE: The driver will do whatever it wants with the memory. This is
///   a good option if you know that every single pixel will be touched in the
///   render pass.
///
/// The store_op field determines what is done with the depth results of the
/// render pass.
///
/// - STORE: Stores the depth results in the texture.
/// - DONT_CARE: The driver will do whatever it wants with the depth results.
///   This is often a good option for depth/stencil textures that don't need to
///   be reused again.
///
/// The stencil_load_op field determines what is done with the stencil contents
/// of the texture at the beginning of the render pass.
///
/// - LOAD: Loads the stencil values currently in the texture.
/// - CLEAR: Clears the stencil values to a single value.
/// - DONT_CARE: The driver will do whatever it wants with the memory. This is
///   a good option if you know that every single pixel will be touched in the
///   render pass.
///
/// The stencil_store_op field determines what is done with the stencil results
/// of the render pass.
///
/// - STORE: Stores the stencil results in the texture.
/// - DONT_CARE: The driver will do whatever it wants with the stencil results.
///   This is often a good option for depth/stencil textures that don't need to
///   be reused again.
///
/// Note that depth/stencil targets do not support multisample resolves.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPURenderPass`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUDepthStencilTargetInfo {
    /// The texture that will be used as the depth stencil target by the render pass.
    pub texture: *mut SDL_GPUTexture,
    /// The value to clear the depth component to at the beginning of the render pass. Ignored if [`SDL_GPU_LOADOP_CLEAR`] is not used.
    pub clear_depth: ::core::ffi::c_float,
    /// What is done with the depth contents at the beginning of the render pass.
    pub load_op: SDL_GPULoadOp,
    /// What is done with the depth results of the render pass.
    pub store_op: SDL_GPUStoreOp,
    /// What is done with the stencil contents at the beginning of the render pass.
    pub stencil_load_op: SDL_GPULoadOp,
    /// What is done with the stencil results of the render pass.
    pub stencil_store_op: SDL_GPUStoreOp,
    /// true cycles the texture if the texture is bound and any load ops are not LOAD
    pub cycle: ::core::primitive::bool,
    /// The value to clear the stencil component to at the beginning of the render pass. Ignored if [`SDL_GPU_LOADOP_CLEAR`] is not used.
    pub clear_stencil: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
}

impl ::core::default::Default for SDL_GPUDepthStencilTargetInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure containing parameters for a blit command.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BlitGPUTexture`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBlitInfo {
    /// The source region for the blit.
    pub source: SDL_GPUBlitRegion,
    /// The destination region for the blit.
    pub destination: SDL_GPUBlitRegion,
    /// What is done with the contents of the destination before the blit.
    pub load_op: SDL_GPULoadOp,
    /// The color to clear the destination region to before the blit. Ignored if load_op is not [`SDL_GPU_LOADOP_CLEAR`].
    pub clear_color: SDL_FColor,
    /// The flip mode for the source region.
    pub flip_mode: SDL_FlipMode,
    /// The filter mode used when blitting.
    pub filter: SDL_GPUFilter,
    /// true cycles the destination texture if it is already bound.
    pub cycle: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

impl ::core::default::Default for SDL_GPUBlitInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying parameters in a buffer binding call.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BindGPUVertexBuffers`]
/// - [`SDL_BindGPUIndexBuffer`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUBufferBinding {
    /// The buffer to bind. Must have been created with [`SDL_GPU_BUFFERUSAGE_VERTEX`] for [`SDL_BindGPUVertexBuffers`], or [`SDL_GPU_BUFFERUSAGE_INDEX`] for [`SDL_BindGPUIndexBuffer`].
    pub buffer: *mut SDL_GPUBuffer,
    /// The starting byte of the data to bind in the buffer.
    pub offset: Uint32,
}

impl ::core::default::Default for SDL_GPUBufferBinding {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying parameters in a sampler binding call.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BindGPUVertexSamplers`]
/// - [`SDL_BindGPUFragmentSamplers`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUTextureSamplerBinding {
    /// The texture to bind. Must have been created with [`SDL_GPU_TEXTUREUSAGE_SAMPLER`].
    pub texture: *mut SDL_GPUTexture,
    /// The sampler to bind.
    pub sampler: *mut SDL_GPUSampler,
}

impl ::core::default::Default for SDL_GPUTextureSamplerBinding {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying parameters related to binding buffers in a compute
/// pass.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPUComputePass`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStorageBufferReadWriteBinding {
    /// The buffer to bind. Must have been created with [`SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_WRITE`].
    pub buffer: *mut SDL_GPUBuffer,
    /// true cycles the buffer if it is already bound.
    pub cycle: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

impl ::core::default::Default for SDL_GPUStorageBufferReadWriteBinding {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// A structure specifying parameters related to binding textures in a compute
/// pass.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPUComputePass`]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_GPUStorageTextureReadWriteBinding {
    /// The texture to bind. Must have been created with [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_WRITE`] or [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE`].
    pub texture: *mut SDL_GPUTexture,
    /// The mip level index to bind.
    pub mip_level: Uint32,
    /// The layer index to bind.
    pub layer: Uint32,
    /// true cycles the texture if it is already bound.
    pub cycle: ::core::primitive::bool,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding1: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding2: Uint8,
    #[deprecated(note = "padding fields are exempt from semver; init with `..Default::default()`")]
    pub padding3: Uint8,
}

impl ::core::default::Default for SDL_GPUStorageTextureReadWriteBinding {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

extern "C" {
    /// Checks for GPU runtime support.
    ///
    /// ## Parameters
    /// - `format_flags`: a bitflag indicating which shader formats the app is
    ///   able to provide.
    /// - `name`: the preferred GPU driver, or NULL to let SDL pick the optimal
    ///   driver.
    ///
    /// ## Return value
    /// Returns true if supported, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUDevice`]
    pub fn SDL_GPUSupportsShaderFormats(
        format_flags: SDL_GPUShaderFormat,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Checks for GPU runtime support.
    ///
    /// ## Parameters
    /// - `props`: the properties to use.
    ///
    /// ## Return value
    /// Returns true if supported, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUDeviceWithProperties`]
    pub fn SDL_GPUSupportsProperties(props: SDL_PropertiesID) -> ::core::primitive::bool;
}

extern "C" {
    /// Creates a GPU context.
    ///
    /// ## Parameters
    /// - `format_flags`: a bitflag indicating which shader formats the app is
    ///   able to provide.
    /// - `debug_mode`: enable debug mode properties and validations.
    /// - `name`: the preferred GPU driver, or NULL to let SDL pick the optimal
    ///   driver.
    ///
    /// ## Return value
    /// Returns a GPU context on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetGPUShaderFormats`]
    /// - [`SDL_GetGPUDeviceDriver`]
    /// - [`SDL_DestroyGPUDevice`]
    /// - [`SDL_GPUSupportsShaderFormats`]
    pub fn SDL_CreateGPUDevice(
        format_flags: SDL_GPUShaderFormat,
        debug_mode: ::core::primitive::bool,
        name: *const ::core::ffi::c_char,
    ) -> *mut SDL_GPUDevice;
}

extern "C" {
    /// Creates a GPU context.
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOLEAN`]\: enable debug mode
    ///   properties and validations, defaults to true.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOLEAN`]\: enable to prefer
    ///   energy efficiency over maximum GPU performance, defaults to false.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING`]\: the name of the GPU driver to
    ///   use, if a specific one is desired.
    ///
    /// These are the current shader format properties:
    ///
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOLEAN`]\: The app is able to
    ///   provide shaders for an NDA platform.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOLEAN`]\: The app is able to
    ///   provide SPIR-V shaders if applicable.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOLEAN`]\: The app is able to
    ///   provide DXBC shaders if applicable
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOLEAN`]\: The app is able to
    ///   provide DXIL shaders if applicable.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOLEAN`]\: The app is able to
    ///   provide MSL shaders if applicable.
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOLEAN`]\: The app is able to
    ///   provide Metal shader libraries if applicable.
    ///
    /// With the D3D12 renderer:
    ///
    /// - [`SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING`]\: the prefix to
    ///   use for all vertex semantics, default is "TEXCOORD".
    ///
    /// ## Parameters
    /// - `props`: the properties to use.
    ///
    /// ## Return value
    /// Returns a GPU context on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetGPUShaderFormats`]
    /// - [`SDL_GetGPUDeviceDriver`]
    /// - [`SDL_DestroyGPUDevice`]
    /// - [`SDL_GPUSupportsProperties`]
    pub fn SDL_CreateGPUDeviceWithProperties(props: SDL_PropertiesID) -> *mut SDL_GPUDevice;
}

pub const SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.debugmode".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.preferlowpower".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.name".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.private".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.spirv".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.dxbc".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.dxil".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.msl".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.shaders.metallib".as_ptr();

pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.device.create.d3d12.semantic".as_ptr();

extern "C" {
    /// Destroys a GPU context previously returned by [`SDL_CreateGPUDevice`].
    ///
    /// ## Parameters
    /// - `device`: a GPU Context to destroy.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUDevice`]
    pub fn SDL_DestroyGPUDevice(device: *mut SDL_GPUDevice);
}

extern "C" {
    /// Get the number of GPU drivers compiled into SDL.
    ///
    /// ## Return value
    /// Returns the number of built in GPU drivers.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetGPUDriver`]
    pub fn SDL_GetNumGPUDrivers() -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the name of a built in GPU driver.
    ///
    /// The GPU drivers are presented in the order in which they are normally
    /// checked during initialization.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "vulkan",
    /// "metal" or "direct3d12". These never have Unicode characters, and are not
    /// meant to be proper names.
    ///
    /// ## Parameters
    /// - `index`: the index of a GPU driver.
    ///
    /// ## Return value
    /// Returns the name of the GPU driver with the given **index**.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetNumGPUDrivers`]
    pub fn SDL_GetGPUDriver(index: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Returns the name of the backend used to create this GPU context.
    ///
    /// ## Parameters
    /// - `device`: a GPU context to query.
    ///
    /// ## Return value
    /// Returns the name of the device's driver, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetGPUDeviceDriver(device: *mut SDL_GPUDevice) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Returns the supported shader formats for this GPU context.
    ///
    /// ## Parameters
    /// - `device`: a GPU context to query.
    ///
    /// ## Return value
    /// Returns a bitflag indicating which shader formats the driver is able to
    ///   consume.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetGPUShaderFormats(device: *mut SDL_GPUDevice) -> SDL_GPUShaderFormat;
}

extern "C" {
    /// Creates a pipeline object to be used in a compute workflow.
    ///
    /// Shader resource bindings must be authored to follow a particular order
    /// depending on the shader format.
    ///
    /// For SPIR-V shaders, use the following resource sets:
    ///
    /// - 0: Sampled textures, followed by read-only storage textures, followed by
    ///   read-only storage buffers
    /// - 1: Read-write storage textures, followed by read-write storage buffers
    /// - 2: Uniform buffers
    ///
    /// For DXBC and DXIL shaders, use the following register order:
    ///
    /// - (t\[n\], space0): Sampled textures, followed by read-only storage textures,
    ///   followed by read-only storage buffers
    /// - (u\[n\], space1): Read-write storage textures, followed by read-write
    ///   storage buffers
    /// - (b\[n\], space2): Uniform buffers
    ///
    /// For MSL/metallib, use the following order:
    ///
    /// - \[\[buffer\]\]: Uniform buffers, followed by read-only storage buffers,
    ///   followed by read-write storage buffers
    /// - \[\[texture\]\]: Sampled textures, followed by read-only storage textures,
    ///   followed by read-write storage textures
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING`]\: a name that can be
    ///   displayed in debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the compute pipeline to
    ///   create.
    ///
    /// ## Return value
    /// Returns a compute pipeline object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_BindGPUComputePipeline`]
    /// - [`SDL_ReleaseGPUComputePipeline`]
    pub fn SDL_CreateGPUComputePipeline(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUComputePipelineCreateInfo,
    ) -> *mut SDL_GPUComputePipeline;
}

pub const SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.computepipeline.create.name".as_ptr();

extern "C" {
    /// Creates a pipeline object to be used in a graphics workflow.
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING`]\: a name that can be
    ///   displayed in debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the graphics pipeline to
    ///   create.
    ///
    /// ## Return value
    /// Returns a graphics pipeline object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    /// - [`SDL_BindGPUGraphicsPipeline`]
    /// - [`SDL_ReleaseGPUGraphicsPipeline`]
    pub fn SDL_CreateGPUGraphicsPipeline(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUGraphicsPipelineCreateInfo,
    ) -> *mut SDL_GPUGraphicsPipeline;
}

pub const SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.graphicspipeline.create.name".as_ptr();

extern "C" {
    /// Creates a sampler object to be used when binding textures in a graphics
    /// workflow.
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING`]\: a name that can be displayed
    ///   in debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the sampler to create.
    ///
    /// ## Return value
    /// Returns a sampler object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_BindGPUVertexSamplers`]
    /// - [`SDL_BindGPUFragmentSamplers`]
    /// - [`SDL_ReleaseGPUSampler`]
    pub fn SDL_CreateGPUSampler(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUSamplerCreateInfo,
    ) -> *mut SDL_GPUSampler;
}

pub const SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.sampler.create.name".as_ptr();

extern "C" {
    /// Creates a shader to be used when creating a graphics pipeline.
    ///
    /// Shader resource bindings must be authored to follow a particular order
    /// depending on the shader format.
    ///
    /// For SPIR-V shaders, use the following resource sets:
    ///
    /// For vertex shaders:
    ///
    /// - 0: Sampled textures, followed by storage textures, followed by storage
    ///   buffers
    /// - 1: Uniform buffers
    ///
    /// For fragment shaders:
    ///
    /// - 2: Sampled textures, followed by storage textures, followed by storage
    ///   buffers
    /// - 3: Uniform buffers
    ///
    /// For DXBC and DXIL shaders, use the following register order:
    ///
    /// For vertex shaders:
    ///
    /// - (t\[n\], space0): Sampled textures, followed by storage textures, followed
    ///   by storage buffers
    /// - (s\[n\], space0): Samplers with indices corresponding to the sampled
    ///   textures
    /// - (b\[n\], space1): Uniform buffers
    ///
    /// For pixel shaders:
    ///
    /// - (t\[n\], space2): Sampled textures, followed by storage textures, followed
    ///   by storage buffers
    /// - (s\[n\], space2): Samplers with indices corresponding to the sampled
    ///   textures
    /// - (b\[n\], space3): Uniform buffers
    ///
    /// For MSL/metallib, use the following order:
    ///
    /// - \[\[texture\]\]: Sampled textures, followed by storage textures
    /// - \[\[sampler\]\]: Samplers with indices corresponding to the sampled textures
    /// - \[\[buffer\]\]: Uniform buffers, followed by storage buffers. Vertex buffer 0
    ///   is bound at \[\[buffer(14)\]\], vertex buffer 1 at \[\[buffer(15)\]\], and so on.
    ///   Rather than manually authoring vertex buffer indices, use the
    ///   \[\[stage_in\]\] attribute which will automatically use the vertex input
    ///   information from the [`SDL_GPUGraphicsPipeline`].
    ///
    /// Shader semantics other than system-value semantics do not matter in D3D12
    /// and for ease of use the SDL implementation assumes that non system-value
    /// semantics will all be TEXCOORD. If you are using HLSL as the shader source
    /// language, your vertex semantics should start at TEXCOORD0 and increment
    /// like so: TEXCOORD1, TEXCOORD2, etc. If you wish to change the semantic
    /// prefix to something other than TEXCOORD you can use
    /// [`SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING`] with
    /// [`SDL_CreateGPUDeviceWithProperties()`].
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_SHADER_CREATE_NAME_STRING`]\: a name that can be displayed in
    ///   debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the shader to create.
    ///
    /// ## Return value
    /// Returns a shader object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUGraphicsPipeline`]
    /// - [`SDL_ReleaseGPUShader`]
    pub fn SDL_CreateGPUShader(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUShaderCreateInfo,
    ) -> *mut SDL_GPUShader;
}

pub const SDL_PROP_GPU_SHADER_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.shader.create.name".as_ptr();

extern "C" {
    /// Creates a texture object to be used in graphics or compute workflows.
    ///
    /// The contents of this texture are undefined until data is written to the
    /// texture.
    ///
    /// Note that certain combinations of usage flags are invalid. For example, a
    /// texture cannot have both the SAMPLER and GRAPHICS_STORAGE_READ flags.
    ///
    /// If you request a sample count higher than the hardware supports, the
    /// implementation will automatically fall back to the highest available sample
    /// count.
    ///
    /// There are optional properties that can be provided through
    /// SDL_GPUTextureCreateInfo's `props`. These are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT`]\: (Direct3D 12 only) if
    ///   the texture usage is [`SDL_GPU_TEXTUREUSAGE_COLOR_TARGET`], clear the texture
    ///   to a color with this red intensity. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT`]\: (Direct3D 12 only) if
    ///   the texture usage is [`SDL_GPU_TEXTUREUSAGE_COLOR_TARGET`], clear the texture
    ///   to a color with this green intensity. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT`]\: (Direct3D 12 only) if
    ///   the texture usage is [`SDL_GPU_TEXTUREUSAGE_COLOR_TARGET`], clear the texture
    ///   to a color with this blue intensity. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT`]\: (Direct3D 12 only) if
    ///   the texture usage is [`SDL_GPU_TEXTUREUSAGE_COLOR_TARGET`], clear the texture
    ///   to a color with this alpha intensity. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT`]\: (Direct3D 12 only)
    ///   if the texture usage is [`SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET`], clear
    ///   the texture to a depth of this value. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER`]\: (Direct3D 12
    ///   only) if the texture usage is [`SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET`],
    ///   clear the texture to a stencil of this Uint8 value. Defaults to zero.
    /// - [`SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING`]\: a name that can be displayed
    ///   in debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the texture to create.
    ///
    /// ## Return value
    /// Returns a texture object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_UploadToGPUTexture`]
    /// - [`SDL_DownloadFromGPUTexture`]
    /// - [`SDL_BindGPUVertexSamplers`]
    /// - [`SDL_BindGPUVertexStorageTextures`]
    /// - [`SDL_BindGPUFragmentSamplers`]
    /// - [`SDL_BindGPUFragmentStorageTextures`]
    /// - [`SDL_BindGPUComputeStorageTextures`]
    /// - [`SDL_BlitGPUTexture`]
    /// - [`SDL_ReleaseGPUTexture`]
    /// - [`SDL_GPUTextureSupportsFormat`]
    pub fn SDL_CreateGPUTexture(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUTextureCreateInfo,
    ) -> *mut SDL_GPUTexture;
}

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.r".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.g".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.b".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.a".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.depth".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.d3d12.clear.stencil".as_ptr();

pub const SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.texture.create.name".as_ptr();

extern "C" {
    /// Creates a buffer object to be used in graphics or compute workflows.
    ///
    /// The contents of this buffer are undefined until data is written to the
    /// buffer.
    ///
    /// Note that certain combinations of usage flags are invalid. For example, a
    /// buffer cannot have both the VERTEX and INDEX flags.
    ///
    /// If you use a STORAGE flag, the data in the buffer must respect std140
    /// layout conventions. In practical terms this means you must ensure that vec3
    /// and vec4 fields are 16-byte aligned.
    ///
    /// For better understanding of underlying concepts and memory management with
    /// SDL GPU API, you may refer
    /// [this blog post](https://moonside.games/posts/sdl-gpu-concepts-cycling/)
    /// .
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING`]\: a name that can be displayed in
    ///   debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the buffer to create.
    ///
    /// ## Return value
    /// Returns a buffer object on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_UploadToGPUBuffer`]
    /// - [`SDL_DownloadFromGPUBuffer`]
    /// - [`SDL_CopyGPUBufferToBuffer`]
    /// - [`SDL_BindGPUVertexBuffers`]
    /// - [`SDL_BindGPUIndexBuffer`]
    /// - [`SDL_BindGPUVertexStorageBuffers`]
    /// - [`SDL_BindGPUFragmentStorageBuffers`]
    /// - [`SDL_DrawGPUPrimitivesIndirect`]
    /// - [`SDL_DrawGPUIndexedPrimitivesIndirect`]
    /// - [`SDL_BindGPUComputeStorageBuffers`]
    /// - [`SDL_DispatchGPUComputeIndirect`]
    /// - [`SDL_ReleaseGPUBuffer`]
    pub fn SDL_CreateGPUBuffer(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUBufferCreateInfo,
    ) -> *mut SDL_GPUBuffer;
}

pub const SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.buffer.create.name".as_ptr();

extern "C" {
    /// Creates a transfer buffer to be used when uploading to or downloading from
    /// graphics resources.
    ///
    /// Download buffers can be particularly expensive to create, so it is good
    /// practice to reuse them if data will be downloaded regularly.
    ///
    /// There are optional properties that can be provided through `props`. These
    /// are the supported properties:
    ///
    /// - [`SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING`]\: a name that can be
    ///   displayed in debugging tools.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `createinfo`: a struct describing the state of the transfer buffer to
    ///   create.
    ///
    /// ## Return value
    /// Returns a transfer buffer on success, or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_UploadToGPUBuffer`]
    /// - [`SDL_DownloadFromGPUBuffer`]
    /// - [`SDL_UploadToGPUTexture`]
    /// - [`SDL_DownloadFromGPUTexture`]
    /// - [`SDL_ReleaseGPUTransferBuffer`]
    pub fn SDL_CreateGPUTransferBuffer(
        device: *mut SDL_GPUDevice,
        createinfo: *const SDL_GPUTransferBufferCreateInfo,
    ) -> *mut SDL_GPUTransferBuffer;
}

pub const SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING: *const ::core::ffi::c_char =
    c"SDL.gpu.transferbuffer.create.name".as_ptr();

extern "C" {
    /// Sets an arbitrary string constant to label a buffer.
    ///
    /// You should use [`SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING`] with
    /// [`SDL_CreateGPUBuffer`] instead of this function to avoid thread safety issues.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `buffer`: a buffer to attach the name to.
    /// - `text`: a UTF-8 string constant to mark as the name of the buffer.
    ///
    /// ## Thread safety
    /// This function is not thread safe, you must make sure the
    ///   buffer is not simultaneously used by any other thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUBuffer`]
    pub fn SDL_SetGPUBufferName(
        device: *mut SDL_GPUDevice,
        buffer: *mut SDL_GPUBuffer,
        text: *const ::core::ffi::c_char,
    );
}

extern "C" {
    /// Sets an arbitrary string constant to label a texture.
    ///
    /// You should use [`SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING`] with
    /// [`SDL_CreateGPUTexture`] instead of this function to avoid thread safety
    /// issues.
    ///
    /// ## Parameters
    /// - `device`: a GPU Context.
    /// - `texture`: a texture to attach the name to.
    /// - `text`: a UTF-8 string constant to mark as the name of the texture.
    ///
    /// ## Thread safety
    /// This function is not thread safe, you must make sure the
    ///   texture is not simultaneously used by any other thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUTexture`]
    pub fn SDL_SetGPUTextureName(
        device: *mut SDL_GPUDevice,
        texture: *mut SDL_GPUTexture,
        text: *const ::core::ffi::c_char,
    );
}

extern "C" {
    /// Inserts an arbitrary string label into the command buffer callstream.
    ///
    /// Useful for debugging.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `text`: a UTF-8 string constant to insert as the label.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_InsertGPUDebugLabel(
        command_buffer: *mut SDL_GPUCommandBuffer,
        text: *const ::core::ffi::c_char,
    );
}

extern "C" {
    /// Begins a debug group with an arbitary name.
    ///
    /// Used for denoting groups of calls when viewing the command buffer
    /// callstream in a graphics debugging tool.
    ///
    /// Each call to [`SDL_PushGPUDebugGroup`] must have a corresponding call to
    /// [`SDL_PopGPUDebugGroup`].
    ///
    /// On some backends (e.g. Metal), pushing a debug group during a
    /// render/blit/compute pass will create a group that is scoped to the native
    /// pass rather than the command buffer. For best results, if you push a debug
    /// group during a pass, always pop it in the same pass.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `name`: a UTF-8 string constant that names the group.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_PopGPUDebugGroup`]
    pub fn SDL_PushGPUDebugGroup(
        command_buffer: *mut SDL_GPUCommandBuffer,
        name: *const ::core::ffi::c_char,
    );
}

extern "C" {
    /// Ends the most-recently pushed debug group.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_PushGPUDebugGroup`]
    pub fn SDL_PopGPUDebugGroup(command_buffer: *mut SDL_GPUCommandBuffer);
}

extern "C" {
    /// Frees the given texture as soon as it is safe to do so.
    ///
    /// You must not reference the texture after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `texture`: a texture to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUTexture(device: *mut SDL_GPUDevice, texture: *mut SDL_GPUTexture);
}

extern "C" {
    /// Frees the given sampler as soon as it is safe to do so.
    ///
    /// You must not reference the sampler after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `sampler`: a sampler to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUSampler(device: *mut SDL_GPUDevice, sampler: *mut SDL_GPUSampler);
}

extern "C" {
    /// Frees the given buffer as soon as it is safe to do so.
    ///
    /// You must not reference the buffer after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `buffer`: a buffer to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUBuffer(device: *mut SDL_GPUDevice, buffer: *mut SDL_GPUBuffer);
}

extern "C" {
    /// Frees the given transfer buffer as soon as it is safe to do so.
    ///
    /// You must not reference the transfer buffer after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `transfer_buffer`: a transfer buffer to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUTransferBuffer(
        device: *mut SDL_GPUDevice,
        transfer_buffer: *mut SDL_GPUTransferBuffer,
    );
}

extern "C" {
    /// Frees the given compute pipeline as soon as it is safe to do so.
    ///
    /// You must not reference the compute pipeline after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `compute_pipeline`: a compute pipeline to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUComputePipeline(
        device: *mut SDL_GPUDevice,
        compute_pipeline: *mut SDL_GPUComputePipeline,
    );
}

extern "C" {
    /// Frees the given shader as soon as it is safe to do so.
    ///
    /// You must not reference the shader after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `shader`: a shader to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUShader(device: *mut SDL_GPUDevice, shader: *mut SDL_GPUShader);
}

extern "C" {
    /// Frees the given graphics pipeline as soon as it is safe to do so.
    ///
    /// You must not reference the graphics pipeline after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `graphics_pipeline`: a graphics pipeline to be destroyed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_ReleaseGPUGraphicsPipeline(
        device: *mut SDL_GPUDevice,
        graphics_pipeline: *mut SDL_GPUGraphicsPipeline,
    );
}

extern "C" {
    /// Acquire a command buffer.
    ///
    /// This command buffer is managed by the implementation and should not be
    /// freed by the user. The command buffer may only be used on the thread it was
    /// acquired on. The command buffer should be submitted on the thread it was
    /// acquired on.
    ///
    /// It is valid to acquire multiple command buffers on the same thread at once.
    /// In fact a common design pattern is to acquire two command buffers per frame
    /// where one is dedicated to render and compute passes and the other is
    /// dedicated to copy passes and other preparatory work such as generating
    /// mipmaps. Interleaving commands between the two command buffers reduces the
    /// total amount of passes overall which improves rendering performance.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    ///
    /// ## Return value
    /// Returns a command buffer, or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SubmitGPUCommandBuffer`]
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    pub fn SDL_AcquireGPUCommandBuffer(device: *mut SDL_GPUDevice) -> *mut SDL_GPUCommandBuffer;
}

extern "C" {
    /// Pushes data to a vertex uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// The data being pushed must respect std140 layout conventions. In practical
    /// terms this means you must ensure that vec3 and vec4 fields are 16-byte
    /// aligned.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `slot_index`: the vertex uniform slot to push data to.
    /// - `data`: client data to write.
    /// - `length`: the length of the data to write.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_PushGPUVertexUniformData(
        command_buffer: *mut SDL_GPUCommandBuffer,
        slot_index: Uint32,
        data: *const ::core::ffi::c_void,
        length: Uint32,
    );
}

extern "C" {
    /// Pushes data to a fragment uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// The data being pushed must respect std140 layout conventions. In practical
    /// terms this means you must ensure that vec3 and vec4 fields are 16-byte
    /// aligned.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `slot_index`: the fragment uniform slot to push data to.
    /// - `data`: client data to write.
    /// - `length`: the length of the data to write.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_PushGPUFragmentUniformData(
        command_buffer: *mut SDL_GPUCommandBuffer,
        slot_index: Uint32,
        data: *const ::core::ffi::c_void,
        length: Uint32,
    );
}

extern "C" {
    /// Pushes data to a uniform slot on the command buffer.
    ///
    /// Subsequent draw calls will use this uniform data.
    ///
    /// The data being pushed must respect std140 layout conventions. In practical
    /// terms this means you must ensure that vec3 and vec4 fields are 16-byte
    /// aligned.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `slot_index`: the uniform slot to push data to.
    /// - `data`: client data to write.
    /// - `length`: the length of the data to write.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_PushGPUComputeUniformData(
        command_buffer: *mut SDL_GPUCommandBuffer,
        slot_index: Uint32,
        data: *const ::core::ffi::c_void,
        length: Uint32,
    );
}

extern "C" {
    /// Begins a render pass on a command buffer.
    ///
    /// A render pass consists of a set of texture subresources (or depth slices in
    /// the 3D texture case) which will be rendered to during the render pass,
    /// along with corresponding clear values and load/store operations. All
    /// operations related to graphics pipelines must take place inside of a render
    /// pass. A default viewport and scissor state are automatically set when this
    /// is called. You cannot begin another render pass, or begin a compute pass or
    /// copy pass until you have ended the render pass.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `color_target_infos`: an array of texture subresources with
    ///   corresponding clear values and load/store ops.
    /// - `num_color_targets`: the number of color targets in the
    ///   color_target_infos array.
    /// - `depth_stencil_target_info`: a texture subresource with corresponding
    ///   clear value and load/store ops, may be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a render pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_EndGPURenderPass`]
    pub fn SDL_BeginGPURenderPass(
        command_buffer: *mut SDL_GPUCommandBuffer,
        color_target_infos: *const SDL_GPUColorTargetInfo,
        num_color_targets: Uint32,
        depth_stencil_target_info: *const SDL_GPUDepthStencilTargetInfo,
    ) -> *mut SDL_GPURenderPass;
}

extern "C" {
    /// Binds a graphics pipeline on a render pass to be used in rendering.
    ///
    /// A graphics pipeline must be bound before making any draw calls.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `graphics_pipeline`: the graphics pipeline to bind.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BindGPUGraphicsPipeline(
        render_pass: *mut SDL_GPURenderPass,
        graphics_pipeline: *mut SDL_GPUGraphicsPipeline,
    );
}

extern "C" {
    /// Sets the current viewport state on a command buffer.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `viewport`: the viewport to set.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_SetGPUViewport(
        render_pass: *mut SDL_GPURenderPass,
        viewport: *const SDL_GPUViewport,
    );
}

extern "C" {
    /// Sets the current scissor state on a command buffer.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `scissor`: the scissor area to set.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_SetGPUScissor(render_pass: *mut SDL_GPURenderPass, scissor: *const SDL_Rect);
}

extern "C" {
    /// Sets the current blend constants on a command buffer.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `blend_constants`: the blend constant color.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GPU_BLENDFACTOR_CONSTANT_COLOR`]
    /// - [`SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR`]
    pub fn SDL_SetGPUBlendConstants(
        render_pass: *mut SDL_GPURenderPass,
        blend_constants: SDL_FColor,
    );
}

extern "C" {
    /// Sets the current stencil reference value on a command buffer.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `reference`: the stencil reference value to set.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_SetGPUStencilReference(render_pass: *mut SDL_GPURenderPass, reference: Uint8);
}

extern "C" {
    /// Binds vertex buffers on a command buffer for use with subsequent draw
    /// calls.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the vertex buffer slot to begin binding from.
    /// - `bindings`: an array of [`SDL_GPUBufferBinding`] structs containing vertex
    ///   buffers and offset values.
    /// - `num_bindings`: the number of bindings in the bindings array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BindGPUVertexBuffers(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        bindings: *const SDL_GPUBufferBinding,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds an index buffer on a command buffer for use with subsequent draw
    /// calls.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `binding`: a pointer to a struct containing an index buffer and offset.
    /// - `index_element_size`: whether the index values in the buffer are 16- or
    ///   32-bit.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BindGPUIndexBuffer(
        render_pass: *mut SDL_GPURenderPass,
        binding: *const SDL_GPUBufferBinding,
        index_element_size: SDL_GPUIndexElementSize,
    );
}

extern "C" {
    /// Binds texture-sampler pairs for use on the vertex shader.
    ///
    /// The textures must have been created with [`SDL_GPU_TEXTUREUSAGE_SAMPLER`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the vertex sampler slot to begin binding from.
    /// - `texture_sampler_bindings`: an array of texture-sampler binding
    ///   structs.
    /// - `num_bindings`: the number of texture-sampler pairs to bind from the
    ///   array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUVertexSamplers(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        texture_sampler_bindings: *const SDL_GPUTextureSamplerBinding,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage textures for use on the vertex shader.
    ///
    /// These textures must have been created with
    /// [`SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the vertex storage texture slot to begin binding from.
    /// - `storage_textures`: an array of storage textures.
    /// - `num_bindings`: the number of storage texture to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUVertexStorageTextures(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        storage_textures: *const *mut SDL_GPUTexture,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage buffers for use on the vertex shader.
    ///
    /// These buffers must have been created with
    /// [`SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the vertex storage buffer slot to begin binding from.
    /// - `storage_buffers`: an array of buffers.
    /// - `num_bindings`: the number of buffers to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUVertexStorageBuffers(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        storage_buffers: *const *mut SDL_GPUBuffer,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds texture-sampler pairs for use on the fragment shader.
    ///
    /// The textures must have been created with [`SDL_GPU_TEXTUREUSAGE_SAMPLER`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the fragment sampler slot to begin binding from.
    /// - `texture_sampler_bindings`: an array of texture-sampler binding
    ///   structs.
    /// - `num_bindings`: the number of texture-sampler pairs to bind from the
    ///   array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUFragmentSamplers(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        texture_sampler_bindings: *const SDL_GPUTextureSamplerBinding,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage textures for use on the fragment shader.
    ///
    /// These textures must have been created with
    /// [`SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the fragment storage texture slot to begin binding from.
    /// - `storage_textures`: an array of storage textures.
    /// - `num_bindings`: the number of storage textures to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUFragmentStorageTextures(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        storage_textures: *const *mut SDL_GPUTexture,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage buffers for use on the fragment shader.
    ///
    /// These buffers must have been created with
    /// [`SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `first_slot`: the fragment storage buffer slot to begin binding from.
    /// - `storage_buffers`: an array of storage buffers.
    /// - `num_bindings`: the number of storage buffers to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUFragmentStorageBuffers(
        render_pass: *mut SDL_GPURenderPass,
        first_slot: Uint32,
        storage_buffers: *const *mut SDL_GPUBuffer,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Draws data using bound graphics state with an index buffer and instancing
    /// enabled.
    ///
    /// You must not call this function before binding a graphics pipeline.
    ///
    /// Note that the `first_vertex` and `first_instance` parameters are NOT
    /// compatible with built-in vertex/instance ID variables in shaders (for
    /// example, SV_VertexID); GPU APIs and shader languages do not define these
    /// built-in variables consistently, so if your shader depends on them, the
    /// only way to keep behavior consistent and portable is to always pass 0 for
    /// the correlating parameter in the draw calls.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `num_indices`: the number of indices to draw per instance.
    /// - `num_instances`: the number of instances to draw.
    /// - `first_index`: the starting index within the index buffer.
    /// - `vertex_offset`: value added to vertex index before indexing into the
    ///   vertex buffer.
    /// - `first_instance`: the ID of the first instance to draw.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DrawGPUIndexedPrimitives(
        render_pass: *mut SDL_GPURenderPass,
        num_indices: Uint32,
        num_instances: Uint32,
        first_index: Uint32,
        vertex_offset: Sint32,
        first_instance: Uint32,
    );
}

extern "C" {
    /// Draws data using bound graphics state.
    ///
    /// You must not call this function before binding a graphics pipeline.
    ///
    /// Note that the `first_vertex` and `first_instance` parameters are NOT
    /// compatible with built-in vertex/instance ID variables in shaders (for
    /// example, SV_VertexID); GPU APIs and shader languages do not define these
    /// built-in variables consistently, so if your shader depends on them, the
    /// only way to keep behavior consistent and portable is to always pass 0 for
    /// the correlating parameter in the draw calls.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `num_vertices`: the number of vertices to draw.
    /// - `num_instances`: the number of instances that will be drawn.
    /// - `first_vertex`: the index of the first vertex to draw.
    /// - `first_instance`: the ID of the first instance to draw.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DrawGPUPrimitives(
        render_pass: *mut SDL_GPURenderPass,
        num_vertices: Uint32,
        num_instances: Uint32,
        first_vertex: Uint32,
        first_instance: Uint32,
    );
}

extern "C" {
    /// Draws data using bound graphics state and with draw parameters set from a
    /// buffer.
    ///
    /// The buffer must consist of tightly-packed draw parameter sets that each
    /// match the layout of [`SDL_GPUIndirectDrawCommand`]. You must not call this
    /// function before binding a graphics pipeline.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `buffer`: a buffer containing draw parameters.
    /// - `offset`: the offset to start reading from the draw buffer.
    /// - `draw_count`: the number of draw parameter sets that should be read
    ///   from the draw buffer.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DrawGPUPrimitivesIndirect(
        render_pass: *mut SDL_GPURenderPass,
        buffer: *mut SDL_GPUBuffer,
        offset: Uint32,
        draw_count: Uint32,
    );
}

extern "C" {
    /// Draws data using bound graphics state with an index buffer enabled and with
    /// draw parameters set from a buffer.
    ///
    /// The buffer must consist of tightly-packed draw parameter sets that each
    /// match the layout of [`SDL_GPUIndexedIndirectDrawCommand`]. You must not call
    /// this function before binding a graphics pipeline.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    /// - `buffer`: a buffer containing draw parameters.
    /// - `offset`: the offset to start reading from the draw buffer.
    /// - `draw_count`: the number of draw parameter sets that should be read
    ///   from the draw buffer.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DrawGPUIndexedPrimitivesIndirect(
        render_pass: *mut SDL_GPURenderPass,
        buffer: *mut SDL_GPUBuffer,
        offset: Uint32,
        draw_count: Uint32,
    );
}

extern "C" {
    /// Ends the given render pass.
    ///
    /// All bound graphics state on the render pass command buffer is unset. The
    /// render pass handle is now invalid.
    ///
    /// ## Parameters
    /// - `render_pass`: a render pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_EndGPURenderPass(render_pass: *mut SDL_GPURenderPass);
}

extern "C" {
    /// Begins a compute pass on a command buffer.
    ///
    /// A compute pass is defined by a set of texture subresources and buffers that
    /// may be written to by compute pipelines. These textures and buffers must
    /// have been created with the COMPUTE_STORAGE_WRITE bit or the
    /// COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE bit. If you do not create a texture
    /// with COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE, you must not read from the
    /// texture in the compute pass. All operations related to compute pipelines
    /// must take place inside of a compute pass. You must not begin another
    /// compute pass, or a render pass or copy pass before ending the compute pass.
    ///
    /// A VERY IMPORTANT NOTE - Reads and writes in compute passes are NOT
    /// implicitly synchronized. This means you may cause data races by both
    /// reading and writing a resource region in a compute pass, or by writing
    /// multiple times to a resource region. If your compute work depends on
    /// reading the completed output from a previous dispatch, you MUST end the
    /// current compute pass and begin a new one before you can safely access the
    /// data. Otherwise you will receive unexpected results. Reading and writing a
    /// texture in the same compute pass is only supported by specific texture
    /// formats. Make sure you check the format support!
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `storage_texture_bindings`: an array of writeable storage texture
    ///   binding structs.
    /// - `num_storage_texture_bindings`: the number of storage textures to bind
    ///   from the array.
    /// - `storage_buffer_bindings`: an array of writeable storage buffer binding
    ///   structs.
    /// - `num_storage_buffer_bindings`: the number of storage buffers to bind
    ///   from the array.
    ///
    /// ## Return value
    /// Returns a compute pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_EndGPUComputePass`]
    pub fn SDL_BeginGPUComputePass(
        command_buffer: *mut SDL_GPUCommandBuffer,
        storage_texture_bindings: *const SDL_GPUStorageTextureReadWriteBinding,
        num_storage_texture_bindings: Uint32,
        storage_buffer_bindings: *const SDL_GPUStorageBufferReadWriteBinding,
        num_storage_buffer_bindings: Uint32,
    ) -> *mut SDL_GPUComputePass;
}

extern "C" {
    /// Binds a compute pipeline on a command buffer for use in compute dispatch.
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `compute_pipeline`: a compute pipeline to bind.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BindGPUComputePipeline(
        compute_pass: *mut SDL_GPUComputePass,
        compute_pipeline: *mut SDL_GPUComputePipeline,
    );
}

extern "C" {
    /// Binds texture-sampler pairs for use on the compute shader.
    ///
    /// The textures must have been created with [`SDL_GPU_TEXTUREUSAGE_SAMPLER`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `first_slot`: the compute sampler slot to begin binding from.
    /// - `texture_sampler_bindings`: an array of texture-sampler binding
    ///   structs.
    /// - `num_bindings`: the number of texture-sampler bindings to bind from the
    ///   array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUComputeSamplers(
        compute_pass: *mut SDL_GPUComputePass,
        first_slot: Uint32,
        texture_sampler_bindings: *const SDL_GPUTextureSamplerBinding,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage textures as readonly for use on the compute pipeline.
    ///
    /// These textures must have been created with
    /// [`SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `first_slot`: the compute storage texture slot to begin binding from.
    /// - `storage_textures`: an array of storage textures.
    /// - `num_bindings`: the number of storage textures to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUComputeStorageTextures(
        compute_pass: *mut SDL_GPUComputePass,
        first_slot: Uint32,
        storage_textures: *const *mut SDL_GPUTexture,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Binds storage buffers as readonly for use on the compute pipeline.
    ///
    /// These buffers must have been created with
    /// [`SDL_GPU_BUFFERUSAGE_COMPUTE_STORAGE_READ`].
    ///
    /// Be sure your shader is set up according to the requirements documented in
    /// [`SDL_CreateGPUShader()`].
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `first_slot`: the compute storage buffer slot to begin binding from.
    /// - `storage_buffers`: an array of storage buffer binding structs.
    /// - `num_bindings`: the number of storage buffers to bind from the array.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_CreateGPUShader`]
    pub fn SDL_BindGPUComputeStorageBuffers(
        compute_pass: *mut SDL_GPUComputePass,
        first_slot: Uint32,
        storage_buffers: *const *mut SDL_GPUBuffer,
        num_bindings: Uint32,
    );
}

extern "C" {
    /// Dispatches compute work.
    ///
    /// You must not call this function before binding a compute pipeline.
    ///
    /// A VERY IMPORTANT NOTE If you dispatch multiple times in a compute pass, and
    /// the dispatches write to the same resource region as each other, there is no
    /// guarantee of which order the writes will occur. If the write order matters,
    /// you MUST end the compute pass and begin another one.
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `groupcount_x`: number of local workgroups to dispatch in the X
    ///   dimension.
    /// - `groupcount_y`: number of local workgroups to dispatch in the Y
    ///   dimension.
    /// - `groupcount_z`: number of local workgroups to dispatch in the Z
    ///   dimension.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DispatchGPUCompute(
        compute_pass: *mut SDL_GPUComputePass,
        groupcount_x: Uint32,
        groupcount_y: Uint32,
        groupcount_z: Uint32,
    );
}

extern "C" {
    /// Dispatches compute work with parameters set from a buffer.
    ///
    /// The buffer layout should match the layout of
    /// [`SDL_GPUIndirectDispatchCommand`]. You must not call this function before
    /// binding a compute pipeline.
    ///
    /// A VERY IMPORTANT NOTE If you dispatch multiple times in a compute pass, and
    /// the dispatches write to the same resource region as each other, there is no
    /// guarantee of which order the writes will occur. If the write order matters,
    /// you MUST end the compute pass and begin another one.
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    /// - `buffer`: a buffer containing dispatch parameters.
    /// - `offset`: the offset to start reading from the dispatch buffer.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DispatchGPUComputeIndirect(
        compute_pass: *mut SDL_GPUComputePass,
        buffer: *mut SDL_GPUBuffer,
        offset: Uint32,
    );
}

extern "C" {
    /// Ends the current compute pass.
    ///
    /// All bound compute state on the command buffer is unset. The compute pass
    /// handle is now invalid.
    ///
    /// ## Parameters
    /// - `compute_pass`: a compute pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_EndGPUComputePass(compute_pass: *mut SDL_GPUComputePass);
}

extern "C" {
    /// Maps a transfer buffer into application address space.
    ///
    /// You must unmap the transfer buffer before encoding upload commands. The
    /// memory is owned by the graphics driver - do NOT call [`SDL_free()`] on the
    /// returned pointer.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `transfer_buffer`: a transfer buffer.
    /// - `cycle`: if true, cycles the transfer buffer if it is already bound.
    ///
    /// ## Return value
    /// Returns the address of the mapped transfer buffer memory, or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_MapGPUTransferBuffer(
        device: *mut SDL_GPUDevice,
        transfer_buffer: *mut SDL_GPUTransferBuffer,
        cycle: ::core::primitive::bool,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Unmaps a previously mapped transfer buffer.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `transfer_buffer`: a previously mapped transfer buffer.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_UnmapGPUTransferBuffer(
        device: *mut SDL_GPUDevice,
        transfer_buffer: *mut SDL_GPUTransferBuffer,
    );
}

extern "C" {
    /// Begins a copy pass on a command buffer.
    ///
    /// All operations related to copying to or from buffers or textures take place
    /// inside a copy pass. You must not begin another copy pass, or a render pass
    /// or compute pass before ending the copy pass.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    ///
    /// ## Return value
    /// Returns a copy pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BeginGPUCopyPass(command_buffer: *mut SDL_GPUCommandBuffer) -> *mut SDL_GPUCopyPass;
}

extern "C" {
    /// Uploads data from a transfer buffer to a texture.
    ///
    /// The upload occurs on the GPU timeline. You may assume that the upload has
    /// finished in subsequent commands.
    ///
    /// You must align the data in the transfer buffer to a multiple of the texel
    /// size of the texture format.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: the source transfer buffer with image layout information.
    /// - `destination`: the destination texture region.
    /// - `cycle`: if true, cycles the texture if the texture is bound, otherwise
    ///   overwrites the data.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_UploadToGPUTexture(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUTextureTransferInfo,
        destination: *const SDL_GPUTextureRegion,
        cycle: ::core::primitive::bool,
    );
}

extern "C" {
    /// Uploads data from a transfer buffer to a buffer.
    ///
    /// The upload occurs on the GPU timeline. You may assume that the upload has
    /// finished in subsequent commands.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: the source transfer buffer with offset.
    /// - `destination`: the destination buffer with offset and size.
    /// - `cycle`: if true, cycles the buffer if it is already bound, otherwise
    ///   overwrites the data.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_UploadToGPUBuffer(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUTransferBufferLocation,
        destination: *const SDL_GPUBufferRegion,
        cycle: ::core::primitive::bool,
    );
}

extern "C" {
    /// Performs a texture-to-texture copy.
    ///
    /// This copy occurs on the GPU timeline. You may assume the copy has finished
    /// in subsequent commands.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: a source texture region.
    /// - `destination`: a destination texture region.
    /// - `w`: the width of the region to copy.
    /// - `h`: the height of the region to copy.
    /// - `d`: the depth of the region to copy.
    /// - `cycle`: if true, cycles the destination texture if the destination
    ///   texture is bound, otherwise overwrites the data.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_CopyGPUTextureToTexture(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUTextureLocation,
        destination: *const SDL_GPUTextureLocation,
        w: Uint32,
        h: Uint32,
        d: Uint32,
        cycle: ::core::primitive::bool,
    );
}

extern "C" {
    /// Performs a buffer-to-buffer copy.
    ///
    /// This copy occurs on the GPU timeline. You may assume the copy has finished
    /// in subsequent commands.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: the buffer and offset to copy from.
    /// - `destination`: the buffer and offset to copy to.
    /// - `size`: the length of the buffer to copy.
    /// - `cycle`: if true, cycles the destination buffer if it is already bound,
    ///   otherwise overwrites the data.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_CopyGPUBufferToBuffer(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUBufferLocation,
        destination: *const SDL_GPUBufferLocation,
        size: Uint32,
        cycle: ::core::primitive::bool,
    );
}

extern "C" {
    /// Copies data from a texture to a transfer buffer on the GPU timeline.
    ///
    /// This data is not guaranteed to be copied until the command buffer fence is
    /// signaled.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: the source texture region.
    /// - `destination`: the destination transfer buffer with image layout
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DownloadFromGPUTexture(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUTextureRegion,
        destination: *const SDL_GPUTextureTransferInfo,
    );
}

extern "C" {
    /// Copies data from a buffer to a transfer buffer on the GPU timeline.
    ///
    /// This data is not guaranteed to be copied until the command buffer fence is
    /// signaled.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    /// - `source`: the source buffer with offset and size.
    /// - `destination`: the destination transfer buffer with offset.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_DownloadFromGPUBuffer(
        copy_pass: *mut SDL_GPUCopyPass,
        source: *const SDL_GPUBufferRegion,
        destination: *const SDL_GPUTransferBufferLocation,
    );
}

extern "C" {
    /// Ends the current copy pass.
    ///
    /// ## Parameters
    /// - `copy_pass`: a copy pass handle.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_EndGPUCopyPass(copy_pass: *mut SDL_GPUCopyPass);
}

extern "C" {
    /// Generates mipmaps for the given texture.
    ///
    /// This function must not be called inside of any pass.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command_buffer.
    /// - `texture`: a texture with more than 1 mip level.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GenerateMipmapsForGPUTexture(
        command_buffer: *mut SDL_GPUCommandBuffer,
        texture: *mut SDL_GPUTexture,
    );
}

extern "C" {
    /// Blits from a source texture region to a destination texture region.
    ///
    /// This function must not be called inside of any pass.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `info`: the blit info struct containing the blit parameters.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_BlitGPUTexture(
        command_buffer: *mut SDL_GPUCommandBuffer,
        info: *const SDL_GPUBlitInfo,
    );
}

extern "C" {
    /// Determines whether a swapchain composition is supported by the window.
    ///
    /// The window must be claimed before calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`].
    /// - `swapchain_composition`: the swapchain composition to check.
    ///
    /// ## Return value
    /// Returns true if supported, false if unsupported.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ClaimWindowForGPUDevice`]
    pub fn SDL_WindowSupportsGPUSwapchainComposition(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
        swapchain_composition: SDL_GPUSwapchainComposition,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Determines whether a presentation mode is supported by the window.
    ///
    /// The window must be claimed before calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`].
    /// - `present_mode`: the presentation mode to check.
    ///
    /// ## Return value
    /// Returns true if supported, false if unsupported.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ClaimWindowForGPUDevice`]
    pub fn SDL_WindowSupportsGPUPresentMode(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
        present_mode: SDL_GPUPresentMode,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Claims a window, creating a swapchain structure for it.
    ///
    /// This must be called before [`SDL_AcquireGPUSwapchainTexture`] is called using
    /// the window. You should only call this function from the thread that created
    /// the window.
    ///
    /// The swapchain will be created with [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR`] and
    /// [`SDL_GPU_PRESENTMODE_VSYNC`]. If you want to have different swapchain
    /// parameters, you must call [`SDL_SetGPUSwapchainParameters`] after claiming the
    /// window.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`].
    ///
    /// ## Return value
    /// Returns true on success, or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called from the thread that
    ///   created the window.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_ReleaseWindowFromGPUDevice`]
    /// - [`SDL_WindowSupportsGPUPresentMode`]
    /// - [`SDL_WindowSupportsGPUSwapchainComposition`]
    pub fn SDL_ClaimWindowForGPUDevice(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Unclaims a window, destroying its swapchain structure.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`] that has been claimed.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ClaimWindowForGPUDevice`]
    pub fn SDL_ReleaseWindowFromGPUDevice(device: *mut SDL_GPUDevice, window: *mut SDL_Window);
}

extern "C" {
    /// Changes the swapchain parameters for the given claimed window.
    ///
    /// This function will fail if the requested present mode or swapchain
    /// composition are unsupported by the device. Check if the parameters are
    /// supported via [`SDL_WindowSupportsGPUPresentMode`] /
    /// [`SDL_WindowSupportsGPUSwapchainComposition`] prior to calling this function.
    ///
    /// [`SDL_GPU_PRESENTMODE_VSYNC`] with [`SDL_GPU_SWAPCHAINCOMPOSITION_SDR`] are always
    /// supported.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`] that has been claimed.
    /// - `swapchain_composition`: the desired composition of the swapchain.
    /// - `present_mode`: the desired present mode for the swapchain.
    ///
    /// ## Return value
    /// Returns true if successful, false on error; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WindowSupportsGPUPresentMode`]
    /// - [`SDL_WindowSupportsGPUSwapchainComposition`]
    pub fn SDL_SetGPUSwapchainParameters(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
        swapchain_composition: SDL_GPUSwapchainComposition,
        present_mode: SDL_GPUPresentMode,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Configures the maximum allowed number of frames in flight.
    ///
    /// The default value when the device is created is 2. This means that after
    /// you have submitted 2 frames for presentation, if the GPU has not finished
    /// working on the first frame, [`SDL_AcquireGPUSwapchainTexture()`] will fill the
    /// swapchain texture pointer with NULL, and
    /// [`SDL_WaitAndAcquireGPUSwapchainTexture()`] will block.
    ///
    /// Higher values increase throughput at the expense of visual latency. Lower
    /// values decrease visual latency at the expense of throughput.
    ///
    /// Note that calling this function will stall and flush the command queue to
    /// prevent synchronization issues.
    ///
    /// The minimum value of allowed frames in flight is 1, and the maximum is 3.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `allowed_frames_in_flight`: the maximum number of frames that can be
    ///   pending on the GPU.
    ///
    /// ## Return value
    /// Returns true if successful, false on error; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_SetGPUAllowedFramesInFlight(
        device: *mut SDL_GPUDevice,
        allowed_frames_in_flight: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Obtains the texture format of the swapchain for the given window.
    ///
    /// Note that this format can change if the swapchain parameters change.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: an [`SDL_Window`] that has been claimed.
    ///
    /// ## Return value
    /// Returns the texture format of the swapchain.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetGPUSwapchainTextureFormat(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
    ) -> SDL_GPUTextureFormat;
}

extern "C" {
    /// Acquire a texture to use in presentation.
    ///
    /// When a swapchain texture is acquired on a command buffer, it will
    /// automatically be submitted for presentation when the command buffer is
    /// submitted. The swapchain texture should only be referenced by the command
    /// buffer used to acquire it.
    ///
    /// This function will fill the swapchain texture handle with NULL if too many
    /// frames are in flight. This is not an error.
    ///
    /// If you use this function, it is possible to create a situation where many
    /// command buffers are allocated while the rendering context waits for the GPU
    /// to catch up, which will cause memory usage to grow. You should use
    /// [`SDL_WaitAndAcquireGPUSwapchainTexture()`] unless you know what you are doing
    /// with timing.
    ///
    /// The swapchain texture is managed by the implementation and must not be
    /// freed by the user. You MUST NOT call this function from any thread other
    /// than the one that created the window.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `window`: a window that has been claimed.
    /// - `swapchain_texture`: a pointer filled in with a swapchain texture
    ///   handle.
    /// - `swapchain_texture_width`: a pointer filled in with the swapchain
    ///   texture width, may be NULL.
    /// - `swapchain_texture_height`: a pointer filled in with the swapchain
    ///   texture height, may be NULL.
    ///
    /// ## Return value
    /// Returns true on success, false on error; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called from the thread that
    ///   created the window.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ClaimWindowForGPUDevice`]
    /// - [`SDL_SubmitGPUCommandBuffer`]
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    /// - [`SDL_CancelGPUCommandBuffer`]
    /// - [`SDL_GetWindowSizeInPixels`]
    /// - [`SDL_WaitForGPUSwapchain`]
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_SetGPUAllowedFramesInFlight`]
    pub fn SDL_AcquireGPUSwapchainTexture(
        command_buffer: *mut SDL_GPUCommandBuffer,
        window: *mut SDL_Window,
        swapchain_texture: *mut *mut SDL_GPUTexture,
        swapchain_texture_width: *mut Uint32,
        swapchain_texture_height: *mut Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Blocks the thread until a swapchain texture is available to be acquired.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `window`: a window that has been claimed.
    ///
    /// ## Return value
    /// Returns true on success, false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called from the thread that
    ///   created the window.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AcquireGPUSwapchainTexture`]
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_SetGPUAllowedFramesInFlight`]
    pub fn SDL_WaitForGPUSwapchain(
        device: *mut SDL_GPUDevice,
        window: *mut SDL_Window,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Blocks the thread until a swapchain texture is available to be acquired,
    /// and then acquires it.
    ///
    /// When a swapchain texture is acquired on a command buffer, it will
    /// automatically be submitted for presentation when the command buffer is
    /// submitted. The swapchain texture should only be referenced by the command
    /// buffer used to acquire it. It is an error to call
    /// [`SDL_CancelGPUCommandBuffer()`] after a swapchain texture is acquired.
    ///
    /// This function can fill the swapchain texture handle with NULL in certain
    /// cases, for example if the window is minimized. This is not an error. You
    /// should always make sure to check whether the pointer is NULL before
    /// actually using it.
    ///
    /// The swapchain texture is managed by the implementation and must not be
    /// freed by the user. You MUST NOT call this function from any thread other
    /// than the one that created the window.
    ///
    /// The swapchain texture is write-only and cannot be used as a sampler or for
    /// another reading operation.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    /// - `window`: a window that has been claimed.
    /// - `swapchain_texture`: a pointer filled in with a swapchain texture
    ///   handle.
    /// - `swapchain_texture_width`: a pointer filled in with the swapchain
    ///   texture width, may be NULL.
    /// - `swapchain_texture_height`: a pointer filled in with the swapchain
    ///   texture height, may be NULL.
    ///
    /// ## Return value
    /// Returns true on success, false on error; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function should only be called from the thread that
    ///   created the window.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SubmitGPUCommandBuffer`]
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    /// - [`SDL_AcquireGPUSwapchainTexture`]
    pub fn SDL_WaitAndAcquireGPUSwapchainTexture(
        command_buffer: *mut SDL_GPUCommandBuffer,
        window: *mut SDL_Window,
        swapchain_texture: *mut *mut SDL_GPUTexture,
        swapchain_texture_width: *mut Uint32,
        swapchain_texture_height: *mut Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Submits a command buffer so its commands can be processed on the GPU.
    ///
    /// It is invalid to use the command buffer after this is called.
    ///
    /// This must be called from the thread the command buffer was acquired on.
    ///
    /// All commands in the submission are guaranteed to begin executing before any
    /// command in a subsequent submission begins executing.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    ///
    /// ## Return value
    /// Returns true on success, false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AcquireGPUCommandBuffer`]
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_AcquireGPUSwapchainTexture`]
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    pub fn SDL_SubmitGPUCommandBuffer(
        command_buffer: *mut SDL_GPUCommandBuffer,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Submits a command buffer so its commands can be processed on the GPU, and
    /// acquires a fence associated with the command buffer.
    ///
    /// You must release this fence when it is no longer needed or it will cause a
    /// leak. It is invalid to use the command buffer after this is called.
    ///
    /// This must be called from the thread the command buffer was acquired on.
    ///
    /// All commands in the submission are guaranteed to begin executing before any
    /// command in a subsequent submission begins executing.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    ///
    /// ## Return value
    /// Returns a fence associated with the command buffer, or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AcquireGPUCommandBuffer`]
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_AcquireGPUSwapchainTexture`]
    /// - [`SDL_SubmitGPUCommandBuffer`]
    /// - [`SDL_ReleaseGPUFence`]
    pub fn SDL_SubmitGPUCommandBufferAndAcquireFence(
        command_buffer: *mut SDL_GPUCommandBuffer,
    ) -> *mut SDL_GPUFence;
}

extern "C" {
    /// Cancels a command buffer.
    ///
    /// None of the enqueued commands are executed.
    ///
    /// It is an error to call this function after a swapchain texture has been
    /// acquired.
    ///
    /// This must be called from the thread the command buffer was acquired on.
    ///
    /// You must not reference the command buffer after calling this function.
    ///
    /// ## Parameters
    /// - `command_buffer`: a command buffer.
    ///
    /// ## Return value
    /// Returns true on success, false on error; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WaitAndAcquireGPUSwapchainTexture`]
    /// - [`SDL_AcquireGPUCommandBuffer`]
    /// - [`SDL_AcquireGPUSwapchainTexture`]
    pub fn SDL_CancelGPUCommandBuffer(
        command_buffer: *mut SDL_GPUCommandBuffer,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Blocks the thread until the GPU is completely idle.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    ///
    /// ## Return value
    /// Returns true on success, false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_WaitForGPUFences`]
    pub fn SDL_WaitForGPUIdle(device: *mut SDL_GPUDevice) -> ::core::primitive::bool;
}

extern "C" {
    /// Blocks the thread until the given fences are signaled.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `wait_all`: if 0, wait for any fence to be signaled, if 1, wait for all
    ///   fences to be signaled.
    /// - `fences`: an array of fences to wait on.
    /// - `num_fences`: the number of fences in the fences array.
    ///
    /// ## Return value
    /// Returns true on success, false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    /// - [`SDL_WaitForGPUIdle`]
    pub fn SDL_WaitForGPUFences(
        device: *mut SDL_GPUDevice,
        wait_all: ::core::primitive::bool,
        fences: *const *mut SDL_GPUFence,
        num_fences: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Checks the status of a fence.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `fence`: a fence.
    ///
    /// ## Return value
    /// Returns true if the fence is signaled, false if it is not.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    pub fn SDL_QueryGPUFence(
        device: *mut SDL_GPUDevice,
        fence: *mut SDL_GPUFence,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Releases a fence obtained from [`SDL_SubmitGPUCommandBufferAndAcquireFence`].
    ///
    /// You must not reference the fence after calling this function.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `fence`: a fence.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
    pub fn SDL_ReleaseGPUFence(device: *mut SDL_GPUDevice, fence: *mut SDL_GPUFence);
}

extern "C" {
    /// Obtains the texel block size for a texture format.
    ///
    /// ## Parameters
    /// - `format`: the texture format you want to know the texel size of.
    ///
    /// ## Return value
    /// Returns the texel block size of the texture format.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_UploadToGPUTexture`]
    pub fn SDL_GPUTextureFormatTexelBlockSize(format: SDL_GPUTextureFormat) -> Uint32;
}

extern "C" {
    /// Determines whether a texture format is supported for a given type and
    /// usage.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `format`: the texture format to check.
    /// - `type`: the type of texture (2D, 3D, Cube).
    /// - `usage`: a bitmask of all usage scenarios to check.
    ///
    /// ## Return value
    /// Returns whether the texture format is supported for this type and usage.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GPUTextureSupportsFormat(
        device: *mut SDL_GPUDevice,
        format: SDL_GPUTextureFormat,
        r#type: SDL_GPUTextureType,
        usage: SDL_GPUTextureUsageFlags,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Determines if a sample count for a texture format is supported.
    ///
    /// ## Parameters
    /// - `device`: a GPU context.
    /// - `format`: the texture format to check.
    /// - `sample_count`: the sample count to check.
    ///
    /// ## Return value
    /// Returns whether the sample count is supported for this texture format.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GPUTextureSupportsSampleCount(
        device: *mut SDL_GPUDevice,
        format: SDL_GPUTextureFormat,
        sample_count: SDL_GPUSampleCount,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Calculate the size in bytes of a texture format with dimensions.
    ///
    /// ## Parameters
    /// - `format`: a texture format.
    /// - `width`: width in pixels.
    /// - `height`: height in pixels.
    /// - `depth_or_layer_count`: depth for 3D textures or layer count otherwise.
    ///
    /// ## Return value
    /// Returns the size of a texture with this format and dimensions.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_CalculateGPUTextureFormatSize(
        format: SDL_GPUTextureFormat,
        width: Uint32,
        height: Uint32,
        depth_or_layer_count: Uint32,
    ) -> Uint32;
}

apply_cfg!(#[cfg(any(doc, all(windows, feature = "target-gdk")))] => {
    extern "C" {
        /// Call this to suspend GPU operation on Xbox when you receive the
        /// [`SDL_EVENT_DID_ENTER_BACKGROUND`] event.
        ///
        /// Do NOT call any SDL_GPU functions after calling this function! This must
        /// also be called before calling [`SDL_GDKSuspendComplete`].
        ///
        /// ## Parameters
        /// - `device`: a GPU context.
        ///
        /// ## Availability
        /// This function is available since SDL 3.2.0.
        ///
        /// ## See also
        /// - [`SDL_AddEventWatch`]
        pub fn SDL_GDKSuspendGPU(device: *mut SDL_GPUDevice);
    }

    extern "C" {
        /// Call this to resume GPU operation on Xbox when you receive the
        /// [`SDL_EVENT_WILL_ENTER_FOREGROUND`] event.
        ///
        /// When resuming, this function MUST be called before calling any other
        /// SDL_GPU functions.
        ///
        /// ## Parameters
        /// - `device`: a GPU context.
        ///
        /// ## Availability
        /// This function is available since SDL 3.2.0.
        ///
        /// ## See also
        /// - [`SDL_AddEventWatch`]
        pub fn SDL_GDKResumeGPU(device: *mut SDL_GPUDevice);
    }

});

/// An opaque handle representing a buffer.
///
/// Used for vertices, indices, indirect draw commands, and general compute
/// data.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUBuffer`]
/// - [`SDL_UploadToGPUBuffer`]
/// - [`SDL_DownloadFromGPUBuffer`]
/// - [`SDL_CopyGPUBufferToBuffer`]
/// - [`SDL_BindGPUVertexBuffers`]
/// - [`SDL_BindGPUIndexBuffer`]
/// - [`SDL_BindGPUVertexStorageBuffers`]
/// - [`SDL_BindGPUFragmentStorageBuffers`]
/// - [`SDL_DrawGPUPrimitivesIndirect`]
/// - [`SDL_DrawGPUIndexedPrimitivesIndirect`]
/// - [`SDL_BindGPUComputeStorageBuffers`]
/// - [`SDL_DispatchGPUComputeIndirect`]
/// - [`SDL_ReleaseGPUBuffer`]
#[repr(C)]
pub struct SDL_GPUBuffer {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a command buffer.
///
/// Most state is managed via command buffers. When setting state using a
/// command buffer, that state is local to the command buffer.
///
/// Commands only begin execution on the GPU once [`SDL_SubmitGPUCommandBuffer`] is
/// called. Once the command buffer is submitted, it is no longer valid to use
/// it.
///
/// Command buffers are executed in submission order. If you submit command
/// buffer A and then command buffer B all commands in A will begin executing
/// before any command in B begins executing.
///
/// In multi-threading scenarios, you should only access a command buffer on
/// the thread you acquired it from.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_AcquireGPUCommandBuffer`]
/// - [`SDL_SubmitGPUCommandBuffer`]
/// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
#[repr(C)]
pub struct SDL_GPUCommandBuffer {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a compute pass.
///
/// This handle is transient and should not be held or referenced after
/// [`SDL_EndGPUComputePass`] is called.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPUComputePass`]
/// - [`SDL_EndGPUComputePass`]
#[repr(C)]
pub struct SDL_GPUComputePass {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a compute pipeline.
///
/// Used during compute passes.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUComputePipeline`]
/// - [`SDL_BindGPUComputePipeline`]
/// - [`SDL_ReleaseGPUComputePipeline`]
#[repr(C)]
pub struct SDL_GPUComputePipeline {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a copy pass.
///
/// This handle is transient and should not be held or referenced after
/// [`SDL_EndGPUCopyPass`] is called.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPUCopyPass`]
/// - [`SDL_EndGPUCopyPass`]
#[repr(C)]
pub struct SDL_GPUCopyPass {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing the SDL_GPU context.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_GPUDevice {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a fence.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_SubmitGPUCommandBufferAndAcquireFence`]
/// - [`SDL_QueryGPUFence`]
/// - [`SDL_WaitForGPUFences`]
/// - [`SDL_ReleaseGPUFence`]
#[repr(C)]
pub struct SDL_GPUFence {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a graphics pipeline.
///
/// Used during render passes.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUGraphicsPipeline`]
/// - [`SDL_BindGPUGraphicsPipeline`]
/// - [`SDL_ReleaseGPUGraphicsPipeline`]
#[repr(C)]
pub struct SDL_GPUGraphicsPipeline {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a render pass.
///
/// This handle is transient and should not be held or referenced after
/// [`SDL_EndGPURenderPass`] is called.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_BeginGPURenderPass`]
/// - [`SDL_EndGPURenderPass`]
#[repr(C)]
pub struct SDL_GPURenderPass {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a sampler.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUSampler`]
/// - [`SDL_BindGPUVertexSamplers`]
/// - [`SDL_BindGPUFragmentSamplers`]
/// - [`SDL_ReleaseGPUSampler`]
#[repr(C)]
pub struct SDL_GPUSampler {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a compiled shader object.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUShader`]
/// - [`SDL_CreateGPUGraphicsPipeline`]
/// - [`SDL_ReleaseGPUShader`]
#[repr(C)]
pub struct SDL_GPUShader {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a texture.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTexture`]
/// - [`SDL_UploadToGPUTexture`]
/// - [`SDL_DownloadFromGPUTexture`]
/// - [`SDL_CopyGPUTextureToTexture`]
/// - [`SDL_BindGPUVertexSamplers`]
/// - [`SDL_BindGPUVertexStorageTextures`]
/// - [`SDL_BindGPUFragmentSamplers`]
/// - [`SDL_BindGPUFragmentStorageTextures`]
/// - [`SDL_BindGPUComputeStorageTextures`]
/// - [`SDL_GenerateMipmapsForGPUTexture`]
/// - [`SDL_BlitGPUTexture`]
/// - [`SDL_ReleaseGPUTexture`]
#[repr(C)]
pub struct SDL_GPUTexture {
    _opaque: [::core::primitive::u8; 0],
}

/// An opaque handle representing a transfer buffer.
///
/// Used for transferring data to and from the device.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_CreateGPUTransferBuffer`]
/// - [`SDL_MapGPUTransferBuffer`]
/// - [`SDL_UnmapGPUTransferBuffer`]
/// - [`SDL_UploadToGPUBuffer`]
/// - [`SDL_UploadToGPUTexture`]
/// - [`SDL_DownloadFromGPUBuffer`]
/// - [`SDL_DownloadFromGPUTexture`]
/// - [`SDL_ReleaseGPUTransferBuffer`]
#[repr(C)]
pub struct SDL_GPUTransferBuffer {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
