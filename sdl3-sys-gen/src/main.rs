use std::{
    env,
    fs::read_dir,
    io::{IsTerminal, stderr},
    path::PathBuf,
    process,
};

fn main() {
    let root = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".."]);
    let mut crates: Vec<String> = env::args()
        .filter_map(|s| s.strip_suffix("-sys").map(|s| s.to_owned()))
        .collect();
    if crates.is_empty() {
        for entry in read_dir(&root).expect("couldn't read workspace root dir") {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if let Some(name) = name.strip_suffix("-sys") {
                    crates.push(name.to_owned());
                }
            }
        }
    } else if !crates.contains(&String::from("sdl3")) {
        crates.push("sdl3".into());
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
