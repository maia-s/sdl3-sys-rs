// Based on load-and-play.c from the SDL_mixer examples.
// Like the original, this code is public domain.

use core::{ffi::c_char, ptr};
use sdl3_main::{AppResult, MainThreadData, app_impl};
use sdl3_mixer_sys::mixer::{
    MIX_CreateMixerDevice, MIX_CreateTrack, MIX_Init, MIX_LoadAudio, MIX_PlayTrack, MIX_Quit,
    MIX_SetTrackAudio, MIX_Track, MIX_TrackPlaying,
};
use sdl3_sys::{
    audio::SDL_AudioDeviceID,
    error::SDL_GetError,
    events::{SDL_Event, SDL_EventType},
    hints::{SDL_HINT_MAIN_CALLBACK_RATE, SDL_SetHint},
    init::{SDL_Init, SDL_InitFlags, SDL_Quit, SDL_SetAppMetadata},
    log::SDL_Log,
    properties::SDL_PropertiesID,
    render::{SDL_CreateWindowAndRenderer, SDL_RenderClear, SDL_RenderPresent, SDL_Renderer},
    video::SDL_WindowFlags,
};
use std::sync::Mutex;

const MUSIC_FILENAME_CSTRPTR: *const c_char =
    concat!(env!("CARGO_MANIFEST_DIR"), "/examples/music.mp3\0")
        .as_ptr()
        .cast();

struct App {
    main: MainThreadData<AppMain>,
}

struct AppMain {
    renderer: *mut SDL_Renderer,

    // This doesn't have to be restricted to the main thread, it's just here
    // because pointers aren't Send without a wrapper
    track: *mut MIX_Track,
}

#[app_impl]
impl App {
    // This function runs once at startup.
    fn app_init() -> Option<Box<Mutex<App>>> {
        unsafe {
            SDL_SetAppMetadata(
                c"Example Load And Play (rust)".as_ptr(),
                c"1.0".as_ptr(),
                c"com.example.load-and-play.rust".as_ptr(),
            )
        };

        // this doesn't have to run very much, so give up tons of CPU time between iterations. Optional!
        unsafe { SDL_SetHint(SDL_HINT_MAIN_CALLBACK_RATE, c"5".as_ptr()) };

        // we don't need video, but we'll make a window for smooth operation.
        if !unsafe { SDL_Init(SDL_InitFlags::VIDEO) } {
            unsafe { SDL_Log(c"Couldn't initialize SDL: %s".as_ptr(), SDL_GetError()) };
            return None;
        }

        let mut window = ptr::null_mut();
        let mut renderer = ptr::null_mut();
        if !unsafe {
            SDL_CreateWindowAndRenderer(
                c"examples/basic/load-and-play".as_ptr(),
                640,
                480,
                SDL_WindowFlags::RESIZABLE,
                &mut window,
                &mut renderer,
            )
        } {
            unsafe {
                SDL_Log(
                    c"Couldn't create window/renderer: %s".as_ptr(),
                    SDL_GetError(),
                )
            };
            return None;
        }

        if !unsafe { MIX_Init() } {
            unsafe {
                SDL_Log(
                    c"Couldn't init SDL_mixer library: %s".as_ptr(),
                    SDL_GetError(),
                )
            };
            return None;
        }

        // Create a mixer on the default audio device. Don't care about the specific audio format.
        let mixer =
            unsafe { MIX_CreateMixerDevice(SDL_AudioDeviceID::DEFAULT_PLAYBACK, ptr::null()) };
        if mixer.is_null() {
            unsafe {
                SDL_Log(
                    c"Couldn't create mixer on default device: %s".as_ptr(),
                    SDL_GetError(),
                )
            };
            return None;
        }

        // load a sound file
        let audio = unsafe { MIX_LoadAudio(mixer, MUSIC_FILENAME_CSTRPTR, false) };
        if audio.is_null() {
            unsafe {
                SDL_Log(
                    c"Couldn't load %s: %s".as_ptr(),
                    MUSIC_FILENAME_CSTRPTR,
                    SDL_GetError(),
                )
            };
            return None;
        }

        // we need a track on the mixer to play the audio. Each track has audio assigned to it, and
        // all playing tracks are mixed together for the final output.
        let track = unsafe { MIX_CreateTrack(mixer) };
        if track.is_null() {
            unsafe {
                SDL_Log(
                    c"Couldn't create a mixer track: %s".as_ptr(),
                    SDL_GetError(),
                )
            };
            return None;
        }
        unsafe { MIX_SetTrackAudio(track, audio) };

        // start the audio playing!
        unsafe { MIX_PlayTrack(track, SDL_PropertiesID::new(0)) }; // no extra options this time, so a zero for the second argument.

        // we don't save `audio`; SDL_mixer will clean it up for us during MIX_Quit().

        // carry on with the program!
        Some(Box::new(Mutex::new(App {
            main: MainThreadData::assert_new(AppMain { renderer, track }),
        })))
    }

    // This function runs when a new event (mouse input, keypresses, etc) occurs.
    fn app_event(&mut self, event: &SDL_Event) -> AppResult {
        if event.event_type() == SDL_EventType::QUIT {
            AppResult::Success // end the program, reporting success to the OS.
        } else {
            AppResult::Continue // carry on with the program!
        }
    }

    // This function runs once per frame, and is the heart of the program.
    fn app_iterate(&mut self) -> AppResult {
        let main = self.main.assert_get();
        unsafe {
            SDL_RenderClear(main.renderer);
            SDL_RenderPresent(main.renderer);

            // when the track has finished playing, end the program.
            if !MIX_TrackPlaying(main.track) {
                return AppResult::Success;
            }
        }

        AppResult::Continue // carry on with the program!
    }

    // This function runs once at shutdown.
    fn app_quit(_self: Option<&mut Self>) {
        // SDL will clean up the window/renderer for us, MIX_Quit() destroys any mixer objects we made.
        unsafe {
            MIX_Quit();
            SDL_Quit();
        }
    }
}
