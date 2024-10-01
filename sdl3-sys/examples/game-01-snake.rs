// This is a Rust port of examples/game/01-snake from SDL.
//
// While it uses some Rust concepts, it's not intended to be idiomatic Rust,
// but rather a close translation of the original.
//
// Like the original example, this code is public domain.
//
// original description:
//
// Logic implementation of the Snake game. It is designed to efficiently
// represent the state of the game in memory.
//
// This code is public domain. Feel free to use it for any purpose!

use core::{ffi::c_char, mem::transmute, ptr::null_mut};
use std::sync::Mutex;

use sdl3_main::{app_event, app_init, app_iterate, app_quit, AppResult};

// You can `use sdl3_sys::everything::*` if you don't want to specify everything explicitly
use sdl3_sys::{
    events::{SDL_Event, SDL_EventType, SDL_EVENT_KEY_DOWN, SDL_EVENT_QUIT},
    init::{
        SDL_Init, SDL_SetAppMetadata, SDL_SetAppMetadataProperty, SDL_INIT_VIDEO,
        SDL_PROP_APP_METADATA_COPYRIGHT_STRING, SDL_PROP_APP_METADATA_CREATOR_STRING,
        SDL_PROP_APP_METADATA_TYPE_STRING, SDL_PROP_APP_METADATA_URL_STRING,
    },
    pixels::SDL_ALPHA_OPAQUE,
    rect::SDL_FRect,
    render::{
        SDL_CreateWindowAndRenderer, SDL_DestroyRenderer, SDL_RenderClear, SDL_RenderFillRect,
        SDL_RenderPresent, SDL_Renderer, SDL_SetRenderDrawColor,
    },
    scancode::{
        SDL_Scancode, SDL_SCANCODE_DOWN, SDL_SCANCODE_ESCAPE, SDL_SCANCODE_LEFT, SDL_SCANCODE_Q,
        SDL_SCANCODE_R, SDL_SCANCODE_RIGHT, SDL_SCANCODE_UP,
    },
    stdinc::SDL_rand,
    timer::SDL_GetTicks,
    video::{SDL_DestroyWindow, SDL_Window},
};

const STEP_RATE_IN_MILLISECONDS: u32 = 125;
const SNAKE_BLOCK_SIZE_IN_PIXELS: i32 = 24;
const SDL_WINDOW_WIDTH: i32 = SNAKE_BLOCK_SIZE_IN_PIXELS * SNAKE_GAME_WIDTH as i32;
const SDL_WINDOW_HEIGHT: i32 = SNAKE_BLOCK_SIZE_IN_PIXELS * SNAKE_GAME_HEIGHT as i32;

const SNAKE_GAME_WIDTH: i8 = 24;
const SNAKE_GAME_HEIGHT: i8 = 18;
const SNAKE_MATRIX_SIZE: u16 = SNAKE_GAME_WIDTH as u16 * SNAKE_GAME_HEIGHT as u16;
const SNAKE_ARRAY_SIZE: usize = (((SNAKE_MATRIX_SIZE + 1) * SNAKE_CELL_MAX_BITS + 7) / 8) as usize;

const THREE_BITS: u16 = (1_u16 << SNAKE_CELL_MAX_BITS) - 1;
const fn shift(x: i8, y: i8) -> (usize, u16) {
    let shift = (x as u16 + (y as u16 * SNAKE_GAME_WIDTH as u16)) * SNAKE_CELL_MAX_BITS;
    ((shift / 8) as usize, shift % 8)
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SnakeCell {
    Nothing,
    SRight,
    SUp,
    SLeft,
    SDown,
    Food,
}

impl TryFrom<u8> for SnakeCell {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX: u8 = SnakeCell::Food as u8;
        if value <= MAX {
            Ok(unsafe { transmute::<u8, SnakeCell>(value) })
        } else {
            Err("value out of range")
        }
    }
}

pub const SNAKE_CELL_MAX_BITS: u16 = ((SnakeCell::Food as u8).ilog2() + 1) as u16;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SnakeDirection {
    Right,
    Up,
    Left,
    Down,
}

struct SnakeContext {
    cells: [u8; SNAKE_ARRAY_SIZE],
    head_xpos: i8,
    head_ypos: i8,
    tail_xpos: i8,
    tail_ypos: i8,
    next_dir: SnakeDirection,
    inhibit_tail_step: u8,
    occupied_cells: u16,
}

struct AppState {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    snake_ctx: SnakeContext,
    last_step: u64,
}

impl Drop for AppState {
    fn drop(&mut self) {
        unsafe {
            if !self.renderer.is_null() {
                SDL_DestroyRenderer(self.renderer);
            }
            if !self.window.is_null() {
                SDL_DestroyWindow(self.window);
            }
        }
    }
}

unsafe impl Send for AppState {}

impl AppState {
    fn new() -> Self {
        Self {
            window: null_mut(),
            renderer: null_mut(),
            snake_ctx: SnakeContext::new(),
            last_step: 0,
        }
    }
}

fn set_rect_xy(r: &mut SDL_FRect, x: i16, y: i16) {
    r.x = (x as i32 * SNAKE_BLOCK_SIZE_IN_PIXELS) as f32;
    r.y = (y as i32 * SNAKE_BLOCK_SIZE_IN_PIXELS) as f32;
}

fn wrap_around(val: &mut i8, max: i8) {
    if *val < 0 {
        *val = max - 1;
    } else if *val > max - 1 {
        *val = 0
    }
}

impl SnakeContext {
    fn new() -> Self {
        let mut ctx = Self {
            cells: [0; SNAKE_ARRAY_SIZE],
            head_xpos: SNAKE_GAME_WIDTH / 2,
            head_ypos: SNAKE_GAME_HEIGHT / 2,
            tail_xpos: SNAKE_GAME_WIDTH / 2,
            tail_ypos: SNAKE_GAME_HEIGHT / 2,
            next_dir: SnakeDirection::Right,
            inhibit_tail_step: 4,
            occupied_cells: 3,
        };
        ctx.put_cell_at(ctx.tail_xpos, ctx.tail_ypos, SnakeCell::SRight);
        for _ in 0..4 {
            ctx.new_food_pos();
            ctx.occupied_cells += 1;
        }
        ctx
    }

    fn cell_at(&self, x: i8, y: i8) -> SnakeCell {
        let (bi, adjust) = shift(x, y);
        let range = u16::from_ne_bytes([self.cells[bi], self.cells[bi + 1]]);
        ((range >> adjust) as u8 & THREE_BITS as u8)
            .try_into()
            .unwrap()
    }

    fn put_cell_at(&mut self, x: i8, y: i8, ct: SnakeCell) {
        let (bi, adjust) = shift(x, y);
        let mut range = u16::from_ne_bytes([self.cells[bi], self.cells[bi + 1]]);
        range &= !(THREE_BITS << adjust); // clear bits
        range |= (ct as u16 & THREE_BITS) << adjust;
        [self.cells[bi], self.cells[bi + 1]] = range.to_ne_bytes();
    }

    fn are_cells_full(&self) -> bool {
        self.occupied_cells == SNAKE_GAME_WIDTH as u16 * SNAKE_GAME_HEIGHT as u16
    }

    fn new_food_pos(&mut self) {
        loop {
            let (x, y) = unsafe {
                (
                    SDL_rand(SNAKE_GAME_WIDTH as i32) as i8,
                    SDL_rand(SNAKE_GAME_HEIGHT as i32) as i8,
                )
            };
            if self.cell_at(x, y) == SnakeCell::Nothing {
                self.put_cell_at(x, y, SnakeCell::Food);
                break;
            }
        }
    }

    fn redir(&mut self, dir: SnakeDirection) {
        let ct = self.cell_at(self.head_xpos, self.head_ypos);
        if (dir == SnakeDirection::Right && ct != SnakeCell::SLeft)
            || (dir == SnakeDirection::Up && ct != SnakeCell::SDown)
            || (dir == SnakeDirection::Left && ct != SnakeCell::SRight)
            || (dir == SnakeDirection::Down && ct != SnakeCell::SUp)
        {
            self.next_dir = dir
        }
    }

    fn step(&mut self) {
        let dir_as_cell: SnakeCell = (self.next_dir as u8 + 1).try_into().unwrap();
        self.inhibit_tail_step -= 1;

        // Move tail forward
        if self.inhibit_tail_step == 0 {
            self.inhibit_tail_step += 1;
            let ct = self.cell_at(self.tail_xpos, self.tail_ypos);
            self.put_cell_at(self.tail_xpos, self.tail_ypos, SnakeCell::Nothing);
            match ct {
                SnakeCell::SRight => self.tail_xpos += 1,
                SnakeCell::SUp => self.tail_ypos -= 1,
                SnakeCell::SLeft => self.tail_xpos -= 1,
                SnakeCell::SDown => self.tail_ypos += 1,
                _ => (),
            }
            wrap_around(&mut self.tail_xpos, SNAKE_GAME_WIDTH);
            wrap_around(&mut self.tail_ypos, SNAKE_GAME_HEIGHT);
        }

        // Move head forward
        let prev_xpos = self.head_xpos;
        let prev_ypos = self.head_ypos;
        match self.next_dir {
            SnakeDirection::Right => self.head_xpos += 1,
            SnakeDirection::Up => self.head_ypos -= 1,
            SnakeDirection::Left => self.head_xpos -= 1,
            SnakeDirection::Down => self.head_ypos += 1,
        }
        wrap_around(&mut self.head_xpos, SNAKE_GAME_WIDTH);
        wrap_around(&mut self.head_ypos, SNAKE_GAME_HEIGHT);

        // Collisions
        let ct = self.cell_at(self.head_xpos, self.head_ypos);
        if ct != SnakeCell::Nothing && ct != SnakeCell::Food {
            *self = Self::new();
            return;
        }
        self.put_cell_at(prev_xpos, prev_ypos, dir_as_cell);
        self.put_cell_at(self.head_xpos, self.head_ypos, dir_as_cell);
        if ct == SnakeCell::Food {
            if self.are_cells_full() {
                *self = Self::new();
                return;
            }
            self.new_food_pos();
            self.inhibit_tail_step += 1;
            self.occupied_cells += 1;
        }
    }

    fn handle_key_event(&mut self, key_code: SDL_Scancode) -> AppResult {
        match key_code {
            SDL_SCANCODE_ESCAPE | SDL_SCANCODE_Q => return AppResult::Success,
            SDL_SCANCODE_R => *self = Self::new(),
            SDL_SCANCODE_RIGHT => self.redir(SnakeDirection::Right),
            SDL_SCANCODE_UP => self.redir(SnakeDirection::Up),
            SDL_SCANCODE_LEFT => self.redir(SnakeDirection::Left),
            SDL_SCANCODE_DOWN => self.redir(SnakeDirection::Down),
            _ => (),
        }
        AppResult::Continue
    }
}

#[app_iterate]
fn app_iterate(app: &mut AppState) -> AppResult {
    let ctx = &mut app.snake_ctx;
    let mut r = SDL_FRect {
        x: 0.0,
        y: 0.0,
        w: SNAKE_BLOCK_SIZE_IN_PIXELS as f32,
        h: SNAKE_BLOCK_SIZE_IN_PIXELS as f32,
    };
    unsafe {
        let now = SDL_GetTicks();

        // run game logic if we're at or past the time to run it.
        // if we're _really_ behind the time to run it, run it
        // several times.
        while (now - app.last_step) >= STEP_RATE_IN_MILLISECONDS as u64 {
            ctx.step();
            app.last_step += STEP_RATE_IN_MILLISECONDS as u64;
        }

        SDL_SetRenderDrawColor(app.renderer, 0, 0, 0, SDL_ALPHA_OPAQUE);
        SDL_RenderClear(app.renderer);

        for j in 0..SNAKE_GAME_HEIGHT {
            for i in 0..SNAKE_GAME_WIDTH {
                let ct = ctx.cell_at(i, j);
                if ct == SnakeCell::Nothing {
                    continue;
                }
                set_rect_xy(&mut r, i as _, j as _);
                if ct == SnakeCell::Food {
                    SDL_SetRenderDrawColor(app.renderer, 80, 80, 255, SDL_ALPHA_OPAQUE);
                } else {
                    // body
                    SDL_SetRenderDrawColor(app.renderer, 0, 128, 0, SDL_ALPHA_OPAQUE);
                }
                SDL_RenderFillRect(app.renderer, &r);
            }
        }

        SDL_SetRenderDrawColor(app.renderer, 255, 255, 0, SDL_ALPHA_OPAQUE); // head
        set_rect_xy(&mut r, ctx.head_xpos as _, ctx.head_ypos as _);
        SDL_RenderFillRect(app.renderer, &r);
        SDL_RenderPresent(app.renderer);
        AppResult::Continue
    }
}

const EXTENDED_METADATA: &[(*const c_char, *const c_char)] = &[
    (
        SDL_PROP_APP_METADATA_URL_STRING,
        c"https://examples.libsdl.org/SDL3/game/01-snake/".as_ptr(),
    ),
    (SDL_PROP_APP_METADATA_CREATOR_STRING, c"SDL team".as_ptr()),
    (
        SDL_PROP_APP_METADATA_COPYRIGHT_STRING,
        c"Placed in the public domain".as_ptr(),
    ),
    (SDL_PROP_APP_METADATA_TYPE_STRING, c"game".as_ptr()),
];

#[app_init]
fn app_init() -> Option<Box<Mutex<AppState>>> {
    unsafe {
        if !SDL_SetAppMetadata(
            c"Example Snake game (Rust port)".as_ptr(),
            c"1.0".as_ptr(),
            c"com.example.Snake".as_ptr(),
        ) {
            return None;
        }

        for (key, value) in EXTENDED_METADATA.iter().copied() {
            if !SDL_SetAppMetadataProperty(key, value) {
                return None;
            }
        }

        if !SDL_Init(SDL_INIT_VIDEO) {
            return None;
        }

        let mut app = AppState::new();

        if !SDL_CreateWindowAndRenderer(
            c"examples/game/snake".as_ptr(),
            SDL_WINDOW_WIDTH,
            SDL_WINDOW_HEIGHT,
            0,
            &mut app.window,
            &mut app.renderer,
        ) {
            return None;
        }

        app.snake_ctx = SnakeContext::new();
        app.last_step = SDL_GetTicks();

        Some(Box::new(Mutex::new(app)))
    }
}

#[app_event]
fn app_event(app: &mut AppState, event: &SDL_Event) -> AppResult {
    unsafe {
        match SDL_EventType(event.r#type) {
            SDL_EVENT_QUIT => AppResult::Success,
            SDL_EVENT_KEY_DOWN => app.snake_ctx.handle_key_event(event.key.scancode),
            _ => AppResult::Continue,
        }
    }
}

#[app_quit]
fn app_quit() {}
