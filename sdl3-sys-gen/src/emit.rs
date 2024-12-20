use crate::{
    common_ident_prefix,
    parse::{
        ArgDecl, CanCopy, Conditional, ConditionalExpr, Define, DocComment, DocCommentFile, Expr,
        FnAbi, FnDeclArgs, FnPointer, Function, GetSpan, Ident, IdentOrKwT, Include,
        IntegerLiteral, Item, Items, Literal, ParseErr, PreProcBlock, PrimitiveType, StructFields,
        StructKind, StructOrUnion, Type, TypeDef, TypeEnum,
    },
};
use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
    io,
    ops::Deref,
    rc::Rc,
};
use str_block::str_block;

mod expr;
pub use expr::Value;
mod item;
mod patch;
use patch::{patch_emit_define, patch_emit_function, patch_emit_macro_call, patch_emit_type_def};
mod state;
pub use state::{Cfg, DefineState, EmitContext, InnerEmitContext, Sym, SymKind};
use state::{EmitStatus, PreProcState, StructSym};

pub const fn is_valid_ident(s: &str) -> bool {
    matches!(s.as_bytes()[0], b'a'..=b'z' | b'A'..=b'Z' | b'_')
}

pub fn doc_link_sym(s: &str) -> Option<(&str, &str)> {
    match s {
        // libraries
        "SDL_image"
        | "SDL_ttf"
        // unused preprocessor syms, environment variables, etc
        | "SDL_FILESYSTEM_BASE_DIR_TYPE"
        | "SDL_FUNCTION_POINTER_IS_VOID_POINTER"
        | "SDL_GDK_TEXTINPUT"
        | "SDL_HAPTIC_GAIN_MAX"
        | "SDL_HIDAPI_DISABLED"
        | "SDL_MAIN_HANDLED"
        | "SDL_MAIN_USE_CALLBACKS"
        | "SDL_SINT64_C"
        | "SDL_UDEV_deviceclass"
        | "SDL_UINT64_C"
        // SDL 1.2 syms
        | "SDL_Flip()"
        | "SDL_UpdateRects()"
        // text
        | "SDL_GPU"
        | "SDL_HINT_X"
        // FIXME: work around for rustdoc bug: https://github.com/rust-lang/rust/issues/133150
        | "SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER"
        | "SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER"
            => None,

        "SDL_Rects" => Some(("SDL_Rect", "s")),

        _ => Some((s, "")),
    }
}

fn emit_extern_start(ctx: &mut EmitContext, abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
    if let Some(abi) = &abi {
        match abi.ident.as_str() {
            "__cdecl" => write!(ctx, "extern \"cdecl\" ")?,
            "SDLCALL" => {
                // SDL explicitly uses the cdecl ABI on non-GNU Windows, but cdecl is the
                // default ABI on 32-bit Windows and is ignored on 64-bit Windows, so we can
                // just use C everywhere
                write!(ctx, "extern \"C\" ")?;
            }
            _ => return Err(ParseErr::new(abi.span(), "can't emit this abi").into()),
        }
    } else {
        write!(ctx, "extern \"C\" ")?;
    }
    if !for_fn_ptr {
        writeln!(ctx, "{{")?;
        ctx.increase_indent();
    }
    Ok(())
}

fn emit_extern_end(ctx: &mut EmitContext, _abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
    if !for_fn_ptr {
        ctx.decrease_indent();
        writeln!(ctx, "}}")?;
        writeln!(ctx)?;
    }
    Ok(())
}

pub type EmitResult = Result<(), EmitErr>;

#[derive(Debug)]
pub enum EmitErr {
    Parse(ParseErr),
    Fmt(fmt::Error),
    Io(io::Error),
}

impl From<ParseErr> for EmitErr {
    fn from(value: ParseErr) -> Self {
        Self::Parse(value)
    }
}

impl From<fmt::Error> for EmitErr {
    fn from(value: fmt::Error) -> Self {
        Self::Fmt(value)
    }
}

impl Display for EmitErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => Display::fmt(e, f),
            Self::Fmt(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
        }
    }
}

pub trait Emit: core::fmt::Debug {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult;

    fn emit_with_define_state(
        &self,
        ctx: &mut EmitContext,
        define_state: &DefineState,
    ) -> Result<PreProcState, EmitErr> {
        let pps = {
            let (pps, _pps_guard) = ctx.with_target_dependent_preproc_state_guard();
            write!(ctx, "apply_cfg!(")?;
            ctx.emit_define_state_cfg(define_state)?;
            writeln!(ctx, " => {{")?;
            ctx.increase_indent();
            if define_state
                == &DefineState::one(Ident::new_inline("SDL_WIKI_DOCUMENTATION_SECTION")).not()
            {
                self.emit(ctx)?;
            } else {
                self.emit(&mut { ctx.new_top_level() })?;
            }
            ctx.decrease_indent();
            writeln!(ctx, "}});")?;
            writeln!(ctx)?;
            pps
        };
        Ok(Rc::into_inner(pps).unwrap().into_inner())
    }
}

impl<T: Emit> Emit for Box<T> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.deref().emit(ctx)
    }
}

impl<T: Emit> Emit for Option<T> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(t) = self {
            t.emit(ctx)
        } else {
            Ok(())
        }
    }
}

impl<T: Emit> Emit for Vec<T> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for i in self.iter() {
            i.emit(ctx)?;
        }
        Ok(())
    }
}

pub trait Eval {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr>;
}

impl Emit for Item {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Item::PreProcBlock(ppb) => ppb.emit(ctx),
            Item::Block(_) => todo!(),
            Item::Skipped(_) => todo!(),
            Item::Define(d) => d.emit(ctx),
            Item::Undef(_) => Ok(()),
            Item::Include(i) => i.emit(ctx),
            Item::Pragma(p) => {
                writeln!(ctx, "// pragma `{}`", p.as_str())?;
                Ok(())
            }
            Item::Error(e) => {
                writeln!(ctx, "::core::compile_error!({:?});", e.as_str())?;
                Ok(())
            }
            Item::Warning(_) => todo!(),
            Item::FileDoc(dc) => dc.emit(ctx),
            Item::StructOrUnion(s) => {
                if let Some(ident) = &s.ident {
                    ctx.scope_mut().register_struct_or_union_sym(StructSym {
                        kind: s.kind,
                        doc: s.doc.clone(),
                        ident: ident.clone(),
                        fields: s.fields.clone(),
                        emit_status: EmitStatus::NotEmitted,
                        hidden: s.hidden,
                        can_copy: s.can_copy,
                        can_construct: s.can_construct,
                    })?;
                    Ok(())
                } else {
                    todo!()
                }
            }
            Item::Enum(_) => todo!(),
            Item::Function(f) => f.emit(ctx),
            Item::Expr(e) => e.emit(ctx),
            Item::FnCall(call) => call.emit(ctx),
            Item::TypeDef(td) => td.emit(ctx),
            Item::VarDecl(_) => todo!(),
            Item::DoWhile(_) => todo!(),
            Item::While(_) => todo!(),
            Item::For(_) => todo!(),
            Item::IfElse(_) => todo!(),
            Item::Return(_) => todo!(),
            Item::EnumVariant(_) => todo!(),
            Item::Break(_) => todo!(),
            Item::Continue(_) => todo!(),
        }
    }
}

impl Emit for Items {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.0.emit(ctx)
    }
}

impl DocComment {
    fn insert_links(line: &str) -> Result<String, EmitErr> {
        let mut patched = String::new();
        let mut i0 = 0;
        let mut quoted = 0;
        for (i, _) in line.match_indices(['h', 'S', '[', ']']) {
            if i < i0 {
                continue;
            }
            quoted += line[i0..i].chars().filter(|c| *c == '`').count();

            if (quoted & 1 == 0)
                || (line.as_bytes().get(i.wrapping_sub(1)).copied() == Some(b'`')
                    && line.as_bytes().get(i.wrapping_sub(2)).copied() != Some(b'['))
            {
                if (line[i..].starts_with("https://") || line[i..].starts_with("http://"))
                    && (i == 0
                        || line.as_bytes()[i - 1].is_ascii_whitespace()
                        || (line.as_bytes()[i - 1] == b'('
                            && line.as_bytes().get(i.saturating_sub(2)).copied() != Some(b']')))
                {
                    write!(patched, "{}", &line[i0..i])?;
                    i0 = i + line[i..]
                        .find(|c: char| {
                            c.is_ascii_whitespace()
                                || c == ','
                                || line.as_bytes().get(i - 1).copied() == Some(b'(') && c == ')'
                        })
                        .unwrap_or(line.len() - i);
                    write!(patched, "<{}>", &line[i..i0])?;
                } else if line[i..].starts_with("SDL_")
                    && (i == 0
                        || line.as_bytes()[i - 1].is_ascii_whitespace()
                        || line.as_bytes()[i - 1] == b'('
                        || (line.as_bytes()[i - 1] == b'`'
                            && line.as_bytes().get(i.wrapping_sub(2)).copied() != Some(b'[')))
                {
                    let mut end = i + line[i..]
                        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                        .unwrap_or(line.len() - i);
                    if end > i + 4
                        && (line.len() == end
                            || line.as_bytes()[end].is_ascii_whitespace()
                            || matches!(line.as_bytes()[end], b')' | b',' | b';' | b':' | b'`')
                            || (line.as_bytes()[end] == b'.'
                                && line.as_bytes().get(end + 1).copied() != Some(b'h'))
                            || (line.as_bytes()[end] == b'('
                                && line.as_bytes().get(end + 1).copied() == Some(b')')))
                        && (quoted & 1 == 0 || line.as_bytes()[end] == b'`')
                    {
                        if line.as_bytes().get(end).copied() == Some(b'(')
                            && line.as_bytes().get(end + 1).copied() == Some(b')')
                        {
                            end += 2;
                        }
                        let Some((sym, post)) = doc_link_sym(&line[i..end]) else {
                            write!(patched, "{}", &line[i0..i])?;
                            i0 = i;
                            continue;
                        };
                        if quoted & 1 == 0 {
                            write!(patched, "{}", &line[i0..i])?;
                            write!(patched, "[`{sym}`]{post}")?;
                            i0 = end;
                        } else {
                            write!(patched, "{}", &line[i0..i - 1])?;
                            write!(patched, "[`{sym}`]{post}")?;
                            i0 = end + 1;
                            quoted += 1;
                        }
                    } else {
                        write!(patched, "{}", &line[i0..i])?;
                        i0 = i;
                    }
                } else {
                    write!(patched, "{}", &line[i0..i])?;
                    i0 = i;

                    if quoted & 1 == 0 {
                        // escape `[` and `]` that aren't markdown links
                        match line.as_bytes()[i] {
                            b'[' => {
                                if let Some(e) = line[i + 1..].find(['[', ']']).map(|e| e + i + 1) {
                                    if (line.as_bytes()[e] == b'['
                                        && line.as_bytes().get(e + 1).copied() != Some(b'\''))
                                        || !matches!(
                                            line.as_bytes().get(e + 1).copied(),
                                            Some(b'(') | Some(b'\''),
                                        )
                                    {
                                        patched.write_char('\\')?;
                                    }
                                }
                            }
                            b']' => {
                                if !matches!(
                                    line.as_bytes().get(i + 1).copied(),
                                    Some(b'(') | Some(b'\''),
                                ) {
                                    patched.write_char('\\')?;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            } else {
                write!(patched, "{}", &line[i0..i])?;
                i0 = i;
            }
        }
        write!(patched, "{}", &line[i0..])?;
        Ok(patched)
    }

    fn emit_rust(&self, ctx: &mut EmitContext, pfx: &str) -> EmitResult {
        let lines = self.to_string();
        let mut lines = lines.lines().peekable();
        let mut current_section = "";

        let mut section = |ctx: &mut EmitContext, name| -> EmitResult {
            if current_section != name {
                if name == "Return value" && current_section == "Parameters" {
                    writeln!(ctx, "{pfx}")?;
                }
                current_section = name;
                writeln!(ctx, "{pfx} ### {name}")?;
            }
            Ok(())
        };

        'lines: while let Some(line) = lines.next() {
            let line = Self::insert_links(line)?;

            if line.is_empty() {
                writeln!(ctx, "{pfx}")?;
                continue;
            } else if let Some(cmd) = line.strip_prefix('\\') {
                let Some((cmd, rest)) = cmd.split_once(char::is_whitespace) else {
                    writeln!(ctx, "{pfx} {line}")?;
                    continue;
                };
                let rest = rest.trim_start();
                let mut emit_block = |ctx: &mut EmitContext, plen| -> EmitResult {
                    let bpfx = " ".repeat(plen);
                    while let Some(line) = lines.peek().and_then(|s| s.strip_prefix(&bpfx)) {
                        writeln!(ctx, "{pfx}   {}", Self::insert_links(line)?)?;
                        lines.next();
                    }
                    Ok(())
                };
                match cmd {
                    "param" => {
                        section(ctx, "Parameters")?;
                        let (param, rest) = rest.split_once(char::is_whitespace).unwrap();
                        let rest = rest.trim_start();
                        writeln!(ctx, "{pfx} - `{param}`: {rest}")?;
                        emit_block(ctx, line.len() - rest.len())?;
                    }
                    "returns" => {
                        section(ctx, "Return value")?;
                        writeln!(ctx, "{pfx} Returns {rest}")?;
                        emit_block(ctx, line.len() - rest.len())?;
                    }
                    "sa" => {
                        section(ctx, "See also")?;
                        writeln!(ctx, "{pfx} - {}", rest.trim())?;
                    }
                    "since" => {
                        section(ctx, "Availability")?;
                        writeln!(ctx, "{pfx} {rest}")?;
                    }
                    "threadsafety" => {
                        section(ctx, "Thread safety")?;
                        writeln!(ctx, "{pfx} {rest}")?;
                        emit_block(ctx, line.len() - rest.len())?;
                    }
                    _ => writeln!(ctx, "{pfx} {line}")?,
                }
            } else if let Some(fmt) = line.strip_prefix("```") {
                match fmt {
                    "" => writeln!(ctx, "{pfx} ```text")?,
                    _ => writeln!(ctx, "{pfx} {line}")?,
                }
                for line in lines.by_ref() {
                    writeln!(ctx, "{pfx} {line}")?;
                    if line.starts_with("```") {
                        break;
                    }
                }
            } else if let Some(line) = line.strip_prefix("    ") {
                writeln!(ctx, "{pfx} ```text")?;
                writeln!(ctx, "{pfx} {line}")?;
                let mut empty_lines = 0;
                for line in lines.by_ref() {
                    if line.is_empty() {
                        empty_lines += 1;
                    } else if let Some(line) = line.strip_prefix("    ") {
                        for _ in 0..empty_lines {
                            writeln!(ctx, "{pfx}")?;
                        }
                        empty_lines = 0;
                        writeln!(ctx, "{pfx} {line}")?;
                    } else {
                        writeln!(ctx, "{pfx} ```")?;
                        for _ in 0..empty_lines {
                            writeln!(ctx, "{pfx}")?;
                        }
                        writeln!(ctx, "{pfx} {line}")?;
                        continue 'lines;
                    }
                }
                writeln!(ctx, "{pfx} ```")?;
            } else {
                writeln!(ctx, "{pfx} {line}")?;
            }
        }
        Ok(())
    }
}

impl Emit for DocComment {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.emit_rust(ctx, "///")
    }
}

impl Emit for DocCommentFile {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if !ctx.emitted_file_doc() {
            ctx.set_emitted_file_doc(true);
            self.0.emit_rust(ctx, "//!")?;
            writeln!(ctx)?;
        }
        Ok(())
    }
}

impl Conditional {
    fn emit_cfg(&self, ctx: &mut EmitContext) -> EmitResult {
        fn eval(
            ctx: &mut EmitContext,
            cond: DefineState,
            c: &ConditionalExpr,
            positive: bool,
        ) -> Result<DefineState, EmitErr> {
            match c {
                ConditionalExpr::If(expr) => {
                    let mut value = {
                        let _eval_mode = ctx.preproc_eval_mode_guard();
                        expr.try_eval(ctx)?
                    };
                    if value.is_none() {
                        if let Expr::BinaryOp(bop) = &expr {
                            if let Expr::Ident(lhs) = &bop.lhs {
                                value = ctx.try_target_dependent_if_compare(
                                    bop.op.as_str(),
                                    lhs.as_str(),
                                    &bop.rhs,
                                );
                            }
                        }
                    }
                    if let Some(value) = value {
                        Ok(if let Value::TargetDependent(define_state) = value {
                            if positive {
                                cond.all(define_state)
                            } else {
                                cond.any(define_state)
                            }
                        } else if value.is_truthy() {
                            // unconditional yes
                            cond
                        } else {
                            // unconditional no
                            if positive {
                                cond.all(DefineState::never())
                            } else {
                                cond.any(DefineState::never())
                            }
                        })
                    } else {
                        Err(ParseErr::new(expr.span(), "couldn't evaluate if expression").into())
                    }
                }
                ConditionalExpr::IfDef(_ident) => todo!(),
                ConditionalExpr::IfNDef(_ident) => todo!(),
            }
        }

        let mut cond = DefineState::none();
        for c in self.not.iter() {
            cond = eval(ctx, cond, c, false)?;
        }
        cond = cond.not();
        if let Some(req) = &self.require {
            cond = eval(ctx, cond, req, true)?;
        }
        ctx.emit_define_state_cfg(&cond)
    }
}

impl<const ALLOW_INITIAL_ELSE: bool> Emit for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.cond_expr {
            Some(ConditionalExpr::If(expr)) => {
                let mut value = {
                    let _eval_mode = ctx.preproc_eval_mode_guard();
                    expr.try_eval(ctx)?
                };
                if value.is_none() {
                    if let Expr::BinaryOp(bop) = &expr {
                        if let Expr::Ident(lhs) = &bop.lhs {
                            value = ctx.try_target_dependent_if_compare(
                                bop.op.as_str(),
                                lhs.as_str(),
                                &bop.rhs,
                            );
                        }
                    }
                }
                if let Some(value) = value {
                    if let Value::TargetDependent(define_state) = value {
                        let pps1 = self.block.emit_with_define_state(ctx, &define_state)?;
                        if let Some(else_block) = &self.else_block {
                            let pps2 =
                                else_block.emit_with_define_state(ctx, &define_state.not())?;
                            ctx.merge_target_dependent_preproc_state(pps2);
                        }
                        ctx.merge_target_dependent_preproc_state(pps1);
                        Ok(())
                    } else if value.is_truthy() {
                        self.block.emit(ctx)
                    } else {
                        self.else_block.emit(ctx)
                    }
                } else {
                    Err(ParseErr::new(expr.span(), "couldn't evaluate if expression").into())
                }
            }

            Some(ConditionalExpr::IfDef(ident)) => {
                if ctx.preproc_state().borrow().is_target_define(ident) {
                    let define_state = DefineState::one(ident.clone());
                    let pps1 = self.block.emit_with_define_state(ctx, &define_state)?;
                    if let Some(else_block) = &self.else_block {
                        let pps2 = else_block.emit_with_define_state(ctx, &define_state.not())?;
                        ctx.merge_target_dependent_preproc_state(pps2);
                    }
                    ctx.merge_target_dependent_preproc_state(pps1);
                    Ok(())
                } else if ctx.preproc_state().borrow().is_defined(ident)? {
                    self.block.emit(ctx)
                } else {
                    self.else_block.emit(ctx)
                }
            }

            Some(ConditionalExpr::IfNDef(ident)) => {
                if ctx.preproc_state().borrow().is_target_define(ident) {
                    let define_state = DefineState::one(ident.clone());
                    let pps1 = self
                        .block
                        .emit_with_define_state(ctx, &define_state.clone().not())?;
                    if let Some(else_block) = &self.else_block {
                        let pps2 = else_block.emit_with_define_state(ctx, &define_state)?;
                        ctx.merge_target_dependent_preproc_state(pps2);
                    }
                    ctx.merge_target_dependent_preproc_state(pps1);
                    Ok(())
                } else if !ctx.preproc_state().borrow().is_defined(ident)? {
                    self.block.emit(ctx)
                } else {
                    self.else_block.emit(ctx)
                }
            }

            None => {
                assert!(self.else_block.is_none());
                self.block.emit(ctx)
            }
        }
    }
}

impl Emit for Define {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if patch_emit_define(ctx, self)? {
            // patched
        } else if let Some(args) = &self.args {
            // function-like define
            if self.value.is_empty() {
                return Ok(());
            }
            let body = {
                let _guard = ctx.subscope_guard();
                for arg in args.iter() {
                    let Ok(ident) = arg.ident.clone().try_into() else {
                        ctx.log_skipped("function-like define", self.ident.as_str())?;
                        return Ok(());
                    };
                    ctx.register_sym(
                        ident,
                        None,
                        Some(arg.ty.clone()),
                        None,
                        SymKind::Other,
                        false,
                        arg.ty.can_derive_debug(ctx),
                    )?;
                }
                let _guard = ctx.expect_unresolved_sym_dependency_guard();
                if let Ok(Some(body)) = self.value.try_eval(ctx) {
                    body
                } else if ctx.emit_after_unresolved_sym_dependencies(self.clone()) {
                    return Ok(());
                } else {
                    ctx.log_skipped("function-like define", self.ident.as_str())?;
                    return Ok(());
                }
            };

            if let Ok(f) = ctx.capture_output(|ctx| {
                self.doc.emit(ctx)?;
                writeln!(ctx, "#[inline(always)]")?;
                write!(
                    ctx,
                    "pub {}{}fn {}(",
                    if body.is_const() { "const " } else { "" },
                    if body.is_unsafe() { "unsafe " } else { "" },
                    self.ident
                )?;
                for arg in args.iter() {
                    write!(ctx, "{}: ", arg.ident)?;
                    arg.ty.emit(ctx)?;
                    write!(ctx, ", ")?;
                }
                write!(ctx, ")")?;
                if !body.ty()?.is_void() {
                    write!(ctx, " -> ")?;
                    body.ty()?.emit(ctx)?;
                }
                writeln!(ctx, " {{")?;
                ctx.increase_indent();
                body.emit(ctx)?;
                writeln!(ctx)?;
                ctx.decrease_indent();
                writeln!(ctx, "}}")?;
                writeln!(ctx)?;
                Ok(())
            }) {
                let return_type = body.ty()?;
                ctx.register_sym(
                    self.ident.clone().try_into().unwrap(),
                    None,
                    Some(Type::function(
                        args.iter().map(|arg| arg.ty.clone()).collect(),
                        return_type,
                        body.is_const(),
                        body.is_unsafe(),
                    )),
                    None,
                    SymKind::Other,
                    false,
                    false,
                )?;
                writeln!(ctx, "{f}")?;
                return Ok(());
            }

            ctx.log_skipped("function-like define", self.ident.as_str())?;
        } else {
            // constant value define
            ctx.preproc_state().borrow_mut().define(
                self.ident.clone().try_into().unwrap(),
                self.args.clone(),
                self.value.clone(),
            )?;
            if self.value.is_empty() {
                return Ok(());
            }
            if let Some(value) = self.value.try_eval(ctx)? {
                let ty = value.ty()?;
                ctx.register_sym(
                    self.ident.clone().try_into().unwrap(),
                    None,
                    Some(ty.clone()),
                    None,
                    SymKind::Other,
                    true,
                    ty.can_derive_debug(ctx),
                )?;
                self.doc.emit(ctx)?;
                write!(ctx, "pub const ")?;
                self.ident.emit(ctx)?;
                write!(ctx, ": ")?;
                ty.emit(ctx)?;
                write!(ctx, " = ")?;
                value.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
            } else {
                ctx.log_skipped("constant value define", self.ident.as_str())?;
            }
        }
        Ok(())
    }
}

impl Emit for Include {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(module) = self.path.as_str().strip_prefix("SDL3/SDL_") {
            let module = module.strip_suffix(".h").unwrap();
            if !ctx.r#gen.emitted.borrow().contains_key(module) {
                ctx.r#gen.emit(module)?;
            }
            if let Some(included) = &ctx.r#gen.emitted.borrow_mut().get(module) {
                ctx.preproc_state()
                    .borrow_mut()
                    .include(&included.preproc_state.borrow())?;
                ctx.scope_mut().include(&included.scope)?;
                writeln!(ctx, "use super::{module}::*;")?;
                writeln!(ctx)?;
            }
        }
        Ok(())
    }
}

impl<const ALLOW_KEYWORDS: bool> Emit for IdentOrKwT<ALLOW_KEYWORDS> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        write!(ctx, "{self}")?;
        Ok(())
    }
}

impl Emit for Function {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if patch_emit_function(ctx, self)? {
        } else if self.static_kw.is_none() && !self.attr.contains("SDL_FORCE_INLINE") {
            ctx.register_sym(
                self.ident.clone(),
                None,
                Some(Type::function(
                    self.args.args.iter().map(|arg| arg.ty.clone()).collect(),
                    self.return_type.clone(),
                    false,
                    true,
                )),
                None,
                SymKind::Other,
                false,
                false,
            )?;
            emit_extern_start(ctx, &self.abi, false)?;
            self.doc.emit(ctx)?;
            write!(ctx, "pub fn ")?;
            self.ident.emit(ctx)?;
            self.args.emit(ctx)?;
            if !self.return_type.is_void() {
                write!(ctx, " -> ")?;
                self.return_type.emit(ctx)?;
            }
            writeln!(ctx, ";")?;
            emit_extern_end(ctx, &self.abi, false)?;
        } else {
            let Some(body) = &self.body else {
                ctx.log_skipped("inline function", self.ident.as_str())?;
                return Err(ParseErr::new(self.span(), "inline function with no body").into());
            };

            let mut ctx_body = ctx.new_top_level();
            ctx_body.set_function_return_type(self.return_type.clone());

            for arg in self.args.args.iter() {
                if let Some(ident) = &arg.ident {
                    ctx_body.register_sym(
                        ident.clone(),
                        None,
                        Some(arg.ty.clone()),
                        None,
                        SymKind::Other,
                        false,
                        arg.ty.can_derive_debug(&ctx_body),
                    )?;
                }
            }
            let Some(body) = body.try_eval(&ctx_body)? else {
                drop(ctx_body);
                ctx.log_skipped("inline function", self.ident.as_str())?;
                return Err(
                    ParseErr::new(self.span(), "can't eval body of inline function").into(),
                );
            };
            drop(ctx_body);

            ctx.register_sym(
                self.ident.clone(),
                None,
                Some(Type::function(
                    self.args.args.iter().map(|arg| arg.ty.clone()).collect(),
                    self.return_type.clone(),
                    body.is_const(),
                    body.is_unsafe(),
                )),
                None,
                SymKind::Other,
                false,
                false,
            )?;

            self.doc.emit(ctx)?;
            writeln!(ctx, "#[inline(always)]")?;
            write!(
                ctx,
                "pub {}{}fn ",
                if body.is_const() { "const " } else { "" },
                if body.is_unsafe() { "unsafe " } else { "" }
            )?;
            self.ident.emit(ctx)?;
            self.args.emit(ctx)?;
            if !self.return_type.is_void() {
                write!(ctx, " -> ")?;
                self.return_type.emit(ctx)?;
            }
            write!(ctx, " ")?;
            body.emit(ctx)?;
            writeln!(ctx)?;
            writeln!(ctx)?;
        }
        Ok(())
    }
}

impl Emit for FnPointer {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        write!(ctx, "::core::option::Option<")?;
        emit_extern_start(ctx, &self.abi, true)?;
        write!(ctx, "fn")?;
        self.args.emit(ctx)?;
        if !self.return_type.is_void() {
            write!(ctx, " -> ")?;
            self.return_type.emit(ctx)?;
        }
        emit_extern_end(ctx, &self.abi, true)?;
        write!(ctx, ">")?;
        Ok(())
    }
}

impl Emit for FnDeclArgs {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.write_char('(')?;
        if !self.args.is_empty() {
            let mut args = self.args.iter();
            args.next().unwrap().emit(ctx)?;
            for arg in args {
                write!(ctx, ", ")?;
                arg.emit(ctx)?;
            }
        }
        ctx.write_char(')')?;
        Ok(())
    }
}

impl Emit for ArgDecl {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(ident) = &self.ident {
            ident.emit(ctx)?;
            write!(ctx, ": ")?;
        }
        self.ty.emit(ctx)
    }
}

impl StructOrUnion {
    pub fn can_derive_copy(
        &self,
        ctx: &EmitContext,
        fields_override: Option<&StructFields>,
    ) -> bool {
        match self.can_copy {
            CanCopy::Always => return true,
            CanCopy::Never => return false,
            CanCopy::Default => (),
        }
        if let Some(fields) = fields_override.or(self.fields.as_ref()) {
            for field in fields.fields.iter() {
                if !field.ty.can_derive_copy(ctx) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn can_derive_debug(&self, ctx: &EmitContext) -> bool {
        if matches!(self.kind, StructKind::Union) {
            return false;
        }
        if let Some(fields) = &self.fields {
            for field in fields.fields.iter() {
                if !field.ty.can_derive_debug(ctx) {
                    return false;
                }
            }
        }
        true
    }

    pub fn emit_with_doc_and_ident(
        &self,
        ctx: &mut EmitContext,
        doc: Option<DocComment>,
        with_ident: bool,
    ) -> EmitResult {
        let ident = &self.generated_ident;
        let doc = self.doc.clone().or(doc);

        let sym = ctx.scope_mut().register_struct_or_union_sym(StructSym {
            kind: self.kind,
            ident: ident.clone(),
            fields: self.fields.clone(),
            doc: doc.clone(),
            emit_status: EmitStatus::Requested,
            hidden: self.hidden,
            can_copy: self.can_copy,
            can_construct: self.can_construct,
        })?;

        if let (true, Some(fields)) = (sym.emit_status != EmitStatus::Emitted, &sym.fields) {
            ctx.scope_mut().register_struct_or_union_sym(StructSym {
                kind: self.kind,
                ident: ident.clone(),
                fields: None,
                doc: None,
                emit_status: EmitStatus::Emitted,
                hidden: self.hidden,
                can_copy: self.can_copy,
                can_construct: self.can_construct,
            })?;

            let is_interface = if sym
                .doc
                .as_ref()
                .map(|doc| doc.span.contains("SDL_INIT_INTERFACE"))
                .unwrap_or(false)
            {
                let first_field = &fields.fields[0];
                if let TypeEnum::Ident(ty) = &first_field.ty.ty {
                    first_field.ident.as_str() == "version" && ty.as_str() == "Uint32"
                } else {
                    false
                }
            } else {
                false
            };

            let can_derive_copy = !is_interface
                && (sym.can_copy == CanCopy::Always
                    || (sym.can_copy != CanCopy::Never && self.can_derive_copy(ctx, Some(fields))));

            let ctx_ool = &mut { ctx.with_ool_output() };
            if self.hidden {
                writeln!(ctx_ool, "#[doc(hidden)]")?;
            }
            sym.doc.emit(ctx_ool)?;
            writeln!(ctx_ool, "#[repr(C)]")?;
            if !sym.can_construct {
                // !!!FIXME
                writeln!(ctx_ool, "// #[non_exhaustive] // temporarily disabled bc of https://github.com/rust-lang/rust/issues/132699")?;
            }
            if can_derive_copy {
                writeln!(ctx_ool, "#[derive(Clone, Copy)]")?;
            }
            if self.can_derive_debug(ctx_ool) {
                writeln!(
                    ctx_ool,
                    r#"#[cfg_attr(feature = "debug-impls", derive(Debug))]"#
                )?;
            }
            writeln!(
                ctx_ool,
                "pub {} {} {{",
                if self.is_struct() { "struct" } else { "union" },
                ident
            )?;
            ctx_ool.increase_indent();

            for field in fields.fields.iter() {
                field.doc.emit(ctx_ool)?;
                write!(ctx_ool, "pub ")?;
                field.ident.emit(ctx_ool)?;
                write!(ctx_ool, ": ")?;
                field.ty.emit(ctx_ool)?;
                writeln!(ctx_ool, ",")?;
            }

            ctx_ool.decrease_indent();
            writeln!(ctx_ool, "}}")?;
            writeln!(ctx_ool)?;

            if is_interface {
                writeln!(ctx_ool, "impl {ident} {{")?;
                ctx_ool.increase_indent();
                writeln!(
                    ctx_ool,
                    "/// Create a new `{ident}` initialized with `SDL_INIT_INTERFACE`"
                )?;
                writeln!(ctx_ool, "#[inline]")?;
                writeln!(ctx_ool, "pub const fn new() -> Self {{")?;
                ctx_ool.increase_indent();
                writeln!(
                    ctx_ool,
                    "::core::assert!(::core::mem::size_of::<Self>() <= ::core::primitive::u32::MAX as usize);"
                )?;
                writeln!(ctx_ool, "let mut this = unsafe {{ ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }};")?;
                writeln!(
                    ctx_ool,
                    "this.version = ::core::mem::size_of::<Self>() as ::core::primitive::u32;"
                )?;
                writeln!(ctx_ool, "this")?;
                ctx_ool.decrease_indent();
                writeln!(ctx_ool, "}}")?;
                ctx_ool.decrease_indent();
                writeln!(ctx_ool, "}}")?;
                writeln!(ctx_ool)?;
                writeln!(ctx_ool, "impl ::core::default::Default for {ident} {{")?;
                ctx_ool.increase_indent();
                writeln!(
                    ctx_ool,
                    "/// Create a new `{ident}` initialized with `SDL_INIT_INTERFACE`"
                )?;
                writeln!(ctx_ool, "#[inline(always)]")?;
                writeln!(ctx_ool, "fn default() -> Self {{")?;
                ctx_ool.increase_indent();
                writeln!(ctx_ool, "Self::new()")?;
                ctx_ool.decrease_indent();
                writeln!(ctx_ool, "}}")?;
                ctx_ool.decrease_indent();
                writeln!(ctx_ool, "}}")?;
                writeln!(ctx_ool)?;
            }
        }
        if with_ident {
            ident.emit(ctx)?;
        }
        Ok(())
    }
}

impl Type {
    pub fn is_c_enum(&self, ctx: &EmitContext) -> Result<Option<Sym>, EmitErr> {
        if let Some(Type {
            ty: TypeEnum::Ident(ident),
            ..
        }) = self.inner_ty()
        {
            if let Some(sym) = ctx.lookup_sym(&ident) {
                if ctx.lookup_enum_sym(&ident).is_some() {
                    return Ok(Some(sym));
                }
            } else {
                ctx.add_unresolved_sym_dependency(ident.clone())?;
                return Err(ParseErr::new(ident.span(), "unresolved symbol").into());
            }
        }
        Ok(None)
    }

    pub fn conjure_primitive_value(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        if let Some(ty) = &self.inner_ty() {
            match &ty.ty {
                TypeEnum::Primitive(p) => {
                    return Ok(Some(match p {
                        PrimitiveType::Float => Value::F32(0.0),
                        PrimitiveType::Double => Value::F64(0.0),
                        PrimitiveType::Bool => Value::Bool(false),
                        PrimitiveType::Int8T => Value::I32(0),
                        PrimitiveType::Uint8T => Value::U31(0),
                        PrimitiveType::Int16T => Value::I32(0),
                        PrimitiveType::Uint16T => Value::U31(0),
                        PrimitiveType::Int32T | PrimitiveType::Int => Value::I32(0),
                        PrimitiveType::Uint32T | PrimitiveType::UnsignedInt => Value::U32(0),
                        PrimitiveType::Int64T => Value::I64(0),
                        PrimitiveType::Uint64T => Value::U64(0),
                        PrimitiveType::SizeT => Value::Usize(0),
                        _ => return Ok(None),
                    }));
                }
                TypeEnum::Ident(i) => {
                    if let Some(sym) = ctx.lookup_sym(i) {
                        if let Some(ty) = &sym.alias_ty {
                            return ty.conjure_primitive_value(ctx);
                        }
                    } else {
                        ctx.add_unresolved_sym_dependency(i.clone())?;
                    }
                }
                _ => (),
            }
        }
        Ok(None)
    }
}

impl Emit for Type {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.ty {
            TypeEnum::Primitive(t) => match t {
                PrimitiveType::Char => write!(ctx, "::core::ffi::c_char")?,
                PrimitiveType::SignedChar => write!(ctx, "::core::ffi::c_schar")?,
                PrimitiveType::UnsignedChar => write!(ctx, "::core::ffi::c_uchar")?,
                PrimitiveType::Short => write!(ctx, "::core::ffi::c_short")?,
                PrimitiveType::UnsignedShort => write!(ctx, "::core::ffi::c_ushort")?,
                PrimitiveType::Int => write!(ctx, "::core::ffi::c_int")?,
                PrimitiveType::UnsignedInt => write!(ctx, "::core::ffi::c_uint")?,
                PrimitiveType::Long => write!(ctx, "::core::ffi::c_long")?,
                PrimitiveType::UnsignedLong => write!(ctx, "::core::ffi::c_ulong")?,
                PrimitiveType::LongLong => write!(ctx, "::core::ffi::c_longlong")?,
                PrimitiveType::UnsignedLongLong => write!(ctx, "::core::ffi::c_ulonglong")?,
                PrimitiveType::Float => write!(ctx, "::core::ffi::c_float")?,
                PrimitiveType::Double => write!(ctx, "::core::ffi::c_double")?,
                PrimitiveType::Void => write!(ctx, "()")?,
                PrimitiveType::Bool => write!(ctx, "::core::primitive::bool")?,
                PrimitiveType::SizeT => write!(ctx, "::core::primitive::usize")?,
                PrimitiveType::Int8T => write!(ctx, "::core::primitive::i8")?,
                PrimitiveType::Uint8T => write!(ctx, "::core::primitive::u8")?,
                PrimitiveType::Int16T => write!(ctx, "::core::primitive::i16")?,
                PrimitiveType::Uint16T => write!(ctx, "::core::primitive::u16")?,
                PrimitiveType::Int32T => write!(ctx, "::core::primitive::i32")?,
                PrimitiveType::Uint32T => write!(ctx, "::core::primitive::u32")?,
                PrimitiveType::Int64T => write!(ctx, "::core::primitive::i64")?,
                PrimitiveType::Uint64T => write!(ctx, "::core::primitive::u64")?,
                PrimitiveType::IntPtrT => write!(ctx, "::core::primitive::isize")?,
                PrimitiveType::UintPtrT => write!(ctx, "::core::primitive::usize")?,
                PrimitiveType::WcharT => write!(ctx, "crate::ffi::c_wchar_t")?,
                PrimitiveType::VaList => write!(ctx, "crate::ffi::VaList")?,
            },

            TypeEnum::Ident(i) => {
                ctx.use_ident(i)?;
                i.emit(ctx)?;
            }

            TypeEnum::Enum(_) => todo!(),

            TypeEnum::Struct(s) => return s.emit_with_doc_and_ident(ctx, None, true),

            TypeEnum::Pointer(p) => {
                if p.is_const {
                    write!(ctx, "*const ")?;
                } else {
                    write!(ctx, "*mut ")?;
                }
                if p.is_void() {
                    write!(ctx, "::core::ffi::c_void")?;
                } else {
                    p.emit(ctx)?;
                }
            }

            TypeEnum::Array(ty, len) => {
                write!(ctx, "[")?;
                ty.emit(ctx)?;
                write!(ctx, "; ")?;
                if let Some(len) = len.try_eval(ctx)? {
                    len.emit(ctx)?;
                } else {
                    return Err(ParseErr::new(len.span(), "invalid array length").into());
                };
                // check if the length is an enum value
                if let Expr::Ident(ident) = len {
                    if let Some(ty) = ctx
                        .lookup_sym(&ident.clone().try_into().unwrap())
                        .and_then(|s| s.value_ty)
                    {
                        if let TypeEnum::Ident(ident) = &ty.ty {
                            if ctx.lookup_enum_sym(ident).is_some() {
                                write!(ctx, ".0 as ::core::primitive::usize")?;
                            }
                        }
                    }
                }
                write!(ctx, "]")?;
            }

            TypeEnum::FnPointer(fnp) => fnp.emit(ctx)?,

            TypeEnum::DotDotDot => write!(ctx, "...")?,

            TypeEnum::Rust(r) => write!(ctx, "{}", r.string)?,

            TypeEnum::Function(_) => todo!(),

            TypeEnum::Infer(i) => {
                if let Some(ty) = &*i.borrow() {
                    return ty.emit(ctx);
                } else {
                    return Err(ParseErr::new(self.span(), "can't emit uninferred type").into());
                }
            }
        }
        Ok(())
    }
}

impl Emit for TypeDef {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if patch_emit_type_def(ctx, self.ident.as_str(), self)? {
            return Ok(());
        }

        let mut assoc_doc = String::new();
        let mut ctx_assoc_doc = ctx.with_output(&mut assoc_doc);
        if !self.associated_defines.borrow().is_empty() {
            if self.doc.is_some() {
                writeln!(ctx_assoc_doc, "///")?;
            }
            writeln!(ctx_assoc_doc, "/// ### Known values (`sdl3-sys`)")?;
            writeln!(ctx_assoc_doc, "/// | Constant | Description |")?;
            writeln!(ctx_assoc_doc, "/// | -------- | ----------- |")?;
            for (ident, doc) in self.associated_defines.borrow().iter() {
                write!(ctx_assoc_doc, "/// | [`{ident}`] |")?;
                if let Some(doc) = doc {
                    let doc = doc.to_string();
                    for line in doc.lines() {
                        write!(ctx_assoc_doc, " {}", DocComment::insert_links(line)?)?;
                    }
                }
                writeln!(ctx_assoc_doc, " |")?;
            }
        }
        drop(ctx_assoc_doc);

        match &self.ty.ty {
            TypeEnum::Primitive(_) => {
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    SymKind::Other,
                    true,
                    true,
                )?;
                self.doc.emit(ctx)?;
                write!(ctx, "{assoc_doc}")?;
                write!(ctx, "pub type ")?;
                self.ident.emit(ctx)?;
                write!(ctx, " = ")?;
                self.ty.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Ident(sym) => {
                let sym = ctx.lookup_sym(sym).ok_or_else(|| {
                    ParseErr::new(sym.span(), format!("`{}` not defined", sym.as_str()))
                })?;
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    sym.enum_base_ty,
                    sym.kind,
                    sym.can_derive_copy,
                    sym.can_derive_debug,
                )?;
                self.doc.emit(ctx)?;
                write!(ctx, "{assoc_doc}")?;
                write!(ctx, "pub type ")?;
                self.ident.emit(ctx)?;
                write!(ctx, " = ")?;
                sym.ident.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Enum(e) => {
                if let Some(ident) = &e.ident {
                    ctx.scope_mut().register_enum_sym(ident.clone())?;
                }

                if e.hidden {
                    writeln!(ctx, "#[doc(hidden)]")?;
                }
                self.doc.emit(ctx)?;
                assert!(e.doc.is_none());

                let enum_ident = self.ident.as_str();
                let enum_base_type = e.base_type.clone();
                let mut known_values = Vec::new();

                let prefix = if e.variants.len() > 1 {
                    let mut prefix = e
                        .variants
                        .first()
                        .map(|v| v.ident.as_str())
                        .unwrap_or_default();

                    for variant in &e.variants {
                        known_values.push(variant.ident.as_str());
                        prefix = common_ident_prefix(prefix, variant.ident.as_str());
                    }
                    prefix
                } else {
                    if let Some(variant) = e.variants.first() {
                        known_values.push(variant.ident.as_str());
                    }
                    ""
                };

                let enum_base_type = enum_base_type.unwrap_or(Type::primitive(PrimitiveType::Int));

                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    Some(enum_base_type.clone()),
                    SymKind::Other,
                    true,
                    true,
                )?;

                let mut doc_consts = String::new();
                let mut ctx_doc = ctx.with_output(&mut doc_consts);
                let mut impl_consts = String::new();
                let mut ctx_impl = ctx.with_output(&mut impl_consts);
                let mut global_consts = String::new();
                let mut ctx_global = ctx.with_output(&mut global_consts);
                let mut debug_consts = String::new();
                let mut ctx_debug = ctx.with_output(&mut debug_consts);
                let mut next_expr = Some(Expr::Literal(Literal::Integer(IntegerLiteral::zero())));

                if !known_values.is_empty() {
                    if self.doc.is_some() {
                        writeln!(ctx_doc, "///")?;
                    }
                    writeln!(ctx_doc, "/// ### Known values (`sdl3-sys`)")?;
                    writeln!(
                        ctx_doc,
                        "/// | Associated constant | Global constant | Description |"
                    )?;
                    writeln!(
                        ctx_doc,
                        "/// | ------------------- | --------------- | ----------- |"
                    )?;
                }

                let mut seen_target_dependent = HashSet::new();

                for variant in &e.variants {
                    ctx.register_sym(
                        variant.ident.clone(),
                        None,
                        Some(Type::ident(self.ident.clone())),
                        None,
                        SymKind::Other,
                        true,
                        true,
                    )?;

                    let variant_ident = variant.ident.as_str();
                    let mut short_variant_ident = variant_ident.strip_prefix(prefix).unwrap();
                    if !is_valid_ident(short_variant_ident) {
                        short_variant_ident =
                            &variant_ident[variant_ident.len() - short_variant_ident.len() - 1..];
                    }

                    let Some(expr) = variant.expr.as_ref().or(next_expr.as_ref()) else {
                        return Err(ParseErr::new(
                            variant.ident.span(),
                            "couldn't evaluate value for enum",
                        )
                        .into());
                    };

                    let mut value = ctx.capture_output(|ctx| expr.emit(ctx))?;
                    let need_wrap = if let Expr::Ident(ident) = &expr {
                        if let Some(TypeEnum::Ident(tid)) = ctx
                            .lookup_sym(&ident.clone().try_into().unwrap())
                            .and_then(|s| s.value_ty)
                            .map(|t| t.ty)
                        {
                            tid.as_str() != enum_ident
                        } else {
                            true
                        }
                    } else {
                        true
                    };
                    if need_wrap {
                        value.insert_str(0, "Self(");
                        value.push(')');
                    }

                    next_expr = expr.try_eval_plus_one(ctx)?.map(Expr::Value);

                    if !seen_target_dependent.contains(variant_ident) {
                        write!(
                            ctx_doc,
                            "/// | [`{short_variant_ident}`]({enum_ident}::{short_variant_ident}) | [`{variant_ident}`] |",
                        )?;
                        if !variant.cond.is_empty() {
                            seen_target_dependent.insert(variant_ident);
                            write!(ctx_doc, " (target dependent)")?;
                        }
                        if let Some(doc) = &variant.doc {
                            let doc = doc.to_string();
                            for line in doc.lines() {
                                write!(ctx_doc, " {}", DocComment::insert_links(line)?)?;
                            }
                        }
                        writeln!(ctx_doc, " |")?;
                    }

                    variant.cond.emit_cfg(&mut ctx_impl)?;
                    variant.doc.emit(&mut ctx_impl)?;
                    if !variant.cond.is_empty() {
                        writeln!(
                            ctx_impl,
                            r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                        )?;
                    }
                    writeln!(ctx_impl, "pub const {short_variant_ident}: Self = {value};")?;

                    if e.hidden {
                        writeln!(ctx_global, "#[doc(hidden)]")?;
                    }
                    variant.cond.emit_cfg(&mut ctx_global)?;
                    variant.doc.emit(&mut ctx_global)?;
                    if !variant.cond.is_empty() {
                        writeln!(
                            ctx_global,
                            r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                        )?;
                    }
                    writeln!(ctx_global, "pub const {variant_ident}: {enum_ident} = {enum_ident}::{short_variant_ident};")?;

                    writeln!(
                        ctx_debug,
                        "Self::{short_variant_ident} => {variant_ident:?},"
                    )?;
                }

                drop(ctx_debug);
                drop(ctx_global);
                drop(ctx_impl);
                drop(ctx_doc);

                ctx.write_str(&doc_consts)?;

                writeln!(ctx, "#[repr(transparent)]")?;
                writeln!(
                    ctx,
                    "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                )?;
                write!(ctx, "pub struct {enum_ident}(pub ")?;
                enum_base_type.emit(ctx)?;
                writeln!(ctx, ");")?;
                writeln!(ctx)?;

                write!(ctx, "impl From<{enum_ident}> for ")?;
                enum_base_type.emit(ctx)?;
                writeln!(
                    ctx,
                    str_block! {r#"
                        {{
                            #[inline(always)]
                            fn from(value: {}) -> Self {{
                                value.0
                            }}
                        }}
                    "#},
                    enum_ident
                )?;

                writeln!(
                    ctx,
                    str_block! {"
                        #[cfg(feature = \"debug-impls\")]
                        impl ::core::fmt::Debug for {} {{
                            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{
                                #[allow(unreachable_patterns)]
                                f.write_str(match *self {{
                                    {}
                                    _ => return write!(f, {}, self.0),
                                }})
                            }}
                        }}
                    "},
                    enum_ident,
                    debug_consts,
                    format!("\"{}({{}})\"", enum_ident)
                )?;

                writeln!(ctx, "impl {enum_ident} {{")?;
                ctx.increase_indent();
                ctx.write_str(&impl_consts)?;
                ctx.decrease_indent();
                writeln!(ctx, "}}")?;
                writeln!(ctx)?;
                ctx.write_str(&global_consts)?;
                writeln!(ctx)?;
                ctx.flush_ool_output()?;
                Ok(())
            }

            TypeEnum::Struct(s) => {
                s.emit_with_doc_and_ident(ctx, self.doc.clone(), false)?;

                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    if s.kind == StructKind::Struct {
                        SymKind::StructAlias
                    } else {
                        SymKind::UnionAlias
                    }(s.ident.clone().unwrap()),
                    s.can_derive_copy(ctx, None),
                    s.can_derive_debug(ctx),
                )?;

                ctx.flush_ool_output()?;

                if let Some(ident) = &s.ident {
                    if self.ident.as_str() != ident.as_str() {
                        write!(ctx, "pub type ")?;
                        self.ident.emit(ctx)?;
                        write!(ctx, " = ")?;
                        ident.emit(ctx)?;
                        writeln!(ctx, ";")?;
                        writeln!(ctx)?;
                    }
                }

                Ok(())
            }

            TypeEnum::Pointer(_) => {
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    SymKind::Other,
                    false,
                    true,
                )?;
                self.doc.emit(ctx)?;
                write!(ctx, "pub type {} = ", self.ident.as_str())?;
                self.ty.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Array(ty, _) => {
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    SymKind::Other,
                    ty.can_derive_copy(ctx),
                    ty.can_derive_debug(ctx),
                )?;
                self.doc.emit(ctx)?;
                todo!()
            }

            TypeEnum::FnPointer(f) => {
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    SymKind::Other,
                    false,
                    true,
                )?;
                self.doc.emit(ctx)?;
                write!(
                    ctx,
                    "pub type {} = ::core::option::Option<unsafe ",
                    self.ident.as_str()
                )?;
                emit_extern_start(ctx, &f.abi, true)?;
                write!(ctx, "fn")?;
                f.args.emit(ctx)?;
                if !f.return_type.is_void() {
                    write!(ctx, " -> ")?;
                    f.return_type.emit(ctx)?;
                }
                emit_extern_end(ctx, &f.abi, true)?;
                writeln!(ctx, ">;")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::DotDotDot => todo!(),

            TypeEnum::Rust(r) => {
                ctx.register_sym(
                    self.ident.clone(),
                    Some(self.ty.clone()),
                    None,
                    None,
                    SymKind::Other,
                    r.can_derive_copy,
                    r.can_derive_debug,
                )?;
                writeln!(ctx, "pub type {} = {};", self.ident.as_str(), r.string)?;
                Ok(())
            }

            TypeEnum::Function(_) => todo!(),
            TypeEnum::Infer(_) => todo!(),
        }
    }
}
