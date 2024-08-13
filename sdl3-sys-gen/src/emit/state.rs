use super::{EmitErr, EmitResult};
use crate::{
    parse::{DefineValue, GetSpan, Ident, IdentOrKw, IdentOrKwT, ParseErr},
    Gen,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{self, Write},
    iter::Extend,
    rc::Rc,
};

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
    pub gen: &'b Gen,
}

pub struct InnerEmitContext {
    module: String,
    pub preprocstate: PreProcState,
    scope: Scope,
    enum_scope: Scope,
    preproc_eval_mode: usize,
}

impl<'a, 'b> EmitContext<'a, 'b> {
    pub fn new(
        module: impl Into<String>,
        output: &'a mut dyn Write,
        gen: &'b Gen,
    ) -> Result<Self, EmitErr> {
        let module = module.into();
        let mut preprocstate = PreProcState::default();
        preprocstate.undefine(Ident::new_inline("__cplusplus"));
        preprocstate.undefine(Ident::new_inline(format!("SDL_{module}_h_")));
        preprocstate.define(
            Ident::new_inline("__STDC_VERSION__"),
            None,
            DefineValue::parse_expr("202311L")?,
        )?;
        preprocstate.define(
            Ident::new_inline("SDL_WIKI_DOCUMENTATION_SECTION"),
            None,
            DefineValue::one(),
        )?;
        Ok(Self {
            inner: Rc::new(RefCell::new(InnerEmitContext {
                module,
                preprocstate,
                scope: Scope::new(),
                enum_scope: Scope::new(),
                preproc_eval_mode: 0,
            })),
            output,
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

    pub fn preprocstate(&self) -> Ref<PreProcState> {
        self.inner_map(|ctx| &ctx.preprocstate)
    }

    pub fn preprocstate_mut(&mut self) -> RefMut<PreProcState> {
        self.inner_mut_map(|ctx| &mut ctx.preprocstate)
    }

    pub fn scope(&self) -> Ref<Scope> {
        self.inner_map(|ctx| &ctx.scope)
    }

    pub fn scope_mut(&mut self) -> RefMut<Scope> {
        self.inner_mut_map(|ctx| &mut ctx.scope)
    }

    pub fn enum_scope(&self) -> Ref<Scope> {
        self.inner_map(|ctx| &ctx.enum_scope)
    }

    pub fn enum_scope_mut(&mut self) -> RefMut<Scope> {
        self.inner_mut_map(|ctx| &mut ctx.enum_scope)
    }

    pub fn with_output<'o>(&self, output: &'o mut dyn Write) -> EmitContext<'o, 'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output,
            gen: self.gen,
        }
    }

    pub fn use_ident(&self, ident: &Ident) -> EmitResult {
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
}

impl Write for EmitContext<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.write_str(s)
    }
}

#[derive(Default)]
pub struct PreProcState {
    defined: HashMap<Ident, (Option<Vec<IdentOrKw>>, DefineValue)>,
    undefined: HashSet<Ident>,
    target_defines: HashSet<Ident>,
}

impl PreProcState {
    pub fn include(&mut self, include: &Self) -> EmitResult {
        for key in include.undefined.iter() {
            self.undefine(key.clone());
        }
        for (key, (args, value)) in include.defined.iter() {
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
        if self.defined.contains_key(&key) {
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

    pub fn register_target_define(&mut self, key: Ident) {
        self.target_defines.insert(key);
    }

    pub fn lookup(
        &self,
        key: &Ident,
    ) -> Result<Option<&(Option<Vec<IdentOrKw>>, DefineValue)>, EmitErr> {
        if let Some(value) = self.defined.get(key) {
            Ok(Some(value))
        } else if self.undefined.contains(key) {
            Ok(None)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
    }

    pub fn is_defined(&self, key: &Ident) -> Result<bool, EmitErr> {
        if self.defined.contains_key(key) {
            Ok(true)
        } else if self.undefined.contains(key) {
            Ok(false)
        } else {
            Err(ParseErr::new(key.span(), "unknown define").into())
        }
    }

    pub fn is_target_define(&self, key: &Ident) -> bool {
        self.target_defines.contains(key)
    }
}

#[derive(Clone)]
pub struct Scope(Rc<RefCell<InnerScope>>);

struct InnerScope {
    parent: Option<Rc<RefCell<InnerScope>>>,
    syms: HashSet<Ident>,
}

impl Scope {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerScope {
            parent: None,
            syms: HashSet::new(),
        })))
    }

    pub fn push(&mut self) {
        self.0 = Rc::new(RefCell::new(InnerScope {
            parent: Some(Rc::clone(&self.0)),
            syms: HashSet::new(),
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
}
