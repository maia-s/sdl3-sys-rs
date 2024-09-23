use super::{GetSpan, Parse, ParseContext, ParseErr, ParseRawRes, ParseRes, Span};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    num::NonZeroU8,
    str,
};

#[derive(Clone, Copy)]
pub struct Precedence(NonZeroU8);

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

    pub const fn comma() -> Self {
        Self::left_to_right(15)
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

#[derive(Clone)]
pub struct Op<const OP1: char, const OP2: char = '\0', const OP3: char = '\0'> {
    pub span: Span,
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
    const UNARY_PRECEDENCE: Option<Precedence> = Self::_unary_precedence(Self::STR.as_bytes());
    const BINARY_PRECEDENCE: Option<Precedence> = Self::_binary_precedence(Self::STR.as_bytes());

    const fn _unary_precedence(s: &[u8]) -> Option<Precedence> {
        match s {
            b"#" => Some(Precedence::left_to_right(1)),
            b"+" | b"++" | b"-" | b"--" | b"!" | b"~" | b"*" | b"&" => {
                Some(Precedence::right_to_left(2))
            }
            _ => None,
        }
    }

    const fn _binary_precedence(s: &[u8]) -> Option<Precedence> {
        match s {
            b"++" | b"--" | b"(" | b"[" | b"." | b"->" => Some(Precedence::left_to_right(1)),
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
            b"," => Some(Precedence::left_to_right(15)),
            _ => None,
        }
    }

    pub fn unary_precedence(&self) -> Option<Precedence> {
        if Self::STR.is_empty() {
            Self::_unary_precedence(self.span.as_bytes())
        } else {
            Self::UNARY_PRECEDENCE
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

impl ExprOp {
    pub fn try_parse_unop(
        ctx: &ParseContext,
        input: &mut Span,
    ) -> ParseRes<Option<(Precedence, Self)>> {
        if let (rest, Some(op)) = ExprOp::try_parse_raw(ctx, input)? {
            if let Some(prec) = op.unary_precedence() {
                *input = rest;
                return Ok(Some((prec, op)));
            }
        }
        Ok(None)
    }

    pub fn try_parse_binop(
        ctx: &ParseContext,
        input: &mut Span,
    ) -> ParseRes<Option<(Precedence, Self)>> {
        if let (rest, Some(op)) = ExprOp::try_parse_raw(ctx, input)? {
            if let Some(prec) = op.binary_precedence() {
                *input = rest;
                return Ok(Some((prec, op)));
            }
        }
        Ok(None)
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

    fn try_parse_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
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
                    b"!=" | b"##" | b"%=" | b"&&" | b"&=" | b"*=" | b"++" | b"+=" | b"--"
                    | b"-=" | b"->" | b"/=" | b"<<" | b"<=" | b"==" | b">=" | b">>" | b"^="
                    | b"|=" | b"||" => {
                        let (span, rest) = input.split_at(2);
                        return Ok((rest, Some(Self { span })));
                    }
                    _ => (),
                }
            }
            if !input.is_empty() {
                match input.as_bytes()[0] {
                    b'!' | b'#' | b'%' | b'&' | b'(' | b'*' | b'+' | b',' | b'-' | b'.' | b'/'
                    | b':' | b'<' | b'=' | b'>' | b'?' | b'[' | b'^' | b'|' | b'~' => {
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
