const PACKAGE_NAME: &str = "sdl3-image";
const LIB_NAME: &str = "SDL3_image";
const LIB_MIN_VERSION: &str = "3.1.0";

#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_image_src::SOURCE_DIR;

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
                panic!("SDL3_image is currently missing a configuration option to build as a framework. You can download the official framework build from <https://github.com/libsdl-org/SDL_image/releases>.");
                //config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            }

            config.define("SDLIMAGE_SAMPLES", "OFF");

            cmake_vars! { config =>
                SDLIMAGE_DEPS_SHARED,
                SDLIMAGE_VENDORED,
                SDLIMAGE_BACKEND_STB,
                SDLIMAGE_BACKEND_WIC,
                SDLIMAGE_BACKEND_IMAGEIO,
                SDLIMAGE_AVIF,
                SDLIMAGE_BMP,
                SDLIMAGE_GIF,
                SDLIMAGE_JPG,
                SDLIMAGE_JXL,
                SDLIMAGE_LBM,
                SDLIMAGE_PCX,
                SDLIMAGE_PNG,
                SDLIMAGE_PNM,
                SDLIMAGE_QOI,
                SDLIMAGE_SVG,
                SDLIMAGE_TGA,
                SDLIMAGE_TIF,
                SDLIMAGE_WEBP,
                SDLIMAGE_XCF,
                SDLIMAGE_XPM,
                SDLIMAGE_XV,
                SDLIMAGE_AVIF_SAVE,
                SDLIMAGE_JPG_SAVE,
                SDLIMAGE_PNG_SAVE,
                SDLIMAGE_AVIF_SHARED,
                SDLIMAGE_JPG_SHARED,
                SDLIMAGE_JXL_SHARED,
                SDLIMAGE_PNG_SHARED,
                SDLIMAGE_TIF_SHARED,
                SDLIMAGE_WEBP_SHARED,
            }
        }
        Ok(())
    })?;
    Ok(())
}
