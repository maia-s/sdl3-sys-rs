// to edit, edit `build-common.rs` in the repo root and run generate-and-check.sh

use std::{collections::BTreeMap, env, error::Error, fs::read_to_string, sync::OnceLock};

#[cfg(feature = "build-from-source")]
type BuildConfig = cmake::Config;
#[cfg(not(feature = "build-from-source"))]
type BuildConfig = ();

#[allow(unused)]
macro_rules! cmake_vars {
    ($config:ident => $($cvar:ident),* $(,)?) => {
        $(
            let cvar = stringify!($cvar);
            if env::var_os(format!("CARGO_FEATURE_NO_{cvar}")).is_some() {
                $config.define(cvar, "OFF");
            }
            if env::var_os(format!("CARGO_FEATURE_{cvar}")).is_some() {
                $config.define(cvar, "ON");
            }
        )*
    }
}

fn config(key: &str) -> &str {
    struct Config {
        map: BTreeMap<String, String>,
    }

    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG
        .get_or_init(|| {
            let config_file = "config.txt";
            let config = read_to_string(config_file)
                .unwrap_or_else(|e| panic!("error reading {config_file}: {e}"));
            let mut map = BTreeMap::new();
            for line in config.lines() {
                let (key, value) = line
                    .split_once(":")
                    .unwrap_or_else(|| panic!("invalid config line: `{line}`"));
                let (key, value) = (key.trim(), value.trim());
                if let Some(prev) = map.insert(key.to_owned(), value.to_owned()) {
                    panic!(
                        "config key `{}` already set to `{}`, new value `{}`",
                        key, prev, value
                    );
                }
            }
            Config { map }
        })
        .map
        .get(key)
        .unwrap_or_else(|| panic!("config key {key} not set"))
}

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

#[cfg(feature = "build-from-source")]
fn find_and_output_cmake_dir_metadata(out_dir: &std::path::Path) -> Result<(), Box<dyn Error>> {
    use std::path::{Path, PathBuf};
    fn try_dir(dir: &Path) -> bool {
        if dir
            .join(format!("{}Config.cmake", config("lib_name")))
            .exists()
        {
            println!("cargo::metadata=CMAKE_DIR={}", dir.display());
            true
        } else {
            false
        }
    }
    let lib_name = config("lib_name");
    if try_dir(&out_dir.join(PathBuf::from_iter(["lib", "cmake", lib_name])))
        || try_dir(&out_dir.join(PathBuf::from_iter(["lib64", "cmake", lib_name])))
        || try_dir(&out_dir.join("cmake"))
    {
        Ok(())
    } else {
        Err(format!("cmake dir not found in {}", out_dir.display()).into())
    }
}

fn rust_version() -> Option<(usize, usize)> {
    let out = std::process::Command::new(env::var("RUSTC").unwrap())
        .arg("--version")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let out = String::from_utf8_lossy(&out.stdout);
    let out = out.strip_prefix("rustc ")?;
    let mut it = out.split('.');
    let major = it.next()?.parse().ok()?;
    let mut minor = it.next()?.parse().ok()?;
    let (micro, _) = it.next()?.split_once(' ')?;
    if let Some((_, "nightly")) = micro.split_once('-') {
        minor -= 1;
    }
    Some((major, minor))
}

#[allow(dead_code)]
fn rust_version_at_least(major: usize, minor: usize) -> bool {
    if let Some((have_major, have_minor)) = rust_version() {
        have_major > major || (have_major == major && have_minor >= minor)
    } else {
        false
    }
}

fn build(
    f: impl FnOnce(&mut BuildConfig) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let _ = &f;

    if env::var("DOCS_RS").is_ok() {
        // don't build/link on docs.rs
    } else {
        let link_kind = if cfg!(feature = "link-static") {
            "static="
        } else {
            ""
        };

        let lib_name = config("lib_name");

        #[cfg(feature = "build-from-source")]
        {
            use rpkg_config::{Link, PkgConfig};
            use std::path::Path;

            let package_name = config("package_name");

            let mut build_config = BuildConfig::new(SOURCE_DIR);
            f(&mut build_config)?;
            let out_dir = build_config.build();
            println!("cargo::metadata=OUT_DIR={}", out_dir.display());

            if let Ok(cfg) = PkgConfig::open(
                &out_dir.join(format!("lib/pkgconfig/{package_name}.pc")),
            )
            .or_else(|_| {
                PkgConfig::open(&out_dir.join(format!("lib64/pkgconfig/{package_name}.pc")))
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
                            if path == Path::new(lib_name) {
                                if cfg!(feature = "link-static")
                                    && env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc"
                                {
                                    println!("cargo::rustc-link-lib=static={lib_name}-static")
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
                println!("cargo::rustc-link-lib=framework={lib_name}");
            } else {
                println!("cargo::rustc-link-search={}", out_dir.display());
                println!("cargo::rustc-link-search={}/lib", out_dir.display());
                println!("cargo::rustc-link-search={}/lib64", out_dir.display());
                println!("cargo::rustc-link-lib={link_kind}{lib_name}");
            }

            find_and_output_cmake_dir_metadata(&out_dir)?;

            #[cfg(all(windows, not(feature = "link-static")))]
            {
                // Windows can't find the built dll when run, so copy it to the target dir
                std::fs::copy(
                    out_dir.join("bin").join(format!("{lib_name}.dll")),
                    top_level_cargo_target_dir().join(format!("{lib_name}.dll")),
                )?;
            }
        }

        #[cfg(not(feature = "build-from-source"))]
        {
            if LINK_FRAMEWORK {
                // FIXME: rust doesn't support linking to xcframeworks
                let link_search = |name| {
                    println!("cargo::rustc-link-search=framework=/Library/Frameworks/{lib_name}.xcframework/{name}");
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
                println!("cargo::rustc-link-lib=framework={lib_name}");
            } else {
                #[allow(unused_mut)]
                let mut handled = false;

                #[cfg(feature = "use-pkg-config")]
                if !handled {
                    if let Ok(lib) = pkg_config::Config::new()
                        .statik(cfg!(feature = "link-static"))
                        .atleast_version(config("lib_min_version"))
                        .probe(config("package_name"))
                    {
                        handled = true;
                        for path in lib.link_paths.iter() {
                            println!("cargo::rustc-link-search=native={}", path.display());
                        }
                        for path in lib.framework_paths.iter() {
                            println!("cargo::rustc-link-search=framework={}", path.display());
                        }
                        for s in lib.libs.iter() {
                            if s == lib_name {
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
                    handled = vcpkg::find_package(config("package_name")).is_ok();
                }

                if !handled {
                    // yolo
                    if cfg!(target_os = "macos") {
                        println!("cargo::rustc-link-search=/opt/homebrew/lib");
                    }
                    println!("cargo::rustc-link-lib={link_kind}{lib_name}");
                }
            }
        }
    }

    Ok(())
}
