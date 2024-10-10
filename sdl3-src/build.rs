use cmake::Config;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = if env::var("DOCS_RS").is_ok() {
        // don't build SDL on docs.rs
        std::path::PathBuf::new()
    } else {
        let mut config = Config::new("SDL");

        let build_static = cfg!(feature = "build-static");
        let build_framework = cfg!(feature = "build-framework");
        if build_framework {
            config.define("SDL_FRAMEWORK", "ON");
        } else if build_static {
            config.define("SDL_STATIC", "ON");
        }

        config.build().canonicalize()?
    };

    let mut out = BufWriter::new(File::create(
        Path::new(&env::var_os("OUT_DIR").unwrap()).join("config.rs"),
    )?);
    // cargo requires utf-8 paths anyway so just output this as a str
    writeln!(out, "pub const OUT_DIR: &str = {:?};", out_dir)?;
    out.flush()?;

    Ok(())
}
