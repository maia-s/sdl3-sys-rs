use crate::parse::ParseErr;

use super::{
    Delimited, GetSpan, Ident, IdentOrKw, Kw_sizeof, Literal, Op, Parse, ParseRawRes, Precedence,
    Punctuated, Span, Type, WsAndComments,
};

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    FnCall(FnCall),
    Cast(Box<Cast>),
    SizeOf(Box<SizeOf>),
}

impl Expr {
    pub fn try_parse_raw_with_prec(input: &Span, prec: Precedence) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(cast)) = Cast::try_parse_raw(input)? {
            // FIXME: precedence
            Ok((rest, Some(Self::Cast(Box::new(cast)))))
        } else if let (rest, Some(sizeof)) = SizeOf::try_parse_raw(input)? {
            // FIXME: precedence
            Ok((rest, Some(Self::SizeOf(Box::new(sizeof)))))
        } else if let (
            rest,
            Some(Delimited {
                open: _,
                value: expr,
                close: _,
            }),
        ) = Delimited::<Op<'('>, Expr, Op<')'>>::try_parse_raw(input)?
        {
            Ok((rest, Some(expr)))
        } else if let (rest, Some(call)) = FnCall::try_parse_raw(input)? {
            Ok((rest, Some(Self::FnCall(call))))
        } else if let (rest, Some(ident)) = Ident::try_parse_raw(input)? {
            Ok((rest, Some(Self::Ident(ident))))
        } else if let (rest, Some(lit)) = Literal::try_parse_raw(input)? {
            Ok((rest, Some(Self::Literal(lit))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl GetSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Ident(e) => e.span(),
            Self::Literal(e) => e.span(),
            Self::FnCall(e) => e.span(),
            Self::Cast(e) => e.span(),
            Self::SizeOf(e) => e.span(),
        }
    }
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(input: &super::Span) -> ParseRawRes<Option<Self>> {
        Expr::try_parse_raw_with_prec(input, Precedence::None)
    }
}

#[derive(Default)]
pub struct CallArgs(Vec<Expr>);

impl Parse for CallArgs {
    fn desc() -> std::borrow::Cow<'static, str> {
        "arguments".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(args)) = Punctuated::<Expr, Op![,]>::try_parse_raw(input)? {
            Ok((
                rest,
                Some(Self(
                    args.0.into_iter().map(|(v, _)| v).collect::<Vec<Expr>>(),
                )),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Debug)]
pub struct Cast {
    span: Span,
    ty: Type,
    expr: Expr,
}

impl GetSpan for Cast {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Cast {
    fn desc() -> std::borrow::Cow<'static, str> {
        "cast expression".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(open_paren)) = Op::<'('>::try_parse_raw(input)? {
            // this may just be a parenthesized expression, so don't error if something fails to parse
            let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
            if let (rest, Some(ty)) = Type::try_parse_raw(&rest)? {
                let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
                if let (rest, Some(_close_paren)) = Op::<')'>::try_parse_raw(&rest)? {
                    let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
                    if let (rest, Some(expr)) =
                        Expr::try_parse_raw_with_prec(&rest, Precedence::RightToLeft(2))?
                    {
                        return Ok((
                            rest,
                            Some(Self {
                                span: open_paren.span.join(&expr.span()),
                                ty,
                                expr,
                            }),
                        ));
                    }
                }
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Debug)]
pub struct FnCall {
    span: Span,
    ident: IdentOrKw,
    args: Vec<Expr>,
}

impl GetSpan for FnCall {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for FnCall {
    fn desc() -> std::borrow::Cow<'static, str> {
        "function call".into()
    }

    fn try_parse_raw(input: &Span) -> super::ParseRawRes<Option<Self>> {
        let mut rest = input.trim_wsc_start()?;
        if let Some(ident) = IdentOrKw::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            if let Some((args, close)) =
                Delimited::<Op<'('>, Option<CallArgs>, Op<')'>>::try_parse(&mut rest)?
                    .map(|args| (args.value.unwrap_or_default().0, args.close))
            {
                let span = ident.span.join(&close.span);
                return Ok((rest, Some(Self { span, ident, args })));
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Debug)]
pub enum SizeOf {
    Type(Span, Type),
    Expr(Span, Expr),
}

impl GetSpan for SizeOf {
    fn span(&self) -> Span {
        match self {
            Self::Type(span, _) | Self::Expr(span, _) => span.clone(),
        }
    }
}

impl Parse for SizeOf {
    fn desc() -> std::borrow::Cow<'static, str> {
        "sizeof".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(kw)) = Kw_sizeof::try_parse_raw(input)? {
            let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
            if let (rest, Some(_)) = Op::<'('>::try_parse_raw(&rest)? {
                // could be a parenthesized expression
                let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
                if let (rest, Some(ty)) = Type::try_parse_raw(&rest)? {
                    let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
                    let (rest, close_paren) = Op::<')'>::parse_raw(&rest)?;
                    return Ok((rest, Some(Self::Type(kw.span.join(&close_paren.span), ty))));
                }
            }
            if let (rest, Some(expr)) =
                Expr::try_parse_raw_with_prec(&rest, Precedence::RightToLeft(2))?
            {
                return Ok((rest, Some(Self::Expr(kw.span.join(&expr.span()), expr))));
            }
            Err(ParseErr::new(kw.span, "missing operand for `sizeof`"))
        } else {
            Ok((input.clone(), None))
        }
    }
}
