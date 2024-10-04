use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("DOCS_RS").is_ok() {
        // don't build/link SDL on docs.rs
    } else {
        #[cfg(feature = "build-from-source")]
        {
            use rpkg_config::{Link, PkgConfig};

            for link in PkgConfig::open(&PathBuf::from_iter([
                sdl3_src::LIB_DIR,
                "pkgconfig",
                "sdl3.pc",
            ]))?
            .libs()?
            {
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
        }

        #[cfg(not(feature = "build-from-source"))]
        {
            if cfg!(feature = "link-static") {
                println!("cargo::rustc-link-lib=static=SDL3");
            } else if cfg!(feature = "link-framework") {
                println!("cargo::rustc-link-lib=framework=SDL3");
            } else {
                println!("cargo::rustc-link-lib=dylib=SDL3");
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
