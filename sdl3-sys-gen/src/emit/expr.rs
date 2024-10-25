use super::{
    patch::patch_eval_macro_call, patch_emit_macro_call, DefineState, Emit, EmitContext, EmitErr,
    EmitResult, Eval, Sym,
};
use crate::parse::{
    Alternative, Ambiguous, BinaryOp, Cast, DefineValue, Expr, FloatLiteral, FnCall, GetSpan,
    Ident, IntegerLiteral, IntegerLiteralType, Literal, Op, Parenthesized, ParseErr, PrimitiveType,
    RustCode, RustType, SizeOf, Span, StringLiteral, Ternary, Type, TypeEnum,
};
use core::fmt::{self, Display, Write};

pub const STRING_TYPE: &str = "*const ::core::ffi::c_char";

#[derive(Clone, Debug)]
pub enum Value {
    I32(i32),
    U31(u32),
    U32(u32),
    I64(i64),
    U63(u64),
    U64(u64),
    Usize(usize),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(StringLiteral),
    TargetDependent(DefineState),
    RustCode(Box<RustCode>),
    Place(Box<Value>, Option<Ident>),
}

#[derive(Clone, Copy, Debug)]
pub enum Promoted {
    None = 0,
    Equal,
    Left,
    Right,
    Both,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            &Value::I32(i) => i != 0,
            &Value::U31(u) => u != 0,
            &Value::U32(u) => u != 0,
            &Value::I64(i) => i != 0,
            &Value::U63(u) => u != 0,
            &Value::U64(u) => u != 0,
            &Value::Usize(u) => u != 0,
            &Value::F32(f) => !f.is_nan() && f != 0.0,
            &Value::F64(f) => !f.is_nan() && f != 0.0,
            &Value::Bool(b) => b,
            Value::String(_) => true,
            Value::TargetDependent(_) | Value::RustCode(_) | Value::Place(_, _) => unreachable!(),
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn is_const(&self) -> bool {
        match self {
            Value::I32(_)
            | Value::U31(_)
            | Value::U32(_)
            | Value::I64(_)
            | Value::U63(_)
            | Value::U64(_)
            | Value::Usize(_)
            | Value::F32(_)
            | Value::F64(_)
            | Value::Bool(_)
            | Value::String(_) => true,
            Value::TargetDependent(_) => false,
            Value::RustCode(rc) => rc.is_const,
            Value::Place(ptr, _) => ptr.is_const(),
        }
    }

    pub fn is_unsafe(&self) -> bool {
        match self {
            Value::I32(_)
            | Value::U31(_)
            | Value::U32(_)
            | Value::I64(_)
            | Value::U63(_)
            | Value::U64(_)
            | Value::Usize(_)
            | Value::F32(_)
            | Value::F64(_)
            | Value::Bool(_)
            | Value::String(_)
            | Value::TargetDependent(_) => false,
            Value::RustCode(rc) => rc.is_unsafe,
            Value::Place(ptr, _) => ptr.is_unsafe(),
        }
    }

    pub fn ty(&self) -> Result<Type, EmitErr> {
        match self {
            Value::I32(_) => Ok(Type::primitive(PrimitiveType::Int32T)),
            Value::U31(_) => Ok(Type::primitive(PrimitiveType::Int32T)),
            Value::U32(_) => Ok(Type::primitive(PrimitiveType::Uint32T)),
            Value::I64(_) => Ok(Type::primitive(PrimitiveType::Int64T)),
            Value::U63(_) => Ok(Type::primitive(PrimitiveType::Int64T)),
            Value::U64(_) => Ok(Type::primitive(PrimitiveType::Uint64T)),
            Value::Usize(_) => Ok(Type::primitive(PrimitiveType::SizeT)),
            Value::F32(_) => Ok(Type::primitive(PrimitiveType::Float)),
            Value::F64(_) => Ok(Type::primitive(PrimitiveType::Double)),
            Value::Bool(_) => Ok(Type::bool()),
            Value::String(_) => Ok(Type::rust(RustType {
                string: STRING_TYPE.into(),
                can_derive_copy: true,
                can_derive_debug: true,
            })),
            Value::RustCode(r) => Ok(r.ty.clone()),
            Value::TargetDependent(_) => todo!(),
            Value::Place(ptr, _) => {
                let Some(Type {
                    ty: TypeEnum::Pointer(p),
                    ..
                }) = ptr.ty()?.inner_ty()
                else {
                    return Err(ParseErr::new(
                        ptr.ty()?.span(),
                        "can't get type of place of unknown type",
                    )
                    .into());
                };
                Ok(*p)
            }
        }
    }

    pub fn promote(ctx: &EmitContext, lhs: &mut Self, rhs: &mut Self) -> Result<Promoted, EmitErr> {
        macro_rules! set {
            ($r:ident, $s:ident, $v:ident, $t:ident) => {{
                *$s = Value::$t(*$v as _);
                Ok(Promoted::$r)
            }};
        }
        match (&*lhs, &*rhs) {
            (Value::Place(_, _), _) => {
                *lhs = lhs.clone().deref_read(ctx)?;
                Value::promote(ctx, lhs, rhs)
            }
            (_, Value::Place(_, _)) => {
                *rhs = rhs.clone().deref_read(ctx)?;
                Value::promote(ctx, lhs, rhs)
            }
            (Value::I32(_), Value::I32(_)) => Ok(Promoted::Equal),
            (Value::I32(_), Value::U31(rhv)) => set!(Right, rhs, rhv, I32),
            (Value::I32(lhv), Value::U32(_)) => set!(Left, lhs, lhv, U32),
            (Value::I32(lhv), Value::I64(_)) => set!(Left, lhs, lhv, I64),
            (Value::I32(lhv), Value::U63(rhv)) => {
                *lhs = Value::I64(*lhv as _);
                *rhs = Value::I64(*rhv as _);
                Ok(Promoted::Both)
            }
            (Value::I32(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::I32(lhv), Value::Usize(_)) => set!(Left, lhs, lhv, Usize),
            (Value::I32(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::I32(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::I32(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, I32),

            (Value::U31(lhv), Value::I32(_)) => set!(Left, lhs, lhv, I32),
            (Value::U31(_), Value::U31(_)) => Ok(Promoted::Equal),
            (Value::U31(lhv), Value::U32(_)) => set!(Left, lhs, lhv, U32),
            (Value::U31(lhv), Value::I64(_)) => set!(Left, lhs, lhv, I64),
            (Value::U31(lhv), Value::U63(_)) => set!(Left, lhs, lhv, U63),
            (Value::U31(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::U31(lhv), Value::Usize(_)) => set!(Left, lhs, lhv, Usize),
            (Value::U31(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::U31(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::U31(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, U31),

            (Value::U32(_), Value::I32(rhv)) => set!(Right, rhs, rhv, U32),
            (Value::U32(_), Value::U31(rhv)) => set!(Right, rhs, rhv, U32),
            (Value::U32(_), Value::U32(_)) => Ok(Promoted::Equal),
            (Value::U32(lhv), Value::I64(_)) => set!(Left, lhs, lhv, I64),
            (Value::U32(lhv), Value::U63(_)) => set!(Left, lhs, lhv, U63),
            (Value::U32(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::U32(lhv), Value::Usize(_)) => set!(Left, lhs, lhv, Usize),
            (Value::U32(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::U32(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::U32(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, U32),

            (Value::I64(_), Value::I32(rhv)) => set!(Right, rhs, rhv, I64),
            (Value::I64(_), Value::U31(rhv)) => set!(Right, rhs, rhv, I64),
            (Value::I64(_), Value::U32(rhv)) => set!(Right, rhs, rhv, I64),
            (Value::I64(_), Value::I64(_)) => Ok(Promoted::Equal),
            (Value::I64(_), Value::U63(rhv)) => set!(Right, rhs, rhv, I64),
            (Value::I64(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::I64(_), Value::Usize(_)) => Ok(Promoted::None),
            (Value::I64(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::I64(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::I64(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, I64),

            (Value::U63(lhv), Value::I32(rhv)) => {
                *lhs = Value::I64(*lhv as _);
                *rhs = Value::I64(*rhv as _);
                Ok(Promoted::Both)
            }
            (Value::U63(_), Value::U31(rhv)) => set!(Right, rhs, rhv, U63),
            (Value::U63(_), Value::U32(rhv)) => set!(Right, rhs, rhv, U63),
            (Value::U63(lhv), Value::I64(_)) => set!(Left, lhs, lhv, I64),
            (Value::U63(_), Value::U63(_)) => Ok(Promoted::Equal),
            (Value::U63(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::U63(_), Value::Usize(_)) => Ok(Promoted::None),
            (Value::U63(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::U63(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::U63(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, U63),

            (Value::U64(_), Value::I32(rhv)) => set!(Right, rhs, rhv, U64),
            (Value::U64(_), Value::U31(rhv)) => set!(Right, rhs, rhv, U64),
            (Value::U64(_), Value::U32(rhv)) => set!(Right, rhs, rhv, U64),
            (Value::U64(_), Value::I64(rhv)) => set!(Right, rhs, rhv, U64),
            (Value::U64(_), Value::U63(rhv)) => set!(Right, rhs, rhv, U64),
            (Value::U64(_), Value::U64(_)) => Ok(Promoted::Equal),
            (Value::U64(_), Value::Usize(_)) => Ok(Promoted::None),
            (Value::U64(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::U64(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::U64(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, U64),

            (Value::Usize(_), Value::I32(rhv)) => set!(Right, rhs, rhv, Usize),
            (Value::Usize(_), Value::U31(rhv)) => set!(Right, rhs, rhv, Usize),
            (Value::Usize(_), Value::U32(rhv)) => set!(Right, rhs, rhv, Usize),
            (Value::Usize(_), Value::I64(_)) => Ok(Promoted::None),
            (Value::Usize(_), Value::U63(_)) => Ok(Promoted::None),
            (Value::Usize(_), Value::U64(_)) => Ok(Promoted::None),
            (Value::Usize(_), Value::Usize(_)) => Ok(Promoted::Equal),
            (Value::Usize(lhv), Value::F32(_)) => set!(Left, lhs, lhv, F32),
            (Value::Usize(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::Usize(_), Value::Bool(rhv)) => set!(Right, rhs, rhv, Usize),

            (Value::F32(_), Value::I32(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::U31(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::U32(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::I64(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::U63(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::U64(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::Usize(rhv)) => set!(Right, rhs, rhv, F32),
            (Value::F32(_), Value::F32(_)) => Ok(Promoted::Equal),
            (Value::F32(lhv), Value::F64(_)) => set!(Left, lhs, lhv, F64),
            (Value::F32(_), Value::Bool(rhv)) => {
                *rhs = Value::F32(if *rhv { 1.0 } else { 0.0 });
                Ok(Promoted::Right)
            }

            (Value::F64(_), Value::I32(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::U31(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::U32(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::I64(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::U63(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::U64(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::Usize(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::F32(rhv)) => set!(Right, rhs, rhv, F64),
            (Value::F64(_), Value::F64(_)) => Ok(Promoted::Equal),
            (Value::F64(_), Value::Bool(rhv)) => {
                *rhs = Value::F64(if *rhv { 1.0 } else { 0.0 });
                Ok(Promoted::Right)
            }

            (Value::Bool(lhv), Value::I32(_)) => set!(Left, lhs, lhv, I32),
            (Value::Bool(lhv), Value::U31(_)) => set!(Left, lhs, lhv, U31),
            (Value::Bool(lhv), Value::U32(_)) => set!(Left, lhs, lhv, U32),
            (Value::Bool(lhv), Value::I64(_)) => set!(Left, lhs, lhv, I64),
            (Value::Bool(lhv), Value::U63(_)) => set!(Left, lhs, lhv, U63),
            (Value::Bool(lhv), Value::U64(_)) => set!(Left, lhs, lhv, U64),
            (Value::Bool(lhv), Value::Usize(_)) => set!(Left, lhs, lhv, Usize),
            (Value::Bool(lhv), Value::F32(_)) => {
                *lhs = Value::F32(if *lhv { 1.0 } else { 0.0 });
                Ok(Promoted::Left)
            }
            (Value::Bool(lhv), Value::F64(_)) => {
                *lhs = Value::F64(if *lhv { 1.0 } else { 0.0 });
                Ok(Promoted::Left)
            }
            (Value::Bool(_), Value::Bool(_)) => Ok(Promoted::Equal),
            (Value::Bool(_), _) => Ok(Promoted::None),

            (Value::String(_), Value::String(_)) => Ok(Promoted::Equal),

            (Value::String(_) | Value::TargetDependent(_), _)
            | (_, Value::String(_) | Value::TargetDependent(_)) => Ok(Promoted::None),

            (Value::RustCode(lhv), Value::RustCode(rhv)) => {
                if lhv.ty.is_c_enum(ctx)?.is_some() {
                    *lhs = lhs.coerce_to_int(ctx)?.unwrap();
                    return Value::promote(ctx, lhs, rhs);
                }
                if rhv.ty.is_c_enum(ctx)?.is_some() {
                    *rhs = rhs.coerce_to_int(ctx)?.unwrap();
                    return Value::promote(ctx, lhs, rhs);
                }
                if let Some(lt) = lhv.ty.inner_ty() {
                    if let Some(rt) = rhv.ty.inner_ty() {
                        if lt == rt {
                            Ok(Promoted::Equal)
                        } else if let (Some(mut lc), Some(mut rc)) = (
                            lt.conjure_primitive_value(ctx)?,
                            rt.conjure_primitive_value(ctx)?,
                        ) {
                            let p = Value::promote(ctx, &mut lc, &mut rc)?;
                            if matches!(p, Promoted::Left | Promoted::Both) {
                                let Some(cast) =
                                    Cast::boxed(Span::none(), rt, Expr::Value(lhs.clone()))
                                        .try_eval(ctx)?
                                else {
                                    unreachable!()
                                };
                                *lhs = cast;
                            }
                            if matches!(p, Promoted::Right | Promoted::Both) {
                                let Some(cast) =
                                    Cast::boxed(Span::none(), lt, Expr::Value(rhs.clone()))
                                        .try_eval(ctx)?
                                else {
                                    unreachable!()
                                };
                                *rhs = cast;
                            }
                            Ok(p)
                        } else {
                            Ok(Promoted::None)
                        }
                    } else {
                        rhv.ty.resolve_to(lt);
                        Value::promote(ctx, lhs, rhs)
                    }
                } else if let Some(rt) = rhv.ty.inner_ty() {
                    lhv.ty.resolve_to(rt);
                    Value::promote(ctx, lhs, rhs)
                } else {
                    Ok(Promoted::None)
                }
            }
            (Value::RustCode(lhv), _) => {
                if lhv.ty.is_c_enum(ctx)?.is_some() {
                    *lhs = lhs.coerce_to_int(ctx)?.unwrap();
                    return Value::promote(ctx, lhs, rhs);
                }
                if let Some(lt) = lhv.ty.inner_ty() {
                    if let Some(mut lc) = lt.conjure_primitive_value(ctx)? {
                        match Value::promote(ctx, &mut lc, rhs)? {
                            Promoted::None => Ok(Promoted::None),
                            Promoted::Equal => Ok(Promoted::Equal),
                            p @ Promoted::Left | p @ Promoted::Both => {
                                let Some(cast) =
                                    Cast::boxed(Span::none(), rhs.ty()?, Expr::Value(lhs.clone()))
                                        .try_eval(ctx)?
                                else {
                                    unreachable!()
                                };
                                *lhs = cast;
                                Ok(p)
                            }
                            Promoted::Right => Ok(Promoted::Right),
                        }
                    } else {
                        Ok(Promoted::None)
                    }
                } else {
                    lhv.ty.resolve_to(rhs.ty()?);
                    Value::promote(ctx, lhs, rhs)
                }
            }
            (_, Value::RustCode(rhv)) => {
                if rhv.ty.is_c_enum(ctx)?.is_some() {
                    *rhs = rhs.coerce_to_int(ctx)?.unwrap();
                    return Value::promote(ctx, lhs, rhs);
                }
                if let Some(rt) = rhv.ty.inner_ty() {
                    if let Some(mut rc) = rt.conjure_primitive_value(ctx)? {
                        match Value::promote(ctx, lhs, &mut rc)? {
                            Promoted::None => Ok(Promoted::None),
                            Promoted::Equal => Ok(Promoted::Equal),
                            Promoted::Left => Ok(Promoted::Left),
                            p @ Promoted::Right | p @ Promoted::Both => {
                                let Some(cast) =
                                    Cast::boxed(Span::none(), lhs.ty()?, Expr::Value(rhs.clone()))
                                        .try_eval(ctx)?
                                else {
                                    unreachable!()
                                };
                                *rhs = cast;
                                Ok(p)
                            }
                        }
                    } else {
                        Ok(Promoted::None)
                    }
                } else {
                    rhv.ty.resolve_to(lhs.ty()?);
                    Value::promote(ctx, lhs, rhs)
                }
            }
        }
    }

    pub fn coerce(&self, ctx: &EmitContext, target: &Type) -> Result<Option<Value>, EmitErr> {
        let Some(target) = target.inner_ty() else {
            return Ok(None);
        };

        let target_bool = matches!(
            target,
            Type {
                ty: TypeEnum::Primitive(PrimitiveType::Bool),
                ..
            }
        );
        let is_const;
        let is_unsafe;

        if let Ok(ty) = self.ty() {
            if ty == target {
                return Ok(Some(self.clone()));
            }
        }

        match self {
            Value::I32(_)
            | Value::U31(_)
            | Value::U32(_)
            | Value::I64(_)
            | Value::U63(_)
            | Value::U64(_) => {
                if target_bool {
                    let out = ctx.capture_output(|ctx| {
                        write!(ctx, "(")?;
                        self.emit(ctx)?;
                        write!(ctx, " != 0)")?;
                        Ok(())
                    })?;
                    return Ok(Some(Value::RustCode(RustCode::boxed(
                        out,
                        target.clone(),
                        true,
                        false,
                    ))));
                }
            }

            Value::RustCode(rc) => {
                if rc.ty.is_uninferred() {
                    rc.ty.resolve_to(target.clone());
                }
                let ty = rc.ty.inner_ty().unwrap();
                is_const = rc.is_const;
                is_unsafe = rc.is_unsafe;

                if let Some(sym) = rc.ty.is_c_enum(ctx)? {
                    // coerce enums to their value type
                    let Some(inner_ty) = &sym.enum_base_ty else {
                        unreachable!()
                    };
                    if target_bool || inner_ty.compatible_passing_to(&target) {
                        let out = ctx.capture_output(|ctx| {
                            write!(ctx, "(")?;
                            self.emit(ctx)?;
                            write!(ctx, ".0")?;
                            if target_bool {
                                write!(ctx, " != 0")?;
                            }
                            write!(ctx, ")")?;
                            Ok(())
                        })?;
                        return Ok(Some(Value::RustCode(RustCode::boxed(
                            out,
                            target.clone(),
                            is_const,
                            is_unsafe,
                        ))));
                    }
                } else if target_bool {
                    if matches!(
                        ty,
                        Type {
                            ty: TypeEnum::Pointer(_),
                            ..
                        }
                    ) {
                        let value = ctx.capture_output(|ctx| {
                            write!(ctx, "!")?;
                            self.emit(ctx)?;
                            write!(ctx, ".is_null()")?;
                            Ok(())
                        })?;
                        return Ok(Some(Value::RustCode(RustCode::boxed(
                            value,
                            target.clone(),
                            false, // FIXME: can't compare pointers in const
                            is_unsafe,
                        ))));
                    } else if !matches!(
                        ty,
                        Type {
                            ty: TypeEnum::Primitive(PrimitiveType::Bool),
                            ..
                        }
                    ) {
                        let value = ctx.capture_output(|ctx| {
                            write!(ctx, "(")?;
                            self.emit(ctx)?;
                            write!(ctx, " != 0)")?;
                            Ok(())
                        })?;
                        return Ok(Some(Value::RustCode(RustCode::boxed(
                            value,
                            target.clone(),
                            is_const,
                            is_unsafe,
                        ))));
                    }
                }
            }

            _ => (),
        }

        Ok(None)
    }

    pub fn coerce_to_int(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        if let Self::RustCode(rc) = self {
            if let Some(sym) = rc.ty.is_c_enum(ctx)? {
                // coerce enums to their value type
                let Some(inner_ty) = &sym.enum_base_ty else {
                    unreachable!()
                };
                let out = ctx.capture_output(|ctx| {
                    self.emit(ctx)?;
                    write!(ctx, ".0")?;
                    Ok(())
                })?;
                return Ok(Some(Value::RustCode(RustCode::boxed(
                    out,
                    inner_ty.clone(),
                    rc.is_const,
                    rc.is_unsafe,
                ))));
            } else if let Some(ty) = rc.ty.inner_ty() {
                #[allow(clippy::single_match)]
                match ty.ty {
                    TypeEnum::Primitive(
                        PrimitiveType::Int8T
                        | PrimitiveType::Int16T
                        | PrimitiveType::Uint8T
                        | PrimitiveType::Uint16T,
                    ) => {
                        return Ok(Some(Value::RustCode(RustCode::boxed(
                            format!("({} as ::core::ffi::c_int)", rc.value),
                            Type::primitive(PrimitiveType::Int),
                            rc.is_const,
                            rc.is_unsafe,
                        ))));
                    }
                    _ => (),
                }
            }
        }
        Ok(None)
    }

    pub fn deref_read(self, ctx: &EmitContext) -> Result<Value, EmitErr> {
        if let Value::Place(ptr, field) = self {
            let Some(ty) = ptr.ty()?.inner_ty() else {
                return Err(
                    ParseErr::new(ptr.ty()?.span(), "can't read value of unknown type").into(),
                );
            };
            let Type {
                ty: TypeEnum::Pointer(p),
                ..
            } = ty
            else {
                todo!()
            };
            let (value, ty) = if let Some(field) = field {
                let TypeEnum::Ident(s) = &p.ty else { todo!() };
                let Some(sym) = ctx.lookup_sym(s) else {
                    ctx.add_unresolved_sym_dependency(s.clone())?;
                    return Err(ParseErr::new(s.span(), "unresolved type").into());
                };
                let Some(fty) = sym.field_type(ctx, field.as_str()) else {
                    dbg!(&ptr, &field);
                    todo!()
                };
                (
                    ctx.capture_output(|ctx| {
                        write!(ctx, "unsafe {{ ::core::ptr::addr_of!((*")?;
                        ptr.emit(ctx)?;
                        write!(ctx, ").")?;
                        field.emit(ctx)?;
                        write!(ctx, ").read() }}")?;
                        Ok(())
                    })?,
                    fty,
                )
            } else {
                (
                    ctx.capture_output(|ctx| {
                        write!(ctx, "unsafe {{ ")?;
                        ptr.emit(ctx)?;
                        write!(ctx, ".read() }}")?;
                        {
                            write!(ctx, ".")?;
                            field.emit(ctx)?;
                        }
                        Ok(())
                    })?,
                    *p,
                )
            };
            Ok(Value::RustCode(RustCode::boxed(
                value,
                ty,
                ptr.is_const(),
                true,
            )))
        } else {
            Ok(self)
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::I32(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err(())
                }
            }
            Value::U31(u) => Ok(u as u64),
            Value::U32(u) => Ok(u as u64),
            Value::I64(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err(())
                }
            }
            Value::U63(u) => Ok(u),
            Value::U64(u) => Ok(u),
            _ => Err(()),
        }
    }
}

impl Emit for Value {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Value::I32(i) => write!(ctx, "{i}_i32")?,
            Value::U31(u) => write!(ctx, "{u}")?,
            Value::U32(u) => write!(ctx, "{u}_u32")?,
            Value::I64(i) => write!(ctx, "{i}_i64")?,
            Value::U63(i) => write!(ctx, "{i}_i64")?,
            Value::U64(u) => write!(ctx, "{u}_u64")?,
            Value::Usize(u) => write!(ctx, "{u}_usize")?,
            &Value::F32(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    let dec = if s.contains('.') { "" } else { ".0" };
                    write!(ctx, "{s}{dec}_f32")?
                } else {
                    todo!()
                }
            }
            &Value::F64(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    let dec = if s.contains('.') { "" } else { ".0" };
                    write!(ctx, "{s}{dec}_f64")?
                } else {
                    todo!()
                }
            }
            &Value::Bool(b) => write!(ctx, "{b}")?,
            Value::String(s) => return s.emit(ctx),
            Value::TargetDependent(_) => {
                return Err(ParseErr::new(Span::none(), "can't emit indeterminate value").into())
            }
            Value::RustCode(s) => ctx.write_str(&s.value)?,
            Value::Place(_, _) => self.clone().deref_read(ctx)?.emit(ctx)?,
        }
        Ok(())
    }
}

impl Display for RustCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl Eval for DefineValue {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Self::Expr(expr) => expr.try_eval(ctx),
            Self::Items(items) => items.try_eval(ctx),
            Self::TargetDependent
            | Self::Type(_)
            | Self::Other(_)
            | Self::Empty
            | Self::ExprFollowedBy(_, _) => Ok(None),
        }
    }
}

impl Emit for Ambiguous {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(value) = self.try_eval(ctx)? {
            value.emit(ctx)
        } else {
            Err(ParseErr::new(self.span(), "can't resolve ambiguous expression").into())
        }
    }
}

impl Eval for Ambiguous {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let mut result = None;
        for alt in self.alternatives.iter() {
            if let Ok(Some(value)) = match alt {
                Alternative::Expr(expr) => expr.try_eval(ctx),
                Alternative::Type(_) => continue,
                Alternative::Items(_) => continue,
            } {
                if result.is_none() {
                    result = Some(value);
                } else {
                    return Ok(None);
                }
            }
        }
        Ok(result)
    }
}

impl Eval for Cast {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let mut is_const = false;
        let mut is_unsafe = false;

        if let Some(Type {
            ty: TypeEnum::Ident(ident),
            ..
        }) = self.ty.inner_ty()
        {
            if let Some(sym) = ctx.lookup_sym(&ident) {
                if sym.value_ty.is_some() {
                    // not a type
                    return Err(ParseErr::new(self.span(), "can't cast to non-type").into());
                }
            } else {
                ctx.add_unresolved_sym_dependency(ident.clone())?;
                return Err(ParseErr::new(self.span(), "cast to unresolved type").into());
            }
        }
        let expr_i = if let Some(expr) = self.expr.try_eval(ctx)? {
            is_const = expr.is_const();
            is_unsafe = expr.is_unsafe();
            if expr.ty()?.is_uninferred() {
                expr.ty()?.resolve_to(self.ty.clone());
            }
            if expr.ty()?.inner_ty() == self.ty.inner_ty() {
                return Ok(Some(expr));
            }
            if self.ty.is_bool() {
                return expr.coerce(ctx, &self.ty);
            }
            expr.coerce_to_int(ctx)?
        } else {
            None
        };

        let out = if self.ty.is_void() {
            ctx.capture_output(|ctx| {
                write!(ctx, "{{ let _ = ")?;
                self.expr.emit(ctx)?;
                write!(ctx, "; }}")?;
                Ok(())
            })
        } else if let Some(sym) = self.ty.is_c_enum(ctx)? {
            ctx.capture_output(|ctx| {
                write!(ctx, "{}(", sym.ident)?;
                self.expr.emit(ctx)?;
                write!(ctx, ")")?;
                Ok(())
            })
        } else {
            ctx.capture_output(|ctx| {
                write!(ctx, "(")?;
                if let Some(expr) = expr_i {
                    expr.emit(ctx)?;
                } else {
                    self.expr.emit(ctx)?;
                }
                write!(ctx, " as ")?;
                self.ty.emit(ctx)?;
                write!(ctx, ")")?;
                Ok(())
            })
        }?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            out,
            self.ty.clone(),
            is_const,
            is_unsafe,
        ))))
    }
}

impl Eval for IntegerLiteral {
    fn try_eval(&self, _ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self.kind {
            IntegerLiteralType::Unsuffixed => {
                if self.value <= i32::MAX as u64 {
                    Ok(Some(Value::U31(self.value as u32)))
                } else if self.value <= u32::MAX as u64 {
                    Ok(Some(Value::U32(self.value as u32)))
                } else if self.value <= i64::MAX as u64 {
                    Ok(Some(Value::U63(self.value)))
                } else {
                    Ok(Some(Value::U64(self.value)))
                }
            }
            IntegerLiteralType::Unsigned => {
                if self.value <= u32::MAX as u64 {
                    Ok(Some(Value::U32(self.value as u32)))
                } else {
                    todo!()
                }
            }
            IntegerLiteralType::Long => {
                // !!! FIXME: long can be 32-bit or 64-bit depending on platform
                if self.value <= i32::MAX as u64 {
                    Ok(Some(Value::I32(self.value as i32)))
                } else if self.value <= i64::MAX as u64 {
                    Ok(Some(Value::I64(self.value as i64)))
                } else {
                    todo!()
                }
            }
            IntegerLiteralType::UnsignedLong => todo!(),
            IntegerLiteralType::LongLong => Ok(Some(Value::I64(self.value as i64))),
            IntegerLiteralType::UnsignedLongLong => Ok(Some(Value::U64(self.value))),
        }
    }
}

impl Eval for SizeOf {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            SizeOf::Type(span, ty) => match &ty.ty {
                TypeEnum::Primitive(_) | TypeEnum::Pointer(_) => {
                    let out = ctx.capture_output(|ctx| {
                        write!(ctx, "::core::mem::size_of::<")?;
                        ty.emit(ctx)?;
                        write!(ctx, ">()")?;
                        Ok(())
                    })?;
                    Ok(Some(Value::RustCode(RustCode::boxed(
                        out,
                        Type::primitive(PrimitiveType::SizeT),
                        true,
                        false,
                    ))))
                }

                TypeEnum::Ident(ident) => {
                    if ctx.lookup_sym(ident).is_some() {
                        Ok(Some(Value::RustCode(RustCode::boxed(
                            format!("::core::mem::size_of::<{}>()", ident.as_str()),
                            Type::primitive(PrimitiveType::SizeT),
                            true,
                            false,
                        ))))
                    } else {
                        todo!()
                    }
                }

                TypeEnum::Infer(i) => {
                    if let Some(ty) = &*i.borrow() {
                        SizeOf::Type(span.clone(), ty.clone()).try_eval(ctx)
                    } else {
                        Ok(None)
                    }
                }

                TypeEnum::Enum(_) => todo!(),
                TypeEnum::Struct(_) => todo!(),
                TypeEnum::Array(_, _) => todo!(),
                TypeEnum::FnPointer(_) => todo!(),
                TypeEnum::DotDotDot => todo!(),
                TypeEnum::Rust(_) => todo!(),
                TypeEnum::Function(_) => todo!(),
            },

            SizeOf::Expr(_, expr) => {
                let expr = expr.deparenthesize();
                match expr {
                    Expr::UnaryOp(uop) => {
                        // special case: sizeof(*pointer_variable)
                        if uop.op.as_str() == "*" {
                            if let Expr::Ident(ident) = &uop.expr.deparenthesize() {
                                if let Some(Sym {
                                    value_ty: Some(ty), ..
                                }) = ctx.lookup_sym(&ident.clone().try_into().unwrap())
                                {
                                    if let Some(ty) = ty.get_pointer_type() {
                                        let code = ctx.capture_output(|ctx| {
                                            write!(ctx, "::core::mem::size_of::<")?;
                                            ty.emit(ctx)?;
                                            write!(ctx, ">()")?;
                                            Ok(())
                                        })?;
                                        return Ok(Some(Value::RustCode(RustCode::boxed(
                                            code,
                                            Type::primitive(PrimitiveType::SizeT),
                                            true,
                                            false,
                                        ))));
                                    }
                                }
                            }
                        }
                    }

                    Expr::BinaryOp(bop) => {
                        // special case: sizeof(((Type*)_)->field)
                        if bop.op.as_str() == "->" {
                            let lhs = bop.lhs.deparenthesize();
                            if let (Expr::Cast(lhs), Expr::Ident(rhs)) = (&lhs, &bop.rhs) {
                                if let Some(ty) = lhs.ty.get_pointer_type() {
                                    let code = ctx.capture_output(|ctx| {
                                        write!(ctx, "crate::size_of_field!(")?;
                                        ty.emit(ctx)?;
                                        write!(ctx, ", {rhs})")?;
                                        Ok(())
                                    })?;
                                    return Ok(Some(Value::RustCode(RustCode::boxed(
                                        code,
                                        Type::primitive(PrimitiveType::SizeT),
                                        true,
                                        false,
                                    ))));
                                }
                            }
                        }
                    }

                    _ => (),
                }

                if let Ok(Some(val)) = expr.try_eval(ctx) {
                    let code = ctx.capture_output(|ctx| {
                        write!(ctx, "::core::mem::size_of::<")?;
                        val.ty()?.emit(ctx)?;
                        write!(ctx, ">()")?;
                        Ok(())
                    })?;
                    return Ok(Some(Value::RustCode(RustCode::boxed(
                        code,
                        Type::primitive(PrimitiveType::SizeT),
                        true,
                        false,
                    ))));
                }

                // !!! FIXME: size_of_val isn't const, so this'd require finding the type of the expression.
                // skip it for now
                Ok(None)
            }
        }
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
        macro_rules! eval {
            ($expr:expr) => {{
                let expr = &$expr;
                match expr.try_eval(ctx) {
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        log_debug!(ctx, "expr evaled to None: {}", expr.span());
                        return Ok(None);
                    }
                    Err(err) => {
                        log_debug!(ctx, "expr errored: {}", expr.span());
                        log_debug!(ctx, "{err:#?}");
                        return Err(err);
                    }
                }
            }};
        }

        match self {
            Expr::Parenthesized(p) => return p.expr.try_eval(ctx),

            Expr::Ident(ident) => {
                if let Ok(i) = ident.clone().try_into() {
                    return if let Some(sym) = ctx.lookup_sym(&i) {
                        if let Some(ty) = sym.value_ty {
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                i.to_string(),
                                ty,
                                true,
                                false,
                            ))))
                        } else {
                            Err(ParseErr::new(ident.span(), "symbol isn't a value").into())
                        }
                    } else if let Ok(Some((args, value))) = ctx.preproc_state().borrow().lookup(&i)
                    {
                        if args.is_some() {
                            return Err(ParseErr::new(
                                self.span(),
                                "define with arguments used without arguments",
                            )
                            .into());
                        }
                        value.try_eval(ctx)
                    } else if !ctx.is_preproc_eval_mode() {
                        ctx.add_unresolved_sym_dependency(i)?;
                        Err(ParseErr::new(ident.span(), "unresolved symbol").into())
                    } else {
                        Ok(None)
                    };
                }

                match ident.as_str() {
                    "true" => return Ok(Some(Value::Bool(true))),
                    "false" => return Ok(Some(Value::Bool(false))),
                    "size_t" => return Ok(None),
                    _ => {
                        if let Ok(ident) = Ident::try_from(ident.clone()) {
                            if let Some(sym) = ctx.lookup_sym(&ident) {
                                return if let Some(ty) = &sym.value_ty {
                                    Ok(Some(Value::RustCode(RustCode::boxed(
                                        ident.to_string(),
                                        ty.clone(),
                                        true,
                                        false,
                                    ))))
                                } else {
                                    Ok(None)
                                };
                            }
                        }
                    }
                }
            }

            Expr::Literal(lit) => {
                return match lit {
                    Literal::Integer(i) => i.try_eval(ctx),
                    Literal::Float(f) => match f {
                        FloatLiteral::Float(f) => Ok(Some(Value::F32(f.value))),
                        FloatLiteral::Double(f) => Ok(Some(Value::F64(f.value))),
                    },
                    Literal::String(s) => Ok(Some(Value::String(s.clone()))),
                }
            }

            Expr::FnCall(call) => return call.try_eval(ctx),

            Expr::Ambiguous(amb) => return amb.try_eval(ctx),

            Expr::Cast(cast) => return cast.try_eval(ctx),

            Expr::Asm(_) => return Ok(None),

            Expr::SizeOf(s) => return s.try_eval(ctx),

            Expr::UnaryOp(uop) => {
                let expr = eval!(uop.expr);

                return match uop.op.as_str().as_bytes() {
                    b"!" => {
                        if ctx.is_preproc_eval_mode() {
                            if let Value::TargetDependent(ds) = expr {
                                return Ok(Some(Value::TargetDependent(ds.not())));
                            }
                        }
                        let expr = expr.coerce(ctx, &Type::bool())?.unwrap_or(expr);
                        match expr {
                            Value::Bool(b) => Ok(Some(Value::Bool(!b))),
                            Value::RustCode(mut rc) => {
                                rc.value.insert_str(0, "!(");
                                rc.value.push(')');
                                Ok(Some(Value::RustCode(rc)))
                            }
                            _ => unreachable!(),
                        }
                    }

                    b"*" => Ok(Some(Value::Place(Box::new(expr), None))),

                    b"+" => match expr {
                        Value::String(_) => Ok(None),
                        _ => Ok(Some(expr)),
                    },

                    b"-" => match expr {
                        Value::I32(value) => {
                            Ok(Some(Value::I32(value.checked_neg().ok_or_else(|| {
                                ParseErr::new(uop.span(), "can't negate i32::MIN")
                            })?)))
                        }
                        Value::U31(value) | Value::U32(value) => {
                            Ok(Some(Value::I32(-(value as i32))))
                        }
                        Value::I64(value) => {
                            Ok(Some(Value::I64(value.checked_neg().ok_or_else(|| {
                                ParseErr::new(uop.span(), "can't negate i64::MIN")
                            })?)))
                        }
                        Value::U63(value) | Value::U64(value) => {
                            Ok(Some(Value::I64(-(value as i64))))
                        }
                        Value::F32(value) => Ok(Some(Value::F32(-value))),
                        Value::F64(value) => Ok(Some(Value::F64(-value))),
                        Value::TargetDependent(_) => Ok(Some(expr)),
                        _ => Ok(None),
                    },

                    b"~" => match expr {
                        Value::I32(value) => Ok(Some(Value::I32(!value))),
                        Value::U31(value) => Ok(Some(Value::I32(!(value as i32)))),
                        Value::U32(value) => Ok(Some(Value::U32(!value))),
                        Value::I64(value) => Ok(Some(Value::I64(!value))),
                        Value::U63(value) => Ok(Some(Value::I64(!(value as i64)))),
                        Value::U64(value) => Ok(Some(Value::U64(!value))),
                        Value::RustCode(c) => Ok(Some(Value::RustCode(RustCode::boxed(
                            format!("!({})", c.value),
                            c.ty,
                            c.is_const,
                            c.is_unsafe,
                        )))),
                        _ => todo!(),
                    },

                    _ => Err(ParseErr::new(uop.span(), "missing implementation for eval").into()),
                };
            }

            Expr::BinaryOp(bop) => {
                macro_rules! assign {
                    ($op:tt) => {{
                        let rhs = eval!(bop.rhs);
                        if let Expr::Ident(_) = &bop.lhs {
                            todo!()
                        } else {
                            let lhs = eval!(bop.lhs);
                            let Value::Place(ptr, field) = lhs else {
                                todo!()
                            };
                            let Some(Type {
                                ty: TypeEnum::Pointer(p),
                                ..
                            }) = ptr.ty()?.inner_ty()
                            else {
                                todo!()
                            };
                            let (value, value_ty) = if let Some(field) = field {
                                let TypeEnum::Ident(s) = &p.ty else { todo!() };
                                let Some(sym) = ctx.lookup_sym(s) else {
                                    todo!()
                                };
                                let Some(fty) = sym.field_type(ctx, field.as_str()) else {
                                    todo!()
                                };
                                let value = ctx.capture_output(|ctx| {
                                    writeln!(ctx, "{{")?;
                                    ctx.increase_indent();
                                    write!(ctx, "let (ptr, value) = (unsafe {{ ::core::ptr::addr_of_mut!((*")?;
                                    ptr.emit(ctx)?;
                                    write!(ctx, ").")?;
                                    field.emit(ctx)?;
                                    write!(ctx, ") }}, ")?;
                                    rhs.emit(ctx)?;
                                    writeln!(ctx, ");")?;
                                    writeln!(ctx, "unsafe {{ ptr.write(value) }};")?;
                                    writeln!(ctx, "value")?;
                                    ctx.decrease_indent();
                                    write!(ctx, "}}")?;
                                    Ok(())
                                })?;
                                (value, fty)
                            } else {
                                let value = ctx.capture_output(|ctx| {
                                    writeln!(ctx, "unsafe {{")?;
                                    ctx.increase_indent();
                                    write!(ctx, "let (ptr, value) = (")?;
                                    ptr.emit(ctx)?;
                                    write!(ctx, ", ")?;
                                    rhs.emit(ctx)?;
                                    writeln!(ctx, ");")?;
                                    writeln!(ctx, "ptr.write(value);")?;
                                    writeln!(ctx, "value")?;
                                    ctx.decrease_indent();
                                    write!(ctx, "}}")?;
                                    Ok(())
                                })?;
                                (value, *p)
                            };
                            return Ok(Some(Value::RustCode(RustCode::boxed(
                                value, value_ty, false, true,
                            ))));
                        }
                    }};
                }

                macro_rules! bitop {
                    ($op:tt) => {{
                        let mut lhs = eval!(bop.lhs);
                        let mut rhs = eval!(bop.rhs);
                        return if let Promoted::None = Value::promote(ctx, &mut lhs, &mut rhs)? {
                            Ok(None)
                        } else {
                            let code = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " {} ", stringify!($op))?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                code,
                                lhs.ty()?,
                                lhs.is_const() && rhs.is_const(),
                                lhs.is_unsafe() || rhs.is_unsafe(),
                            ))))
                        };
                    }};
                }

                macro_rules! calc {
                    ($op:tt) => {{
                        let lhs = eval!(bop.lhs);
                        let rhs = eval!(bop.rhs);
                        match (lhs, rhs) {
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
                                    Ok(Some(Value::U32(value)))
                                }
                            }
                            (Value::U31(lhs), Value::U32(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::U32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::U32(lhs), Value::U32(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::F32(lhs), Value::F32(rhs)) => Ok(Some(Value::F32(lhs $op rhs))),
                            (Value::F64(lhs), Value::F64(rhs)) => Ok(Some(Value::F64(lhs $op rhs))),

                            (Value::U31(lhs), Value::RustCode(rhs)) => {
                                // FIXME
                                assert!(matches!(rhs.ty.ty, TypeEnum::Primitive(PrimitiveType::SizeT)));
                                Ok(Some(Value::RustCode(RustCode::boxed(format!("({} {} {})", lhs, stringify!($op), rhs.value), rhs.ty, rhs.is_const, rhs.is_unsafe))))
                            }

                            (mut lhs, mut rhs) => {
                                if let Promoted::None = Value::promote(ctx, &mut lhs, &mut rhs)? {
                                    Ok(None)
                                } else {
                                    let code = ctx.capture_output(|ctx| {
                                        write!(ctx, "(")?;
                                        lhs.emit(ctx)?;
                                        write!(ctx, " {} ", stringify!($op))?;
                                        rhs.emit(ctx)?;
                                        write!(ctx, ")")?;
                                        Ok(())
                                    })?;
                                    Ok(Some(Value::RustCode(RustCode::boxed(code, lhs.ty()?, lhs.is_const() && rhs.is_const(),lhs.is_unsafe() || rhs.is_unsafe()))))
                                }
                            }
                        }
                    }};

                    (@ checked * ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_mul($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked / ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_div($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked % ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_rem($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked + ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_add($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked - ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_add($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };
                }

                macro_rules! compare {
                    ($op:tt) => {{
                        let op = stringify!($op);
                        let mut lhs = eval!(bop.lhs);
                        let mut rhs = eval!(bop.rhs);
                        Value::promote(ctx, &mut lhs, &mut rhs)?;
                        match (lhs, rhs) {
                            (Value::I32(lhs), Value::I32(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::U31(lhs), Value::U31(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::U32(lhs), Value::U32(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::I64(lhs), Value::I64(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::U63(lhs), Value::U63(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::U64(lhs), Value::U64(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::F32(lhs), Value::F32(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (Value::F64(lhs), Value::F64(rhs)) => Ok(Some(Value::Bool(lhs $op rhs))),
                            (lhs, Value::RustCode(rhs)) => {
                                if matches!(op, "==" | "!=") && rhs.ty.is_c_enum(ctx)?.is_some() {
                                    let code = ctx.capture_output(|ctx| {
                                        if op == "!=" {
                                            write!(ctx, "!")?;
                                        }
                                        write!(ctx, "::core::matches!(")?;
                                        lhs.emit(ctx)?;
                                        write!(ctx, ", {rhs})")?;
                                        Ok(())
                                    })?;
                                    return Ok(Some(Value::RustCode(RustCode::boxed(
                                        code,
                                        Type::bool(),
                                        lhs.is_const(), lhs.is_unsafe()
                                    ))))
                                }
                                let code = ctx.capture_output(|ctx| {
                                    write!(ctx, "(")?;
                                    lhs.emit(ctx)?;
                                    write!(ctx, " {op} {rhs})")?;
                                    Ok(())
                                })?;
                                Ok(Some(Value::RustCode(RustCode::boxed(
                                    code,
                                    Type::bool(),
                                    lhs.is_const() && rhs.is_const,
                                    lhs.is_unsafe() || rhs.is_unsafe
                                ))))
                            }
                            (Value::RustCode(lhs), rhs) => {
                                if matches!(op, "==" | "!=") && lhs.ty.is_c_enum(ctx)?.is_some() {
                                    let code = ctx.capture_output(|ctx| {
                                        if op == "!=" {
                                            write!(ctx, "!")?;
                                        }
                                        write!(ctx, "::core::matches!(")?;
                                        rhs.emit(ctx)?;
                                        write!(ctx, ", {lhs})")?;
                                        Ok(())
                                    })?;
                                    return Ok(Some(Value::RustCode(RustCode::boxed(
                                        code,
                                        Type::bool(),
                                        rhs.is_const(), rhs.is_unsafe()
                                    ))))
                                }
                                let code = ctx.capture_output(|ctx| {
                                    write!(ctx, "({lhs} {op} ")?;
                                    rhs.emit(ctx)?;
                                    write!(ctx, ")")?;
                                    Ok(())
                                })?;
                                Ok(Some(Value::RustCode(RustCode::boxed(code, Type::bool(), lhs.is_const && rhs.is_const(), lhs.is_unsafe||rhs.is_unsafe()))))
                            }
                            _ => Err(ParseErr::new(bop.span(), format!("invalid operands to `{op}`")).into()),
                        }
                    }};
                }

                macro_rules! shift {
                    ($op:tt) => {{
                        let op = stringify!($op);

                        let lhs = eval!(bop.lhs);
                        let lhs = lhs.coerce_to_int(ctx)?.unwrap_or(lhs);

                        let rhs = eval!(bop.rhs);
                        let rhs = rhs.coerce_to_int(ctx)?.unwrap_or(rhs);

                        if let Ok(shift) = u64::try_from(rhs.clone()) {
                            match &lhs {
                                Value::I32(lhs) => Ok(Some(Value::I32(lhs $op shift))),
                                Value::U31(lhs) => Ok(Some(Value::I32((*lhs as i32) $op shift))),
                                Value::U32(lhs) => Ok(Some(Value::U32(lhs $op shift))),
                                Value::I64(lhs) => Ok(Some(Value::I64(lhs $op shift))),
                                Value::U63(lhs) => Ok(Some(Value::I64((*lhs as i64) $op shift))),
                                Value::U64(lhs) => Ok(Some(Value::U64(lhs $op shift))),
                                Value::RustCode(rc) => {
                                    let code = ctx.capture_output(|ctx| {
                                        write!(ctx, "(")?;
                                        lhs.emit(ctx)?;
                                        write!(ctx, " {op} {shift})")?;
                                        Ok(())
                                    })?;
                                    Ok(Some(Value::RustCode(RustCode::boxed(code, rc.ty.clone(), rc.is_const, rc.is_unsafe))))
                                }

                                _ => Err(ParseErr::new(
                                    bop.lhs.span(),
                                    format!("invalid operand to `{op}`"),
                                )
                                .into()),
                            }
                        } else {
                            let code = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " {op} ")?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                code,
                                lhs.ty()?,
                                lhs.is_const() && rhs.is_const(),
                                lhs.is_unsafe() || rhs.is_unsafe()
                            ))))
                        }
                    }};
                }

                return match bop.op.as_str().as_bytes() {
                    b"*" => calc!(*),
                    b"/" => calc!(/),
                    b"%" => calc!(%),
                    b"+" => calc!(+),
                    b"-" => calc!(-),

                    b"&" => bitop!(&),
                    b"|" => bitop!(|),

                    b"," => {
                        let _lhs = eval!(bop.lhs);
                        let rhs = eval!(bop.rhs);
                        Ok(Some(rhs))
                    }

                    b"." => {
                        let Expr::Ident(lhs) = bop.lhs.deparenthesize() else {
                            todo!()
                        };
                        let Expr::Ident(rhs) = &bop.rhs else { todo!() };
                        let Some(sym) = ctx.lookup_sym(&lhs.clone().try_into().unwrap()) else {
                            todo!()
                        };
                        let Some(Type {
                            ty: TypeEnum::Ident(vty),
                            ..
                        }) = sym.value_ty
                        else {
                            todo!()
                        };
                        let Some(sym) = ctx.lookup_sym(&vty) else {
                            todo!()
                        };
                        let Some(ty) = sym.field_type(ctx, rhs.as_str()) else {
                            todo!();
                        };
                        let value = ctx.capture_output(|ctx| {
                            lhs.emit(ctx)?;
                            write!(ctx, ".")?;
                            rhs.emit(ctx)?;
                            Ok(())
                        })?;
                        Ok(Some(Value::RustCode(RustCode::boxed(
                            value, ty, true, false,
                        ))))
                    }

                    b"&&" => {
                        let lhs = eval!(bop.lhs);
                        if let Value::TargetDependent(lhs) = lhs {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(lhs.all(rhs))))
                            } else if rhs.is_truthy() {
                                Ok(Some(Value::TargetDependent(lhs)))
                            } else {
                                Ok(Some(Value::Bool(false)))
                            }
                        } else if let Value::RustCode(_) = lhs {
                            let rhs = eval!(bop.rhs);
                            let lhs = lhs.coerce(ctx, &Type::bool())?.unwrap_or(lhs);
                            let rhs = rhs.coerce(ctx, &Type::bool())?.unwrap_or(rhs);
                            let out = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " && ")?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                out,
                                Type::bool(),
                                lhs.is_const() && rhs.is_const(),
                                lhs.is_unsafe() || rhs.is_unsafe(),
                            ))))
                        } else if lhs.is_truthy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::Bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::Bool(false)))
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
                                Ok(Some(Value::Bool(true)))
                            }
                        } else if let Value::RustCode(_) = lhs {
                            let rhs = eval!(bop.rhs);
                            let lhs = lhs.coerce(ctx, &Type::bool())?.unwrap_or(lhs);
                            let rhs = rhs.coerce(ctx, &Type::bool())?.unwrap_or(rhs);
                            let out = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " || ")?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                out,
                                Type::bool(),
                                lhs.is_const() && rhs.is_const(),
                                lhs.is_unsafe() || rhs.is_unsafe(),
                            ))))
                        } else if lhs.is_falsy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::Bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::Bool(true)))
                        }
                    }

                    b"=" => assign!(=),

                    b"==" => compare!(==),
                    b"!=" => compare!(!=),
                    b"<" => compare!(<),
                    b"<=" => compare!(<=),
                    b">" => compare!(>),
                    b">=" => compare!(>=),

                    b"<<" => shift!(<<),
                    b">>" => shift!(>>),

                    b"->" => {
                        let lhs = eval!(bop.lhs);
                        let Expr::Ident(rhs) = &bop.rhs else {
                            return Err(
                                ParseErr::new(bop.rhs.span(), "invalid operator to `->`").into()
                            );
                        };
                        return Ok(Some(Value::Place(
                            Box::new(lhs.deref_read(ctx)?),
                            Some(rhs.clone().try_into().unwrap()),
                        )));
                    }

                    _ => {
                        return Err(
                            ParseErr::new(bop.span(), "missing implementation for eval").into()
                        )
                    }
                };
            }

            Expr::PostOp(_) => (),

            Expr::Ternary(top) => return top.try_eval(ctx),

            Expr::ArrayIndex { .. } => (),
            Expr::ArrayValues { .. } => (),

            Expr::HasInclude(_) => return Ok(Some(Value::Bool(false))),

            Expr::Value(value) => return Ok(Some(value.clone())),
        }

        Err(ParseErr::new(self.span(), "missing implementation for eval").into())
    }
}

impl Emit for Expr {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Expr::Parenthesized(e) => e.emit(ctx),
            Expr::Ident(i) => {
                if let Some(sym) = ctx.lookup_sym(&i.clone().try_into().unwrap()) {
                    sym.ident.emit(ctx)
                } else {
                    Err(ParseErr::new(i.span(), "undefined symbol").into())
                }
            }
            Expr::Literal(lit) => lit.emit(ctx),
            Expr::FnCall(call) => call.emit(ctx),
            Expr::Ambiguous(amb) => amb.emit(ctx),
            Expr::Cast(cast) => cast
                .try_eval(ctx)?
                .ok_or_else(|| ParseErr::new(cast.span(), "couldn't eval"))?
                .emit(ctx),
            Expr::Asm(_) => todo!(),
            Expr::SizeOf(sizeof) => sizeof
                .try_eval(ctx)?
                .ok_or_else(|| ParseErr::new(sizeof.span(), "couldn't eval"))?
                .emit(ctx),

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

            Expr::BinaryOp(bop) => {
                if let Some(value) = self.try_eval(ctx)? {
                    value.emit(ctx)
                } else {
                    Err(ParseErr::new(bop.span(), "can't evaluate this expression").into())
                }
            }

            Expr::PostOp(_) => todo!(),
            Expr::Ternary(ternary) => ternary.emit(ctx),
            Expr::ArrayIndex { .. } => todo!(),
            Expr::ArrayValues { .. } => todo!(),
            Expr::Value(value) => value.emit(ctx),
            Expr::HasInclude(_) => todo!(),
        }
    }
}

impl Emit for Parenthesized {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.write_char('(')?;
        self.expr.emit(ctx)?;
        ctx.write_char(')')?;
        Ok(())
    }
}

impl Emit for FnCall {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Expr::Ident(ident) = &*self.func {
            if patch_emit_macro_call(ctx, ident.as_str(), self)? {
                return Ok(());
            }
        }
        if let Some(value) = self.try_eval(ctx)? {
            value.emit(ctx)
        } else {
            Err(ParseErr::new(self.span(), "can't emit this macro call").into())
        }
    }
}

impl Eval for FnCall {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        if let Expr::Ident(ident) = &*self.func {
            if let Some(value) = patch_eval_macro_call(ctx, ident.as_str(), self)? {
                return Ok(Some(value));
            }

            if let Some(Sym {
                value_ty:
                    Some(Type {
                        ty: TypeEnum::Function(f),
                        ..
                    }),
                ..
            }) = ctx.lookup_sym(&ident.clone().try_into().unwrap())
            {
                let out = ctx.capture_output(|ctx| {
                    if f.is_unsafe {
                        write!(ctx, "unsafe {{ ")?;
                    }
                    ident.emit(ctx)?;
                    write!(ctx, "(")?;
                    let mut first = true;
                    for (arg, arg_ty) in self.args.iter().zip(f.args.iter()) {
                        if !first {
                            write!(ctx, ", ")?;
                        }
                        first = false;
                        if let Ok(Some(argval)) = arg.try_eval(ctx) {
                            if let Some(argval) = argval.coerce(ctx, arg_ty)? {
                                argval.emit(ctx)?;
                                continue;
                            }
                        }
                        arg.emit(ctx)?;
                    }
                    write!(ctx, ")")?;
                    if f.is_unsafe {
                        write!(ctx, " }}")?;
                    }
                    Ok(())
                })?;
                return Ok(Some(Value::RustCode(RustCode::boxed(
                    out,
                    f.return_type.clone(),
                    f.is_const,
                    f.is_unsafe,
                ))));
            } else {
                ctx.add_unresolved_sym_dependency(ident.clone().try_into().unwrap())?;
                return Ok(None);
            }
        }

        Ok(None)
    }
}

impl Emit for Literal {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Literal::Integer(lit) => lit.emit(ctx),
            Literal::Float(lit) => lit.emit(ctx),
            Literal::String(lit) => lit.emit(ctx),
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
            s = format!("{}{s}", "0".repeat(self.ndigits as usize - s.len()));
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
        write!(ctx, "c\"")?;
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
        write!(ctx, "\".as_ptr()")?;
        Ok(())
    }
}

impl Emit for Ternary {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        self.try_eval(ctx)?
            .ok_or_else(|| ParseErr::new(self.span(), "couldn't eval ternary"))?
            .emit(ctx)
    }
}

impl Eval for Ternary {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        let Some(cond) = self.cond.try_eval(ctx)? else {
            return Ok(None);
        };
        let cond = cond.coerce(ctx, &Type::bool())?.unwrap_or(cond);
        let Some(mut on_true) = self.on_true.try_eval(ctx)? else {
            return Ok(None);
        };
        let Some(mut on_false) = self.on_false.try_eval(ctx)? else {
            return Ok(None);
        };
        if matches!(
            Value::promote(ctx, &mut on_true, &mut on_false)?,
            Promoted::None
        ) {
            return Ok(None);
        }
        let value = ctx.capture_output(|ctx| {
            write!(ctx, "if ")?;
            cond.emit(ctx)?;
            write!(ctx, "{{ ")?;
            on_true.emit(ctx)?;
            write!(ctx, " }} else {{ ")?;
            on_false.emit(ctx)?;
            write!(ctx, " }}")?;
            Ok(())
        })?;
        Ok(Some(Value::RustCode(RustCode::boxed(
            value,
            on_true.ty()?,
            cond.is_const() && on_true.is_const() && on_false.is_const(),
            cond.is_unsafe() || on_true.is_unsafe() || on_false.is_unsafe(),
        ))))
    }
}
