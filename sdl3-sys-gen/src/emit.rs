use crate::{
    common_prefix,
    parse::{
        ArgDecl, Define, DocComment, DocCommentFile, Expr, FnAbi, FnDeclArgs, Function, GetSpan,
        Ident, Include, IntegerLiteral, Item, Items, Literal, ParseErr, PreProcBlock, Type,
        TypeDef,
    },
};
use core::fmt::{self, Display, Write};
use std::ops::Deref;

mod expr;
pub use expr::Value;
mod state;
pub use state::{DefineState, EmitContext};

fn emit_extern_start(ctx: &mut EmitContext, abi: &Option<FnAbi>, for_fn_ptr: bool) -> EmitResult {
    if let Some(abi) = &abi {
        match abi.ident.as_str() {
            "SDLCALL" => {
                if for_fn_ptr {
                    write!(ctx, "extern_sdl_ptr!(")?
                } else {
                    writeln!(ctx, "extern_sdl! {{")?
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
        writeln!(ctx, "}}")?;
    }
    Ok(())
}

pub type EmitResult = Result<(), EmitErr>;

#[derive(Debug)]
pub enum EmitErr {
    ParseError(ParseErr),
    FmtError(fmt::Error),
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
        }
    }
}

pub trait Emit {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult;

    fn emit_with_define_state(
        &self,
        ctx: &mut EmitContext,
        define_state: &DefineState,
    ) -> EmitResult {
        todo!(); // ctx.emit_define_state_cfg(define_state)?;
        writeln!(ctx, "emit! {{")?;
        self.emit(ctx)?;
        writeln!(ctx, "}}")?;
        Ok(())
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
            Item::Pragma(_) => todo!(),
            Item::Error(_) => todo!(),
            Item::FileDoc(dc) => dc.emit(ctx),
            Item::StructOrUnion(_) => todo!(),
            Item::Enum(_) => todo!(),
            Item::Function(f) => f.emit(ctx),
            Item::Expr(_) => todo!(),
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
            writeln!(ctx, "/// {}", line)?;
        }
        Ok(())
    }
}

impl Emit for DocCommentFile {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for line in self.0.to_string().lines() {
            writeln!(ctx, "//! {}", line)?;
        }
        writeln!(ctx)?;
        Ok(())
    }
}

impl<const ALLOW_INITIAL_ELSE: bool> Emit for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.kind {
            crate::parse::PreProcBlockKind::If(expr) => {
                let value = {
                    let _eval_mode = ctx.preproc_eval_mode_guard();
                    expr.try_eval(ctx)?
                };
                if let Some(value) = value {
                    if let Value::TargetDependent(define_state) = value {
                        self.block.emit_with_define_state(ctx, &define_state)?;
                        if let Some(else_block) = &self.else_block {
                            else_block.emit_with_define_state(ctx, &define_state.not())?;
                        }
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

            crate::parse::PreProcBlockKind::IfDef(ident) => {
                if ctx.preprocstate().is_target_define(ident) {
                    let define_state = DefineState::defined(ident.clone());
                    self.block.emit_with_define_state(ctx, &define_state)?;
                    if let Some(else_block) = &self.else_block {
                        else_block.emit_with_define_state(ctx, &define_state.not())?;
                    }
                    Ok(())
                } else if ctx.preprocstate().is_defined(ident)? {
                    self.block.emit(ctx)
                } else {
                    self.else_block.emit(ctx)
                }
            }

            crate::parse::PreProcBlockKind::IfNDef(ident) => {
                if ctx.preprocstate().is_target_define(ident) {
                    let define_state = DefineState::defined(ident.clone());
                    self.block
                        .emit_with_define_state(ctx, &define_state.clone().not())?;
                    if let Some(else_block) = &self.else_block {
                        else_block.emit_with_define_state(ctx, &define_state)?;
                    }
                    Ok(())
                } else if !ctx.preprocstate().is_defined(ident)? {
                    self.block.emit(ctx)
                } else {
                    self.else_block.emit(ctx)
                }
            }

            crate::parse::PreProcBlockKind::None => todo!(),
        }
    }
}

impl Emit for Define {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.preprocstate_mut()
            .define(self.ident.clone(), self.args.clone(), self.value.clone())
    }
}

impl Emit for Include {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(module) = self.path.as_str().strip_prefix("SDL3/SDL_") {
            let module = module.strip_suffix(".h").unwrap();
            writeln!(ctx, "use super::{module}::*;")?;
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
            write!(ctx, "fn ")?;
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
            crate::parse::TypeEnum::Primitive(t) => match t {
                crate::parse::PrimitiveType::Char => write!(ctx, "::core::ffi::c_char")?,
                crate::parse::PrimitiveType::SignedChar => write!(ctx, "::core::ffi::c_schar")?,
                crate::parse::PrimitiveType::UnsignedChar => write!(ctx, "::core::ffi::c_uchar")?,
                crate::parse::PrimitiveType::Short => write!(ctx, "::core::ffi::c_short")?,
                crate::parse::PrimitiveType::UnsignedShort => write!(ctx, "::core::ffi::c_ushort")?,
                crate::parse::PrimitiveType::Int => write!(ctx, "::core::ffi::c_int")?,
                crate::parse::PrimitiveType::UnsignedInt => write!(ctx, "::core::ffi::c_uint")?,
                crate::parse::PrimitiveType::Long => write!(ctx, "::core::ffi::c_long")?,
                crate::parse::PrimitiveType::UnsignedLong => write!(ctx, "::core::ffi::c_ulong")?,
                crate::parse::PrimitiveType::LongLong => write!(ctx, "::core::ffi::c_longlong")?,
                crate::parse::PrimitiveType::UnsignedLongLong => {
                    write!(ctx, "::core::ffi::c_ulonglong")?
                }
                crate::parse::PrimitiveType::Float => write!(ctx, "::core::ffi::c_float")?,
                crate::parse::PrimitiveType::Double => write!(ctx, "::core::ffi::c_double")?,
                crate::parse::PrimitiveType::Void => write!(ctx, "()")?,
                crate::parse::PrimitiveType::Bool => {
                    return Err(ParseErr::new(self.span(), "can't emit this type").into())
                }
                crate::parse::PrimitiveType::SizeT => write!(ctx, "::core::primitive::usize")?,
                crate::parse::PrimitiveType::Int8T => write!(ctx, "::core::primitive::i8")?,
                crate::parse::PrimitiveType::Uint8T => write!(ctx, "::core::primitive::u8")?,
                crate::parse::PrimitiveType::Int16T => write!(ctx, "::core::primitive::i16")?,
                crate::parse::PrimitiveType::Uint16T => write!(ctx, "::core::primitive::u16")?,
                crate::parse::PrimitiveType::Int32T => write!(ctx, "::core::primitive::i32")?,
                crate::parse::PrimitiveType::Uint32T => write!(ctx, "::core::primitive::u32")?,
                crate::parse::PrimitiveType::Int64T => write!(ctx, "::core::primitive::i64")?,
                crate::parse::PrimitiveType::Uint64T => write!(ctx, "::core::primitive::u64")?,
                crate::parse::PrimitiveType::IntPtrT => write!(ctx, "::core::primitive::isize")?,
                crate::parse::PrimitiveType::UintPtrT => write!(ctx, "::core::primitive::usize")?,
            },
            crate::parse::TypeEnum::Ident(i) => {
                ctx.use_ident(i)?;
                i.emit(ctx)?;
            }
            crate::parse::TypeEnum::Enum(_) => todo!(),
            crate::parse::TypeEnum::Struct(_) => todo!(),
            crate::parse::TypeEnum::Pointer(p) => {
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
            crate::parse::TypeEnum::Array(_, _) => todo!(),
            crate::parse::TypeEnum::FnPointer(_) => todo!(),
            crate::parse::TypeEnum::DotDotDot => write!(ctx, "...")?,
        }
        Ok(())
    }
}

impl Emit for TypeDef {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.doc.emit(ctx)?;

        match &self.ty.ty {
            crate::parse::TypeEnum::Primitive(_) => todo!(),
            crate::parse::TypeEnum::Ident(_) => todo!(),

            crate::parse::TypeEnum::Enum(e) => {
                assert!(e.doc.is_none());

                let mut enum_rust_type = None;
                let mut known_values = Vec::new();

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

                if !known_values.is_empty() {
                    if self.doc.is_some() {
                        writeln!(ctx, "///")?;
                    }
                    write!(ctx, "/// This is a `C` enum. Known values: ")?;
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

                writeln!(ctx, "impl {enum_ident} {{")?;
                ctx.write_str(&impl_consts)?;
                writeln!(ctx, "}}")?;
                ctx.write_str(&global_consts)?;
                Ok(())
            }

            crate::parse::TypeEnum::Struct(_) => todo!(),
            crate::parse::TypeEnum::Pointer(_) => todo!(),
            crate::parse::TypeEnum::Array(_, _) => todo!(),

            crate::parse::TypeEnum::FnPointer(f) => {
                write!(ctx, "pub type {} = Option<", self.ident.as_str())?;
                emit_extern_start(ctx, &f.abi, true)?;
                write!(ctx, "fn")?;
                f.args.emit(ctx)?;
                if !f.return_type.is_void() {
                    write!(ctx, " -> ")?;
                    f.return_type.emit(ctx)?;
                }
                emit_extern_end(ctx, &f.abi, true)?;
                writeln!(ctx, ">;")?;
                Ok(())
            }

            crate::parse::TypeEnum::DotDotDot => todo!(),
        }
    }
}
