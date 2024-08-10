use crate::parse::{
    Define, DefineValue, DocComment, DocCommentFile, GetSpan, Ident, IdentOrKw, Include, Item,
    ParseErr, PreProcBlock,
};
use core::fmt::{self, Display, Write};
use std::collections::{HashMap, HashSet};

pub type EmitResult = Result<(), EmitErr>;

#[derive(Debug)]
pub enum EmitErr {
    ParseError(ParseErr),
    FmtError(fmt::Error),
}

impl From<ParseErr> for EmitErr {
    fn from(value: ParseErr) -> Self {
        Self::ParseError(value)
    }
}

impl From<fmt::Error> for EmitErr {
    fn from(value: fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

impl Display for EmitErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(e) => Display::fmt(e, f),
            Self::FmtError(e) => Display::fmt(e, f),
        }
    }
}

pub struct EmitContext<'a> {
    module: &'a str,
    output: &'a mut dyn Write,
    preprocstate: PreProcState,
}

impl<'a> EmitContext<'a> {
    pub fn new(module: &'a str, output: &'a mut dyn Write) -> Self {
        let mut preprocstate = PreProcState::default();
        preprocstate.undefine(Ident::new_inline("__cplusplus"));
        preprocstate.undefine(Ident::new_inline(format!("SDL_{module}_h_")));
        Self {
            module,
            output,
            preprocstate,
        }
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

pub trait Emit {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult;
}

impl<T: Emit> Emit for Vec<T> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for i in self.iter() {
            i.emit(ctx)?;
        }
        Ok(())
    }
}

impl Emit for Item {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Item::PreProcBlock(ppb) => ppb.emit(ctx),
            Item::Block(_) => todo!(),
            Item::Skipped(_) => todo!(),
            Item::Define(d) => d.emit(ctx),
            Item::Undef(_) => todo!(),
            Item::Include(i) => i.emit(ctx),
            Item::Pragma(_) => todo!(),
            Item::Error(_) => todo!(),
            Item::FileDoc(dc) => dc.emit(ctx),
            Item::StructOrUnion(_) => todo!(),
            Item::Enum(_) => todo!(),
            Item::Function(_) => todo!(),
            Item::Expr(_) => todo!(),
            Item::FnCall(_) => todo!(),
            Item::TypeDef(_) => todo!(),
            Item::VarDecl(_) => todo!(),
            Item::DoWhile(_) => todo!(),
            Item::For(_) => todo!(),
            Item::IfElse(_) => todo!(),
            Item::Return(_) => todo!(),
        }
    }
}

impl Emit for DocComment {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for line in self.to_string().lines() {
            writeln!(ctx, "/// {}", line)?;
        }
        Ok(())
    }
}

impl Emit for DocCommentFile {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        for line in self.0.to_string().lines() {
            writeln!(ctx, "//! {}", line)?;
        }
        writeln!(ctx)?;
        Ok(())
    }
}

impl<const ALLOW_INITIAL_ELSE: bool> Emit for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match &self.kind {
            crate::parse::PreProcBlockKind::If(_) => todo!(),

            crate::parse::PreProcBlockKind::IfDef(ident) => {
                if ctx.preprocstate.is_defined(ident)? {
                    self.block.emit(ctx)
                } else if let Some(else_block) = &self.else_block {
                    else_block.emit(ctx)
                } else {
                    Ok(())
                }
            }

            crate::parse::PreProcBlockKind::IfNDef(ident) => {
                if !ctx.preprocstate.is_defined(ident)? {
                    self.block.emit(ctx)
                } else if let Some(else_block) = &self.else_block {
                    else_block.emit(ctx)
                } else {
                    Ok(())
                }
            }

            crate::parse::PreProcBlockKind::None => todo!(),
        }
    }
}

impl Emit for Define {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.preprocstate
            .define(self.ident.clone(), self.args.clone(), self.value.clone())
    }
}

impl Emit for Include {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Some(module) = self.path.as_str().strip_prefix("SDL3/SDL_") {
            let module = module.strip_suffix(".h").unwrap();
            writeln!(ctx, "use super::{module}::*;")?;
        }
        Ok(())
    }
}
