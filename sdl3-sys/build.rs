use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("DOCS_RS").is_ok() {
        // don't build/link SDL on docs.rs
    } else {
        #[cfg(feature = "build-from-source")]
        {
            use rpkg_config::{Link, PkgConfig};
            use std::path::PathBuf;

            if let Ok(cfg) = PkgConfig::open(&PathBuf::from_iter([
                sdl3_src::BUILD_DIR,
                "lib",
                "pkgconfig",
                "sdl3.pc",
            ])) {
                for link in cfg.libs()? {
                    match link {
                        Link::SearchLib(path) => {
                            println!("cargo::rustc-link-search=native={}", path.display())
                        }

                        Link::SearchFramework(path) => {
                            println!("cargo::rustc-link-search=framework={}", path.display())
                        }

                        Link::Lib(path) => {
                            println!("cargo::rustc-link-lib={}", path.display())
                        }

                        Link::Framework(path) => {
                            println!("cargo::rustc-link-lib=framework={}", path.display())
                        }

                        Link::WeakFramework(path) => {
                            // FIXME: rust doesn't support weak linking to frameworks for normal crates
                            println!("cargo::rustc-link-lib=framework={}", path.display())
                        }

                        _ => (),
                    }
                }
            } else if cfg!(feature = "link-framework") {
                println!("cargo::rustc-link-search=framework={}", sdl3_src::BUILD_DIR);
                println!("cargo::rustc-link-lib=framework=SDL3");
            } else {
                println!("cargo::rustc-link-search={}", sdl3_src::BUILD_DIR);
                println!("cargo::rustc-link-search={}/lib", sdl3_src::BUILD_DIR);
                println!("cargo::rustc-link-lib=SDL3");
            }
        }

        #[cfg(not(feature = "build-from-source"))]
        {
            if cfg!(feature = "link-framework") {
                // FIXME: rust doesn't support linking to xcframeworks
                let link_search = |name| {
                    println!("cargo::rustc-link-search=framework=/Library/Frameworks/SDL3.xcframework/{name}");
                };
                if cfg!(target_os = "macos") {
                    link_search("macos-arm64_x86_64");
                } else if cfg!(target_os = "ios") {
                    if cfg!(target_abi = "sim") {
                        link_search("ios-arm64_x86_64-simulator");
                    } else {
                        link_search("ios-arm64");
                    }
                } else if cfg!(target_os = "tvos") {
                    if cfg!(target_abi = "sim") {
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
                            println!("cargo::rustc-link-lib={s}");
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
                    if cfg!(feature = "link-static") {
                        println!("cargo::rustc-link-lib=static=SDL3");
                    } else {
                        println!("cargo::rustc-link-lib=SDL3");
                    }
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
