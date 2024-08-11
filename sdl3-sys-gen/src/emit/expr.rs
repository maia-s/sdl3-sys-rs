use super::{Emit, EmitContext, EmitResult};
use crate::parse::{
    Alternative, BinaryOp, Expr, FloatLiteral, GetSpan, IntegerLiteral, Literal, Op, ParseErr,
    Span, StringLiteral,
};
use core::fmt::Write;

#[derive(Clone, Debug)]
pub enum Value {
    I32(i32),
    U31(u32),
    F32(f32),
    F64(f64),
    String(StringLiteral),
}

impl Emit for Value {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Value::I32(i) => write!(ctx, "{i}")?,
            Value::U31(u) => write!(ctx, "{u}")?,
            &Value::F32(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    write!(ctx, "{s}_f32")?
                } else {
                    todo!()
                }
            }
            &Value::F64(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    write!(ctx, "{s}_f64")?
                } else {
                    todo!()
                }
            }
            Value::String(s) => return s.emit(ctx),
        }
        Ok(())
    }
}

impl Expr {
    pub fn try_eval(&self, ctx: &EmitContext) -> Option<Value> {
        match self {
            Expr::Parenthesized(_) => todo!(),
            Expr::Ident(_) => todo!(),

            Expr::Literal(lit) => match lit {
                Literal::Integer(i) => {
                    if i.value <= i32::MAX as u64 {
                        Some(Value::U31(i.value as u32))
                    } else {
                        None
                    }
                }
                Literal::Float(f) => match f {
                    FloatLiteral::Float(f) => Some(Value::F32(f.value)),
                    FloatLiteral::Double(f) => Some(Value::F64(f.value)),
                },
                Literal::String(s) => Some(Value::String(s.clone())),
            },

            Expr::FnCall(_) => todo!(),

            Expr::Ambiguous(amb) => {
                let mut result = None;
                for alt in amb.alternatives.iter() {
                    if let Alternative::Expr(expr) = alt {
                        if let Some(value) = expr.try_eval(ctx) {
                            if result.is_none() {
                                result = Some(value.clone());
                            } else {
                                return None;
                            }
                        }
                    }
                }
                result
            }

            Expr::Cast(_) => todo!(),
            Expr::Asm(_) => None,
            Expr::SizeOf(_) => todo!(),

            Expr::UnaryOp(uop) => {
                let expr = uop.expr.try_eval(ctx)?;

                match uop.op.as_str().as_bytes() {
                    b"+" => match expr {
                        Value::String(_) => None,
                        _ => Some(expr),
                    },

                    b"-" => match expr {
                        Value::I32(value) => Some(Value::I32(value.checked_neg()?)),
                        Value::U31(value) => Some(Value::I32(-(value as i32))),
                        Value::F32(value) => Some(Value::F32(-value)),
                        Value::F64(value) => Some(Value::F64(-value)),
                        _ => None,
                    },

                    _ => None,
                }
            }

            Expr::BinaryOp(bop) => {
                let lhs = bop.lhs.try_eval(ctx)?;
                let rhs = bop.rhs.try_eval(ctx)?;

                match bop.op.as_str().as_bytes() {
                    b"+" => match (lhs, rhs) {
                        (Value::I32(lhs), Value::I32(rhs)) => {
                            Some(Value::I32(lhs.checked_add(rhs)?))
                        }
                        (Value::I32(lhs), Value::U31(rhs)) => {
                            Some(Value::I32(lhs.checked_add(rhs as i32)?))
                        }
                        (Value::U31(lhs), Value::I32(rhs)) => {
                            Some(Value::I32((lhs as i32).checked_add(rhs)?))
                        }
                        (Value::U31(lhs), Value::U31(rhs)) => {
                            let value = lhs.checked_add(rhs)?;
                            if value <= i32::MAX as u32 {
                                Some(Value::U31(lhs.checked_add(rhs)?))
                            } else {
                                todo!()
                            }
                        }
                        (Value::F32(lhs), Value::F32(rhs)) => Some(Value::F32(lhs + rhs)),
                        (Value::F64(lhs), Value::F64(rhs)) => Some(Value::F64(lhs + rhs)),
                        _ => None,
                    },

                    _ => None,
                }
            }

            Expr::PostOp(_) => todo!(),
            Expr::Ternary(_) => todo!(),
            Expr::ArrayIndex { span, base, index } => todo!(),
            Expr::ArrayValues { span, values } => todo!(),

            Expr::Value(value) => Some(value.clone()),
        }
    }

    pub fn try_eval_plus_one(&self, ctx: &EmitContext) -> Option<Value> {
        Expr::BinaryOp(Box::new(BinaryOp {
            op: Op {
                span: Span::new_inline("+"),
            },
            lhs: self.clone(),
            rhs: Expr::Literal(Literal::Integer(IntegerLiteral::one())),
        }))
        .try_eval(ctx)
    }
}

impl Emit for Expr {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Expr::Parenthesized(_) => todo!(),
            Expr::Ident(_) => todo!(),
            Expr::Literal(lit) => lit.emit(ctx),
            Expr::FnCall(_) => todo!(),
            Expr::Ambiguous(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::Asm(_) => todo!(),
            Expr::SizeOf(_) => todo!(),

            Expr::UnaryOp(uop) => {
                if let Some(value) = self.try_eval(ctx) {
                    value.emit(ctx)
                } else {
                    match uop.op.as_str().as_bytes() {
                        b"+" => uop.expr.emit(ctx),
                        b"-" => {
                            write!(ctx, "-(")?;
                            uop.expr.emit(ctx)?;
                            write!(ctx, ")")?;
                            Ok(())
                        }
                        _ => todo!(),
                    }
                }
            }

            Expr::BinaryOp(_) => todo!(),
            Expr::PostOp(_) => todo!(),
            Expr::Ternary(_) => todo!(),
            Expr::ArrayIndex { span, base, index } => todo!(),
            Expr::ArrayValues { span, values } => todo!(),
            Expr::Value(value) => value.emit(ctx),
        }
    }
}

impl Emit for Literal {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Literal::Integer(lit) => lit.emit(ctx),
            Literal::Float(lit) => lit.emit(ctx),
            Literal::String(_) => todo!(),
        }
    }
}

impl Emit for IntegerLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        let (pfx, mut s) = match self.base {
            2 => ("0b", format!("{:b}", self.value)),
            8 => ("0o", format!("{:o}", self.value)),
            10 => ("", format!("{}", self.value)),
            16 => ("0x", format!("{:x}", self.value)),
            _ => return Err(ParseErr::new(self.span(), "can't emit base").into()),
        };
        if s.len() < self.ndigits as usize {
            s = format!("{s}{}", "0".repeat(self.ndigits as usize - s.len()));
        }
        write!(ctx, "{}{}", pfx, s)?;
        Ok(())
    }
}

impl Emit for FloatLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            FloatLiteral::Float(f) => Value::F32(f.value).emit(ctx),
            FloatLiteral::Double(f) => Value::F64(f.value).emit(ctx),
        }
    }
}

impl Emit for StringLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        write!(
            ctx,
            "unsafe {{ ::core::ffi::Cstr::from_bytes_with_nul_unchecked(b\""
        )?;
        for &b in self.str.as_bytes() {
            match b {
                0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7f => ctx.write_char(b as char)?,
                b'\"' | b'\\' => {
                    ctx.write_char('\\')?;
                    ctx.write_char(b as char)?
                }
                _ => write!(ctx, "\\x{:02x}", b)?,
            }
        }
        write!(ctx, "\0\" }})")?;
        Ok(())
    }
}
