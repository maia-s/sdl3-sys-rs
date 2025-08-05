//! Based on the official example in C here:
//! https://github.com/TheSpydog/SDL_gpu_examples/blob/main/Examples/BasicTriangle.c

use std::ffi::CStr;
use std::ptr::null_mut;
use std::sync::Mutex;

use sdl3_main::{AppResult, AppResultWithState, app_impl};
use sdl3_sys::everything::*;

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

            gpu_common::deinit_gpu_window(self.device, self.window);
        }
    }
}

unsafe impl Send for AppState {}

#[app_impl]
impl AppState {
    fn app_init() -> AppResultWithState<Box<Mutex<Self>>> {
        unsafe {
            gpu_common::set_framerate_hint(60);

            const TITLE: &CStr = c"Basic Triangle Example";
            let Some((window, device)) =
                gpu_common::init_gpu_window(TITLE, SDL_WindowFlags::default())
            else {
                return AppResultWithState::Failure(None);
            };

            let vert_shader = gpu_common::load_shader(device, "RawTriangle.vert");
            if vert_shader.is_null() {
                gpu_common::dbg_sdl_error("failed to load vert shader");
                return AppResultWithState::Failure(None);
            }

            let frag_shader = gpu_common::load_shader(device, "SolidColor.frag");
            if frag_shader.is_null() {
                gpu_common::dbg_sdl_error("failed to load frag shader");
                return AppResultWithState::Failure(None);
            }

            let texture_format = SDL_GetGPUSwapchainTextureFormat(device, window);
            let color_target_descriptions = [SDL_GPUColorTargetDescription {
                format: texture_format,
                ..Default::default()
            }];
            let mut pipeline_create_info = SDL_GPUGraphicsPipelineCreateInfo {
                target_info: SDL_GPUGraphicsPipelineTargetInfo {
                    num_color_targets: 1,
                    color_target_descriptions: color_target_descriptions.as_ptr(),
                    ..Default::default()
                },
                primitive_type: SDL_GPU_PRIMITIVETYPE_TRIANGLELIST,
                vertex_shader: vert_shader,
                fragment_shader: frag_shader,
                ..Default::default()
            };

            pipeline_create_info.rasterizer_state.fill_mode = SDL_GPU_FILLMODE_FILL;
            let fill_pipeline = SDL_CreateGPUGraphicsPipeline(device, &pipeline_create_info);
            if fill_pipeline.is_null() {
                gpu_common::dbg_sdl_error("failed to create fill pipeline");
                return AppResultWithState::Failure(None);
            }

            pipeline_create_info.rasterizer_state.fill_mode = SDL_GPU_FILLMODE_LINE;
            let line_pipeline = SDL_CreateGPUGraphicsPipeline(device, &pipeline_create_info);
            if line_pipeline.is_null() {
                gpu_common::dbg_sdl_error("failed to create line pipeline");
                return AppResultWithState::Failure(None);
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

            AppResultWithState::Continue(Box::new(Mutex::new(app)))
        }
    }

    fn app_iterate(&mut self) -> AppResult {
        unsafe {
            let ticks = SDL_GetTicks();
            self.game_state.step(ticks);

            let command_buffer = SDL_AcquireGPUCommandBuffer(self.device);
            if command_buffer.is_null() {
                gpu_common::dbg_sdl_error("failed to acquire command buffer");
                return AppResult::Failure;
            }

            let mut swapchain_texture: *mut SDL_GPUTexture = null_mut();
            if !SDL_AcquireGPUSwapchainTexture(
                command_buffer,
                self.window,
                &mut swapchain_texture,
                null_mut(),
                null_mut(),
            ) {
                gpu_common::dbg_sdl_error("failed to acquire swapchain texture");
                return AppResult::Failure;
            }

            if !swapchain_texture.is_null() {
                let color_target_info = SDL_GPUColorTargetInfo {
                    texture: swapchain_texture,
                    clear_color: SDL_FColor {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    load_op: SDL_GPU_LOADOP_CLEAR,
                    store_op: SDL_GPU_STOREOP_STORE,
                    ..Default::default()
                };
                let num_color_targets = 1;
                let depth_stencil_target_info = null_mut();
                let render_pass = SDL_BeginGPURenderPass(
                    command_buffer,
                    &color_target_info,
                    num_color_targets,
                    depth_stencil_target_info,
                );

                let pipeline = if self.game_state.use_wire_frame_mode {
                    self.line_pipeline
                } else {
                    self.fill_pipeline
                };
                SDL_BindGPUGraphicsPipeline(render_pass, pipeline);

                if self.game_state.use_small_viewport {
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

                if self.game_state.use_scissor_rect {
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

    fn app_event(&mut self, event: &SDL_Event) -> AppResult {
        let event_type = unsafe { event.r#type };

        match SDL_EventType(event_type) {
            SDL_EVENT_QUIT => return AppResult::Success,

            SDL_EVENT_KEY_DOWN => {
                let scancode = unsafe { event.key.scancode };
                if !self.game_state.keys_just_pressed.contains(&scancode) {
                    self.game_state.keys_just_pressed.push(scancode);
                }
            }

            SDL_EVENT_KEY_UP => {
                let scancode = unsafe { event.key.scancode };
                self.game_state.keys_just_pressed.retain(|k| *k != scancode);
            }

            _ => {}
        }

        AppResult::Continue
    }
}

const MILLIS_PER_FRAME: u64 = 16;

struct GameState {
    accumulated_ticks: u64,
    last_step: u64,
    keys_just_pressed: Vec<SDL_Scancode>,

    use_wire_frame_mode: bool,
    use_small_viewport: bool,
    use_scissor_rect: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            accumulated_ticks: 0,
            last_step: 0,
            keys_just_pressed: vec![],

            use_wire_frame_mode: false,
            use_small_viewport: false,
            use_scissor_rect: false,
        }
    }

    fn step(&mut self, current_tick: u64) {
        let new_ticks = current_tick - self.last_step;
        self.accumulated_ticks += new_ticks;
        self.last_step = current_tick;

        while self.accumulated_ticks >= MILLIS_PER_FRAME {
            self.accumulated_ticks -= MILLIS_PER_FRAME;
            self.fixed_step();

            self.keys_just_pressed.clear();
        }
    }

    fn fixed_step(&mut self) {
        if self.keys_just_pressed.contains(&SDL_Scancode::LEFT) {
            self.use_wire_frame_mode = !self.use_wire_frame_mode;
        }
        if self.keys_just_pressed.contains(&SDL_Scancode::DOWN) {
            self.use_small_viewport = !self.use_small_viewport;
        }
        if self.keys_just_pressed.contains(&SDL_Scancode::RIGHT) {
            self.use_scissor_rect = !self.use_scissor_rect;
        }
    }
}
