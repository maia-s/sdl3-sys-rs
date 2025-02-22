// Based on editbox.c/h from the SDL_ttf examples
// This is an almost direct source port; it's not idiomatic Rust
/*
  Copyright (C) 1997-2025 Sam Lantinga <slouken@libsdl.org>
  Rust port (C) 2025 Maia S. R.

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely.
*/

const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

use core::{
    ffi::{c_char, c_int},
    ptr,
};
use sdl3_sys::{
    clipboard::{SDL_GetClipboardText, SDL_SetClipboardText},
    error::SDL_GetError,
    events::{
        SDL_Event, SDL_EventType, SDL_TextEditingCandidatesEvent, SDL_TextEditingEvent,
        SDL_EVENT_KEY_DOWN, SDL_EVENT_MOUSE_BUTTON_DOWN, SDL_EVENT_MOUSE_BUTTON_UP,
        SDL_EVENT_MOUSE_MOTION, SDL_EVENT_TEXT_EDITING, SDL_EVENT_TEXT_EDITING_CANDIDATES,
        SDL_EVENT_TEXT_INPUT,
    },
    hints::{SDL_SetHint, SDL_HINT_IME_IMPLEMENTED_UI},
    keyboard::{SDL_ClearComposition, SDL_SetTextInputArea, SDL_StartTextInput, SDL_StopTextInput},
    keycode::{
        SDLK_A, SDLK_BACKSPACE, SDLK_C, SDLK_DELETE, SDLK_DOWN, SDLK_END, SDLK_ESCAPE, SDLK_HOME,
        SDLK_LEFT, SDLK_RETURN, SDLK_RIGHT, SDLK_UP, SDLK_V, SDLK_X, SDL_KMOD_CTRL,
    },
    log::SDL_Log,
    pixels::SDL_FColor,
    properties::SDL_GetPointerProperty,
    rect::{SDL_FPoint, SDL_FRect, SDL_PointInRectFloat, SDL_Rect, SDL_RectToFRect},
    render::{
        SDL_FlushRenderer, SDL_GetRenderSafeArea, SDL_GetRendererProperties,
        SDL_RenderCoordinatesToWindow, SDL_RenderFillRect, SDL_RenderRect, SDL_Renderer,
        SDL_SetRenderDrawColor, SDL_PROP_RENDERER_SURFACE_POINTER,
    },
    stdinc::{SDL_StepBackUTF8, SDL_StepUTF8, SDL_free, SDL_malloc, SDL_memcpy, SDL_strlen},
    surface::SDL_Surface,
    timer::SDL_GetTicks,
    video::SDL_Window,
};
use sdl3_ttf_sys::ttf::{
    TTF_CreateText, TTF_DeleteTextString, TTF_DestroyText, TTF_DrawRendererText,
    TTF_DrawSurfaceText, TTF_Font, TTF_GetFontHeight, TTF_GetTextColorFloat, TTF_GetTextEngine,
    TTF_GetTextSize, TTF_GetTextSubString, TTF_GetTextSubStringForLine,
    TTF_GetTextSubStringForPoint, TTF_GetTextSubStringsForRange, TTF_InsertTextString,
    TTF_SetTextColorFloat, TTF_SetTextWrapWhitespaceVisible, TTF_SetTextWrapWidth, TTF_SubString,
    TTF_Text, TTF_TextEngine, TTF_DIRECTION_RTL, TTF_SUBSTRING_DIRECTION_MASK,
    TTF_SUBSTRING_LINE_END, TTF_SUBSTRING_TEXT_END,
};

const TEST_SURFACE_ENGINE: bool = true;

pub struct EditBox {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    font: *mut TTF_Font,
    pub text: *mut TTF_Text,
    rect: SDL_FRect,
    pub has_focus: bool,

    // Cursor support
    cursor: c_int,
    cursor_length: c_int,
    cursor_visible: bool,
    last_cursor_change: u64,
    cursor_rect: SDL_FRect,

    // Highlight support
    highlighting: bool,
    highlight1: c_int,
    highlight2: c_int,

    // IME composition
    composition_start: c_int,
    composition_length: c_int,
    composition_cursor: c_int,
    composition_cursor_length: c_int,

    // IME candidates
    candidates: *mut TTF_Text,
    selected_candidate_start: c_int,
    selected_candidate_length: c_int,

    window_surface: *mut SDL_Surface,
}

impl Drop for EditBox {
    fn drop(&mut self) {
        self.clear_candidates();
        unsafe { TTF_DestroyText(self.text) };
    }
}

impl EditBox {
    fn draw_text(&mut self, text: *mut TTF_Text, x: f32, y: f32) {
        unsafe {
            if TEST_SURFACE_ENGINE && !self.window_surface.is_null() {
                // Flush the renderer so we can draw directly to the window surface
                SDL_FlushRenderer(self.renderer);
                TTF_DrawSurfaceText(text, x as i32, y as i32, self.window_surface);
                return;
            }
            TTF_DrawRendererText(text, x, y);
        }
    }

    fn get_highlight_extents(&mut self, marker: &mut c_int, length: &mut c_int) -> bool {
        if self.highlight1 >= 0 && self.highlight2 >= 0 {
            let marker1 = self.highlight1.min(self.highlight2);
            let marker2 = self.highlight1.max(self.highlight2);
            if marker2 > marker1 {
                *marker = marker1;
                *length = marker2 - marker1;
                return true;
            }
        }
        false
    }

    fn reset_composition(&mut self) {
        self.composition_start = 0;
        self.composition_length = 0;
        self.composition_cursor = 0;
        self.composition_cursor_length = 0;
    }

    fn utf8_byte_length(mut text: *const c_char, mut num_codepoints: c_int) -> c_int {
        let start = text;
        while num_codepoints > 0 {
            let ch = unsafe { SDL_StepUTF8(&mut text, ptr::null_mut()) };
            if ch == 0 {
                break;
            }
            num_codepoints -= 1;
        }
        unsafe { text.offset_from(start) as c_int }
    }

    fn handle_composition(&mut self, event: &SDL_TextEditingEvent) {
        unsafe {
            self.delete_highlight();

            if self.composition_length > 0 {
                TTF_DeleteTextString(self.text, self.composition_start, self.composition_length);
                self.reset_composition();
            }

            let length = SDL_strlen(event.text) as c_int;
            if length > 0 {
                self.composition_start = self.cursor;
                self.composition_length = length;
                TTF_InsertTextString(
                    self.text,
                    self.composition_start,
                    event.text,
                    self.composition_length as usize,
                );
                if event.start > 0 || event.length > 0 {
                    self.composition_cursor = Self::utf8_byte_length(
                        (*self.text).text.add(self.composition_start as usize),
                        event.start,
                    );
                    self.composition_cursor = Self::utf8_byte_length(
                        (*self.text)
                            .text
                            .add((self.composition_start + self.composition_cursor) as usize),
                        event.length,
                    );
                } else {
                    self.composition_cursor = length;
                    self.composition_cursor_length = 0;
                }
            }
        }
    }

    fn cancel_composition(&mut self) {
        self.reset_composition();
        unsafe { SDL_ClearComposition(self.window) };
    }

    fn draw_composition(&mut self) {
        unsafe {
            // Draw an underline under the composed text
            let renderer = self.renderer;
            let font_height = TTF_GetFontHeight(self.font);
            let substrings = TTF_GetTextSubStringsForRange(
                self.text,
                self.composition_start,
                self.composition_length,
                ptr::null_mut(),
            );
            if !substrings.is_null() {
                let mut i = 0;
                while !substrings.add(i).read().is_null() {
                    let mut rect = SDL_FRect::default();
                    SDL_RectToFRect(&(*substrings.add(i).read()).rect, &mut rect);
                    rect.x += self.rect.x;
                    rect.y += self.rect.y + font_height as f32;
                    rect.h = 1.0;
                    SDL_RenderFillRect(renderer, &rect);
                    i += 1;
                }
                SDL_free(substrings.cast());
            }

            // Thicken the underline under the active clause in the composed text
            if self.composition_cursor_length > 0 {
                let substrings = TTF_GetTextSubStringsForRange(
                    self.text,
                    self.composition_start + self.composition_cursor,
                    self.composition_cursor_length,
                    ptr::null_mut(),
                );
                if !substrings.is_null() {
                    let mut i = 0;
                    while !substrings.add(i).read().is_null() {
                        let mut rect = SDL_FRect::default();
                        SDL_RectToFRect(&(*substrings.add(i).read()).rect, &mut rect);
                        rect.x += self.rect.x;
                        rect.y += self.rect.y + font_height as f32 - 1.0;
                        rect.h = 1.0;
                        SDL_RenderFillRect(renderer, &rect);
                        i += 1;
                    }
                    SDL_free(substrings.cast());
                }
            }
        }
    }

    fn draw_composition_cursor(&mut self) {
        unsafe {
            let renderer = self.renderer;
            if self.composition_cursor_length == 0 {
                let mut cursor = TTF_SubString::default();
                if TTF_GetTextSubString(
                    self.text,
                    self.composition_start + self.composition_cursor,
                    &mut cursor,
                ) {
                    let mut rect = SDL_FRect::default();
                    SDL_RectToFRect(&cursor.rect, &mut rect);
                    if cursor.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0 {
                        rect.x += cursor.rect.w as f32;
                    }
                    rect.x += self.rect.x;
                    rect.y += self.rect.y;
                    rect.w = 1.0;
                    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0xff);
                    SDL_RenderFillRect(renderer, &rect);
                }
            }
        }
    }

    fn clear_candidates(&mut self) {
        if !self.candidates.is_null() {
            unsafe { TTF_DestroyText(self.candidates) };
            self.candidates = ptr::null_mut();
        }
        self.selected_candidate_start = 0;
        self.selected_candidate_length = 0;
    }

    fn save_candidates(&mut self, event: &SDL_TextEditingCandidatesEvent) {
        unsafe {
            self.clear_candidates();

            let horizontal = event.horizontal;
            let num_candidates = event.num_candidates;
            let selected_candidate = event.selected_candidate;

            // Calculate the length of the candidates text
            let mut length = 0;
            for i in 0..num_candidates {
                if horizontal && i > 0 {
                    length += 1;
                }

                length += SDL_strlen(event.candidates.add(i as usize).read());

                if !horizontal {
                    length += 1;
                }
            }
            if length == 0 {
                return;
            }
            length += 1; // For null terminator

            let candidate_text = SDL_malloc(length).cast::<c_char>();
            if candidate_text.is_null() {
                return;
            }

            let mut dst = candidate_text;
            for i in 0..num_candidates {
                if horizontal && i > 0 {
                    dst.write(b' ' as _);
                    dst = dst.add(1);
                }

                let length = SDL_strlen(event.candidates.add(i as usize).read()) as c_int;
                if i == selected_candidate {
                    self.selected_candidate_start = dst.offset_from(candidate_text) as c_int;
                    self.selected_candidate_length = length;
                }
                SDL_memcpy(
                    dst.cast(),
                    event.candidates.add(i as usize).read().cast(),
                    length as usize,
                );
                dst = dst.add(length as usize);

                if !horizontal {
                    dst.write(b'\n' as _);
                    dst = dst.add(1);
                }
            }
            dst.write(0);

            self.candidates =
                TTF_CreateText(TTF_GetTextEngine(self.text), self.font, candidate_text, 0);
            SDL_free(candidate_text.cast());
            if !self.candidates.is_null() {
                let mut color = SDL_FColor::default();
                TTF_GetTextColorFloat(
                    self.text,
                    &mut color.r,
                    &mut color.g,
                    &mut color.b,
                    &mut color.a,
                );
                TTF_SetTextColorFloat(self.candidates, color.r, color.g, color.b, color.a);
            } else {
                self.clear_candidates();
            }
        }
    }

    fn draw_candidates(&mut self) {
        unsafe {
            let renderer = self.renderer;

            // Position the candidate window
            let mut offset = self.composition_start;
            if self.composition_cursor_length > 0 {
                // Place the candidates at the active clause
                offset += self.composition_cursor;
            }
            let mut cursor = TTF_SubString::default();
            if !TTF_GetTextSubString(self.text, offset, &mut cursor) {
                return;
            }

            let mut safe_rect = SDL_Rect::default();
            SDL_GetRenderSafeArea(renderer, &mut safe_rect);
            let mut candidates_w = 0;
            let mut candidates_h = 0;
            TTF_GetTextSize(self.candidates, &mut candidates_w, &mut candidates_h);
            let mut candidates_rect = SDL_FRect {
                x: self.rect.x + cursor.rect.x as f32,
                y: self.rect.y + cursor.rect.y as f32 + cursor.rect.h as f32 + 2.0,
                w: 1.0 + 2.0 + candidates_w as f32 + 2.0 + 1.0,
                h: 1.0 + 2.0 + candidates_h as f32 + 2.0 + 1.0,
            };
            if candidates_rect.x + candidates_rect.w > safe_rect.w as f32 {
                candidates_rect.x = safe_rect.w as f32 - candidates_rect.w;
                if candidates_rect.x < 0.0 {
                    candidates_rect.x = 0.0;
                }
            }

            // Draw the candidate background
            SDL_SetRenderDrawColor(renderer, 0xaa, 0xaa, 0xaa, 0xff);
            SDL_RenderFillRect(renderer, &candidates_rect);
            SDL_SetRenderDrawColor(renderer, 0x00, 0x00, 0x00, 0xff);
            SDL_RenderRect(renderer, &candidates_rect);

            // Draw the candidates
            let x = candidates_rect.x + 3.0;
            let y = candidates_rect.y + 3.0;
            self.draw_text(self.candidates, x, y);

            // Underline the selected candidate
            if self.selected_candidate_length > 0 {
                let font_height = TTF_GetFontHeight(self.font);
                let substrings = TTF_GetTextSubStringsForRange(
                    self.candidates,
                    self.selected_candidate_start,
                    self.selected_candidate_length,
                    ptr::null_mut(),
                );
                if !substrings.is_null() {
                    let mut i = 0;
                    while !substrings.add(i).read().is_null() {
                        let mut rect = SDL_FRect::default();
                        SDL_RectToFRect(&(*substrings.add(i).read()).rect, &mut rect);
                        rect.x += x;
                        rect.y += y + font_height as f32;
                        rect.h = 1.0;
                        SDL_RenderFillRect(renderer, &rect);
                        i += 1;
                    }
                    SDL_free(substrings.cast());
                }
            }
        }
    }

    fn update_text_input_area(&mut self) {
        unsafe {
            // Convert the text input area and cursor into window coordinates
            let renderer = self.renderer;
            let mut window_edit_rect_min = SDL_FPoint::default();
            let mut window_edit_rect_max = SDL_FPoint::default();
            let mut window_cursor = SDL_FPoint::default();
            if !SDL_RenderCoordinatesToWindow(
                renderer,
                self.rect.x,
                self.rect.y,
                &mut window_edit_rect_min.x,
                &mut window_edit_rect_min.y,
            ) || !SDL_RenderCoordinatesToWindow(
                renderer,
                self.rect.x + self.rect.w,
                self.rect.y + self.rect.h,
                &mut window_edit_rect_max.x,
                &mut window_edit_rect_max.y,
            ) || !SDL_RenderCoordinatesToWindow(
                renderer,
                self.cursor_rect.x,
                self.cursor_rect.y,
                &mut window_cursor.x,
                &mut window_cursor.y,
            ) {
                return;
            }

            let rect = SDL_Rect {
                x: window_edit_rect_min.x as _,
                y: window_edit_rect_min.y as _,
                w: (window_edit_rect_max.x - window_edit_rect_min.x) as _,
                h: (window_edit_rect_max.y - window_edit_rect_min.y) as _,
            };
            let cursor_offset = (window_cursor.x - window_edit_rect_min.x) as _;
            SDL_SetTextInputArea(self.window, &rect, cursor_offset);
        }
    }

    fn draw_cursor(&mut self) {
        if self.composition_length > 0 {
            self.draw_composition_cursor();
            return;
        }

        let renderer = self.renderer;
        unsafe {
            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0xff);
            SDL_RenderFillRect(renderer, &self.cursor_rect);
        }
    }

    pub fn new(
        window: *mut SDL_Window,
        renderer: *mut SDL_Renderer,
        engine: *mut TTF_TextEngine,
        font: *mut TTF_Font,
        rect: &SDL_FRect,
    ) -> Self {
        let text = unsafe { TTF_CreateText(engine, font, ptr::null_mut(), 0) };
        assert!(!text.is_null());
        let mut edit = Self {
            window,
            renderer,
            font,
            text,
            rect: *rect,
            highlight1: -1,
            highlight2: -1,

            has_focus: false,
            cursor: 0,
            cursor_length: 0,
            cursor_visible: false,
            last_cursor_change: 0,
            cursor_rect: SDL_FRect::default(),
            highlighting: false,
            composition_start: 0,
            composition_length: 0,
            composition_cursor: 0,
            composition_cursor_length: 0,
            candidates: ptr::null_mut(),
            selected_candidate_start: 0,
            selected_candidate_length: 0,
            window_surface: ptr::null_mut(),
        };
        unsafe {
            // Wrap the editbox text within the editbox area
            TTF_SetTextWrapWidth(edit.text, rect.w as _);

            // Show the whitespace when wrapping, so it can be edited
            TTF_SetTextWrapWhitespaceVisible(edit.text, true);

            if TEST_SURFACE_ENGINE {
                // Grab the window surface if we want to test the surface text engine.
                // This isn't strictly necessary, we can still use the renderer if it's
                // a software renderer targeting an SDL_Surface.
                edit.window_surface = SDL_GetPointerProperty(
                    SDL_GetRendererProperties(renderer),
                    SDL_PROP_RENDERER_SURFACE_POINTER,
                    ptr::null_mut(),
                )
                .cast();
            }

            // We support rendering the composition and candidates
            SDL_SetHint(
                SDL_HINT_IME_IMPLEMENTED_UI,
                c"composition,candidates".as_ptr(),
            );
        }

        edit
    }

    pub fn set_focus(&mut self, focus: bool) {
        if self.has_focus == focus {
            return;
        }

        self.has_focus = focus;

        unsafe {
            if self.has_focus {
                SDL_StartTextInput(self.window);
            } else {
                SDL_StopTextInput(self.window);
            }
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            let renderer = self.renderer;
            let x = self.rect.x;
            let y = self.rect.y;

            // Draw any highlight
            let mut marker = 0;
            let mut length = 0;
            if self.get_highlight_extents(&mut marker, &mut length) {
                let highlights =
                    TTF_GetTextSubStringsForRange(self.text, marker, length, ptr::null_mut());
                if !highlights.is_null() {
                    SDL_SetRenderDrawColor(renderer, 0xee, 0xee, 0x00, 0xff);
                    let mut i = 0;
                    while !highlights.add(i).read().is_null() {
                        let mut rect = SDL_FRect::default();
                        SDL_RectToFRect(&(*highlights.add(i).read()).rect, &mut rect);
                        rect.x += x;
                        rect.y += y;
                        SDL_RenderFillRect(renderer, &rect);
                        i += 1;
                    }
                    SDL_free(highlights.cast());
                }
            }

            self.draw_text(self.text, x, y);

            if self.has_focus {
                // Draw the cursor
                let now = SDL_GetTicks();
                if now - self.last_cursor_change >= CURSOR_BLINK_INTERVAL_MS {
                    self.cursor_visible = !self.cursor_visible;
                    self.last_cursor_change = now;
                }

                // Calculate the cursor rect, used for positioning candidates
                let mut cursor = TTF_SubString::default();
                if TTF_GetTextSubString(self.text, self.cursor, &mut cursor) {
                    let mut cursor_rect = SDL_FRect::default();
                    SDL_RectToFRect(&cursor.rect, &mut cursor_rect);
                    if cursor.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0 {
                        cursor_rect.x += cursor.rect.w as f32;
                    }
                    cursor_rect.x += self.rect.x;
                    cursor_rect.y += self.rect.y;
                    cursor_rect.w = 1.0;
                    self.cursor_rect = cursor_rect;

                    self.update_text_input_area();
                }

                if self.composition_length > 0 {
                    self.draw_composition();
                }

                if !self.candidates.is_null() {
                    self.draw_candidates();
                }

                if self.cursor_visible {
                    self.draw_cursor();
                }
            }
        }
    }

    fn get_cursor_text_index(x: c_int, substring: &TTF_SubString) -> c_int {
        if substring.flags & (TTF_SUBSTRING_LINE_END | TTF_SUBSTRING_TEXT_END) != 0 {
            return substring.offset;
        }

        let round_down = if substring.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0 {
            x > substring.rect.x + substring.rect.w / 2
        } else {
            x < substring.rect.x + substring.rect.w / 2
        };
        if round_down {
            // Start the cursor before the selected text
            substring.offset
        } else {
            // Place the cursor after the selected text
            substring.offset + substring.length
        }
    }

    fn set_cursor_position(&mut self, position: c_int) {
        if self.composition_length > 0 {
            // Don't let the cursor be moved into the composition
            if position >= self.composition_start
                && position <= self.composition_start + self.composition_length
            {
                return;
            }

            self.cancel_composition();
        }

        self.cursor = position;
    }

    fn move_cursor_index(&mut self, direction: c_int) {
        let mut substring = TTF_SubString::default();

        if direction < 0 {
            if unsafe { TTF_GetTextSubString(self.text, self.cursor - 1, &mut substring) } {
                self.set_cursor_position(substring.offset);
            }
        } else if unsafe {
            TTF_GetTextSubString(self.text, self.cursor, &mut substring)
                && TTF_GetTextSubString(
                    self.text,
                    substring.offset + substring.length.max(1),
                    &mut substring,
                )
        } {
            self.set_cursor_position(substring.offset);
        }
    }

    pub fn move_cursor_left(&mut self) {
        let mut substring = TTF_SubString::default();
        if unsafe { TTF_GetTextSubString(self.text, self.cursor, &mut substring) }
            && substring.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0
        {
            self.move_cursor_index(1);
        } else {
            self.move_cursor_index(-1);
        }
    }

    pub fn move_cursor_right(&mut self) {
        let mut substring = TTF_SubString::default();
        if unsafe { TTF_GetTextSubString(self.text, self.cursor, &mut substring) }
            && substring.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0
        {
            self.move_cursor_index(-1);
        } else {
            self.move_cursor_index(1);
        }
    }

    pub fn move_cursor_up(&mut self) {
        unsafe {
            let mut substring = TTF_SubString::default();
            if TTF_GetTextSubString(self.text, self.cursor, &mut substring) {
                let font_height = TTF_GetFontHeight(self.font);
                let x = if substring.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0 {
                    substring.rect.x + substring.rect.w - 1
                } else {
                    substring.rect.x
                };
                let y = substring.rect.y - font_height / 2;
                if TTF_GetTextSubStringForPoint(self.text, x, y, &mut substring) {
                    self.set_cursor_position(Self::get_cursor_text_index(x, &substring));
                }
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        unsafe {
            let mut substring = TTF_SubString::default();
            if TTF_GetTextSubString(self.text, self.cursor, &mut substring) {
                let font_height = TTF_GetFontHeight(self.font);
                let x = if substring.flags & TTF_SUBSTRING_DIRECTION_MASK == TTF_DIRECTION_RTL.0 {
                    substring.rect.x + substring.rect.w - 1
                } else {
                    substring.rect.x
                };
                let y = substring.rect.y + substring.rect.h + font_height / 2;
                if TTF_GetTextSubStringForPoint(self.text, x, y, &mut substring) {
                    self.set_cursor_position(Self::get_cursor_text_index(x, &substring));
                }
            }
        }
    }

    pub fn move_cursor_beginning_of_line(&mut self) {
        unsafe {
            let mut substring = TTF_SubString::default();
            if TTF_GetTextSubString(self.text, self.cursor, &mut substring)
                && TTF_GetTextSubStringForLine(self.text, substring.line_index, &mut substring)
            {
                self.set_cursor_position(substring.offset);
            }
        }
    }

    pub fn move_cursor_end_of_line(&mut self) {
        unsafe {
            let mut substring = TTF_SubString::default();
            if TTF_GetTextSubString(self.text, self.cursor, &mut substring)
                && TTF_GetTextSubStringForLine(self.text, substring.line_index, &mut substring)
            {
                self.set_cursor_position(substring.offset + substring.length);
            }
        }
    }

    pub fn move_cursor_beginning(&mut self) {
        self.set_cursor_position(0);
    }

    pub fn move_cursor_end(&mut self) {
        let text = unsafe { (*self.text).text };
        if !text.is_null() {
            self.set_cursor_position(unsafe { SDL_strlen(text) } as _);
        }
    }

    pub fn backspace(&mut self) {
        unsafe {
            let text = (*self.text).text;
            if text.is_null() || self.delete_highlight() {
                return;
            }

            if self.cursor > 0 {
                let start = text.cast_const().add(self.cursor as _);
                let mut next = start;
                SDL_StepBackUTF8(text, &mut next);
                let length = start.offset_from(next) as _;
                TTF_DeleteTextString(self.text, self.cursor - length, length);
                self.cursor -= length;
            }
        }
    }

    pub fn backspace_to_beginning(&mut self) {
        // Delete to the beginning of the string
        unsafe { TTF_DeleteTextString(self.text, 0, self.cursor) };
        self.set_cursor_position(0);
    }

    pub fn delete_to_end(&mut self) {
        // Delete to the end of the string
        unsafe { TTF_DeleteTextString(self.text, self.cursor, -1) };
    }

    pub fn delete(&mut self) {
        unsafe {
            let text = (*self.text).text;
            if text.is_null() || self.delete_highlight() {
                return;
            }

            let start = text.cast_const().add(self.cursor as _);
            let mut next = start;
            let mut length = SDL_strlen(next);
            SDL_StepUTF8(&mut next, &mut length);
            let length = next.offset_from(start) as _;
            TTF_DeleteTextString(self.text, self.cursor, length);
        }
    }

    fn handle_mouse_down(&mut self, x: f32, y: f32) -> bool {
        let pt = SDL_FPoint { x, y };
        if !unsafe { SDL_PointInRectFloat(&pt, &self.rect) } {
            if self.has_focus {
                self.set_focus(false);
                return true;
            }
            return false;
        }

        if !self.has_focus {
            self.set_focus(true);
        }

        // Set the cursor position
        let mut substring = TTF_SubString::default();
        let text_x = (x - self.rect.x) as c_int;
        let text_y = (y - self.rect.y) as c_int;
        if !unsafe { TTF_GetTextSubStringForPoint(self.text, text_x, text_y, &mut substring) } {
            unsafe { SDL_Log(c"Couldn't get cursor location: %s".as_ptr(), SDL_GetError()) };
            return false;
        }

        self.set_cursor_position(Self::get_cursor_text_index(text_x, &substring));
        self.highlighting = true;
        self.highlight1 = self.cursor;
        self.highlight2 = -1;

        true
    }

    fn handle_mouse_motion(&mut self, x: f32, y: f32) -> bool {
        if !self.highlighting {
            return false;
        }

        // Set the highlight position
        let mut substring = TTF_SubString::default();
        let text_x = (x - self.rect.x) as c_int;
        let text_y = (y - self.rect.y) as c_int;
        if !unsafe { TTF_GetTextSubStringForPoint(self.text, text_x, text_y, &mut substring) } {
            unsafe { SDL_Log(c"Couldn't get cursor location: %s".as_ptr(), SDL_GetError()) };
            return false;
        }

        self.set_cursor_position(Self::get_cursor_text_index(text_x, &substring));
        self.highlight2 = self.cursor;

        true
    }

    fn handle_mouse_up(&mut self, _x: f32, _y: f32) -> bool {
        if !self.highlighting {
            return false;
        }

        self.highlighting = false;
        true
    }

    pub fn select_all(&mut self) {
        let text = unsafe { (*self.text).text };
        if text.is_null() {
            return;
        }

        self.highlight1 = 0;
        self.highlight2 = unsafe { SDL_strlen(text) } as _;
    }

    pub fn delete_highlight(&mut self) -> bool {
        let text = unsafe { (*self.text).text };
        if text.is_null() {
            return false;
        }

        let mut marker = 0;
        let mut length = 0;
        if self.get_highlight_extents(&mut marker, &mut length) {
            unsafe { TTF_DeleteTextString(self.text, marker, length) };
            self.set_cursor_position(marker);
            self.highlight1 = -1;
            self.highlight2 = -1;
            return true;
        }
        false
    }

    fn copy_cut(&mut self, cut: bool) {
        let text = unsafe { (*self.text).text };
        if text.is_null() {
            return;
        }

        let mut marker = 0;
        let mut length = 0;
        unsafe {
            if self.get_highlight_extents(&mut marker, &mut length) {
                let temp = SDL_malloc(length as usize + 1).cast::<c_char>();
                if !temp.is_null() {
                    SDL_memcpy(temp.cast(), text.add(marker as _).cast(), length as _);
                    temp.add(length as _).write(0);
                    SDL_SetClipboardText(temp);
                    SDL_free(temp.cast());
                }
                if cut {
                    TTF_DeleteTextString(self.text, marker, length);
                    self.set_cursor_position(marker);
                    self.highlight1 = -1;
                    self.highlight2 = -1
                }
            } else {
                SDL_SetClipboardText(text);
                if cut {
                    TTF_DeleteTextString(self.text, 0, -1);
                }
            }
        }
    }

    pub fn copy(&mut self) {
        self.copy_cut(false);
    }

    pub fn cut(&mut self) {
        self.copy_cut(true);
    }

    pub fn paste(&mut self) {
        let text = unsafe { SDL_GetClipboardText() };
        self.insert(text);
    }

    pub fn insert(&mut self, text: *const c_char) {
        if text.is_null() {
            return;
        }

        self.delete_highlight();

        if self.composition_length > 0 {
            unsafe {
                TTF_DeleteTextString(self.text, self.composition_start, self.composition_length)
            };
            self.composition_length = 0;
        }

        let length = unsafe { SDL_strlen(text) };
        unsafe { TTF_InsertTextString(self.text, self.cursor, text, length) };
        self.set_cursor_position(self.cursor + length as c_int);
    }

    pub fn handle_event(&mut self, event: &SDL_Event) -> bool {
        unsafe {
            match SDL_EventType(event.r#type) {
                SDL_EVENT_MOUSE_BUTTON_DOWN => {
                    return self.handle_mouse_down(event.button.x, event.button.y)
                }
                SDL_EVENT_MOUSE_MOTION => {
                    return self.handle_mouse_motion(event.motion.x, event.motion.y)
                }
                SDL_EVENT_MOUSE_BUTTON_UP => {
                    return self.handle_mouse_up(event.button.x, event.button.y)
                }
                SDL_EVENT_KEY_DOWN => {
                    if self.has_focus {
                        match event.key.key {
                            SDLK_A => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.select_all();
                                }
                            }
                            SDLK_C => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.copy();
                                }
                            }
                            SDLK_V => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.paste();
                                }
                            }
                            SDLK_X => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.cut();
                                }
                            }
                            SDLK_LEFT => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.move_cursor_beginning_of_line();
                                } else {
                                    self.move_cursor_left();
                                }
                            }
                            SDLK_RIGHT => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.move_cursor_end_of_line();
                                } else {
                                    self.move_cursor_right();
                                }
                            }
                            SDLK_UP => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.move_cursor_beginning();
                                } else {
                                    self.move_cursor_up();
                                }
                            }
                            SDLK_DOWN => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.move_cursor_end();
                                } else {
                                    self.move_cursor_down();
                                }
                            }
                            SDLK_HOME => self.move_cursor_beginning(),
                            SDLK_END => self.move_cursor_end(),
                            SDLK_BACKSPACE => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.backspace_to_beginning();
                                } else {
                                    self.backspace();
                                }
                            }
                            SDLK_DELETE => {
                                if event.key.r#mod & SDL_KMOD_CTRL != 0 {
                                    self.delete_to_end();
                                } else {
                                    self.delete();
                                }
                            }
                            SDLK_RETURN => self.insert(c"\n".as_ptr()),
                            SDLK_ESCAPE => self.set_focus(false),
                            _ => (),
                        }
                        return true;
                    }
                }
                SDL_EVENT_TEXT_INPUT => {
                    self.insert(event.text.text);
                    return true;
                }
                SDL_EVENT_TEXT_EDITING => {
                    self.handle_composition(&event.edit);
                }
                SDL_EVENT_TEXT_EDITING_CANDIDATES => {
                    self.clear_candidates();
                    self.save_candidates(&event.edit_candidates);
                }
                _ => (),
            }
            false
        }
    }
}
