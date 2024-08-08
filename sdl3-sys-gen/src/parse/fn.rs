use super::{
    ArgAttribute, Block, DocComment, FnAbi, FnAttribute, FnAttributes, GetSpan, Ident, Kw_extern,
    Kw_static, Op, Parse, ParseErr, ParseRawRes, Punctuated, Span, Type, TypeWithIdent,
    TypeWithOptIdent, WsAndComments,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Function {
    span: Span,
    doc: Option<DocComment>,
    static_kw: Option<Kw_static>,
    extern_kw: Option<Kw_extern>,
    attr: Vec<FnAttribute>,
    abi: Option<FnAbi>,
    ident: Ident,
    return_type: Type,
    args: FnDeclArgs,
    body: Option<Block>,
}

impl Parse for Function {
    fn desc() -> Cow<'static, str> {
        "function declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        let span0 = rest.start();
        let static_kw = Kw_static::try_parse(&mut rest)?;
        WsAndComments::try_parse(&mut rest)?;
        let extern_kw = Kw_extern::try_parse(&mut rest)?;
        if let (Some(_), Some(extern_kw)) = (&static_kw, &extern_kw) {
            return Err(ParseErr::new(extern_kw.span(), "static extern"));
        }
        WsAndComments::try_parse(&mut rest)?;
        let mut attr: Vec<FnAttribute> = FnAttributes::parse(&mut rest)?.into();
        WsAndComments::try_parse(&mut rest)?;
        if let Some(ty) = Type::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let abi = FnAbi::try_parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            if let Some(ident) = Ident::try_parse(&mut rest)? {
                WsAndComments::try_parse(&mut rest)?;
                if let Some(args) = FnDeclArgs::try_parse(&mut rest)? {
                    WsAndComments::try_parse(&mut rest)?;
                    let attr2: Vec<FnAttribute> = FnAttributes::parse(&mut rest)?.into();
                    attr.extend(attr2);
                    WsAndComments::try_parse(&mut rest)?;
                    let semi = if extern_kw.is_some() {
                        Some(<Op![;]>::parse(&mut rest)?)
                    } else {
                        <Op![;]>::try_parse(&mut rest)?
                    };
                    let body = if semi.is_none() {
                        Some(Block::parse(&mut rest).map_err(|e| {
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
    span: Span,
    args: Vec<ArgDecl>,
}

impl Parse for FnDeclArgs {
    fn desc() -> Cow<'static, str> {
        "function arguments declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(open_paren) = Op::<'('>::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let args = Punctuated::<ArgDecl, Op![,]>::try_parse(&mut rest)?
                .unwrap_or_default()
                .into();
            WsAndComments::try_parse(&mut rest)?;
            if let Some(close_paren) = Op::<')'>::try_parse(&mut rest)? {
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
    attr: Option<ArgAttribute>,
    ident: Option<Ident>,
    ty: Type,
}

impl Parse for ArgDecl {
    fn desc() -> Cow<'static, str> {
        "argument declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, attr) = ArgAttribute::try_parse_raw(input)?;
        let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
        if let (rest, Some(op)) = <Op![...]>::try_parse_raw(&rest)? {
            Ok((
                rest,
                Some(Self {
                    attr,
                    ident: None,
                    ty: Type::dotdotdot(op.span),
                }),
            ))
        } else if let (rest, Some(TypeWithIdent { ty, ident })) =
            TypeWithOptIdent::try_parse_raw(&rest)?
        {
            Ok((rest, Some(Self { attr, ty, ident })))
        } else {
            Ok((input.clone(), None))
        }
    }
}
