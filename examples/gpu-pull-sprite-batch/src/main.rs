//! Based on the official example in C here:
//! https://github.com/TheSpydog/SDL_gpu_examples/blob/main/Examples/PullSpriteBatch.c
//! which uses the method described in this blog post:
//! https://moonside.games/posts/sdl-gpu-sprite-batcher/

use std::ffi::{c_void, CStr};
use std::ptr::null_mut;
use std::sync::Mutex;

use sdl3_main::{app_impl, AppResult};
use sdl3_sys::everything::*;

mod common;
use common::{dbg_sdl_error, deinit_gpu_window, init_gpu_window, load_bmp, load_shader, Matrix4x4};

const SPRITE_COUNT: u32 = 8192;
const GPU_SPRITE_BUFFER_SIZE: u32 = SPRITE_COUNT * std::mem::size_of::<GPUSprite>() as u32;

struct AppState {
    window: *mut SDL_Window,
    device: *mut SDL_GPUDevice,

    render_pipeline: *mut SDL_GPUGraphicsPipeline,
    sampler: *mut SDL_GPUSampler,
    texture: *mut SDL_GPUTexture,
    sprite_data_transfer_buffer: *mut SDL_GPUTransferBuffer,
    sprite_data_buffer: *mut SDL_GPUBuffer,

    cpu_sprites: [CPUSprite; SPRITE_COUNT as usize],
    last_tick: u64,
    accumulated_ticks: u64,
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

/// see SpriteData in PullSpriteBatch.vert.hlsl
#[allow(dead_code)] // fields directly written to gpu buffer
#[derive(Clone)]
struct GPUSprite {
    x: f32,
    y: f32,
    z: f32,
    rotation: f32,
    w: f32,
    h: f32,
    padding_a: f32,
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

impl GPUSprite {
    fn init() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: 0.0,
            w: 32.0,
            h: 32.0,
            padding_a: 0.0,
            padding_b: 0.0,
            tex_u: 0.0,
            tex_v: 0.0,
            tex_w: 0.5,
            tex_h: 0.5,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

/// The 'gameplay data' of a sprite
#[derive(Default)]
struct CPUSprite {
    x: f32,
    y: f32,
    rotation: f32,
    ravioli: usize, // 0-3
}

impl CPUSprite {
    // sprite atlas offsets
    const U_COORDS: [f32; 4] = [0.0, 0.5, 0.0, 0.5];
    const V_COORDS: [f32; 4] = [0.0, 0.0, 0.5, 0.5];

    // write data for the shader into the gpu transfer buffer
    fn write_to_gpu(&self, gpu_sprite: &mut GPUSprite) {
        *gpu_sprite = GPUSprite::init();

        gpu_sprite.x = self.x;
        gpu_sprite.y = self.y;
        gpu_sprite.rotation = self.rotation;
        gpu_sprite.tex_u = Self::U_COORDS[self.ravioli];
        gpu_sprite.tex_v = Self::V_COORDS[self.ravioli];
    }

    fn randomize(&mut self) {
        unsafe {
            self.x = SDL_rand(640) as f32;
            self.y = SDL_rand(480) as f32;
            self.rotation = SDL_randf() * SDL_PI_F * 2.0;
            self.ravioli = SDL_rand(4) as usize;
        }
    }
}

#[app_impl]
impl AppState {
    fn app_init() -> Option<Box<Mutex<AppState>>> {
        unsafe {
            const TITLE: &CStr = c"Pull Sprite Batch Example";
            let Some((window, device)) =
                init_gpu_window(TITLE.as_ptr(), SDL_WindowFlags::default())
            else {
                return None;
            };

            let present_mode = if SDL_WindowSupportsGPUPresentMode(
                device,
                window,
                SDL_GPUPresentMode::IMMEDIATE,
            ) {
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

            let vert_shader = load_shader(device, "PullSpriteBatch.vert");
            if vert_shader.is_null() {
                dbg_sdl_error("failed to load vert shader");
                return None;
            }

            let frag_shader = load_shader(device, "TexturedQuadColor.frag");
            if frag_shader.is_null() {
                dbg_sdl_error("failed to load frag shader");
                return None;
            }

            let color_target_descriptions = [SDL_GPUColorTargetDescription {
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
            }];
            let render_pipeline = SDL_CreateGPUGraphicsPipeline(
                device,
                &SDL_GPUGraphicsPipelineCreateInfo {
                    primitive_type: SDL_GPUPrimitiveType::TRIANGLELIST,
                    vertex_shader: vert_shader,
                    fragment_shader: frag_shader,
                    target_info: SDL_GPUGraphicsPipelineTargetInfo {
                        num_color_targets: 1,
                        color_target_descriptions: color_target_descriptions.as_ptr(),
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
                    size: GPU_SPRITE_BUFFER_SIZE,
                    ..Default::default()
                },
            );

            let sprite_data_buffer = SDL_CreateGPUBuffer(
                device,
                &SDL_GPUBufferCreateInfo {
                    usage: SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ,
                    size: GPU_SPRITE_BUFFER_SIZE,
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

            let cpu_sprites: [CPUSprite; SPRITE_COUNT as usize] =
                std::array::from_fn(|_| CPUSprite::default());

            let app = AppState {
                window,
                device,
                render_pipeline,
                sampler,
                texture,
                sprite_data_transfer_buffer,
                sprite_data_buffer,
                cpu_sprites,
                last_tick: 0,
                accumulated_ticks: 0,
            };

            Some(Box::new(Mutex::new(app)))
        }
    }

    fn app_iterate(&mut self) -> AppResult {
        let current_tick = unsafe { SDL_GetTicks() };
        let new_ticks = current_tick - self.last_tick;
        self.accumulated_ticks += new_ticks;
        self.last_tick = current_tick;

        while self.accumulated_ticks >= MILLIS_PER_FRAME {
            self.accumulated_ticks -= MILLIS_PER_FRAME;
            let app_result = update_and_draw(self);
            if app_result != AppResult::Continue {
                return app_result;
            }
        }

        AppResult::Continue
    }

    fn app_event(&mut self, event: &SDL_Event) -> AppResult {
        let event_type = unsafe { event.r#type };

        match SDL_EventType(event_type) {
            SDL_EVENT_QUIT => AppResult::Success,
            _ => AppResult::Continue,
        }
    }
}

const MILLIS_PER_FRAME: u64 = 16;

fn update_and_draw(app: &mut AppState) -> AppResult {
    for sprite in &mut app.cpu_sprites {
        sprite.randomize();
    }

    draw_sprites(app)
}

fn draw_sprites(app: &mut AppState) -> AppResult {
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
                    as *mut GPUSprite;
            if data_ptr.is_null() {
                dbg_sdl_error("failed to map gpu transfer buffer");
                return AppResult::Failure;
            }

            for (i, cpu_sprite) in app.cpu_sprites.iter().enumerate() {
                let gpu_sprite = &mut *data_ptr.offset(i as isize);
                cpu_sprite.write_to_gpu(gpu_sprite);
            }

            SDL_UnmapGPUTransferBuffer(app.device, app.sprite_data_transfer_buffer);

            // upload gpu sprite instance data
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
                    size: GPU_SPRITE_BUFFER_SIZE,
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
            const CAMERA_MATRIX: Matrix4x4 =
                Matrix4x4::create_orthographic_off_center(0.0, 640.0, 480.0, 0.0, 0.0, -1.0);
            SDL_PushGPUVertexUniformData(
                command_buffer,
                0,
                &CAMERA_MATRIX as *const Matrix4x4 as *const c_void,
                std::mem::size_of::<Matrix4x4>() as u32,
            );
            SDL_DrawGPUPrimitives(render_pass, SPRITE_COUNT * 6, 1, 0, 0);

            SDL_EndGPURenderPass(render_pass);
        }

        SDL_SubmitGPUCommandBuffer(command_buffer);
    }

    AppResult::Continue
}
