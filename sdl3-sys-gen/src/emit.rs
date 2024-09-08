use crate::{
    common_prefix,
    parse::{
        ArgDecl, Define, DocComment, DocCommentFile, Expr, FnAbi, FnDeclArgs, FnPointer, Function,
        GetSpan, Ident, Include, IntegerLiteral, Item, Items, Literal, ParseErr, PreProcBlock,
        PreProcBlockKind, PrimitiveType, StructKind, StructOrUnion, Type, TypeDef, TypeEnum,
    },
};
use std::{
    fmt::{self, Display, Write},
    io,
    ops::Deref,
    rc::Rc,
};

mod expr;
pub use expr::Value;
mod patch;
pub use patch::{patch_macro_call, patch_sdl_compile_time_assert};
mod state;
use state::PreProcState;
pub use state::{DefineState, EmitContext, InnerEmitContext};

pub const fn is_rust_keyword(s: &str) -> bool {
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

pub const fn is_valid_ident(s: &str) -> bool {
    matches!(s.as_bytes()[0], b'a'..=b'z' | b'A'..=b'Z' | b'_')
}

fn emit_extern_start(ctx: &mut EmitContext, abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
    if let Some(abi) = &abi {
        match abi.ident.as_str() {
            "__cdecl" => write!(ctx, "extern \"cdecl\" ")?,
            "SDLCALL" => {
                // SDL explicitly uses the cdecl ABI on 32-bit non-GNU Windows, but cdecl is
                // the default ABI on 32-bit Windows, so we can just use C everywhere
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

fn emit_extern_end(ctx: &mut EmitContext, abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
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
    ParseError(ParseErr),
    FmtError(fmt::Error),
    IoError(io::Error),
}

impl From<ParseErr> for EmitErr {
    fn from(value: ParseErr) -> Self {
        Self::ParseError(value)
    }
}

impl From<fmt::Error> for EmitErr {
    fn from(value: fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

impl Display for EmitErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(e) => Display::fmt(e, f),
            Self::FmtError(e) => Display::fmt(e, f),
            Self::IoError(e) => Display::fmt(e, f),
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
            ctx.emit_define_state_cfg(define_state)?;
            writeln!(ctx, "emit! {{")?;
            ctx.increase_indent();
            self.emit(ctx)?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
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
            Item::Undef(_) => todo!(),
            Item::Include(i) => i.emit(ctx),
            Item::Pragma(p) => {
                writeln!(ctx, "// pragma `{}`", p.as_str())?;
                Ok(())
            }
            Item::Error(e) => {
                writeln!(ctx, "::core::compile_error!({:?});", e.as_str())?;
                Ok(())
            }
            Item::FileDoc(dc) => dc.emit(ctx),
            Item::StructOrUnion(s) => {
                if let (Some(ident), false) = (&s.ident, s.fields.is_some()) {
                    ctx.scope_mut().register_struct_or_union_sym(
                        s.kind,
                        ident.clone(),
                        false,
                        s.doc.clone(),
                    )?;
                    Ok(())
                } else {
                    dbg!(s);
                    todo!()
                }
            }
            Item::Enum(_) => todo!(),
            Item::Function(f) => f.emit(ctx),
            Item::Expr(e) => e.emit(ctx),
            Item::FnCall(call) => {
                if let (Expr::Ident(call), Expr::Ident(arg)) = (&*call.func, &call.args[0]) {
                    if patch_macro_call(ctx, call.as_str(), arg.as_str())? {
                        Ok(())
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            Item::TypeDef(td) => td.emit(ctx),
            Item::VarDecl(_) => todo!(),
            Item::DoWhile(_) => todo!(),
            Item::For(_) => todo!(),
            Item::IfElse(_) => todo!(),
            Item::Return(_) => todo!(),
        }
    }
}

impl Emit for Items {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.0.emit(ctx)
    }
}

impl Emit for DocComment {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for line in self.to_string().lines() {
            writeln!(ctx, "///{}{}", if line.is_empty() { "" } else { " " }, line)?;
        }
        Ok(())
    }
}

impl Emit for DocCommentFile {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if !ctx.emitted_file_doc() {
            ctx.set_emitted_file_doc(true);
            for line in self.0.to_string().lines() {
                writeln!(ctx, "//!{}{}", if line.is_empty() { "" } else { " " }, line)?;
            }
            writeln!(ctx)?;
        }
        Ok(())
    }
}

impl<const ALLOW_INITIAL_ELSE: bool> Emit for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.kind {
            PreProcBlockKind::If(expr) => {
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

            PreProcBlockKind::IfDef(ident) => {
                if ctx.preproc_state().borrow().is_target_define(ident) {
                    let define_state = DefineState::defined(ident.clone());
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

            PreProcBlockKind::IfNDef(ident) => {
                if ctx.preproc_state().borrow().is_target_define(ident) {
                    let define_state = DefineState::defined(ident.clone());
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

            PreProcBlockKind::None => {
                assert!(self.else_block.is_none());
                self.block.emit(ctx)
            }
        }
    }
}

impl Emit for Define {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.preproc_state().borrow_mut().define(
            self.ident.clone(),
            self.args.clone(),
            self.value.clone(),
        )?;
        if self.args.is_none() {
            let ident = self.ident.as_str();
            if ident.ends_with("_h_") || ident == "SDL_locale_h" {
                // skip include guard define
                return Ok(());
            }
            if let Some(value) = self.value.try_eval(ctx)? {
                let ty = value.ty();
                ctx.register_sym(
                    self.ident.clone(),
                    Some(ty.clone()),
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

impl Emit for Ident {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if is_rust_keyword(self.as_str()) {
            write!(ctx, "r#")?;
        }
        write!(ctx, "{}", self.as_str())?;
        Ok(())
    }
}

impl Emit for Function {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if self.static_kw.is_none() && !self.attr.contains("SDL_FORCE_INLINE") {
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
            writeln!(ctx, "// skipped inline function `{}`", self.ident.as_str())?;
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

        ctx.scope_mut().register_struct_or_union_sym(
            self.kind,
            ident.clone(),
            self.fields.is_some(),
            doc.clone(),
        )?;

        if let Some(fields) = &self.fields {
            let ctx_ool = &mut { ctx.with_ool_output() };
            doc.emit(ctx_ool)?;
            writeln!(ctx_ool, "#[repr(C)]")?;
            writeln!(ctx_ool, "#[derive(Clone, Copy)]")?;
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
        }
        if with_ident {
            ident.emit(ctx)?;
        }
        Ok(())
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
                        .and_then(|s| s.ty)
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

            TypeEnum::Rust(r, _) => write!(ctx, "{r}")?,
        }
        Ok(())
    }
}

impl Emit for TypeDef {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.ty.ty {
            TypeEnum::Primitive(p) => {
                ctx.register_sym(self.ident.clone(), None, true)?;
                self.doc.emit(ctx)?;
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
                ctx.register_sym(self.ident.clone(), None, sym.can_derive_debug)?;
                self.doc.emit(ctx)?;
                write!(ctx, "pub type ")?;
                self.ident.emit(ctx)?;
                write!(ctx, " = ")?;
                sym.ident.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Enum(e) => {
                ctx.register_sym(self.ident.clone(), None, true)?;

                if let Some(ident) = &e.ident {
                    ctx.scope_mut().register_enum_sym(ident.clone())?;
                }

                self.doc.emit(ctx)?;
                assert!(e.doc.is_none());

                let mut enum_rust_type = None;
                let mut known_values = Vec::new();

                let prefix = if e.variants.len() > 1 {
                    let mut prefix = e
                        .variants
                        .first()
                        .map(|v| v.ident.as_str())
                        .unwrap_or_default();

                    for variant in &e.variants {
                        known_values.push(", ".into());
                        known_values.push(format!("[`{}`]", variant.ident.as_str()));
                        prefix = common_prefix(prefix, variant.ident.as_str(), Some(b'_'));
                    }
                    prefix
                } else {
                    ""
                };

                if !known_values.is_empty() {
                    if self.doc.is_some() {
                        writeln!(ctx, "///")?;
                    }
                    write!(ctx, "/// sdl3-sys note: This is a `C` enum. Known values: ")?;
                    let mut known_values = known_values.into_iter();
                    known_values.next();
                    for s in known_values {
                        ctx.write_str(&s)?;
                    }
                    writeln!(ctx)?;
                }

                let enum_rust_type = enum_rust_type.unwrap_or("::core::ffi::c_int");

                let enum_ident = self.ident.as_str();
                writeln!(ctx, "#[repr(transparent)]")?;
                writeln!(
                    ctx,
                    "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                )?;
                writeln!(
                    ctx,
                    r#"#[cfg_attr(feature = "debug-impls", derive(Debug))]"#
                )?;
                writeln!(ctx, "pub struct {enum_ident}(pub {enum_rust_type});")?;

                let mut impl_consts = String::new();
                let mut ctx_impl = ctx.with_output(&mut impl_consts);
                let mut global_consts = String::new();
                let mut ctx_global = ctx.with_output(&mut global_consts);
                let mut next_expr = Some(Expr::Literal(Literal::Integer(IntegerLiteral::zero())));

                for variant in &e.variants {
                    ctx.register_sym(
                        variant.ident.clone(),
                        Some(Type::ident(self.ident.clone())),
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

                    let mut value = String::new();
                    let mut ctx_value = ctx.with_output(&mut value);
                    expr.emit(&mut ctx_value)?;
                    drop(ctx_value);
                    let need_wrap = if let Expr::Ident(ident) = &expr {
                        if let Some(TypeEnum::Ident(tid)) = ctx
                            .lookup_sym(&ident.clone().try_into().unwrap())
                            .and_then(|s| s.ty)
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

                    variant.doc.emit(&mut ctx_impl)?;
                    writeln!(ctx_impl, "pub const {short_variant_ident}: Self = {value};")?;

                    variant.doc.emit(&mut ctx_global)?;
                    writeln!(ctx_global, "pub const {variant_ident}: {enum_ident} = {enum_ident}::{short_variant_ident};")?;
                }

                drop(ctx_impl);
                drop(ctx_global);

                writeln!(ctx, "impl {enum_ident} {{")?;
                ctx.increase_indent();
                ctx.write_str(&impl_consts)?;
                ctx.decrease_indent();
                writeln!(ctx, "}}")?;
                ctx.write_str(&global_consts)?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Struct(s) => {
                ctx.register_sym(self.ident.clone(), None, s.can_derive_debug(ctx))?;

                s.emit_with_doc_and_ident(ctx, self.doc.clone(), false)?;
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
                ctx.register_sym(self.ident.clone(), None, true)?;
                self.doc.emit(ctx)?;
                write!(ctx, "pub type {} = ", self.ident.as_str())?;
                self.ty.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Array(_, _) => {
                ctx.register_sym(self.ident.clone(), None, true)?;
                self.doc.emit(ctx)?;
                todo!()
            }

            TypeEnum::FnPointer(f) => {
                ctx.register_sym(self.ident.clone(), None, true)?;
                self.doc.emit(ctx)?;
                write!(
                    ctx,
                    "pub type {} = ::core::option::Option<",
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

            TypeEnum::DotDotDot => {
                ctx.register_sym(self.ident.clone(), None, false)?;
                self.doc.emit(ctx)?;
                todo!()
            }

            TypeEnum::Rust(r, can_derive_debug) => {
                ctx.register_sym(self.ident.clone(), None, *can_derive_debug)?;
                writeln!(ctx, "pub type {} = {r};", self.ident.as_str())?;
                Ok(())
            }
        }
    }
}
