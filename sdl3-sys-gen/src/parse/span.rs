#![allow(unexpected_cfgs)]

use super::{Parse, ParseContext, ParseErr, ParseRawRes, ParseRes};
use core::hash::Hash;
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
    ops::{Bound, RangeBounds},
    rc::Rc,
    str::{CharIndices, Chars},
};

const NO_SOURCE: &Source = &Source {
    text: String::new(),
    name: String::new(),
};

pub trait GetSpan {
    fn span(&self) -> Span;
}

#[derive(Clone, Debug, Default)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

impl<T> GetSpan for Spanned<T> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<T: Parse> Parse for Spanned<T> {
    fn desc() -> std::borrow::Cow<'static, str> {
        T::desc()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, value) = T::try_parse_raw(ctx, input)?;
        if let Some(value) = value {
            let span = input.start().join(&rest.start());
            Ok((rest, Some(Self { span, value })))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub struct Source {
    pub text: String,
    pub name: String,
}

impl Source {
    pub fn new(name: String, text: String) -> Self {
        Self { text, name }
    }
}

impl Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Source")
            .field("text", &self.text.as_ptr())
            .field("name", &self.name)
            .finish()
    }
}

impl From<Source> for Span {
    fn from(value: Source) -> Self {
        Span::new(Rc::new(value))
    }
}

#[derive(Clone, Default)]
pub struct Span {
    src: Option<Rc<Source>>,
    start: usize,
    end: usize,
    #[cfg(feature = "extra-debugging")]
    #[allow(unused)]
    str: String,
}

impl Span {
    pub fn new(src: impl Into<Rc<Source>>) -> Self {
        let src = src.into();
        let end = src.text.len();
        Self {
            #[cfg(feature = "extra-debugging")]
            str: src.text.clone(),
            src: Some(src),
            start: 0,
            end,
        }
    }

    pub fn new_inline(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Source::new("<inline>".into(), src.into().to_string()))
    }

    pub fn none() -> Self {
        Self {
            #[cfg(feature = "extra-debugging")]
            str: String::new(),
            src: None,
            start: 0,
            end: 0,
        }
    }

    pub fn clone_range(&self, start: usize, end: usize) -> Self {
        let src = self.src.clone();
        Self {
            #[cfg(feature = "extra-debugging")]
            str: src
                .as_ref()
                .map(|s| s.text[start..end].to_owned())
                .unwrap_or_default(),
            src,
            start,
            end,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn source(&self) -> &Source {
        self.src.as_deref().unwrap_or(NO_SOURCE)
    }

    pub fn as_str(&self) -> &str {
        match &self.src {
            Some(s) => &s.text[self.start..self.end],
            None => "",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    pub fn chars(&self) -> Chars {
        self.as_str().chars()
    }

    pub fn char_indices(&self) -> CharIndices {
        self.as_str().char_indices()
    }

    pub fn start(&self) -> Self {
        self.clone_range(self.start, self.start)
    }

    pub fn start_pos(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> Self {
        self.clone_range(self.end, self.end)
    }

    pub fn end_pos(&self) -> usize {
        self.end
    }

    pub fn join(&self, other: &Self) -> Self {
        assert!(self.source().text.as_ptr() == other.source().text.as_ptr());
        assert!(self.source().text.len() == other.source().text.len());
        self.clone_range(self.start.min(other.start), self.end.max(other.end))
    }

    #[must_use]
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Self {
        let start = match range.start_bound() {
            Bound::Included(i) => self.start.checked_add(*i).unwrap(),
            Bound::Excluded(i) => self
                .start
                .checked_add(*i)
                .and_then(|i| i.checked_add(1))
                .unwrap(),
            Bound::Unbounded => self.start,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => self
                .start
                .checked_add(*i)
                .and_then(|i| i.checked_add(1))
                .unwrap(),
            Bound::Excluded(i) => self.start.checked_add(*i).unwrap(),
            Bound::Unbounded => self.end,
        };
        assert!(start <= end && end <= self.source().text.len());
        self.clone_range(start, end)
    }

    #[must_use]
    pub fn split_at(&self, index: usize) -> (Self, Self) {
        assert!(self.start + index <= self.end);
        (
            self.clone_range(self.start, self.start + index),
            self.clone_range(self.start + index, self.end),
        )
    }

    pub fn contains(&self, pat: &str) -> bool {
        self.as_str().contains(pat)
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.as_str().starts_with(pat)
    }

    pub fn starts_with_ch(&self, pat: char) -> bool {
        self.as_str().starts_with(pat)
    }

    pub fn ends_with(&self, pat: &str) -> bool {
        self.as_str().ends_with(pat)
    }

    pub fn strip_prefix(&self, prefix: impl AsRef<str>) -> Option<Self> {
        let prefix = prefix.as_ref();
        if self.as_str().starts_with(prefix) {
            Some(self.slice(prefix.len()..))
        } else {
            None
        }
    }

    pub fn strip_prefix_ch(&self, prefix: char) -> Option<Self> {
        if self.as_str().starts_with(prefix) {
            Some(self.slice(prefix.len_utf8()..))
        } else {
            None
        }
    }

    pub fn trim(&self) -> Span {
        self.trim_start().trim_end()
    }

    pub fn trim_start(&self) -> Span {
        for (i, ch) in self.char_indices() {
            if ch.is_whitespace() {
                continue;
            }
            return self.slice(i..);
        }
        self.end()
    }

    pub fn trim_end(&self) -> Span {
        for (i, ch) in self.char_indices().rev() {
            if ch.is_whitespace() {
                continue;
            }
            return self.slice(..=i);
        }
        self.start()
    }

    pub fn trim_wsc(&self) -> ParseRes<Span> {
        self.trim_wsc_start()?.trim_wsc_end()
    }

    pub fn trim_wsc_start(&self) -> ParseRes<Span> {
        let mut chars = self.char_indices();
        'trim: while let Some((i, ch)) = chars.next() {
            if ch.is_whitespace() {
                continue;
            }
            if ch == '\\' {
                if let Some((_, '\n')) = chars.next() {
                    continue;
                }
            } else if ch == '/'
                && self.as_bytes().get(i + 1) == Some(&b'*')
                && (self.as_bytes().get(i + 2) != Some(&b'*')
                    || self.as_bytes().get(i + 3) == Some(&b'/'))
            {
                // strip /* .. */ comments, but not /** .. */ doc comments
                for (i, ch) in chars.by_ref() {
                    if ch == '/' && self.as_bytes()[i - 1] == b'*' {
                        continue 'trim;
                    }
                }
                return Err(ParseErr::new(
                    self.slice(i..=i + 1),
                    "unterminated block comment",
                ));
            }
            return Ok(self.slice(i..));
        }
        Ok(self.end())
    }

    pub fn trim_wsc_end(&self) -> ParseRes<Span> {
        let mut chars = self.char_indices().rev();
        'trim: while let Some((i, ch)) = chars.next() {
            if ch == '\n' && self.as_bytes()[i.saturating_sub(1)] == b'\\' {
                chars.next();
                continue;
            }
            if ch.is_whitespace() {
                continue;
            }
            if i > 0 && ch == '/' && self.as_bytes()[i - 1] == b'*' {
                // strip /* .. */ comments, but not /** .. */ doc comments
                for (j, ch) in chars.by_ref() {
                    if ch == '/' && self.as_bytes()[j + 1] == b'*' {
                        if j + 2 != i - 1 && self.as_bytes()[j + 2] == b'*' {
                            return Ok(self.slice(..=i));
                        }
                        continue 'trim;
                    }
                }
                return Err(ParseErr::new(
                    self.slice(i - 1..=i),
                    "block comment end with no beginning",
                ));
            }
            return Ok(self.slice(..=i));
        }
        Ok(self.start())
    }
}

impl AsRef<str> for Span {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("src", &self.src)
            .field("as_str()", &self.as_str())
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl Eq for Span {}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for Span {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
