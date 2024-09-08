use super::{EmitContext, EmitErr};
use crate::parse::Expr;
use core::fmt::Write;

pub struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    ident: &'static str,
    patch: fn(&mut EmitContext, &T) -> Result<bool, EmitErr>,
}

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
