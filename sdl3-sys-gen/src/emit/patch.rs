use super::{DefineState, Emit, EmitContext, EmitErr, EmitResult, Eval, Value};
use crate::parse::{
    Block, Define, DefineValue, Expr, FnCall, Function, GetSpan, Ident, IdentOrKw, Item, Items,
    Kw_static, ParseErr, RustCode, Span, StringLiteral, Type,
};
use core::fmt::Write;
use std::ffi::CString;

struct EmitPatch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&mut EmitContext, &T) -> Result<bool, EmitErr>,
}

struct EvalPatch<T: ?Sized> {
    matches: fn(&EmitContext, &str) -> bool,
    patch: fn(&EmitContext, &T) -> Result<Option<Value>, EmitErr>,
}

pub fn patch_emit_function(ctx: &mut EmitContext, f: &Function) -> Result<bool, EmitErr> {
    patch_emit(ctx, f, f.ident.as_str(), EMIT_FUNCTION_PATCHES)
}

type EmitFunctionPatch = EmitPatch<Function>;

const EMIT_FUNCTION_PATCHES: &[EmitFunctionPatch] = &[
    EmitFunctionPatch {
        // skip emitting these
        module: None,
        match_ident: |i| matches!(i, "__debugbreak" | "_ReadWriteBarrier"),
        patch: |_, _| Ok(true),
    },
    EmitFunctionPatch {
        module: Some("bits"),
        match_ident: |i| i == "SDL_MostSignificantBitIndex32",
        patch: |ctx, f| {
            f.doc.emit(ctx)?;
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(
                ctx,
                "pub const fn {}(x: Uint32) -> ::core::ffi::c_int {{",
                f.ident
            )?;
            ctx.increase_indent();
            writeln!(ctx, "31 - (x.leading_zeros() as ::core::ffi::c_int)")?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitFunctionPatch {
        module: Some("stdinc"),
        match_ident: |i| matches!(i, "SDL_memcpy" | "SDL_memmove"),
        patch: |ctx, f| {
            let mut f = f.clone();
            f.extern_kw = None;
            f.static_kw = Some(Kw_static { span: Span::none() });
            f.body = Some(Block {
                span: Span::none(),
                items: Items(vec![Item::Expr(Expr::Value(Value::RustCode(
                    RustCode::boxed(
                        format!(
                            "unsafe {{ ::core::ptr::{}(src.cast::<Uint8>(), dst.cast::<Uint8>(), len) }}; return dst",
                            if f.ident.as_str() == "SDL_memcpy" {
                                "copy_nonoverlapping"
                            } else {
                                "copy"
                            }
                        ),
                        Type::pointer(Type::void(), true),
                        false,
                        true,
                    ),
                )))]),
            });
            f.emit(ctx)?;
            Ok(true)
        },
    },
];

pub fn patch_emit_define(ctx: &mut EmitContext, define: &Define) -> Result<bool, EmitErr> {
    patch_emit(ctx, define, define.ident.as_str(), EMIT_DEFINE_PATCHES)
}

type EmitDefinePatch = EmitPatch<Define>;

const EMIT_DEFINE_PATCHES: &[EmitDefinePatch] = &[
    EmitDefinePatch {
        // skip emitting these
        module: None,
        match_ident: |i| {
            matches!(
                i,
                "SDL_arraysize"
                    | "SDL_BeginThreadFunction"
                    | "SDL_COMPILE_TIME_ASSERT"
                    | "SDL_const_cast"
                    | "SDL_EndThreadFunction"
                    | "SDL_HAS_BUILTIN"
                    | "SDL_InvalidParamError"
                    | "SDL_memcpy"
                    | "SDL_memmove"
                    | "SDL_memset"
                    | "SDL_PRILLd"
                    | "SDL_PRILLu"
                    | "SDL_PRILLx"
                    | "SDL_PRILLX"
                    | "SDL_reinterpret_cast"
                    | "SDL_SINT64_C"
                    | "SDL_stack_alloc"
                    | "SDL_stack_free"
                    | "SDL_static_cast"
                    | "SDL_STRINGIFY_ARG"
                    | "SDL_TriggerBreakpoint"
                    | "SDL_UINT64_C"
                    | "SDL_zero"
                    | "SDL_zeroa"
            )
        },
        patch: |_, _| Ok(true),
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| matches!(i, "SDL_clamp"),
        patch: |ctx, _| {
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(
                ctx,
                "pub fn SDL_clamp<T: Copy + PartialOrd>(x: T, a: T, b: T) -> T {{"
            )?;
            ctx.increase_indent();
            writeln!(ctx, "if x < a {{ a }} else if x > b {{ b }} else {{ x }}",)?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("atomic"),
        match_ident: |i| i == "SDL_CompilerBarrier",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(ctx, "pub fn SDL_CompilerBarrier() {{")?;
            ctx.increase_indent();
            writeln!(
                ctx,
                "::core::sync::atomic::fence(::core::sync::atomic::Ordering::SeqCst)"
            )?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_copyp",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            writeln!(ctx, "///")?;
            writeln!(ctx, "/// # Safety")?;
            writeln!(ctx, "/// It must be valid to write the memory pointed to by `src` to the memory pointed to by `dst`,")?;
            writeln!(
                ctx,
                "/// and the memory pointed to by `src` and `dst` must not overlap"
            )?;
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(
                ctx,
                "pub unsafe fn SDL_copyp<Dst: Sized, Src: Sized>(dst: *mut Dst, src: *const Src) -> *mut Dst {{"
            )?;
            ctx.increase_indent();
            writeln!(ctx, "const {{ assert!(::core::mem::size_of::<Dst>() == ::core::mem::size_of::<Src>()) }}")?;
            writeln!(
                ctx,
                "unsafe {{ ::core::ptr::copy_nonoverlapping(src.cast::<Uint8>(), dst.cast::<Uint8>(), ::core::mem::size_of::<Src>()) }};"
            )?;
            writeln!(ctx, "dst")?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_INIT_INTERFACE",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            writeln!(ctx, "///")?;
            writeln!(ctx, "/// # Safety")?;
            writeln!(ctx, "/// The type `T` must correctly implement [`crate::Interface`], and it must be valid to write a `T` to the memory pointed to by `iface`")?;
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(
                ctx,
                "pub unsafe fn SDL_INIT_INTERFACE<T: crate::Interface>(iface: *mut T) {{"
            )?;
            ctx.increase_indent();
            writeln!(ctx, "unsafe {{")?;
            ctx.increase_indent();
            writeln!(ctx, "iface.write_bytes(0, 1);")?;
            writeln!(
                ctx,
                "iface.cast::<Uint32>().write(::core::mem::size_of::<T>() as Uint32);"
            )?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| matches!(i, "SDL_min" | "SDL_max"),
        patch: |ctx, define| {
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(
                ctx,
                "pub fn {}<T: Copy + PartialOrd>(x: T, y: T) -> T {{",
                define.ident
            )?;
            ctx.increase_indent();
            writeln!(
                ctx,
                "if x {} y {{ x }} else {{ y }}",
                if define.ident.as_str() == "SDL_min" {
                    "<"
                } else {
                    ">"
                }
            )?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_PRILL_PREFIX",
        patch: |ctx, define| {
            define.emit(ctx)?;
            let Some(Value::String(StringLiteral { str, .. })) = define.value.try_eval(ctx)? else {
                unreachable!()
            };
            let mut bytes = str.into_bytes_with_nul();
            let edit = bytes.len() - 1;
            bytes.push(0);
            for ch in [b'd', b'u', b'x', b'X'] {
                bytes[edit] = ch;
                Define {
                    span: Span::none(),
                    doc: None,
                    ident: IdentOrKw::new_inline(format!("SDL_PRILL{}", char::from(ch))),
                    args: None,
                    value: DefineValue::Expr(Expr::Value(Value::String(StringLiteral {
                        span: Span::none(),
                        str: CString::from_vec_with_nul(bytes.clone()).unwrap(),
                    }))),
                }
                .emit(ctx)?;
            }
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_zerop",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            writeln!(ctx, "///")?;
            writeln!(ctx, "/// # Safety")?;
            writeln!(ctx, "/// It must be valid to zero all bytes of `T`, and it must be valid to write a `T` to the memory pointed to by `x`")?;
            writeln!(ctx, "#[inline(always)]")?;
            writeln!(ctx, "pub unsafe fn SDL_zerop<T>(x: *mut T) -> *mut T {{")?;
            ctx.increase_indent();
            writeln!(ctx, "unsafe {{ x.write_bytes(0, 1) }};")?;
            writeln!(ctx, "x")?;
            ctx.decrease_indent();
            writeln!(ctx, "}}")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("thread"),
        match_ident: |i| matches!(i, "SDL_CreateThread" | "SDL_CreateThreadWithProperties"),
        patch: |ctx, define| {
            emit_begin_end_thread_function(ctx)?;
            define.emit(ctx)?;
            Ok(true)
        },
    },
];

fn emit_begin_end_thread_function(ctx: &mut EmitContext) -> EmitResult {
    let btf = Ident::new_inline("SDL_BeginThreadFunction");
    if ctx.lookup_sym(&btf).is_some() {
        return Ok(());
    }
    let etf = Ident::new_inline("SDL_EndThreadFunction");
    let ty = Type::ident(Ident::new_inline("SDL_FunctionPointer"));

    ctx.register_sym(btf, None, Some(ty.clone()), None, None, true)?;
    ctx.register_sym(etf, None, Some(ty), None, None, true)?;

    let cfg_default = "#[cfg(not(windows))]";
    let cfg_win = "#[cfg(windows)]";
    writeln!(ctx, "{cfg_default}")?;
    writeln!(
        ctx,
        "pub const SDL_BeginThreadFunction: SDL_FunctionPointer = unsafe {{ ::core::mem::transmute::<*const ::core::ffi::c_void, SDL_FunctionPointer>(core::ptr::null()) }};"
    )?;
    writeln!(ctx, "{cfg_default}")?;
    writeln!(
        ctx,
        "pub const SDL_EndThreadFunction: SDL_FunctionPointer = unsafe {{ ::core::mem::transmute::<*const ::core::ffi::c_void, SDL_FunctionPointer>(core::ptr::null()) }};"
    )?;
    writeln!(ctx, "{cfg_win}")?;
    writeln!(ctx, "extern \"cdecl\" {{")?;
    ctx.increase_indent();
    writeln!(ctx, "fn _beginthreadex(security: *mut ::core::ffi::c_void, stack_size: ::core::ffi::c_uint, start_address: Option<extern \"stdcall\" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_uint>, arglist: *mut ::core::ffi::c_void, initflag: ::core::ffi::c_uint, thrdaddr: ::core::ffi::c_uint) -> ::core::primitive::usize;")?;
    writeln!(ctx, "fn _endthreadex(retval: ::core::ffi::c_uint);")?;
    ctx.decrease_indent();
    writeln!(ctx, "}}")?;
    writeln!(ctx, "{cfg_win}")?;
    writeln!(
        ctx,
        "pub const SDL_BeginThreadFunction: SDL_FunctionPointer = unsafe {{ ::core::mem::transmute::<unsafe extern \"cdecl\" fn(*mut ::core::ffi::c_void, ::core::ffi::c_uint, Option<extern \"stdcall\" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_uint>, *mut ::core::ffi::c_void, ::core::ffi::c_uint, ::core::ffi::c_uint) -> ::core::primitive::usize, SDL_FunctionPointer>(_beginthreadex) }};"
    )?;
    writeln!(ctx, "{cfg_win}")?;
    writeln!(
        ctx,
        "pub const SDL_EndThreadFunction: SDL_FunctionPointer = unsafe {{ ::core::mem::transmute::<unsafe extern \"cdecl\" fn (::core::ffi::c_uint), SDL_FunctionPointer>(_endthreadex) }};"
    )?;
    writeln!(ctx)?;
    Ok(())
}

pub fn patch_emit_macro_call(
    ctx: &mut EmitContext,
    ident: &str,
    call: &FnCall,
) -> Result<bool, EmitErr> {
    patch_emit(ctx, call, ident, EMIT_MACRO_CALL_PATCHES)
}

type EmitMacroCallPatch = EmitPatch<FnCall>;

const EMIT_MACRO_CALL_PATCHES: &[EmitMacroCallPatch] = &[
    EmitMacroCallPatch {
        module: None,
        match_ident: |i| i == "SDL_COMPILE_TIME_ASSERT",
        patch: |ctx, call| {
            write!(ctx, "const _: () = ")?;
            call.try_eval(ctx)?.emit(ctx)?;
            writeln!(ctx, ";")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitMacroCallPatch {
        module: Some("vulkan"),
        match_ident: |i| i == "VK_DEFINE_HANDLE",
        patch: |ctx, call| {
            let Expr::Ident(arg) = &call.args[0] else {
                panic!()
            };
            writeln!(ctx, "pub type {arg} = *mut __{arg};")?;
            writeln!(ctx)?;
            writeln!(ctx, "#[repr(C)]")?;
            writeln!(ctx, "#[non_exhaustive]")?;
            writeln!(
                ctx,
                "pub struct __{arg} {{ _opaque: [::core::primitive::u8; 0] }}",
            )?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
    EmitMacroCallPatch {
        module: Some("vulkan"),
        match_ident: |i| i == "VK_DEFINE_NON_DISPATCHABLE_HANDLE",
        patch: |ctx, call| {
            let Expr::Ident(arg) = &call.args[0] else {
                panic!()
            };
            writeln!(ctx, r#"#[cfg(target_pointer_width = "64")]"#)?;
            writeln!(ctx, "pub type {arg} = *mut __{arg};")?;
            writeln!(ctx)?;
            writeln!(ctx, r#"#[cfg(target_pointer_width = "64")]"#)?;
            writeln!(ctx, "#[repr(C)]")?;
            writeln!(ctx, "#[non_exhaustive]")?;
            writeln!(
                ctx,
                "pub struct __{arg} {{ _opaque: [::core::primitive::u8; 0] }}",
            )?;
            writeln!(ctx)?;
            writeln!(ctx, r#"#[cfg(not(target_pointer_width = "64"))]"#)?;
            writeln!(ctx, "pub type {arg} = ::core::primitive::u64;")?;
            writeln!(ctx)?;
            Ok(true)
        },
    },
];

pub fn patch_eval_macro_call(
    ctx: &EmitContext,
    ident: &str,
    call: &FnCall,
) -> Result<Option<Value>, EmitErr> {
    patch_eval(ctx, call, ident, EVAL_MACRO_CALL_PATCHES)
}

type EvalMacroCallPatch = EvalPatch<FnCall>;

const EVAL_MACRO_CALL_PATCHES: &[EvalMacroCallPatch] = &[
    EvalMacroCallPatch {
        matches: |ctx, i| ctx.is_preproc_eval_mode() && i == "defined",
        patch: |ctx, call| {
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
            if ctx.preproc_state().borrow().is_target_define(&define) {
                Ok(Some(Value::TargetDependent(DefineState::defined(define))))
            } else if ctx.preproc_state().borrow().is_defined(&define)? {
                Ok(Some(Value::Bool(true)))
            } else {
                Ok(Some(Value::Bool(false)))
            }
        },
    },
    EvalMacroCallPatch {
        matches: |_, i| i == "SDL_COMPILE_TIME_ASSERT",
        patch: |ctx, call| match call.args[1].try_eval(ctx)? {
            Some(Value::RustCode(mut rc)) if call.args.len() == 2 => {
                rc.value.insert_str(0, "::core::assert!(");
                rc.value.push(')');
                rc.ty = Type::void();
                Ok(Some(Value::RustCode(rc)))
            }
            _ => Err(ParseErr::new(call.span(), "invalid assert").into()),
        },
    },
    EvalMacroCallPatch {
        matches: |_, i| i == "SDL_HAS_BUILTIN",
        patch: |_, call| {
            let args = &*call.args;
            let err = || {
                Err(ParseErr::new(
                    call.span(),
                    "SDL_HAS_BUILTIN takes one argument of type ident",
                )
                .into())
            };
            if args.len() != 1 {
                return err();
            }
            let Expr::Ident(builtin) = &args[0] else {
                return err();
            };
            Ok(Some(Value::Bool(match builtin.as_str() {
                "__builtin_add_overflow" | "__builtin_mul_overflow" => false,
                _ => return Err(ParseErr::new(builtin.span(), "unknown builtin").into()),
            })))
        },
    },
    EvalMacroCallPatch {
        matches: |_, i| i == "SDL_SINT64_C",
        patch: |ctx, call| {
            assert!(call.args.len() == 1);
            let Some(arg) = call.args[0].try_eval(ctx)? else {
                return Ok(None);
            };
            Ok(Some(match arg {
                Value::I32(i) => Value::I64(i as i64),
                Value::U31(u) | Value::U32(u) => Value::I64(u as i64),
                Value::I64(i) => Value::I64(i),
                Value::U63(u) | Value::U64(u) => Value::I64(u as i64),
                _ => todo!(),
            }))
        },
    },
    EvalMacroCallPatch {
        matches: |_, i| i == "SDL_UINT64_C",
        patch: |ctx, call| {
            assert!(call.args.len() == 1);
            let Some(arg) = call.args[0].try_eval(ctx)? else {
                return Ok(None);
            };
            Ok(Some(match arg {
                Value::I32(i) => Value::U64(i as u64),
                Value::U31(u) | Value::U32(u) => Value::U64(u as u64),
                Value::I64(i) => Value::U64(i as u64),
                Value::U63(u) | Value::U64(u) => Value::U64(u),
                _ => todo!(),
            }))
        },
    },
];

fn patch_emit<T: ?Sized>(
    ctx: &mut EmitContext,
    arg: &T,
    ident: &str,
    patches: &[EmitPatch<T>],
) -> Result<bool, EmitErr> {
    if ctx.patch_enabled() {
        let _guard = ctx.disable_patch_guard();
        for patch in patches.iter() {
            if (patch.module.is_none() || patch.module == Some(&*ctx.module()))
                && (patch.match_ident)(ident)
            {
                return (patch.patch)(ctx, arg);
            }
        }
    }
    Ok(false)
}

fn patch_eval<T: ?Sized>(
    ctx: &EmitContext,
    arg: &T,
    ident: &str,
    patches: &[EvalPatch<T>],
) -> Result<Option<Value>, EmitErr> {
    for patch in patches.iter() {
        if (patch.matches)(ctx, ident) {
            return (patch.patch)(ctx, arg);
        }
    }
    Ok(None)
}
