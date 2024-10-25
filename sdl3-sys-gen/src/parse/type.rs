use super::{
    patch_parsed_typedef, DocComment, Enum, Expr, FnAbi, FnDeclArgs, GetSpan, Ident, Kw_const,
    Kw_struct, Kw_typedef, Op, Parse, ParseContext, ParseRawRes, PrimitiveType, PrimitiveTypeParse,
    Span, StructOrUnion, WsAndComments,
};
use crate::emit::EmitContext;
use core::cell::RefCell;
use std::{borrow::Cow, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanCopy {
    Default,
    Always,
    Never,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub span: Span,
    pub is_const: bool,
    pub ty: TypeEnum,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.is_const == other.is_const && self.ty == other.ty
    }
}

impl GetSpan for Type {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Type {
    pub fn inner_ty(&self) -> Option<Self> {
        if let TypeEnum::Infer(i) = &self.ty {
            if let Some(ty) = &*i.borrow() {
                ty.inner_ty()
            } else {
                None
            }
        } else {
            Some(self.clone())
        }
    }

    pub fn dotdotdot(span: Span) -> Self {
        Self {
            span,
            is_const: false,
            ty: TypeEnum::DotDotDot,
        }
    }

    pub fn primitive(primitive: PrimitiveType) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Primitive(primitive),
        }
    }

    pub fn pointer(mut ty: Type, is_pointer_to_const: bool) -> Self {
        ty.is_const = is_pointer_to_const;
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Pointer(Box::new(ty)),
        }
    }

    pub fn bool() -> Self {
        Self::primitive(PrimitiveType::Bool)
    }

    pub fn void() -> Self {
        Self::primitive(PrimitiveType::Void)
    }

    pub fn ident(ident: Ident) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Ident(ident),
        }
    }

    pub fn ident_str(ident: &'static str) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Ident(Ident::new_inline(ident)),
        }
    }

    pub fn rust(t: RustType) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Rust(t),
        }
    }

    pub fn function(args: Vec<Type>, return_type: Type, is_const: bool, is_unsafe: bool) -> Self {
        Self {
            span: Span::none(),
            is_const: true,
            ty: TypeEnum::Function(Box::new(FnType {
                return_type,
                args,
                is_const,
                is_unsafe,
            })),
        }
    }

    pub fn infer() -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Infer(Rc::new(RefCell::new(None))),
        }
    }

    pub fn compatible_passing_to(&self, target: &Type) -> bool {
        if self == target {
            return true;
        }
        if let (TypeEnum::Primitive(src), TypeEnum::Primitive(dst)) = (&self.ty, &target.ty) {
            return matches!(
                (src, dst),
                (PrimitiveType::Int, PrimitiveType::Int32T)
                    | (PrimitiveType::Int32T, PrimitiveType::Int)
            );
        }
        false
    }

    pub fn strictly_left_aligned(&self) -> bool {
        self.ty.strictly_left_aligned()
    }

    pub fn get_pointer_type(&self) -> Option<Type> {
        if let TypeEnum::Pointer(p) = &self.ty {
            Some((**p).clone())
        } else {
            None
        }
    }

    pub fn is_array_or_pointer(&self) -> bool {
        matches!(self.ty, TypeEnum::Array(_, _) | TypeEnum::Pointer(_))
    }

    pub fn is_void(&self) -> bool {
        matches!(self.ty, TypeEnum::Primitive(PrimitiveType::Void))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.ty, TypeEnum::Primitive(PrimitiveType::Bool))
    }

    pub fn is_uninferred(&self) -> bool {
        if let TypeEnum::Infer(i) = &self.ty {
            i.borrow().is_none()
        } else {
            false
        }
    }

    pub fn resolve_to(&self, ty: Type) {
        if let TypeEnum::Infer(i) = &self.ty {
            let mut i = i.borrow_mut();
            if i.is_none() {
                *i = Some(ty);
                return;
            }
        }
        assert_eq!(*self, ty, "type already resolved to different type")
    }

    pub fn can_derive_copy(&self, ctx: &EmitContext) -> bool {
        match &self.ty {
            TypeEnum::Primitive(_) => true,
            TypeEnum::Ident(ident) => ctx
                .lookup_sym(ident)
                .map(|s| s.can_derive_copy)
                .unwrap_or(false),
            TypeEnum::Enum(_) => true,
            TypeEnum::Struct(s) => s.can_derive_copy(ctx, None),
            TypeEnum::Pointer(p) => p.is_const,
            TypeEnum::Array(ty, _) => ty.can_derive_copy(ctx),
            TypeEnum::FnPointer(_) => true,
            TypeEnum::DotDotDot => false,
            TypeEnum::Rust(r) => r.can_derive_copy,
            TypeEnum::Function(_) => false,
            TypeEnum::Infer(i) => i
                .borrow()
                .as_ref()
                .map(|i| i.can_derive_copy(ctx))
                .unwrap_or(false),
        }
    }

    pub fn can_derive_debug(&self, ctx: &EmitContext) -> bool {
        match &self.ty {
            TypeEnum::Primitive(_) => true,
            TypeEnum::Ident(ident) => ctx
                .lookup_sym(ident)
                .map(|s| s.can_derive_debug)
                .unwrap_or(false),
            TypeEnum::Enum(_) => true,
            TypeEnum::Struct(s) => s.can_derive_debug(ctx),
            TypeEnum::Pointer(_) => true,
            TypeEnum::Array(ty, _) => ty.can_derive_debug(ctx),
            TypeEnum::FnPointer(_) => true,
            TypeEnum::DotDotDot => false,
            TypeEnum::Rust(r) => r.can_derive_debug,
            TypeEnum::Function(_) => false,
            TypeEnum::Infer(i) => i
                .borrow()
                .as_ref()
                .map(|i| i.can_derive_debug(ctx))
                .unwrap_or(false),
        }
    }
}

impl Parse for Type {
    fn desc() -> Cow<'static, str> {
        "type".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(ty)) = TypeWithNoIdent::try_parse_raw(ctx, input)? {
            Ok((rest, Some(ty.ty)))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeEnum {
    Primitive(PrimitiveType),
    Ident(Ident),
    Enum(Box<Enum>),
    Struct(Box<StructOrUnion>),
    Pointer(Box<Type>),
    Array(Box<Type>, Expr),
    FnPointer(Box<FnPointer>),
    DotDotDot,
    Rust(RustType),
    Function(Box<FnType>),
    Infer(Rc<RefCell<Option<Type>>>),
}

impl TypeEnum {
    pub fn strictly_left_aligned(&self) -> bool {
        matches!(
            self,
            Self::Primitive(_) | Self::Ident(_) | Self::Enum(_) | Self::Struct(_)
        )
    }
}

impl PartialEq for TypeEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(s), Self::Primitive(o)) => s == o,
            (Self::Ident(s), Self::Ident(o)) => s.as_str() == o.as_str(),
            (Self::Pointer(s), Self::Pointer(o)) => s == o,
            (Self::Rust(s), Self::Rust(o)) => s.string == o.string,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RustType {
    pub string: String,
    pub can_derive_copy: bool,
    pub can_derive_debug: bool,
}

#[derive(Clone, Debug)]
pub struct FnPointer {
    pub abi: Option<FnAbi>,
    pub return_type: Type,
    pub args: FnDeclArgs,
}

#[derive(Clone, Debug)]
pub struct FnType {
    pub return_type: Type,
    pub args: Vec<Type>,
    pub is_const: bool,
    pub is_unsafe: bool,
}

const NO_IDENT: u8 = 0;
const OPT_IDENT: u8 = 1;
const REQ_IDENT: u8 = 2;

pub type TypeWithNoIdent = TypeWithIdent<NO_IDENT>;
pub type TypeWithOptIdent = TypeWithIdent<OPT_IDENT>;
pub type TypeWithReqIdent = TypeWithIdent<REQ_IDENT>;

pub struct TypeWithIdent<const IDENT_SPEC: u8> {
    pub ty: Type,
    pub ident: Option<Ident>,
}

impl<const IDENT_SPEC: u8> GetSpan for TypeWithIdent<IDENT_SPEC> {
    fn span(&self) -> Span {
        self.ty.span.join(&self.ty.span)
    }
}

impl<const IDENT_SPEC: u8> Parse for TypeWithIdent<IDENT_SPEC> {
    fn desc() -> Cow<'static, str> {
        "type".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let mut ty = if let Some(PrimitiveTypeParse { span, ty, is_const }) =
            PrimitiveTypeParse::try_parse(ctx, &mut rest)?
        {
            Type {
                span,
                is_const,
                ty: TypeEnum::Primitive(ty),
            }
        } else {
            let mut rest2 = rest.clone();
            let is_const = Kw_const::try_parse(ctx, &mut rest2)?.is_some();
            WsAndComments::try_parse(ctx, &mut rest2)?;
            if let Some(e) = Enum::try_parse(ctx, &mut rest2)? {
                rest = rest2;
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Enum(Box::new(e)),
                }
            } else if let Some(s) = StructOrUnion::try_parse(ctx, &mut rest2)? {
                rest = rest2;
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Struct(Box::new(s)),
                }
            } else if let Some(_kw_struct) = Kw_struct::try_parse(ctx, &mut rest2)? {
                // FIXME: this assumes the struct is typedef'd to the same name
                rest = rest2;
                WsAndComments::try_parse(ctx, &mut rest)?;
                let ident = Ident::parse(ctx, &mut rest)?;
                let is_const = is_const || Kw_const::try_parse(ctx, &mut rest)?.is_some();
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Ident(ident),
                }
            } else if let Some(ident) = Ident::try_parse(ctx, &mut rest2)? {
                rest = rest2;
                let is_const = is_const || Kw_const::try_parse(ctx, &mut rest)?.is_some();
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Ident(ident),
                }
            } else {
                return Ok((input.clone(), None));
            }
        };

        // pointer
        let mut rest2 = rest.clone();
        WsAndComments::try_parse(ctx, &mut rest2)?;
        while <Op![*]>::try_parse(ctx, &mut rest2)?.is_some() {
            rest = rest2.clone();
            WsAndComments::try_parse(ctx, &mut rest2)?;
            let is_const = Kw_const::try_parse(ctx, &mut rest2)?.is_some();
            if is_const {
                rest = rest2.clone();
                WsAndComments::try_parse(ctx, &mut rest2)?;
            }
            let is_restrict =
                Ident::try_parse_if(ctx, &mut rest2, |i| i.as_str() == "SDL_RESTRICT")?.is_some();
            if is_restrict {
                rest = rest2.clone();
                WsAndComments::try_parse(ctx, &mut rest2)?;
            }
            let span = input.start().join(&rest.start());
            ty = Type {
                is_const,
                ty: TypeEnum::Pointer(Box::new(ty)),
                span,
            };
        }

        // prepare function pointer
        rest2 = rest.clone();
        WsAndComments::try_parse(ctx, &mut rest2)?;
        let (abi, ident, args) = if Op::<'('>::try_parse(ctx, &mut rest2)?.is_some() {
            rest = rest2;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let abi = FnAbi::try_parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            if <Op![*]>::try_parse(ctx, &mut rest)?.is_none() {
                let ok = if let Some(abi) = &abi {
                    abi.ident.as_str().ends_with("APIENTRYP")
                } else {
                    false
                };
                if !ok {
                    // it's not a function pointer, and not a type
                    return Ok((input.clone(), None));
                }
            }
            WsAndComments::try_parse(ctx, &mut rest)?;
            let ident = if IDENT_SPEC == NO_IDENT {
                None
            } else if IDENT_SPEC == REQ_IDENT {
                Some(Ident::parse(ctx, &mut rest)?)
            } else {
                Ident::try_parse(ctx, &mut rest)?
            };
            WsAndComments::try_parse(ctx, &mut rest)?;
            Op::<')'>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let args = FnDeclArgs::parse(ctx, &mut rest)?;
            (abi, ident, Some(args))
        } else {
            let ident = if IDENT_SPEC == NO_IDENT {
                None
            } else if IDENT_SPEC == REQ_IDENT {
                rest = rest2;
                if let Some(ident) = Ident::try_parse(ctx, &mut rest)? {
                    Some(ident)
                } else {
                    return Ok((input.clone(), None));
                }
            } else {
                let ident = Ident::try_parse(ctx, &mut rest2)?;
                if ident.is_some() {
                    rest = rest2;
                }
                ident
            };
            (None, ident, None)
        };

        // array
        if ident.is_some() {
            rest2 = rest.clone();
            WsAndComments::try_parse(ctx, &mut rest2)?;
            while Op::<'['>::try_parse(ctx, &mut rest2)?.is_some() {
                rest = rest2;
                WsAndComments::try_parse(ctx, &mut rest)?;
                let expr = Expr::try_parse(ctx, &mut rest)?;
                WsAndComments::try_parse(ctx, &mut rest)?;
                Op::<']'>::parse(ctx, &mut rest)?;
                ty = Type {
                    span: input.start().join(&rest.start()),
                    is_const: true,
                    ty: if let Some(expr) = expr {
                        TypeEnum::Array(Box::new(ty), expr)
                    } else {
                        TypeEnum::Pointer(Box::new(ty))
                    },
                };
                rest2 = rest.clone();
                WsAndComments::try_parse(ctx, &mut rest2)?;
            }
        }

        // finish function pointer
        if let Some(args) = args {
            let span = input.start().join(&rest.start());
            ty = Type {
                span,
                is_const: true,
                ty: TypeEnum::FnPointer(Box::new(FnPointer {
                    abi,
                    return_type: ty,
                    args,
                })),
            };
        }

        Ok((rest, Some(Self { ty, ident })))
    }
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub ident: Ident,
    pub ty: Type,
    pub use_for_defines: Option<&'static str>,
    pub associated_defines: Rc<RefCell<Vec<(Ident, Option<DocComment>)>>>,
}

impl Parse for TypeDef {
    fn desc() -> Cow<'static, str> {
        "typedef".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        if let Some(typedef_kw) = Kw_typedef::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let TypeWithIdent { ty, ident } = TypeWithReqIdent::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let semi = <Op![;]>::parse(ctx, &mut rest)?;
            let span = typedef_kw.span.join(&semi.span);
            let doc = DocComment::try_parse_combine_postfix(ctx, &mut rest, doc)?;
            let use_for_defines = matches!(ty.ty, TypeEnum::Ident(_) | TypeEnum::Primitive(_));
            let mut this = TypeDef {
                span,
                doc,
                ident: ident.unwrap(),
                ty,
                use_for_defines: use_for_defines.then_some(""),
                associated_defines: Rc::new(RefCell::new(Vec::new())),
            };
            patch_parsed_typedef(ctx, &mut this)?;
            Ok((rest, Some(this)))
        } else {
            Ok((input.clone(), None))
        }
    }
}
