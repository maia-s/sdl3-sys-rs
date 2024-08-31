use crate::{
    common_prefix,
    parse::{
        ArgDecl, Define, DocComment, DocCommentFile, Expr, FnAbi, FnDeclArgs, Function, GetSpan,
        Ident, Include, IntegerLiteral, Item, Items, Literal, ParseErr, PreProcBlock,
        PreProcBlockKind, PrimitiveType, Type, TypeDef, TypeEnum,
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
mod state;
use state::PreProcState;
pub use state::{DefineState, EmitContext, InnerEmitContext};

fn emit_extern_start(ctx: &mut EmitContext, abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
    if let Some(abi) = &abi {
        match abi.ident.as_str() {
            "SDLCALL" => {
                if for_fn_ptr {
                    write!(ctx, "extern_sdlcall!(")?
                } else {
                    writeln!(ctx, "extern_sdlcall! {{{{")?;
                    ctx.increase_indent();
                }
                return Ok(());
            }
            "__cdecl" => write!(ctx, "extern \"cdecl\" ")?,
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
    if for_fn_ptr {
        if let Some(abi) = &abi {
            if abi.ident.as_str() == "SDLCALL" {
                write!(ctx, ")")?;
            }
        }
    } else {
        ctx.decrease_indent();
        if let Some(abi) = &abi {
            if abi.ident.as_str() == "SDLCALL" {
                writeln!(ctx, "}}}}")?;
                writeln!(ctx)?;
                return Ok(());
            }
        }
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
            Item::StructOrUnion(_) => todo!(),
            Item::Enum(_) => todo!(),
            Item::Function(f) => f.emit(ctx),
            Item::Expr(e) => e.emit(ctx),
            Item::FnCall(_) => todo!(),
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
                        if bop.op.as_str() == "==" {
                            if let Expr::Ident(lhs) = &bop.lhs {
                                if let Some(rhs) = bop.rhs.try_eval(ctx)? {
                                    value = ctx.try_target_dependent_if_compare(lhs.as_str(), rhs);
                                }
                            }
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
            if let Some(value) = self.value.try_eval(ctx)? {
                match value {
                    Value::I32(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::i32 = {val};")?
                    }
                    Value::U31(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::i32 = {val};")?
                    }
                    Value::U32(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::u32 = {val};")?
                    }
                    Value::I64(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::i64 = {val};")?
                    }
                    Value::U63(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::i64 = {val};")?
                    }
                    Value::U64(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::u64 = {val};")?
                    }
                    Value::F32(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::f32 = {val};")?
                    }
                    Value::F64(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::f64 = {val};")?
                    }
                    Value::Bool(val) => {
                        writeln!(ctx, "pub const {ident}: ::core::primitive::bool = {val};")?
                    }
                    Value::String(val) => {
                        write!(ctx, "pub const {ident}: &::core::ffi::CStr = ")?;
                        val.emit(ctx)?;
                        writeln!(ctx, ";")?;
                    }
                    Value::TargetDependent(_) => todo!(),
                    Value::RustCode(val) => {
                        write!(ctx, "pub const {ident}: ")?;
                        val.ty.emit(ctx)?;
                        writeln!(ctx, " = {val};")?;
                    }
                }
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
            if let Some(included) = &ctx
                .r#gen
                .emitted
                .borrow_mut()
                .get(module)
                .map(|m| Rc::clone(&m.preproc_state))
            {
                ctx.preproc_state()
                    .borrow_mut()
                    .include(&included.borrow())?;
                writeln!(ctx, "use super::{module}::*;")?;
            }
        }
        Ok(())
    }
}

impl Emit for Ident {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        write!(ctx, "{}", self.as_str())?;
        Ok(())
    }
}

impl Emit for Function {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if self.static_kw.is_none() {
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
            Ok(())
        } else {
            todo!()
        }
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
            write!(ctx, "{}: ", ident.as_str())?;
        }
        self.ty.emit(ctx)
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
                PrimitiveType::Bool => {
                    return Err(ParseErr::new(self.span(), "can't emit this type").into())
                }
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

            TypeEnum::Struct(s) => {
                if let Some(ident) = s.ident.as_ref() {
                    if ctx.lookup_struct_sym(ident).is_none() {
                        if s.fields.is_none() {
                            ctx.scope_mut().register_struct_sym(ident.clone(), false)?;
                            write!(ctx, "{}", ident.as_str())?;
                        } else {
                            dbg!(self);
                            todo!()
                        }
                    } else {
                        write!(ctx, "{}", ident.as_str())?;
                    }
                } else {
                    todo!()
                }
            }

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

            TypeEnum::Array(_, _) => todo!(),

            TypeEnum::FnPointer(_) => todo!(),

            TypeEnum::DotDotDot => write!(ctx, "...")?,
        }
        Ok(())
    }
}

impl Emit for TypeDef {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.ty.ty {
            TypeEnum::Primitive(p) => {
                self.doc.emit(ctx)?;
                let p = match p {
                    PrimitiveType::Char => "::core::ffi::c_char",
                    PrimitiveType::SignedChar => "::core::ffi::c_schar",
                    PrimitiveType::UnsignedChar => "::core::ffi::c_uchar",
                    PrimitiveType::Short => "::core::ffi::c_short",
                    PrimitiveType::UnsignedShort => "::core::ffi::c_ushort",
                    PrimitiveType::Int => "::core::ffi::c_int",
                    PrimitiveType::UnsignedInt => "::core::ffi::c_uint",
                    PrimitiveType::Long => "::core::ffi::c_long",
                    PrimitiveType::UnsignedLong => "::core::ffi::c_ulong",
                    PrimitiveType::LongLong => "::core::ffi::c_longlong",
                    PrimitiveType::UnsignedLongLong => "::core::ffi::c_ulonglong",
                    PrimitiveType::Float => "::core::primitive::f32",
                    PrimitiveType::Double => "::core::primitive::f64",
                    PrimitiveType::Void => "()",
                    PrimitiveType::Bool => "::core::primitive::bool",
                    PrimitiveType::SizeT => "::core::primitive::usize",
                    PrimitiveType::Int8T => "::core::primitive::i8",
                    PrimitiveType::Uint8T => "::core::primitive::u8",
                    PrimitiveType::Int16T => "::core::primitive::i16",
                    PrimitiveType::Uint16T => "::core::primitive::u16",
                    PrimitiveType::Int32T => "::core::primitive::i32",
                    PrimitiveType::Uint32T => "::core::primitive::u32",
                    PrimitiveType::Int64T => "::core::primitive::i64",
                    PrimitiveType::Uint64T => "::core::primitive::u64",
                    PrimitiveType::IntPtrT => "::core::primitive::isize",
                    PrimitiveType::UintPtrT => "::core::primitive::usize",
                    PrimitiveType::WcharT => "crate::ffi::c_wchar_t",
                    PrimitiveType::VaList => "crate::ffi::VaList",
                };
                ctx.scope_mut().register_sym(self.ident.clone())?;
                writeln!(ctx, "pub type {} = {p};", self.ident.as_str())?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Ident(sym) => {
                let sym = ctx.lookup_sym(sym).ok_or_else(|| {
                    ParseErr::new(sym.span(), format!("`{}` not defined", sym.as_str()))
                })?;
                ctx.scope_mut().register_sym(self.ident.clone())?;
                self.doc.emit(ctx)?;
                writeln!(ctx, "pub type {} = {};", self.ident.as_str(), sym.as_str())?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Enum(e) => {
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
                        prefix = common_prefix(prefix, variant.ident.as_str());
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

                ctx.scope_mut().register_sym(self.ident.clone())?;
                let enum_ident = self.ident.as_str();
                writeln!(ctx, "#[repr(transparent)]")?;
                writeln!(
                    ctx,
                    "#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                )?;
                writeln!(ctx, "pub struct {enum_ident}(pub {enum_rust_type});")?;

                let mut impl_consts = String::new();
                let mut ctx_impl = ctx.with_output(&mut impl_consts);
                let mut global_consts = String::new();
                let mut ctx_global = ctx.with_output(&mut global_consts);
                let mut next_expr = Some(Expr::Literal(Literal::Integer(IntegerLiteral::zero())));

                for variant in &e.variants {
                    ctx.scope_mut().register_sym(variant.ident.clone())?;
                    let variant_ident = variant.ident.as_str();
                    let short_variant_ident = variant_ident.strip_prefix(prefix).unwrap();
                    variant.doc.emit(&mut ctx_impl)?;
                    write!(ctx_impl, "pub const {short_variant_ident}: Self = Self(")?;
                    next_expr = if let Some(expr) = &variant.expr {
                        expr.emit(&mut ctx_impl)?;
                        expr.try_eval_plus_one(ctx)?.map(Expr::Value)
                    } else if let Some(next_expr) = next_expr {
                        next_expr.emit(&mut ctx_impl)?;
                        next_expr.try_eval_plus_one(ctx)?.map(Expr::Value)
                    } else {
                        return Err(ParseErr::new(
                            variant.ident.span(),
                            "couldn't evaluate value for enum",
                        )
                        .into());
                    };
                    writeln!(ctx_impl, ");")?;
                    variant.doc.emit(&mut ctx_global)?;
                    writeln!(
                        ctx_global,
                        "pub const {variant_ident}: {enum_ident} = {enum_ident}::{short_variant_ident};"
                    )?;
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
                if let Some(ident) = &s.ident {
                    if s.fields.is_none() {
                        if ctx.lookup_struct_sym(ident).is_none() {
                            self.ty.emit(&mut ctx.with_ool_output())?;
                        }
                    } else {
                        ctx.scope_mut().register_struct_sym(ident.clone(), true)?;
                    }
                }

                if let Some(fields) = &s.fields {
                    self.doc.emit(ctx)?;
                    writeln!(ctx, "#[repr(C)]")?;
                    writeln!(ctx, "#[derive(Clone, Copy, Debug)]")?;
                    writeln!(
                        ctx,
                        "pub {} {} {{",
                        if s.is_struct() { "struct" } else { "union" },
                        self.ident
                    )?;
                    ctx.increase_indent();

                    for field in fields.fields.iter() {
                        field.doc.emit(ctx)?;
                        field.ident.emit(ctx)?;
                        write!(ctx, ": ")?;
                        field.ty.emit(ctx)?;
                        writeln!(ctx, ",")?;
                    }

                    ctx.decrease_indent();
                    writeln!(ctx, "}}")?;
                    writeln!(ctx)?;
                }

                ctx.flush_ool_output()?;
                Ok(())
            }

            TypeEnum::Pointer(_) => {
                ctx.scope_mut().register_sym(self.ident.clone())?;
                self.doc.emit(ctx)?;
                write!(ctx, "pub type {} = ", self.ident.as_str())?;
                self.ty.emit(ctx)?;
                writeln!(ctx, ";")?;
                writeln!(ctx)?;
                Ok(())
            }

            TypeEnum::Array(_, _) => {
                self.doc.emit(ctx)?;
                todo!()
            }

            TypeEnum::FnPointer(f) => {
                ctx.scope_mut().register_sym(self.ident.clone())?;
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
                self.doc.emit(ctx)?;
                todo!()
            }
        }
    }
}
