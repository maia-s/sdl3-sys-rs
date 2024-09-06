use super::{EmitContext, EmitErr};
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
