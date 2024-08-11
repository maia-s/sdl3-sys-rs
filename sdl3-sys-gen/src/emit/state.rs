use super::{EmitErr, EmitResult};
use crate::parse::{DefineValue, GetSpan, Ident, IdentOrKw, ParseErr};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    fmt::{self, Write},
    rc::Rc,
};

pub struct EmitContext<'a> {
    inner: Rc<RefCell<InnerEmitContext>>,
    output: &'a mut dyn Write,
}

pub struct InnerEmitContext {
    module: String,
    preprocstate: PreProcState,
    scope: Scope,
    enum_scope: Scope,
}

impl<'a> EmitContext<'a> {
    pub fn new(module: impl Into<String>, output: &'a mut dyn Write) -> Self {
        let module = module.into();
        let mut preprocstate = PreProcState::default();
        preprocstate.undefine(Ident::new_inline("__cplusplus"));
        preprocstate.undefine(Ident::new_inline(format!("SDL_{module}_h_")));
        Self {
            inner: Rc::new(RefCell::new(InnerEmitContext {
                module,
                preprocstate,
                scope: Scope::new(),
                enum_scope: Scope::new(),
            })),
            output,
        }
    }

    pub fn module(&self) -> Ref<str> {
        Ref::map(self.inner.borrow(), |ctx| ctx.module.as_str())
    }

    pub fn preprocstate(&mut self) -> Ref<PreProcState> {
        Ref::map(self.inner.borrow(), |ctx| &ctx.preprocstate)
    }

    pub fn preprocstate_mut(&mut self) -> RefMut<PreProcState> {
        RefMut::map(self.inner.borrow_mut(), |ctx| &mut ctx.preprocstate)
    }

    pub fn scope(&mut self) -> Ref<Scope> {
        Ref::map(self.inner.borrow(), |ctx| &ctx.scope)
    }

    pub fn scope_mut(&mut self) -> RefMut<Scope> {
        RefMut::map(self.inner.borrow_mut(), |ctx| &mut ctx.scope)
    }

    pub fn enum_scope(&mut self) -> Ref<Scope> {
        Ref::map(self.inner.borrow(), |ctx| &ctx.enum_scope)
    }

    pub fn enum_scope_mut(&mut self) -> RefMut<Scope> {
        RefMut::map(self.inner.borrow_mut(), |ctx| &mut ctx.enum_scope)
    }

    pub fn with_output<'b>(&self, output: &'b mut dyn Write) -> EmitContext<'b> {
        EmitContext {
            inner: Rc::clone(&self.inner),
            output,
        }
    }

    pub fn use_ident(&self, ident: &Ident) -> EmitResult {
        Ok(())
    }
}

impl Write for EmitContext<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.write_str(s)
    }
}

#[derive(Default)]
pub struct PreProcState {
    defined: HashMap<Ident, (Option<Vec<IdentOrKw>>, DefineValue)>,
    undefined: HashSet<Ident>,
}

impl PreProcState {
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

    pub fn lookup(
        &self,
        key: &Ident,
    ) -> Result<Option<(&Ident, &(Option<Vec<IdentOrKw>>, DefineValue))>, EmitErr> {
        if let Some((ident, value)) = self.defined.get_key_value(key) {
            Ok(Some((ident, value)))
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
