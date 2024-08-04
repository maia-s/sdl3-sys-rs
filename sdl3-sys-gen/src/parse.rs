use std::{borrow::Cow, fmt::Display};

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

pub type ExprOp = Op<'\0'>;

struct Op<const OP1: char, const OP2: char = '\0', const OP3: char = '\0'> {
    span: Span,
}

impl<const OP1: char, const OP2: char, const OP3: char> GetSpan for Op<OP1, OP2, OP3> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const OP1: char, const OP2: char, const OP3: char> Parse for Op<OP1, OP2, OP3> {
    fn desc() -> Cow<'static, str> {
        if OP1 == '\0' {
            "operator".into()
        } else if OP2 == '\0' {
            format!("`{OP1}`").into()
        } else if OP3 == '\0' {
            format!("`{OP1}{OP2}`").into()
        } else {
            format!("`{OP1}{OP2}{OP3}`").into()
        }
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if OP1 == '\0' {
            const { assert!(OP2 == '\0') }
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
        } else if let Some(rest) = input.strip_prefix_ch(OP1) {
            if OP2 == '\0' {
                const { assert!(OP3 == '\0') }
                let span = input.join(&rest.start());
                return Ok((rest, Some(Self { span })));
            } else if let Some(rest) = rest.strip_prefix_ch(OP2) {
                if OP3 == '\0' {
                    let span = input.join(&rest.start());
                    return Ok((rest, Some(Self { span })));
                } else if let Some(rest) = rest.strip_prefix_ch(OP3) {
                    let span = input.join(&rest.start());
                    return Ok((rest, Some(Self { span })));
                }
            }
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

struct Punctuated<T, P>(pub Vec<(T, Option<P>)>);

impl<T, P> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self(Vec::new())
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
