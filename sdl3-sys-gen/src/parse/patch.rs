use super::{Define, ParseContext, ParseErr, PrimitiveType, Type};

struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&ParseContext, &mut T) -> Result<bool, ParseErr>,
}

pub fn patch_parsed_define(ctx: &ParseContext, define: &mut Define) -> Result<bool, ParseErr> {
    patch(ctx, define, |d| d.ident.as_str(), DEFINE_PATCHES)
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[Patch<Define>] = &[DefinePatch {
    module: Some("joystick"),
    match_ident: |i| i.starts_with("SDL_HAT_"),
    patch: |_ctx, define| {
        define.value = define
            .value
            .cast_expr(Type::primitive(PrimitiveType::Uint8T));
        Ok(true)
    },
}];

fn patch<T: ?Sized>(
    ctx: &ParseContext,
    parsed: &mut T,
    get_ident: impl Fn(&T) -> &str,
    patches: &[Patch<T>],
) -> Result<bool, ParseErr> {
    for patch in patches.iter() {
        if (patch.module.is_none() || patch.module == Some(ctx.module()))
            && (patch.match_ident)(get_ident(parsed))
        {
            return (patch.patch)(ctx, parsed);
        }
    }
    Ok(false)
}
