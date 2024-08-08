use super::{
    Balanced, Delimited, ExprOp, GetSpan, Ident, IdentOrKw, Kw_sizeof, Literal, Op, Parse,
    ParseErr, ParseRawRes, Precedence, Punctuated, Span, Type, WsAndComments
};

#[derive(Clone, Debug)]
pub enum Expr {
    Ident(IdentOrKw),
    Literal(Literal),
    FnCall(FnCall),
    Ambiguous(Ambiguous),
    Cast(Box<Cast>),
    Asm(Asm),
    SizeOf(Box<SizeOf>),
    UnaryOp(Box<UnaryOp>),
    BinaryOp(Box<BinaryOp>),
    PostOp(Box<UnaryOp>),
    Ternary(Box<Ternary>),
    ArrayIndex {
        span: Span,
        base: Box<Expr>,
        index: Box<Expr>,
    },

    // only created by VarDecl
    ArrayValues {
        span: Span,
        values: Vec<Expr>
    },
}

impl Expr {
    pub fn try_parse_raw_with_prec(
        input: &Span,
        prec: Precedence,
        allow_cast: bool,
    ) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let mut is_cast = false;
        let mut lhs = if let Some((uprec, op)) = ExprOp::try_parse_unop(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let (rest_, expr) = Expr::parse_raw_with_prec(&rest, uprec, true)?;
            rest = rest_;
            Self::UnaryOp(Box::new(UnaryOp { op, expr }))
        } else if let (true, Some(cast)) = (allow_cast, Cast::try_parse(&mut rest)?) {
            if prec.parse_rhs_first(Precedence::right_to_left(2)) {
                is_cast = true;
                Self::Cast(Box::new(cast))
            } else if let (rest, Some(not_cast)) =
                Expr::try_parse_raw_with_prec(input, prec, false)?
            {
                // not valid as a cast here, but valid as a non-cast expression
                // (this isn't exactly right in general, but works for now)
                return Ok((rest, Some(not_cast)));
            } else {
                return Ok((input.clone(), None));
            }
        } else if let Some(asm) = Asm::try_parse(&mut rest)? {
            Self::Asm(asm)
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
            if rest2.starts_with("/**<") {
                // trailing doc comment after expression
                break;
            }

            if let Some((new_prec, op)) = ExprOp::try_parse_binop(&mut rest2)? {
                if prec.parse_rhs_first(new_prec) {
                    WsAndComments::try_parse(&mut rest2)?;
                    match op.as_str().as_bytes() {
                        b"++" | b"--" => {
                            rest = rest2;
                            lhs = Expr::PostOp(Box::new(UnaryOp {
                                op,
                                expr: lhs,
                            }))
                        }

                        b"(" => {
                            let (rest_, args) = CallArgs::parse_raw(&op.span.join(&rest2))?;
                            rest = rest_;
                            lhs = Expr::FnCall(FnCall {
                                span: lhs.span().join(&args.span()),
                                func: Box::new(lhs),
                                args: args.into(),
                            })
                        }

                        b"[" => {
                            let index = Expr::parse(&mut rest2)?;
                            WsAndComments::try_parse(&mut rest2)?;
                            let close = Op::<']'>::parse(&mut rest2)?;
                            rest = rest2;
                            lhs = Expr::ArrayIndex {
                                span: lhs.span().join(&close.span),
                                base: Box::new(lhs),
                                index: Box::new(index),
                            }
                        }

                        b"?" => {
                            let on_true = Expr::parse(&mut rest2)?;
                            WsAndComments::try_parse(&mut rest2)?;
                            Op::<':'>::parse(&mut rest2)?;
                            WsAndComments::try_parse(&mut rest2)?;
                            let (rest_, on_false) =
                                Expr::parse_raw_with_prec(&rest2, new_prec, true)?;
                            rest = rest_;
                            lhs = Expr::Ternary(Box::new(Ternary {
                                cond: lhs,
                                on_true,
                                on_false,
                            }));
                        }

                        _ => {
                            if let (rest_, Some(rhs)) = Expr::try_parse_raw_with_prec(&rest2, new_prec, true)? {
                                rest = rest_;
                                lhs = Expr::BinaryOp(Box::new(BinaryOp { op, lhs, rhs }));
                            } else {
                                return Ok((input.clone(),None))
                            }
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if is_cast {
            if let (not_cast_rest, Some(not_cast)) =
                Expr::try_parse_raw_with_prec(input, prec, false)?
            {
                if rest.start_pos() != not_cast_rest.start_pos() {
                    return Err(ParseErr::new(
                        lhs.span(), 
                        "expression is valid both as a cast and as a non-cast expression, but parse lengths differ"
                    ));
                }
                lhs = Expr::Ambiguous(Ambiguous {
                    span: lhs.span().join(&not_cast.span()),
                    alternatives: vec![lhs, not_cast],
                });
            }
        }

        Ok((rest, Some(lhs)))
    }

    pub fn parse_raw_with_prec(
        input: &Span,
        prec: Precedence,
        allow_cast: bool,
    ) -> ParseRawRes<Self> {
        match Self::try_parse_raw_with_prec(input, prec, allow_cast)? {
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
            Self::Ambiguous(e) => e.span(),
            Self::Asm(e) => e.span(),
            Self::SizeOf(e) => e.span(),
            Self::UnaryOp(e) | Self::PostOp(e) => e.span(),
            Self::BinaryOp(e) => e.span(),
            Self::Ternary(e) => e.span(),
            Self::ArrayIndex { span, .. } => span.clone(),
            Self::ArrayValues { span, .. } => span.clone(),
        }
    }
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(input: &super::Span) -> ParseRawRes<Option<Self>> {
        Expr::try_parse_raw_with_prec(input, Precedence::max(), true)
    }
}

pub struct ExprNoComma(pub Expr);

impl Parse for ExprNoComma {
    fn desc() -> std::borrow::Cow<'static, str> {
        Expr::desc()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, expr) = Expr::try_parse_raw_with_prec(input, Precedence::comma(), true)?;
        Ok((rest, expr.map(Self)))
    }
}

#[derive(Clone, Debug)]
pub struct Asm {
    pub span: Span,
    pub kind: Span,
}

impl GetSpan for Asm {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Asm {
    fn desc() -> std::borrow::Cow<'static, str> {
        "asm".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(ident)) = Ident::try_parse_raw(input)? {
            match ident.as_str().as_bytes() {
                b"__asm__" => {
                    WsAndComments::try_parse(&mut rest)?;
                    if let (rest_, Some(_)) = Ident::try_parse_raw_if(&rest, |i| i.as_str() == "__volatile__")? {
                        rest = rest_;
                        WsAndComments::try_parse(&mut rest)?;
                    }
                    let body = Balanced::<Op<'('>, Op<')'>>::parse(&mut rest)?;
                    return Ok((rest, Some(Self { span: ident.span.join(&body.span), kind: ident.span() })))
                }

                b"_asm" => {
                    WsAndComments::try_parse(&mut rest)?;
                    let body = Balanced::<Op<'{'>, Op<'}'>>::parse(&mut rest)?;
                    return Ok((rest, Some(Self { span: ident.span.join(&body.span), kind: ident.span() })))
                }

                _ => ()
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone, Debug)]
pub struct UnaryOp {
    op: ExprOp,
    expr: Expr,
}

impl GetSpan for UnaryOp {
    fn span(&self) -> Span {
        self.op.span().join(&self.expr.span())
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Ternary {
    cond: Expr,
    on_true: Expr,
    on_false: Expr,
}

impl GetSpan for Ternary {
    fn span(&self) -> Span {
        self.cond.span().join(&self.on_false.span())
    }
}

#[derive(Clone, Debug)]
pub struct Ambiguous {
    pub span: Span,
    pub alternatives: Vec<Expr>
}

impl GetSpan for Ambiguous {
    fn span(&self) -> Span {
        self.span.clone()
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
            Delimited::<Op<'('>, Option<Punctuated<ExprNoComma, Op![,]>>, Op<')'>>::try_parse_raw(input)?
        {
            let span = args.open.span.join(&args.close.span);
            let args: Vec<ExprNoComma> = args.value.map(|v| v.into()).unwrap_or_default();
            let args = args.into_iter().map(|v| v.0).collect();
            Ok((rest, Some(Self { span, args })))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
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
                        Expr::try_parse_raw_with_prec(&rest, Precedence::right_to_left(2), true)?
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

#[derive(Clone, Debug)]
pub struct FnCall {
    span: Span,
    func: Box<Expr>,
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
        // this only parses ident(), not general expr()
        let mut rest = input.trim_wsc_start()?;
        if let Some(func) = IdentOrKw::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            if let Some(args) = CallArgs::try_parse(&mut rest)? {
                let span = func.span().join(&args.span());
                return Ok((
                    rest,
                    Some(Self {
                        span,
                        func: Box::new(Expr::Ident(func)),
                        args: args.into(),
                    }),
                ));
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone, Debug)]
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
                Expr::try_parse_raw_with_prec(&rest, Precedence::right_to_left(2), true)?
            {
                return Ok((rest, Some(Self::Expr(kw.span.join(&expr.span()), expr))));
            }
            Err(ParseErr::new(kw.span, "missing operand for `sizeof`"))
        } else {
            Ok((input.clone(), None))
        }
    }
}
