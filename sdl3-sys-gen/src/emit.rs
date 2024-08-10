use crate::{
    common_prefix,
    parse::{
        Define, DocComment, DocCommentFile, Expr, GetSpan, Include, IntegerLiteral, Item, Literal,
        ParseErr, PreProcBlock, TypeDef,
    },
};
use core::fmt::{self, Display, Write};

mod expr;
pub use expr::Value;
mod state;
pub use state::EmitContext;

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
            Item::Function(_) => todo!(),
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
            crate::parse::PreProcBlockKind::If(_) => todo!(),

            crate::parse::PreProcBlockKind::IfDef(ident) => {
                if ctx.preprocstate().is_defined(ident)? {
                    self.block.emit(ctx)
                } else if let Some(else_block) = &self.else_block {
                    else_block.emit(ctx)
                } else {
                    Ok(())
                }
            }

            crate::parse::PreProcBlockKind::IfNDef(ident) => {
                if !ctx.preprocstate().is_defined(ident)? {
                    self.block.emit(ctx)
                } else if let Some(else_block) = &self.else_block {
                    else_block.emit(ctx)
                } else {
                    Ok(())
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
                        expr.try_eval_plus_one(ctx).map(Expr::Value)
                    } else if let Some(next_expr) = next_expr {
                        next_expr.emit(&mut ctx_impl)?;
                        next_expr.try_eval_plus_one(ctx).map(Expr::Value)
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
            crate::parse::TypeEnum::FnPointer(_) => todo!(),
            crate::parse::TypeEnum::DotDotDot => todo!(),
        }
    }
}
