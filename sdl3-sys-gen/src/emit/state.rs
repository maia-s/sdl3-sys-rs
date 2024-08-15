use super::{EmitErr, EmitResult};
use crate::{
    parse::{DefineValue, GetSpan, Ident, IdentOrKw, ParseErr, Span},
    Gen,
};
use core::mem;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeSet, HashMap, HashSet},
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

        const ALWAYS_FALSE: &str = "any()";

        preproc_state.register_target_define(
            "__LP64__",
            CfgExpr(r#"all(not(windows), target_pointer_width = "64")"#),
        );
        preproc_state
            .register_target_define("_MSC_VER", CfgExpr(r#"all(windows, target_env = "msvc")"#));
        preproc_state.register_target_define("SDL_PLATFORM_3DS", CfgExpr(ALWAYS_FALSE));
        preproc_state
            .register_target_define("SDL_PLATFORM_APPLE", CfgExpr(r#"target_vendor = "apple""#));
        preproc_state.register_target_define("SDL_PLATFORM_GDK", CfgExpr(ALWAYS_FALSE)); // change WIN32 if this is changed
        preproc_state.register_target_define("SDL_PLATFORM_VITA", CfgExpr(ALWAYS_FALSE));
        preproc_state.register_target_define("SDL_PLATFORM_WIN32", CfgExpr("windows"));

        preproc_state.undefine(Ident::new_inline("__clang_analyzer__"));
        preproc_state.undefine(Ident::new_inline("__cplusplus"));
        preproc_state.undefine(Ident::new_inline("PRId32"));
        preproc_state.undefine(Ident::new_inline("PRIs64"));
        preproc_state.undefine(Ident::new_inline("PRIu32"));
        preproc_state.undefine(Ident::new_inline("PRIu64"));
        preproc_state.undefine(Ident::new_inline("PRIx32"));
        preproc_state.undefine(Ident::new_inline("PRIX32"));
        preproc_state.undefine(Ident::new_inline("PRIx64"));
        preproc_state.undefine(Ident::new_inline("PRIX64"));
        preproc_state.undefine(Ident::new_inline("DOXYGEN_SHOULD_IGNORE_THIS"));
        preproc_state.undefine(Ident::new_inline(format!("SDL_{module}_h_")));
        preproc_state.undefine(Ident::new_inline("SDL_COMPILE_TIME_ASSERT"));
        preproc_state.undefine(Ident::new_inline("SDL_FUNCTION_POINTER_IS_VOID_POINTER"));
        preproc_state.undefine(Ident::new_inline("SDL_memcpy"));
        preproc_state.undefine(Ident::new_inline("SDL_memmove"));
        preproc_state.undefine(Ident::new_inline("SDL_memset"));
        preproc_state.undefine(Ident::new_inline("SDL_PI_D"));
        preproc_state.undefine(Ident::new_inline("SDL_PI_F"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIs32"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIs64"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIu32"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIu64"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIx32"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIX32"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIx64"));
        preproc_state.undefine(Ident::new_inline("SDL_PRIX64"));
        preproc_state.undefine(Ident::new_inline("SDL_SLOW_MEMCPY"));
        preproc_state.undefine(Ident::new_inline("SDL_SLOW_MEMMOVE"));
        preproc_state.undefine(Ident::new_inline("SDL_SLOW_MEMSET"));

        preproc_state.define(
            Ident::new_inline("__STDC_VERSION__"),
            None,
            DefineValue::parse_expr("202311L")?,
        )?;
        preproc_state.define(
            Ident::new_inline("FLT_EPSILON"),
            None,
            DefineValue::RustCode("::core::primitive::f32::EPSILON".into()),
        )?;
        preproc_state.define(
            Ident::new_inline("SIZE_MAX"),
            None,
            DefineValue::RustCode("::core::primitive::usize::MAX".into()),
        )?;
        preproc_state.define(
            Ident::new_inline("__has_builtin"),
            Some(vec![IdentOrKw::new_inline("builtin")]),
            DefineValue::Other(Span::new_inline("__has_builtin")),
        )?;
        preproc_state.define(
            Ident::new_inline("SDL_WIKI_DOCUMENTATION_SECTION"),
            None,
            DefineValue::one(),
        )?;

        preproc_state.define(
            Ident::new_inline("SDL_DISABLE_ALLOCA"),
            None,
            DefineValue::one(),
        )?;
        preproc_state.define(
            Ident::new_inline("SDL_DISABLE_ANALYZE_MACROS"),
            None,
            DefineValue::one(),
        )?;
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
        Rc::into_inner(self.inner).unwrap().into_inner()
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

    pub fn lookup_sym(&self, key: &Ident) -> Option<Ident> {
        if let Ok(Some(_)) = self.preproc_state().borrow().lookup(key) {
            todo!()
        } else {
            self.scope().lookup(key)
        }
    }

    pub fn lookup_enum_sym(&self, key: &Ident) -> Option<Ident> {
        if let Ok(Some(_)) = self.preproc_state().borrow().lookup(key) {
            todo!()
        } else {
            self.scope().lookup_enum(key)
        }
    }

    pub fn lookup_struct_sym(&self, key: &Ident) -> Option<Ident> {
        if let Ok(Some(_)) = self.preproc_state().borrow().lookup(key) {
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

/*
"" => [""]
"\n" => ["", ""]
"aaa" => ["aaa"]
"aaa\n" => ["aaa", ""]
"aaa\nbbb" => ["aaa", "bbb"]
"aaa\nbbb\n" => ["aaa, "bbb", ""]
*/

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
        if self.defined.contains_key(key) {
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

struct InnerScope {
    parent: Option<Rc<RefCell<InnerScope>>>,
    syms: HashSet<Ident>,
    enum_syms: HashSet<Ident>,
    struct_syms: HashSet<Ident>,
}

impl Scope {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerScope {
            parent: None,
            syms: HashSet::new(),
            enum_syms: HashSet::new(),
            struct_syms: HashSet::new(),
        })))
    }

    pub fn push(&mut self) {
        self.0 = Rc::new(RefCell::new(InnerScope {
            parent: Some(Rc::clone(&self.0)),
            syms: HashSet::new(),
            enum_syms: HashSet::new(),
            struct_syms: HashSet::new(),
        }));
    }

    pub fn pop(&mut self) {
        let parent = if let Some(parent) = &self.0.borrow().parent {
            Rc::clone(parent)
        } else {
            panic!("popped top level scope");
        };
        self.0 = parent;
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

    pub fn register_struct_sym(&mut self, ident: Ident) -> EmitResult {
        let span = ident.span();
        if !self.0.borrow_mut().struct_syms.insert(ident) {
            return Err(ParseErr::new(span, "struct symbol already defined in this scope").into());
        }
        Ok(())
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().syms.get(ident).cloned()
    }

    pub fn lookup_enum(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().enum_syms.get(ident).cloned()
    }

    pub fn lookup_struct(&self, ident: &Ident) -> Option<Ident> {
        self.0.borrow().struct_syms.get(ident).cloned()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
