use super::{GetSpan, Op, Parse, ParseErr, ParseRawRes, Span, Spanned};
use std::{borrow::Cow, ffi::CString};

#[derive(Clone, Debug)]
pub enum Literal {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
}

impl GetSpan for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Integer(l) => l.span(),
            Self::Float(l) => l.span(),
            Self::String(l) => l.span(),
        }
    }
}

impl Parse for Literal {
    fn desc() -> Cow<'static, str> {
        "literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(lit)) = FloatLiteral::try_parse_raw(input)? {
            Ok((rest, Some(Self::Float(lit))))
        } else if let (rest, Some(lit)) = IntegerLiteral::try_parse_raw(input)? {
            Ok((rest, Some(Self::Integer(lit))))
        } else if let (rest, Some(lit)) = StringLiteral::try_parse_raw(input)? {
            Ok((rest, Some(Self::String(lit))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub enum FloatLiteral {
    Float(Spanned<f32>),
    Double(Spanned<f64>),
}

impl GetSpan for FloatLiteral {
    fn span(&self) -> Span {
        match self {
            Self::Float(lit) => lit.span(),
            Self::Double(lit) => lit.span(),
        }
    }
}

impl Parse for FloatLiteral {
    fn desc() -> Cow<'static, str> {
        "float literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut chars = input.char_indices().peekable();
        let mut have_digits = false;

        let next = 'int_part: {
            for (i, ch) in chars.by_ref() {
                match ch {
                    '0'..='9' => have_digits = true,
                    '.' => break 'int_part ch,
                    'e' | 'E' if have_digits => break 'int_part ch,
                    'f' | 'F' if have_digits => {
                        let span = input.slice(..i);
                        let rest = input.slice(i + 1..);
                        let value = span.as_str().parse().unwrap();
                        return Ok((rest, Some(FloatLiteral::Float(Spanned { span, value }))));
                    }
                    _ => return Ok((input.clone(), None)),
                }
            }
            // integer
            return Ok((input.clone(), None));
        };

        if next == '.' {
            'dec_part: {
                for (i, ch) in chars.by_ref() {
                    match ch {
                        '0'..='9' => have_digits = true,
                        'e' | 'E' if have_digits => break 'dec_part,
                        'f' | 'F' if have_digits => {
                            let span = input.slice(..i);
                            let rest = input.slice(i + 1..);
                            let value = span.as_str().parse().unwrap();
                            return Ok((rest, Some(FloatLiteral::Float(Spanned { span, value }))));
                        }
                        _ => {
                            if have_digits {
                                let (span, rest) = input.split_at(i);
                                let value = span.as_str().parse().unwrap();
                                return Ok((
                                    rest,
                                    Some(FloatLiteral::Double(Spanned { span, value })),
                                ));
                            } else {
                                return Ok((input.clone(), None));
                            }
                        }
                    }
                }
                if have_digits {
                    let value = input.as_str().parse().unwrap();
                    return Ok((
                        input.end(),
                        Some(FloatLiteral::Double(Spanned {
                            span: input.clone(),
                            value,
                        })),
                    ));
                } else {
                    return Ok((input.clone(), None));
                }
            }
        }

        if let Some((_, '+' | '-')) = chars.peek() {
            chars.next();
        }

        have_digits = false;

        for (i, ch) in chars {
            match ch {
                '0'..='9' => have_digits = true,
                'f' | 'F' if have_digits => {
                    let span = input.slice(..i);
                    let rest = input.slice(i + 1..);
                    let value = span.as_str().parse().unwrap();
                    return Ok((rest, Some(FloatLiteral::Float(Spanned { span, value }))));
                }
                _ => {
                    if have_digits {
                        let (span, rest) = input.split_at(i);
                        let value = span.as_str().parse().unwrap();
                        return Ok((rest, Some(FloatLiteral::Double(Spanned { span, value }))));
                    } else {
                        return Ok((input.clone(), None));
                    }
                }
            }
        }

        if have_digits {
            let value = input.as_str().parse().unwrap();
            Ok((
                input.end(),
                Some(FloatLiteral::Double(Spanned {
                    span: input.clone(),
                    value,
                })),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub enum IntegerLiteral {
    Unsuffixed(Spanned<u64>),
    Unsigned(Spanned<u64>),
    Long(Spanned<u64>),
    UnsignedLong(Spanned<u64>),
    LongLong(Spanned<u64>),
    UnsignedLongLong(Spanned<u64>),
}

impl GetSpan for IntegerLiteral {
    fn span(&self) -> Span {
        match self {
            Self::Unsuffixed(lit)
            | Self::Unsigned(lit)
            | Self::Long(lit)
            | Self::UnsignedLong(lit)
            | Self::LongLong(lit)
            | Self::UnsignedLongLong(lit) => lit.span(),
        }
    }
}

impl Parse for IntegerLiteral {
    fn desc() -> Cow<'static, str> {
        "integer literal".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        fn parse_suffix_and_create(
            rest: Span,
            span: Span,
            value: u64,
        ) -> ParseRawRes<Option<IntegerLiteral>> {
            if rest.starts_with("ull") || rest.starts_with("ULL") {
                let (sspan, rest) = rest.split_at(3);
                Ok((
                    rest,
                    Some(IntegerLiteral::UnsignedLongLong(Spanned {
                        span: span.join(&sspan),
                        value,
                    })),
                ))
            } else if rest.starts_with("ul") || rest.starts_with("UL") {
                let (sspan, rest) = rest.split_at(2);
                Ok((
                    rest,
                    Some(IntegerLiteral::UnsignedLong(Spanned {
                        span: span.join(&sspan),
                        value,
                    })),
                ))
            } else if rest.starts_with("u") || rest.starts_with("U") {
                let (sspan, rest) = rest.split_at(1);
                Ok((
                    rest,
                    Some(IntegerLiteral::Unsigned(Spanned {
                        span: span.join(&sspan),
                        value,
                    })),
                ))
            } else if rest.starts_with("ll") || rest.starts_with("LL") {
                let (sspan, rest) = rest.split_at(2);
                Ok((
                    rest,
                    Some(IntegerLiteral::LongLong(Spanned {
                        span: span.join(&sspan),
                        value,
                    })),
                ))
            } else if rest.starts_with("l") || rest.starts_with("L") {
                let (sspan, rest) = rest.split_at(1);
                Ok((
                    rest,
                    Some(IntegerLiteral::Long(Spanned {
                        span: span.join(&sspan),
                        value,
                    })),
                ))
            } else {
                Ok((
                    rest,
                    Some(IntegerLiteral::Unsuffixed(Spanned { span, value })),
                ))
            }
        }

        let rest = input.clone();
        let mut chars = rest.char_indices();
        if let Some((_, ch)) = chars.next() {
            let mut base = 10;
            let mut value = 0_u64;
            let mut need_digit;

            if ch == '0' {
                if let Some((_, ch)) = chars.next() {
                    base = match ch {
                        '0'..='9' => {
                            value = (ch as u8 - b'0') as _;
                            need_digit = false;
                            8
                        }
                        'x' | 'X' => {
                            need_digit = true;
                            16
                        }
                        _ => {
                            let (span, rest) = rest.split_at(1);
                            return parse_suffix_and_create(rest, span, 0);
                        }
                    };
                } else {
                    return parse_suffix_and_create(rest.end(), rest, 0);
                }
            } else if matches!(ch, '1'..='9') {
                need_digit = false;
                value = (ch as u8 - b'0') as _;
            } else {
                return Ok((input.clone(), None));
            }

            let mut endi = input.len();
            let digitkind = match base {
                8 => {
                    for (i, ch) in chars.by_ref() {
                        match ch {
                            '0'..='7' => {
                                need_digit = false;
                                value = value
                                    .checked_shl(3)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as _))
                                    .ok_or_else(|| {
                                        ParseErr::new(rest.slice(..=i), "octal literal overflow")
                                    })?;
                            }
                            '\'' if !need_digit => need_digit = true,
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
                                need_digit = false;
                                value = value
                                    .checked_mul(10)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as _))
                                    .ok_or_else(|| {
                                        ParseErr::new(rest.slice(..=i), "decimal literal overflow")
                                    })?;
                            }
                            '\'' if !need_digit => need_digit = true,
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
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'0') as _))
                            }
                            'a'..='f' => {
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'a' + 10) as _))
                            }
                            'A'..='F' => {
                                need_digit = false;
                                value
                                    .checked_shl(4)
                                    .and_then(|v| v.checked_add((ch as u8 - b'A' + 10) as _))
                            }
                            '\'' if !need_digit => {
                                need_digit = true;
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

            let (span, rest) = rest.split_at(endi);
            parse_suffix_and_create(rest, span, value)
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
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
                    let span = rest.slice(..i);
                    let rest = rest.slice(i + 1..);
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
