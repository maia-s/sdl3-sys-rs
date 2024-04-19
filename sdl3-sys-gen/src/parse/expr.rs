use super::{Ident, Literal, Parse};

pub enum Expr {
    Ident(Ident),
    Literal(Literal),
}

impl Parse for Expr {
    fn desc() -> std::borrow::Cow<'static, str> {
        "expression".into()
    }

    fn try_parse_raw(input: &super::Span) -> super::ParseRawRes<Option<Self>> {
        if let (rest, Some(ident)) = Ident::try_parse_raw(input)? {
            Ok((rest, Some(Self::Ident(ident))))
        } else if let (rest, Some(lit)) = Literal::try_parse_raw(input)? {
            Ok((rest, Some(Self::Literal(lit))))
        } else {
            Ok((input.clone(), None))
        }
    }
}
