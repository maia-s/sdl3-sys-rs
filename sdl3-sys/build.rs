use std::{env, error::Error};

#[cfg(all(windows, feature = "build-from-source", not(feature = "link-static")))]
// based on find_cargo_target_dir from sdl2-sys
fn top_level_cargo_target_dir() -> std::path::PathBuf {
    use std::path::PathBuf;
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut target = PathBuf::from(&out_dir);
    let pop = |target: &mut PathBuf| assert!(target.pop(), "malformed OUT_DIR: {:?}", out_dir);
    while !target
        .file_name()
        .unwrap()
        .to_string_lossy()
        .contains(&pkg_name)
    {
        pop(&mut target);
    }
    pop(&mut target);
    pop(&mut target);
    target
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("DOCS_RS").is_ok() {
        // don't build/link SDL on docs.rs
    } else {
        let link_kind = if cfg!(feature = "link-static") {
            "static="
        } else {
            ""
        };

        #[cfg(feature = "build-from-source")]
        {
            use cmake::Config;
            use rpkg_config::{Link, PkgConfig};
            use std::path::Path;

            let mut config = Config::new(sdl3_src::SOURCE_DIR);
            config.define("SDL_REVISION", sdl3_src::REVISION);
            if cfg!(feature = "sdl-unix-console-build") {
                config.define("SDL_UNIX_CONSOLE_BUILD", "ON");
            }
            if cfg!(feature = "link-framework") {
                config.define("SDL_FRAMEWORK", "ON");
            } else if cfg!(feature = "link-static") {
                config.define("SDL_STATIC", "ON");
            }
            let out_dir = config.build();

            if let Ok(cfg) = PkgConfig::open(&out_dir.join("lib/pkgconfig/sdl3.pc"))
                .or_else(|_| PkgConfig::open(&out_dir.join("lib64/pkgconfig/sdl3.pc")))
            {
                for link in cfg.libs_with_private(cfg!(feature = "link-static"))? {
                    match link {
                        Link::SearchLib(path) => {
                            println!("cargo::rustc-link-search=native={}", path.display())
                        }

                        Link::SearchFramework(path) => {
                            println!("cargo::rustc-link-search=framework={}", path.display())
                        }

                        Link::Lib(path) => {
                            if path == Path::new("SDL3") {
                                if cfg!(feature = "link-static")
                                    && env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc"
                                {
                                    println!("cargo::rustc-link-lib=static=SDL3-static")
                                } else {
                                    println!("cargo::rustc-link-lib={link_kind}{}", path.display())
                                }
                            } else {
                                println!("cargo::rustc-link-lib={}", path.display())
                            }
                        }

                        Link::Framework(path) => {
                            println!("cargo::rustc-link-lib=framework={}", path.display())
                        }

                        Link::WeakFramework(path) => {
                            // FIXME: rust doesn't support weak linking to frameworks for normal crates
                            println!("cargo::rustc-link-lib=framework={}", path.display())
                        }

                        _ => (),
                    };
                }
            } else if cfg!(feature = "link-framework") {
                println!("cargo::rustc-link-search=framework={}", out_dir.display());
                println!("cargo::rustc-link-lib=framework=SDL3");
            } else {
                println!("cargo::rustc-link-search={}", out_dir.display());
                println!("cargo::rustc-link-search={}/lib", out_dir.display());
                println!("cargo::rustc-link-search={}/lib64", out_dir.display());
                println!("cargo::rustc-link-lib={link_kind}SDL3");
            }

            #[cfg(all(windows, not(feature = "link-static")))]
            {
                // Windows can't find the built dll when run, so copy it to the target dir
                std::fs::copy(
                    out_dir.join("bin").join("SDL3.dll"),
                    top_level_cargo_target_dir().join("SDL3.dll"),
                )?;
            }
        }

        #[cfg(not(feature = "build-from-source"))]
        {
            if cfg!(feature = "link-framework") {
                // FIXME: rust doesn't support linking to xcframeworks
                let link_search = |name| {
                    println!("cargo::rustc-link-search=framework=/Library/Frameworks/SDL3.xcframework/{name}");
                };
                if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
                    link_search("macos-arm64_x86_64");
                } else if env::var("CARGO_CFG_TARGET_OS").unwrap() == "ios" {
                    if env::var("CARGO_CFG_TARGET_ABI").unwrap() == "sim" {
                        link_search("ios-arm64_x86_64-simulator");
                    } else {
                        link_search("ios-arm64");
                    }
                } else if env::var("CARGO_CFG_TARGET_OS").unwrap() == "tvos" {
                    if env::var("CARGO_CFG_TARGET_ABI").unwrap() == "sim" {
                        link_search("tvos-arm64_x86_64-simulator");
                    } else {
                        link_search("tvos-arm64");
                    }
                }
                println!("cargo::rustc-link-lib=framework=SDL3");
            } else {
                #[allow(unused_mut)]
                let mut handled = false;

                #[cfg(feature = "use-pkg-config")]
                if !handled {
                    if let Ok(lib) = pkg_config::Config::new()
                        .statik(cfg!(feature = "link-static"))
                        .atleast_version("3.1.3") // first abi stable version of sdl 3
                        .probe("sdl3")
                    {
                        handled = true;
                        for path in lib.link_paths.iter() {
                            println!("cargo::rustc-link-search=native={}", path.display());
                        }
                        for path in lib.framework_paths.iter() {
                            println!("cargo::rustc-link-search=framework={}", path.display());
                        }
                        for s in lib.libs.iter() {
                            if s == "SDL3" {
                                println!("cargo::rustc-link-lib={link_kind}{s}");
                            } else {
                                println!("cargo::rustc-link-lib={s}");
                            }
                        }
                        for s in lib.frameworks.iter() {
                            println!("cargo::rustc-link-lib=framework={s}")
                        }
                    }
                }

                #[cfg(feature = "use-vcpkg")]
                if !handled {
                    handled = vcpkg::find_package("sdl3").is_ok();
                }

                if !handled {
                    // yolo
                    println!("cargo::rustc-link-lib={link_kind}SDL3");
                }
            }
        }
    }

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
