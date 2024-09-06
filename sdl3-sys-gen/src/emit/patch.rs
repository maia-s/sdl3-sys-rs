use super::{EmitContext, EmitErr};
use crate::parse::{Ident, StructKind, StructOrUnion};
use core::fmt::Write;

pub struct CompileTimeAssertPatch {
    module: &'static str,
    id: &'static str,
    replace_output: &'static str,
}

const SDL_COMPILE_TIME_ASSERT_PATCHES: &[CompileTimeAssertPatch] = &[CompileTimeAssertPatch {
    module: "events",
    id: "SDL_Event",
    replace_output: "const _: () = ::core::assert!(::core::mem::size_of(SDL_Event) == 128);",
}];

pub struct MacroCallPatch {
    module: &'static str,
    ident: &'static str,
    patch: fn(&mut EmitContext, &str) -> Result<(), EmitErr>,
}

const MACRO_CALL_PATCHES: &[MacroCallPatch] = &[
    MacroCallPatch {
        module: "vulkan",
        ident: "VK_DEFINE_HANDLE",
        patch: |ctx, arg| {
            writeln!(ctx, "pub type {arg} = *mut __{arg};")?;
            writeln!(ctx)?;
            writeln!(ctx, "#[repr(C)]")?;
            writeln!(ctx, "#[non_exhaustive]")?;
            writeln!(
                ctx,
                "pub struct __{arg} {{ _opaque: [::core::primitive::u8; 0] }}",
            )?;
            writeln!(ctx)?;
            Ok(())
        },
    },
    MacroCallPatch {
        module: "vulkan",
        ident: "VK_DEFINE_NON_DISPATCHABLE_HANDLE",
        patch: |ctx, arg| {
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
            writeln!(ctx, r#"#[not(cfg(target_pointer_width = "64"))]"#)?;
            writeln!(ctx, "pub type {arg} = ::core::primitive::u64;")?;
            writeln!(ctx)?;
            Ok(())
        },
    },
];

pub fn patch_sdl_compile_time_assert(ctx: &mut EmitContext, id: &str) -> Result<bool, EmitErr> {
    for patch in SDL_COMPILE_TIME_ASSERT_PATCHES.iter() {
        if patch.module == &*ctx.module() && patch.id == id {
            writeln!(ctx, "{}", patch.replace_output)?;
            writeln!(ctx)?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn patch_macro_call(ctx: &mut EmitContext, ident: &str, arg: &str) -> Result<bool, EmitErr> {
    for patch in MACRO_CALL_PATCHES.iter() {
        if patch.module == &*ctx.module() && patch.ident == ident {
            (patch.patch)(ctx, arg)?;
            return Ok(true);
        }
    }
    Ok(false)
}
