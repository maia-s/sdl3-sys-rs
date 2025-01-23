const PACKAGE_NAME: &str = "sdl3-ttf";
const LIB_NAME: &str = "SDL3_ttf";
const LIB_MIN_VERSION: &str = "3.1.0";

#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_ttf_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = false;

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    build(|config| {
        let _ = config;
        #[cfg(feature = "build-from-source")]
        {
            if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
                config.define("SDL3_DIR", sdl3_cmake_dir);
            }
            if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            }
        }
        Ok(())
    })?;
    Ok(())
}
