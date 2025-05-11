#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_ttf_src::SOURCE_DIR;

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
                panic!("SDL3_ttf is currently missing a configuration option to build as a framework. You can download the official framework build from <https://github.com/libsdl-org/SDL_ttf/releases>.");
                //config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            }

            config.define("SDLTTF_SAMPLES", "OFF");

            cmake_vars! { config =>
                SDLTTF_VENDORED,
                SDLTTF_HARFBUZZ,
                SDLTTF_PLUTOSVG,
            }
        }
        Ok(())
    })?;
    Ok(())
}
