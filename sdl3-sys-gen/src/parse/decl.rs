use super::{
    DocComment, Ident, Kw_extern, Op, Parse, ParseRawRes, Punctuated, Span, Type, TypeWithIdent,
    TypeWithReqIdent, WsAndComments,
};
use std::borrow::Cow;

pub struct FnDecl {
    span: Span,
    doc: Option<DocComment>,
    ident: Ident,
    return_type: Type,
    args: Punctuated<VarDecl, Op![,]>,
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
                Ident::parse_eq(&mut rest, "SDLCALL")?;
                WsAndComments::parse(&mut rest)?;
                let ident = Ident::parse(&mut rest)?;
                WsAndComments::try_parse(&mut rest)?;
                Op::<'('>::parse(&mut rest)?;
                WsAndComments::try_parse(&mut rest)?;
                let args = Punctuated::try_parse(&mut rest)?.unwrap_or_default();
                WsAndComments::try_parse(&mut rest)?;
                Op::<')'>::parse(&mut rest)?;
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
