use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
    num::NonZeroU8,
    str, u8,
};

macro_rules! Op {
    ($($tt:tt)*) => {
        $crate::parse::Op::<
            { $crate::parse::op_first_ch(stringify!($($tt)*)) },
            { $crate::parse::op_second_ch(stringify!($($tt)*)) },
            { $crate::parse::op_third_ch(stringify!($($tt)*)) },
        >
    };
}

macro_rules! submodules {
    ($($submod:ident),* $(,)?) => { $(
        mod $submod;
        pub use $submod::*;
    )* };
}
submodules!(
    decl,
    doc_comment,
    r#enum,
    expr,
    ident,
    item,
    keywords,
    literal,
    preproc,
    primitive,
    result,
    span,
    r#struct,
    r#type,
);

pub const fn op_first_ch(str: &str) -> char {
    let ch = str.as_bytes()[0];
    assert!(ch <= 0x7f);
    ch as char
}

pub const fn op_second_ch(str: &str) -> char {
    if str.len() < 2 {
        '\0'
    } else {
        let ch = str.as_bytes()[1];
        assert!(ch <= 0x7f);
        ch as char
    }
}

pub const fn op_third_ch(str: &str) -> char {
    assert!(str.len() <= 3);
    if str.len() < 3 {
        '\0'
    } else {
        let ch = str.as_bytes()[2];
        assert!(ch <= 0x7f);
        ch as char
    }
}

pub trait Parse: Sized {
    fn desc() -> Cow<'static, str>;

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        match Self::parse_raw(input) {
            Ok((rest, parsed)) => Ok((rest, Some(parsed))),
            Err(e) => Err(e),
        }
    }

    fn try_parse_raw_eq<T>(input: &Span, cmp: T) -> ParseRawRes<Option<Self>>
    where
        Self: PartialEq<T>,
    {
        if let (rest, Some(parsed)) = Self::try_parse_raw(input)? {
            if parsed == cmp {
                return Ok((rest, Some(parsed)));
            }
        }
        Ok((input.clone(), None))
    }

    fn parse_raw(input: &Span) -> ParseRawRes<Self> {
        match Self::try_parse_raw(input) {
            Ok((rest, Some(parsed))) => Ok((rest, parsed)),
            Ok((rest, None)) => Err(ParseErr::new(
                input.start().join(&rest.start()),
                format!("expected {}", Self::desc()),
            )),
            Err(e) => Err(e),
        }
    }

    fn parse_raw_eq<T>(input: &Span, cmp: T) -> ParseRawRes<Self>
    where
        Self: PartialEq<T>,
        T: Display,
    {
        match Self::try_parse_raw(input) {
            Ok((rest, Some(parsed))) if parsed == cmp => Ok((rest, parsed)),
            Ok((rest, _)) => Err(ParseErr::new(
                input.start().join(&rest.start()),
                format!("expected `{cmp}`"),
            )),
            Err(e) => Err(e),
        }
    }

    fn try_parse(input: &mut Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw(input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_eq<T>(input: &mut Span, cmp: T) -> ParseRes<Option<Self>>
    where
        Self: PartialEq<T>,
    {
        match Self::try_parse_raw_eq(input, cmp) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn parse(input: &mut Span) -> ParseRes<Self> {
        match Self::parse_raw(input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_eq<T>(input: &mut Span, cmp: T) -> ParseRes<Self>
    where
        Self: PartialEq<T>,
        T: Display,
    {
        match Self::parse_raw_eq(input, cmp) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_all(input: Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw(&input) {
            Ok((rest, parsed)) => {
                if rest.is_empty() {
                    Ok(parsed)
                } else {
                    Err(ParseErr::new(
                        rest,
                        format!("unexpected data after {}", Self::desc()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_all(input: Span) -> ParseRes<Self> {
        match Self::parse_raw(&input) {
            Ok((rest, parsed)) => {
                let rest = rest.trim_wsc_start()?;
                if rest.is_empty() {
                    Ok(parsed)
                } else {
                    Err(ParseErr::new(
                        rest.trim_wsc_end()?,
                        format!("unexpected data after {}", Self::desc()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}

trait ParseRev: Parse {
    fn try_parse_rev_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        match Self::parse_rev_raw(input) {
            Ok((rest, parsed)) => Ok((rest, Some(parsed))),
            Err(e) => Err(e),
        }
    }

    fn parse_rev_raw(input: &Span) -> ParseRawRes<Self> {
        match Self::try_parse_rev_raw(input) {
            Ok((rest, Some(parsed))) => Ok((rest, parsed)),
            Ok((rest, None)) => Err(ParseErr::new(
                input.start().join(&rest.start()),
                format!("expected {}", Self::desc()),
            )),
            Err(e) => Err(e),
        }
    }

    fn try_parse_rev(input: &mut Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_rev_raw(input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_rev(input: &mut Span) -> ParseRes<Self> {
        match Self::parse_rev_raw(input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_rev_all(input: &Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_rev_raw(&input) {
            Ok((rest, parsed)) => {
                if rest.is_empty() {
                    Ok(parsed)
                } else {
                    Err(ParseErr::new(
                        rest,
                        format!("unexpected data before {}", Self::desc()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_rev_all(input: Span) -> ParseRes<Self> {
        match Self::parse_rev_raw(&input) {
            Ok((rest, parsed)) => {
                if rest.is_empty() {
                    Ok(parsed)
                } else {
                    Err(ParseErr::new(
                        rest,
                        format!("unexpected data before {}", Self::desc()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl<T: Parse> Parse for Option<T> {
    fn desc() -> Cow<'static, str> {
        format!("optional {}", T::desc()).into()
    }

    #[inline(always)]
    fn parse_raw(input: &Span) -> ParseRawRes<Self> {
        T::try_parse_raw(input)
    }
}

impl<T: ParseRev> ParseRev for Option<T> {
    #[inline(always)]
    fn parse_rev_raw(input: &Span) -> ParseRawRes<Self> {
        T::try_parse_rev_raw(input)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn desc() -> Cow<'static, str> {
        T::desc()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, Some(parsed)) = T::try_parse_raw(input)? else {
            return Ok((input.clone(), None));
        };
        vec.push(parsed);
        while let (rest_, Some(parsed)) = T::try_parse_raw(&rest)? {
            rest = rest_;
            vec.push(parsed);
        }
        Ok((rest, Some(vec)))
    }
}

impl<T: ParseRev> ParseRev for Vec<T> {
    fn try_parse_rev_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, Some(parsed)) = T::try_parse_rev_raw(input)? else {
            return Ok((input.clone(), None));
        };
        vec.push(parsed);
        while let (rest_, Some(parsed)) = T::try_parse_rev_raw(&rest)? {
            rest = rest_;
            vec.push(parsed);
        }
        Ok((rest, Some(vec)))
    }
}

pub struct Keyword<const KW_INDEX: usize> {
    span: Span,
}

impl<const KW_INDEX: usize> Debug for Keyword<KW_INDEX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Keyword<`{}`>", KEYWORDS[KW_INDEX]))
            .field("span", &self.span)
            .finish()
    }
}

impl<const KW_INDEX: usize> GetSpan for Keyword<KW_INDEX> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const KW_INDEX: usize> Parse for Keyword<KW_INDEX> {
    fn desc() -> Cow<'static, str> {
        format!("`{}`", KEYWORDS[KW_INDEX]).into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(ident)) = IdentOrKw::try_parse_raw_eq(input, KEYWORDS[KW_INDEX])? {
            Ok((rest, Some(Keyword { span: ident.span })))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Copy)]
struct Precedence(NonZeroU8);

impl Precedence {
    pub const fn left_to_right(prec: u8) -> Self {
        assert!(prec != 0);
        match prec.checked_shl(1) {
            Some(prec) => Self(unsafe { NonZeroU8::new_unchecked(prec) }),
            None => panic!("precedence out of range"),
        }
    }

    pub const fn right_to_left(prec: u8) -> Self {
        assert!(prec != 0);
        match prec.checked_shl(1) {
            Some(prec) => Self(unsafe { NonZeroU8::new_unchecked(prec + 1) }),
            None => panic!("precedence out of range"),
        }
    }

    pub const fn max() -> Self {
        Self(NonZeroU8::MAX)
    }

    pub const fn parse_rhs_first(self, rhs: Self) -> bool {
        let l = self.0.get();
        let r = rhs.0.get() & !1;
        r < l
    }
}

impl Debug for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Precedence({}({}))",
            if self.0.get() & 1 != 0 { "RTL" } else { "LTR" },
            self.0.get() >> 1
        )
    }
}

pub type ExprOp = Op<'\0'>;

struct Op<const OP1: char, const OP2: char = '\0', const OP3: char = '\0'> {
    span: Span,
}

impl<const OP1: char, const OP2: char, const OP3: char> Op<OP1, OP2, OP3> {
    const STR: &'static str = {
        assert!(OP1 as u32 <= 0x7f && OP2 as u32 <= 0x7f && OP3 as u32 <= 0x7f);
        let bstr: &[u8] = if OP1 == '\0' && OP2 == '\0' && OP3 == '\0' {
            &[]
        } else if OP1 != '\0' && OP2 == '\0' && OP3 == '\0' {
            &[OP1 as u8]
        } else if OP1 != '\0' && OP2 != '\0' && OP3 == '\0' {
            &[OP1 as u8, OP2 as u8]
        } else if OP1 != '\0' && OP2 != '\0' && OP3 != '\0' {
            &[OP1 as u8, OP2 as u8, OP3 as u8]
        } else {
            panic!("invalid op")
        };
        unsafe { str::from_utf8_unchecked(bstr) }
    };
    const BINARY_PRECEDENCE: Option<Precedence> = Self::_binary_precedence(Self::STR.as_bytes());

    const fn _binary_precedence(s: &[u8]) -> Option<Precedence> {
        match s {
            b"." | b"->" => Some(Precedence::left_to_right(1)),
            b"*" | b"/" | b"%" => Some(Precedence::left_to_right(3)),
            b"+" | b"-" => Some(Precedence::left_to_right(4)),
            b"<<" | b">>" => Some(Precedence::left_to_right(5)),
            b"<" | b"<=" | b">" | b">=" => Some(Precedence::left_to_right(6)),
            b"==" | b"!=" => Some(Precedence::left_to_right(7)),
            b"&" => Some(Precedence::left_to_right(8)),
            b"^" => Some(Precedence::left_to_right(9)),
            b"|" => Some(Precedence::left_to_right(10)),
            b"&&" => Some(Precedence::left_to_right(11)),
            b"||" => Some(Precedence::left_to_right(12)),
            b"?" => Some(Precedence::right_to_left(13)),
            b"=" | b"+=" | b"-=" | b"*=" | b"/=" | b"%=" | b"<<=" | b">>=" | b"&=" | b"^="
            | b"|=" => Some(Precedence::right_to_left(14)),
            _ => None,
        }
    }

    pub fn binary_precedence(&self) -> Option<Precedence> {
        if Self::STR.is_empty() {
            Self::_binary_precedence(self.span.as_bytes())
        } else {
            Self::BINARY_PRECEDENCE
        }
    }

    pub fn as_str(&self) -> &str {
        self.span.as_str()
    }
}

impl<const OP1: char, const OP2: char, const OP3: char> Debug for Op<OP1, OP2, OP3> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Op<`{}`>", Self::STR))
            .field("span", &self.span)
            .finish()
    }
}

impl<const OP1: char, const OP2: char, const OP3: char> TryFrom<Span> for Op<OP1, OP2, OP3> {
    type Error = ParseErr;

    fn try_from(span: Span) -> Result<Self, Self::Error> {
        if Self::STR.is_empty() || Self::STR == span.as_str() {
            Ok(Self { span })
        } else {
            Err(ParseErr::new(span, format!("expected `{}`", Self::STR)))
        }
    }
}

impl<const OP1: char, const OP2: char, const OP3: char> GetSpan for Op<OP1, OP2, OP3> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const OP1: char, const OP2: char, const OP3: char> Parse for Op<OP1, OP2, OP3> {
    fn desc() -> Cow<'static, str> {
        if Self::STR.is_empty() {
            "operator".into()
        } else {
            format!("`{}`", Self::STR).into()
        }
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if Self::STR.is_empty() {
            if input.len() >= 3 {
                match &input.as_bytes()[..3] {
                    b"..." | b"<<=" | b">>=" => {
                        let (span, rest) = input.split_at(3);
                        return Ok((rest, Some(Self { span })));
                    }
                    _ => (),
                }
            }
            if input.len() >= 2 {
                match &input.as_bytes()[..2] {
                    b"!=" | b"%=" | b"&&" | b"&=" | b"*=" | b"++" | b"+=" | b"--" | b"-="
                    | b"->" | b"/=" | b"<<" | b"<=" | b"==" | b">=" | b">>" | b"^=" | b"|="
                    | b"||" => {
                        let (span, rest) = input.split_at(2);
                        return Ok((rest, Some(Self { span })));
                    }
                    _ => (),
                }
            }
            if !input.is_empty() {
                match input.as_bytes()[0] {
                    b'!' | b'%' | b'&' | b'*' | b'+' | b'-' | b'.' | b'/' | b':' | b'<' | b'='
                    | b'>' | b'?' | b'^' | b'|' | b'~' => {
                        let (span, rest) = input.split_at(1);
                        return Ok((rest, Some(Self { span })));
                    }
                    _ => (),
                }
            }
        } else if let Some(rest) = input.strip_prefix(Self::STR) {
            let span = input.start().join(&rest.start());
            return Ok((rest, Some(Self { span })));
        }
        Ok((input.clone(), None))
    }
}

struct Delimited<Open, T, Close> {
    open: Open,
    value: T,
    close: Close,
}

impl<Open: Parse, T: Parse, Close: Parse> Parse for Delimited<Open, T, Close> {
    fn desc() -> Cow<'static, str> {
        format!("{} {} {}", Open::desc(), T::desc(), Close::desc()).into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(open) = Open::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let value = T::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            let close = Close::parse(&mut rest)?;
            Ok((rest, Some(Self { open, value, close })))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Debug)]
struct Punctuated<T, P>(pub Vec<(T, Option<P>)>);

impl<T, P> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T, P> From<Punctuated<T, P>> for Vec<T> {
    fn from(value: Punctuated<T, P>) -> Self {
        value.0.into_iter().map(|(t, _)| t).collect()
    }
}

impl<T, P> Parse for Punctuated<T, P>
where
    T: Parse,
    P: Parse,
{
    fn desc() -> Cow<'static, str> {
        format!("{} punctuated by {}", T::desc(), P::desc()).into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, value) = T::try_parse_raw(input)?;
        if let Some(mut value) = value {
            while let (rest_, Some(punct)) = P::try_parse_raw(&rest.trim_wsc_start()?)? {
                rest = rest_;
                vec.push((value, Some(punct)));
                let (rest_, value_) = T::parse_raw(&rest.trim_wsc_start()?)?;
                rest = rest_;
                value = value_;
            }
            vec.push((value, None));
            Ok((rest, Some(Self(vec))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

struct Terminated<T, Term> {
    value: T,
    term: Term,
}

impl<T: Parse, Term: Parse> Parse for Terminated<T, Term> {
    fn desc() -> Cow<'static, str> {
        T::desc()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(value) = T::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let term = Term::parse(&mut rest)?;
            Ok((rest, Some(Self { value, term })))
        } else {
            Ok((input.clone(), None))
        }
    }
}

struct WsAndComments(Span);

impl Parse for WsAndComments {
    fn desc() -> Cow<'static, str> {
        "whitespace or comments".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_start()?;
        let rest_len = rest.len();
        if rest_len < input.len() {
            Ok((rest, Some(Self(input.slice(..input.len() - rest_len)))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl ParseRev for WsAndComments {
    fn try_parse_rev_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_end()?;
        let rest_len = rest.len();
        if rest_len < input.len() {
            Ok((rest, Some(Self(input.slice(input.len() - rest_len..)))))
        } else {
            Ok((input.clone(), None))
        }
    }
}
