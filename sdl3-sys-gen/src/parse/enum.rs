use super::{
    DocComment, Expr, ExprNoComma, Ident, Kw_enum, Op, Parse, ParseContext, ParseRawRes, Span,
    WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Enum {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub ident: Option<Ident>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub doc: Option<DocComment>,
    pub ident: Ident,
    pub expr: Option<Expr>,
}

impl Parse for Enum {
    fn desc() -> Cow<'static, str> {
        "enum".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        if let Some(enum_kw) = Kw_enum::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let ident = Ident::try_parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            Op::<'{'>::parse(ctx, &mut rest)?;
            let mut variants = Vec::new();
            loop {
                WsAndComments::try_parse(ctx, &mut rest)?;
                let v_doc = DocComment::try_parse(ctx, &mut rest)?;
                if let Some(v_ident) = Ident::try_parse(ctx, &mut rest)? {
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    let expr = if <Op![=]>::try_parse(ctx, &mut rest)?.is_some() {
                        WsAndComments::try_parse(ctx, &mut rest)?;
                        let expr = ExprNoComma::parse(ctx, &mut rest)?;
                        WsAndComments::try_parse(ctx, &mut rest)?;
                        Some(expr.0)
                    } else {
                        None
                    };
                    let got_comma = <Op![,]>::try_parse(ctx, &mut rest)?.is_some();
                    let v_doc = DocComment::try_parse_combine_postfix(ctx, &mut rest, v_doc)?;
                    variants.push(EnumVariant {
                        doc: v_doc,
                        ident: v_ident,
                        expr,
                    });
                    if !got_comma {
                        break;
                    }
                } else if let Some(_v_doc) = v_doc {
                    // FIXME: group doc comment (skip for now)
                } else {
                    break;
                }
            }
            WsAndComments::try_parse(ctx, &mut rest)?;
            let closing_brace = Op::<'}'>::parse(ctx, &mut rest)?;
            let span = doc
                .as_ref()
                .map(|dc| dc.span.clone())
                .unwrap_or(enum_kw.span)
                .join(&closing_brace.span);
            Ok((
                rest,
                Some(Self {
                    span,
                    doc,
                    ident,
                    variants,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
