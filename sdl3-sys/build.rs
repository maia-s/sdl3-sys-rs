#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = cfg!(feature = "link-framework");

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    build(|config| {
        let _ = config;
        #[cfg(feature = "build-from-source")]
        {
            config.define("SDL_REVISION", sdl3_src::REVISION);

            if LINK_FRAMEWORK {
                config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("SDL_STATIC", "ON");
            }

            cmake_vars! { config =>
                SDL_ASAN,
                SDL_CCACHE,
                SDL_GPU_DXVK,
                SDL_LIBC,
                SDL_RPATH,
                SDL_UNIX_CONSOLE_BUILD,
            }
        }
        Ok(())
    })?;

    let enabled_assert_levels = cfg!(feature = "assert-level-disabled") as usize
        + cfg!(feature = "assert-level-release") as usize
        + cfg!(feature = "assert-level-debug") as usize
        + cfg!(feature = "assert-level-paranoid") as usize;
    if enabled_assert_levels == 0 {
        if env::var("DEBUG").unwrap() == "true" {
            println!(r#"cargo::rustc-cfg=feature="assert-level-debug""#);
        } else {
            println!(r#"cargo::rustc-cfg=feature="assert-level-release""#);
        }
    }

    Ok(())
}
