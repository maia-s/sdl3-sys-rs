use super::{EmitErr, EmitResult, Eval, Value};
use crate::{
    parse::{
        DefineArg, DefineValue, DocComment, Expr, GetSpan, Ident, IdentOrKw, ParseErr,
        PrimitiveType, RustCode, Span, StructKind, Type,
    },
    Gen,
};
use core::{fmt::Display, mem};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::{self, Write},
    iter::Extend,
    rc::Rc,
};

#[derive(Clone, Copy, Debug)]
pub struct CfgExpr(&'static str);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefineState {
    state: Option<Coll<Ident>>,
}

impl DefineState {
    pub fn none() -> Self {
        Self { state: None }
    }

    pub fn never() -> Self {
        Self {
            state: Some(Coll::Any(BTreeSet::new())),
        }
    }

    pub fn is_none(&self) -> bool {
        self.state.is_none()
    }

    pub fn defined(ident: Ident) -> Self {
        Self {
            state: Some(Coll::One(ident)),
        }
    }

    #[must_use]
    pub fn all(self, rhs: Self) -> Self {
        let state = if let Some(self_state) = self.state {
            Some(if let Some(rhs_state) = rhs.state {
                self_state.all(rhs_state)
            } else {
                self_state
            })
        } else {
            rhs.state
        };
        Self { state }
    }

    #[must_use]
    pub fn any(self, rhs: Self) -> Self {
        let state = if let Some(self_state) = self.state {
            Some(if let Some(rhs_state) = rhs.state {
                self_state.any(rhs_state)
            } else {
                self_state
            })
        } else {
            rhs.state
        };
        Self { state }
    }

    #[must_use]
    pub fn not(self) -> Self {
        Self {
            state: self.state.map(|coll| coll.not()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Coll<T> {
    All(BTreeSet<Coll<T>>),
    Any(BTreeSet<Coll<T>>),
    Not(Box<Coll<T>>),
    One(T),
}

impl<T: Clone + Ord> Coll<T> {
    #[must_use]
    pub fn all(self, rhs: Self) -> Self {
        if let Coll::All(mut all) = self {
            match rhs {
                Coll::All(rhs) => all.extend(rhs),
                Coll::Any(_) | Coll::Not(_) | Coll::One(_) => {
                    all.insert(rhs);
                }
            }
            Coll::All(all)
        } else if let Coll::All(mut all) = rhs {
            match self {
                Coll::All(_) => unreachable!(),
                Coll::Any(_) | Coll::Not(_) | Coll::One(_) => {
                    all.insert(self);
                }
            }
            Coll::All(all)
        } else {
            let mut all = BTreeSet::new();
            all.insert(self.clone());
            all.insert(rhs);
            Coll::All(all)
        }
    }

    #[must_use]
    pub fn any(self, rhs: Self) -> Self {
        if let Coll::Any(mut any) = self {
            match rhs {
                Coll::Any(rhs) => any.extend(rhs),
                Coll::All(_) | Coll::Not(_) | Coll::One(_) => {
                    any.insert(rhs);
                }
            }
            Coll::Any(any)
        } else if let Coll::Any(mut any) = rhs {
            match self {
                Coll::Any(_) => unreachable!(),
                Coll::All(_) | Coll::Not(_) | Coll::One(_) => {
                    any.insert(self);
                }
            }
            Coll::Any(any)
        } else {
            let mut any = BTreeSet::new();
            any.insert(self.clone());
            any.insert(rhs);
            Coll::Any(any)
        }
    }

    #[must_use]
    pub fn not(self) -> Self {
        if let Coll::Not(not) = self {
            // not not
            *not
        } else {
            Coll::Not(Box::new(self))
        }
    }
}

pub struct EmitContext<'a, 'b> {
    inner: Rc<RefCell<InnerEmitContext>>,
    output: &'a mut dyn Write,
    indent: usize,
    newline_count: usize,
    do_indent: bool,
    ool_output: String,
    pub gen: &'b Gen,
    top: bool,
}

#[derive(Default)]
pub struct InnerEmitContext {
    module: String,
    pub preproc_state: Rc<RefCell<PreProcState>>,
    pub scope: Scope,
    preproc_eval_mode: usize,
    emitted_file_doc: bool,
    patch_enabled: bool,
    log_debug_enabled: bool,
}

impl<'a, 'b> EmitContext<'a, 'b> {
    pub fn new(
        module: impl Into<String>,
        output: &'a mut dyn Write,
        gen: &'b Gen,
    ) -> Result<Self, EmitErr> {
        let module = module.into();
        let mut preproc_state = PreProcState::default();

        macro_rules! always_true {
            ($define:literal) => {
                concat!("all(/* always enabled: ", $define, " */)")
            };
        }

        macro_rules! always_false {
            ($define:literal) => {
                concat!("any(/* always disabled: ", $define, " */)")
            };
        }

        macro_rules! target_defines {
            ($($define:literal = $cfg:expr;)*) => {
                $( preproc_state.register_target_define($define, $cfg); )*
            };
        }
        target_defines! {
            ".sdl3-sys.assert-level-disabled" = CfgExpr(r#"feature = "assert-level-disabled""#);
            ".sdl3-sys.assert-level-release" = CfgExpr(r#"feature = "assert-level-release""#);
            ".sdl3-sys.assert-level-debug" = CfgExpr(r#"feature = "assert-level-debug""#);
            ".sdl3-sys.assert-level-paranoid" = CfgExpr(r#"feature = "assert-level-paranoid""#);
            ".sdl3-sys.big-endian" = CfgExpr(r#"target_endian = "big""#);
            ".sdl3-sys.little-endian" = CfgExpr(r#"target_endian = "little""#);
            "__aarch64__" = CfgExpr(r#"target_arch = "aarch64""#);
            "__arm__" = CfgExpr(r#"target_arch = "arm""#);
            "__ARM_ARCH_7__" = CfgExpr(r#"all(target_arch = "arm", target_feature = "v7")"#);
            "__ARM_ARCH_7A__" = CfgExpr(always_false!("__ARM_ARCH_7A__")); // ?
            "__ARM_ARCH_7EM__" = CfgExpr(always_false!("__ARM_ARCH_7EM__")); // ?
            "__ARM_ARCH_7R__" = CfgExpr(always_false!("__ARM_ARCH_7R__")); // ?
            "__ARM_ARCH_7M__" = CfgExpr(always_false!("__ARM_ARCH_7M__")); // ?
            "__ARM_ARCH_7S__" = CfgExpr(always_false!("__ARM_ARCH_7S__")); // ?
            "__ARM_ARCH_8A__" = CfgExpr(always_false!("__ARM_ARCH_8A__")); // ?
            "__clang__" = CfgExpr(always_false!("__clang__"));
            "__GNUC__" = CfgExpr(always_false!("__GNUC__"));
            "__i386__" = CfgExpr(r#"target_arch = "x86""#);
            "__ia64" = CfgExpr(always_false!("__ia64"));
            "__LP64__" = CfgExpr(r#"all(not(windows), target_pointer_width = "64")"#);
            "__OPTIMIZE__" = CfgExpr("not(debug_assertions)");
            "__powerpc__" = CfgExpr(r#"any(target_arch = "powerpc", target_arch = "powerpc64")"#);
            "__powerpc64__" = CfgExpr(r#"target_arch = "powerpc64""#);
            "__ppc__" = CfgExpr(r#"any(target_arch = "powerpc", target_arch = "powerpc64")"#);
            "__x86_64__" = CfgExpr(r#"target_arch = "x86_64""#);
            "_DEBUG" = CfgExpr("debug_assertions");
            "_M_IA64" = CfgExpr(always_false!("_M_IA64"));
            "_M_X64" = CfgExpr(r#"target_arch = "x86_64""#);
            "_MSC_VER" = CfgExpr(r#"all(windows, target_env = "msvc")"#);
            "_WIN64" = CfgExpr(r#"all(windows, target_pointer_width = "64")"#);
            "ANDROID" = CfgExpr(r#"target_os = "android""#);
            "DEBUG" = CfgExpr("debug_assertions");
            "SDL_BYTEORDER" = CfgExpr(always_true!("byte order")); // this has a non-boolean value
            "SDL_PLATFORM_3DS" = CfgExpr(always_false!("SDL_PLATFORM_3DS"));
            "SDL_PLATFORM_ANDROID" = CfgExpr(r#"target_os = "android""#);
            "SDL_PLATFORM_APPLE" = CfgExpr(r#"target_vendor = "apple""#);
            "SDL_PLATFORM_EMSCRIPTEN" = CfgExpr(r#"target_os = "emscripten""#);
            "SDL_PLATFORM_GDK" = CfgExpr(always_false!("SDL_PLATFORM_GDK")); // change WIN32 if this is changed
            "SDL_PLATFORM_IOS" = CfgExpr(r#"any(target_os = "ios", target_os = "tvos", target_os = "watchos")"#);
            "SDL_PLATFORM_LINUX" = CfgExpr(r#"target_os = "linux""#);
            "SDL_PLATFORM_VITA" = CfgExpr(always_false!("SDL_PLATFORM_VITA"));
            "SDL_PLATFORM_WIN32" = CfgExpr("windows");
            "SDL_PLATFORM_WINDOWS" = CfgExpr("windows");
            "SDL_PLATFORM_WINGDK" = CfgExpr(always_false!("SDL_PLATFORM_WINGDK"));
            "SDL_WIKI_DOCUMENTATION_SECTION" = CfgExpr("doc");
        }

        macro_rules! undefines {
            ($($ident:expr),* $(,)?) => {
                $( preproc_state.undefine(Ident::new_inline($ident)); )*
            };
        }
        undefines! {
            "__ARM_ARCH",
            "__clang_analyzer__",
            "__cplusplus",
            "__SUNPRO_C",
            "__WATCOMC__",
            "assert",
            "PRId32",
            "PRIs64",
            "PRIu32",
            "PRIu64",
            "PRIx32",
            "PRIX32",
            "PRIx64",
            "PRIX64",
            "DOXYGEN_SHOULD_IGNORE_THIS",
            "NO_SDL_VULKAN_TYPEDEFS",
            format!("SDL_{module}_h_"), "SDL_locale_h",
            "SDL_ASSERT_LEVEL",
            "SDL_AssertBreakpoint",
            "SDL_AtomicDecRef",
            "SDL_AtomicIncRef",
            "SDL_BeginThreadFunction",
            "SDL_COMPILE_TIME_ASSERT",
            "SDL_DEFAULT_ASSERT_LEVEL", // !!! FIXME
            "SDL_EndThreadFunction",
            "SDL_FUNCTION_POINTER_IS_VOID_POINTER",
            "SDL_memcpy",
            "SDL_memmove",
            "SDL_memset",
            "SDL_PI_D",
            "SDL_PI_F",
            "SDL_PRIs32",
            "SDL_PRIs64",
            "SDL_PRIu32",
            "SDL_PRIu64",
            "SDL_PRIx32",
            "SDL_PRIX32",
            "SDL_PRIx64",
            "SDL_PRIX64",
            "SDL_SLOW_MEMCPY",
            "SDL_SLOW_MEMMOVE",
            "SDL_SLOW_MEMSET",
            "SDL_THREAD_SAFETY_ANALYSIS",
            "SDL_VENDOR_INFO",
            "VULKAN_H_",
        }

        macro_rules! defines {
            ($($define:literal $(($($args:literal),*))? = $value:expr;)*) => {
                $( defines!(@ Ident::new_inline($define), $(($($args),*),)? $value); )*
            };
            (@ $define:expr, $value:expr) => {
                preproc_state.define($define, None, $value)?;
            };
            (@ $define:expr, ($($args:literal),*), $value:expr) => {
                preproc_state.define($define, Some(vec![$(DefineArg::new(IdentOrKw::new_inline($args), Type::infer()))*]), $value)?;
            };
        }
        defines! {
            "__GNUC__" = DefineValue::parse_expr("4")?; // not currently used
            "__STDC_VERSION__" = DefineValue::parse_expr("202311L")?;
            "_MSC_VER" = DefineValue::parse_expr("1700")?;
            "FLT_EPSILON" = DefineValue::RustCode(RustCode::boxed("::core::primitive::f32::EPSILON".into(), Type::primitive(PrimitiveType::Float), true, false));
            "INT64_C"("x") = DefineValue::RustCode(RustCode::boxed("{x}_i64".into(), Type::primitive(PrimitiveType::Int64T), true, false));
            "UINT64_C"("x") = DefineValue::RustCode(RustCode::boxed("{x}_u64".into(), Type::primitive(PrimitiveType::Uint64T), true, false));
            "SIZE_MAX" = DefineValue::RustCode(RustCode::boxed("::core::primitive::usize::MAX".into(), Type::primitive(PrimitiveType::SizeT), true, false));
            "__has_builtin"("builtin") = DefineValue::Other(Span::new_inline("__has_builtin"));
            "SDL_BIG_ENDIAN" = DefineValue::parse_expr("4321")?;
            "SDL_DISABLE_ALLOCA" = DefineValue::one();
            "SDL_DISABLE_ANALYZE_MACROS" = DefineValue::one();
            "SDL_LIL_ENDIAN" = DefineValue::parse_expr("1234")?;
        }

        Ok(Self {
            inner: Rc::new(RefCell::new(InnerEmitContext {
                module,
                preproc_state: Rc::new(RefCell::new(preproc_state)),
                scope: Scope::new(),
                preproc_eval_mode: 0,
                emitted_file_doc: false,
                patch_enabled: true,
                log_debug_enabled: false,
            })),
            output,
            indent: 0,
            newline_count: 0,
            do_indent: false,
            ool_output: String::new(),
            gen,
            top: true,
        })
    }

    pub fn try_target_dependent_if_compare(
        &self,
        op: &str,
        ident: &str,
        rhs: &Expr,
    ) -> Option<Value> {
        let target_dependent_value = |define| {
            Some(Value::TargetDependent(DefineState::defined(
                Ident::new_inline(define),
            )))
        };
        match op {
            "==" => {
                let Ok(Some(value)) = rhs.try_eval(self) else {
                    return None;
                };
                match ident {
                    "SDL_ASSERT_LEVEL" => match u64::try_from(value) {
                        Ok(0) => target_dependent_value(".sdl3-sys.assert-level-disabled"),
                        Ok(1) => target_dependent_value(".sdl3-sys.assert-level-release"),
                        Ok(2) => target_dependent_value(".sdl3-sys.assert-level-debug"),
                        Ok(3) => target_dependent_value(".sdl3-sys.assert-level-paranoid"),
                        _ => panic!("invalid assert level"),
                    },

                    "SDL_BYTEORDER" => match u64::try_from(value) {
                        Ok(1234) => target_dependent_value(".sdl3-sys.little-endian"),
                        Ok(4321) => target_dependent_value(".sdl3-sys.big-endian"),
                        _ => panic!("invalid byte order"),
                    },

                    _ => todo!(),
                }
            }

            _ => todo!(),
        }
    }

    pub fn log_skipped(&mut self, what: &str, ident: &str) -> EmitResult {
        eprintln!("[sdl3-sys-gen] skipped {what} `{}::{ident}`", self.module());
        writeln!(self, "// [sdl3-sys-gen] skipped {what} `{ident}`")?;
        writeln!(self)?;
        Ok(())
    }

    pub fn log_debug(&self, what: impl Display) -> EmitResult {
        if self.inner().log_debug_enabled {
            eprintln!("[sdl3-sys-gen][debug] {what}");
        }
        Ok(())
    }

    pub fn set_debug_log_guard(&self, enable: bool) -> impl Drop {
        pub struct Guard(Rc<RefCell<InnerEmitContext>>, bool);

        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.borrow_mut().log_debug_enabled = self.1;
            }
        }

        let was_enabled = self.inner().log_debug_enabled;
        self.inner_mut().log_debug_enabled = enable;

        Guard(Rc::clone(&self.inner), was_enabled)
    }

    pub fn into_inner(self) -> InnerEmitContext {
        let inner = Rc::clone(&self.inner);
        drop(self);
        Rc::into_inner(inner).unwrap().into_inner()
    }

    fn inner(&self) -> Ref<InnerEmitContext> {
        self.inner.borrow()
    }

    fn inner_mut(&mut self) -> RefMut<InnerEmitContext> {
        self.inner.borrow_mut()
    }

    fn inner_map<T: ?Sized>(&self, map: impl FnOnce(&InnerEmitContext) -> &T) -> Ref<T> {
        Ref::map(self.inner(), map)
    }

    fn inner_mut_map<T: ?Sized>(
        &mut self,
        map: impl FnOnce(&mut InnerEmitContext) -> &mut T,
    ) -> RefMut<T> {
        RefMut::map(self.inner_mut(), map)
    }

    pub fn module(&self) -> Ref<str> {
        self.inner_map(|ctx| ctx.module.as_str())
    }

    pub fn preproc_state(&self) -> Rc<RefCell<PreProcState>> {
        Rc::clone(&self.inner().preproc_state)
    }

    pub fn increase_indent(&mut self) {
        self.indent += 4;
    }

    pub fn decrease_indent(&mut self) {
        self.indent -= 4;
    }

    pub fn with_target_dependent_preproc_state_guard(
        &mut self,
    ) -> (Rc<RefCell<PreProcState>>, impl Drop) {
        pub struct Guard(Rc<RefCell<InnerEmitContext>>, Rc<RefCell<PreProcState>>);

        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.borrow_mut().preproc_state = Rc::clone(&self.1);
            }
        }

        let parent = Rc::clone(&self.inner().preproc_state);
        let pps = Rc::new(RefCell::new(PreProcState::with_parent(Rc::clone(&parent))));
        self.inner_mut().preproc_state = Rc::clone(&pps);

        (pps, Guard(Rc::clone(&self.inner), parent))
    }

    pub fn merge_target_dependent_preproc_state(&mut self, pps: PreProcState) {
        let parent = self.preproc_state();
        assert_eq!(
            Rc::as_ptr(&parent),
            Rc::as_ptr(pps.parent.as_ref().unwrap())
        );
        let mut parent = parent.borrow_mut();

        for key in pps.undefined.iter() {
            if let Ok(Some((_, DefineValue::TargetDependent))) = parent.lookup(key) {
                continue;
            }
            parent
                .define(key.clone(), None, DefineValue::TargetDependent)
                .unwrap();
        }

        for (key, (args, _)) in pps.defined.iter() {
            if let Ok(Some((_, DefineValue::TargetDependent))) = parent.lookup(key) {
                continue;
            }
            parent
                .define(key.clone(), args.clone(), DefineValue::TargetDependent)
                .unwrap();
        }
    }

    pub fn scope(&self) -> Ref<Scope> {
        self.inner_map(|ctx| &ctx.scope)
    }

    pub fn scope_mut(&mut self) -> RefMut<Scope> {
        self.inner_mut_map(|ctx| &mut ctx.scope)
    }

    pub fn subscope_guard(&mut self) -> impl Drop {
        pub struct Guard(Rc<RefCell<InnerEmitContext>>);

        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.borrow_mut().scope.pop();
            }
        }

        self.scope_mut().push();

        Guard(Rc::clone(&self.inner))
    }

    pub fn capture_output(
        &self,
        f: impl FnOnce(&mut EmitContext) -> EmitResult,
    ) -> Result<String, EmitErr> {
        let mut output = String::new();
        f(&mut { self.with_output(&mut output) })?;
        Ok(output)
    }

    pub fn with_output<'o>(&self, output: &'o mut dyn Write) -> EmitContext<'o, 'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output,
            indent: 0,
            newline_count: 0,
            do_indent: false,
            ool_output: String::new(),
            gen: self.gen,
            top: false,
        }
    }

    pub fn with_ool_output<'o>(&'o mut self) -> EmitContext<'o, 'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output: &mut self.ool_output,
            indent: 0,
            newline_count: 0,
            do_indent: false,
            ool_output: String::new(),
            gen: self.gen,
            top: false,
        }
    }

    pub fn flush_ool_output(&mut self) -> EmitResult {
        self.output.write_str(&self.ool_output)?;
        self.ool_output.clear();
        Ok(())
    }

    pub fn use_ident(&self, ident: &Ident) -> EmitResult {
        if self.lookup_sym(ident).is_some() {
            Ok(())
        } else {
            //Err(ParseErr::new(ident.span(), "undefined symbol").into())
            // FIXME
            Ok(())
        }
    }

    pub fn is_preproc_eval_mode(&self) -> bool {
        self.inner().preproc_eval_mode != 0
    }

    pub fn preproc_eval_mode_guard(&mut self) -> impl Drop {
        pub struct Guard(Rc<RefCell<InnerEmitContext>>);

        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.borrow_mut().preproc_eval_mode -= 1;
            }
        }

        self.inner_mut().preproc_eval_mode += 1;

        Guard(Rc::clone(&self.inner))
    }

    pub fn emitted_file_doc(&self) -> bool {
        self.inner().emitted_file_doc
    }

    pub fn set_emitted_file_doc(&mut self, value: bool) {
        self.inner_mut().emitted_file_doc = value;
    }

    pub fn register_sym(
        &mut self,
        ident: Ident,
        ty: Option<Type>,
        can_derive_debug: bool,
    ) -> EmitResult {
        let module = self.inner().module.clone();
        self.scope_mut().register_sym(Sym {
            module,
            ident,
            ty,
            can_derive_debug,
        })
    }

    pub fn lookup_preproc(&self, key: &Ident) -> Option<(Option<Vec<DefineArg>>, DefineValue)> {
        if let Ok(Some(def)) = self.preproc_state().borrow().lookup(key) {
            Some(def)
        } else {
            None
        }
    }

    pub fn lookup_sym(&self, key: &Ident) -> Option<Sym> {
        self.scope().lookup(key)
    }

    pub fn lookup_enum_sym(&self, key: &Ident) -> Option<Ident> {
        if self.lookup_preproc(key).is_some() {
            todo!()
        } else {
            self.scope().lookup_enum(key)
        }
    }

    pub fn lookup_struct_or_union_sym(
        &self,
        kind: StructKind,
        key: &Ident,
    ) -> Option<(Ident, bool)> {
        match kind {
            StructKind::Struct => self.lookup_struct_sym(key),
            StructKind::Union => self.lookup_union_sym(key),
        }
    }

    pub fn lookup_struct_sym(&self, key: &Ident) -> Option<(Ident, bool)> {
        if self.lookup_preproc(key).is_some() {
            todo!()
        } else {
            self.scope().lookup_struct(key)
        }
    }

    pub fn lookup_union_sym(&self, key: &Ident) -> Option<(Ident, bool)> {
        if self.lookup_preproc(key).is_some() {
            todo!()
        } else {
            self.scope().lookup_union(key)
        }
    }

    pub fn emit_define_state_cfg(&mut self, define_state: &DefineState) -> EmitResult {
        fn emit_cfg_r(ctx: &mut EmitContext, coll: &Coll<Ident>) -> EmitResult {
            match coll {
                Coll::All(c) => {
                    write!(ctx, "all(")?;
                    let mut first = true;
                    for cfg in c.iter() {
                        if !first {
                            write!(ctx, ", ")?;
                        }
                        first = false;
                        emit_cfg_r(ctx, cfg)?;
                    }
                    write!(ctx, ")")?;
                    Ok(())
                }

                Coll::Any(c) => {
                    write!(ctx, "any(")?;
                    let mut first = true;
                    for cfg in c.iter() {
                        if !first {
                            write!(ctx, ", ")?;
                        }
                        first = false;
                        emit_cfg_r(ctx, cfg)?;
                    }
                    write!(ctx, ")")?;
                    Ok(())
                }

                Coll::Not(c) => {
                    write!(ctx, "not(")?;
                    emit_cfg_r(ctx, c)?;
                    write!(ctx, ")")?;
                    Ok(())
                }

                Coll::One(c) => ctx.emit_cfg_from_target_define(c),
            }
        }

        if let Some(coll) = &define_state.state {
            write!(self, "#[cfg(")?;
            emit_cfg_r(self, coll)?;
            writeln!(self, ")]")?;
        }
        Ok(())
    }

    fn emit_cfg_from_target_define(&mut self, target_define: &Ident) -> EmitResult {
        write!(
            self,
            "{}",
            self.preproc_state()
                .borrow()
                .get_target_define(target_define)
                .ok_or_else(|| ParseErr::new(target_define.span(), "undefined target define"))?
                .0
        )?;
        Ok(())
    }

    pub fn patch_enabled(&self) -> bool {
        self.inner().patch_enabled
    }

    pub fn disable_patch_guard(&mut self) -> impl Drop {
        pub struct Guard(Rc<RefCell<InnerEmitContext>>, bool);

        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.borrow_mut().patch_enabled = self.1;
            }
        }

        let patch_enabled = mem::replace(&mut self.inner_mut().patch_enabled, false);

        Guard(Rc::clone(&self.inner), patch_enabled)
    }
}

impl Drop for EmitContext<'_, '_> {
    fn drop(&mut self) {
        self.flush_ool_output().unwrap();
        if self.top {
            self.inner
                .borrow()
                .scope
                .emit_opaque_structs_and_unions(self.output)
                .unwrap();
        }
    }
}

impl Write for EmitContext<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let indent = " ".repeat(self.indent);
        for line in s.split('\n') {
            if line.is_empty() {
                self.newline_count += 1;
            } else {
                for _ in 0..self.newline_count {
                    self.output.write_char('\n')?;
                }
                if self.do_indent || self.newline_count != 0 {
                    self.output.write_str(&indent)?;
                }
                self.output.write_str(line)?;
                self.newline_count = 1;
            }
        }
        self.newline_count -= 1;
        self.do_indent = self.newline_count != 0;
        if self.newline_count != 0 {
            self.newline_count -= 1;
            self.output.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PreProcState {
    parent: Option<Rc<RefCell<PreProcState>>>,
    defined: HashMap<Ident, (Option<Vec<DefineArg>>, DefineValue)>,
    undefined: HashSet<Ident>,
    undefined_prefixes: HashSet<String>,
    target_defines: HashMap<&'static str, CfgExpr>,
}

impl PreProcState {
    pub fn with_parent(parent: Rc<RefCell<PreProcState>>) -> Self {
        Self {
            parent: Some(parent),
            defined: HashMap::new(),
            undefined: HashSet::new(),
            undefined_prefixes: HashSet::new(),
            target_defines: HashMap::new(),
        }
    }

    pub fn include(&mut self, include: &Self) -> EmitResult {
        for key in include.undefined.iter() {
            self.undefine(key.clone());
        }
        for (key, (args, value)) in include.defined.iter() {
            self.undefine(key.clone());
            self.define(key.clone(), args.clone(), value.clone())?;
        }
        Ok(())
    }

    pub fn define(
        &mut self,
        key: Ident,
        args: Option<Vec<DefineArg>>,
        value: DefineValue,
    ) -> EmitResult {
        if let Ok(true) = self.is_defined_ignore_target(&key) {
            return Err(ParseErr::new(key.span, "already defined").into());
        }
        self.undefined.remove(&key);
        self.defined.insert(key, (args, value));
        Ok(())
    }

    pub fn undefine(&mut self, key: Ident) {
        self.defined.remove(&key);
        self.undefined.insert(key);
    }

    pub fn undefine_prefix(&mut self, pfx: &str) {
        self.undefined_prefixes.insert(pfx.to_string());
    }

    pub fn register_target_define(&mut self, key: &'static str, value: CfgExpr) {
        self.target_defines.insert(key, value);
    }

    pub fn lookup(
        &self,
        key: &Ident,
    ) -> Result<Option<(Option<Vec<DefineArg>>, DefineValue)>, EmitErr> {
        if let Some(value) = self.defined.get(key) {
            Ok(Some(value.clone()))
        } else if self.undefined.contains(key) || self.undefined_prefixes_matches(key.as_str()) {
            Ok(None)
        } else if self.target_defines.contains_key(key.as_str()) {
            Ok(Some((None, DefineValue::TargetDependent)))
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup(key)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
    }

    pub fn is_defined(&self, key: &Ident) -> Result<bool, EmitErr> {
        if self.is_target_define(key) {
            Ok(true)
        } else {
            self.is_defined_ignore_target(key)
        }
    }

    pub fn is_defined_ignore_target(&self, key: &Ident) -> Result<bool, EmitErr> {
        if let Some((_, value)) = self.defined.get(key) {
            if matches!(value, DefineValue::TargetDependent) {
                // workaround for mutually exclusive defines not part of the same #if chain
                Ok(false)
            } else {
                Ok(true)
            }
        } else if self.undefined.contains(key) || self.undefined_prefixes_matches(key.as_str()) {
            Ok(false)
        } else if let Some(parent) = &self.parent {
            parent.borrow().is_defined(key)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
    }

    pub fn undefined_prefixes_matches(&self, s: &str) -> bool {
        for pfx in self.undefined_prefixes.iter() {
            if s.starts_with(pfx) {
                return true;
            }
        }
        false
    }

    pub fn is_target_define(&self, key: &Ident) -> bool {
        key.as_str().starts_with("SDL_PLATFORM")
            || self.target_defines.contains_key(key.as_str())
            || self
                .parent
                .as_ref()
                .map(|p| p.borrow().target_defines.contains_key(key.as_str()))
                .unwrap_or(false)
    }

    pub fn get_target_define(&self, key: &Ident) -> Option<CfgExpr> {
        if let Some(cfg) = self.target_defines.get(key.as_str()) {
            Some(*cfg)
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_target_define(key)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sym {
    pub module: String,
    pub ident: Ident,
    pub ty: Option<Type>,
    pub can_derive_debug: bool,
}

#[derive(Clone)]
pub struct Scope(Rc<RefCell<InnerScope>>);

#[derive(Default)]
struct InnerScope {
    parent: Option<Rc<RefCell<InnerScope>>>,
    syms: HashMap<Ident, Sym>,
    enum_syms: HashSet<Ident>,
    struct_syms: BTreeMap<Ident, bool>,
    struct_docs: BTreeMap<Ident, DocComment>,
    union_syms: BTreeMap<Ident, bool>,
    union_docs: BTreeMap<Ident, DocComment>,
}

impl Scope {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerScope::default())))
    }

    pub fn push(&mut self) {
        self.0 = Rc::new(RefCell::new(InnerScope {
            parent: Some(Rc::clone(&self.0)),
            ..Default::default()
        }));
    }

    pub fn pop(&mut self) {
        let parent = self.0.borrow().parent.as_ref().map(Rc::clone);
        if let Some(parent) = parent {
            self.0 = parent;
        } else {
            panic!("popped top level scope")
        }
    }

    pub fn emit_opaque_structs_and_unions(&self, f: &mut dyn Write) -> EmitResult {
        let scope = self.0.borrow();

        macro_rules! emit {
            ($s:expr, $docs:expr) => {
                if let Some(doc) = $docs.get($s) {
                    for line in doc.to_string().lines() {
                        writeln!(f, "///{}{}", if line.is_empty() { "" } else { " " }, line)?;
                    }
                }
                writeln!(f, "#[repr(C)]")?;
                writeln!(f, "#[non_exhaustive]")?;
                writeln!(
                    f,
                    "pub struct {} {{ _opaque: [::core::primitive::u8; 0] }}",
                    $s.as_str()
                )?;
                writeln!(f)?;
            };
        }

        for s in scope
            .struct_syms
            .iter()
            .filter_map(|(s, d)| (!d).then_some(s))
        {
            emit!(s, scope.struct_docs);
        }

        for s in scope
            .union_syms
            .iter()
            .filter_map(|(s, d)| (!d).then_some(s))
        {
            // opaque unions are emitted as opaque structs
            emit!(s, scope.union_docs);
        }

        Ok(())
    }

    pub fn include(&mut self, scope: &Scope) -> EmitResult {
        for sym in scope.0.borrow().syms.values() {
            self.register_sym(sym.clone())?;
        }
        Ok(())
    }

    pub fn register_sym(&mut self, sym: Sym) -> EmitResult {
        let span = sym.ident.span();
        if let Some(s) = self.lookup(&sym.ident) {
            if sym.module != s.module {
                return Err(ParseErr::new(span, "symbol already defined in this scope").into());
            }
        }
        self.0
            .borrow_mut()
            .syms
            .insert(sym.ident.clone(), sym.clone());
        Ok(())
    }

    pub fn register_enum_sym(&mut self, ident: Ident) -> EmitResult {
        let span = ident.span();
        if self.lookup_enum(&ident).is_some() {
            return Err(ParseErr::new(span, "enum symbol already defined in this scope").into());
        }
        self.0.borrow_mut().enum_syms.insert(ident);
        Ok(())
    }

    pub fn register_struct_or_union_sym(
        &mut self,
        kind: StructKind,
        ident: Ident,
        defined: bool,
        doc: Option<DocComment>,
    ) -> EmitResult {
        match kind {
            StructKind::Struct => self.register_struct_sym(ident, defined, doc),
            StructKind::Union => self.register_union_sym(ident, defined, doc),
        }
    }

    pub fn register_struct_sym(
        &mut self,
        ident: Ident,
        defined: bool,
        doc: Option<DocComment>,
    ) -> EmitResult {
        let span = ident.span();
        if let Some(doc) = doc {
            let mut docs = self.0.borrow_mut();
            let docs = &mut docs.struct_docs;
            if docs.insert(ident.clone(), doc).is_some() {
                return Err(ParseErr::new(span, "docs already defined for this struct").into());
            }
        }
        if let Some((_, true)) = self.lookup_struct(&ident) {
            if defined {
                return Err(
                    ParseErr::new(span, "struct symbol already defined in this scope").into(),
                );
            }
        } else {
            self.0.borrow_mut().struct_syms.insert(ident, defined);
        }
        Ok(())
    }

    pub fn register_union_sym(
        &mut self,
        ident: Ident,
        defined: bool,
        doc: Option<DocComment>,
    ) -> EmitResult {
        let span = ident.span();
        if let Some(doc) = doc {
            let mut docs = self.0.borrow_mut();
            let docs = &mut docs.union_docs;
            if docs.insert(ident.clone(), doc).is_some() {
                return Err(ParseErr::new(span, "docs already defined for this union").into());
            }
        }
        if let Some((_, true)) = self.lookup_union(&ident) {
            if defined {
                return Err(
                    ParseErr::new(span, "union symbol already defined in this scope").into(),
                );
            }
        } else {
            self.0.borrow_mut().union_syms.insert(ident, defined);
        }
        Ok(())
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Sym> {
        self.0.borrow().lookup(ident)
    }

    pub fn lookup_enum(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().lookup_enum(ident)
    }

    pub fn lookup_struct(&self, ident: &Ident) -> Option<(Ident, bool)> {
        self.0.borrow().lookup_struct_(ident)
    }

    pub fn lookup_union(&self, ident: &Ident) -> Option<(Ident, bool)> {
        self.0.borrow().lookup_union(ident)
    }
}

impl InnerScope {
    pub fn lookup(&self, ident: &Ident) -> Option<Sym> {
        if let Some(sym) = self.syms.get(ident) {
            Some(sym.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup(ident)
        } else {
            None
        }
    }

    pub fn lookup_enum(&self, ident: &Ident) -> Option<Ident> {
        if let Some(sym) = self.enum_syms.get(ident) {
            Some(sym.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup_enum(ident)
        } else {
            None
        }
    }

    pub fn lookup_struct_(&self, ident: &Ident) -> Option<(Ident, bool)> {
        if let Some(sym) = self
            .struct_syms
            .get_key_value(ident)
            .map(|(i, d)| (i.clone(), *d))
        {
            Some(sym)
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup_struct_(ident)
        } else {
            None
        }
    }

    pub fn lookup_union(&self, ident: &Ident) -> Option<(Ident, bool)> {
        if let Some(sym) = self
            .union_syms
            .get_key_value(ident)
            .map(|(i, d)| (i.clone(), *d))
        {
            Some(sym)
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup_union(ident)
        } else {
            None
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
