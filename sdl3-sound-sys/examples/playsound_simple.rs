//! Based on `playsound_simple.c` from the SDL_sound source code, originally
//! written by Ryan C. Gordon and released under the Zlib license.
//! Rust sys version by Maia S. Ravn

use core::{
    ffi::{CStr, c_char, c_int, c_void},
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};
use sdl3_sound_sys::sound::{
    Sound_Decode, Sound_FreeSample, Sound_GetError, Sound_Init, Sound_NewSampleFromFile,
    Sound_Quit, Sound_Sample, Sound_SampleFlags,
};
use sdl3_sys::{
    audio::{
        SDL_AudioDeviceID, SDL_AudioSpec, SDL_AudioStream, SDL_DestroyAudioStream,
        SDL_GetAudioDeviceFormat, SDL_GetAudioStreamDevice, SDL_OpenAudioDeviceStream,
        SDL_PauseAudioStreamDevice, SDL_PutAudioStreamData, SDL_ResumeAudioStreamDevice,
    },
    error::SDL_GetError,
    init::SDL_Quit,
    timer::SDL_Delay,
};
use std::{env, process::ExitCode};

// simple defer macro. the argument is executed at any exit from the scope this was called in
macro_rules! defer {
    ($expr:expr) => {
        let _defer = Defer(Some(move || {
            $expr;
        }));
    };
}

struct Defer<T: FnOnce()>(Option<T>);

impl<T: FnOnce()> Drop for Defer<T> {
    #[inline]
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

// global decoding state
#[derive(Default)]
struct AudioCallbackData {
    buffer: Vec<u8>,
    sample: *mut Sound_Sample,
    devformat: SDL_AudioSpec,
    decoded_ptr: *mut u8,
    decoded_bytes: u32,
}

// set to true when the audio callback has finished playing the whole file
static GLOBAL_DONE_FLAG: AtomicBool = AtomicBool::new(false);

fn sdl_error() -> String {
    unsafe { c_string(SDL_GetError()) }
}

fn sound_error() -> String {
    unsafe { c_string(Sound_GetError()) }
}

unsafe fn c_string(ptr: *const c_char) -> String {
    if !ptr.is_null() {
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    } else {
        String::new()
    }
}

// The audio callback. SDL calls this frequently to feed the audio device.
// We decode the audio file being played in here in small chunks and feed
// the device as necessary. Other solutions may want to predecode more
// (or all) of the file, since this needs to run fast and frequently,
// but since we're only sitting here and waiting for the file to play,
// the only real requirement is that we can decode a given audio file
// faster than realtime, which isn't really a problem with any modern format
// on even pretty old hardware at this point.
extern "C" fn audio_callback(
    userdata: *mut c_void,
    stream: *mut SDL_AudioStream,
    additional_amount: c_int,
    _total_amount: c_int,
) {
    if additional_amount <= 0 {
        return;
    }

    let len = additional_amount as u32;
    // we can't make a mutable reference to this because one is already active on another thread
    let data = unsafe { &mut *userdata.cast::<AudioCallbackData>() };
    if (data.buffer.len() as u32) < len {
        // because rust doesn't support arbitrary sized allocations on the stack, we instead
        // keep a persistent buffer on the heap and grow it as needed
        data.buffer.resize(len as _, 0);
    }
    let (buffer, sample, mut decoded_bytes, mut decoded_ptr) = (
        data.buffer.as_mut_ptr(),
        data.sample,
        data.decoded_bytes,
        data.decoded_ptr,
    );
    let mut bytes_written = 0;

    while bytes_written < len {
        if decoded_bytes == 0 {
            // need more data!
            if unsafe { (*sample).flags } & (Sound_SampleFlags::ERROR | Sound_SampleFlags::EOF) == 0
            {
                // if there wasn't previously an error or eof, read more
                decoded_bytes = unsafe { Sound_Decode(sample) };
                decoded_ptr = unsafe { (*sample).buffer.cast() };
            }
            if decoded_bytes == 0 {
                // there isn't more data to read; fill rest with silence and stop
                unsafe {
                    ptr::write_bytes(
                        buffer.add(bytes_written as _),
                        0,
                        (len - bytes_written) as _,
                    )
                };
                GLOBAL_DONE_FLAG.store(true, Ordering::Relaxed);
                break;
            }
        }

        // we have data decoded and ready to write to the device
        let copy_size = (len - bytes_written).min(decoded_bytes);
        if copy_size > 0 {
            // write this iteration's data to the device
            unsafe {
                buffer
                    .add(bytes_written as _)
                    .copy_from_nonoverlapping(decoded_ptr, copy_size as _)
            };
            // update state for next iteration or callback
            bytes_written += copy_size;
            decoded_ptr = unsafe { decoded_ptr.add(copy_size as _) };
            decoded_bytes -= copy_size;
        }
    }

    data.decoded_bytes = decoded_bytes;
    data.decoded_ptr = decoded_ptr;
    unsafe { SDL_PutAudioStreamData(stream, buffer.cast(), len as _) };
}

fn play_one_sound_file(filename: &str) {
    let sample = unsafe {
        Sound_NewSampleFromFile(format!("{filename}\0").as_ptr().cast(), ptr::null(), 65536)
    };
    if sample.is_null() {
        eprintln!("Couldn't load {filename}: {}", sound_error());
        return;
    }
    defer!(unsafe { Sound_FreeSample(sample) });

    // Open device in format of the the sound to be played.
    // We open and close the device for each sound file, so that SDL
    // handles the data conversion to hardware format; this is the
    // easy way out, but isn't practical for most apps. Usually you'll
    // want to pick one format for all the data or one format for the
    // audio device and convert the data when needed. This is a more
    // complex issue than I can describe in a source code comment, though.
    let devformat = unsafe { (*sample).actual };
    let mut data = AudioCallbackData {
        sample,
        devformat,
        ..Default::default()
    };

    let stream = unsafe {
        SDL_OpenAudioDeviceStream(
            SDL_AudioDeviceID::DEFAULT_PLAYBACK,
            &data.devformat,
            Some(audio_callback),
            (&mut data as *mut AudioCallbackData).cast(),
        )
    };
    if stream.is_null() {
        eprintln!("Couldn't open audio device: {}", sdl_error());
        return;
    }
    defer!(unsafe { SDL_DestroyAudioStream(stream) });

    println!("Now playing {filename}...");

    // SDL audio device is paused right after opening
    unsafe { SDL_ResumeAudioStreamDevice(stream) };

    while !GLOBAL_DONE_FLAG.load(Ordering::Acquire) {
        // wait for the audio callback to finish
        unsafe { SDL_Delay(10) };
    }

    // at this point, we've played the entire audio file, so stop the device
    unsafe { SDL_PauseAudioStreamDevice(stream) };

    // Sleep two buffers' worth of audio before closing, in order
    // to allow the playback to finish. This isn't always enough;
    // perhaps SDL needs a way to explicitly wait for device drain?
    //
    // Most apps don't have this issue, since they aren't explicitly
    // closing the device as soon as a sound file is done playback.
    //
    // As an alternative for this app, you could also change the callback
    // to write silence for a call or two before flipping global_done_flag.
    let mut spec = SDL_AudioSpec::default();
    let mut sample_frames = 0;
    unsafe {
        SDL_GetAudioDeviceFormat(
            SDL_GetAudioStreamDevice(stream),
            &mut spec,
            &mut sample_frames,
        );
        SDL_Delay((sample_frames * 1000 / spec.freq) as _);
    }

    // if there was an error, tell the user
    if unsafe { (*sample).flags } & Sound_SampleFlags::ERROR != 0 {
        eprintln!("Error decoding file: {}", sound_error());
    }
}

fn main() -> ExitCode {
    // Sound_Init calls SDL_Init(SDL_INIT_AUDIO)
    defer!(unsafe { SDL_Quit() });
    if unsafe { Sound_Init() } == 0 {
        eprintln!("Sound_Init() failed: {}", sound_error());
        return ExitCode::FAILURE;
    }
    defer!(unsafe { Sound_Quit() });

    let mut args = env::args();
    args.next(); // skip program name arg
    for arg in args {
        // use each argument as the filename for a sound file to play
        play_one_sound_file(&arg);
    }

    ExitCode::SUCCESS
}
