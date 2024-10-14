use super::{
    patch_parsed_enum, Conditional, DocComment, Expr, ExprNoComma, GetSpan, Ident, Item, Kw_enum,
    Op, Parse, ParseContext, ParseErr, ParseRawRes, PreProcBlock, Span, Type, WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Enum {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub ident: Option<Ident>,
    pub variants: Vec<EnumVariant>,
    pub base_type: Option<Type>,
    pub hidden: bool,
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
                if let Some(pp) = PreProcBlock::<false>::try_parse(ctx, &mut rest)? {
                    fn merge_pp<const AE: bool>(
                        mut cond: Conditional,
                        variants: &mut Vec<EnumVariant>,
                        pp: &PreProcBlock<AE>,
                    ) -> Result<(), ParseErr> {
                        cond.push(pp.cond_expr.clone());
                        for item in pp.block.iter() {
                            let Item::EnumVariant(ev) = item else {
                                return Err(ParseErr::new(
                                    pp.span(),
                                    "preprocessor block in enum defines non-variants",
                                ));
                            };
                            let mut ev = ev.clone();
                            ev.cond = cond.clone();
                            variants.push(ev);
                        }
                        if let Some(else_block) = &pp.else_block {
                            merge_pp(cond, variants, else_block)
                        } else {
                            Ok(())
                        }
                    }
                    merge_pp(Conditional::new(), &mut variants, &pp)?;
                } else if let Some(variant) = EnumVariant::try_parse(ctx, &mut rest)? {
                    let got_comma = variant.comma.is_some();
                    variants.push(variant);
                    if !got_comma {
                        break;
                    }
                } else if DocComment::try_parse(ctx, &mut rest)?.is_some() {
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
            let mut e = Self {
                span,
                doc,
                ident,
                variants,
                base_type: None,
                hidden: false,
            };
            patch_parsed_enum(ctx, &mut e)?;
            Ok((rest, Some(e)))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub cond: Conditional,
    pub doc: Option<DocComment>,
    pub ident: Ident,
    pub expr: Option<Expr>,
    pub comma: Option<Op![,]>,
}

impl Parse for EnumVariant {
    fn desc() -> Cow<'static, str> {
        "enum variant".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        if let Some(ident) = Ident::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let expr = if <Op![=]>::try_parse(ctx, &mut rest)?.is_some() {
                WsAndComments::try_parse(ctx, &mut rest)?;
                let expr = ExprNoComma::parse(ctx, &mut rest)?;
                WsAndComments::try_parse(ctx, &mut rest)?;
                Some(expr.0)
            } else {
                None
            };
            let comma = <Op![,]>::try_parse(ctx, &mut rest)?;
            let doc = DocComment::try_parse_combine_postfix(ctx, &mut rest, doc)?;
            Ok((
                rest,
                Some(EnumVariant {
                    cond: Conditional::new(),
                    doc,
                    ident,
                    expr,
                    comma,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
