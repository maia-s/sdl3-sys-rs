use super::{
    Cast, Define, DefineValue, Enum, Expr, GetSpan, Ident, ParseContext, ParseErr, PrimitiveType,
    Type,
};

struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&ParseContext, &mut T) -> Result<bool, ParseErr>,
}

pub fn patch_parsed_define(ctx: &ParseContext, define: &mut Define) -> Result<bool, ParseErr> {
    patch(ctx, define, |d| d.ident.as_str(), DEFINE_PATCHES)
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[Patch<Define>] = &[
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_AUDIO_BITSIZE",
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::ident(Ident::new_inline("SDL_AudioFormat"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_AUDIO_FRAMESIZE",
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::ident(Ident::new_inline("SDL_AudioSpec"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("joystick"),
        match_ident: |i| i.starts_with("SDL_HAT_"),
        patch: |_ctx, define| {
            define.value = define
                .value
                .cast_expr(Type::primitive(PrimitiveType::Uint8T));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("mouse"),
        match_ident: |i| matches!(i, "SDL_BUTTON"),
        patch: |_ctx, define| {
            define.value = define
                .value
                .cast_expr(Type::ident(Ident::new_inline("SDL_MouseButtonFlags")));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("mutex"),
        match_ident: |_| true,
        patch: |_, define| {
            if let DefineValue::Expr(Expr::FnCall(call)) = &define.value {
                if let Expr::Ident(ident) = &*call.func {
                    if matches!(
                        ident.as_str(),
                        "__attribute__" | "SDL_THREAD_ANNOTATION_ATTRIBUTE__"
                    ) {
                        define.value = DefineValue::Empty;
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_DEFINE_PIXELFORMAT"),
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            assert!(args[0].ident.as_str() == "type");
            args[0].ty = Type::ident(Ident::new_inline("SDL_PixelType"));
            assert!(args[1].ident.as_str() == "order");
            //args[1].ty = !!! FIXME
            assert!(args[2].ident.as_str() == "layout");
            args[2].ty = Type::ident(Ident::new_inline("SDL_PackedLayout"));
            define.value = define
                .value
                .cast_expr(Type::ident(Ident::new_inline("SDL_PixelFormat")));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_PIXELFLAG" | "SDL_PIXELORDER" | "SDL_BITSPERPIXEL" | "SDL_BYTESPERPIXEL"
            )
        },
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::ident(Ident::new_inline("SDL_PixelFormat"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_PIXELLAYOUT"),
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::ident(Ident::new_inline("SDL_PixelFormat"));
            define.value = define
                .value
                .cast_expr(Type::ident(Ident::new_inline("SDL_PackedLayout")));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_PIXELTYPE"),
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::ident(Ident::new_inline("SDL_PixelFormat"));
            define.value = define
                .value
                .cast_expr(Type::ident(Ident::new_inline("SDL_PixelType")));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_iconv_wchar_utf8",
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::pointer(Type::primitive(PrimitiveType::WcharT), true);
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("surface"),
        match_ident: |i| i == "SDL_MUSTLOCK",
        patch: |_ctx, define| {
            let Some(args) = &mut define.args else {
                unreachable!()
            };
            args[0].ty = Type::pointer(Type::ident(Ident::new_inline("SDL_Surface")), true);
            Ok(true)
        },
    },
];

pub fn patch_parsed_enum(ctx: &ParseContext, e: &mut Enum) -> Result<bool, ParseErr> {
    patch(
        ctx,
        e,
        |e| e.ident.as_ref().map(|i| i.as_str()).unwrap_or(""),
        ENUM_PATCHES,
    )
}

type EnumPatch = Patch<Enum>;

const ENUM_PATCHES: &[EnumPatch] = &[EnumPatch {
    module: Some("events"),
    match_ident: |i| i == "SDL_EventType",
    patch: |_, e| {
        e.base_type = Some(Type::ident(Ident::new_inline("Uint32")));
        Ok(true)
    },
}];

pub fn patch_parsed_expr(_ctx: &ParseContext, expr: &mut Expr) -> Result<bool, ParseErr> {
    #[allow(clippy::single_match)]
    match expr {
        Expr::FnCall(f) => match &*f.func {
            Expr::Ident(i) => match i.as_str() {
                "SDL_const_cast" | "SDL_reinterpret_cast" | "SDL_static_cast" => {
                    let Expr::Ident(ty) = f.args[0].clone() else {
                        todo!()
                    };
                    *expr = Expr::Cast(Cast::boxed(
                        f.span(),
                        Type::ident(ty.try_into().unwrap()),
                        f.args[1].clone(),
                    ));
                    return Ok(true);
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
    Ok(false)
}

fn patch<T: ?Sized>(
    ctx: &ParseContext,
    parsed: &mut T,
    get_ident: impl Fn(&T) -> &str,
    patches: &[Patch<T>],
) -> Result<bool, ParseErr> {
    for patch in patches.iter() {
        if (patch.module.is_none() || patch.module == Some(ctx.module()))
            && (patch.match_ident)(get_ident(parsed))
            && (patch.patch)(ctx, parsed)?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
