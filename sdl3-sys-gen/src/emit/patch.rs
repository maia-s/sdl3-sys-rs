use super::{Emit, EmitContext, EmitErr};
use crate::parse::{Define, Expr, PrimitiveType, Type};
use core::fmt::Write;

pub struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&mut EmitContext, &T) -> Result<bool, EmitErr>,
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[DefinePatch] = &[
    DefinePatch {
        module: Some("joystick"),
        match_ident: |i| i.starts_with("SDL_HAT_"),
        patch: |ctx, define| {
            let mut define = define.clone();
            define.value = define
                .value
                .cast_expr(Type::primitive(PrimitiveType::Uint8T));
            define.emit(ctx)?;
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

type MacroCallPatch = Patch<[Expr]>;

const MACRO_CALL_PATCHES: &[MacroCallPatch] = &[
    MacroCallPatch {
        module: None,
        match_ident: |i| i == "SDL_COMPILE_TIME_ASSERT",
        patch: |ctx, args| {
            let Expr::Ident(id) = &args[0] else { panic!() };
            if id.as_str() == "SDL_Event" {
                writeln!(
                    ctx,
                    "const _: () = ::core::assert!(::core::mem::size_of::<SDL_Event>() == 128);"
                )?;
                writeln!(ctx)?;
                Ok(true)
            } else {
                Ok(false)
            }
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

pub fn patch_define(ctx: &mut EmitContext, define: &Define) -> Result<bool, EmitErr> {
    if ctx.patch_enabled() {
        let _guard = ctx.disable_patch_guard();
        for patch in DEFINE_PATCHES.iter() {
            if (patch.module.is_none() || patch.module == Some(&*ctx.module()))
                && (patch.match_ident)(define.ident.as_str())
            {
                return (patch.patch)(ctx, define);
            }
        }
    }
    Ok(false)
}

pub fn patch_macro_call(
    ctx: &mut EmitContext,
    ident: &str,
    args: &[Expr],
) -> Result<bool, EmitErr> {
    if ctx.patch_enabled() {
        let _guard = ctx.disable_patch_guard();
        for patch in MACRO_CALL_PATCHES.iter() {
            if (patch.module.is_none() || patch.module == Some(&*ctx.module()))
                && (patch.match_ident)(ident)
            {
                return (patch.patch)(ctx, args);
            }
        }
    }
    Ok(false)
}
