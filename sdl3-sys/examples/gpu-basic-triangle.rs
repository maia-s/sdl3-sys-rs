use std::ptr::null_mut;
use std::sync::Mutex;

use sdl3_main::{AppResult, app_event, app_init, app_iterate, app_quit};
use sdl3_sys::everything::*;

#[path = "./common.rs"]
mod common;

use common::*;

struct AppState {
    window: *mut SDL_Window,
    device: *mut SDL_GPUDevice,
    fill_pipeline: *mut SDL_GPUGraphicsPipeline,
    line_pipeline: *mut SDL_GPUGraphicsPipeline,
    game_state: GameState,
}

impl Drop for AppState {
    fn drop(&mut self) {
        unsafe {
            if !self.fill_pipeline.is_null() {
                SDL_ReleaseGPUGraphicsPipeline(self.device, self.fill_pipeline);
            }
            if !self.line_pipeline.is_null() {
                SDL_ReleaseGPUGraphicsPipeline(self.device, self.line_pipeline);
            }

            deinit_gpu_window(self.device, self.window);
        }
    }
}

unsafe impl Send for AppState {}

#[app_iterate]
fn app_iterate(app: &mut AppState) -> AppResult {
    unsafe {
        app.game_state.step();

        let command_buffer = SDL_AcquireGPUCommandBuffer(app.device);
        if command_buffer.is_null() {
            dbg_sdl_error("failed to acquire command buffer");
            return AppResult::Failure;
        }

        let mut swapchain_texture: *mut SDL_GPUTexture = null_mut();
        if !SDL_AcquireGPUSwapchainTexture(
            command_buffer,
            app.window,
            &mut swapchain_texture,
            null_mut(),
            null_mut(),
        ) {
            dbg_sdl_error("failed to acquire swapchain texture");
            return AppResult::Failure;
        }

        if !swapchain_texture.is_null() {
            let mut color_target_info: SDL_GPUColorTargetInfo = Default::default();
            color_target_info.texture = swapchain_texture;
            color_target_info.clear_color = SDL_FColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            };
            color_target_info.load_op = SDL_GPU_LOADOP_CLEAR;
            color_target_info.store_op = SDL_GPU_STOREOP_STORE;

            let num_color_targets = 1;
            let depth_stencil_target_info = null_mut();
            let render_pass = SDL_BeginGPURenderPass(
                command_buffer,
                &color_target_info,
                num_color_targets,
                depth_stencil_target_info,
            );

            let pipeline = if app.game_state.use_wire_frame_mode {
                app.line_pipeline
            } else {
                app.fill_pipeline
            };
            SDL_BindGPUGraphicsPipeline(render_pass, pipeline);

            if app.game_state.use_small_viewport {
                let small_viewport = SDL_GPUViewport {
                    x: 160.0,
                    y: 120.0,
                    w: 320.0,
                    h: 240.0,
                    min_depth: 0.1,
                    max_depth: 1.0,
                };
                SDL_SetGPUViewport(render_pass, &small_viewport);
            }

            if app.game_state.use_scissor_rect {
                let scissor = SDL_Rect {
                    x: 320,
                    y: 240,
                    w: 320,
                    h: 240,
                };
                SDL_SetGPUScissor(render_pass, &scissor);
            }

            SDL_DrawGPUPrimitives(render_pass, 3, 1, 0, 0);
            SDL_EndGPURenderPass(render_pass);
        }

        SDL_SubmitGPUCommandBuffer(command_buffer);
    }

    AppResult::Continue
}

#[app_init]
fn app_init() -> Option<Box<Mutex<AppState>>> {
    unsafe {
        let title = c"Basic Triangle".as_ptr();
        let Some((window, device)) = init_gpu_window(title, 0) else {
            return None;
        };

        let vert_shader = load_shader(device, "RawTriangle.vert");
        if vert_shader.is_null() {
            dbg_sdl_error("failed to load vert shader");
            return None;
        }

        let frag_shader = load_shader(device, "SolidColor.frag");
        if frag_shader.is_null() {
            dbg_sdl_error("failed to load frag shader");
            return None;
        }

        let mut pipeline_create_info = SDL_GPUGraphicsPipelineCreateInfo {
            vertex_shader: vert_shader,
            fragment_shader: frag_shader,
            primitive_type: SDL_GPU_PRIMITIVETYPE_TRIANGLELIST,
            target_info: SDL_GPUGraphicsPipelineTargetInfo {
                num_color_targets: 1,
                color_target_descriptions: [SDL_GPUColorTargetDescription {
                    format: SDL_GetGPUSwapchainTextureFormat(device, window),
                    ..Default::default()
                }]
                .as_ptr(),
                ..Default::default()
            },
            ..Default::default()
        };

        pipeline_create_info.rasterizer_state.fill_mode = SDL_GPU_FILLMODE_FILL;
        let fill_pipeline = SDL_CreateGPUGraphicsPipeline(device, &pipeline_create_info);
        if fill_pipeline.is_null() {
            dbg_sdl_error("failed to create fill pipeline");
            return None;
        }

        pipeline_create_info.rasterizer_state.fill_mode = SDL_GPU_FILLMODE_LINE;
        let line_pipeline = SDL_CreateGPUGraphicsPipeline(device, &pipeline_create_info);
        if line_pipeline.is_null() {
            dbg_sdl_error("failed to create line pipeline");
            return None;
        }

        SDL_ReleaseGPUShader(device, vert_shader);
        SDL_ReleaseGPUShader(device, frag_shader);

        let app = AppState {
            window,
            device,
            fill_pipeline,
            line_pipeline,
            game_state: GameState::new(),
        };

        println!("Press Left to toggle wireframe mode");
        println!("Press Down to toggle small viewport");
        println!("Press Right to toggle scissor rect");

        Some(Box::new(Mutex::new(app)))
    }
}

#[app_event]
fn app_event(app: &mut AppState, event: &SDL_Event) -> AppResult {
    unsafe {
        match SDL_EventType(event.r#type) {
            SDL_EVENT_QUIT => AppResult::Success,

            SDL_EVENT_KEY_DOWN => {
                app.game_state.key_pressed(event.key.scancode);
                AppResult::Continue
            }

            _ => AppResult::Continue,
        }
    }
}

#[app_quit]
fn app_quit() {}

pub struct GameState {
    pub use_wire_frame_mode: bool,
    pub use_small_viewport: bool,
    pub use_scissor_rect: bool,
    keys_just_pressed: Vec<SDL_Scancode>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            use_wire_frame_mode: false,
            use_small_viewport: false,
            use_scissor_rect: false,
            keys_just_pressed: Default::default(),
        }
    }

    pub fn key_pressed(&mut self, scan_code: SDL_Scancode) {
        if !self.keys_just_pressed.contains(&scan_code) {
            self.keys_just_pressed.push(scan_code);
        }
    }

    pub fn step(&mut self) {
        if self.keys_just_pressed.contains(&SDL_Scancode::LEFT) {
            self.use_wire_frame_mode = !self.use_wire_frame_mode;
        }
        if self.keys_just_pressed.contains(&SDL_Scancode::DOWN) {
            self.use_small_viewport = !self.use_small_viewport;
        }
        if self.keys_just_pressed.contains(&SDL_Scancode::RIGHT) {
            self.use_scissor_rect = !self.use_scissor_rect;
        }

        self.keys_just_pressed.clear();
    }
}
