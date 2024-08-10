#![allow(clippy::type_complexity)]

mod emit;
mod parse;

use core::fmt::Write;
use emit::{Emit, EmitContext, EmitErr, EmitResult};
use parse::{Items, Parse, ParseErr, Source, Span};
use std::{
    collections::HashMap,
    error,
    fmt::{self, Debug, Display},
    fs::{read_dir, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

fn skip(module: &str) -> bool {
    [
        "begin_code",
        "close_code",
        "copying",
        "endian",
        "platform_defines",
    ]
    .contains(&module)
        || module.starts_with("main")
        || module.starts_with("test")
}

pub fn generate(headers_path: &Path) -> Result<(), Error> {
    let mut gen = Gen::new();

    for entry in read_dir(headers_path)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let name_lc = name.to_ascii_lowercase();

        if let Some(module) = name_lc
            .strip_prefix("sdl_")
            .and_then(|s| s.strip_suffix(".h"))
        {
            if skip(module) {
                continue;
            }
            let mut buf = String::new();
            File::open(entry.path())?.read_to_string(&mut buf)?;
            let display_path = entry.path();
            let display_path = if let Ok(p) =
                display_path.strip_prefix(PathBuf::from_iter([env!["CARGO_MANIFEST_DIR"], ".."]))
            {
                p.to_string_lossy().to_string()
            } else {
                display_path
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            };
            gen.parse(module, display_path, buf)?;
        }
    }

    gen.emit("log")?;

    Ok(())
}

#[derive(Default)]
pub struct Gen {
    modules: HashMap<String, Items>,
}

impl Gen {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn parse(
        &mut self,
        module: &str,
        filename: String,
        contents: String,
    ) -> Result<(), ParseErr> {
        println!("parsing {filename}");
        let contents: Span = Source::new(filename, contents).into();
        let rest = contents.trim_wsc()?;
        let items = Items::try_parse_all(rest)?.unwrap_or_default();
        self.modules.insert(module.to_owned(), items);
        Ok(())
    }

    pub fn emit(&self, module: &str) -> EmitResult {
        let mut output = StringLog(String::new());
        let mut ctx = EmitContext::new(module, &mut output);
        self.modules[module].emit(&mut ctx)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct StringLog(String);

impl Write for StringLog {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print!("{s}");
        self.0.write_str(s)
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseError(ParseErr),
    EmitError(EmitErr),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => Display::fmt(e, f),
            Error::ParseError(e) => Display::fmt(e, f),
            Error::EmitError(e) => Display::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseErr> for Error {
    fn from(value: ParseErr) -> Self {
        Self::ParseError(value)
    }
}

impl From<EmitErr> for Error {
    fn from(value: EmitErr) -> Self {
        Self::EmitError(value)
    }
}
