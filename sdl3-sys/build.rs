use std::env;

fn main() {
    if env::var("DOCS_RS").is_ok() {
        // don't build/link SDL on docs.rs
    } else {
        #[cfg(feature = "build-from-source")]
        {
            use std::{ffi::OsStr, path::Path};
            let search_path =
                Path::new(unsafe { OsStr::from_encoded_bytes_unchecked(sdl3_src::SEARCH_PATH) });
            println!("cargo::rustc-link-search=native={}", search_path.display());
        }
        if cfg!(feature = "link-static") {
            println!("cargo::rustc-link-lib=static=SDL3");
        } else if cfg!(feature = "link-framework") {
            todo!()
        } else {
            println!("cargo::rustc-link-lib=dylib=SDL3");
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
}
