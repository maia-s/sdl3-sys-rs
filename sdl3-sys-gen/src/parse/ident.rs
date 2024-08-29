use super::{is_keyword, GetSpan, Parse, ParseErr, ParseRawRes, ParseRev, Span};
use core::{borrow::Borrow, fmt::Display, hash::Hash};
use std::borrow::Cow;

pub type Ident = IdentOrKwT<false>;
pub type IdentOrKw = IdentOrKwT<true>;

#[derive(Clone, Debug, PartialOrd, Ord)]
pub struct IdentOrKwT<const ALLOW_KEYWORDS: bool> {
    pub span: Span,
}

impl<const ALLOW_KEYWORDS: bool> Display for IdentOrKwT<ALLOW_KEYWORDS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.span, f)
    }
}

impl TryFrom<IdentOrKw> for Ident {
    type Error = ParseErr;
    fn try_from(value: IdentOrKw) -> Result<Self, Self::Error> {
        if !is_keyword(&value.span) {
            Ok(Self { span: value.span })
        } else {
            Err(ParseErr::new(value.span(), "ident is keyword"))
        }
    }
}

impl<const ALLOW_KEYWORDS: bool> IdentOrKwT<ALLOW_KEYWORDS> {
    pub fn new_inline(ident: impl Into<Cow<'static, str>>) -> Self {
        let ident = ident.into();
        if !ALLOW_KEYWORDS && is_keyword(&ident) {
            panic!("can't create keyword with non-keyword ident");
        }
        Self {
            span: Span::new_inline(ident),
        }
    }
}

impl<const ALLOW_KEYWORDS: bool> IdentOrKwT<ALLOW_KEYWORDS> {
    pub fn as_str(&self) -> &str {
        self.span.as_str()
    }
}

impl<const ALLOW_KEYWORDS: bool> Borrow<str> for IdentOrKwT<ALLOW_KEYWORDS> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const ALLOW_KEYWORDS: bool> GetSpan for IdentOrKwT<ALLOW_KEYWORDS> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const ALLOW_KEYWORDS: bool> PartialEq<str> for IdentOrKwT<ALLOW_KEYWORDS> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const ALLOW_KEYWORDS: bool> PartialEq for IdentOrKwT<ALLOW_KEYWORDS> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const ALLOW_KEYWORDS: bool> Eq for IdentOrKwT<ALLOW_KEYWORDS> {}

impl<const ALLOW_KEYWORDS: bool> Hash for IdentOrKwT<ALLOW_KEYWORDS> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
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
                        return Ok((input.clone(), None));
                    } else {
                        return Ok((input.slice(i..), Some(Self { span })));
                    }
                }
            }
        }
        let span = input.clone();
        if span.is_empty() || (!ALLOW_KEYWORDS && is_keyword(&span)) {
            Ok((input.clone(), None))
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
