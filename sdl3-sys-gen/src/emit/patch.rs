use super::{
    Cfg, DefineState, Emit, EmitContext, EmitErr, EmitResult, Eval, StructSym, SymKind, Value,
};
use crate::parse::{
    Block, Define, DefineValue, Expr, FnCall, Function, GetSpan, Ident, IdentOrKw, Item, Items,
    Kw_static, ParseErr, RustCode, Span, StringLiteral, Type, TypeDef,
};
use core::fmt::Write;
use std::ffi::CString;
use str_block::str_block;

const VULKAN_CRATE_VERSIONS: &[(&str, &[&str])] = &[("ash", &["0.38"])];
const WINDOWS_CRATE_VERSIONS: &[(&str, &[&str])] = &[("windows-sys", &["0.59"])];
const X11_CRATE_VERSIONS: &[(&str, &[&str])] = &[("x11", &["2"]), ("x11-dl", &["2"])];

fn integrate(
    ctx: &mut EmitContext,
    crates: &[(&str, &[&str])],
    f: impl Fn(&mut EmitContext, &str) -> EmitResult,
    f_else: impl FnOnce(&mut EmitContext) -> EmitResult,
) -> EmitResult {
    let mut not_cfg = Cfg::none();
    for (krate, ver_it) in crates {
        for ver in ver_it.iter() {
            let (vmaj, vmin) = ver.split_once('.').unwrap_or((ver, ""));
            let mut feature = format!("use-{krate}-v{vmaj}");
            let mut krate = format!("{krate}_v{vmaj}").replace('-', "_");
            if !vmin.is_empty() {
                feature = format!("{feature}-{vmin}");
                krate = format!("{krate}_{vmin}");
            }
            let feature = Cfg::one(feature);
            write!(ctx, "apply_cfg!(")?;
            ctx.emit_feature_cfg(&not_cfg.clone().not().all(feature.clone()))?;
            writeln!(ctx, " => {{")?;
            ctx.increase_indent();
            f(ctx, &krate)?;
            ctx.decrease_indent();
            writeln!(ctx, "}});")?;
            writeln!(ctx)?;
            not_cfg = not_cfg.any(feature);
        }
    }
    write!(ctx, "apply_cfg!(")?;
    ctx.emit_feature_cfg(&not_cfg.not())?;
    writeln!(ctx, " => {{")?;
    ctx.increase_indent();
    f_else(ctx)?;
    ctx.decrease_indent();
    writeln!(ctx, "}});")?;
    writeln!(ctx)?;
    Ok(())
}

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
        module: Some("atomic"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_MemoryBarrierAcquireFunction" | "SDL_MemoryBarrierReleaseFunction"
            )
        },
        patch: |ctx, f| {
            let ident = f.ident.as_str().strip_suffix("Function").unwrap();
            let ordering = ident.strip_prefix("SDL_MemoryBarrier").unwrap();
            writeln!(
                ctx,
                str_block! {r#"
                    #[inline(always)]
                    pub fn {}() {{
                        ::core::sync::atomic::fence(::core::sync::atomic::Ordering::{})
                    }}

                "#},
                ident, ordering
            )?;
            Ok(false)
        },
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
                "main"
                    | "SDL_arraysize"
                    | "SDL_BeginThreadFunction"
                    | "SDL_COMPILE_TIME_ASSERT"
                    | "SDL_const_cast"
                    | "SDL_EndThreadFunction"
                    | "SDL_FILE"
                    | "SDL_FUNCTION"
                    | "SDL_HAS_BUILTIN"
                    | "SDL_IN_BYTECAP"
                    | "SDL_INOUT_Z_CAP"
                    | "SDL_LINE"
                    | "SDL_MAIN_USE_CALLBACKS"
                    | "SDL_memcpy"
                    | "SDL_memmove"
                    | "SDL_MemoryBarrierAcquire" // emitted in function patch for SDL_MemoryBarrier*Function
                    | "SDL_MemoryBarrierRelease" // emitted in function patch for SDL_MemoryBarrier*Function
                    | "SDL_memset"
                    | "SDL_NULL_WHILE_LOOP_CONDITION"
                    | "SDL_OUT_BYTECAP"
                    | "SDL_OUT_CAP"
                    | "SDL_OUT_Z_BYTECAP"
                    | "SDL_OUT_Z_CAP"
                    | "SDL_PRILLd"
                    | "SDL_PRILLu"
                    | "SDL_PRILLx"
                    | "SDL_PRILLX"
                    | "SDL_PRINTF_FORMAT_STRING"
                    | "SDL_PRINTF_VARARG_FUNC"
                    | "SDL_PRINTF_VARARG_FUNCV"
                    | "SDL_reinterpret_cast"
                    | "SDL_SCANF_FORMAT_STRING"
                    | "SDL_SCANF_VARARG_FUNC"
                    | "SDL_SCANF_VARARG_FUNCV"
                    | "SDL_SINT64_C"
                    | "SDL_stack_alloc"
                    | "SDL_stack_free"
                    | "SDL_static_cast"
                    | "SDL_STRINGIFY_ARG"
                    | "SDL_UINT64_C"
                    | "SDL_zero"
                    | "SDL_zeroa"
                    | "VK_DEFINE_HANDLE"
                    | "VK_DEFINE_NON_DISPATCHABLE_HANDLE"
            )
        },
        patch: |_, _| Ok(true),
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_assert" | "SDL_assert_always" | "SDL_assert_paranoid" | "SDL_assert_release"
            )
        },
        patch: |ctx, define| {
            let ident = define.ident.as_str();
            let func = if let DefineValue::Expr(Expr::FnCall(call)) = &define.value {
                let Expr::Ident(func) = &*call.func else {
                    unreachable!()
                };
                func.as_str()
            } else {
                // doc example
                "SDL_disabled_assert"
            };

            define.doc.emit(ctx)?;
            writeln!(
                ctx,
                str_block! {r#"
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! {} {{
                        ($condition:expr) => {{ $crate::assert::{}!($condition) }};
                    }}
                    #[doc(inline)]
                    pub use {};
                "#},
                ident, func, ident
            )?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| i == "SDL_ASSERT_LEVEL",
        patch: |ctx, define| {
            if let Ok(Some(_)) = define.value.try_eval(ctx) {
                // skip non-doc
                // (actual SDL_ASSERT_LEVEL is emitted in the patch for SDL_disabled_assert)
            } else {
                // doc
                let mut define = define.clone();
                define.value = DefineValue::one();
                define.emit(ctx)?;
            }
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| i == "SDL_AssertBreakpoint",
        patch: |ctx, define| {
            ctx.register_sym(
                Ident::new_inline("SDL_AssertBreakpoint"),
                None,
                Some(Type::function(Vec::new(), Type::void(), true, true)),
                None,
                SymKind::Other,
                false,
                false,
            )?;
            define.doc.emit(ctx)?;
            ctx.write_str(str_block! {r#"
                #[inline(always)]
                pub unsafe fn SDL_AssertBreakpoint() {
                    unsafe { SDL_TriggerBreakpoint() }
                }

            "#})?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| i == "SDL_TriggerBreakpoint",
        patch: |ctx, define| {
            ctx.register_sym(
                Ident::new_inline("SDL_TriggerBreakpoint"),
                None,
                Some(Type::function(Vec::new(), Type::void(), true, true)),
                None,
                SymKind::Other,
                false,
                false,
            )?;
            define.doc.emit(ctx)?;
            ctx.write_str(str_block! {r#"
                #[inline(always)]
                pub unsafe fn SDL_TriggerBreakpoint() {
                    crate::breakpoint()
                }

            "#})?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| matches!(i, "SDL_disabled_assert"),
        patch: |ctx, define| {
            // emit SDL_ASSERT_LEVEL here to avoid extra target dependent config
            // (don't register the sym)
            ctx.write_str(str_block! {r#"
                #[cfg(all(not(doc), feature = "assert-level-disabled"))]
                pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 0;
                #[cfg(all(not(any(doc, feature = "assert-level-disabled")), feature = "assert-level-release"))]
                pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 1;
                #[cfg(all(not(any(doc, feature = "assert-level-disabled", feature = "assert-level-release")), feature = "assert-level-debug"))]
                pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 2;
                #[cfg(all(not(any(doc, feature = "assert-level-disabled", feature = "assert-level-release", feature = "assert-level-debug")), feature = "assert-level-paranoid"))]
                pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 3;

            "#})?;

            // emit SDL_disabled_assert
            define.doc.emit(ctx)?;
            ctx.write_str(str_block! {r#"
                #[doc(hidden)]
                #[macro_export]
                macro_rules! SDL_disabled_assert {
                    ($condition:expr) => {{
                        if false {
                            let _ = $condition;
                        }
                    }};
                }
                #[doc(inline)]
                pub use SDL_disabled_assert;
                
            "#})?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("assert"),
        match_ident: |i| matches!(i, "SDL_enabled_assert"),
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            ctx.write_str(str_block! {r#"
                #[doc(hidden)]
                #[macro_export]
                macro_rules! SDL_enabled_assert {
                    ($condition:expr) => {{
                        while !$condition {
                            // Yes, this is wildly unsafe, but it's fine! :thisisfine:
                            // - SDL uses a mutex to protect access to SDL_AssertData
                            // - The static mut can only be accessed through the pointer that's passed to SDL
                            let assert_data = {
                                $crate::__const_c_str!(CONDITION = ::core::stringify!($condition));
                                static mut SDL_ASSERT_DATA: $crate::assert::SDL_AssertData = $crate::assert::SDL_AssertData {
                                    always_ignore: false,
                                    trigger_count: 0,
                                    condition: CONDITION.as_ptr(),
                                    filename: ::core::ptr::null(),
                                    linenum: 0,
                                    function: ::core::ptr::null(),
                                    next: ::core::ptr::null(),
                                };
                                ::core::ptr::addr_of_mut!(SDL_ASSERT_DATA)
                            };
                            const LOCATION: &::core::panic::Location = ::core::panic::Location::caller();
                            $crate::__const_c_str!(FILENAME = LOCATION.file());
                            match unsafe {
                                $crate::assert::SDL_ReportAssertion(
                                    assert_data,
                                    b"???\0".as_ptr().cast::<::core::ffi::c_char>(),
                                    FILENAME.as_ptr(),
                                    LOCATION.line() as ::core::ffi::c_int,
                                )
                            } {
                                $crate::assert::SDL_ASSERTION_RETRY => continue,
                                $crate::assert::SDL_ASSERTION_BREAK => unsafe { $crate::assert::SDL_AssertBreakpoint() },
                                _ => (),
                            }
                            break
                        }
                    }};
                }
                #[doc(inline)]
                pub use SDL_enabled_assert;

            "#})?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("atomic"),
        match_ident: |i| i == "SDL_CompilerBarrier",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            ctx.write_str(str_block! {r#"
                #[inline(always)]
                pub fn SDL_CompilerBarrier() {
                    ::core::sync::atomic::compiler_fence(::core::sync::atomic::Ordering::SeqCst)
                }

            "#})?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("atomic"),
        match_ident: |i| i == "SDL_CPUPauseInstruction",
        patch: |ctx, define| {
            define.doc.emit(ctx)?;
            // TODO: add more archs
            ctx.write_str(str_block! {r#"
                #[inline(always)]
                pub fn SDL_CPUPauseInstruction() {
                    #[cfg(all(feature = "nightly", any(target_arch = "aarch64", target_arch = "arm64ec")))]
                    unsafe { ::core::arch::aarch64::__yield() }
                    #[cfg(all(feature = "nightly", target_arch = "arm"))]
                    unsafe { ::core::arch::arm::__yield() }
                    #[cfg(target_arch = "x86")]
                    unsafe { ::core::arch::x86::_mm_pause() }
                    #[cfg(target_arch = "x86_64")]
                    unsafe { ::core::arch::x86_64::_mm_pause() }
                }

            "#})?;
            Ok(true)
        },
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
            writeln!(ctx, "/// ### Safety (`sdl3-sys`)")?;
            writeln!(
                ctx,
                "/// - `iface` must point to memory that is valid for writing the type `T`."
            )?;
            writeln!(ctx, "/// - The type `T` must be a `repr(C)` struct.")?;
            writeln!(
                ctx,
                "/// - The first field of the struct must be of type `u32`. It will be set to"
            )?;
            writeln!(ctx, "///   the size of the struct in bytes.")?;
            writeln!(
                ctx,
                "/// - The rest of the struct will be initialized as all zero bytes."
            )?;

            writeln!(ctx, "#[inline(always)]")?;
            writeln!(ctx, "pub unsafe fn SDL_INIT_INTERFACE<T>(iface: *mut T) {{")?;
            ctx.increase_indent();
            writeln!(ctx, "const {{ ::core::assert!(::core::mem::size_of::<T>() <= ::core::primitive::u32::MAX as usize) }};")?;
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
        match_ident: |i| i.starts_with("SDL_PRI") && i.ends_with("64"),
        patch: |ctx, define| {
            writeln!(
                ctx,
                r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
            )?;
            define.emit(ctx)?;
            Ok(true)
        },
    },
    EmitDefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_PRILL_PREFIX",
        patch: |ctx, define| {
            writeln!(
                ctx,
                r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
            )?;
            define.emit(ctx)?;
            let Some(Value::String(StringLiteral { str, .. })) = define.value.try_eval(ctx)? else {
                unreachable!()
            };
            let mut bytes = str.into_bytes_with_nul();
            let edit = bytes.len() - 1;
            bytes.push(0);
            for ch in [b'd', b'u', b'x', b'X'] {
                bytes[edit] = ch;
                writeln!(
                    ctx,
                    r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                )?;
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
    let ty = Type::ident_str("SDL_FunctionPointer");

    ctx.register_sym(
        btf,
        None,
        Some(ty.clone()),
        None,
        SymKind::Other,
        false,
        true,
    )?;
    ctx.register_sym(etf, None, Some(ty), None, SymKind::Other, false, true)?;

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
                unreachable!()
            };
            let name = arg.as_str().strip_prefix("Vk").unwrap();
            let doc = format!("(`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::{name}` from the `ash` crate. Otherwise it's a pointer to an opaque struct.");
            integrate(
                ctx,
                VULKAN_CRATE_VERSIONS,
                |ctx, krate| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(ctx, "pub type {arg} = ::{krate}::vk::{name};")?;
                    Ok(())
                },
                |ctx| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(ctx, "pub type {arg} = *mut __{arg};")?;
                    writeln!(ctx)?;
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "#[doc(hidden)]")?;
                    writeln!(ctx, "#[repr(C)]")?;
                    writeln!(
                        ctx,
                        "pub struct __{arg} {{ _opaque: [::core::primitive::u8; 0] }}",
                    )?;
                    Ok(())
                },
            )?;
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
            let name = arg.as_str().strip_prefix("Vk").unwrap();
            let doc = format!("(`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::{name}` from the `ash` crate. Otherwise it's a target dependent opaque type.");
            integrate(
                ctx,
                VULKAN_CRATE_VERSIONS,
                |ctx, krate| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(ctx, "pub type {arg} = ::{krate}::vk::{name};")?;
                    Ok(())
                },
                |ctx| {
                    writeln!(ctx, r#"#[cfg(target_pointer_width = "64")]"#)?;
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "pub type {arg} = *mut __{arg};")?;
                    writeln!(ctx)?;
                    writeln!(ctx, r#"#[cfg(target_pointer_width = "64")]"#)?;
                    writeln!(ctx, "#[doc(hidden)]")?;
                    writeln!(ctx, "#[repr(C)]")?;
                    writeln!(
                        ctx,
                        "pub struct __{arg} {{ _opaque: [::core::primitive::u8; 0] }}",
                    )?;
                    writeln!(ctx)?;
                    writeln!(ctx, r#"#[cfg(not(target_pointer_width = "64"))]"#)?;
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(ctx, "pub type {arg} = ::core::primitive::u64;")?;
                    Ok(())
                },
            )?;
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
                Ok(Some(Value::TargetDependent(DefineState::one(define))))
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

type EmitOpaqueStructPatch = EmitPatch<StructSym>;

const EMIT_OPAQUE_STRUCT_PATCHES: &[EmitOpaqueStructPatch] = &[EmitOpaqueStructPatch {
    module: Some("vulkan"),
    match_ident: |i| i == "VkAllocationCallbacks",
    patch: |ctx, s| {
        let name = s.ident.as_str().strip_prefix("Vk").unwrap();
        let doc = format!("(`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::{name}::<'static>` from the `ash` crate. Otherwise it's an opaque type. {}",
            "<div class=\"warning\">The `'static` lifetime is too long. `ash` requires a lifetime for this, but as it's a C ffi type there's no way for `sdl3-sys` to set the correct lifetime.</div>");
        integrate(
            ctx,
            VULKAN_CRATE_VERSIONS,
            |ctx, krate| {
                writeln!(
                    ctx,
                    r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                )?;
                writeln!(ctx, "/// {doc}")?;
                writeln!(ctx, "pub type Vk{name} = ::{krate}::vk::{name}::<'static>;")?;
                Ok(())
            },
            |ctx| {
                writeln!(
                    ctx,
                    r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                )?;
                writeln!(ctx, "/// {doc}")?;
                writeln!(ctx, "#[repr(C)]")?;
                writeln!(
                    ctx,
                    "pub struct Vk{name} {{ _opaque: [::core::primitive::u8; 0] }}"
                )?;
                Ok(())
            },
        )?;
        Ok(true)
    },
}];

pub fn patch_emit_opaque_struct(
    ctx: &mut EmitContext,
    ident: &str,
    s: &StructSym,
) -> Result<bool, EmitErr> {
    patch_emit(ctx, s, ident, EMIT_OPAQUE_STRUCT_PATCHES)
}

type EmitTypeDefPatch = EmitPatch<TypeDef>;

const EMIT_TYPEDEF_PATCHES: &[EmitTypeDefPatch] = &[
    EmitTypeDefPatch {
        module: Some("gamepad"),
        match_ident: |i| i == "SDL_GamepadBinding",
        patch: |ctx, _td| {
            ctx.write_str(str_block! {r#"
                #[cfg(feature = "debug-impls")]
                impl ::core::fmt::Debug for SDL_GamepadBinding {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let mut s = f.debug_struct("SDL_GamepadBinding");
                        s.field("input_type", &self.input_type);
                        match self.input_type {
                            SDL_GamepadBindingType::BUTTON => { s.field("input.button", unsafe { &self.input.button }); },
                            SDL_GamepadBindingType::AXIS => { s.field("input.axis", unsafe { &self.input.axis }); },
                            SDL_GamepadBindingType::HAT => { s.field("input.hat", unsafe { &self.input.hat }); },
                            _ => (),
                        }
                        s.field("output_type", &self.output_type);
                        match self.output_type {
                            SDL_GamepadBindingType::BUTTON => { s.field("output.button", unsafe { &self.output.button }); },
                            SDL_GamepadBindingType::AXIS => { s.field("output.axis", unsafe { &self.output.axis }); },
                            _ => (),
                        }
                        s.finish()
                    }
                }

            "#})?;
            Ok(false)
        },
    },
    EmitTypeDefPatch {
        module: Some("system"),
        match_ident: |i| i == "MSG",
        patch: |ctx, td| {
            let doc = "(`sdl3-sys`) Enable a `use-windows-sys-*` feature to alias this to `MSG` from the `windows-sys` crate. Otherwise it's an opaque struct.";
            integrate(
                ctx,
                WINDOWS_CRATE_VERSIONS,
                |ctx, krate| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(windows)))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(
                        ctx,
                        "pub type MSG = ::{krate}::Win32::UI::WindowsAndMessaging::MSG;"
                    )?;
                    Ok(())
                },
                |ctx| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(windows)))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    td.emit(ctx)?;
                    Ok(())
                },
            )?;
            Ok(true)
        },
    },
    EmitTypeDefPatch {
        module: Some("system"),
        match_ident: |i| i == "XEvent",
        patch: |ctx, td| {
            let doc = "(`sdl3-sys`) Enable either a `use-x11-*` or a `use-x11-dl-*` feature to alias this to `XEvent` from the `x11` or `x11-dl` crates, respectively. Otherwise it's an opaque struct.";
            integrate(
                ctx,
                X11_CRATE_VERSIONS,
                |ctx, krate| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    writeln!(ctx, "pub type XEvent = ::{krate}::xlib::XEvent;")?;
                    Ok(())
                },
                |ctx| {
                    writeln!(
                        ctx,
                        r#"#[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]"#
                    )?;
                    writeln!(ctx, "/// {doc}")?;
                    td.emit(ctx)?;
                    Ok(())
                },
            )?;
            Ok(true)
        },
    },
];

pub fn patch_emit_type_def(
    ctx: &mut EmitContext,
    ident: &str,
    typedef: &TypeDef,
) -> Result<bool, EmitErr> {
    patch_emit(ctx, typedef, ident, EMIT_TYPEDEF_PATCHES)
}

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
