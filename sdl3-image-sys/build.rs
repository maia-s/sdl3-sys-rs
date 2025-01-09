const PACKAGE_NAME: &str = "sdl3-image";
const LIB_NAME: &str = "SDL3_image";
const LIB_MIN_VERSION: &str = "3.1.0";

#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_image_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = false;

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    build(|config| {
        let _ = config;
        #[cfg(feature = "build-from-source")]
        {
            if let Some(sdl3_out) = env::var_os("DEP_SDL3_OUT_DIR") {
                let path = std::path::PathBuf::from(sdl3_out)
                    .join(std::path::PathBuf::from_iter(["lib", "cmake", "SDL3"]));
                config.define("SDL3_DIR", path);
            }
            if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            }
        }
        Ok(())
    })?;
    Ok(())
}
