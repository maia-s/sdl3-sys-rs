use super::{EmitContext, EmitErr, Eval, Value};
use crate::{
    emit::Emit,
    parse::{
        Block, DoWhile, GetSpan, IfElse, Item, Items, ParseErr, Return, RustCode, Type, While,
    },
};
use core::fmt::Write;

impl Eval for Items {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let mut is_const = true;
        let mut is_unsafe = false;

        let mut value = String::new();
        let mut ctx_items = ctx.with_output(&mut value);
        for item in self.0.iter() {
            let Some(value) = item.try_eval(ctx)? else {
                return Ok(None);
            };
            if !value.is_const() {
                is_const = false;
            }
            if value.is_unsafe() {
                is_unsafe = true;
            }
            value.emit(&mut ctx_items)?;
        }
        drop(ctx_items);

        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            Type::void(),
            is_const,
            is_unsafe,
        ))))
    }
}

impl Eval for Item {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Item::PreProcBlock(_) => Ok(None),
            Item::Block(_) => todo!(),
            Item::Skipped(_) => todo!(),
            Item::Define(_) => todo!(),
            Item::Undef(_) => todo!(),
            Item::Include(_) => todo!(),
            Item::Pragma(_) => todo!(),
            Item::Error(_) => todo!(),
            Item::Warning(_) => todo!(),
            Item::FileDoc(_) => todo!(),
            Item::StructOrUnion(_) => todo!(),
            Item::Enum(_) => todo!(),
            Item::Function(_) => Ok(None),
            Item::Expr(expr) => {
                let Some(expr) = expr.try_eval(ctx)? else {
                    todo!()
                };
                let value = ctx.capture_output(|ctx| {
                    expr.emit(ctx)?;
                    writeln!(ctx, ";")?;
                    Ok(())
                })?;
                Ok(Some(Value::RustCode(RustCode::boxed(
                    value,
                    Type::void(),
                    expr.is_const(),
                    expr.is_unsafe(),
                ))))
            }
            Item::FnCall(_) => todo!(),
            Item::TypeDef(_) => todo!(),
            Item::VarDecl(_) => Ok(None),
            Item::DoWhile(dw) => dw.try_eval(ctx),
            Item::While(w) => w.try_eval(ctx),
            Item::For(_) => todo!(),
            Item::IfElse(if_else) => if_else.try_eval(ctx),
            Item::Return(ret) => ret.try_eval(ctx),
            Item::EnumVariant(_) => todo!(),
            Item::Break(_) => todo!(),
            Item::Continue(_) => todo!(),
        }
    }
}

impl Eval for Block {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let _guard = ctx.subscope_guard();
        let Some(items) = self.items.try_eval(ctx)? else {
            return Ok(None);
        };
        let value = ctx.capture_output(|ctx| {
            writeln!(ctx, "{{")?;
            ctx.increase_indent();
            items.emit(ctx)?;
            ctx.decrease_indent();
            write!(ctx, "}}")?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            items.ty()?,
            items.is_const(),
            items.is_unsafe(),
        ))))
    }
}

impl Eval for DoWhile {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let err = || ParseErr::new(self.span(), "can't eval");
        let cond = self.cond.try_eval(ctx)?.ok_or_else(err)?;
        let cond = cond.coerce(ctx, &Type::bool())?.unwrap_or(cond);
        let block = self.block.try_eval(ctx)?.ok_or_else(err)?;
        let value = ctx.capture_output(|ctx| {
            // written so that break and continue works as expected
            writeln!(ctx, "{{")?;
            ctx.increase_indent();
            writeln!(ctx, "let mut __sdl3sysgen_first = true;")?;
            write!(ctx, "while __sdl3sysgen_first || ")?;
            cond.emit(ctx)?;
            write!(ctx, " {{")?;
            ctx.increase_indent();
            write!(ctx, "__sdl3sysgen_first = false;")?;
            write!(ctx, "let __sdl3sysgen_first = ();")?;
            block.emit(ctx)?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            Type::void(),
            cond.is_const() && block.is_const(),
            cond.is_unsafe() || block.is_unsafe(),
        ))))
    }
}

impl Eval for IfElse {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let Some(cond) = self.cond.try_eval(ctx)? else {
            return Ok(None);
        };
        let Some(on_true) = self.on_true.try_eval(ctx)? else {
            todo!()
        };
        let on_false = if let Some(on_false) = &self.on_false {
            let Some(on_false) = on_false.try_eval(ctx)? else {
                todo!()
            };
            Some(on_false)
        } else {
            None
        };
        let value = ctx.capture_output(|ctx| {
            write!(ctx, "if ")?;
            cond.emit(ctx)?;
            on_true.emit(ctx)?;
            if let Some(on_false) = &on_false {
                write!(ctx, " else ")?;
                on_false.emit(ctx)?;
            }
            writeln!(ctx)?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            Type::void(),
            cond.is_const()
                && on_true.is_const()
                && on_false.as_ref().map(|v| v.is_const()).unwrap_or(true),
            cond.is_unsafe()
                || on_true.is_unsafe()
                || on_false.as_ref().map(|v| v.is_unsafe()).unwrap_or(false),
        ))))
    }
}

impl Eval for Return {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let Some(value) = self.expr.try_eval(ctx)? else {
            return Err(ParseErr::new(self.span(), "couldn't evaluate expression").into());
        };
        let val = value
            .coerce(ctx, &ctx.function_return_type())?
            .unwrap_or(value);
        let value = ctx.capture_output(|ctx| {
            write!(ctx, "return ")?;
            val.emit(ctx)?;
            writeln!(ctx, ";")?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            Type::void(),
            val.is_const(),
            val.is_unsafe(),
        ))))
    }
}

impl Eval for While {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let err = || ParseErr::new(self.span(), "can't eval");
        let cond = self.cond.try_eval(ctx)?.ok_or_else(err)?;
        let cond = cond.coerce(ctx, &Type::bool())?.unwrap_or(cond);
        let block = self.block.try_eval(ctx)?.ok_or_else(err)?;
        let value = ctx.capture_output(|ctx| {
            write!(ctx, "while ")?;
            cond.emit(ctx)?;
            write!(ctx, " ")?;
            block.emit(ctx)?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            Type::void(),
            cond.is_const() && block.is_const(),
            cond.is_unsafe() || block.is_unsafe(),
        ))))
    }
}
