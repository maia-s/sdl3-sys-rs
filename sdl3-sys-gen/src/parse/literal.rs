use super::{GetSpan, Op, Parse, ParseErr, ParseRawRes, Span, WsAndComments};
use std::{borrow::Cow, ffi::CString};

#[derive(Debug)]
pub enum Literal {
    Integer(IntegerLiteral),
    String(StringLiteral),
}

impl GetSpan for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Integer(l) => l.span(),
            Self::String(l) => l.span(),
        }
    }
}

impl Parse for Literal {
    fn desc() -> Cow<'static, str> {
        "literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(lit)) = IntegerLiteralT::try_parse_raw(input)? {
            Ok((rest, Some(Self::Integer(lit))))
        } else if let (rest, Some(lit)) = StringLiteral::try_parse_raw(input)? {
            Ok((rest, Some(Self::String(lit))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub const ALLOW_I32_LITERAL: u8 = 1 << 0;
pub const ALLOW_U32_LITERAL: u8 = 1 << 1;
pub const ALLOW_I64_LITERAL: u8 = 1 << 2;
pub const ALLOW_U64_LITERAL: u8 = 1 << 3;
pub const ALLOW_ANY_INT_LITERAL: u8 =
    ALLOW_I32_LITERAL | ALLOW_U32_LITERAL | ALLOW_I64_LITERAL | ALLOW_U64_LITERAL;

pub type IntegerLiteral = IntegerLiteralT<ALLOW_ANY_INT_LITERAL>;
pub type UintLiteral = IntegerLiteralT<{ ALLOW_U32_LITERAL | ALLOW_U64_LITERAL }>;

#[derive(Debug)]
pub enum IntegerLiteralT<const ALLOW: u8> {
    Int32(Span, i32),
    Uint31(Span, u32),
    Uint32(Span, u32),
    Int64(Span, i64),
    Uint63(Span, u64),
    Uint64(Span, u64),
}

impl<const ALLOW: u8> GetSpan for IntegerLiteralT<ALLOW> {
    fn span(&self) -> Span {
        match self {
            Self::Int32(span, _)
            | Self::Uint31(span, _)
            | Self::Uint32(span, _)
            | Self::Int64(span, _)
            | Self::Uint63(span, _)
            | Self::Uint64(span, _) => span.clone(),
        }
    }
}

impl<const ALLOW: u8> IntegerLiteralT<ALLOW> {
    pub fn zero(span: Span) -> Self {
        Self::Uint31(span, 0)
    }

    pub fn into_all(self) -> IntegerLiteral {
        match self {
            Self::Int32(span, value) => IntegerLiteral::Int32(span, value),
            Self::Uint31(span, value) => IntegerLiteral::Uint31(span, value),
            Self::Uint32(span, value) => IntegerLiteral::Uint32(span, value),
            Self::Int64(span, value) => IntegerLiteral::Int64(span, value),
            Self::Uint63(span, value) => IntegerLiteral::Uint63(span, value),
            Self::Uint64(span, value) => IntegerLiteral::Uint64(span, value),
        }
    }

    pub fn i32(&self) -> Result<i32, ParseErr> {
        match self {
            Self::Int32(_, value) => Ok(*value),
            Self::Uint31(_, value) => Ok(*value as i32),
            Self::Uint32(span, _)
            | Self::Int64(span, _)
            | Self::Uint63(span, _)
            | Self::Uint64(span, _) => {
                Err(ParseErr::new(span.clone(), "expected 32-bit signed int"))
            }
        }
    }

    pub fn u32(&self) -> Result<u32, ParseErr> {
        match self {
            Self::Uint31(_, value) | Self::Uint32(_, value) => Ok(*value),
            Self::Int32(span, _)
            | Self::Int64(span, _)
            | Self::Uint63(span, _)
            | Self::Uint64(span, _) => {
                Err(ParseErr::new(span.clone(), "expected 32-bit unsigned int"))
            }
        }
    }

    pub fn i64(&self) -> Result<i64, ParseErr> {
        match self {
            Self::Int32(_, value) => Ok(*value as i64),
            Self::Uint31(_, value) | Self::Uint32(_, value) => Ok(*value as i64),
            Self::Int64(_, value) => Ok(*value),
            Self::Uint63(_, value) => Ok(*value as i64),
            Self::Uint64(span, _) => Err(ParseErr::new(span.clone(), "expected 64-bit signed int")),
        }
    }

    pub fn u64(&self) -> Result<u64, ParseErr> {
        match self {
            Self::Uint31(_, value) | Self::Uint32(_, value) => Ok(*value as u64),
            Self::Uint63(_, value) | Self::Uint64(_, value) => Ok(*value),
            Self::Int32(span, _) | Self::Int64(span, _) => {
                Err(ParseErr::new(span.clone(), "expected 64-bit unsigned int"))
            }
        }
    }

    #[must_use]
    pub fn checked_add_1(&self) -> Option<Self> {
        match *self {
            Self::Int32(_, value) => {
                if value == -1 {
                    Some(Self::Uint31(Span::default(), 0))
                } else {
                    Some(Self::Int32(Span::default(), value + 1))
                }
            }
            Self::Uint31(_, value) => {
                if value == i32::MAX as u32 {
                    Some(Self::Uint32(Span::default(), value + 1))
                } else {
                    Some(Self::Uint31(Span::default(), value + 1))
                }
            }
            Self::Uint32(_, value) => {
                if value == u32::MAX {
                    Some(Self::Uint63(Span::default(), value as u64 + 1))
                } else {
                    Some(Self::Uint32(Span::default(), value + 1))
                }
            }
            Self::Int64(_, value) => {
                if value == -1 {
                    Some(Self::Uint31(Span::default(), 0))
                } else {
                    Some(Self::Int64(Span::default(), value + 1))
                }
            }
            Self::Uint63(_, value) => {
                if value == i64::MAX as u64 {
                    Some(Self::Uint64(Span::default(), value + 1))
                } else {
                    Some(Self::Uint63(Span::default(), value + 1))
                }
            }
            Self::Uint64(_, value) => {
                if value == u64::MAX {
                    None
                } else {
                    Some(Self::Uint64(Span::default(), value + 1))
                }
            }
        }
    }
}

impl<const ALLOW: u8> Parse for IntegerLiteralT<ALLOW> {
    fn desc() -> Cow<'static, str> {
        "integer literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let (negative_op_span, rest) =
            if let (mut rest, Some(negative_op)) = <Op![-]>::try_parse_raw(input)? {
                WsAndComments::try_parse(&mut rest)?;
                (Some(negative_op.span()), rest)
            } else {
                (None, input.clone())
            };

        let mut chars = rest.char_indices();
        if let Some((_, ch)) = chars.next() {
            let mut base = 10;
            let mut value = 0_i128;
            let mut need_digit;

            if ch == '0' {
                if let Some((_, ch)) = chars.next() {
                    base = match ch {
                        '0'..='9' => {
                            value = (ch as u8 - b'0') as i128;
                            need_digit = false;
                            8
                        }
                        'x' | 'X' => {
                            need_digit = true;
                            16
                        }
                        _ => {
                            let (mut span, rest) = rest.split_at(1);
                            if let Some(negative_op_span) = negative_op_span {
                                span = negative_op_span.join(&span);
                            }
                            return Ok((rest, Some(Self::Uint31(span, 0))));
                        }
                    };
                } else {
                    return Ok((rest.end(), Some(Self::Uint31(input.clone(), 0))));
                }
            } else if matches!(ch, '1'..='9') {
                need_digit = false;
                value = (ch as u8 - b'0') as i128;
            } else if let Some(negative_op_span) = negative_op_span {
                return Err(ParseErr::new(
                    negative_op_span,
                    "expected integer after `-`",
                ));
            } else {
                return Ok((input.clone(), None));
            }

            let mut endi = input.len();
            let mut suffix = None;
            let digitkind = match base {
                8 => {
                    for (i, ch) in chars.by_ref() {
                        match ch {
                            '0'..='7' => {
                                if suffix.is_some() {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "digit after suffix",
                                    ));
                                }
                                need_digit = false;
                                value = value
                                    .checked_shl(3)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as i128))
                                    .ok_or_else(|| {
                                        ParseErr::new(rest.slice(..=i), "octal literal overflow")
                                    })?;
                            }
                            '\'' if !need_digit && suffix.is_none() => need_digit = true,
                            _ => {
                                endi = i;
                                break;
                            }
                        }
                    }
                    "octal"
                }
                10 => {
                    for (i, ch) in chars.by_ref() {
                        match ch {
                            '0'..='9' => {
                                if suffix.is_some() {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "digit after suffix",
                                    ));
                                }
                                need_digit = false;
                                value = value
                                    .checked_mul(10)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as i128))
                                    .ok_or_else(|| {
                                        ParseErr::new(rest.slice(..=i), "decimal literal overflow")
                                    })?;
                            }
                            '\'' if !need_digit && suffix.is_none() => need_digit = true,
                            _ => {
                                endi = i;
                                break;
                            }
                        }
                    }
                    "decimal"
                }
                16 => {
                    for (i, ch) in chars.by_ref() {
                        value = match ch {
                            '0'..='9' => {
                                if suffix.is_some() {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "digit after suffix",
                                    ));
                                }
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as i128))
                            }
                            'a'..='f' => {
                                if suffix.is_some() {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "digit after suffix",
                                    ));
                                }
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'a' + 10) as i128))
                            }
                            'A'..='F' => {
                                if suffix.is_some() {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "digit after suffix",
                                    ));
                                }
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'A' + 10) as i128))
                            }
                            '\'' if !need_digit && suffix.is_none() => {
                                need_digit = true;
                                continue;
                            }
                            'u' => {
                                if suffix.is_none() {
                                    suffix = Some("u");
                                } else {
                                    return Err(ParseErr::new(
                                        rest.slice(i..=i),
                                        "double `u` suffix",
                                    ));
                                }
                                continue;
                            }
                            _ => {
                                endi = i;
                                break;
                            }
                        }
                        .ok_or_else(|| {
                            ParseErr::new(rest.slice(..=i), "hexadecimal literal overflow")
                        })?
                    }
                    "hexadecimal"
                }
                n => unimplemented!("base {n} integer literal"),
            };

            if need_digit {
                let span = if endi == rest.len() {
                    rest.end()
                } else {
                    let endie = chars.next().map(|(i, _)| i).unwrap_or(rest.len());
                    rest.slice(endi..endie)
                };
                return Err(ParseErr::new(span, format!("expected {digitkind} digit")));
            }

            let (mut span, rest) = rest.split_at(endi);

            if let Some(negative_op_span) = negative_op_span {
                span = negative_op_span.join(&span);
                value = -value;
            }

            let value = if let Ok(value) = i32::try_from(value) {
                if value >= 0 {
                    Self::Uint31(span.clone(), value as u32)
                } else {
                    Self::Int32(span.clone(), value)
                }
            } else if let Ok(value) = u32::try_from(value) {
                Self::Uint32(span.clone(), value)
            } else if let Ok(value) = i64::try_from(value) {
                if value >= 0 {
                    Self::Uint63(span.clone(), value as u64)
                } else {
                    Self::Int64(span.clone(), value)
                }
            } else if let Ok(value) = u64::try_from(value) {
                Self::Uint64(span.clone(), value)
            } else {
                return Err(ParseErr::new(span, "integer literal out of range"));
            };

            if ALLOW & ALLOW_I32_LITERAL == 0 && matches!(value, Self::Int32(_, _))
                || (ALLOW & ALLOW_U32_LITERAL == 0 && matches!(value, Self::Uint32(_, _)))
                || (ALLOW & ALLOW_I64_LITERAL == 0 && matches!(value, Self::Int64(_, _)))
                || (ALLOW & ALLOW_U64_LITERAL == 0 && matches!(value, Self::Uint64(_, _)))
                || (ALLOW & (ALLOW_I64_LITERAL | ALLOW_U64_LITERAL) == 0
                    && matches!(value, Self::Uint63(_, _)))
            {
                let mut expecteds = Vec::new();
                if ALLOW & ALLOW_I32_LITERAL != 0 {
                    expecteds.push("32-bit signed int");
                }
                if ALLOW & ALLOW_U32_LITERAL != 0 {
                    expecteds.push("32-bit unsigned int");
                }
                if ALLOW & ALLOW_I64_LITERAL != 0 {
                    expecteds.push("64-bit signed int");
                }
                if ALLOW & ALLOW_U64_LITERAL != 0 {
                    expecteds.push("64-bit unsigned int");
                }
                let mut expected = expecteds.remove(0).to_owned();
                for i in expecteds.drain(..) {
                    expected.push_str(" or ");
                    expected.push_str(i);
                }
                return Err(ParseErr::new(span, expected));
            }

            Ok((rest, Some(value)))
        } else if let Some(negative_op_span) = negative_op_span {
            Err(ParseErr::new(
                negative_op_span,
                "expected integer after `-`",
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Debug)]
pub struct StringLiteral {
    span: Span,
    str: CString,
}

impl GetSpan for StringLiteral {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for StringLiteral {
    fn desc() -> Cow<'static, str> {
        "string literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(open_quote) = Op::<'\"'>::try_parse(&mut rest)? {
            if let Some((i, ch)) = rest.char_indices().find(|(_, c)| matches!(c, '\\' | '\"')) {
                if ch == '\\' {
                    Err(ParseErr::new(rest.slice(i..=i), "escapes aren't supported"))
                } else {
                    let (span, rest) = rest.split_at(i);
                    let str = CString::new(span.as_str()).unwrap();
                    Ok((rest, Some(Self { span, str })))
                }
            } else {
                Err(ParseErr::new(
                    open_quote.span,
                    "unterminated string literal",
                ))
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}
