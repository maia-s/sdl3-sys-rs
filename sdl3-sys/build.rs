#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = sdl3_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = cfg!(feature = "link-framework");

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    println!(r#"cargo::rustc-check-cfg=cfg(feature, values("-core-float"))"#);
    if rust_version_at_least(1, 84) {
        println!(r#"cargo::rustc-cfg=feature="-core-float""#);
    }

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

            if cfg!(feature = "sdl-unix-console-build") {
                config.define("SDL_UNIX_CONSOLE_BUILD", "ON");
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
