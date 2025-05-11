//! Based on the official example in C here:
//! https://github.com/TheSpydog/SDL_gpu_examples/blob/main/Examples/PullSpriteBatch.c
//! which uses the method described in this blog post:
//! https://moonside.games/posts/sdl-gpu-sprite-batcher/

use std::sync::Mutex;
use std::{ffi::c_void, ptr::null_mut};

use sdl3_main::{AppResult, app_event, app_init, app_iterate, app_quit};
use sdl3_sys::everything::*;

#[path = "../../common.rs"]
mod common;

use common::*;

const SPRITE_COUNT: u32 = 8192;

struct AppState {
    window: *mut SDL_Window,
    device: *mut SDL_GPUDevice,

    render_pipeline: *mut SDL_GPUGraphicsPipeline,
    sampler: *mut SDL_GPUSampler,
    texture: *mut SDL_GPUTexture,
    sprite_data_transfer_buffer: *mut SDL_GPUTransferBuffer,
    sprite_data_buffer: *mut SDL_GPUBuffer,
}

impl Drop for AppState {
    fn drop(&mut self) {
        unsafe {
            if !self.render_pipeline.is_null() {
                SDL_ReleaseGPUGraphicsPipeline(self.device, self.render_pipeline);
            }
            if !self.sampler.is_null() {
                SDL_ReleaseGPUSampler(self.device, self.sampler);
            }
            if !self.texture.is_null() {
                SDL_ReleaseGPUTexture(self.device, self.texture);
            }
            if !self.sprite_data_transfer_buffer.is_null() {
                SDL_ReleaseGPUTransferBuffer(self.device, self.sprite_data_transfer_buffer);
            }
            if !self.sprite_data_buffer.is_null() {
                SDL_ReleaseGPUBuffer(self.device, self.sprite_data_buffer);
            }

            deinit_gpu_window(self.device, self.window);
        }
    }
}

unsafe impl Send for AppState {}

struct SpriteInstance {
    x: f32,
    y: f32,
    z: f32,
    rotation: f32,
    w: f32,
    h: f32,
    #[allow(dead_code)]
    padding_a: f32,
    #[allow(dead_code)]
    padding_b: f32,
    tex_u: f32,
    tex_v: f32,
    tex_w: f32,
    tex_h: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[app_init]
fn app_init() -> Option<Box<Mutex<AppState>>> {
    unsafe {
        let title = c"Pull Sprite Batch".as_ptr();
        let Some((window, device)) = init_gpu_window(title, Default::default()) else {
            return None;
        };

        let present_mode =
            if SDL_WindowSupportsGPUPresentMode(device, window, SDL_GPUPresentMode::IMMEDIATE) {
                SDL_GPUPresentMode::IMMEDIATE
            } else if SDL_WindowSupportsGPUPresentMode(device, window, SDL_GPUPresentMode::MAILBOX)
            {
                SDL_GPUPresentMode::MAILBOX
            } else {
                SDL_GPUPresentMode::VSYNC
            };
        SDL_SetGPUSwapchainParameters(
            device,
            window,
            SDL_GPUSwapchainComposition::SDR,
            present_mode,
        );

        // TODO copy content
        let vert_shader = load_shader(device, "PullSpriteBatch.vert");
        if vert_shader.is_null() {
            dbg_sdl_error("failed to load vert shader");
            return None;
        }

        // TODO copy content
        let frag_shader = load_shader(device, "TexturedQuadColor.frag");
        if frag_shader.is_null() {
            dbg_sdl_error("failed to load frag shader");
            return None;
        }

        let render_pipeline = SDL_CreateGPUGraphicsPipeline(
            device,
            &SDL_GPUGraphicsPipelineCreateInfo {
                primitive_type: SDL_GPUPrimitiveType::TRIANGLELIST,
                vertex_shader: vert_shader,
                fragment_shader: frag_shader,
                target_info: SDL_GPUGraphicsPipelineTargetInfo {
                    num_color_targets: 1,
                    color_target_descriptions: [SDL_GPUColorTargetDescription {
                        format: SDL_GetGPUSwapchainTextureFormat(device, window),
                        blend_state: SDL_GPUColorTargetBlendState {
                            enable_blend: true,
                            color_blend_op: SDL_GPUBlendOp::ADD,
                            alpha_blend_op: SDL_GPUBlendOp::ADD,
                            src_color_blendfactor: SDL_GPUBlendFactor::SRC_ALPHA,
                            dst_color_blendfactor: SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA,
                            src_alpha_blendfactor: SDL_GPUBlendFactor::SRC_ALPHA,
                            dst_alpha_blendfactor: SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA,
                            ..Default::default()
                        },
                    }]
                    .as_ptr(),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        SDL_ReleaseGPUShader(device, vert_shader);
        SDL_ReleaseGPUShader(device, frag_shader);

        let image_ptr = load_bmp("ravioli_atlas.bmp");
        if image_ptr.is_null() {
            dbg_sdl_error("failed to load image");
            return None;
        }
        let image = &mut *image_ptr;
        let image_size = (image.w * image.h * 4) as u32;

        let transfer_buffer = SDL_CreateGPUTransferBuffer(
            device,
            &SDL_GPUTransferBufferCreateInfo {
                usage: SDL_GPUTransferBufferUsage::UPLOAD,
                size: image_size,
                ..Default::default()
            },
        );
        let texture_transfer_ptr = SDL_MapGPUTransferBuffer(device, transfer_buffer, false);
        SDL_memcpy(texture_transfer_ptr, image.pixels, image_size as usize);
        SDL_UnmapGPUTransferBuffer(device, transfer_buffer);

        // create the GPU resources

        let texture = SDL_CreateGPUTexture(
            device,
            &SDL_GPUTextureCreateInfo {
                r#type: SDL_GPUTextureType::_2D,
                format: SDL_GPUTextureFormat::R8G8B8A8_UNORM,
                width: image.w as u32,
                height: image.h as u32,
                layer_count_or_depth: 1,
                num_levels: 1,
                usage: SDL_GPU_TEXTUREUSAGE_SAMPLER,
                ..Default::default()
            },
        );

        let sampler = SDL_CreateGPUSampler(
            device,
            &SDL_GPUSamplerCreateInfo {
                min_filter: SDL_GPU_FILTER_NEAREST,
                mag_filter: SDL_GPU_FILTER_NEAREST,
                mipmap_mode: SDL_GPU_SAMPLERMIPMAPMODE_NEAREST,
                address_mode_u: SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE,
                address_mode_v: SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE,
                address_mode_w: SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE,
                ..Default::default()
            },
        );

        let sprite_data_transfer_buffer = SDL_CreateGPUTransferBuffer(
            device,
            &SDL_GPUTransferBufferCreateInfo {
                usage: SDL_GPUTransferBufferUsage::UPLOAD,
                size: SPRITE_COUNT * std::mem::size_of::<SpriteInstance>() as u32,
                ..Default::default()
            },
        );

        let sprite_data_buffer = SDL_CreateGPUBuffer(
            device,
            &SDL_GPUBufferCreateInfo {
                usage: SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ,
                size: SPRITE_COUNT * std::mem::size_of::<SpriteInstance>() as u32,
                ..Default::default()
            },
        );

        // transfer the up-front data

        let upload_command_buf = SDL_AcquireGPUCommandBuffer(device);
        let copy_pass = SDL_BeginGPUCopyPass(upload_command_buf);
        SDL_UploadToGPUTexture(
            copy_pass,
            &SDL_GPUTextureTransferInfo {
                transfer_buffer,
                offset: 0,
                ..Default::default()
            },
            &SDL_GPUTextureRegion {
                texture,
                w: image.w as u32,
                h: image.h as u32,
                d: 1,
                ..Default::default()
            },
            false,
        );
        SDL_EndGPUCopyPass(copy_pass);
        SDL_SubmitGPUCommandBuffer(upload_command_buf);

        SDL_DestroySurface(image_ptr);
        SDL_ReleaseGPUTransferBuffer(device, transfer_buffer);

        let app = AppState {
            window,
            device,
            render_pipeline,
            sampler,
            texture,
            sprite_data_transfer_buffer,
            sprite_data_buffer,
        };

        Some(Box::new(Mutex::new(app)))
    }
}

#[app_iterate]
fn app_iterate(app: &mut AppState) -> AppResult {
    let camera_matrix =
        Matrix4x4::create_orthographic_off_center(0.0, 640.0, 480.0, 0.0, 0.0, -1.0);

    unsafe {
        let command_buffer = SDL_AcquireGPUCommandBuffer(app.device);
        if command_buffer.is_null() {
            dbg_sdl_error("AcquireGPUCommandBuffer failed");
            return AppResult::Failure;
        }

        let mut swapchain_texture: *mut SDL_GPUTexture = null_mut();
        if !SDL_WaitAndAcquireGPUSwapchainTexture(
            command_buffer,
            app.window,
            &mut swapchain_texture as *mut *mut SDL_GPUTexture,
            null_mut(),
            null_mut(),
        ) {
            dbg_sdl_error("WaitAndAcquireGPUSwapchainTexture failed");
            return AppResult::Failure;
        }

        if !swapchain_texture.is_null() {
            // build sprite instance transfer
            let data_ptr =
                SDL_MapGPUTransferBuffer(app.device, app.sprite_data_transfer_buffer, true)
                    as *mut SpriteInstance;

            for i in 0..SPRITE_COUNT {
                let ravioli = SDL_rand(4) as usize;
                let p = data_ptr.offset(i as isize);
                let sprite = &mut *p;

                const U_COORDS: [f32; 4] = [0.0, 0.5, 0.0, 0.5];
                const V_COORDS: [f32; 4] = [0.0, 0.0, 0.5, 0.5];

                sprite.x = SDL_rand(640) as f32;
                sprite.y = SDL_rand(480) as f32;
                sprite.z = 0.0;
                sprite.rotation = SDL_randf() * SDL_PI_F * 2.0;
                sprite.w = 32.0;
                sprite.h = 32.0;
                sprite.tex_u = U_COORDS[ravioli];
                sprite.tex_v = V_COORDS[ravioli];
                sprite.tex_w = 0.5;
                sprite.tex_h = 0.5;
                sprite.r = 1.0;
                sprite.g = 1.0;
                sprite.b = 1.0;
                sprite.a = 1.0;
            }

            SDL_UnmapGPUTransferBuffer(app.device, app.sprite_data_transfer_buffer);

            // upload instance data
            let copy_pass = SDL_BeginGPUCopyPass(command_buffer);
            SDL_UploadToGPUBuffer(
                copy_pass,
                &SDL_GPUTransferBufferLocation {
                    transfer_buffer: app.sprite_data_transfer_buffer,
                    offset: 0,
                },
                &SDL_GPUBufferRegion {
                    buffer: app.sprite_data_buffer,
                    offset: 0,
                    size: SPRITE_COUNT * std::mem::size_of::<SpriteInstance>() as u32,
                },
                true,
            );
            SDL_EndGPUCopyPass(copy_pass);

            // render sprites
            let render_pass = SDL_BeginGPURenderPass(
                command_buffer,
                &SDL_GPUColorTargetInfo {
                    texture: swapchain_texture,
                    cycle: false,
                    load_op: SDL_GPULoadOp::CLEAR,
                    store_op: SDL_GPUStoreOp::STORE,
                    clear_color: SDL_FColor {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    ..Default::default()
                },
                1,
                null_mut(),
            );

            SDL_BindGPUGraphicsPipeline(render_pass, app.render_pipeline);
            SDL_BindGPUVertexStorageBuffers(render_pass, 0, &app.sprite_data_buffer, 1);
            SDL_BindGPUFragmentSamplers(
                render_pass,
                0,
                &SDL_GPUTextureSamplerBinding {
                    texture: app.texture,
                    sampler: app.sampler,
                },
                1,
            );
            SDL_PushGPUVertexUniformData(
                command_buffer,
                0,
                &camera_matrix as *const Matrix4x4 as *const c_void,
                std::mem::size_of::<Matrix4x4>() as u32,
            );
            SDL_DrawGPUPrimitives(render_pass, SPRITE_COUNT * 6, 1, 0, 0);

            SDL_EndGPURenderPass(render_pass);
        }

        SDL_SubmitGPUCommandBuffer(command_buffer);
    }

    AppResult::Continue
}

#[app_event]
fn app_event(_app: &mut AppState, event: &SDL_Event) -> AppResult {
    unsafe {
        match SDL_EventType(event.r#type) {
            SDL_EVENT_QUIT => AppResult::Success,
            _ => AppResult::Continue,
        }
    }
}

#[app_quit]
fn app_quit() {}
