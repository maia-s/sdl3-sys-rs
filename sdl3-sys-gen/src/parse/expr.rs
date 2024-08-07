use crate::parse::{ExprOp, ParseErr};

use super::{
    Delimited, GetSpan, IdentOrKw, Kw_sizeof, Literal, Op, Parse, ParseRawRes, Precedence,
    Punctuated, Span, Type, WsAndComments,
};

#[derive(Debug)]
pub enum Expr {
    Ident(IdentOrKw),
    Literal(Literal),
    FnCall(FnCall),
    Cast(Box<Cast>),
    SizeOf(Box<SizeOf>),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    pub fn try_parse_raw_with_prec(input: &Span, prec: Precedence) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let mut lhs = if let Some(cast) = Cast::try_parse(&mut rest)? {
            if prec.parse_rhs_first(Precedence::right_to_left(2)) {
                Self::Cast(Box::new(cast))
            } else {
                return Ok((input.clone(), None));
            }
        } else if let Some(sizeof) = SizeOf::try_parse(&mut rest)? {
            if prec.parse_rhs_first(Precedence::right_to_left(2)) {
                Self::SizeOf(Box::new(sizeof))
            } else {
                return Ok((input.clone(), None));
            }
        } else if let Some(Delimited {
            open: _,
            value: expr,
            close: _,
        }) = Delimited::<Op<'('>, Expr, Op<')'>>::try_parse(&mut rest)?
        {
            expr
        } else if let Some(call) = FnCall::try_parse(&mut rest)? {
            Self::FnCall(call)
        } else if let Some(ident) = IdentOrKw::try_parse(&mut rest)? {
            Self::Ident(ident)
        } else if let Some(lit) = Literal::try_parse(&mut rest)? {
            Self::Literal(lit)
        } else {
            return Ok((input.clone(), None));
        };

        loop {
            let mut rest2 = rest.clone();
            WsAndComments::try_parse(&mut rest2)?;
            if let Some(op) = ExprOp::try_parse(&mut rest2)? {
                if let Some(new_prec) = op.binary_precedence() {
                    if prec.parse_rhs_first(new_prec) {
                        WsAndComments::try_parse(&mut rest2)?;
                        let (rest_, rhs) = Expr::parse_raw_with_prec(&rest2, new_prec)?;
                        rest = rest_;
                        lhs = Expr::BinaryOp(Box::new(BinaryOp { op, lhs, rhs }));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok((rest, Some(lhs)))
    }

    pub fn parse_raw_with_prec(input: &Span, prec: Precedence) -> ParseRawRes<Self> {
        match Self::try_parse_raw_with_prec(input, prec)? {
            (rest, Some(parsed)) => Ok((rest, parsed)),
            _ => Err(ParseErr::new(input.start(), "expected expression")),
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
            Self::BinaryOp(e) => e.span(),
        }
    }
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(input: &super::Span) -> ParseRawRes<Option<Self>> {
        Expr::try_parse_raw_with_prec(input, Precedence::max())
    }
}

#[derive(Debug)]
pub struct BinaryOp {
    op: ExprOp,
    lhs: Expr,
    rhs: Expr,
}

impl GetSpan for BinaryOp {
    fn span(&self) -> Span {
        self.lhs.span().join(&self.rhs.span())
    }
}

#[derive(Default)]
pub struct CallArgs {
    span: Span,
    args: Vec<Expr>,
}

impl From<CallArgs> for Vec<Expr> {
    fn from(value: CallArgs) -> Self {
        value.args
    }
}

impl GetSpan for CallArgs {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for CallArgs {
    fn desc() -> std::borrow::Cow<'static, str> {
        "arguments".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(args)) =
            Delimited::<Op<'('>, Option<Punctuated<Expr, Op![,]>>, Op<')'>>::try_parse_raw(input)?
        {
            let span = args.open.span.join(&args.close.span);
            let args = args.value.map(|v| v.into()).unwrap_or_default();
            Ok((rest, Some(Self { span, args })))
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
                        Expr::try_parse_raw_with_prec(&rest, Precedence::right_to_left(2))?
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
            if let Some(args) = CallArgs::try_parse(&mut rest)? {
                let span = args.span();
                return Ok((
                    rest,
                    Some(Self {
                        span,
                        ident,
                        args: args.into(),
                    }),
                ));
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
                    if let (rest, Some(close_paren)) = Op::<')'>::try_parse_raw(&rest)? {
                        return Ok((rest, Some(Self::Type(kw.span.join(&close_paren.span), ty))));
                    }
                }
            }
            if let (rest, Some(expr)) =
                Expr::try_parse_raw_with_prec(&rest, Precedence::right_to_left(2))?
            {
                return Ok((rest, Some(Self::Expr(kw.span.join(&expr.span()), expr))));
            }
            Err(ParseErr::new(kw.span, "missing operand for `sizeof`"))
        } else {
            Ok((input.clone(), None))
        }
    }
}
