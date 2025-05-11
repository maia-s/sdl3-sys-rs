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

#[derive(Clone, Copy, PartialEq, Eq)]
enum LinkKind {
    Default,
    Static,
}

#[derive(Debug)]
enum LinkFlag {
    SearchLib(String),
    SearchFramework(String),
    Lib(String),
    StaticLib(String),
    Framework(String),
    #[allow(dead_code)]
    WeakFramework(String),
}

#[derive(Default)]
struct LinkFlags(Vec<LinkFlag>);

impl LinkFlags {
    fn search_lib(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::SearchLib(s.to_string()));
    }

    fn search_framework(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::SearchFramework(s.to_string()));
    }

    fn link_lib(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::Lib(s.to_string()));
    }

    fn link_static_lib(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::StaticLib(s.to_string()));
    }

    fn link_framework(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::Framework(s.to_string()));
    }

    #[allow(dead_code)]
    fn link_weak_framework(&mut self, s: impl ToString) {
        self.0.push(LinkFlag::WeakFramework(s.to_string()));
    }

    fn send_to_cargo_link_flags(self) {
        for flag in self.0.iter() {
            match flag {
                LinkFlag::SearchLib(path) => {
                    println!("cargo::rustc-link-search=native={path}");
                }
                LinkFlag::SearchFramework(path) => {
                    println!("cargo::rustc-link-search=framework={path}");
                }
                LinkFlag::Lib(lib) => {
                    println!("cargo::rustc-link-lib={lib}");
                }
                LinkFlag::StaticLib(lib) => {
                    println!("cargo::rustc-link-lib=static={lib}");
                }
                LinkFlag::Framework(lib) | LinkFlag::WeakFramework(lib) => {
                    // FIXME: rust doesn't support weak linking to frameworks for normal crates
                    println!("cargo::rustc-link-lib=framework={lib}");
                }
            }
        }
    }

    fn send_to_cargo_metadata(self) {
        let mut search_libs = String::new();
        let mut search_frameworks = String::new();
        let mut libs = String::new();
        let mut static_libs = String::new();
        let mut frameworks = String::new();
        let mut weak_frameworks = String::new();
        let mut clang = String::new();
        let sep = ';';
        for flag in self.0.iter() {
            match flag {
                LinkFlag::SearchLib(path) => {
                    assert!(
                        !path.contains(sep),
                        "library search path contains `{sep}`: {path}"
                    );
                    if !search_libs.is_empty() {
                        search_libs.push(sep);
                    }
                    search_libs.push_str(path);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-L");
                    clang.push_str(path);
                }
                LinkFlag::SearchFramework(path) => {
                    assert!(
                        !path.contains(sep),
                        "framework search path contains `{sep}`: {path}"
                    );
                    if !search_frameworks.is_empty() {
                        search_frameworks.push(sep);
                    }
                    search_frameworks.push_str(path);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-F");
                    clang.push_str(path);
                }
                LinkFlag::Lib(lib) => {
                    assert!(!lib.contains(sep), "library name contains `{sep}`: {lib}");
                    if !libs.is_empty() {
                        libs.push(sep);
                    }
                    libs.push_str(lib);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-l");
                    clang.push_str(lib);
                }
                LinkFlag::StaticLib(lib) => {
                    assert!(
                        !lib.contains(sep),
                        "static library name contains `{sep}`: {lib}"
                    );
                    if !static_libs.is_empty() {
                        static_libs.push(sep);
                    }
                    static_libs.push_str(lib);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-Wl,-Bstatic");
                    clang.push(sep);
                    clang.push_str("-l");
                    clang.push(sep);
                    clang.push_str(lib);
                    clang.push(sep);
                    clang.push_str("-Wl,-Bdynamic");
                }
                LinkFlag::Framework(lib) => {
                    assert!(!lib.contains(sep), "framework name contains `{sep}`: {lib}");
                    if !frameworks.is_empty() {
                        frameworks.push(sep);
                    }
                    frameworks.push_str(lib);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-framework");
                    clang.push(sep);
                    clang.push_str(lib);
                }
                LinkFlag::WeakFramework(lib) => {
                    assert!(
                        !lib.contains(sep),
                        "weak framework name contains `{sep}`: {lib}"
                    );
                    if !weak_frameworks.is_empty() {
                        weak_frameworks.push(sep);
                    }
                    weak_frameworks.push_str(lib);
                    if !clang.is_empty() {
                        clang.push(sep);
                    }
                    clang.push_str("-weak_framework");
                    clang.push(sep);
                    clang.push_str(lib);
                }
            }
        }
        println!("cargo::metadata=LINK_FLAGS_SEARCH_LIBS={search_libs}");
        println!("cargo::metadata=LINK_FLAGS_SEARCH_FRAMEWORKS={search_frameworks}");
        println!("cargo::metadata=LINK_FLAGS_LIBS={libs}");
        println!("cargo::metadata=LINK_FLAGS_STATIC_LIBS={static_libs}");
        println!("cargo::metadata=LINK_FLAGS_FRAMEWORKS={frameworks}");
        println!("cargo::metadata=LINK_FLAGS_WEAK_FRAMEWORKS={weak_frameworks}");
        println!("cargo::metadata=LINK_FLAGS_CLANG={clang}");
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
                    panic!("config key `{key}` already set to `{prev}`, new value `{value}`");
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

    if cfg!(feature = "no-link") || env::var("DOCS_RS").is_ok() {
        // don't build/link with no-link feature or on docs.rs
    } else {
        let do_link = !cfg!(feature = "no-link");
        let mut link_flags = LinkFlags::default();

        let link_kind = if cfg!(feature = "link-static") {
            LinkKind::Static
        } else {
            LinkKind::Default
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
                            link_flags.search_lib(path.display());
                        }

                        Link::SearchFramework(path) => {
                            link_flags.search_framework(path.display());
                        }

                        Link::Lib(path) => {
                            if path == Path::new(lib_name) {
                                match link_kind {
                                    LinkKind::Static => {
                                        if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
                                            link_flags
                                                .link_static_lib(format!("{lib_name}-static"));
                                        } else {
                                            link_flags.link_static_lib(path.display());
                                        }
                                    }
                                    LinkKind::Default => {
                                        link_flags.link_lib(path.display());
                                    }
                                }
                            } else {
                                link_flags.link_lib(path.display());
                            }
                        }

                        Link::Framework(path) => {
                            link_flags.link_framework(path.display());
                        }

                        Link::WeakFramework(path) => {
                            link_flags.link_weak_framework(path.display());
                        }

                        _ => (),
                    };
                }
            } else if LINK_FRAMEWORK {
                link_flags.search_framework(out_dir.display());
                link_flags.link_framework(lib_name);
            } else {
                link_flags.search_lib(out_dir.display());
                link_flags.search_lib(format!("{}/lib", out_dir.display()));
                link_flags.search_lib(format!("{}/lib64", out_dir.display()));
                match link_kind {
                    LinkKind::Static => link_flags.link_static_lib(lib_name),
                    LinkKind::Default => link_flags.link_lib(lib_name),
                }
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
                let home = env::var("HOME");
                let link_search = |name| {
                    if let Ok(home) = home {
                        link_flags.search_framework(format!(
                            "{home}/Library/Frameworks/{lib_name}.xcframework/{name}"
                        ));
                    }
                    link_flags.search_framework(format!(
                        "/Library/Frameworks/{lib_name}.xcframework/{name}"
                    ));
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
                link_flags.link_framework(lib_name);
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
                            link_flags.search_lib(path.display());
                        }
                        for path in lib.framework_paths.iter() {
                            link_flags.search_framework(path.display());
                        }
                        for s in lib.libs.iter() {
                            if s == lib_name {
                                match link_kind {
                                    LinkKind::Static => link_flags.link_static_lib(s),
                                    LinkKind::Default => link_flags.link_lib(s),
                                }
                            } else {
                                link_flags.link_lib(s);
                            }
                        }
                        for s in lib.frameworks.iter() {
                            link_flags.link_framework(s);
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
                        link_flags.search_lib("/opt/homebrew/lib");
                    }
                    match link_kind {
                        LinkKind::Static => link_flags.link_static_lib(lib_name),
                        LinkKind::Default => link_flags.link_lib(lib_name),
                    }
                }
            }
        }

        if do_link {
            link_flags.send_to_cargo_link_flags();
        } else {
            link_flags.send_to_cargo_metadata();
        }
    }

    Ok(())
}
