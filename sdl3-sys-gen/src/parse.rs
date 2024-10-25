use crate::Defer;
use core::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    fmt::Display,
};
use std::{borrow::Cow, collections::HashMap, fmt::Debug, marker::PhantomData, str};

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
    attr,
    doc_comment,
    r#enum,
    r#fn,
    expr,
    ident,
    item,
    keyword,
    literal,
    op,
    preproc,
    primitive,
    result,
    span,
    r#struct,
    r#type,
);
mod patch;
use patch::*;

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

pub struct ParseContext {
    module: String,
    pub parent_struct_ident: RefCell<Option<Ident>>,
    pub sibling_struct_index: RefCell<usize>,
    pub log_debug_enabled: Cell<bool>,
    patch_idents: RefCell<HashMap<IdentOrKw, IdentOrKw>>,
    pub active_typedef: RefCell<Option<TypeDef>>,
}

impl ParseContext {
    pub const FIRST_SIBLING: usize = 1;

    pub fn new(module: Option<String>) -> Self {
        Self {
            module: module.unwrap_or_default(),
            parent_struct_ident: RefCell::new(None),
            sibling_struct_index: RefCell::new(Self::FIRST_SIBLING),
            log_debug_enabled: Cell::new(false),
            patch_idents: RefCell::new(HashMap::new()),
            active_typedef: RefCell::new(None),
        }
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn with_parent_struct_guard(&self, ident: Option<Ident>) -> impl Drop + '_ {
        let mut psi = self.parent_struct_ident.replace(ident);
        let ssi = self.sibling_struct_index.replace(Self::FIRST_SIBLING);
        Defer::new(move || {
            *self.parent_struct_ident.borrow_mut() = psi.take();
            *self.sibling_struct_index.borrow_mut() = *ssi.borrow();
        })
    }

    #[must_use]
    pub fn set_debug_log_guard(&self, enable: bool) -> impl Drop + '_ {
        let was_enabled = self.log_debug_enabled.replace(enable);
        Defer::new(move || self.log_debug_enabled.set(was_enabled))
    }

    pub fn log_debug(&self, what: impl Display) -> Result<(), ParseErr> {
        if self.log_debug_enabled.get() {
            eprintln!("[sdl3-sys-gen][debug] {what}");
        }
        Ok(())
    }

    #[must_use]
    pub fn patch_idents_state_guard(&self) -> impl Drop + '_ {
        let prev = self.patch_idents.replace_with(|prev| prev.clone());
        Defer::new(move || {
            self.patch_idents.replace(prev);
        })
    }

    pub fn add_patch_ident(&self, src: IdentOrKw, dst: IdentOrKw) {
        self.patch_idents.borrow_mut().insert(src, dst);
    }

    pub fn patch_ident(&self, src: &IdentOrKw) -> Option<IdentOrKw> {
        self.patch_idents.borrow().get(src).cloned()
    }
}

pub trait Parse: Sized {
    fn desc() -> Cow<'static, str>;

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        match Self::parse_raw(ctx, input) {
            Ok((rest, parsed)) => Ok((rest, Some(parsed))),
            Err(e) => Err(e),
        }
    }

    fn try_parse_raw_if(
        ctx: &ParseContext,
        input: &Span,
        accept: impl FnOnce(&Self) -> bool,
    ) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(parsed)) = Self::try_parse_raw(ctx, input)? {
            if accept(&parsed) {
                return Ok((rest, Some(parsed)));
            }
        }
        Ok((input.clone(), None))
    }

    fn parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Self> {
        match Self::try_parse_raw(ctx, input) {
            Ok((rest, Some(parsed))) => Ok((rest, parsed)),
            Ok((rest, None)) => Err(ParseErr::new(
                input.start().join(&rest.start()),
                format!("expected {}", Self::desc()),
            )),
            Err(e) => Err(e),
        }
    }

    fn try_parse(ctx: &ParseContext, input: &mut Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw(ctx, input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_if(
        ctx: &ParseContext,
        input: &mut Span,
        accept: impl FnOnce(&Self) -> bool,
    ) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw_if(ctx, input, accept) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn parse(ctx: &ParseContext, input: &mut Span) -> ParseRes<Self> {
        match Self::parse_raw(ctx, input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_try_all(ctx: &ParseContext, input: &Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw(ctx, input) {
            Ok((rest, parsed)) => {
                if rest.trim_wsc_start()?.is_empty() {
                    Ok(parsed)
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_all(ctx: &ParseContext, input: Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_raw(ctx, &input) {
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

    fn parse_all(ctx: &ParseContext, input: Span) -> ParseRes<Self> {
        match Self::parse_raw(ctx, &input) {
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
    fn try_parse_rev_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        match Self::parse_rev_raw(ctx, input) {
            Ok((rest, parsed)) => Ok((rest, Some(parsed))),
            Err(e) => Err(e),
        }
    }

    fn parse_rev_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Self> {
        match Self::try_parse_rev_raw(ctx, input) {
            Ok((rest, Some(parsed))) => Ok((rest, parsed)),
            Ok((rest, None)) => Err(ParseErr::new(
                input.start().join(&rest.start()),
                format!("expected {}", Self::desc()),
            )),
            Err(e) => Err(e),
        }
    }

    fn try_parse_rev(ctx: &ParseContext, input: &mut Span) -> ParseRes<Option<Self>> {
        match Self::try_parse_rev_raw(ctx, input) {
            Ok((rest, parsed)) => {
                *input = rest;
                Ok(parsed)
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
    fn parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Self> {
        T::try_parse_raw(ctx, input)
    }
}

impl<T: ParseRev> ParseRev for Option<T> {
    #[inline(always)]
    fn parse_rev_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Self> {
        T::try_parse_rev_raw(ctx, input)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn desc() -> Cow<'static, str> {
        T::desc()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, Some(parsed)) = T::try_parse_raw(ctx, input)? else {
            return Ok((input.clone(), None));
        };
        vec.push(parsed);
        while let (rest_, Some(parsed)) = T::try_parse_raw(ctx, &rest)? {
            rest = rest_;
            vec.push(parsed);
        }
        Ok((rest, Some(vec)))
    }
}

impl<T: ParseRev> ParseRev for Vec<T> {
    fn try_parse_rev_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, Some(parsed)) = T::try_parse_rev_raw(ctx, input)? else {
            return Ok((input.clone(), None));
        };
        vec.push(parsed);
        while let (rest_, Some(parsed)) = T::try_parse_rev_raw(ctx, &rest)? {
            rest = rest_;
            vec.push(parsed);
        }
        Ok((rest, Some(vec)))
    }
}

struct Balanced<Open, Close> {
    span: Span,
    inner: Span,
    _ph: PhantomData<fn() -> (Open, Close)>,
}

impl<Open, Close> GetSpan for Balanced<Open, Close> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<Open: GetSpan + Parse, Close: GetSpan + Parse> Parse for Balanced<Open, Close> {
    fn desc() -> Cow<'static, str> {
        format!("balanced {}...{}", Open::desc(), Close::desc()).into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(open)) = Open::try_parse_raw(ctx, input)? {
            let inner_s = rest.start();
            let mut nesting = 1;
            loop {
                if let (rest_, Some(_)) = Open::try_parse_raw(ctx, &rest)? {
                    rest = rest_;
                    nesting += 1;
                } else if let (rest_, Some(close)) = Close::try_parse_raw(ctx, &rest)? {
                    let inner_e = rest.start();
                    rest = rest_;
                    nesting -= 1;
                    if nesting == 0 {
                        return Ok((
                            rest,
                            Some(Self {
                                span: open.span().join(&close.span()),
                                inner: inner_s.join(&inner_e),
                                _ph: PhantomData,
                            }),
                        ));
                    }
                } else if let Some(ch) = rest.chars().next() {
                    let (_, rest_) = rest.split_at(ch.len_utf8());
                    rest = rest_;
                } else {
                    return Err(ParseErr::new(
                        open.span(),
                        format!(
                            "no matching balanced {} for {}",
                            Close::desc(),
                            Open::desc(),
                        ),
                    ));
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

impl<Open: GetSpan, T, Close: GetSpan> GetSpan for Delimited<Open, T, Close> {
    fn span(&self) -> Span {
        self.open.span().join(&self.close.span())
    }
}

impl<Open: Parse, T: Parse, Close: Parse> Parse for Delimited<Open, T, Close> {
    fn desc() -> Cow<'static, str> {
        format!("{} {} {}", Open::desc(), T::desc(), Close::desc()).into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        if let Some(open) = Open::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            if let Some(value) = T::try_parse(ctx, &mut rest)? {
                WsAndComments::try_parse(ctx, &mut rest)?;
                let close = Close::parse(ctx, &mut rest)?;
                return Ok((rest, Some(Self { open, value, close })));
            }
        }
        Ok((input.clone(), None))
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut vec = Vec::new();
        let (mut rest, value) = T::try_parse_raw(ctx, input)?;
        if let Some(mut value) = value {
            while let (rest_, Some(punct)) = P::try_parse_raw(ctx, &rest.trim_wsc_start()?)? {
                rest = rest_;
                vec.push((value, Some(punct)));
                let (rest_, value_) = T::parse_raw(ctx, &rest.trim_wsc_start()?)?;
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
    #[allow(dead_code)]
    term: Term,
}

impl<T: Parse, Term: Parse> Parse for Terminated<T, Term> {
    fn desc() -> Cow<'static, str> {
        T::desc()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(value)) = T::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            if let Some(term) = Term::try_parse(ctx, &mut rest)? {
                return Ok((rest, Some(Self { value, term })));
            }
        }
        Ok((input.clone(), None))
    }
}

struct WsAndComments(#[allow(dead_code)] Span);

impl Parse for WsAndComments {
    fn desc() -> Cow<'static, str> {
        "whitespace or comments".into()
    }

    fn try_parse_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
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
    fn try_parse_rev_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_end()?;
        let rest_len = rest.len();
        if rest_len < input.len() {
            Ok((rest, Some(Self(input.slice(input.len() - rest_len..)))))
        } else {
            Ok((input.clone(), None))
        }
    }
}
