use super::{EmitErr, EmitResult};
use crate::{
    parse::{DefineValue, GetSpan, Ident, IdentOrKw, ParseErr, Span},
    Gen,
};
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
}

#[derive(Default)]
pub struct InnerEmitContext {
    module: String,
    pub preproc_state: Rc<RefCell<PreProcState>>,
    scope: Scope,
    preproc_eval_mode: usize,
    emitted_file_doc: bool,
}

impl<'a, 'b> EmitContext<'a, 'b> {
    pub fn new(
        module: impl Into<String>,
        output: &'a mut dyn Write,
        gen: &'b Gen,
    ) -> Result<Self, EmitErr> {
        let module = module.into();
        let mut preproc_state = PreProcState::default();

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
            "__GNUC__" = CfgExpr(always_false!("__GNUC__"));
            "__LP64__" = CfgExpr(r#"all(not(windows), target_pointer_width = "64")"#);
            "__OPTIMIZE__" = CfgExpr("not(debug_assertions)");
            "_DEBUG" = CfgExpr("debug_assertions");
            "_MSC_VER" = CfgExpr(r#"all(windows, target_env = "msvc")"#);
            "ANDROID" = CfgExpr(r#"target_os = "android""#);
            "DEBUG" = CfgExpr("debug_assertions");
            "SDL_PLATFORM_3DS" = CfgExpr(always_false!("SDL_PLATFORM_3DS"));
            "SDL_PLATFORM_ANDROID" = CfgExpr(r#"target_os = "android""#);
            "SDL_PLATFORM_APPLE" = CfgExpr(r#"target_vendor = "apple""#);
            "SDL_PLATFORM_GDK" = CfgExpr(always_false!("SDL_PLATFORM_GDK")); // change WIN32 if this is changed
            "SDL_PLATFORM_VITA" = CfgExpr(always_false!("SDL_PLATFORM_VITA"));
            "SDL_PLATFORM_WIN32" = CfgExpr("windows");
            "SDL_WIKI_DOCUMENTATION_SECTION" = CfgExpr("doc");
        }

        macro_rules! undefines {
            ($($ident:expr),* $(,)?) => {
                $( preproc_state.undefine(Ident::new_inline($ident)); )*
            };
        }
        undefines! {
            "__clang_analyzer__",
            "__cplusplus",
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
            format!("SDL_{module}_h_"),
            "SDL_ASSERT_LEVEL",
            "SDL_AssertBreakpoint",
            "SDL_COMPILE_TIME_ASSERT",
            "SDL_DEFAULT_ASSERT_LEVEL", // !!! FIXME
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
        }

        macro_rules! defines {
            ($($define:literal $(($($args:literal),*))? = $value:expr;)*) => {
                $( defines!(@ Ident::new_inline($define), $(($($args),*),)? $value); )*
            };
            (@ $define:expr, $value:expr) => {
                preproc_state.define($define, None, $value)?;
            };
            (@ $define:expr, ($($args:literal),*), $value:expr) => {
                preproc_state.define($define, Some(vec![$(IdentOrKw::new_inline($args))*]), $value)?;
            };
        }
        defines! {
            "__STDC_VERSION__" = DefineValue::parse_expr("202311L")?;
            "FLT_EPSILON" = DefineValue::RustCode("::core::primitive::f32::EPSILON".into());
            "INT64_C"("x") = DefineValue::RustCode("{x}_i64".into());
            "UINT64_C"("x") = DefineValue::RustCode("{x}_u64".into());
            "SIZE_MAX" = DefineValue::RustCode("::core::primitive::usize::MAX".into());
            "__has_builtin"("builtin") = DefineValue::Other(Span::new_inline("__has_builtin"));
            "SDL_DISABLE_ALLOCA" = DefineValue::one();
            "SDL_DISABLE_ANALYZE_MACROS" = DefineValue::one();
        }

        Ok(Self {
            inner: Rc::new(RefCell::new(InnerEmitContext {
                module,
                preproc_state: Rc::new(RefCell::new(preproc_state)),
                scope: Scope::new(),
                preproc_eval_mode: 0,
                emitted_file_doc: false,
            })),
            output,
            indent: 0,
            newline_count: 0,
            do_indent: false,
            ool_output: String::new(),
            gen,
        })
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

    pub fn with_output<'o>(&self, output: &'o mut dyn Write) -> EmitContext<'o, 'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output,
            indent: 0,
            newline_count: 0,
            do_indent: false,
            ool_output: String::new(),
            gen: self.gen,
        }
    }

    pub fn with_ool_output<'o>(&'o mut self) -> EmitContext<'o, 'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output: &mut self.ool_output,
            indent: self.indent,
            newline_count: 0,
            do_indent: self.do_indent,
            ool_output: String::new(),
            gen: self.gen,
        }
    }

    pub fn flush_ool_output(&mut self) -> EmitResult {
        self.output.write_str(&self.ool_output)?;
        self.ool_output.clear();
        Ok(())
    }

    pub fn use_ident(&self, _ident: &Ident) -> EmitResult {
        Ok(())
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

    pub fn lookup_preproc(&self, key: &Ident) -> Option<(Option<Vec<IdentOrKw>>, DefineValue)> {
        if let Ok(Some(def)) = self.preproc_state().borrow().lookup(key) {
            Some(def)
        } else {
            None
        }
    }

    pub fn lookup_sym(&self, key: &Ident) -> Option<Ident> {
        if let Some(_) = self.lookup_preproc(key) {
            todo!()
        } else {
            self.scope().lookup(key)
        }
    }

    pub fn lookup_enum_sym(&self, key: &Ident) -> Option<Ident> {
        if let Some(_) = self.lookup_preproc(key) {
            todo!()
        } else {
            self.scope().lookup_enum(key)
        }
    }

    pub fn lookup_struct_sym(&self, key: &Ident) -> Option<(Ident, bool)> {
        if let Some(_) = self.lookup_preproc(key) {
            todo!()
        } else {
            self.scope().lookup_struct(key)
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
}

impl Drop for EmitContext<'_, '_> {
    fn drop(&mut self) {
        self.flush_ool_output().unwrap();
        self.inner.borrow_mut().scope.pop(self.output).unwrap();
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
    defined: HashMap<Ident, (Option<Vec<IdentOrKw>>, DefineValue)>,
    undefined: HashSet<Ident>,
    target_defines: HashMap<&'static str, CfgExpr>,
}

impl PreProcState {
    pub fn with_parent(parent: Rc<RefCell<PreProcState>>) -> Self {
        Self {
            parent: Some(parent),
            defined: HashMap::new(),
            undefined: HashSet::new(),
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
        args: Option<Vec<IdentOrKw>>,
        value: DefineValue,
    ) -> EmitResult {
        if let Ok(true) = self.is_defined(&key) {
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

    pub fn register_target_define(&mut self, key: &'static str, value: CfgExpr) {
        self.target_defines.insert(key, value);
    }

    pub fn lookup(
        &self,
        key: &Ident,
    ) -> Result<Option<(Option<Vec<IdentOrKw>>, DefineValue)>, EmitErr> {
        if let Some(value) = self.defined.get(key) {
            Ok(Some(value.clone()))
        } else if self.undefined.contains(key) {
            Ok(None)
        } else if let Some(parent) = &self.parent {
            parent.borrow().lookup(key)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
    }

    pub fn is_defined(&self, key: &Ident) -> Result<bool, EmitErr> {
        if self.is_target_define(key) || self.defined.contains_key(key) {
            Ok(true)
        } else if self.undefined.contains(key) {
            Ok(false)
        } else if let Some(parent) = &self.parent {
            parent.borrow().is_defined(key)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
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

#[derive(Clone)]
pub struct Scope(Rc<RefCell<InnerScope>>);

#[derive(Default)]
struct InnerScope {
    parent: Option<Rc<RefCell<InnerScope>>>,
    syms: HashSet<Ident>,
    enum_syms: HashSet<Ident>,
    struct_syms: BTreeMap<Ident, bool>,
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

    pub fn pop(&mut self, f: &mut dyn Write) -> EmitResult {
        for s in self
            .0
            .borrow()
            .struct_syms
            .iter()
            .filter_map(|(s, d)| (!d).then_some(s))
        {
            writeln!(f, "#[repr(C)]")?;
            writeln!(f, "#[non_exhaustive]")?;
            writeln!(
                f,
                "pub struct {} {{ _opaque: [::core::primitive::u8; 0] }}",
                s.as_str()
            )?;
            writeln!(f)?;
        }

        let parent = self.0.borrow().parent.as_ref().map(Rc::clone);
        if let Some(parent) = parent {
            self.0 = parent;
        }

        Ok(())
    }

    pub fn register_sym(&mut self, ident: Ident) -> EmitResult {
        let span = ident.span();
        if !self.0.borrow_mut().syms.insert(ident) {
            return Err(ParseErr::new(span, "symbol already defined in this scope").into());
        }
        Ok(())
    }

    pub fn register_enum_sym(&mut self, ident: Ident) -> EmitResult {
        let span = ident.span();
        if !self.0.borrow_mut().enum_syms.insert(ident) {
            return Err(ParseErr::new(span, "enum symbol already defined in this scope").into());
        }
        Ok(())
    }

    pub fn register_struct_sym(&mut self, ident: Ident, defined: bool) -> EmitResult {
        let span = ident.span();
        if let Some(true) = self.0.borrow_mut().struct_syms.insert(ident, defined) {
            if defined {
                return Err(
                    ParseErr::new(span, "struct symbol already defined in this scope").into(),
                );
            }
        }
        Ok(())
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().syms.get(ident).cloned()
    }

    pub fn lookup_enum(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().enum_syms.get(ident).cloned()
    }

    pub fn lookup_struct(&self, ident: &Ident) -> Option<(Ident, bool)> {
        self.0
            .borrow()
            .struct_syms
            .get_key_value(ident)
            .map(|(i, d)| (i.clone(), *d))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
