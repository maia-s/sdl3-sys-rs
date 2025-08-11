use super::{
    CanCmp, CanCopy, Cast, Define, DefineValue, Enum, EnumKind, Expr, Function, GetSpan,
    ParseContext, ParseErr, PrimitiveType, StructOrUnion, Type, TypeDef, TypeDefKind,
};

pub fn patch_parsed_define(ctx: &ParseContext, define: &mut Define) -> Result<bool, ParseErr> {
    match (ctx.module(), define.ident.as_str()) {
        ("audio", "SDL_AUDIO_BITSIZE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioFormat");
            Ok(true)
        }
        ("audio", "SDL_AUDIO_FRAMESIZE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioSpec");
            Ok(true)
        }
        ("audio", i) if i.starts_with("SDL_AUDIO_IS") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioFormat");
            define.value = define.value.cast_expr(Type::bool());
            Ok(true)
        }
        ("audio", "SDL_DEFINE_AUDIO_FORMAT") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::bool();
            args[1].ty = Type::bool();
            args[2].ty = Type::bool();
            args[3].ty = Type::primitive(PrimitiveType::Uint8T);
            define.value = define.value.cast_expr(Type::ident_str("SDL_AudioFormat"));
            Ok(true)
        }
        ("error", "SDL_InvalidParamError") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::primitive(PrimitiveType::Char), true);
            Ok(true)
        }
        (
            "haptic",
            "SDL_HAPTIC_CARTESIAN"
            | "SDL_HAPTIC_POLAR"
            | "SDL_HAPTIC_SPHERICAL"
            | "SDL_HAPTIC_STEERING_AXIS",
        ) => {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        }
        (
            "haptic",
            "SDL_HAPTIC_CONSTANT"
            | "SDL_HAPTIC_CUSTOM"
            | "SDL_HAPTIC_DAMPER"
            | "SDL_HAPTIC_FRICTION"
            | "SDL_HAPTIC_INERTIA"
            | "SDL_HAPTIC_LEFTRIGHT"
            | "SDL_HAPTIC_RAMP"
            | "SDL_HAPTIC_RESERVED1"
            | "SDL_HAPTIC_RESERVED2"
            | "SDL_HAPTIC_RESERVED3"
            | "SDL_HAPTIC_SAWTOOTHDOWN"
            | "SDL_HAPTIC_SAWTOOTHUP"
            | "SDL_HAPTIC_SINE"
            | "SDL_HAPTIC_SPRING"
            | "SDL_HAPTIC_SQUARE"
            | "SDL_HAPTIC_TRIANGLE",
        ) => {
            define.value = define.value.cast_expr(Type::ident_str("Uint16"));
            Ok(true)
        }
        ("joystick", i) if i.starts_with("SDL_HAT_") => {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        }
        ("joystick", "SDL_JOYSTICK_AXIS_MAX" | "SDL_JOYSTICK_AXIS_MIN") => {
            define.value = define.value.cast_expr(Type::ident_str("Sint16"));
            Ok(true)
        }
        ("keycode", "SDL_SCANCODE_TO_KEYCODE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_Scancode");
            Ok(true)
        }
        ("mouse", "SDL_BUTTON_MASK") => {
            define.value = define
                .value
                .cast_expr(Type::ident_str("SDL_MouseButtonFlags"));
            Ok(true)
        }
        ("mutex", _) => {
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
        }
        ("pixels", "SDL_ALPHA_OPAQUE" | "SDL_ALPHA_TRANSPARENT") => {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        }
        ("pixels", "SDL_BITSPERPIXEL" | "SDL_BYTESPERPIXEL") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define
                .value
                .cast_expr(Type::primitive(PrimitiveType::Uint8T));
            Ok(true)
        }
        ("pixels", i) if i.starts_with("SDL_COLORSPACE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_Colorspace");
            Ok(true)
        }
        ("pixels", "SDL_DEFINE_COLORSPACE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_ColorType");
            args[1].ty = Type::ident_str("SDL_ColorRange");
            args[2].ty = Type::ident_str("SDL_ColorPrimaries");
            args[3].ty = Type::ident_str("SDL_TransferCharacteristics");
            args[4].ty = Type::ident_str("SDL_MatrixCoefficients");
            args[5].ty = Type::ident_str("SDL_ChromaLocation");
            define.value = define.value.cast_expr(Type::ident_str("SDL_Colorspace"));
            Ok(true)
        }
        ("pixels", "SDL_DEFINE_PIXELFORMAT") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelType");
            args[1].ty = Type::primitive(PrimitiveType::Int); // inner type of order enums
            args[2].ty = Type::ident_str("SDL_PackedLayout");
            args[3].ty = Type::primitive(PrimitiveType::Uint8T);
            args[4].ty = Type::primitive(PrimitiveType::Uint8T);
            define.value = define.value.cast_expr(Type::ident_str("SDL_PixelFormat"));
            Ok(true)
        }
        ("pixels", "SDL_PIXELFLAG" | "SDL_PIXELORDER") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            Ok(true)
        }
        ("pixels", "SDL_PIXELLAYOUT") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define.value.cast_expr(Type::ident_str("SDL_PackedLayout"));
            Ok(true)
        }
        ("pixels", "SDL_PIXELTYPE") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define.value.cast_expr(Type::ident_str("SDL_PixelType"));
            Ok(true)
        }
        ("render", i) if i.starts_with("SDL_RENDERER_VSYNC_") => {
            define.value = define.value.cast_expr(Type::primitive(PrimitiveType::Int));
            Ok(true)
        }
        ("stdinc", "SDL_iconv_wchar_utf8") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::primitive(PrimitiveType::WcharT), true);
            Ok(true)
        }
        ("surface", "SDL_MUSTLOCK") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::ident_str("SDL_Surface"), true);
            Ok(true)
        }
        ("system", i) if i.starts_with("SDL_ANDROID_EXTERNAL_STORAGE_") => {
            define.value = define.value.cast_expr(Type::ident_str("Uint32"));
            Ok(true)
        }
        ("timer", i) if i.starts_with("SDL_NS_TO_") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("Uint64");
            Ok(true)
        }
        ("video", "SDL_WINDOWPOS_CENTERED_DISPLAY" | "SDL_WINDOWPOS_UNDEFINED_DISPLAY") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_DisplayID");
            define.value = define.value.cast_expr(Type::primitive(PrimitiveType::Int));
            Ok(true)
        }
        ("video", "SDL_WINDOWPOS_ISCENTERED" | "SDL_WINDOWPOS_ISUNDEFINED") => {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::primitive(PrimitiveType::Int);
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn patch_parsed_enum(ctx: &ParseContext, e: &mut Enum) -> Result<bool, ParseErr> {
    match (
        ctx.module(),
        e.ident.as_ref().map(|i| i.as_str()).unwrap_or(""),
    ) {
        ("audio", "SDL_AudioFormat") => {
            e.base_type = Some(Type::primitive(PrimitiveType::UnsignedInt));
            Ok(true)
        }
        ("events", "SDL_EventType") => {
            e.base_type = Some(Type::ident_str("Uint32"));
            Ok(true)
        }
        ("pixels", "SDL_Colorspace") => {
            e.base_type = Some(Type::ident_str("Uint32"));
            Ok(true)
        }
        (
            "pixels",
            "SDL_ChromaLocation"
            | "SDL_ColorPrimaries"
            | "SDL_ColorRange"
            | "SDL_ColorType"
            | "SDL_MatrixCoefficients"
            | "SDL_TransferCharacteristics",
        ) => {
            e.base_type = Some(Type::primitive(PrimitiveType::UnsignedInt));
            Ok(true)
        }
        ("stdinc", "SDL_DUMMY_ENUM") => {
            e.hidden = true;
            e.emit_metadata = false;
            Ok(true)
        }
        ("ttf", "TTF_Direction") => {
            e.base_type = Some(Type::primitive(PrimitiveType::Uint32T));
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn patch_parsed_function(ctx: &ParseContext, f: &mut Function) -> Result<bool, ParseErr> {
    // TODO/FIXME: need fix/info for safe: events, filesystem, gamepad, gpu
    match (ctx.module(), f.ident.as_str()) {
        ("asyncio", "SDL_CreateAsyncIOQueue")
        | (
            "audio", // audio functions properly check if system is inited
            "SDL_AudioDevicePaused"
            | "SDL_CloseAudioDevice"
            | "SDL_GetAudioDeviceGain"
            | "SDL_GetAudioDeviceName"
            | "SDL_GetAudioDriver"
            | "SDL_GetAudioFormatName"
            | "SDL_GetCurrentAudioDriver"
            | "SDL_GetNumAudioDrivers"
            | "SDL_GetSilenceValueForFormat"
            | "SDL_IsAudioDevicePhysical"
            | "SDL_IsAudioDevicePlayback"
            | "SDL_PauseAudioDevice"
            | "SDL_ResumeAudioDevice"
            | "SDL_SetAudioDeviceGain",
        )
        | ("blendmode", "SDL_ComposeCustomBlendMode")
        | (
            "camera", // camera functions properly check if system is inited
            "SDL_GetCameraDriver"
            | "SDL_GetCameraName"
            | "SDL_GetCameraPosition"
            | "SDL_GetCurrentCameraDriver"
            | "SDL_GetNumCameraDrivers",
        )
        | ("error", "SDL_ClearError" | "SDL_GetError" | "SDL_OutOfMemory" | "SDL_Unsupported")
        | ("init", "SDL_InitSubSystem" | "SDL_IsMainThread" | "SDL_WasInit")
        | (
            "stdinc",
            "SDL_abs"
            | "SDL_absf"
            | "SDL_acos"
            | "SDL_acosf"
            | "SDL_asin"
            | "SDL_asinf"
            | "SDL_atan"
            | "SDL_atan2"
            | "SDL_atan2f"
            | "SDL_atanf"
            | "SDL_ceil"
            | "SDL_ceilf"
            | "SDL_copysign"
            | "SDL_copysignf"
            | "SDL_cos"
            | "SDL_cosf"
            | "SDL_exp"
            | "SDL_expf"
            | "SDL_fabs"
            | "SDL_fabsf"
            | "SDL_floor"
            | "SDL_floorf"
            | "SDL_fmod"
            | "SDL_fmodf"
            | "SDL_GetNumAllocations"
            | "SDL_isalnum"
            | "SDL_isalpha"
            | "SDL_isblank"
            | "SDL_iscntrl"
            | "SDL_isdigit"
            | "SDL_isgraph"
            | "SDL_isinf"
            | "SDL_isinff"
            | "SDL_islower"
            | "SDL_isnan"
            | "SDL_isnanf"
            | "SDL_isprint"
            | "SDL_ispunct"
            | "SDL_isspace"
            | "SDL_isupper"
            | "SDL_isxdigit"
            | "SDL_log"
            | "SDL_log10"
            | "SDL_log10f"
            | "SDL_logf"
            | "SDL_lround"
            | "SDL_lroundf"
            | "SDL_pow"
            | "SDL_powf"
            | "SDL_round"
            | "SDL_roundf"
            | "SDL_scalbn"
            | "SDL_scalbnf"
            | "SDL_sin"
            | "SDL_sinf"
            | "SDL_sqrt"
            | "SDL_sqrtf"
            | "SDL_tan"
            | "SDL_tanf"
            | "SDL_tolower"
            | "SDL_toupper"
            | "SDL_trunc"
            | "SDL_truncf",
        ) => {
            f.is_unsafe = false;
            Ok(true)
        }
        ("cpuinfo", i) if i.starts_with("SDL_Get") || i.starts_with("SDL_Has") => {
            f.is_unsafe = false;
            Ok(true)
        }
        (_, i) if i.ends_with("_Version") || i.ends_with("_GetVersion") || i.ends_with("_Init") => {
            // FIXME: Should Quit functions be safe?
            f.is_unsafe = false;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn patch_parsed_struct(ctx: &ParseContext, s: &mut StructOrUnion) -> Result<bool, ParseErr> {
    match (
        ctx.module(),
        s.ident.as_ref().map(|i| i.as_str()).unwrap_or(""),
    ) {
        (_, i) if i.starts_with('_') => {
            s.hidden = true;
            Ok(true)
        }
        ("atomic", "SDL_AtomicInt" | "SDL_AtomicU32") => {
            // atomic
            s.can_copy = CanCopy::Never;
            s.can_eq = CanCmp::No;
            Ok(true)
        }
        ("events", "SDL_ClipboardEvent" | "SDL_UserEvent") => {
            // part of union
            s.can_copy = CanCopy::Always;
            Ok(true)
        }
        (
            "gpu",
            "SDL_GPUBlitRegion"
            | "SDL_GPUBufferBinding"
            | "SDL_GPUBufferLocation"
            | "SDL_GPUBufferRegion"
            | "SDL_GPUColorTargetInfo"
            | "SDL_GPUDepthStencilTargetInfo"
            | "SDL_GPUGraphicsPipelineCreateInfo"
            | "SDL_GPUStorageBufferReadWriteBinding"
            | "SDL_GPUStorageTextureReadWriteBinding"
            | "SDL_GPUTextureLocation"
            | "SDL_GPUTextureRegion"
            | "SDL_GPUTextureSamplerBinding"
            | "SDL_GPUTextureTransferInfo"
            | "SDL_GPUTransferBufferLocation",
        ) => {
            // pointers aren't uniquely owned
            s.can_copy = CanCopy::Always;
            Ok(true)
        }
        ("haptic", "SDL_HapticCustom") => {
            // this must be copy because it's part of a union
            s.can_copy = CanCopy::Always;
            Ok(true)
        }
        ("stdinc", "SDL_alignment_test") | ("system", "tagMSG") => {
            s.hidden = true;
            Ok(true)
        }
        ("textengine", "TTF_CopyOperation") => {
            // part of union
            s.can_copy = CanCopy::Always;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn patch_parsed_typedef(ctx: &ParseContext, td: &mut TypeDef) -> Result<bool, ParseErr> {
    match (ctx.module(), td.ident.as_str()) {
        ("atomic", "SDL_SpinLock") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Lock);
            Ok(true)
        }
        ("blendmode", "SDL_BlendMode") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Flags);
            Ok(true)
        }
        ("gpu", "SDL_GPUShaderFormat") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Flags);
            Ok(true)
        }
        ("haptic", "SDL_HapticDirectionType") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Enum);
            Ok(true)
        }
        ("haptic", "SDL_HapticEffectType") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Flags);
            Ok(true)
        }
        ("keycode", "SDL_Keymod") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Flags);
            Ok(true)
        }
        ("keycode", "SDL_Keycode") => {
            td.kind = TypeDefKind::new_enum(EnumKind::Id);
            Ok(true)
        }
        ("mouse", "SDL_MouseButtonFlags") => {
            let TypeDefKind::Enum { match_define, .. } = &mut td.kind else {
                unreachable!()
            };
            *match_define = |s| s.starts_with("SDL_BUTTON_") && s.ends_with("MASK");
            Ok(true)
        }
        ("pen", "SDL_PenID") => {
            let TypeDefKind::Enum { match_define, .. } = &mut td.kind else {
                unreachable!()
            };
            *match_define = |_| false;
            Ok(true)
        }
        ("sensor", "SDL_SensorID") => {
            let TypeDefKind::Enum { match_define, .. } = &mut td.kind else {
                unreachable!()
            };
            *match_define = |_| false;
            Ok(true)
        }
        ("video", "SDL_WindowID") => {
            let TypeDefKind::Enum { match_define, .. } = &mut td.kind else {
                unreachable!()
            };
            *match_define = |_| false;
            Ok(true)
        }
        ("video", "SDL_WindowFlags") => {
            let TypeDefKind::Enum { match_define, .. } = &mut td.kind else {
                unreachable!()
            };
            *match_define = |s| s.starts_with("SDL_WINDOW_");
            Ok(true)
        }
        (
            "video",
            "SDL_GLContextFlag"
            | "SDL_GLContextReleaseFlag"
            | "SDL_GLContextResetNotification"
            | "SDL_GLProfile",
        ) => {
            td.ty = Type::ident_str("Sint32");
            td.kind = TypeDefKind::new_enum(EnumKind::Flags);
            Ok(true)
        }
        ("ttf", "TTF_HintingFlags") => {
            td.kind = TypeDefKind::Alias;
            Ok(true)
        }
        _ => Ok(false),
    }
}

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
