use super::{
    DocComment, Enum, Expr, FnAbi, FnDeclArgs, GetSpan, Ident, Kw_const, Kw_typedef, Op, Parse,
    ParseContext, ParseRawRes, PrimitiveType, PrimitiveTypeParse, Span, StructOrUnion,
    WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Type {
    pub span: Span,
    pub is_const: bool,
    pub ty: TypeEnum,
}

impl GetSpan for Type {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Type {
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

    pub fn ident(ident: Ident) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Ident(ident),
        }
    }

    pub fn rust(rust: impl Into<String>) -> Self {
        Self {
            span: Span::none(),
            is_const: false,
            ty: TypeEnum::Rust(rust.into()),
        }
    }

    pub fn strictly_left_aligned(&self) -> bool {
        self.ty.strictly_left_aligned()
    }

    pub fn is_array_or_pointer(&self) -> bool {
        matches!(self.ty, TypeEnum::Array(_, _) | TypeEnum::Pointer(_))
    }

    pub fn is_void(&self) -> bool {
        matches!(self.ty, TypeEnum::Primitive(PrimitiveType::Void))
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
    Rust(String),
}

impl TypeEnum {
    pub fn strictly_left_aligned(&self) -> bool {
        matches!(
            self,
            Self::Primitive(_) | Self::Ident(_) | Self::Enum(_) | Self::Struct(_)
        )
    }
}

#[derive(Clone, Debug)]
pub struct FnPointer {
    pub abi: Option<FnAbi>,
    pub return_type: Type,
    pub args: FnDeclArgs,
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
            Ok((
                rest,
                Some(TypeDef {
                    span,
                    doc,
                    ident: ident.unwrap(),
                    ty,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
