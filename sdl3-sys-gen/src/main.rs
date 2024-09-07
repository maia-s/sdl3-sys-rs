use std::{
    io::{stderr, IsTerminal},
    path::PathBuf,
    process,
};

fn main() {
    let sdl_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "..", "SDL"]);
    let output_path = PathBuf::from_iter([
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "sdl3-sys",
        "src",
        "generated",
    ]);
    match sdl3_sys_gen::generate(&sdl_path, &output_path) {
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
