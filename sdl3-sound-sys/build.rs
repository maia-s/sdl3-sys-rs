#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_sound_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = cfg!(feature = "link-framework");

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    build(|config| {
        let _ = config;
        #[cfg(feature = "build-from-source")]
        {
            if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
                config.define("SDL3_DIR", sdl3_cmake_dir);
            }

            if LINK_FRAMEWORK {
                // !!!FIXME
                panic!(
                    "SDL3_sound is currently missing a configuration option to build as a framework."
                );
                //config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("SDLSOUND_BUILD_SHARED", "OFF");
            }

            config.define("SDLSOUND_BUILD_TEST", "OFF");

            cmake_vars! { config =>
                SDLSOUND_WAV,
                SDLSOUND_AIFF,
                SDLSOUND_AU,
                SDLSOUND_VOC,
                SDLSOUND_FLAC,
                SDLSOUND_VORBIS,
                SDLSOUND_RAW,
                SDLSOUND_SHN,
                SDLSOUND_MODPLUG,
                SDLSOUND_MP3,
                SDLSOUND_MIDI,
                SDLSOUND_COREAUDIO,
            }
        }
        Ok(())
    })?;
    Ok(())
}
