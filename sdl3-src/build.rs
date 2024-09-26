use cmake::Config;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let build = if env::var("DOCS_RS").is_ok() {
        // don't build SDL on docs.rs
        std::path::PathBuf::new()
    } else {
        let mut config = Config::new("SDL");

        let build_static = cfg!(feature = "build-static");
        let build_framework = cfg!(feature = "build-framework");
        if build_static {
            config.define("SDL_STATIC", "ON");
            config.define("SDL_SHARED", "OFF");
        } else if build_framework {
            config.define("SDL_FRAMEWORK", "ON");
        } else {
            config.define("SDL_STATIC", "OFF");
            config.define("SDL_SHARED", "ON");
        }

        let mut build = config.build();
        build.push("lib");
        build.canonicalize().unwrap()
    };

    let mut out = BufWriter::new(
        File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("config.rs")).unwrap(),
    );
    write!(out, "pub const SEARCH_PATH: &[u8] = &[").unwrap();
    for byte in build.as_os_str().as_encoded_bytes() {
        write!(out, "0x{byte:02x},").unwrap();
    }
    writeln!(out, "];").unwrap();
    out.flush().unwrap();
}
