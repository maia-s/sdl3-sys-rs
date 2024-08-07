use std::borrow::Cow;

use super::{
    GetSpan, IdentOrKw, Kw_const, Parse, ParseErr, ParseRawRes, Span, Spanned, WsAndComments,
};

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    Void,
    Bool,
    SizeT,
    Int8T,
    Uint8T,
    Int16T,
    Uint16T,
    Int32T,
    Uint32T,
    Int64T,
    Uint64T,
}

pub struct PrimitiveTypeParse {
    pub span: Span,
    pub ty: PrimitiveType,
    pub is_const: bool,
}

impl Parse for PrimitiveTypeParse {
    fn desc() -> Cow<'static, str> {
        "primitive type".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, is_const) = Kw_const::try_parse_raw(input)?;
        let (rest, _) = WsAndComments::try_parse_raw(&rest)?;
        let mut is_const = is_const.is_some();
        if let (
            rest,
            Some(Spanned {
                span,
                value: combine,
            }),
        ) = Spanned::<Vec<PrimitiveCombineKw>>::try_parse_raw(&rest)?
        {
            macro_rules! query {
                ($kw:pat) => {
                    combine.iter().filter(move |i| matches!(**i, $kw)).count()
                };
            }
            let n_const = query!(PrimitiveCombineKw::Const(_));
            let n_signed = query!(PrimitiveCombineKw::Signed(_));
            let n_unsigned = query!(PrimitiveCombineKw::Unsigned(_));
            let n_char = query!(PrimitiveCombineKw::Char(_));
            let n_short = query!(PrimitiveCombineKw::Short(_));
            let n_int = query!(PrimitiveCombineKw::Int(_));
            let n_long = query!(PrimitiveCombineKw::Long(_));

            if n_const > 1
                || (is_const && n_const > 0)
                || n_signed > 1
                || n_unsigned > 1
                || n_char > 1
                || n_short > 1
                || n_int > 1
                || n_long > 2
            {
                return Err(ParseErr::new(span, "too many keywords in primitive type"));
            }

            is_const = n_const > 0;

            if (n_signed > 0 && n_unsigned > 0)
                || (n_short > 0 && n_long > 0)
                || (n_char > 0 && (n_int > 0 || n_short > 0 || n_long > 0))
            {
                return Err(ParseErr::new(
                    span,
                    "conflicting keywords in primitive type",
                ));
            }

            return Ok((
                rest,
                Some(Self {
                    span,
                    is_const,
                    ty: if n_int > 0 {
                        if n_short > 0 {
                            if n_unsigned > 0 {
                                PrimitiveType::UnsignedShort
                            } else {
                                PrimitiveType::Short
                            }
                        } else if n_long > 1 {
                            if n_unsigned > 0 {
                                PrimitiveType::UnsignedLongLong
                            } else {
                                PrimitiveType::LongLong
                            }
                        } else if n_long > 0 {
                            if n_unsigned > 0 {
                                PrimitiveType::UnsignedLong
                            } else {
                                PrimitiveType::Long
                            }
                        } else if n_unsigned > 0 {
                            PrimitiveType::UnsignedInt
                        } else {
                            PrimitiveType::Int
                        }
                    } else if n_short > 0 {
                        if n_unsigned > 0 {
                            PrimitiveType::UnsignedShort
                        } else {
                            PrimitiveType::Short
                        }
                    } else if n_long > 1 {
                        if n_unsigned > 0 {
                            PrimitiveType::UnsignedLongLong
                        } else {
                            PrimitiveType::LongLong
                        }
                    } else if n_long > 0 {
                        if n_unsigned > 0 {
                            PrimitiveType::UnsignedLong
                        } else {
                            PrimitiveType::Long
                        }
                    } else if n_char > 0 {
                        if n_unsigned > 0 {
                            PrimitiveType::UnsignedChar
                        } else if n_signed > 0 {
                            PrimitiveType::SignedChar
                        } else {
                            PrimitiveType::Char
                        }
                    } else if n_unsigned > 0 {
                        PrimitiveType::UnsignedInt
                    } else if n_signed > 0 {
                        PrimitiveType::Int
                    } else {
                        unreachable!()
                    },
                }),
            ));
        } else if let (mut rest, Some(ident)) = IdentOrKw::try_parse_raw(&rest)? {
            let mut span = ident.span();
            if !is_const {
                if let (rest_, Some(const_kw)) = Kw_const::try_parse_raw(&rest)? {
                    rest = rest_;
                    span = span.join(&const_kw.span());
                    is_const = true;
                }
            }
            return Ok((
                rest,
                Some(Self {
                    span,
                    is_const,
                    ty: match ident.span.as_str() {
                        "float" => PrimitiveType::Float,
                        "double" => PrimitiveType::Double,
                        "void" => PrimitiveType::Void,
                        "_Bool" => PrimitiveType::Bool,
                        "size_t" => PrimitiveType::SizeT,
                        "int8_t" => PrimitiveType::Int8T,
                        "uint8_t" => PrimitiveType::Uint8T,
                        "int16_t" => PrimitiveType::Int16T,
                        "uint16_t" => PrimitiveType::Uint16T,
                        "int32_t" => PrimitiveType::Int32T,
                        "uint32_t" => PrimitiveType::Uint32T,
                        "int64_t" => PrimitiveType::Int64T,
                        "uint64_t" => PrimitiveType::Uint64T,
                        _ => return Ok((input.clone(), None)),
                    },
                }),
            ));
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone)]
enum PrimitiveCombineKw {
    Const(Span),
    Signed(Span),
    Unsigned(Span),
    Char(Span),
    Short(Span),
    Int(Span),
    Long(Span),
}

impl GetSpan for PrimitiveCombineKw {
    fn span(&self) -> Span {
        match self {
            Self::Const(span)
            | Self::Signed(span)
            | Self::Unsigned(span)
            | Self::Char(span)
            | Self::Short(span)
            | Self::Int(span)
            | Self::Long(span) => span.clone(),
        }
    }
}

impl Parse for PrimitiveCombineKw {
    fn desc() -> Cow<'static, str> {
        "`signed`, `unsigned`, `char`, `short`, `int` or `long`".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        WsAndComments::try_parse(&mut rest)?;
        if let Some(i) = IdentOrKw::try_parse(&mut rest)? {
            match i.span.as_str() {
                "const" => return Ok((rest, Some(PrimitiveCombineKw::Const(i.span)))),
                "signed" => return Ok((rest, Some(PrimitiveCombineKw::Signed(i.span)))),
                "unsigned" => return Ok((rest, Some(PrimitiveCombineKw::Unsigned(i.span)))),
                "char" => return Ok((rest, Some(PrimitiveCombineKw::Char(i.span)))),
                "short" => return Ok((rest, Some(PrimitiveCombineKw::Short(i.span)))),
                "int" => return Ok((rest, Some(PrimitiveCombineKw::Int(i.span)))),
                "long" => return Ok((rest, Some(PrimitiveCombineKw::Long(i.span)))),
                _ => (),
            }
        }
        Ok((input.clone(), None))
    }
}
