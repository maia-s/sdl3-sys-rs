use super::{
    CallArgs, Delimited, Expr, GetSpan, Ident, Op, Parse, ParseContext, Span, WsAndComments,
};

pub const ATTR_ABI: usize = 0;
pub const ATTR_ARG: usize = 1;
pub const ATTR_FN: usize = 2;

pub type FnAbi = Attribute<ATTR_ABI>;
pub type ArgAttribute = Attribute<ATTR_ARG>;
//pub type FnAttribute = Attribute<ATTR_FN>;

pub type FnAttributes = Attributes<ATTR_FN>;

#[derive(Clone, Debug)]
pub struct Attribute<const KIND: usize> {
    pub ident: Ident,
    pub args: Vec<Expr>,
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
    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> super::ParseRawRes<Option<Self>> {
        let args = Vec::new();
        if let (rest, Some(ident)) = Ident::try_parse_raw(ctx, input)? {
            match KIND {
                ATTR_ABI => match ident.as_str() {
                    "__cdecl" | "APIENTRY" | "APIENTRYP" | "EGLAPIENTRY" | "EGLAPIENTRYP"
                    | "GLAPIENTRY" | "GL_APIENTRY" | "GL_APIENTRYP" | "SDLCALL" | "WINAPI" => {
                        return Ok((rest, Some(Self { ident, args })))
                    }

                    "__attribute__" => {
                        let (rest, args) =
                            Delimited::<Op<'('>, CallArgs, Op<')'>>::parse_raw(ctx, &rest)?;
                        return Ok((
                            rest,
                            Some(Self {
                                ident,
                                args: args.value.into(),
                            }),
                        ));
                    }

                    _ => (),
                },

                ATTR_ARG => match ident.as_str() {
                    "SDL_PRINTF_FORMAT_STRING" | "SDL_SCANF_FORMAT_STRING" | "SDL_UNUSED" => {
                        return Ok((rest, Some(Self { ident, args })))
                    }

                    "SDL_IN_BYTECAP" | "SDL_OUT_BYTECAP" | "SDL_INOUT_Z_CAP" | "SDL_OUT_Z_CAP" => {
                        let (rest, args) = CallArgs::parse_raw(ctx, &rest)?;
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

                ATTR_FN => match ident.as_str() {
                    "EGLAPI"
                    | "GLAPI"
                    | "GL_APICALL"
                    | "SDL_ANALYZER_NORETURN"
                    | "SDL_DECLSPEC"
                    | "SDL_DEPRECATED"
                    | "SDL_FORCE_INLINE"
                    | "SDL_MALLOC"
                    | "SDLMAIN_DECLSPEC"
                    | "__inline"
                    | "__inline__" => return Ok((rest, Some(Self { ident, args }))),

                    "SDL_ACQUIRE"
                    | "SDL_ACQUIRE_SHARED"
                    | "SDL_ALLOC_SIZE"
                    | "SDL_ALLOC_SIZE2"
                    | "SDL_PRINTF_VARARG_FUNC"
                    | "SDL_PRINTF_VARARG_FUNCV"
                    | "SDL_RELEASE"
                    | "SDL_RELEASE_GENERIC"
                    | "SDL_SCANF_VARARG_FUNC"
                    | "SDL_SCANF_VARARG_FUNCV"
                    | "SDL_TRY_ACQUIRE"
                    | "SDL_TRY_ACQUIRE_SHARED"
                    | "SDL_WPRINTF_VARARG_FUNC"
                    | "SDL_WPRINTF_VARARG_FUNCV" => {
                        let (rest, args) = CallArgs::parse_raw(ctx, &rest)?;
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

#[derive(Clone, Debug)]
pub struct Attributes<const KIND: usize>(pub Vec<Attribute<KIND>>);

impl<const KIND: usize> Attributes<KIND> {
    pub fn contains<T: ?Sized>(&self, attr: &T) -> bool
    where
        Ident: PartialEq<T>,
    {
        for i in self.0.iter() {
            if i.ident.eq(attr) {
                return true;
            }
        }
        false
    }
}

impl<const KIND: usize> Parse for Attributes<KIND> {
    fn desc() -> std::borrow::Cow<'static, str> {
        Attribute::<KIND>::desc()
    }

    fn parse_raw(ctx: &ParseContext, input: &Span) -> super::ParseRawRes<Self> {
        let mut rest = input.clone();
        let mut vec = Vec::new();
        while let Some(attr) = Attribute::<KIND>::try_parse(ctx, &mut rest)? {
            vec.push(attr);
            WsAndComments::try_parse(ctx, &mut rest)?;
        }
        Ok((rest, Self(vec)))
    }
}
