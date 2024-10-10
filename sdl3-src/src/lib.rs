#![doc = include_str!("../README.md")]

use cmake::Config;
use std::{env, io, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Default,
    Static,
    Framework,
}

pub fn build(kind: Kind) -> Result<PathBuf, io::Error> {
    let mut config = Config::new(PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "SDL"]));
    match kind {
        Kind::Default => (),
        Kind::Static => {
            config.define("SDL_STATIC", "ON");
        }
        Kind::Framework => {
            config.define("SDL_FRAMEWORK", "ON");
        }
    }
    config.build().canonicalize()
}
