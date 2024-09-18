use super::{Emit, EmitContext, EmitErr, Eval, Value};
use crate::parse::{Define, Expr, Function};
use core::fmt::Write;

struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&mut EmitContext, &T) -> Result<bool, EmitErr>,
}

pub fn patch_emit_function(ctx: &mut EmitContext, f: &Function) -> Result<bool, EmitErr> {
    patch(ctx, f, f.ident.as_str(), FUNCTION_PATCHES)
}

type FunctionPatch = Patch<Function>;

const FUNCTION_PATCHES: &[FunctionPatch] = &[FunctionPatch {
    // skip emitting these
    module: None,
    match_ident: |i| matches!(i, "__debugbreak" | "_ReadWriteBarrier"),
    patch: |_, _| Ok(true),
}];

pub fn patch_emit_define(ctx: &mut EmitContext, define: &Define) -> Result<bool, EmitErr> {
    patch(ctx, define, define.ident.as_str(), DEFINE_PATCHES)
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[DefinePatch] = &[
    DefinePatch {
        // skip emitting these
        module: None,
        match_ident: |i| {
            matches!(
                i,
                "SDL_InvalidParamError" | "SDL_TriggerBreakpoint" | "SDL_zeroa"
            )
        },
        patch: |_, _| Ok(true),
    },
    DefinePatch {
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
    DefinePatch {
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
];

pub fn patch_emit_macro_call(
    ctx: &mut EmitContext,
    ident: &str,
    args: &[Expr],
) -> Result<bool, EmitErr> {
    patch(ctx, args, ident, MACRO_CALL_PATCHES)
}

type MacroCallPatch = Patch<[Expr]>;

const MACRO_CALL_PATCHES: &[MacroCallPatch] = &[
    MacroCallPatch {
        module: None,
        match_ident: |i| i == "SDL_COMPILE_TIME_ASSERT",
        patch: |ctx, args| match args[1].try_eval(ctx)? {
            Some(Value::RustCode(s)) => {
                writeln!(ctx, "const _: () = ::core::assert!({s});")?;
                writeln!(ctx)?;
                Ok(true)
            }
            _ => Ok(false),
        },
    },
    MacroCallPatch {
        module: Some("vulkan"),
        match_ident: |i| i == "VK_DEFINE_HANDLE",
        patch: |ctx, args| {
            let Expr::Ident(arg) = &args[0] else { panic!() };
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
    MacroCallPatch {
        module: Some("vulkan"),
        match_ident: |i| i == "VK_DEFINE_NON_DISPATCHABLE_HANDLE",
        patch: |ctx, args| {
            let Expr::Ident(arg) = &args[0] else { panic!() };
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

fn patch<T: ?Sized>(
    ctx: &mut EmitContext,
    arg: &T,
    ident: &str,
    patches: &[Patch<T>],
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
