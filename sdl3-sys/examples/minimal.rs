use core::ffi::CStr;
use sdl3_sys::{
    error::SDL_GetError,
    init::{SDL_Init, SDL_Quit, SDL_INIT_VIDEO},
};

fn main() -> Result<(), &'static CStr> {
    if unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        println!("Successfully initialized SDL!");
        unsafe { SDL_Quit() };
        Ok(())
    } else {
        Err(unsafe { CStr::from_ptr(SDL_GetError()) })
    }
}
