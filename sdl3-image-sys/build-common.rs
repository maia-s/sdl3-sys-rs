// to edit, edit `build-common.rs` in the repo root and run generate-and-check.sh

use std::{env, error::Error};

#[cfg(feature = "build-from-source")]
type Config = cmake::Config;
#[cfg(not(feature = "build-from-source"))]
type Config = ();

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

fn build(f: impl FnOnce(&mut Config) -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
    let _ = &f;
    let _ = PACKAGE_NAME;
    let _ = LIB_NAME;
    let _ = LIB_MIN_VERSION;

    if env::var("DOCS_RS").is_ok() {
        // don't build/link on docs.rs
    } else {
        let link_kind = if cfg!(feature = "link-static") {
            "static="
        } else {
            ""
        };

        #[cfg(feature = "build-from-source")]
        {
            use rpkg_config::{Link, PkgConfig};
            use std::path::{Path, PathBuf};

            let mut config = Config::new(SOURCE_DIR);
            f(&mut config)?;
            let out_dir = config.build();
            println!("cargo::metadata=OUT_DIR={}", out_dir.display());
            println!(
                "cargo::metadata=CMAKE_DIR={}",
                out_dir
                    .join(PathBuf::from_iter(["lib", "cmake", LIB_NAME]))
                    .display()
            );

            if let Ok(cfg) = PkgConfig::open(
                &out_dir.join(format!("lib/pkgconfig/{PACKAGE_NAME}.pc")),
            )
            .or_else(|_| {
                PkgConfig::open(&out_dir.join(format!("lib64/pkgconfig/{PACKAGE_NAME}.pc")))
            }) {
                for link in cfg.libs_with_private(cfg!(feature = "link-static"))? {
                    match link {
                        Link::SearchLib(path) => {
                            println!("cargo::rustc-link-search=native={}", path.display())
                        }

                        Link::SearchFramework(path) => {
                            println!("cargo::rustc-link-search=framework={}", path.display())
                        }

                        Link::Lib(path) => {
                            if path == Path::new(LIB_NAME) {
                                if cfg!(feature = "link-static")
                                    && env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc"
                                {
                                    println!("cargo::rustc-link-lib=static={LIB_NAME}-static")
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
            } else if LINK_FRAMEWORK {
                println!("cargo::rustc-link-search=framework={}", out_dir.display());
                println!("cargo::rustc-link-lib=framework={LIB_NAME}");
            } else {
                println!("cargo::rustc-link-search={}", out_dir.display());
                println!("cargo::rustc-link-search={}/lib", out_dir.display());
                println!("cargo::rustc-link-search={}/lib64", out_dir.display());
                println!("cargo::rustc-link-lib={link_kind}{LIB_NAME}");
            }

            #[cfg(all(windows, not(feature = "link-static")))]
            {
                // Windows can't find the built dll when run, so copy it to the target dir
                std::fs::copy(
                    out_dir.join("bin").join(format!("{LIB_NAME}.dll")),
                    top_level_cargo_target_dir().join(format!("{LIB_NAME}.dll")),
                )?;
            }
        }

        #[cfg(not(feature = "build-from-source"))]
        {
            if LINK_FRAMEWORK {
                // FIXME: rust doesn't support linking to xcframeworks
                let link_search = |name| {
                    println!("cargo::rustc-link-search=framework=/Library/Frameworks/{LIB_NAME}.xcframework/{name}");
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
                println!("cargo::rustc-link-lib=framework={LIB_NAME}");
            } else {
                #[allow(unused_mut)]
                let mut handled = false;

                #[cfg(feature = "use-pkg-config")]
                if !handled {
                    if let Ok(lib) = pkg_config::Config::new()
                        .statik(cfg!(feature = "link-static"))
                        .atleast_version(LIB_MIN_VERSION)
                        .probe(PACKAGE_NAME)
                    {
                        handled = true;
                        for path in lib.link_paths.iter() {
                            println!("cargo::rustc-link-search=native={}", path.display());
                        }
                        for path in lib.framework_paths.iter() {
                            println!("cargo::rustc-link-search=framework={}", path.display());
                        }
                        for s in lib.libs.iter() {
                            if s == LIB_NAME {
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
                    handled = vcpkg::find_package(PACKAGE_NAME).is_ok();
                }

                if !handled {
                    // yolo
                    println!("cargo::rustc-link-lib={link_kind}{LIB_NAME}");
                }
            }
        }
    }

    Ok(())
}
