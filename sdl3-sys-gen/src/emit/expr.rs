use super::{DefineState, Emit, EmitContext, EmitErr, EmitResult, Eval};
use crate::parse::{
    Alternative, Ambiguous, BinaryOp, DefineValue, Expr, FloatLiteral, GetSpan, IntegerLiteral,
    Literal, Op, ParseErr, Span, StringLiteral,
};
use core::fmt::Write;

#[derive(Clone, Debug)]
pub enum Value {
    I32(i32),
    U31(u32),
    F32(f32),
    F64(f64),
    String(StringLiteral),
    TargetDependent(DefineState),
}

impl Value {
    pub fn bool(value: bool) -> Self {
        Value::U31(value as u32)
    }

    pub fn is_target_dependent(&self) -> bool {
        matches!(self, Self::TargetDependent(_))
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            &Value::I32(i) => i != 0,
            &Value::U31(u) => u != 0,
            &Value::F32(f) => !f.is_nan() && f != 0.0,
            &Value::F64(f) => !f.is_nan() && f != 0.0,
            Value::String(_) => true,
            Value::TargetDependent(_) => unreachable!(),
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
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
            Value::TargetDependent(_) => {
                return Err(ParseErr::new(Span::none(), "can't emit indeterminate value").into())
            }
        }
        Ok(())
    }
}

impl Eval for DefineValue {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Self::Expr(expr) => expr.try_eval(ctx),
            Self::Ambiguous(amb) => amb.try_eval(ctx),
            _ => todo!(),
        }
    }
}

impl Eval for Ambiguous {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        for alt in self.alternatives.iter() {
            if let Alternative::Expr(expr) = alt {
                if let Ok(Some(value)) = expr.try_eval(ctx) {
                    return Ok(Some(value));
                }
            }
        }
        Ok(None)
    }
}

impl Expr {
    pub fn try_eval_plus_one(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
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

impl Eval for Expr {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Expr::Parenthesized(p) => return p.expr.try_eval(ctx),

            Expr::Ident(ident) => {
                if ctx.is_preproc_eval_mode() {
                    return if let Some((args, value)) =
                        ctx.preprocstate().lookup(&ident.clone().try_into()?)?
                    {
                        if args.is_some() {
                            return Err(ParseErr::new(
                                self.span(),
                                "define with arguments used without arguments",
                            )
                            .into());
                        }
                        value.try_eval(ctx)
                    } else {
                        Err(ParseErr::new(ident.span(), "undefined macro").into())
                    };
                }
            }

            Expr::Literal(lit) => {
                return match lit {
                    Literal::Integer(i) => {
                        if i.value <= i32::MAX as u64 {
                            Ok(Some(Value::U31(i.value as u32)))
                        } else {
                            Err(ParseErr::new(i.span(), "value out of range for U31").into())
                        }
                    }
                    Literal::Float(f) => match f {
                        FloatLiteral::Float(f) => Ok(Some(Value::F32(f.value))),
                        FloatLiteral::Double(f) => Ok(Some(Value::F64(f.value))),
                    },
                    Literal::String(s) => Ok(Some(Value::String(s.clone()))),
                }
            }

            Expr::FnCall(call) => {
                if ctx.is_preproc_eval_mode() {
                    if let Expr::Ident(ident) = &*call.func {
                        if ident.as_str() == "defined" {
                            let args = &*call.args;
                            let err = || {
                                Err(ParseErr::new(
                                    call.span(),
                                    "defined() in #if takes one argument of type ident",
                                )
                                .into())
                            };
                            if args.len() != 1 {
                                return err();
                            }
                            let Expr::Ident(define) = &args[0] else {
                                return err();
                            };
                            let define = define.clone().try_into()?;
                            return if ctx.preprocstate().is_target_define(&define) {
                                Ok(Some(Value::TargetDependent(DefineState::defined(define))))
                            } else if ctx.preprocstate().is_defined(&define)? {
                                Ok(Some(Value::bool(true)))
                            } else {
                                Ok(Some(Value::bool(false)))
                            };
                        }
                    }
                }
            }

            Expr::Ambiguous(amb) => {
                let mut result = None;
                for alt in amb.alternatives.iter() {
                    if let Alternative::Expr(expr) = alt {
                        if let Some(value) = expr.try_eval(ctx)? {
                            if result.is_none() {
                                result = Some(value.clone());
                            } else {
                                return Ok(None);
                            }
                        }
                    }
                }
                return Ok(result);
            }

            Expr::Cast(_) => (),
            Expr::Asm(_) => return Ok(None),
            Expr::SizeOf(_) => (),

            Expr::UnaryOp(uop) => {
                let expr = uop.expr.try_eval(ctx)?;
                let Some(expr) = expr else { return Ok(None) };

                return match uop.op.as_str().as_bytes() {
                    b"!" => match expr {
                        Value::String(_) => Ok(None),
                        Value::TargetDependent(ds) => Ok(Some(Value::TargetDependent(ds.not()))),
                        _ => Ok(Some(Value::bool(expr.is_falsy()))),
                    },

                    b"+" => match expr {
                        Value::String(_) => Ok(None),
                        _ => Ok(Some(expr)),
                    },

                    b"-" => match expr {
                        Value::I32(value) => {
                            Ok(Some(Value::I32(value.checked_neg().ok_or_else(|| {
                                ParseErr::new(uop.span(), "can't negate INT_MIN")
                            })?)))
                        }
                        Value::U31(value) => Ok(Some(Value::I32(-(value as i32)))),
                        Value::F32(value) => Ok(Some(Value::F32(-value))),
                        Value::F64(value) => Ok(Some(Value::F64(-value))),
                        Value::TargetDependent(_) => Ok(Some(expr)),
                        _ => Ok(None),
                    },

                    _ => Err(ParseErr::new(uop.span(), "missing implementation for eval").into()),
                };
            }

            Expr::BinaryOp(bop) => {
                macro_rules! eval {
                    ($expr:expr) => {{
                        let Some(value) = $expr.try_eval(ctx)? else {
                            return Ok(None);
                        };
                        value
                    }};
                }

                macro_rules! calc {
                    ($op:tt) => {
                        match (eval!(bop.lhs), eval!(bop.rhs)) {
                            (Value::I32(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::I32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs, rhs as i32)))))
                            }
                            (Value::U31(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs as i32, rhs)))))
                            }
                            (Value::U31(lhs), Value::U31(rhs)) => {
                                let value = calc!(@ checked $op (lhs, rhs));
                                if value <= i32::MAX as u32 {
                                    Ok(Some(Value::U31(value)))
                                } else {
                                    todo!()
                                }
                            }
                            (Value::F32(lhs), Value::F32(rhs)) => Ok(Some(Value::F32(lhs $op rhs))),
                            (Value::F64(lhs), Value::F64(rhs)) => Ok(Some(Value::F64(lhs $op rhs))),
                            _ => {
                                Err(ParseErr::new(bop.span(), "missing implementation for eval").into())
                            }
                        }
                    };

                    (@ checked + ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_add($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };
                }

                macro_rules! compare {
                    ($op:tt) => {
                        match (eval!(bop.lhs), eval!(bop.rhs)) {
                            (Value::I32(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::I32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs as i32) as u32)))
                            }
                            (Value::U31(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::U31(((lhs as i32) $op rhs) as u32)))
                            }
                            (Value::U31(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::F32(lhs), Value::F32(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::F64(lhs), Value::F64(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            _ => Err(ParseErr::new(bop.span(), concat!("invalid operands to `", stringify!($op), "`")).into()),
                        }
                    };
                }

                return match bop.op.as_str().as_bytes() {
                    b"+" => calc!(+),

                    b"&&" => {
                        let lhs = eval!(bop.lhs);
                        if let Value::TargetDependent(lhs) = lhs {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(lhs.all(rhs))))
                            } else if rhs.is_truthy() {
                                Ok(Some(Value::TargetDependent(lhs)))
                            } else {
                                Ok(Some(Value::bool(false)))
                            }
                        } else if lhs.is_truthy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::bool(false)))
                        }
                    }

                    b"||" => {
                        let lhs = eval!(bop.lhs);
                        if let Value::TargetDependent(lhs) = lhs {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(lhs.any(rhs))))
                            } else if rhs.is_falsy() {
                                Ok(Some(Value::TargetDependent(lhs)))
                            } else {
                                Ok(Some(Value::bool(true)))
                            }
                        } else if lhs.is_falsy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::bool(true)))
                        }
                    }

                    b"==" => compare!(==),
                    b"!=" => compare!(!=),
                    b"<" => compare!(<),
                    b"<=" => compare!(<=),
                    b">" => compare!(>),
                    b">=" => compare!(>=),

                    _ => {
                        return Err(
                            ParseErr::new(bop.span(), "missing implementation for eval").into()
                        )
                    }
                };
            }

            Expr::PostOp(_) => (),
            Expr::Ternary(_) => (),
            Expr::ArrayIndex { span, base, index } => (),
            Expr::ArrayValues { span, values } => (),

            Expr::Value(value) => return Ok(Some(value.clone())),
        }

        Err(ParseErr::new(self.span(), "missing implementation for eval").into())
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
                if let Some(value) = self.try_eval(ctx)? {
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
