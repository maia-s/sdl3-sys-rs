// Based on showimage.c from the SDL_image examples.
// Like the original, this code is licensed under the Zlib license.

use sdl3_image_sys::{IMG_Load, IMG_LoadTexture, IMG_SaveAVIF, IMG_SaveJPG, IMG_SavePNG};
use sdl3_sys::everything::*;
use std::{
    env,
    ffi::{CStr, CString},
    process::ExitCode,
    ptr,
};

struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

macro_rules! defer {
    ($expr:expr) => {
        let _deferred = Defer(Some(|| $expr));
    };
}

macro_rules! log_and_quit {
    ($status:expr, $fmt:literal $(, $args:expr)* $(,)?) => {{
        let fmt: &CStr = $fmt;
        unsafe { SDL_Log(fmt.as_ptr(), $($args),*) };
        return ExitCode::from($status);
    }};
}

fn draw_background(renderer: *mut SDL_Renderer, w: i32, h: i32) {
    let dx = 8;
    let dy = 8;

    let mut rect = SDL_FRect {
        w: dx as _,
        h: dy as _,
        ..Default::default()
    };

    for y in (0..h).step_by(dy) {
        for x in (0..w).step_by(dx) {
            let c = 0x66 + 0x33 * (((x ^ y) >> 3) & 1) as u8;
            unsafe { SDL_SetRenderDrawColor(renderer, c, c, c, 0xff) };

            rect.x = x as _;
            rect.y = y as _;
            unsafe { SDL_RenderFillRect(renderer, &rect) };
        }
    }
}

fn main() -> ExitCode {
    let Ok(args) = env::args().map(CString::new).collect::<Result<Vec<_>, _>>() else {
        log_and_quit!(1, c"null byte in argument");
    };

    if args.len() < 2 {
        log_and_quit!(
            1,
            c"Usage: %s [-fullscreen] [-tonemap X] [-save file.png] <image_file> ...",
            &args[0],
        );
    }

    let mut flags = SDL_WINDOW_HIDDEN;

    struct Image<'a> {
        file: &'a CStr,
        tonemap: Option<&'a CStr>,
    }

    let mut images = Vec::new();
    let mut args_it = args.iter().map(|arg| arg.as_c_str());
    let mut quit = false;
    let mut tonemap = None;
    let mut save_file = None;
    let mut result = 0;

    args_it.next();
    while let Some(arg) = args_it.next() {
        match arg {
            arg if arg == c"-fullscreen" => {
                unsafe { SDL_HideCursor() };
                flags |= SDL_WINDOW_FULLSCREEN;
            }
            arg if arg == c"-quit" => {
                quit = true;
                break;
            }
            arg if arg == c"-tonemap" => {
                if let Some(tonemap_) = args_it.next() {
                    tonemap = Some(tonemap_);
                    continue;
                }
            }
            arg if arg == c"-save" => {
                if let Some(save_file_) = args_it.next() {
                    save_file = Some(save_file_);
                    continue;
                }
            }
            _ => (),
        }

        // save the image file, if desired
        if let Some(save_file) = save_file.take() {
            let save_file = save_file.as_ptr();
            unsafe {
                let surface = IMG_Load(arg.as_ptr());
                if !surface.is_null() {
                    let ext = SDL_strrchr(save_file, b'.' as _);
                    let mut saved = false;
                    if !ext.is_null() && SDL_strcasecmp(ext, c".avif".as_ptr()) == 0 {
                        saved = IMG_SaveAVIF(surface, save_file, 90);
                    } else if !ext.is_null() && SDL_strcasecmp(ext, c".bmp".as_ptr()) == 0 {
                        saved = SDL_SaveBMP(surface, save_file);
                    } else if !ext.is_null() && SDL_strcasecmp(ext, c".jpg".as_ptr()) == 0 {
                        saved = IMG_SaveJPG(surface, save_file, 90);
                    } else if !ext.is_null() && SDL_strcasecmp(ext, c".png".as_ptr()) == 0 {
                        saved = IMG_SavePNG(surface, save_file);
                    } else {
                        SDL_SetError(c"Unknown save file type".as_ptr());
                    }
                    SDL_DestroySurface(surface);
                    if !saved {
                        SDL_Log(c"Couldn't save %s: %s".as_ptr(), save_file, SDL_GetError());
                        result = 3;
                    }
                } else {
                    SDL_Log(
                        c"Couldn't load %s: %s".as_ptr(),
                        arg.as_ptr(),
                        SDL_GetError(),
                    );
                    result = 3;
                }
            }
        }

        images.push(Image {
            file: arg,
            tonemap: tonemap.take(),
        });
    }

    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        log_and_quit!(2, c"SDL_Init(SDL_INIT_VIDEO) failed: %s)", SDL_GetError(),);
    }
    defer!(unsafe { SDL_Quit() });

    let window = unsafe { SDL_CreateWindow(c"".as_ptr(), 0, 0, flags) };
    if window.is_null() {
        log_and_quit!(2, c"SDL_CreateWindow() failed: %s", SDL_GetError());
    }
    defer!(unsafe { SDL_DestroyWindow(window) });

    let mut renderer = ptr::null_mut();
    if unsafe {
        SDL_GetBooleanProperty(
            SDL_GetDisplayProperties(SDL_GetPrimaryDisplay()),
            SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN,
            false,
        )
    } {
        unsafe {
            let props = SDL_CreateProperties();
            SDL_SetPointerProperty(
                props,
                SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
                window as *mut _,
            );
            SDL_SetNumberProperty(
                props,
                SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER,
                SDL_Colorspace::SRGB_LINEAR.0 as _,
            );
            renderer = SDL_CreateRendererWithProperties(props);
            SDL_DestroyProperties(props);
        }
    }
    if renderer.is_null() {
        renderer = unsafe { SDL_CreateRenderer(window, ptr::null()) };
        if renderer.is_null() {
            log_and_quit!(2, c"SDL_CreateRenderer() failed: %s", SDL_GetError());
        }
    }
    defer!(unsafe { SDL_DestroyRenderer(renderer) });

    let mut i = 0;
    'images: while i < images.len() {
        let image = &mut images[i];
        let texture;
        let mut w = 0.0;
        let mut h = 0.0;

        // open the image file
        if let Some(tonemap) = image.tonemap {
            unsafe {
                let surface = IMG_Load(image.file.as_ptr());
                if surface.is_null() {
                    SDL_Log(
                        c"Couldn't load %s: %s".as_ptr(),
                        image.file.as_ptr(),
                        SDL_GetError(),
                    );
                    i += 1;
                    continue;
                }

                // use the tonemap operator to convert to SDR output
                SDL_SetStringProperty(
                    SDL_GetSurfaceProperties(surface),
                    SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING,
                    tonemap.as_ptr(),
                );
                let temp = SDL_ConvertSurface(surface, SDL_PixelFormat::RGBA32);
                SDL_DestroySurface(surface);
                if temp.is_null() {
                    SDL_Log(c"Couldn't convert surface: %s".as_ptr(), SDL_GetError());
                    i += 1;
                    continue;
                }

                texture = SDL_CreateTextureFromSurface(renderer, temp);
                SDL_DestroySurface(temp);
                if texture.is_null() {
                    SDL_Log(c"Couldn't create texture: %s".as_ptr(), SDL_GetError());
                    i += 1;
                    continue;
                }
            }
        } else {
            unsafe {
                texture = IMG_LoadTexture(renderer, image.file.as_ptr());
                if texture.is_null() {
                    SDL_Log(
                        c"Couldn't load %s: %s".as_ptr(),
                        image.file.as_ptr(),
                        SDL_GetError(),
                    );
                    i += 1;
                    continue;
                }
            }
        }
        defer!(unsafe { SDL_DestroyTexture(texture) });
        unsafe { SDL_GetTextureSize(texture, &mut w, &mut h) };

        // show the window
        unsafe {
            SDL_SetWindowTitle(window, image.file.as_ptr());
            SDL_SetWindowSize(window, w as _, h as _);
            SDL_ShowWindow(window);
        }

        let mut done = quit && i == images.len() - 1;
        while !done {
            let mut event = SDL_Event::default();
            unsafe {
                while SDL_PollEvent(&mut event) {
                    match SDL_EventType(event.r#type) {
                        SDL_EventType::KEY_UP => match event.key.key {
                            SDLK_LEFT => {
                                if i > 0 {
                                    i -= 1;
                                    continue 'images;
                                }
                            }
                            SDLK_RIGHT => {
                                if i != images.len() - 1 {
                                    done = true
                                }
                            }
                            SDLK_ESCAPE | SDLK_Q => {
                                images.clear();
                                done = true;
                            }
                            SDLK_SPACE | SDLK_TAB => done = true,
                            _ => (),
                        },
                        SDL_EventType::MOUSE_BUTTON_DOWN => done = true,
                        SDL_EventType::QUIT => {
                            images.clear();
                            done = true;
                        }
                        _ => (),
                    }
                }

                // draw a background pattern in case the image has transparency
                draw_background(renderer, w as _, h as _);

                // display the image
                SDL_RenderTexture(renderer, texture, ptr::null(), ptr::null());
                SDL_RenderPresent(renderer);

                SDL_Delay(100);
            }
        }

        i += 1;
    }

    ExitCode::from(result)
}
