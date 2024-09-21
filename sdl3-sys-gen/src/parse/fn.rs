use super::{
    ArgAttribute, Block, DocComment, FnAbi, FnAttributes, GetSpan, Ident, Kw_extern, Kw_static, Op,
    Parse, ParseContext, ParseErr, ParseRawRes, Punctuated, Span, Type, TypeWithIdent,
    TypeWithOptIdent, WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Function {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub static_kw: Option<Kw_static>,
    pub extern_kw: Option<Kw_extern>,
    pub attr: FnAttributes,
    pub abi: Option<FnAbi>,
    pub ident: Ident,
    pub return_type: Type,
    pub args: FnDeclArgs,
    pub body: Option<Block>,
}

impl GetSpan for Function {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Function {
    fn desc() -> Cow<'static, str> {
        "function declaration".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        let span0 = rest.start();
        let static_kw = Kw_static::try_parse(ctx, &mut rest)?;
        WsAndComments::try_parse(ctx, &mut rest)?;
        let extern_kw = Kw_extern::try_parse(ctx, &mut rest)?;
        if let (Some(_), Some(extern_kw)) = (&static_kw, &extern_kw) {
            return Err(ParseErr::new(extern_kw.span(), "static extern"));
        }
        WsAndComments::try_parse(ctx, &mut rest)?;
        let mut attr: FnAttributes = FnAttributes::parse(ctx, &mut rest)?;
        WsAndComments::try_parse(ctx, &mut rest)?;
        if let Some(ty) = Type::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let abi = FnAbi::try_parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            if let Some(ident) = Ident::try_parse(ctx, &mut rest)? {
                WsAndComments::try_parse(ctx, &mut rest)?;
                if let Some(args) = FnDeclArgs::try_parse(ctx, &mut rest)? {
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    let attr2: FnAttributes = FnAttributes::parse(ctx, &mut rest)?;
                    attr.0.extend(attr2.0);
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    let semi = if extern_kw.is_some() {
                        Some(<Op![;]>::parse(ctx, &mut rest)?)
                    } else {
                        <Op![;]>::try_parse(ctx, &mut rest)?
                    };
                    let body = if semi.is_none() {
                        Some(Block::parse(ctx, &mut rest).map_err(|e| {
                            let msg = format!("{} or `;`", e.message);
                            e.map_msg(msg)
                        })?)
                    } else {
                        None
                    };

                    let span = span0.join(&rest.start());

                    return Ok((
                        rest,
                        Some(Self {
                            span,
                            doc,
                            static_kw,
                            extern_kw,
                            attr,
                            abi,
                            ident,
                            return_type: ty,
                            args,
                            body,
                        }),
                    ));
                }
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone, Debug)]
pub struct FnDeclArgs {
    pub span: Span,
    pub args: Vec<ArgDecl>,
}

impl Parse for FnDeclArgs {
    fn desc() -> Cow<'static, str> {
        "function arguments declaration".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(open_paren) = Op::<'('>::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let mut args: Vec<ArgDecl> = Punctuated::<ArgDecl, Op![,]>::try_parse(ctx, &mut rest)?
                .unwrap_or_default()
                .into();
            if args.len() == 1 && args[0].ty.is_void() {
                args.clear()
            }
            WsAndComments::try_parse(ctx, &mut rest)?;
            if let Some(close_paren) = Op::<')'>::try_parse(ctx, &mut rest)? {
                return Ok((
                    rest,
                    Some(Self {
                        span: open_paren.span.join(&close_paren.span),
                        args,
                    }),
                ));
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone, Debug)]
pub struct ArgDecl {
    pub attr: Option<ArgAttribute>,
    pub ident: Option<Ident>,
    pub ty: Type,
}

impl Parse for ArgDecl {
    fn desc() -> Cow<'static, str> {
        "argument declaration".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, attr) = ArgAttribute::try_parse_raw(ctx, input)?;
        let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
        if let (rest, Some(op)) = <Op![...]>::try_parse_raw(ctx, &rest)? {
            Ok((
                rest,
                Some(Self {
                    attr,
                    ident: None,
                    ty: Type::dotdotdot(op.span),
                }),
            ))
        } else if let (rest, Some(TypeWithIdent { ty, ident })) =
            TypeWithOptIdent::try_parse_raw(ctx, &rest)?
        {
            Ok((rest, Some(Self { attr, ty, ident })))
        } else {
            Ok((input.clone(), None))
        }
    }
}
