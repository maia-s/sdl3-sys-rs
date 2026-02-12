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
use parse::{DefineValue, EnumKind, Ident, Items, Parse, ParseContext, ParseErr, Source, Span};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    env::{current_dir, set_current_dir},
    error,
    fmt::{self, Debug, Display},
    fs::{self, DirBuilder, File, OpenOptions, create_dir, read_dir},
    io::{self, BufWriter, Read, Write as _},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};
use str_block::str_block;

fn skip(module: &str) -> bool {
    [
        "begin_code",
        "close_code",
        "copying",
        "dlopennote",
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

fn create_dir_r(dir: &Path) -> Result<(), Error> {
    if !dir.exists() {
        if let Some(parent) = dir.parent() {
            create_dir_r(parent)?;
        }
        create_dir(dir)?;
    }
    Ok(())
}

fn format_and_write(input: String, path: &Path) -> Result<(), Error> {
    create_dir_r(path.parent().unwrap())?;
    let mut fmt = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut fmt_in = fmt.stdin.take().unwrap();
    thread::spawn(move || fmt_in.write_all(input.as_bytes()));
    let output = String::from_utf8(fmt.wait_with_output()?.stdout).unwrap();
    fs::write(path, output)?;
    Ok(())
}

#[derive(Default)]
pub struct Metadata {
    hints: Vec<HintMetadata>,
    properties: Vec<PropertyMetadata>,
    groups: Vec<GroupMetadata>,
}

pub struct HintMetadata {
    name: String,
    doc: String,
}

pub struct PropertyMetadata {
    name: String,
    doc: String,
}

pub struct GroupMetadata {
    kind: EnumKind,
    name: String,
    doc: String,
    values: Vec<GroupValueMetadata>,
}

pub struct GroupValueMetadata {
    name: String,
    short_name: String,
    doc: String,
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

fn read_to_string(path: &Path) -> Result<String, Error> {
    Ok(fs::read_to_string(path)
        .map_err(|e| format!("error reading `{}`: {}", path.display(), e))?)
}

fn patch_file(path: &Path, patches: &[LinesPatch]) -> Result<(), Error> {
    let mut contents = read_to_string(path)?;
    for patch in patches {
        contents = patch.patch(&contents);
    }
    fs::write(path, contents)?;
    Ok(())
}

struct Crate {
    name: String,
    root_path: PathBuf,
    config: BTreeMap<String, String>,
}

impl Crate {
    fn new(root: &Path, name: impl ToString) -> Self {
        let name = name.to_string();
        let root_path = root.join(&name);
        let config_file = root_path.join("config.txt");
        let mut config = BTreeMap::new();
        if let Ok(config_src) = fs::read_to_string(&config_file) {
            for line in config_src.lines() {
                let (key, value) = line
                    .split_once(":")
                    .unwrap_or_else(|| panic!("invalid config line: `{line}`"));
                let (key, value) = (key.trim(), value.trim());
                if let Some(prev) = config.insert(key.to_owned(), value.to_owned()) {
                    panic!("config key `{key}` already set to `{prev}`, new value `{value}`");
                }
            }
        }
        Self {
            name,
            root_path,
            config,
        }
    }

    fn config(&self, key: &str) -> &str {
        self.config
            .get(key)
            .unwrap_or_else(|| panic!("config key {key} not set"))
    }

    fn cargo_toml_path(&self) -> PathBuf {
        self.root_path.join("Cargo.toml")
    }

    fn lib_rs_path(&self) -> PathBuf {
        self.root_path.join("src/lib.rs")
    }

    fn readme_path(&self) -> PathBuf {
        self.root_path.join("README.md")
    }

    fn generated_path(&self) -> PathBuf {
        self.root_path.join("src/generated")
    }
}

struct Library {
    lib_name: String,
    src_crate: Crate,
    sys_crate: Crate,
    revision: String,
}

impl Library {
    fn new(root: &Path, name: &str) -> Result<Self, Error> {
        let name = name.to_string();
        let src_crate = Crate::new(root, format!("{name}-src"));
        let sys_crate = Crate::new(root, format!("{name}-sys"));
        let lib_name = sys_crate.config("lib_name");
        let lib_rev_name = sys_crate.config("lib_rev_name");
        let lib_rev_name_meta = lib_rev_name.replace('_', "-"); // version metadata can't have `_`
        let lib_dir = sys_crate.config("lib_dir");
        let include_dir = sys_crate.config("include_dir");
        let version_header = src_crate
            .root_path
            .join(lib_dir)
            .join(include_dir)
            .join(sys_crate.config("version_header"));
        let define_prefix = sys_crate.config("define_prefix");

        let mut version_major = None;
        let mut version_minor = None;
        let mut version_micro = None;
        for line in read_to_string(&version_header)?.lines() {
            if let Some(line) = line.trim_ascii_start().strip_prefix("#define") {
                if let Some(line) = line.trim_ascii_start().strip_prefix(define_prefix) {
                    if let Some(line) = line.strip_prefix("MAJOR_VERSION") {
                        version_major = Some(
                            line.trim_ascii()
                                .parse::<usize>()
                                .map_err(|e| format!("couldn't parse major version: {e}"))?,
                        );
                    } else if let Some(line) = line.strip_prefix("MINOR_VERSION") {
                        version_minor = Some(
                            line.trim_ascii()
                                .parse::<usize>()
                                .map_err(|e| format!("couldn't parse minor version: {e}"))?,
                        );
                    } else if let Some(line) = line.strip_prefix("MICRO_VERSION") {
                        version_micro = Some(
                            line.trim_ascii()
                                .parse::<usize>()
                                .map_err(|e| format!("couldn't parse micro version: {e}"))?,
                        );
                    }
                    if version_major.is_some() && version_minor.is_some() && version_micro.is_some()
                    {
                        break;
                    }
                }
            }
        }
        let version = format!(
            "{}.{}.{}",
            version_major.unwrap(),
            version_minor.unwrap(),
            version_micro.unwrap()
        );

        let git_describe = {
            let cwd = current_dir()?;
            set_current_dir(src_crate.root_path.join(lib_dir))
                .map_err(|e| format!("error setting cwd to `{lib_dir}`: {e}"))?;
            let _cwd = Defer::new(|| set_current_dir(cwd).unwrap());
            Command::new("git")
                .args(["describe", "--tags", "--long"])
                .output()?
        };
        let git_describe = if git_describe.status.success() {
            String::from_utf8_lossy(&git_describe.stdout)
                .trim()
                .to_owned()
        } else {
            let git_rev = {
                let cwd = current_dir()?;
                set_current_dir(src_crate.root_path.join(lib_dir))
                    .map_err(|e| format!("error setting cwd to `{lib_dir}`: {e}"))?;
                let _cwd = Defer::new(|| set_current_dir(cwd).unwrap());
                Command::new("git")
                    .args(["rev-parse", "--short", "HEAD"])
                    .output()?
            };
            if !git_rev.status.success() {
                return Err("couldn't get git revision".into());
            }
            format!(
                "v0.0.0-none-g{}",
                String::from_utf8_lossy(&git_rev.stdout).trim()
            )
        };

        let (rest, revision_hash) = git_describe.rsplit_once('-').unwrap();
        let (revision_tag, revision_offset) = rest.rsplit_once('-').unwrap();

        let (revision_tag_base, revision_ver) =
            if let Some((revision_tag_base, _)) = revision_tag.rsplit_once('-') {
                match (revision_tag_base, revision_offset) {
                    ("release", "0") => (revision_tag_base, version.clone()),

                    (_, "0") => {
                        let ver = format!("{version}-{revision_tag}");
                        if let Some(ver) = ver.strip_suffix(&format!("-{version}")) {
                            (revision_tag_base, ver.to_owned())
                        } else {
                            (revision_tag_base, ver)
                        }
                    }

                    (_, _) => (
                        revision_tag_base,
                        format!("{version}-{revision_tag}-{revision_offset}-{revision_hash}"),
                    ),
                }
            } else if revision_tag.starts_with('v')
                && revision_tag[1..].chars().next().unwrap().is_ascii_digit()
            {
                if revision_offset == "0" {
                    ("", format!("{version}-{revision_tag}"))
                } else {
                    (
                        "",
                        format!("{version}-{revision_tag}-{revision_offset}-{revision_hash}"),
                    )
                }
            } else {
                todo!()
            };
        let revision = format!("{lib_rev_name}-{revision_ver}");
        let revision_meta = revision.replace('_', "-");

        patch_file(
            &src_crate.cargo_toml_path(),
            &[LinesPatch {
                match_lines: &[&|s| s.starts_with("version =")],
                apply: &|_| format!("version = \"{revision_ver}\"\n"),
            }],
        )?;

        patch_file(
            &src_crate.lib_rs_path(),
            &[
                LinesPatch {
                    match_lines: &[
                        &|s| s == "#[cfg(not(windows))]",
                        &|s| s.starts_with("pub const SOURCE_DIR:"),
                        &|s| s == "#[cfg(windows)]",
                        &|s| s.starts_with("pub const SOURCE_DIR:"),
                    ],
                    apply: &|_| {
                        format!(
                            str_block! {r#"
                                #[cfg(not(windows))]
                                pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/{lib_dir}");
                                #[cfg(windows)]
                                pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\{lib_dir}");
                            "#},
                            lib_dir = lib_dir
                        )
                    },
                },
                LinesPatch {
                    match_lines: &[&|s| s.starts_with("pub const REVISION:")],
                    apply: &|_| format!("pub const REVISION: &str = \"{revision}\";\n"),
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
                    apply: &|_| {
                        format!("pub const REVISION_TAG_BASE: &str = {revision_tag_base:?};\n")
                    },
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
            &sys_crate.cargo_toml_path(),
            &[
                LinesPatch {
                    match_lines: &[&|s| {
                        s.starts_with("version =") && s.contains(&format!("+{lib_rev_name_meta}-"))
                    }],
                    apply: &|lines| {
                        let line = &lines[0];
                        let Some((revision_pos, _)) =
                            line.char_indices().rev().find(|(_, c)| *c == '+')
                        else {
                            unreachable!()
                        };
                        format!("{}{revision_meta}\"\n", &line[..=revision_pos],)
                    },
                },
                LinesPatch {
                    match_lines: &[
                        &|s| s == format!("[build-dependencies.{}]", src_crate.name),
                        &|s| s.starts_with("version ="),
                    ],
                    apply: &|lines| format!("{}version = \"{revision_ver}\"\n", lines[0]),
                },
            ],
        )?;

        patch_file(
            &sys_crate.readme_path(),
            &[LinesPatch {
                match_lines: &[&|s| s.contains(&format!("bindings for {lib_dir} version"))],
                apply: &|lines| {
                    let match_ = &format!("bindings for {lib_dir} version");
                    let line = &lines[0];
                    let pfx = &line[..line.find(match_).unwrap() + match_.len()];
                    if version == "3.2.0" {
                        format!("{pfx} `{version}`.\n")
                    } else if revision_ver.contains('-') {
                        format!("{pfx} `{revision_ver}`.\n")
                    } else {
                        format!("{pfx}s `3.2.0` to `{revision_ver}`, inclusive.\n")
                    }
                },
            }],
        )?;

        Ok(Self {
            lib_name: lib_name.to_owned(),
            src_crate,
            sys_crate,
            revision,
        })
    }

    fn config(&self, key: &str) -> &str {
        self.sys_crate.config(key)
    }

    fn source_root_path(&self) -> PathBuf {
        self.src_crate.root_path.join(self.config("lib_dir"))
    }

    fn headers_path(&self) -> PathBuf {
        self.source_root_path().join(self.config("include_dir"))
    }

    fn headers_prefix(&self) -> &str {
        self.config("headers_prefix")
    }

    fn output_path(&self) -> PathBuf {
        self.sys_crate.generated_path()
    }
}

pub fn generate(root: &Path, libs: &[String]) -> Result<(), Error> {
    let libs = libs
        .iter()
        .map(|lib| Library::new(root, lib))
        .collect::<Result<Vec<_>, _>>()?;

    let mut emitted_sdl3 = EmittedItems::new();

    for (i, lib) in libs.into_iter().enumerate() {
        let headers_path = lib.headers_path();
        let headers_prefix = lib.headers_prefix().to_ascii_lowercase();

        let mut generator = Gen::new(
            lib.lib_name.clone(),
            lib.config("sym_prefix").to_owned(),
            lib.config("hint_prop_prefix").to_owned(),
            emitted_sdl3,
            headers_path.clone(),
            lib.output_path(),
            lib.revision,
        )?;

        for entry in read_dir(&headers_path)
            .map_err(|e| format!("couldn't open dir `{}`: {}", headers_path.display(), e))?
        {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            let name_lc = name.to_ascii_lowercase();

            if let Some(module) = name_lc
                .strip_prefix(&headers_prefix)
                .and_then(|s| s.strip_suffix(".h"))
            {
                if skip(module) {
                    continue;
                }
                let mut buf = String::new();
                let entry_path = entry.path();
                File::open(&entry_path)
                    .map_err(|e| {
                        format!(
                            "error opening `{}` for reading: {}",
                            entry_path.display(),
                            e
                        )
                    })?
                    .read_to_string(&mut buf)
                    .map_err(|e| format!("error reading `{}`: {}", entry_path.display(), e))?;
                let buf = generator.patch_module(module, buf);
                let display_path = entry.path();
                let display_path = display_path
                    .strip_prefix(root)
                    .unwrap_or(&display_path)
                    .to_string_lossy()
                    .to_string();
                generator.parse(module, display_path, buf)?;
            }
        }

        generator.emit_top_level()?; // create empty mod.rs to be appended by emit
        for module in generator.parsed.keys() {
            generator.emit(module)?;
        }
        generator.emit_top_level()?; // rewrite final mod.rs in sorted order

        if i == 0 {
            emitted_sdl3 = generator.emitted.into_inner();
        } else {
            emitted_sdl3 = generator.emitted_sdl3;
        }
    }

    Ok(())
}

pub type ParsedItems = BTreeMap<String, Items>;
pub type EmittedItems = BTreeMap<String, InnerEmitContext>;

#[derive(Default)]
pub struct Gen {
    lib_name: String,
    sym_prefix: String,
    hint_prop_prefix: String,
    revision: String,
    parsed: ParsedItems,
    emitted: RefCell<EmittedItems>,
    skipped: RefCell<HashSet<String>>,
    headers_path: PathBuf,
    output_path: PathBuf,
    emitted_sdl3: EmittedItems,
}

impl Gen {
    pub fn new(
        lib_name: String,
        sym_prefix: String,
        hint_prop_prefix: String,
        emitted_sdl3: EmittedItems,
        headers_path: PathBuf,
        output_path: PathBuf,
        revision: String,
    ) -> Result<Self, Error> {
        DirBuilder::new().recursive(true).create(&output_path)?;
        Ok(Self {
            lib_name,
            sym_prefix,
            hint_prop_prefix,
            revision,
            parsed: ParsedItems::new(),
            emitted_sdl3,
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
        if !self.emitted.borrow().contains_key(module)
            && !self.emitted_sdl3.contains_key(module)
            && !self.skipped.borrow().contains(module)
        {
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
            let mut file = Writable(BufWriter::new(
                OpenOptions::new().append(true).open(&path).map_err(|e| {
                    format!("error opening `{}` for writing: {}", path.display(), e)
                })?,
            ));
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
                "#![allow(",
                "clippy::approx_constant, ",
                "clippy::doc_lazy_continuation, ",
                "clippy::double_parens, ",
                "clippy::eq_op, ",
                "clippy::identity_op, ",
                "clippy::missing_safety_doc, ",
                "clippy::needless_bool, ",
                "clippy::needless_return, ",
                "clippy::nonminimal_bool, ",
                "clippy::ptr_eq, ",
                "clippy::too_long_first_doc_paragraph, ",
                "clippy::unnecessary_cast, ",
                "non_camel_case_types, ",
                "non_snake_case, ",
                "non_upper_case_globals, ",
                "unused_imports, ",
                "unused_parens, ",
                "unused_unsafe, ",
                "unused_variables, ",
                ")]"
            )
        )?;
        writeln!(output)?;
        for module in self.emitted.borrow().keys() {
            writeln!(output, "pub mod {module};")?;
        }
        writeln!(output)?;
        let mut everything_module = String::new();
        writeln!(
            everything_module,
            "/// Reexports of everything from the other modules"
        )?;
        writeln!(everything_module, "pub mod everything {{")?;
        for module in self.emitted.borrow().keys() {
            writeln!(everything_module, "    #[doc(no_inline)]")?;
            writeln!(everything_module, "    pub use super::{module}::*;")?;
        }
        writeln!(everything_module, "}}")?;
        write!(output, "{everything_module}")?;
        format_and_write(output, &path)?;

        let metadata_path = self
            .output_path
            .join("..")
            .join("metadata")
            .join("generated");

        let mut metadata_out = String::from(str_block! {"
            #![allow(non_upper_case_globals, unused)]

            use core::ffi::CStr;
            use sdl3_sys::{{metadata::{{Group, GroupKind, GroupValue, Hint, Property, PropertyType}}, version::SDL_VERSIONNUM}};

        "});
        let mut metadata_out_hints = String::new();
        let mut metadata_out_props = String::new();
        let mut metadata_out_groups = String::new();
        let emitted = self.emitted.borrow();

        fn get_available_since(doc: &str) -> String {
            let mut lines = doc.lines();
            while let Some(line) = lines.next() {
                if line.contains("# Availability") {
                    let avail = lines.next().unwrap();
                    let avail_since = "available since ";
                    let i = avail.find(avail_since).unwrap();
                    let (_, ver) = avail[i + avail_since.len()..].split_once(' ').unwrap();
                    let (major, rest) = ver.split_once('.').unwrap();
                    let (minor, rest) = rest.split_once('.').unwrap();
                    let micro = if let Some((micro, rest)) = rest.split_once('.') {
                        assert!(rest.trim().is_empty());
                        micro
                    } else {
                        rest
                    };
                    let major = major.parse::<i32>().unwrap();
                    let minor = minor.parse::<i32>().unwrap();
                    let micro = micro.parse::<i32>().unwrap();
                    return format!("Some(SDL_VERSIONNUM({major}, {minor}, {micro}))");
                }
            }
            String::from("None")
        }
        macro_rules! write_indented {
            ($target:expr, $indent: expr, $($tt:tt)*) => {{
                let indent = " ".repeat($indent);
                for line in format!($($tt)*).lines() {
                    if line.is_empty() {
                        writeln!($target)?;
                    } else {
                        writeln!($target, "{indent}{line}")?;
                    }
                }
            }};
        }

        writeln!(
            metadata_out_hints,
            str_block! {"
                /// Metadata for hint constants in this crate
                pub const HINTS: &[&Hint] = &["}
        )?;
        writeln!(
            metadata_out_props,
            str_block! {"
                /// Metadata for property constants in this crate
                pub const PROPERTIES: &[&Property] = &["}
        )?;
        writeln!(
            metadata_out_groups,
            str_block! {"
                /// Metadata for groups in this crate
                pub const GROUPS: &[&Group] = &["}
        )?;

        for module in emitted.keys() {
            let metadata = &emitted[module].metadata;
            writeln!(metadata_out, "pub mod {module};")?;
            let mut module_out = format!(
                "//! Metadata for items in the `crate::{module}` module\n\nuse super::*;\n\n"
            );
            let hint_pfx = format!("{}HINT_", self.hint_prop_prefix);
            for hint in &metadata.hints {
                let short_name = hint.name.strip_prefix(&hint_pfx).unwrap();
                writeln!(metadata_out_hints, "    &{module}::METADATA_{},", hint.name)?;
                write!(
                    module_out,
                    str_block! {"
                        pub const METADATA_{name}: Hint = Hint {{
                            module: {module:?},
                            name: {name:?},
                            short_name: {short_name:?},
                            value: crate::{module}::{name},
                            doc: {doc},
                            available_since: {available_since},
                        }};
                    "},
                    module = module,
                    name = hint.name,
                    short_name = short_name,
                    available_since = get_available_since(&hint.doc),
                    doc = if hint.doc.is_empty() {
                        "None".into()
                    } else {
                        format!("Some({:?})", hint.doc)
                    }
                )?;
            }
            let prop_pfx = format!("{}PROP_", self.hint_prop_prefix);
            for prop in &metadata.properties {
                writeln!(metadata_out_props, "    &{module}::METADATA_{},", prop.name)?;
                let short_name = prop.name.strip_prefix(&prop_pfx).unwrap();
                let ty = if short_name.ends_with("_POINTER") {
                    "POINTER"
                } else if short_name.ends_with("_STRING") {
                    "STRING"
                } else if short_name.ends_with("_NUMBER") {
                    "NUMBER"
                } else if short_name.ends_with("_FLOAT") {
                    "FLOAT"
                } else if short_name.ends_with("_BOOLEAN") {
                    "BOOLEAN"
                } else {
                    match prop.name.as_str() {
                        "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_UINT8" => "NUMBER",
                        "SDL_PROP_WINDOW_OPENVR_OVERLAY_ID" => "NUMBER",
                        "TTF_PROP_FONT_CREATE_EXISTING_FONT" => "POINTER",
                        "TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE" => "NUMBER",
                        "TTF_PROP_GPU_TEXT_ENGINE_DEVICE" => "POINTER",
                        "TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE" => "NUMBER",
                        "TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER" => "POINTER",
                        _ => panic!("unknown property type for property {}", prop.name),
                    }
                };
                write!(
                    module_out,
                    str_block! {"
                        pub const METADATA_{name}: Property = Property {{
                            module: {module:?},
                            name: {name:?},
                            short_name: {short_name:?},
                            value: crate::{module}::{name},
                            ty: PropertyType::{ty},
                            doc: {doc},
                            available_since: {available_since},
                        }};
                    "},
                    module = module,
                    name = prop.name,
                    short_name = short_name,
                    ty = ty,
                    doc = if prop.doc.is_empty() {
                        "None".into()
                    } else {
                        format!("Some({:?})", prop.doc)
                    },
                    available_since = get_available_since(&prop.doc),
                )?;
            }
            for group in &metadata.groups {
                let available_since = get_available_since(&group.doc);
                writeln!(
                    metadata_out_groups,
                    "    &{module}::METADATA_{},",
                    group.name
                )?;
                write!(
                    module_out,
                    str_block! {"
                        pub const METADATA_{name}: Group = Group {{
                            module: {module:?},
                            kind: GroupKind::{kind},
                            name: {name:?},
                            short_name: {short_name:?},
                            doc: {doc},
                            available_since: {available_since},
                            values: &[
                    "},
                    module = module,
                    kind = match group.kind {
                        EnumKind::Enum => "Enum",
                        EnumKind::Flags => "Flags",
                        EnumKind::Id => "Id",
                        EnumKind::Lock => "Lock",
                    },
                    name = group.name,
                    short_name = group
                        .name
                        .strip_prefix(&self.sym_prefix)
                        .unwrap_or(&group.name),
                    doc = if group.doc.is_empty() {
                        "None".into()
                    } else {
                        format!("Some({:?})", group.doc)
                    },
                    available_since = available_since,
                )?;
                if !group.values.is_empty() {
                    for value in &group.values {
                        write_indented!(
                            module_out,
                            8,
                            str_block! {"
                                GroupValue {{
                                    name: {name:?},
                                    short_name: {short_name:?},
                                    doc: {doc},
                                    available_since: {available_since},
                                }},
                            "},
                            name = value.name,
                            short_name = value.short_name,
                            doc = if value.doc.is_empty() {
                                "None".into()
                            } else {
                                format!("Some({:?})", value.doc)
                            },
                            available_since = get_available_since(&value.doc),
                        );
                    }
                }
                write!(
                    module_out,
                    str_block! {"
                            ],
                        }};
                    "}
                )?;
            }
            format_and_write(module_out, &metadata_path.join(format!("{module}.rs")))?;
        }

        writeln!(metadata_out)?;
        writeln!(metadata_out, "{everything_module}")?;
        writeln!(metadata_out, "{metadata_out_hints}];")?;
        writeln!(metadata_out)?;
        writeln!(metadata_out, "{metadata_out_props}];")?;
        writeln!(metadata_out)?;
        writeln!(metadata_out, "{metadata_out_groups}];")?;

        format_and_write(metadata_out, &metadata_path.join("mod.rs"))?;
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
                    .patch(read_to_string(&main_impl).unwrap().as_str())
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
    Other(String),
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

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
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

pub const fn is_valid_ident(s: &str) -> bool {
    matches!(s.as_bytes()[0], b'a'..=b'z' | b'A'..=b'Z' | b'_')
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

fn find_common_ident_prefix<'a>(for_type: &str, mut it: impl Iterator<Item = &'a str>) -> &'a str {
    #[allow(clippy::single_match)]
    match for_type {
        "SDL_AudioDeviceID" => return "SDL_AUDIO_DEVICE_",
        "SDL_GlobFlags" => return "SDL_GLOB_",
        _ => (),
    }

    let mut prefix = it.next().unwrap_or_default();
    if let Some(next) = it.next() {
        prefix = common_ident_prefix(prefix, next);
        for i in it {
            prefix = common_ident_prefix(prefix, i);
        }
        prefix
    } else {
        ""
    }
}

fn strip_common_ident_prefix<'a>(ident: &'a str, prefix: &str) -> &'a str {
    let mut stripped = ident.strip_prefix(prefix).unwrap();
    if !is_valid_ident(stripped) {
        stripped = &ident[ident.len() - stripped.len() - 1..];
    }
    stripped
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
