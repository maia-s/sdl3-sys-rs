use std::{
    io::{stderr, IsTerminal},
    path::PathBuf,
    process,
};

fn main() {
    let root = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".."]);
    let crates = ["sdl3", "sdl3-image", "sdl3-ttf"];
    match sdl3_sys_gen::generate(&root, &crates) {
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
