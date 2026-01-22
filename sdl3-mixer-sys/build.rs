#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_mixer_src::SOURCE_DIR;

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
                    "SDL3_mixer is currently missing a configuration option to build as a framework. You can download the official framework build from <https://github.com/libsdl-org/SDL_mixer/releases>."
                );
                //config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            }

            config.define("SDLMIXER_SAMPLES", "OFF");

            cmake_vars! { config =>
                SDLMIXER_DEPS_SHARED,
                SDLMIXER_VENDORED,
                SDLMIXER_AIFF,
                SDLMIXER_WAVE,
                SDLMIXER_VOC,
                SDLMIXER_AU,
                SDLMIXER_FLAC,
                SDLMIXER_FLAC_LIBFLAC,
                SDLMIXER_FLAC_LIBFLAC_SHARED,
                SDLMIXER_FLAC_DRFLAC,
                SDLMIXER_FLAC_GME,
                SDLMIXER_FLAC_GME_SHARED,
                SDLMIXER_MOD_XMP,
                SDLMIXER_MOD_XMP_LITE,
                SDLMIXER_MOD_XMP_SHARED,
                SDLMIXER_MP3_DRMP3,
                SDLMIXER_MP3_MPG123,
                SDLMIXER_MP3_MPG123_SHARED,
                SDLMIXER_MIDI,
                SDLMIXER_MIDI_FLUIDSYNTH,
                SDLMIXER_MIDI_FLUIDSYNTH_SHARED,
                SDLMIXER_MIDI_TIMIDITY,
                SDLMIXER_OPUS,
                SDLMIXER_OPUS_SHARED,
                SDLMIXER_VORBIS_STB,
                SDLMIXER_VORBIS_VORBISFILE,
                SDLMIXER_VORBIS_VORBISFILE_SHARED,
                SDLMIXER_VORBIS_TREMOR,
                SDLMIXER_VORBIS_TREMOR_SHARED,
                SDLMIXER_WAVPACK,
                SDLMIXER_WAVPACK_DSD,
                SDLMIXER_WAVPACK_SHARED,
            }
        }
        Ok(())
    })?;
    Ok(())
}
