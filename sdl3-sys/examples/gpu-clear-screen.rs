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
}

impl Drop for AppState {
    fn drop(&mut self) {
        unsafe {
            if !self.device.is_null() && !self.device.is_null() {
                SDL_ReleaseWindowFromGPUDevice(self.device, self.window);
            }

            if !self.window.is_null() {
                SDL_DestroyWindow(self.window);
            }
            if !self.device.is_null() {
                SDL_DestroyGPUDevice(self.device);
            }
        }
    }
}

unsafe impl Send for AppState {}

#[app_init]
fn app_init() -> Option<Box<Mutex<AppState>>> {
    unsafe {
        if !SDL_Init(SDL_INIT_VIDEO) {
            dbg_sdl_error("SDL_Init failed");
            return None;
        }

        let (window, device) = init_gpu_window(c"GPU Clear Screen".as_ptr(), SDL_WINDOW_RESIZABLE)?;

        let app = AppState { window, device };

        Some(Box::new(Mutex::new(app)))
    }
}

#[app_iterate]
fn app_iterate(app: &mut AppState) -> AppResult {
    unsafe {
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
                cycle: false,
                cycle_resolve_texture: false,
                resolve_texture: null_mut(),
                mip_level: 0,
                layer_or_depth_plane: 0,
                resolve_mip_level: 0,
                resolve_layer: 0,
                padding1: 0,
                padding2: 0,
            };

            let num_color_targets = 1;
            let depth_stencil_target_info = null_mut();
            let render_pass = SDL_BeginGPURenderPass(
                command_buffer,
                &color_target_info,
                num_color_targets,
                depth_stencil_target_info,
            );
            SDL_EndGPURenderPass(render_pass);
        }
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
