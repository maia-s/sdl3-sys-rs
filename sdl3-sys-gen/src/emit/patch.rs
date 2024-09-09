use super::{Emit, EmitContext, EmitErr};
use crate::parse::{Define, Expr};
use core::fmt::Write;

pub struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    ident: &'static str,
    patch: fn(&mut EmitContext, &T) -> Result<bool, EmitErr>,
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[DefinePatch] = &[DefinePatch {
    module: Some("stdinc"),
    ident: "SDL_INIT_INTERFACE",
    patch: |ctx, define| {
        define.doc.emit(ctx)?;
        writeln!(ctx, "///")?;
        writeln!(ctx, "/// # Safety")?;
        writeln!(ctx, "/// `iface` must point to an SDL interface struct")?;
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
            "iface.cast::<::core::primitive::u32>().write(::core::mem::size_of::<T>() as ::core::primitive::u32);"
        )?;
        ctx.decrease_indent();
        writeln!(ctx, "}}")?;
        ctx.decrease_indent();
        writeln!(ctx, "}}")?;
        writeln!(ctx)?;
        Ok(true)
    },
}];

type MacroCallPatch = Patch<[Expr]>;

const MACRO_CALL_PATCHES: &[MacroCallPatch] = &[
    MacroCallPatch {
        module: None,
        ident: "SDL_COMPILE_TIME_ASSERT",
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
        ident: "VK_DEFINE_HANDLE",
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
        ident: "VK_DEFINE_NON_DISPATCHABLE_HANDLE",
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
    for patch in DEFINE_PATCHES.iter() {
        if (patch.module.is_none() || patch.module == Some(&*ctx.module()))
            && patch.ident == define.ident.as_str()
        {
            return (patch.patch)(ctx, define);
        }
    }
    Ok(false)
}

pub fn patch_macro_call(
    ctx: &mut EmitContext,
    ident: &str,
    args: &[Expr],
) -> Result<bool, EmitErr> {
    for patch in MACRO_CALL_PATCHES.iter() {
        if (patch.module.is_none() || patch.module == Some(&*ctx.module())) && patch.ident == ident
        {
            return (patch.patch)(ctx, args);
        }
    }
    Ok(false)
}
