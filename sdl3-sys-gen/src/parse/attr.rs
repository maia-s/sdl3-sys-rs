use super::{CallArgs, Expr, GetSpan, Ident, Parse, Span, WsAndComments};

pub const ATTR_ABI: usize = 0;
pub const ATTR_ARG: usize = 1;
pub const ATTR_FN: usize = 2;

pub type FnAbi = Attribute<ATTR_ABI>;
pub type ArgAttribute = Attribute<ATTR_ARG>;
pub type FnAttribute = Attribute<ATTR_FN>;

pub type FnAttributes = Attributes<ATTR_FN>;

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
        match KIND {
            x if x == ATTR_ABI => "abi attribute",
            x if x == ATTR_ARG => "argument attribute",
            x if x == ATTR_FN => "function attribute",
            _ => "unknown attribute",
        }
        .into()
    }

    #[allow(clippy::single_match)]
    fn try_parse_raw(input: &Span) -> super::ParseRawRes<Option<Self>> {
        let args = Vec::new();
        if let (rest, Some(ident)) = Ident::try_parse_raw(input)? {
            match KIND {
                ATTR_ABI => match ident.as_str() {
                    "SDLCALL" => return Ok((rest, Some(Self { ident, args }))),

                    _ => (),
                },

                ATTR_ARG => match ident.as_str() {
                    "SDL_PRINTF_FORMAT_STRING" => return Ok((rest, Some(Self { ident, args }))),

                    _ => (),
                },

                ATTR_FN => match ident.as_str() {
                    "SDL_ALLOC_SIZE" => {
                        let (rest, args) = CallArgs::parse_raw(&rest)?;
                        return Ok((
                            rest,
                            Some(Self {
                                ident,
                                args: args.into(),
                            }),
                        ));
                    }

                    "SDL_DECLSPEC" => return Ok((rest, Some(Self { ident, args }))),

                    "SDL_FORCE_INLINE" => return Ok((rest, Some(Self { ident, args }))),

                    "SDL_MALLOC" => return Ok((rest, Some(Self { ident, args }))),

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

pub struct Attributes<const KIND: usize>(Vec<Attribute<KIND>>);

impl<const KIND: usize> Parse for Attributes<KIND> {
    fn desc() -> std::borrow::Cow<'static, str> {
        Attribute::<KIND>::desc()
    }

    fn parse_raw(input: &Span) -> super::ParseRawRes<Self> {
        let mut rest = input.clone();
        let mut vec = Vec::new();
        while let Some(attr) = Attribute::<KIND>::try_parse(&mut rest)? {
            vec.push(attr);
            WsAndComments::try_parse(&mut rest)?;
        }
        Ok((rest, Self(vec)))
    }
}

impl<const KIND: usize> From<Attributes<KIND>> for Vec<Attribute<KIND>> {
    fn from(value: Attributes<KIND>) -> Self {
        value.0
    }
}
