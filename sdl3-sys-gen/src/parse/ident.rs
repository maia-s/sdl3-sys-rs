use super::{is_keyword, GetSpan, Parse, ParseErr, ParseRawRes, ParseRev, Span};
use std::borrow::Cow;

pub type Ident = IdentOrKwT<false>;
pub type IdentOrKw = IdentOrKwT<true>;

pub struct IdentOrKwT<const ALLOW_KEYWORDS: bool> {
    pub span: Span,
}

impl<const ALLOW_KEYWORDS: bool> IdentOrKwT<ALLOW_KEYWORDS> {
    pub fn as_str(&self) -> &str {
        self.span.as_str()
    }
}

impl<const ALLOW_KEYWORDS: bool> GetSpan for IdentOrKwT<ALLOW_KEYWORDS> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const ALLOW_KEYWORDS: bool> PartialEq<&str> for IdentOrKwT<ALLOW_KEYWORDS> {
    fn eq(&self, other: &&str) -> bool {
        self.span.as_str() == *other
    }
}

impl<const ALLOW_KEYWORDS: bool> Parse for IdentOrKwT<ALLOW_KEYWORDS> {
    fn desc() -> Cow<'static, str> {
        "ident".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut chars = input.char_indices();
        if let Some(first) = chars.next() {
            match first.1 {
                'a'..='z' | 'A'..='Z' | '_' => (),
                _ => return Ok((input.clone(), None)),
            }
        }
        for (i, ch) in chars {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => (),
                _ => {
                    let span = input.slice(..i);
                    if !ALLOW_KEYWORDS && is_keyword(&span) {
                        let msg = format!("unexpected keyword `{span}`");
                        return Err(ParseErr::new(span, msg));
                    } else {
                        return Ok((input.slice(i..), Some(Self { span })));
                    }
                }
            }
        }
        let span = input.clone();
        if !ALLOW_KEYWORDS && is_keyword(&span) {
            let msg = format!("unexpected keyword `{span}`");
            Err(ParseErr::new(span, msg))
        } else {
            Ok((span.end(), Some(Self { span })))
        }
    }
}

impl<const ALLOW_KEYWORDS: bool> ParseRev for IdentOrKwT<ALLOW_KEYWORDS> {
    fn try_parse_rev_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut id_start = None;
        for (i, ch) in input.char_indices().rev() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => id_start = Some(i),
                '0'..='9' => id_start = None,
                _ => {
                    return if let Some(i) = id_start {
                        let span = input.slice(i..);
                        if !ALLOW_KEYWORDS && is_keyword(&span) {
                            let msg = format!("unexpected keyword `{span}`");
                            Err(ParseErr::new(span, msg))
                        } else {
                            Ok((input.slice(..i), Some(Self { span })))
                        }
                    } else {
                        Ok((input.clone(), None))
                    };
                }
            }
        }
        let span = input.clone();
        if !ALLOW_KEYWORDS && is_keyword(&span) {
            let msg = format!("unexpected keyword `{span}`");
            Err(ParseErr::new(span, msg))
        } else {
            Ok((span.end(), Some(Self { span })))
        }
    }
}
