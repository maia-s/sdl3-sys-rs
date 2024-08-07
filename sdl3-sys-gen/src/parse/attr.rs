use super::{CallArgs, Expr, GetSpan, Ident, Parse, Span};

pub const ATTR_ARG: usize = 0;
pub const ATTR_FN: usize = 1;

pub type ArgAttribute = Attribute<ATTR_ARG>;
pub type FnAttribute = Attribute<ATTR_FN>;

#[derive(Debug)]
pub struct Attribute<const KIND: usize> {
    ident: Ident,
    args: Vec<Expr>,
}

impl<const KIND: usize> GetSpan for Attribute<KIND> {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl<const KIND: usize> Parse for Attribute<KIND> {
    fn desc() -> std::borrow::Cow<'static, str> {
        "attribute".into()
    }

    #[allow(clippy::single_match)]
    fn try_parse_raw(input: &Span) -> super::ParseRawRes<Option<Self>> {
        if let (rest, Some(ident)) = Ident::try_parse_raw(input)? {
            match KIND {
                ATTR_ARG => match ident.as_str() {
                    "SDL_PRINTF_FORMAT_STRING" => {
                        return Ok((
                            rest,
                            Some(Self {
                                ident,
                                args: Vec::new(),
                            }),
                        ))
                    }

                    _ => (),
                },

                ATTR_FN => match ident.as_str() {
                    "SDL_PRINTF_VARARG_FUNC" => {
                        let (rest, args) = CallArgs::parse_raw(&rest)?;
                        return Ok((
                            rest,
                            Some(Self {
                                ident,
                                args: args.into(),
                            }),
                        ));
                    }

                    _ => (),
                },

                _ => unreachable!(),
            }
        }

        Ok((input.clone(), None))
    }
}
