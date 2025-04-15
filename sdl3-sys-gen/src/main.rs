use std::{
    fs::read_dir,
    io::{stderr, IsTerminal},
    path::PathBuf,
    process,
};

fn main() {
    let root = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".."]);
    let mut crates = Vec::new();
    for entry in read_dir(&root).expect("couldn't read workspace root dir") {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let name = entry.file_name();
            let name = format!("{}", name.display());
            if let Some(name) = name.strip_suffix("-sys") {
                crates.push(name.to_owned());
            }
        }
    }
    crates.sort_unstable(); // put sdl3 crate first
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
