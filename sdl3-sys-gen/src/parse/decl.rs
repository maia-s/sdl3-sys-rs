use super::{
    ArgAttribute, DocComment, FnAbi, FnAttribute, Ident, Kw_extern, Op, Parse, ParseRawRes,
    Punctuated, Span, Type, TypeWithIdent, TypeWithOptIdent, TypeWithReqIdent, WsAndComments,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct FnDecl {
    span: Span,
    doc: Option<DocComment>,
    attr: Option<FnAttribute>,
    abi: Option<FnAbi>,
    ident: Ident,
    return_type: Type,
    args: FnDeclArgs,
}

impl Parse for FnDecl {
    fn desc() -> Cow<'static, str> {
        "function declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        if let Some(extern_kw) = Kw_extern::try_parse(&mut rest)? {
            WsAndComments::parse(&mut rest)?;
            if Ident::try_parse_eq(&mut rest, "SDL_DECLSPEC")?.is_some() {
                WsAndComments::parse(&mut rest)?;
                let ty = Type::parse(&mut rest)?;
                WsAndComments::parse(&mut rest)?;
                let abi = FnAbi::parse(&mut rest)?;
                WsAndComments::parse(&mut rest)?;
                let ident = Ident::parse(&mut rest)?;
                WsAndComments::try_parse(&mut rest)?;
                let args = FnDeclArgs::parse(&mut rest)?;
                WsAndComments::try_parse(&mut rest)?;
                let attr = FnAttribute::try_parse(&mut rest)?;
                WsAndComments::try_parse(&mut rest)?;
                let semi = <Op![;]>::parse(&mut rest)?;

                let span = if let Some(doc) = &doc {
                    doc.span.clone()
                } else {
                    extern_kw.span
                }
                .join(&semi.span);
                return Ok((
                    rest,
                    Some(Self {
                        span,
                        doc,
                        attr,
                        abi: Some(abi),
                        ident,
                        return_type: ty,
                        args,
                    }),
                ));
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

#[derive(Debug)]
pub struct VarDecl {
    ident: Ident,
    ty: Type,
}

impl Parse for VarDecl {
    fn desc() -> Cow<'static, str> {
        "variable declaration".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(TypeWithIdent { ty, ident })) = TypeWithReqIdent::try_parse_raw(input)? {
            let ident = ident.unwrap();
            Ok((rest, Some(VarDecl { ty, ident })))
        } else {
            Ok((input.clone(), None))
        }
    }
}
