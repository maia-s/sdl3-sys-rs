use super::{
    patch_parsed_expr, Balanced, Delimited, ExprOp, GetSpan, Ident, IdentOrKw, Items, Kw_sizeof, Literal, Op, Parse, ParseContext, ParseErr, ParseRawRes, Precedence, Punctuated, Span, Type, WsAndComments
};
use crate::emit::Value;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Expr {
    Parenthesized(Box<Parenthesized>),
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
    HasInclude(HasInclude),

    // only created by VarDecl
    ArrayValues {
        span: Span,
        values: Vec<Expr>
    },

    // only created by the emitter
    Value(Value),
}

impl Expr {
    pub fn try_parse_raw_with_prec(ctx: &ParseContext, 
        input: &Span,
        prec: Precedence,
        allow_cast: bool,
    ) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let mut is_cast = false;
        let mut lhs = if let Some((uprec, op)) = ExprOp::try_parse_unop(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let (rest_, expr) = Expr::parse_raw_with_prec(ctx, &rest, uprec, true)?;
            rest = rest_;
            Self::UnaryOp(Box::new(UnaryOp { op, expr }))
        } else if let Some(cast) = Cast::try_parse_if(ctx, &mut rest, |_| allow_cast)? {
            if prec.parse_rhs_first(Precedence::right_to_left(2)) {
                is_cast = true;
                Self::Cast(Box::new(cast))
            } else if let (rest, Some(not_cast)) =
                Expr::try_parse_raw_with_prec(ctx, input, prec, false)?
            {
                // not valid as a cast here, but valid as a non-cast expression
                // (this isn't exactly right in general, but works for now)
                return Ok((rest, Some(not_cast)));
            } else {
                return Ok((input.clone(), None));
            }
        } else if let Some(asm) = Asm::try_parse(ctx, &mut rest)? {
            Self::Asm(asm)
        } else if let Some(sizeof) = SizeOf::try_parse(ctx, &mut rest)? {
            if prec.parse_rhs_first(Precedence::right_to_left(2)) {
                Self::SizeOf(Box::new(sizeof))
            } else {
                return Ok((input.clone(), None));
            }
        } else if let Some(p) = Parenthesized::try_parse(ctx, &mut rest)?
        {
            Expr::Parenthesized(Box::new(p))
        } else if let Some(ident) = IdentOrKw::try_parse(ctx, &mut rest)? {
            Self::Ident(ident)
        } else if let Some(lit) = Literal::try_parse(ctx, &mut rest)? {
            Self::Literal(lit)
        } else {
            return Ok((input.clone(), None));
        };

        loop {
            let mut rest2 = rest.clone();
            WsAndComments::try_parse(ctx, &mut rest2)?;
            if rest2.starts_with("/**<") {
                // trailing doc comment after expression
                break;
            }

            if let Some((new_prec, op)) = ExprOp::try_parse_binop(ctx, &mut rest2)? {
                if prec.parse_rhs_first(new_prec) {
                    WsAndComments::try_parse(ctx, &mut rest2)?;
                    match op.as_str().as_bytes() {
                        b"++" | b"--" => {
                            rest = rest2;
                            lhs = Expr::PostOp(Box::new(UnaryOp {
                                op,
                                expr: lhs,
                            }))
                        }

                        b"(" => {
                            let rest_ = op.span.join(&rest2);

                            if let Expr::Ident(ident) = &lhs {
                                if ident.as_str() == "__has_include" {
                                    let (rest_, arg) = Balanced::<Op::<'('>,Op::<')'>>::parse_raw(ctx, &rest_)?;
                                    rest = rest_;
                                    lhs = Expr::HasInclude(HasInclude { span: lhs.span().join(&arg.span), include: arg.inner });
                                    continue;
                                }
                            }

                            let (rest_, args) = CallArgs::parse_raw(ctx, &op.span.join(&rest_))?;
                            rest = rest_;
                            lhs = Expr::FnCall(FnCall {
                                span: lhs.span().join(&args.span()),
                                func: Box::new(lhs),
                                args: args.into(),
                            });
                            patch_parsed_expr(ctx, &mut lhs)?;
                        }

                        b"[" => {
                            let index = Expr::parse(ctx, &mut rest2)?;
                            WsAndComments::try_parse(ctx, &mut rest2)?;
                            let close = Op::<']'>::parse(ctx, &mut rest2)?;
                            rest = rest2;
                            lhs = Expr::ArrayIndex {
                                span: lhs.span().join(&close.span),
                                base: Box::new(lhs),
                                index: Box::new(index),
                            }
                        }

                        b"?" => {
                            let on_true = Expr::parse(ctx, &mut rest2)?;
                            WsAndComments::try_parse(ctx, &mut rest2)?;
                            Op::<':'>::parse(ctx, &mut rest2)?;
                            WsAndComments::try_parse(ctx, &mut rest2)?;
                            let (rest_, on_false) =
                                Expr::parse_raw_with_prec(ctx, &rest2, new_prec, true)?;
                            rest = rest_;
                            lhs = Expr::Ternary(Box::new(Ternary {
                                cond: lhs,
                                on_true,
                                on_false,
                            }));
                        }

                        _ => {
                            if let (rest_, Some(rhs)) = Expr::try_parse_raw_with_prec(ctx, &rest2, new_prec, true)? {
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
                Expr::try_parse_raw_with_prec(ctx, input, prec, false)?
            {
                match rest.start_pos().cmp(&not_cast_rest.start_pos()) {
                    Ordering::Greater => {
                        // keep the cast
                    }
                    Ordering::Less => {
                        return Err(ParseErr::new(
                            lhs.span(), 
                            "expression is valid both as a cast and as a non-cast expression, but expression is longer"
                        ));
                    }
                    Ordering::Equal => {
                        let mut ambiguous = Ambiguous::new(lhs.span().join(&not_cast.span()));
                        ambiguous.push_expr(lhs);
                        ambiguous.push_expr(not_cast);
                        lhs = Expr::Ambiguous(ambiguous);
                    }
                }
            }
        }

        Ok((rest, Some(lhs)))
    }

    pub fn parse_raw_with_prec(ctx: &ParseContext, 
        input: &Span,
        prec: Precedence,
        allow_cast: bool,
    ) -> ParseRawRes<Self> {
        match Self::try_parse_raw_with_prec(ctx, input, prec, allow_cast)? {
            (rest, Some(parsed)) => Ok((rest, parsed)),
            _ => Err(ParseErr::new(input.start(), "expected expression")),
        }
    }

    pub fn deparenthesize(&self) -> &Self {
        let mut expr = self;
        while let Expr::Parenthesized(e) = &expr {
            expr = &e.expr;
        }
        expr
    }
}

impl GetSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Parenthesized(e) => e.span(),
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
            Self::HasInclude(e) => e.span(),
            Self::ArrayIndex { span, .. } => span.clone(),
            Self::ArrayValues { span, .. } => span.clone(),
            Self::Value(_) => Span::none(),
        }
    }
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &super::Span) -> ParseRawRes<Option<Self>> {
        Expr::try_parse_raw_with_prec(ctx, input, Precedence::max(), true)
    }
}

pub struct ExprNoComma(pub Expr);

impl Parse for ExprNoComma {
    fn desc() -> std::borrow::Cow<'static, str> {
        Expr::desc()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, expr) = Expr::try_parse_raw_with_prec(ctx, input, Precedence::comma(), true)?;
        Ok((rest, expr.map(Self)))
    }
}

#[derive(Debug, Clone)]
pub struct HasInclude {
    pub span: Span,
    pub include: Span,
}

impl GetSpan for HasInclude {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone,Debug)]
pub struct Parenthesized {
    pub span: Span,
    pub expr: Expr,
}

impl GetSpan for Parenthesized {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Parenthesized {
    fn desc() -> std::borrow::Cow<'static, str> {
        "parenthesized expression".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(Delimited {
            open,
            value: expr,
            close,
        })) = Delimited::<Op<'('>, Expr, Op<')'>>::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Self {
                span: open.span.join(&close.span),
                expr,
            })))
        } else {
            Ok((input.clone(), None))
        }
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(ident)) = Ident::try_parse_raw(ctx, input)? {
            match ident.as_str().as_bytes() {
                b"__asm__" => {
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    if let (rest_, Some(_)) = Ident::try_parse_raw_if(ctx, &rest, |i| i.as_str() == "__volatile__")? {
                        rest = rest_;
                        WsAndComments::try_parse(ctx, &mut rest)?;
                    }
                    let body = Balanced::<Op<'('>, Op<')'>>::parse(ctx, &mut rest)?;
                    return Ok((rest, Some(Self { span: ident.span.join(&body.span), kind: ident.span() })))
                }

                b"_asm" => {
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    let body = Balanced::<Op<'{'>, Op<'}'>>::parse(ctx, &mut rest)?;
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
    pub op: ExprOp,
    pub expr: Expr,
}

impl GetSpan for UnaryOp {
    fn span(&self) -> Span {
        self.op.span().join(&self.expr.span())
    }
}

#[derive(Clone, Debug)]
pub struct BinaryOp {
    pub op: ExprOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

impl GetSpan for BinaryOp {
    fn span(&self) -> Span {
        self.lhs.span().join(&self.rhs.span())
    }
}

#[derive(Clone, Debug)]
pub struct Ternary {
    pub cond: Expr,
    pub on_true: Expr,
    pub on_false: Expr,
}

impl GetSpan for Ternary {
    fn span(&self) -> Span {
        self.cond.span().join(&self.on_false.span())
    }
}

#[derive(Clone, Debug)]
pub struct Ambiguous {
    pub span: Span,
    pub alternatives: Vec<Alternative>,
}

#[derive(Clone, Debug)]
pub enum Alternative {
    Expr(Expr),
    Type(Type),
    Items(Items),
}

impl Ambiguous {
    pub fn new(span: Span) -> Self {
        Self { span, alternatives:Vec::new() }
    }

    pub fn push_expr(&mut self, expr: Expr) {
        self.alternatives.push(Alternative::Expr(expr));
    }

    pub fn push_ty(&mut self, ty: Type) {
        self.alternatives.push(Alternative::Type(ty));
    }

    pub fn push_items(&mut self, items: Items) {
        self.alternatives.push(Alternative::Items(items));
    }
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(args)) =
            Delimited::<Op<'('>, Option<Punctuated<ExprNoComma, Op![,]>>, Op<')'>>::try_parse_raw(ctx, input)?
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
    pub span: Span,
    pub ty: Type,
    pub expr: Expr,
}

impl Cast {
    pub fn boxed(span: Span, ty: Type, expr: Expr) -> Box<Self> {
        Box::new(Self {span, ty, expr })
    }
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(open_paren)) = Op::<'('>::try_parse_raw(ctx, input)? {
            // this may just be a parenthesized expression, so don't error if something fails to parse
            let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
            if let (rest, Some(ty)) = Type::try_parse_raw(ctx, &rest)? {
                let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
                if let (rest, Some(_close_paren)) = Op::<')'>::try_parse_raw(ctx, &rest)? {
                    let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
                    if let (rest, Some(expr)) =
                        Expr::try_parse_raw_with_prec(ctx, &rest, Precedence::right_to_left(2), true)?
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
    pub span: Span,
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> super::ParseRawRes<Option<Self>> {
        // this only parses ident(), not general expr()
        let mut rest = input.trim_wsc_start()?;
        if let Some(func) = IdentOrKw::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            if let Some(args) = CallArgs::try_parse(ctx, &mut rest)? {
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(kw)) = Kw_sizeof::try_parse_raw(ctx, input)? {
            let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
            if let (rest, Some(_)) = Op::<'('>::try_parse_raw(ctx, &rest)? {
                // could be a parenthesized expression
                let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
                if let (rest, Some(ty)) = Type::try_parse_raw(ctx, &rest)? {
                    let (rest, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
                    if let (rest, Some(close_paren)) = Op::<')'>::try_parse_raw(ctx, &rest)? {
                        return Ok((rest, Some(Self::Type(kw.span.join(&close_paren.span), ty))));
                    }
                }
            }
            if let (rest, Some(expr)) =
                Expr::try_parse_raw_with_prec(ctx, &rest, Precedence::right_to_left(2), true)?
            {
                return Ok((rest, Some(Self::Expr(kw.span.join(&expr.span()), expr))));
            }
            Err(ParseErr::new(kw.span, "missing operand for `sizeof`"))
        } else {
            Ok((input.clone(), None))
        }
    }
}
