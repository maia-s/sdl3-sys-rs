use super::{
    Delimited, Ident, IdentOrKw, Literal, Op, Parse, ParseRawRes, Punctuated, Span, WsAndComments,
};

pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    FnCall(FnCall),
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(input: &super::Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(call)) = FnCall::try_parse_raw(input)? {
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

pub struct FnCall {
    span: Span,
    ident: IdentOrKw,
    args: Vec<Expr>,
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
