use std::{
    io::{stderr, IsTerminal},
    path::PathBuf,
    process,
};

fn main() {
    let source_crate_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "..", "sdl3-src"]);
    let target_crate_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "..", "sdl3-sys"]);
    match sdl3_sys_gen::generate(&source_crate_path, &target_crate_path) {
        Ok(()) => (),
        Err(e) => {
            if stderr().is_terminal() {
                if cfg!(debug_assertions) {
                    eprintln!("{e:#?}");
                } else {
                    eprintln!("{e:#}");
                }
            } else {
                eprintln!("{e}");
            }
            process::exit(1)
        }
    }
}
