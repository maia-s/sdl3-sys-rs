use core::ffi::CStr;

use sdl3_sys::{
    error::SDL_GetError,
    init::{SDL_Init, SDL_Quit, SDL_INIT_VIDEO},
};

fn main() {
    if unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        println!("Successfully initialized SDL!");
        unsafe {
            SDL_Quit();
        }
    } else {
        panic!(
            "{}",
            unsafe { CStr::from_ptr(SDL_GetError()) }.to_string_lossy()
        );
    }
}
