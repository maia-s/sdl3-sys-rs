#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

#[allow(unused_macros)]
macro_rules! log_debug {
    ($ctx:expr, $($tt:tt)*) => {
        $ctx.log_debug(format_args!($($tt)*)).unwrap();
    };
}

pub struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Defer<F> {
    pub const fn new(f: F) -> Self {
        Self(Some(f))
    }

    pub fn disable(&mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

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
    fs::{self, read_dir, DirBuilder, File, OpenOptions},
    io::{self, BufWriter, Read, Write as _},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

fn skip(module: &str) -> bool {
    [
        "begin_code",
        "close_code",
        "copying",
        "egl",
        "endian",
        "intrin",
        "main_impl",
        "oldnames",
        "platform_defines",
    ]
    .contains(&module)
        || module.starts_with("opengl")
        || module.starts_with("test")
}

fn skip_emit(_module: &str) -> bool {
    false
}

fn format_and_write(input: String, path: &Path) -> Result<(), Error> {
    let mut fmt = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut fmt_in = fmt.stdin.take().unwrap();
    thread::spawn(move || fmt_in.write_all(input.as_bytes()));
    let output = String::from_utf8(fmt.wait_with_output()?.stdout).unwrap();
    fs::write(path, output)?;
    Ok(())
}

struct LinesPatch<'a> {
    match_lines: &'a [&'a dyn Fn(&str) -> bool],
    apply: &'a dyn Fn(&[String]) -> String,
}

impl LinesPatch<'_> {
    fn patch(&self, s: &str) -> String {
        let mut out = String::new();
        let mut matched_lines = Vec::new();
        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            if self.match_lines[matched_lines.len()](line) {
                matched_lines.push(format!("{line}\n"));
                if matched_lines.len() == self.match_lines.len() {
                    out.push_str(&(self.apply)(&matched_lines));
                    for line in lines {
                        out.push_str(line);
                        out.push('\n');
                    }
                    return out;
                }
            } else {
                out.extend(matched_lines.drain(..));
                out.push_str(line);
                out.push('\n');
            }
        }
        out.extend(matched_lines.drain(..));
        out
    }
}

fn patch_file(path: &Path, patches: &[LinesPatch]) -> io::Result<()> {
    let mut contents = fs::read_to_string(path)?;
    for patch in patches {
        contents = patch.patch(&contents);
    }
    fs::write(path, contents)
}

pub fn generate(source_crate_path: &Path, target_crate_path: &Path) -> Result<(), Error> {
    let sdl_path = source_crate_path.join("SDL");
    let showrev_path = sdl_path.join("build-scripts/showrev.sh");
    let headers_path = sdl_path.join("include/SDL3");
    let source_cargo_toml_path = source_crate_path.join("Cargo.toml");
    let source_lib_rs_path = source_crate_path.join("src/lib.rs");
    let target_cargo_toml_path = target_crate_path.join("Cargo.toml");
    let target_readme_path = target_crate_path.join("README.md");
    let output_path = target_crate_path.join("src/generated");

    let Some(revision) = ({
        let output = Command::new(showrev_path).output()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(output.stdout.trim_ascii()).into_owned())
    }) else {
        return Err("couldn't get SDL revision".into());
    };

    let (rest, revision_hash) = revision.rsplit_once('-').unwrap();
    let (revision_tag, revision_offset) = rest.rsplit_once('-').unwrap();
    let (revision_tag_base, version) = revision_tag.rsplit_once('-').unwrap();

    let (sdl3_src_ver, sdl3_src_ver_display) = match revision_tag_base {
        "release" => {
            assert_eq!(revision_offset, "0", "off tag stable release");
            (version.to_string(), version.to_string())
        }
        "preview" => {
            if revision_offset == "0" {
                let ver = format!("{version}-{revision_tag_base}");
                (format!("{ver}-{revision_offset}"), ver)
            } else {
                let ver =
                    format!("{version}-{revision_tag_base}-{revision_offset}-{revision_hash}");
                (ver.clone(), ver)
            }
        }
        _ => return Err("unrecognized SDL tag".into()),
    };

    // match what SDL's cmake script does
    let revision = if revision_offset == "0" {
        format!("SDL3-{revision_tag}")
    } else {
        format!("SDL3-{revision_tag}-{revision_offset}-{revision_hash}")
    };

    patch_file(
        &source_cargo_toml_path,
        &[LinesPatch {
            match_lines: &[&|s| s.starts_with("version =")],
            apply: &|_| format!("version = \"{sdl3_src_ver}\"\n"),
        }],
    )?;

    patch_file(
        &source_lib_rs_path,
        &[
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const REVISION:")],
                apply: &|_| format!("pub const REVISION: &str = {revision:?};\n"),
            },
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const VERSION:")],
                apply: &|_| format!("pub const VERSION: &str = {version:?};\n"),
            },
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const REVISION_TAG:")],
                apply: &|_| format!("pub const REVISION_TAG: &str = {revision_tag:?};\n"),
            },
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const REVISION_TAG_BASE:")],
                apply: &|_| format!("pub const REVISION_TAG_BASE: &str = {revision_tag_base:?};\n"),
            },
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const REVISION_OFFSET:")],
                apply: &|_| format!("pub const REVISION_OFFSET: &str = {revision_offset:?};\n"),
            },
            LinesPatch {
                match_lines: &[&|s| s.starts_with("pub const REVISION_HASH:")],
                apply: &|_| format!("pub const REVISION_HASH: &str = {revision_hash:?};\n"),
            },
        ],
    )?;

    patch_file(
        &target_cargo_toml_path,
        &[
            LinesPatch {
                match_lines: &[&|s| s.starts_with("version =") && s.contains("+SDL")],
                apply: &|lines| {
                    let line = &lines[0];
                    let Some((revision_pos, _)) =
                        line.char_indices().rev().find(|(_, c)| *c == '+')
                    else {
                        unreachable!()
                    };
                    format!("{}{revision}\"\n", &line[..=revision_pos])
                },
            },
            LinesPatch {
                match_lines: &[&|s| s == "[build-dependencies.sdl3-src]", &|s| {
                    s.starts_with("version =")
                }],
                apply: &|lines| format!("{}version = \"{sdl3_src_ver}\"\n", lines[0]),
            },
        ],
    )?;

    patch_file(
        &target_readme_path,
        &[LinesPatch {
            match_lines: &[&|s| s.contains("bindings for SDL version ")],
            apply: &|lines| {
                let match_ = "bindings for SDL version ";
                let line = &lines[0];
                let pfx = &line[..line.find(match_).unwrap() + match_.len()];
                format!("{pfx}`{sdl3_src_ver_display}` and earlier.\n")
            },
        }],
    )?;

    let mut gen = Gen::new(headers_path.clone(), output_path.to_owned(), revision)?;

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
            let buf = gen.patch_module(module, buf);
            let display_path = entry.path();
            let display_path = if let Ok(p) =
                display_path.strip_prefix(PathBuf::from_iter([env!["CARGO_MANIFEST_DIR"], ".."]))
            {
                p.to_string_lossy().to_string()
            } else {
                display_path.to_string_lossy().to_string()
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
    revision: String,
    parsed: BTreeMap<String, Items>,
    emitted: RefCell<BTreeMap<String, InnerEmitContext>>,
    skipped: RefCell<HashSet<String>>,
    headers_path: PathBuf,
    output_path: PathBuf,
}

impl Gen {
    pub fn new(
        headers_path: PathBuf,
        output_path: PathBuf,
        revision: String,
    ) -> Result<Self, Error> {
        DirBuilder::new().recursive(true).create(&output_path)?;
        Ok(Self {
            revision,
            parsed: BTreeMap::new(),
            emitted: RefCell::new(BTreeMap::new()),
            skipped: RefCell::new(HashSet::new()),
            headers_path,
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
                        fs::write(self.1, "compile_error!(\"incomplete generated file\");\n")
                            .unwrap();
                    }
                }
            }
            let mut complete_guard = CompleteGuard(false, &path, module);

            let mut output = String::new();
            let mut ctx = EmitContext::new(module, &mut output, self)?;
            ctx.preproc_state().borrow_mut().define(
                Ident::new_inline("SDL_VENDOR_INFO"),
                None,
                DefineValue::parse_expr(&format!("{:?}", self.revision))?,
            )?;
            self.parsed[module].emit(&mut ctx)?;
            let emitted = ctx.into_inner();
            writeln!(output)?;
            writeln!(output, "#[cfg(doc)]")?;
            writeln!(output, "use crate::everything::*;")?;

            format_and_write(output, &path)?;

            complete_guard.0 = true;

            self.emitted
                .borrow_mut()
                .insert(module.to_string(), emitted);

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
        let mut output = String::new();
        writeln!(
            output,
            concat!(
                "#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, unused_parens, unused_unsafe, ",
                "unused_variables, clippy::approx_constant, clippy::double_parens, clippy::eq_op, clippy::identity_op, ",
                "clippy::missing_safety_doc, clippy::needless_bool, clippy::needless_return, clippy::nonminimal_bool, ",
                "clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]"
            )
        )?;
        writeln!(output)?;
        for module in self.emitted.borrow().keys() {
            writeln!(output, "pub mod {module};")?;
        }
        writeln!(
            output,
            "\n/// Reexports of everything from the other modules"
        )?;
        writeln!(output, "pub mod everything {{")?;
        for module in self.emitted.borrow().keys() {
            writeln!(output, "    #[doc(no_inline)]")?;
            writeln!(output, "    pub use super::{module}::*;")?;
        }
        writeln!(output, "}}")?;
        format_and_write(output, &path)?;
        Ok(())
    }

    fn patch_module(&self, module: &str, input: String) -> String {
        match module {
            "main" => LinesPatch {
                match_lines: &[&|s| s.trim_ascii() == "#include <SDL3/SDL_main_impl.h>"],
                apply: &|_| {
                    let mut main_impl = self.headers_path.clone();
                    main_impl.push("SDL_main_impl.h");
                    LinesPatch {
                        match_lines: &[
                            &|s| s.trim_ascii().starts_with("#if defined"),
                            &|s| s.trim_ascii().starts_with("int WINAPI wWinMain"),
                            &|s| s.trim_ascii().starts_with("#else"),
                            &|s| s.trim_ascii().starts_with("int WINAPI WinMain"),
                            &|s| s.trim_ascii().starts_with("#endif"),
                        ],
                        apply: &|lines| {
                            let mut patched = String::from("\n");
                            patched.push_str(&lines[1]);
                            patched.push_str("\n\n\n");
                            patched
                        },
                    }
                    .patch(fs::read_to_string(&main_impl).unwrap().as_str())
                },
            }
            .patch(input.as_str()),

            _ => input,
        }
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
    Io(io::Error),
    Parse(ParseErr),
    Emit(EmitErr),
    Other(&'static str),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => Display::fmt(e, f),
            Error::Parse(e) => Display::fmt(e, f),
            Error::Emit(e) => Display::fmt(e, f),
            Error::Other(e) => f.write_str(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        Self::Emit(EmitErr::Fmt(value))
    }
}

impl From<ParseErr> for Error {
    fn from(value: ParseErr) -> Self {
        Self::Parse(value)
    }
}

impl From<EmitErr> for Error {
    fn from(value: EmitErr) -> Self {
        Self::Emit(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Other(value)
    }
}

impl From<Error> for EmitErr {
    fn from(value: Error) -> Self {
        match value {
            Error::Io(e) => Self::Io(e),
            Error::Emit(e) => e,
            Error::Parse(_) | Error::Other(_) => unreachable!(),
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
