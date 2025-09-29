// based on showfont.c from the SDL_ttf examples
// This is an almost direct source port; it's not idiomatic Rust
/*
  showfont:  An example of using the SDL_ttf library with 2D graphics.
  Copyright (C) 2001-2025 Sam Lantinga <slouken@libsdl.org>
  Rust port (C) 2025 Maia S. R.

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/

mod editbox;
use editbox::EditBox;

use core::{ffi::c_int, ptr};
use sdl3_sys::{
    assert::SDL_assert,
    error::SDL_GetError,
    events::{
        SDL_EVENT_KEY_DOWN, SDL_EVENT_MOUSE_BUTTON_DOWN, SDL_EVENT_QUIT, SDL_Event,
        SDL_KeyboardEvent, SDL_PollEvent,
    },
    init::{SDL_INIT_VIDEO, SDL_Init, SDL_Quit},
    keycode::{
        SDL_KMOD_CTRL, SDLK_A, SDLK_B, SDLK_DOWN, SDLK_ESCAPE, SDLK_I, SDLK_LEFT, SDLK_O, SDLK_R,
        SDLK_RIGHT, SDLK_S, SDLK_U, SDLK_UP,
    },
    log::SDL_Log,
    pixels::{SDL_ALPHA_OPAQUE, SDL_Color},
    rect::{SDL_FRect, SDL_Rect},
    render::{
        SDL_ConvertEventToRenderCoordinates, SDL_CreateRenderer, SDL_CreateSoftwareRenderer,
        SDL_CreateTextureFromSurface, SDL_DestroyRenderer, SDL_FlushRenderer, SDL_RenderClear,
        SDL_RenderFillRect, SDL_RenderPresent, SDL_RenderRect, SDL_RenderTexture, SDL_Renderer,
        SDL_SetRenderDrawColor, SDL_SetRenderVSync, SDL_Texture,
    },
    surface::{SDL_DestroySurface, SDL_SaveBMP, SDL_Surface},
    video::{
        SDL_CreateWindow, SDL_DestroyWindow, SDL_GetWindowSurface, SDL_SetWindowSurfaceVSync,
        SDL_UpdateWindowSurface, SDL_Window, SDL_WindowFlags,
    },
};
use sdl3_ttf_sys::ttf::{
    TTF_AddFallbackFont, TTF_CloseFont, TTF_CreateRendererTextEngine, TTF_CreateSurfaceTextEngine,
    TTF_CreateText, TTF_DIRECTION_BTT, TTF_DIRECTION_INVALID, TTF_DIRECTION_LTR, TTF_DIRECTION_RTL,
    TTF_DIRECTION_TTB, TTF_DestroyRendererTextEngine, TTF_DestroySurfaceTextEngine,
    TTF_DrawRendererText, TTF_DrawSurfaceText, TTF_Font, TTF_GetFontDirection, TTF_GetFontHeight,
    TTF_GetFontOutline, TTF_GetFontSize, TTF_GetFontStyle, TTF_GetFontWrapAlignment,
    TTF_GetTextPosition, TTF_GetTextSize, TTF_HINTING_LIGHT, TTF_HINTING_MONO, TTF_HINTING_NONE,
    TTF_HINTING_NORMAL, TTF_HORIZONTAL_ALIGN_CENTER, TTF_HORIZONTAL_ALIGN_LEFT,
    TTF_HORIZONTAL_ALIGN_RIGHT, TTF_Init, TTF_OpenFont, TTF_Quit, TTF_RenderGlyph_Shaded,
    TTF_RenderText_Blended, TTF_RenderText_Blended_Wrapped, TTF_RenderText_Shaded,
    TTF_RenderText_Shaded_Wrapped, TTF_RenderText_Solid, TTF_RenderText_Solid_Wrapped,
    TTF_STYLE_BOLD, TTF_STYLE_ITALIC, TTF_STYLE_NORMAL, TTF_STYLE_STRIKETHROUGH,
    TTF_STYLE_UNDERLINE, TTF_SetFontDirection, TTF_SetFontHinting, TTF_SetFontKerning,
    TTF_SetFontOutline, TTF_SetFontSize, TTF_SetFontStyle, TTF_SetFontWrapAlignment,
    TTF_SetTextColor, TTF_SetTextPosition, TTF_Text,
};
use std::{env, ffi::CString, process::ExitCode};

const DEFAULT_PTSIZE: f32 = 18.0;
const DEFAULT_TEXT: &str = "The quick brown fox jumped over the lazy dog";
const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn usage(name: &str) {
    eprintln!(
        "Usage: {name} [--textengine surface|renderer] [--solid] [--shaded] [--blended] [-b] [-i] [-u] [-s] [--outline size] [--hintlight|--hintmono|--hintnone] [--nokerning] [--wrap] [--align left|center|right] [--fgcol r,g,b,a] [--bgcol r,g,b,a] [--disable-editbox] [--fallback <font>.ttf] <font>.ttf [ptsize] [text]"
    );
}

struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

macro_rules! defer {
    ($expr:expr) => {
        let _deferred = Defer(Some(|| {
            $expr;
        }));
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextEngine {
    None,
    Surface,
    Renderer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextRenderMethod {
    Solid,
    Shaded,
    Blended,
}

struct Scene {
    done: bool,
    window: *mut SDL_Window,
    window_surface: *mut SDL_Surface,
    renderer: *mut SDL_Renderer,
    font: *mut TTF_Font,
    caption: *mut TTF_Text,
    caption_rect: SDL_Rect,
    message: *mut SDL_Texture,
    message_rect: SDL_FRect,
    text_engine: TextEngine,
    text_rect: SDL_FRect,
    edit: Option<Box<EditBox>>,
}

impl Scene {
    fn new() -> Self {
        Self {
            done: false,
            window: ptr::null_mut(),
            window_surface: ptr::null_mut(),
            renderer: ptr::null_mut(),
            font: ptr::null_mut(),
            caption: ptr::null_mut(),
            caption_rect: SDL_Rect::default(),
            message: ptr::null_mut(),
            message_rect: SDL_FRect::default(),
            text_engine: TextEngine::None,
            text_rect: SDL_FRect::default(),
            edit: None,
        }
    }

    fn draw(&mut self) {
        unsafe {
            let renderer = self.renderer;

            // Clear the background to background color
            SDL_SetRenderDrawColor(renderer, 0xff, 0xff, 0xff, 0xff);
            SDL_RenderClear(renderer);

            if let Some(edit) = &mut self.edit {
                // Clear the text rect to light gray
                SDL_SetRenderDrawColor(renderer, 0xcc, 0xcc, 0xcc, 0xff);
                SDL_RenderFillRect(renderer, &self.text_rect);

                if edit.has_focus {
                    let mut focus_rect = self.text_rect;
                    focus_rect.x -= 1.0;
                    focus_rect.y -= 1.0;
                    focus_rect.w += 2.0;
                    focus_rect.h += 2.0;
                    SDL_SetRenderDrawColor(renderer, 0x00, 0x00, 0x00, 0xff);
                    SDL_RenderRect(renderer, &focus_rect);
                }

                edit.draw();
            }

            match self.text_engine {
                TextEngine::Surface => {
                    // Flush the renderer so we can draw directly to the window surface
                    SDL_FlushRenderer(renderer);
                    TTF_DrawSurfaceText(
                        self.caption,
                        self.caption_rect.x,
                        self.caption_rect.y,
                        self.window_surface,
                    );
                }
                TextEngine::Renderer => {
                    TTF_DrawRendererText(
                        self.caption,
                        self.caption_rect.x as _,
                        self.caption_rect.y as _,
                    );
                }
                TextEngine::None => {
                    SDL_assert!(false);
                }
            }

            SDL_RenderTexture(renderer, self.message, ptr::null(), &self.message_rect);
            SDL_RenderPresent(renderer);

            if !self.window_surface.is_null() {
                SDL_UpdateWindowSurface(self.window);
            }
        }
    }

    fn handle_key_down(&mut self, event: &SDL_KeyboardEvent) {
        unsafe {
            match event.key {
                SDLK_A => {
                    // Cycle alignment
                    match TTF_GetFontWrapAlignment(self.font) {
                        TTF_HORIZONTAL_ALIGN_LEFT => {
                            TTF_SetFontWrapAlignment(self.font, TTF_HORIZONTAL_ALIGN_CENTER)
                        }
                        TTF_HORIZONTAL_ALIGN_CENTER => {
                            TTF_SetFontWrapAlignment(self.font, TTF_HORIZONTAL_ALIGN_RIGHT)
                        }
                        TTF_HORIZONTAL_ALIGN_RIGHT => {
                            TTF_SetFontWrapAlignment(self.font, TTF_HORIZONTAL_ALIGN_LEFT)
                        }
                        _ => SDL_Log(
                            c"Unknown wrap alignment: %d".as_ptr(),
                            TTF_GetFontWrapAlignment(self.font),
                        ),
                    }
                }
                SDLK_B => {
                    // Toggle bold style
                    TTF_SetFontStyle(self.font, TTF_GetFontStyle(self.font) ^ TTF_STYLE_BOLD);
                }
                SDLK_I => {
                    // Toggle italic style
                    TTF_SetFontStyle(self.font, TTF_GetFontStyle(self.font) ^ TTF_STYLE_ITALIC);
                }
                SDLK_O => {
                    // Toggle outline
                    let mut outline = TTF_GetFontOutline(self.font);
                    if outline != 0 {
                        outline = 0;
                    } else {
                        outline = 1;
                    }
                    TTF_SetFontOutline(self.font, outline);
                }
                SDLK_R => {
                    // Toggle layout direction
                    match TTF_GetFontDirection(self.font) {
                        TTF_DIRECTION_INVALID | TTF_DIRECTION_LTR => {
                            TTF_SetFontDirection(self.font, TTF_DIRECTION_RTL);
                        }
                        TTF_DIRECTION_RTL => {
                            TTF_SetFontDirection(self.font, TTF_DIRECTION_LTR);
                        }
                        TTF_DIRECTION_TTB => {
                            TTF_SetFontDirection(self.font, TTF_DIRECTION_BTT);
                        }
                        TTF_DIRECTION_BTT => {
                            TTF_SetFontDirection(self.font, TTF_DIRECTION_TTB);
                        }
                        _ => (),
                    }
                }
                SDLK_S => {
                    // Toggle strike-through style
                    TTF_SetFontStyle(
                        self.font,
                        TTF_GetFontStyle(self.font) ^ TTF_STYLE_STRIKETHROUGH,
                    );
                }
                SDLK_U => {
                    // Toggle underline style
                    TTF_SetFontStyle(self.font, TTF_GetFontStyle(self.font) ^ TTF_STYLE_UNDERLINE);
                }
                SDLK_LEFT => {
                    if event.r#mod & SDL_KMOD_CTRL != 0 {
                        if let Some(edit) = &self.edit {
                            adjust_text_offset(edit.text, -1, 0);
                        }
                    }
                }
                SDLK_RIGHT => {
                    if event.r#mod & SDL_KMOD_CTRL != 0 {
                        if let Some(edit) = &self.edit {
                            adjust_text_offset(edit.text, 1, 0);
                        }
                    }
                }
                SDLK_UP => {
                    if event.r#mod & SDL_KMOD_CTRL != 0 {
                        if let Some(edit) = &self.edit {
                            adjust_text_offset(edit.text, 0, -1);
                        }
                    } else {
                        // Increase font size
                        let ptsize = TTF_GetFontSize(self.font);
                        TTF_SetFontSize(self.font, ptsize + 1.0);
                    }
                }
                SDLK_DOWN => {
                    if event.r#mod & SDL_KMOD_CTRL != 0 {
                        if let Some(edit) = &self.edit {
                            adjust_text_offset(edit.text, 0, 1);
                        }
                    } else {
                        // Increase font size
                        let ptsize = TTF_GetFontSize(self.font);
                        TTF_SetFontSize(self.font, ptsize - 1.0);
                    }
                }
                SDLK_ESCAPE => self.done = true,
                _ => (),
            }
        }
    }
}

fn adjust_text_offset(text: *mut TTF_Text, xoffset: c_int, yoffset: c_int) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        TTF_GetTextPosition(text, &mut x, &mut y);
        x += xoffset;
        y += yoffset;
        TTF_SetTextPosition(text, x, y);
    }
}

fn main() -> ExitCode {
    let white = SDL_Color {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: SDL_ALPHA_OPAQUE,
    };
    let black = SDL_Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: SDL_ALPHA_OPAQUE,
    };
    let mut forecol = black;
    let mut backcol = white;
    let mut rendermethod = TextRenderMethod::Shaded;
    let mut renderstyle = TTF_STYLE_NORMAL;
    let mut outline = 0;
    let mut hinting = TTF_HINTING_NORMAL;
    let mut kerning = true;
    let mut wrap = false;
    let mut align = TTF_HORIZONTAL_ALIGN_LEFT;
    let mut editbox = true;
    let mut dump = false;
    let mut fallback_font_files = Vec::new();

    let mut scene = Scene::new();
    scene.text_engine = TextEngine::Renderer;

    let mut args_it = env::args().peekable();
    let arg0 = args_it.next().unwrap();

    while let Some(arg) = args_it.peek() {
        if arg.starts_with('-') {
            let arg = args_it.next().unwrap();
            match arg.as_str() {
                "--fallback" => {
                    fallback_font_files.push(
                        CString::new(
                            args_it
                                .next()
                                .expect("missing argument for --fallback")
                                .clone(),
                        )
                        .unwrap(),
                    );
                }
                "--textengine" => {
                    match args_it
                        .next()
                        .expect("missing argument for --textengine")
                        .as_str()
                    {
                        "surface" => scene.text_engine = TextEngine::Surface,
                        "renderer" => scene.text_engine = TextEngine::Renderer,
                        _ => {
                            usage(&arg0);
                            return ExitCode::FAILURE;
                        }
                    }
                }
                "--solid" => rendermethod = TextRenderMethod::Solid,
                "--shaded" => rendermethod = TextRenderMethod::Shaded,
                "--blended" => rendermethod = TextRenderMethod::Blended,
                "-b" => renderstyle |= TTF_STYLE_BOLD,
                "-i" => renderstyle |= TTF_STYLE_ITALIC,
                "-u" => renderstyle |= TTF_STYLE_UNDERLINE,
                "-s" => renderstyle |= TTF_STYLE_STRIKETHROUGH,
                "--outline" => {
                    outline = args_it
                        .next()
                        .expect("missing argument for --outline")
                        .parse()
                        .expect("invalid argument for --outline")
                }
                "--hintlight" => hinting = TTF_HINTING_LIGHT,
                "--hintmono" => hinting = TTF_HINTING_MONO,
                "--hintnone" => hinting = TTF_HINTING_NONE,
                "--nokerning" => kerning = false,
                "--wrap" => wrap = true,
                "--align" => match args_it
                    .next()
                    .expect("missing argument for --align")
                    .as_str()
                {
                    "left" => align = TTF_HORIZONTAL_ALIGN_LEFT,
                    "center" => align = TTF_HORIZONTAL_ALIGN_CENTER,
                    "right" => align = TTF_HORIZONTAL_ALIGN_RIGHT,
                    _ => {
                        usage(&arg0);
                        return ExitCode::FAILURE;
                    }
                },
                "--fgcol" => {
                    let arg = args_it.next().expect("missing argument for --fgcol");
                    let col: Vec<u8> = arg
                        .split(',')
                        .map(|n| n.parse::<u8>().expect("invalid color value"))
                        .collect();
                    forecol.r = col[0];
                    forecol.g = col[1];
                    forecol.b = col[2];
                    forecol.a = col[3];
                }
                "--bgcol" => {
                    let arg = args_it.next().expect("missing argument for --bgcol");
                    let col: Vec<u8> = arg
                        .split(',')
                        .map(|n| n.parse::<u8>().expect("invalid color value"))
                        .collect();
                    backcol.r = col[0];
                    backcol.g = col[1];
                    backcol.b = col[2];
                    backcol.a = col[3];
                }
                "--disable-editbox" => editbox = false,
                "--dump" => dump = true,
                _ => (),
            }
        } else {
            break;
        }
    }

    let font_file = args_it.next().expect("missing font");
    let font_file_c = CString::new(font_file.clone()).unwrap();

    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        unsafe { SDL_Log(c"Couldn't initialize SDL: %s".as_ptr(), SDL_GetError()) };
        return ExitCode::FAILURE;
    }
    defer!(unsafe { SDL_Quit() });

    // Initialize the TTF library
    if !unsafe { TTF_Init() } {
        unsafe { SDL_Log(c"Couldn't initialize TTF: %s".as_ptr(), SDL_GetError()) };
        return ExitCode::FAILURE;
    }
    defer!(unsafe { TTF_Quit() });

    // Open the font file with the requested point size
    let mut ptsize = 0.0;
    if let Some(pt) = args_it.next() {
        ptsize = pt.parse().expect("invalid point size");
    }
    if ptsize == 0.0 {
        ptsize = DEFAULT_PTSIZE;
    }
    let font = unsafe { TTF_OpenFont(font_file_c.as_ptr(), ptsize) };
    if font.is_null() {
        unsafe {
            SDL_Log(
                c"Couldn't load %g pt font from %s: %s".as_ptr(),
                ptsize as f64,
                font_file_c.as_ptr(),
                SDL_GetError(),
            );
            return ExitCode::FAILURE;
        };
    }
    defer!(unsafe { TTF_CloseFont(font) });

    unsafe {
        TTF_SetFontStyle(font, renderstyle);
        TTF_SetFontOutline(font, outline);
        TTF_SetFontKerning(font, kerning);
        TTF_SetFontHinting(font, hinting);
        TTF_SetFontWrapAlignment(font, align);
    }
    scene.font = font;

    let mut fallback_fonts = Vec::with_capacity(fallback_font_files.len());
    for fallback in fallback_font_files {
        let f = unsafe { TTF_OpenFont(fallback.as_ptr(), ptsize) };
        if f.is_null() {
            unsafe {
                SDL_Log(
                    c"Couldn't load %g pt font from %s: %s".as_ptr(),
                    ptsize as f64,
                    fallback.as_ptr(),
                    SDL_GetError(),
                );
            }
            for f in fallback_fonts {
                unsafe { TTF_CloseFont(f) };
            }
            return ExitCode::FAILURE;
        }
        fallback_fonts.push(f);
        unsafe { TTF_AddFallbackFont(font, f) };
    }
    defer!(for f in fallback_fonts {
        unsafe { TTF_CloseFont(f) };
    });

    if dump {
        for i in 48..123 {
            let glyph = unsafe { TTF_RenderGlyph_Shaded(font, i, forecol, backcol) };
            if !glyph.is_null() {
                defer!(unsafe { SDL_DestroySurface(glyph) });
                let outname = CString::new(format!("glyph-{i}.bmp")).unwrap();
                unsafe { SDL_SaveBMP(glyph, outname.as_ptr()) };
            }
        }
        return ExitCode::SUCCESS;
    }

    // Create a window
    scene.window =
        unsafe { SDL_CreateWindow(c"showfont demo".as_ptr(), WIDTH, HEIGHT, SDL_WindowFlags(0)) };
    if scene.window.is_null() {
        unsafe { SDL_Log(c"SDL_CreateWindow() failed: %s".as_ptr(), SDL_GetError()) };
        return ExitCode::FAILURE;
    }
    let _scene_window = scene.window;
    defer!(unsafe { SDL_DestroyWindow(_scene_window) });

    if scene.text_engine == TextEngine::Surface {
        scene.window_surface = unsafe { SDL_GetWindowSurface(scene.window) };
        if scene.window_surface.is_null() {
            unsafe {
                SDL_Log(
                    c"SDL_GetWindowSurface() failed: %s".as_ptr(),
                    SDL_GetError(),
                )
            };
            return ExitCode::FAILURE;
        }
        unsafe {
            SDL_SetWindowSurfaceVSync(scene.window, 1);
            scene.renderer = SDL_CreateSoftwareRenderer(scene.window_surface);
        }
    } else {
        unsafe {
            scene.renderer = SDL_CreateRenderer(scene.window, ptr::null());
            if !scene.renderer.is_null() {
                SDL_SetRenderVSync(scene.renderer, 1);
            }
        }
    }
    if scene.renderer.is_null() {
        unsafe { SDL_Log(c"SDL_CreateRenderer() failed: %s".as_ptr(), SDL_GetError()) };
        return ExitCode::FAILURE;
    }
    let _scene_renderer = scene.renderer;
    defer!(unsafe { SDL_DestroyRenderer(_scene_renderer) });

    let engine;
    match scene.text_engine {
        TextEngine::Surface => unsafe {
            engine = TTF_CreateSurfaceTextEngine();
            if engine.is_null() {
                SDL_Log(
                    c"Couldn't create surface text engine: %s".as_ptr(),
                    SDL_GetError(),
                );
                return ExitCode::FAILURE;
            }
        },
        TextEngine::Renderer => unsafe {
            engine = TTF_CreateRendererTextEngine(scene.renderer);
            if engine.is_null() {
                SDL_Log(
                    c"Couldn't create renderer text engine: %s".as_ptr(),
                    SDL_GetError(),
                );
                return ExitCode::FAILURE;
            }
        },
        TextEngine::None => unsafe {
            SDL_Log(c"No text engine selected".as_ptr());
            return ExitCode::FAILURE;
        },
    }
    let _scene_text_engine = scene.text_engine;
    defer!(unsafe {
        match _scene_text_engine {
            TextEngine::Surface => TTF_DestroySurfaceTextEngine(engine),
            TextEngine::Renderer => TTF_DestroyRendererTextEngine(engine),
            TextEngine::None => (),
        }
    });

    // Show which font file we're looking at
    let string = CString::new(format!("Font file: {font_file}")).unwrap();
    unsafe {
        scene.caption = TTF_CreateText(engine, font, string.as_ptr(), 0);
        TTF_SetTextColor(scene.caption, forecol.r, forecol.g, forecol.b, forecol.a);
        scene.caption_rect.x = 4;
        scene.caption_rect.y = 4;
        TTF_GetTextSize(
            scene.caption,
            &mut scene.caption_rect.w,
            &mut scene.caption_rect.h,
        );
    }

    // Render and center the message
    let message;
    if let Some(msg) = args_it.next() {
        message = CString::new(msg).unwrap();
    } else {
        message = CString::new(DEFAULT_TEXT).unwrap();
    }
    let text = unsafe {
        match rendermethod {
            TextRenderMethod::Solid => {
                if wrap {
                    TTF_RenderText_Solid_Wrapped(font, message.as_ptr(), 0, forecol, 0)
                } else {
                    TTF_RenderText_Solid(font, message.as_ptr(), 0, forecol)
                }
            }
            TextRenderMethod::Shaded => {
                if wrap {
                    TTF_RenderText_Shaded_Wrapped(font, message.as_ptr(), 0, forecol, backcol, 0)
                } else {
                    TTF_RenderText_Shaded(font, message.as_ptr(), 0, forecol, backcol)
                }
            }
            TextRenderMethod::Blended => {
                if wrap {
                    TTF_RenderText_Blended_Wrapped(font, message.as_ptr(), 0, forecol, 0)
                } else {
                    TTF_RenderText_Blended(font, message.as_ptr(), 0, forecol)
                }
            }
        }
    };
    if text.is_null() {
        unsafe { SDL_Log(c"Couldn't render text: %s".as_ptr(), SDL_GetError()) };
        return ExitCode::FAILURE;
    }
    defer!(unsafe { SDL_DestroySurface(text) });

    unsafe {
        scene.message_rect = SDL_FRect {
            x: ((WIDTH - (*text).w) / 2) as _,
            y: ((HEIGHT - (*text).h) / 2) as _,
            w: (*text).w as _,
            h: (*text).h as _,
        };
        scene.message = SDL_CreateTextureFromSurface(scene.renderer, text);
        SDL_Log(
            c"Font is generally %d big, and string is %d big".as_ptr(),
            TTF_GetFontHeight(font),
            (*text).h,
        );
    }

    if editbox {
        scene.text_rect.x = 8.0;
        scene.text_rect.y = (scene.caption_rect.y + scene.caption_rect.h) as f32 + 4.0;
        scene.text_rect.w = (WIDTH / 2) as f32 - scene.text_rect.x * 2.0;
        scene.text_rect.h = scene.message_rect.y - scene.text_rect.y - 16.0;

        let edit_rect = SDL_FRect {
            x: scene.text_rect.x + 4.0,
            y: scene.text_rect.y + 4.0,
            w: scene.text_rect.w - 8.0,
            h: scene.text_rect.h - 8.0,
        };
        scene.edit = Some(Box::new(EditBox::new(
            scene.window,
            scene.renderer,
            engine,
            font,
            &edit_rect,
        )));
        if let Some(edit) = &mut scene.edit {
            unsafe { TTF_SetTextColor(edit.text, forecol.r, forecol.g, forecol.b, forecol.a) };
            edit.insert(message.as_ptr());
        }
    }

    // Wait for a keystroke, and blit text on mouse press
    let mut event = SDL_Event::default();
    while !scene.done {
        unsafe {
            while SDL_PollEvent(&mut event) {
                SDL_ConvertEventToRenderCoordinates(scene.renderer, &mut event);
                match event.event_type() {
                    SDL_EVENT_MOUSE_BUTTON_DOWN => {
                        if scene.edit.is_none()
                            || !scene.edit.as_mut().unwrap().handle_event(&event)
                        {
                            let text = &*text;
                            scene.message_rect = SDL_FRect {
                                x: event.button.x - (text.w / 2) as f32,
                                y: event.button.y - (text.h / 2) as f32,
                                w: text.w as f32,
                                h: text.h as f32,
                            }
                        }
                    }
                    SDL_EVENT_KEY_DOWN => {
                        if scene.edit.is_none()
                            || !scene.edit.as_mut().unwrap().handle_event(&event)
                        {
                            scene.handle_key_down(&event.key);
                        }
                    }
                    SDL_EVENT_QUIT => scene.done = true,
                    _ => {
                        if let Some(edit) = &mut scene.edit {
                            edit.handle_event(&event);
                        }
                    }
                }
            }
            scene.draw();
        }
    }

    ExitCode::SUCCESS
}
