use super::{patch::patch_emit_opaque_struct, Emit, EmitErr, EmitResult, Eval, Value};
use crate::{
    parse::{
        CanCopy, DefineArg, DefineValue, DocComment, Expr, GetSpan, Ident, IdentOrKw, ParseErr,
        PrimitiveType, RustCode, Span, StructFields, StructKind, Type, TypeEnum,
    },
    Defer, Gen,
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

pub type DefineState = Cfg<Ident>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cfg<T> {
    state: Option<Coll<T>>,
}

impl<T: Clone + Ord> Cfg<T> {
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

    pub fn one(item: T) -> Self {
        Self {
            state: Some(Coll::One(item)),
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

pub struct InnerEmitContext {
    module: String,
    pub preproc_state: Rc<RefCell<PreProcState>>,
    pub scope: Scope,
    preproc_eval_mode: usize,
    emitted_file_doc: bool,
    patch_enabled: bool,
    log_debug_enabled: bool,
    sym_dependencies: Option<Vec<Ident>>,
    pending_emits: Vec<(Vec<Ident>, Option<Box<dyn Emit>>)>,
    pending_enabled: bool,
    function_return_type: Type,
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
            "__ARM_ARCH_7A__" = CfgExpr(r#"target_feature = "armv7-a""#);
            "__ARM_ARCH_7EM__" = CfgExpr(r#"target feature = "armv7e-m""#);
            "__ARM_ARCH_7R__" = CfgExpr(r#"target_feature = "armv7-r""#);
            "__ARM_ARCH_7M__" = CfgExpr(r#"target_feature = "armv7-m""#);
            "__ARM_ARCH_7S__" = CfgExpr(r#"target_feature = "armv7s""#);
            "__ARM_ARCH_8A__" = CfgExpr(r#"target_feature = "armv8-a""#);
            "__clang__" = CfgExpr(always_false!("__clang__"));
            "__EMSCRIPTEN__" = CfgExpr(r#"target_os = "emscripten""#);
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
            "SDL_PLATFORM_3DS" = CfgExpr(r#"any(doc, target_os = "horizon")"#);
            "SDL_PLATFORM_ANDROID" = CfgExpr(r#"any(doc, target_os = "android")"#);
            "SDL_PLATFORM_APPLE" = CfgExpr(r#"any(doc, target_vendor = "apple")"#);
            "SDL_PLATFORM_EMSCRIPTEN" = CfgExpr(r#"any(doc, target_os = "emscripten")"#);
            "SDL_PLATFORM_GDK" = CfgExpr(always_false!("SDL_PLATFORM_GDK")); // change WIN32 if this is changed
            "SDL_PLATFORM_IOS" = CfgExpr(r#"any(doc, target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "watchos")"#);
            "SDL_PLATFORM_LINUX" = CfgExpr(r#"any(doc, target_os = "linux")"#);
            "SDL_PLATFORM_NGAGE" = CfgExpr(always_false!("SDL_PLATFORM_NGAGE"));
            "SDL_PLATFORM_PS2" = CfgExpr(always_false!("SDL_PLATFORM_PS2"));
            "SDL_PLATFORM_PSP" = CfgExpr(r#"any(doc, target_os = "psp")"#);
            "SDL_PLATFORM_TVOS" = CfgExpr(r#"any(doc, target_os = "tvos")"#);
            "SDL_PLATFORM_VITA" = CfgExpr(r#"any(doc, target_os = "vita")"#);
            "SDL_PLATFORM_WIN32" = CfgExpr("any(doc, windows)");
            "SDL_PLATFORM_WINDOWS" = CfgExpr("any(doc, windows)");
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
            "PRId64",
            "PRIs64",
            "PRIu32",
            "PRIu64",
            "PRIx32",
            "PRIX32",
            "PRIx64",
            "PRIX64",
            "DOXYGEN_SHOULD_IGNORE_THIS",
            "NO_SDL_VULKAN_TYPEDEFS",
            format!("SDL_{module}_h_"), "SDL_locale_h", "SDL_main_impl_h_",
            "SDL_ASSERT_LEVEL",
            "SDL_AssertBreakpoint",
            "SDL_AtomicDecRef",
            "SDL_AtomicIncRef",
            "SDL_BeginThreadFunction",
            "SDL_COMPILE_TIME_ASSERT",
            "SDL_DEFAULT_ASSERT_LEVEL", // !!! FIXME
            "SDL_DEFINE_STDBOOL",
            "SDL_EndThreadFunction",
            "SDL_FUNCTION_POINTER_IS_VOID_POINTER",
            "SDL_INTERNAL",
            "SDL_MAIN_AVAILABLE",
            "SDL_MAIN_EXPORTED",
            "SDL_MAIN_NEEDED",
            "SDL_MAIN_NOIMPL",
            "SDL_memcpy",
            "SDL_memmove",
            "SDL_memset",
            "SDL_PI_D",
            "SDL_PI_F",
            "SDL_PRILLd",
            "SDL_PRILLu",
            "SDL_PRILLx",
            "SDL_PRILLX",
            "SDL_PRIs32",
            "SDL_PRIs64",
            "SDL_PRIu32",
            "SDL_PRIu64",
            "SDL_PRIx32",
            "SDL_PRIX32",
            "SDL_PRIx64",
            "SDL_PRIX64",
            "SDL_SINT64_C",
            "SDL_SLOW_MEMCPY",
            "SDL_SLOW_MEMMOVE",
            "SDL_SLOW_MEMSET",
            "SDL_THREAD_SAFETY_ANALYSIS",
            "SDL_UINT64_C",
            "SDL_VENDOR_INFO",
            "SDLMAIN_DECLSPEC",
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
        let rust = |value: &str, ty, is_const, is_unsafe| {
            DefineValue::Expr(Expr::Value(Value::RustCode(RustCode::boxed(
                value.into(),
                ty,
                is_const,
                is_unsafe,
            ))))
        };
        defines! {
            "__GNUC__" = DefineValue::parse_expr("4")?; // not currently used
            "__STDC_VERSION__" = DefineValue::parse_expr("202311L")?;
            "_MSC_VER" = DefineValue::parse_expr("1700")?;
            "FLT_EPSILON" = rust("::core::primitive::f32::EPSILON", Type::primitive(PrimitiveType::Float), true, false);
            "INT64_C"("x") = rust("{x}_i64", Type::primitive(PrimitiveType::Int64T), true, false);
            "UINT64_C"("x") = rust("{x}_u64", Type::primitive(PrimitiveType::Uint64T), true, false);
            "SIZE_MAX" = rust("::core::primitive::usize::MAX", Type::primitive(PrimitiveType::SizeT), true, false);
            "UNICODE" = DefineValue::one();
            "__has_builtin"("builtin") = DefineValue::Other(Span::new_inline("__has_builtin"));
            "SDL_BIG_ENDIAN" = DefineValue::parse_expr("4321")?;
            "SDL_DISABLE_ALLOCA" = DefineValue::one();
            "SDL_DISABLE_ANALYZE_MACROS" = DefineValue::one();
            "SDL_LIL_ENDIAN" = DefineValue::parse_expr("1234")?;
            "SDL_MAIN_HANDLED" = DefineValue::one();
            "SDL_MAIN_USE_CALLBACKS" = DefineValue::one();
            "SDL_NULL_WHILE_LOOP_CONDITION" = DefineValue::parse_expr("0")?;
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
                sym_dependencies: None,
                pending_emits: Vec::new(),
                pending_enabled: true,
                function_return_type: Type::void(),
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
            Some(Value::TargetDependent(DefineState::one(Ident::new_inline(
                define,
            ))))
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

    #[must_use]
    pub fn set_debug_log_guard(&self, enable: bool) -> impl Drop {
        let inner = Rc::clone(&self.inner);
        let was_enabled = self.inner().log_debug_enabled;
        self.inner_mut().log_debug_enabled = enable;
        Defer::new(move || inner.borrow_mut().log_debug_enabled = was_enabled)
    }

    pub fn into_inner(self) -> InnerEmitContext {
        let inner = Rc::clone(&self.inner);
        drop(self);
        Rc::into_inner(inner).unwrap().into_inner()
    }

    fn inner(&self) -> Ref<InnerEmitContext> {
        self.inner.borrow()
    }

    fn inner_mut(&self) -> RefMut<InnerEmitContext> {
        self.inner.borrow_mut()
    }

    fn inner_map<T: ?Sized>(&self, map: impl FnOnce(&InnerEmitContext) -> &T) -> Ref<T> {
        Ref::map(self.inner(), map)
    }

    fn inner_mut_map<T: ?Sized>(
        &self,
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

    #[must_use]
    pub fn with_target_dependent_preproc_state_guard(
        &mut self,
    ) -> (Rc<RefCell<PreProcState>>, impl Drop) {
        let parent = Rc::clone(&self.inner().preproc_state);
        let pps = Rc::new(RefCell::new(PreProcState::with_parent(Rc::clone(&parent))));
        self.inner_mut().preproc_state = Rc::clone(&pps);
        let inner = Rc::clone(&self.inner);
        (
            pps,
            Defer::new(move || inner.borrow_mut().preproc_state = Rc::clone(&parent)),
        )
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

    pub fn scope_mut(&self) -> RefMut<Scope> {
        self.inner_mut_map(|ctx| &mut ctx.scope)
    }

    #[must_use]
    pub fn subscope_guard(&self) -> impl Drop {
        self.scope_mut().push();
        let inner = Rc::clone(&self.inner);
        Defer::new(move || inner.borrow_mut().scope.pop())
    }

    pub fn new_top_level(&mut self) -> EmitContext<'_, 'b> {
        let i = self.inner();
        let inner = Rc::new(RefCell::new(InnerEmitContext {
            module: i.module.clone(),
            preproc_state: Rc::clone(&i.preproc_state),
            scope: i.scope.push_new(),
            preproc_eval_mode: i.preproc_eval_mode,
            emitted_file_doc: i.emitted_file_doc,
            patch_enabled: i.patch_enabled,
            log_debug_enabled: i.log_debug_enabled,
            sym_dependencies: None,
            pending_emits: Vec::new(),
            pending_enabled: true,
            function_return_type: Type::void(),
        }));
        drop(i);
        EmitContext {
            inner,
            output: self.output,
            indent: self.indent,
            newline_count: self.newline_count,
            do_indent: self.do_indent,
            ool_output: String::new(),
            gen: self.gen,
            top: true,
        }
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

    #[must_use]
    pub fn preproc_eval_mode_guard(&mut self) -> impl Drop {
        self.inner_mut().preproc_eval_mode += 1;
        let inner = Rc::clone(&self.inner);
        Defer::new(move || inner.borrow_mut().preproc_eval_mode -= 1)
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
        alias_ty: Option<Type>,
        value_ty: Option<Type>,
        enum_base_ty: Option<Type>,
        kind: SymKind,
        can_derive_copy: bool,
        can_derive_debug: bool,
    ) -> EmitResult {
        let module = self.inner().module.clone();
        self.scope_mut().register_sym(Sym {
            module,
            ident,
            alias_ty,
            value_ty,
            enum_base_ty,
            kind,
            can_derive_copy,
            can_derive_debug,
        })?;
        self.emit_pending()?;
        Ok(())
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

    pub fn lookup_struct_or_union_sym(&self, kind: StructKind, key: &Ident) -> Option<StructSym> {
        match kind {
            StructKind::Struct => self.lookup_struct_sym(key),
            StructKind::Union => self.lookup_union_sym(key),
        }
    }

    pub fn lookup_struct_sym(&self, key: &Ident) -> Option<StructSym> {
        if self.lookup_preproc(key).is_some() {
            todo!()
        } else {
            self.scope().lookup_struct(key)
        }
    }

    pub fn lookup_union_sym(&self, key: &Ident) -> Option<StructSym> {
        if self.lookup_preproc(key).is_some() {
            todo!()
        } else {
            self.scope().lookup_union(key)
        }
    }

    pub fn emit_define_state_cfg(&mut self, define_state: &DefineState) -> EmitResult {
        self.emit_cfg(define_state, |ctx, target_define| {
            Ok(ctx.write_str(
                ctx.preproc_state()
                    .borrow()
                    .get_target_define(target_define)
                    .ok_or_else(|| ParseErr::new(target_define.span(), "undefined target define"))?
                    .0,
            )?)
        })
    }

    pub fn emit_feature_cfg(&mut self, cfg: &Cfg<String>) -> EmitResult {
        self.emit_cfg(cfg, |ctx, feature| {
            Ok(write!(ctx, "feature = \"{feature}\"")?)
        })
    }

    pub fn emit_cfg<T>(
        &mut self,
        cfg: &Cfg<T>,
        emit_cfg: impl Fn(&mut EmitContext, &T) -> EmitResult,
    ) -> EmitResult {
        fn emit_cfg_r<T>(
            ctx: &mut EmitContext,
            emit_cfg: &impl Fn(&mut EmitContext, &T) -> EmitResult,
            coll: &Coll<T>,
        ) -> EmitResult {
            match coll {
                Coll::All(c) => {
                    write!(ctx, "all(")?;
                    let mut first = true;
                    for cfg in c.iter() {
                        if !first {
                            write!(ctx, ", ")?;
                        }
                        first = false;
                        emit_cfg_r(ctx, emit_cfg, cfg)?;
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
                        emit_cfg_r(ctx, emit_cfg, cfg)?;
                    }
                    write!(ctx, ")")?;
                    Ok(())
                }

                Coll::Not(c) => {
                    write!(ctx, "not(")?;
                    emit_cfg_r(ctx, emit_cfg, c)?;
                    write!(ctx, ")")?;
                    Ok(())
                }

                Coll::One(c) => emit_cfg(ctx, c),
            }
        }

        if let Some(coll) = &cfg.state {
            write!(self, "#[cfg(")?;
            emit_cfg_r(self, &emit_cfg, coll)?;
            write!(self, ")]")?;
        }
        Ok(())
    }

    pub fn patch_enabled(&self) -> bool {
        self.inner().patch_enabled
    }

    #[must_use]
    pub fn disable_patch_guard(&mut self) -> impl Drop {
        let patch_enabled = mem::replace(&mut self.inner_mut().patch_enabled, false);
        let inner = Rc::clone(&self.inner);
        Defer::new(move || inner.borrow_mut().patch_enabled = patch_enabled)
    }

    #[must_use]
    pub fn expect_unresolved_sym_dependency_guard(&mut self) -> impl Drop {
        if self.inner().sym_dependencies.is_some() {
            panic!("type dependencies already expected by something else")
        }

        self.inner_mut().sym_dependencies = Some(Vec::new());

        let inner = Rc::clone(&self.inner);
        Defer::new(move || inner.borrow_mut().sym_dependencies = None)
    }

    pub fn emit_after_unresolved_sym_dependencies<T: Emit + 'static>(&self, emittable: T) -> bool {
        if self.inner().pending_enabled {
            let deps = self.inner().sym_dependencies.as_ref().unwrap().clone();
            if !deps.is_empty() {
                self.inner_mut()
                    .pending_emits
                    .push((deps, Some(Box::new(emittable))));
                return true;
            }
        }
        false
    }

    pub fn add_unresolved_sym_dependency(&self, ident: Ident) -> EmitResult {
        log_debug!(self, "unresolved dependency `{ident}`");
        if let Some(deps) = &mut self.inner_mut().sym_dependencies {
            deps.push(ident);
            Ok(())
        } else {
            Err(ParseErr::new(ident.span(), "unexpected unresolved type dependency").into())
        }
    }

    pub fn emit_pending(&mut self) -> EmitResult {
        let mut im = self.inner_mut();
        let scope = Rc::clone(&im.scope.0);
        let scope = scope.borrow();
        let mut to_emit = Vec::new();
        im.pending_emits.retain_mut(|(deps, emittable)| {
            deps.retain(|dep| scope.lookup(dep).is_none());
            if deps.is_empty() {
                to_emit.push(emittable.take().unwrap());
                false
            } else {
                true
            }
        });
        drop(scope);
        drop(im);
        let mut ool = self.with_ool_output();
        for e in to_emit {
            e.emit(&mut ool)?;
        }
        Ok(())
    }

    pub fn flush_pending(&mut self) -> EmitResult {
        assert!(self.inner().sym_dependencies.is_none());
        let mut to_emit = Vec::new();
        for (_, emittable) in self.inner_mut().pending_emits.iter_mut() {
            to_emit.push(emittable.take().unwrap());
        }
        self.inner_mut().pending_emits.clear();
        for e in to_emit {
            e.emit(self)?;
        }
        Ok(())
    }

    pub fn function_return_type(&self) -> Type {
        self.inner().function_return_type.clone()
    }

    pub fn set_function_return_type(&self, return_type: Type) {
        self.inner_mut().function_return_type = return_type;
    }
}

impl Drop for EmitContext<'_, '_> {
    fn drop(&mut self) {
        if self.top {
            let scope = Rc::clone(&self.scope_mut().0);
            scope
                .borrow_mut()
                .emit_opaque_structs_and_unions(self)
                .unwrap();
            self.inner_mut().pending_enabled = false;
            self.flush_pending().unwrap();
        }
        self.flush_ool_output().unwrap();
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
            if let DefineValue::Expr(Expr::Literal(new_lit)) = &value {
                if let DefineValue::Expr(Expr::Literal(old_lit)) = &self.lookup(&key)?.unwrap().1 {
                    if new_lit == old_lit {
                        return Ok(());
                    }
                }
            }
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
pub enum SymKind {
    StructAlias(Ident),
    UnionAlias(Ident),
    Other,
}

#[derive(Clone, Debug)]
pub struct Sym {
    pub module: String,
    pub ident: Ident,
    pub alias_ty: Option<Type>,
    pub value_ty: Option<Type>,
    pub enum_base_ty: Option<Type>,
    pub kind: SymKind,
    pub can_derive_copy: bool,
    pub can_derive_debug: bool,
}

impl Sym {
    pub fn field_type(&self, ctx: &EmitContext, name: &str) -> Option<Type> {
        let fields = match &self.kind {
            SymKind::StructAlias(ident) => ctx.lookup_struct_sym(ident)?.fields?,
            SymKind::UnionAlias(ident) => ctx.lookup_union_sym(ident)?.fields?,
            SymKind::Other => return None,
        };
        for field in fields.fields.iter() {
            if field.ident.as_str() == name {
                return Some(field.ty.clone());
            }
        }
        if let Some(alias) = &self.alias_ty {
            if let Some(ty) = &alias.inner_ty() {
                if let TypeEnum::Ident(i) = &ty.ty {
                    if let Some(sym) = ctx.lookup_sym(i) {
                        return sym.field_type(ctx, name);
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct StructSym {
    pub kind: StructKind,
    pub ident: Ident,
    pub doc: Option<DocComment>,
    pub fields: Option<StructFields>,
    pub emit_status: EmitStatus,
    pub hidden: bool,
    pub can_copy: CanCopy,
    pub can_construct: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmitStatus {
    NotEmitted,
    Requested,
    Emitted,
}

#[derive(Clone)]
pub struct Scope(Rc<RefCell<InnerScope>>);

#[derive(Default)]
struct InnerScope {
    parent: Option<Rc<RefCell<InnerScope>>>,
    syms: HashMap<Ident, Sym>,
    enum_syms: HashSet<Ident>,
    struct_syms: BTreeMap<Ident, StructSym>,
    union_syms: BTreeMap<Ident, StructSym>,
}

impl Scope {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerScope::default())))
    }

    pub fn push(&mut self) {
        *self = self.push_new();
    }

    #[must_use]
    pub fn push_new(&self) -> Self {
        Self(Rc::new(RefCell::new(InnerScope {
            parent: Some(Rc::clone(&self.0)),
            ..Default::default()
        })))
    }

    pub fn pop(&mut self) {
        let parent = self.0.borrow().parent.as_ref().map(Rc::clone);
        if let Some(parent) = parent {
            self.0 = parent;
        } else {
            panic!("popped top level scope")
        }
    }

    pub fn include(&mut self, scope: &Scope) -> EmitResult {
        for sym in scope.0.borrow().syms.values() {
            self.register_sym(sym.clone())?;
        }
        for sym in scope.0.borrow().enum_syms.iter() {
            if self.lookup_enum(sym).is_none() {
                self.register_enum_sym(sym.clone())?;
            }
        }
        for (ident, sym) in scope.0.borrow().struct_syms.iter() {
            if self.lookup_struct(ident).is_none() {
                self.0
                    .borrow_mut()
                    .struct_syms
                    .insert(ident.clone(), sym.clone());
            }
        }
        for (ident, sym) in scope.0.borrow().union_syms.iter() {
            if self.lookup_union(ident).is_none() {
                self.0
                    .borrow_mut()
                    .union_syms
                    .insert(ident.clone(), sym.clone());
            }
        }
        Ok(())
    }

    pub fn register_sym(&mut self, sym: Sym) -> EmitResult {
        self.0.borrow_mut().register_sym(sym)
    }

    pub fn register_enum_sym(&mut self, ident: Ident) -> EmitResult {
        let span = ident.span();
        if self.lookup_enum(&ident).is_some() {
            return Err(ParseErr::new(span, "enum symbol already defined in this scope").into());
        }
        self.0.borrow_mut().enum_syms.insert(ident);
        Ok(())
    }

    pub fn register_struct_or_union_sym(&mut self, sym: StructSym) -> Result<StructSym, EmitErr> {
        let (regd, mut syms) = match sym.kind {
            StructKind::Struct => (
                self.lookup_struct(&sym.ident),
                RefMut::map(self.0.borrow_mut(), |s| &mut s.struct_syms),
            ),
            StructKind::Union => (
                self.lookup_union(&sym.ident),
                RefMut::map(self.0.borrow_mut(), |s| &mut s.union_syms),
            ),
        };
        if let Some(mut regd) = regd {
            let mut changed = false;
            if regd.hidden != sym.hidden {
                return Err(ParseErr::new(sym.ident.span(), "inconsistent visibility").into());
            }
            if regd.doc.is_some() {
                if let Some(doc) = sym.doc {
                    return Err(
                        ParseErr::new(doc.span(), "docs already defined for this item").into(),
                    );
                }
            } else if let Some(doc) = sym.doc {
                changed = true;
                regd.doc = Some(doc);
            }
            if sym.can_copy != CanCopy::Default && regd.can_copy != sym.can_copy {
                if regd.can_copy != CanCopy::Default {
                    return Err(ParseErr::new(sym.ident.span(), "conflicting can_copy").into());
                }
                changed = true;
                regd.can_copy = sym.can_copy;
            }
            if sym.can_construct != regd.can_construct && !sym.can_construct {
                changed = true;
                regd.can_construct = sym.can_construct;
            }
            if regd.fields.is_some() {
                if sym.fields.is_some() {
                    return Err(ParseErr::new(
                        sym.ident.span(),
                        "fields already defined for this item",
                    )
                    .into());
                }
            } else if let Some(fields) = sym.fields {
                changed = true;
                regd.fields = Some(fields);
            }
            match (regd.emit_status, sym.emit_status) {
                (EmitStatus::NotEmitted, EmitStatus::Requested)
                | (EmitStatus::NotEmitted, EmitStatus::Emitted)
                | (EmitStatus::Requested, EmitStatus::Emitted) => {
                    changed = true;
                    regd.emit_status = sym.emit_status;
                }
                _ => (),
            }
            if changed {
                syms.insert(sym.ident, regd.clone());
                Ok(regd)
            } else {
                Ok(regd)
            }
        } else {
            syms.insert(sym.ident.clone(), sym.clone());
            Ok(sym)
        }
    }

    pub fn register_struct_sym(&mut self, sym: StructSym) -> Result<StructSym, EmitErr> {
        self.register_struct_or_union_sym(sym)
    }

    pub fn register_union_sym(&mut self, sym: StructSym) -> Result<StructSym, EmitErr> {
        self.register_struct_or_union_sym(sym)
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Sym> {
        self.0.borrow().lookup(ident)
    }

    pub fn lookup_enum(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().lookup_enum(ident)
    }

    pub fn lookup_struct(&self, ident: &Ident) -> Option<StructSym> {
        self.0.borrow().lookup_struct(ident)
    }

    pub fn lookup_union(&self, ident: &Ident) -> Option<StructSym> {
        self.0.borrow().lookup_union(ident)
    }
}

impl InnerScope {
    pub fn register_sym(&mut self, sym: Sym) -> EmitResult {
        let span = sym.ident.span();
        if let Some(s) = self.lookup(&sym.ident) {
            if sym.module != s.module {
                return Err(ParseErr::new(span, "symbol already defined in this scope").into());
            }
        }
        self.syms.insert(sym.ident.clone(), sym.clone());
        Ok(())
    }

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

    pub fn lookup_struct(&self, ident: &Ident) -> Option<StructSym> {
        if let Some(sym) = self.struct_syms.get(ident) {
            Some(sym.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup_struct(ident)
        } else {
            None
        }
    }

    pub fn lookup_union(&self, ident: &Ident) -> Option<StructSym> {
        if let Some(sym) = self.union_syms.get(ident) {
            Some(sym.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup_union(ident)
        } else {
            None
        }
    }

    pub fn emit_opaque_structs_and_unions(&mut self, ctx: &mut EmitContext) -> EmitResult {
        let syms: Vec<_> = self
            .struct_syms
            .values_mut()
            .filter(|s| s.fields.is_none() && s.emit_status != EmitStatus::Emitted)
            .chain(
                self.union_syms
                    .values_mut()
                    .filter(|s| s.fields.is_none() && s.emit_status != EmitStatus::Emitted),
            )
            .map(|s| {
                s.emit_status = EmitStatus::Emitted;
                s.clone()
            })
            .collect();

        for sym in syms {
            if patch_emit_opaque_struct(ctx, sym.ident.as_str(), &sym)? {
                continue;
            }
            if sym.hidden {
                writeln!(ctx, "#[doc(hidden)]")?;
            }
            if let Some(doc) = &sym.doc {
                doc.emit_rust(ctx, "///")?;
            }
            writeln!(ctx, "#[repr(C)]")?;
            writeln!(
                ctx,
                "pub struct {} {{ _opaque: [::core::primitive::u8; 0] }}",
                sym.ident
            )?;
            writeln!(ctx)?;
            self.register_sym(Sym {
                module: ctx.module().to_string(),
                ident: sym.ident.clone(),
                alias_ty: None,
                value_ty: None,
                enum_base_ty: None,
                kind: match sym.kind {
                    StructKind::Struct => SymKind::StructAlias,
                    StructKind::Union => SymKind::UnionAlias,
                }(sym.ident.clone()),
                can_derive_copy: false,
                can_derive_debug: false,
            })?;
        }

        Ok(())
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
