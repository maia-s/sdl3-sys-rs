#![allow(clippy::type_complexity)]

mod emit;
mod parse;

use core::fmt::Write;
use emit::{Emit, EmitContext, EmitErr, InnerEmitContext};
use parse::{DefineValue, Ident, Items, Parse, ParseContext, ParseErr, Source, Span};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    error,
    fmt::{self, Debug, Display},
    fs::{read_dir, DirBuilder, File, OpenOptions},
    io::{self, BufWriter, Read},
    path::{Path, PathBuf},
    process::Command,
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

fn skip_emit(module: &str) -> bool {
    ["egl", "intrin", "oldnames"].contains(&module) || module.starts_with("opengl")
}

fn run_rustfmt(path: &Path) {
    let _ = Command::new("rustfmt").arg(path).spawn();
}

pub fn generate(sdl_path: &Path, target_crate_path: &Path) -> Result<(), Error> {
    let mut showrev_path = sdl_path.to_path_buf();
    showrev_path.push("build-scripts/showrev.sh");

    let mut headers_path = sdl_path.to_path_buf();
    headers_path.push("include/SDL3");

    let mut target_cargo_toml_path = target_crate_path.to_path_buf();
    target_cargo_toml_path.push("Cargo.toml");

    let mut output_path = target_crate_path.to_path_buf();
    output_path.push("src/generated");

    let revision = if let Ok(output) = Command::new(showrev_path).output() {
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(output.stdout.trim_ascii()).into_owned())
    } else {
        None
    };

    if let Some(revision) = &revision {
        let mut buf = String::new();
        File::open(&target_cargo_toml_path)?.read_to_string(&mut buf)?;
        let mut out = Writable(BufWriter::new(File::create(&target_cargo_toml_path)?));
        let mut patched = false;
        for line in buf.lines() {
            if !patched && line.starts_with("version =") && line.contains("+sdl3") {
                let Some((revision_pos, _)) = line.char_indices().rev().find(|(_, c)| *c == '+')
                else {
                    unreachable!()
                };
                patched = true;
                let pfx = &line[..=revision_pos];
                writeln!(out, "{pfx}sdl3-{revision}\"")?;
            } else {
                writeln!(out, "{}", line)?;
            }
        }
    }

    let mut gen = Gen::new(output_path.to_owned(), revision)?;

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

    gen.emit_top_level()?; // create empty mod.rs to be appended by emit
    for module in gen.parsed.keys() {
        gen.emit(module)?;
    }
    gen.emit_top_level()?; // rewrite final mod.rs in sorted order

    Ok(())
}

#[derive(Default)]
pub struct Gen {
    revision: Option<String>,
    parsed: BTreeMap<String, Items>,
    emitted: RefCell<BTreeMap<String, InnerEmitContext>>,
    skipped: RefCell<HashSet<String>>,
    output_path: PathBuf,
}

impl Gen {
    pub fn new(output_path: PathBuf, revision: Option<String>) -> Result<Self, Error> {
        DirBuilder::new().recursive(true).create(&output_path)?;
        Ok(Self {
            revision,
            parsed: BTreeMap::new(),
            emitted: RefCell::new(BTreeMap::new()),
            skipped: RefCell::new(HashSet::new()),
            output_path,
        })
    }

    pub fn parse(
        &mut self,
        module: &str,
        filename: String,
        contents: String,
    ) -> Result<(), ParseErr> {
        let contents: Span = Source::new(filename, contents).into();
        let rest = contents.trim_wsc()?;
        let ctx = ParseContext::new(Some(module.into()));
        let items = Items::try_parse_all(&ctx, rest)?.unwrap_or_default();
        self.parsed.insert(module.to_owned(), items);
        Ok(())
    }

    pub fn emit(&self, module: &str) -> Result<(), Error> {
        if !self.emitted.borrow().contains_key(module) && !self.skipped.borrow().contains(module) {
            if !self.parsed.contains_key(module) || skip_emit(module) {
                eprintln!("[sdl3-sys-gen] skipped module `{module}`");
                self.skipped.borrow_mut().insert(module.to_string());
                return Ok(());
            }

            let mut path = self.output_path.clone();
            path.push(module);
            path.set_extension("rs");

            struct CompleteGuard<'a>(bool, &'a Path, &'a str);
            impl Drop for CompleteGuard<'_> {
                fn drop(&mut self) {
                    if !self.0 {
                        eprintln!("[sdl3-sys-gen] incomplete generated module `{}`", self.2);
                        use io::Write;
                        File::options()
                            .append(true)
                            .open(self.1)
                            .unwrap()
                            .write_all(
                                "\n\ncompile_error!(\"incomplete generated file\");\n".as_bytes(),
                            )
                            .unwrap();
                    }
                }
            }
            let mut complete_guard = CompleteGuard(false, &path, module);

            let mut file = Writable(BufWriter::new(File::create(&path)?));
            let mut ctx = EmitContext::new(module, &mut file, self)?;
            if let Some(revision) = &self.revision {
                ctx.preproc_state().borrow_mut().define(
                    Ident::new_inline("SDL_VENDOR_INFO"),
                    None,
                    DefineValue::parse_expr(&format!("{revision:?}"))?,
                )?;
            }
            self.parsed[module].emit(&mut ctx)?;
            let emitted = ctx.into_inner();
            file.0.into_inner().unwrap().sync_all()?;
            self.emitted
                .borrow_mut()
                .insert(module.to_string(), emitted);

            complete_guard.0 = true;

            run_rustfmt(&path);

            let mut path = self.output_path.clone();
            path.push("mod");
            path.set_extension("rs");
            let mut file = Writable(BufWriter::new(OpenOptions::new().append(true).open(&path)?));
            writeln!(file, "pub mod {module};")?;
        }
        Ok(())
    }

    pub fn emit_top_level(&self) -> Result<(), Error> {
        let mut path = self.output_path.clone();
        path.push("mod");
        path.set_extension("rs");
        let mut file = Writable(BufWriter::new(File::create(&path)?));
        writeln!(
            file,
            concat!(
                "#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, ",
                "clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]"
            )
        )?;
        writeln!(file)?;
        for module in self.emitted.borrow().keys() {
            writeln!(file, "pub mod {module};")?;
        }
        run_rustfmt(&path);

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

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        Self::EmitError(EmitErr::Fmt(value))
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
            Error::IoError(e) => Self::Io(e),
            Error::EmitError(e) => e,
            Error::ParseError(_) => unreachable!(),
        }
    }
}

fn common_doc_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let i = 'pfx: {
        for (i, (ca, cb)) in a.chars().zip(b.chars()).enumerate() {
            if ca != cb || ca.is_alphanumeric() {
                break 'pfx i;
            }
        }
        a.len().min(b.len())
    };
    &a[..i]
}

fn common_ident_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let mut i = 'pfx: {
        for (i, (ca, cb)) in a.chars().zip(b.chars()).enumerate() {
            if ca != cb {
                break 'pfx i;
            }
        }
        a.len().min(b.len())
    };
    let bytes = a.as_bytes();
    while i > 0 {
        if bytes[i - 1] != b'_' {
            i -= 1
        } else {
            break;
        }
    }
    &a[..i]
}

const fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s.as_bytes(),
        b"abstract"
            | b"as"
            | b"async"
            | b"await"
            | b"become"
            | b"box"
            | b"break"
            | b"const"
            | b"continue"
            | b"crate"
            | b"do"
            | b"dyn"
            | b"else"
            | b"enum"
            | b"extern"
            | b"false"
            | b"final"
            | b"fn"
            | b"for"
            | b"gen"
            | b"if"
            | b"impl"
            | b"in"
            | b"let"
            | b"loop"
            | b"macro"
            | b"match"
            | b"mod"
            | b"move"
            | b"mut"
            | b"override"
            | b"priv"
            | b"pub"
            | b"ref"
            | b"return"
            | b"self"
            | b"Self"
            | b"static"
            | b"struct"
            | b"super"
            | b"trait"
            | b"true"
            | b"try"
            | b"type"
            | b"typeof"
            | b"unsafe"
            | b"unsized"
            | b"use"
            | b"virtual"
            | b"where"
            | b"while"
            | b"yield"
    )
}
