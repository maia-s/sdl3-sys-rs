//! Based on the official example in C here:
//! https://github.com/TheSpydog/SDL_gpu_examples/blob/main/Examples/ClearScreen.c

use std::ffi::CStr;
use std::ptr::null_mut;
use std::sync::Mutex;

use sdl3_main::{AppResult, AppResultWithState, app_impl};
use sdl3_sys::everything::*;

struct AppState {
    window: *mut SDL_Window,
    device: *mut SDL_GPUDevice,
}

impl Drop for AppState {
    fn drop(&mut self) {
        unsafe {
            gpu_common::deinit_gpu_window(self.device, self.window);
        }
    }
}

unsafe impl Send for AppState {}

#[app_impl]
impl AppState {
    fn app_init() -> AppResultWithState<Box<Mutex<Self>>> {
        const TITLE: &CStr = c"Clear Screen Example";
        let Some((window, device)) = gpu_common::init_gpu_window(TITLE, SDL_WindowFlags::default())
        else {
            return AppResultWithState::Failure(None);
        };

        let app = AppState { window, device };

        AppResultWithState::Continue(Box::new(Mutex::new(app)))
    }

    fn app_iterate(&mut self) -> AppResult {
        unsafe {
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
                        r: 0.3,
                        g: 0.4,
                        b: 0.5,
                        a: 1.0,
                    },
                    load_op: SDL_GPULoadOp::CLEAR,
                    store_op: SDL_GPUStoreOp::STORE,
                    ..Default::default()
                };
                let render_pass =
                    SDL_BeginGPURenderPass(command_buffer, &color_target_info, 1, null_mut());
                SDL_EndGPURenderPass(render_pass);
            }

            SDL_SubmitGPUCommandBuffer(command_buffer);
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
