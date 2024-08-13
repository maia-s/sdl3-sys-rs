#![allow(clippy::type_complexity)]

mod emit;
mod parse;

use core::fmt::Write;
use emit::{Emit, EmitContext, EmitErr, InnerEmitContext};
use parse::{Items, Parse, ParseErr, Source, Span};
use std::{
    cell::RefCell,
    collections::HashMap,
    error,
    fmt::{self, Debug, Display},
    fs::{read_dir, DirBuilder, File},
    io::{self, BufWriter, Read},
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

pub fn generate(headers_path: &Path, output_path: &Path) -> Result<(), Error> {
    let mut gen = Gen::new(output_path.to_owned())?;

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

    let mut keys = Vec::new();
    for module in gen.parsed.keys() {
        keys.push(module.as_str());
    }
    keys.sort_unstable();
    for module in keys {
        gen.emit(module)?;
    }

    Ok(())
}

#[derive(Default)]
pub struct Gen {
    parsed: HashMap<String, Items>,
    emitted: RefCell<HashMap<String, InnerEmitContext>>,
    output_path: PathBuf,
}

impl Gen {
    pub fn new(output_path: PathBuf) -> Result<Self, Error> {
        DirBuilder::new().recursive(true).create(&output_path)?;
        Ok(Self {
            parsed: HashMap::new(),
            emitted: RefCell::new(HashMap::new()),
            output_path,
        })
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
        self.parsed.insert(module.to_owned(), items);
        Ok(())
    }

    pub fn emit(&self, module: &str) -> Result<(), Error> {
        if !self.emitted.borrow().contains_key(module) {
            if !self.parsed.contains_key(module) {
                eprintln!("skipping {module}");
                return Ok(());
            }

            let mut path = self.output_path.clone();
            path.push(module);
            path.set_extension("rs");

            struct CompleteGuard<'a>(bool, &'a Path);
            impl Drop for CompleteGuard<'_> {
                fn drop(&mut self) {
                    if !self.0 {
                        use io::Write;
                        File::options()
                            .append(true)
                            .open(&self.1)
                            .unwrap()
                            .write_all(
                                "\n\ncompile_error!(\"incomplete generated file\");\n".as_bytes(),
                            )
                            .unwrap();
                    }
                }
            }
            let mut complete_guard = CompleteGuard(false, &path);

            let mut file = Writable(BufWriter::new(File::create(&path)?));
            let mut ctx = EmitContext::new(module, &mut file, self)?;
            self.parsed[module].emit(&mut ctx)?;
            ctx.flush_ool_output()?;
            let emitted = ctx.into_inner();
            file.0.into_inner().unwrap().sync_all()?;
            self.emitted
                .borrow_mut()
                .insert(module.to_string(), emitted);

            complete_guard.0 = true;

            println!("emitted {module}");
        }
        Ok(())
    }
}

pub struct Writable<T>(T);

impl<T: io::Write> Write for Writable<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
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

impl From<Error> for EmitErr {
    fn from(value: Error) -> Self {
        match value {
            Error::IoError(e) => Self::IoError(e),
            Error::EmitError(e) => e,
            Error::ParseError(e) => unreachable!(),
        }
    }
}

fn common_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    for (i, (ca, cb)) in a.chars().zip(b.chars()).enumerate() {
        if ca != cb {
            return &a[..i];
        }
    }
    &a[..a.len().min(b.len())]
}
