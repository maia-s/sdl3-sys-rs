use super::{
    ArgAttribute, Balanced, DocComment, FnAbi, FnAttribute, FnAttributes, GetSpan, Ident,
    Kw_extern, Op, Parse, ParseRawRes, Punctuated, Span, Type, TypeWithIdent, TypeWithOptIdent,
    WsAndComments,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Function {
    span: Span,
    doc: Option<DocComment>,
    attr: Vec<FnAttribute>,
    abi: Option<FnAbi>,
    ident: Ident,
    return_type: Type,
    args: FnDeclArgs,
    body: Option<Span>,
}

impl Parse for Function {
    fn desc() -> Cow<'static, str> {
        "function declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        let span0 = rest.start();
        let _extern_kw = Kw_extern::try_parse(&mut rest)?;
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
                    let semi = <Op![;]>::try_parse(&mut rest)?;
                    let body = if semi.is_none() {
                        Some(
                            Balanced::<Op<'{'>, Op<'}'>>::parse(&mut rest)
                                .map_err(|e| {
                                    let msg = format!("{} or `;`", e.message);
                                    e.map_msg(msg)
                                })?
                                .inner,
                        )
                    } else {
                        None
                    };

                    let span = span0.join(&rest.start());

                    return Ok((
                        rest,
                        Some(Self {
                            span,
                            doc,
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

#[derive(Debug)]
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
            let close_paren = Op::<')'>::parse(&mut rest)?;
            Ok((
                rest,
                Some(Self {
                    span: open_paren.span.join(&close_paren.span),
                    args,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Debug)]
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
