use super::{GetSpan, IdentOrKw, Parse, ParseContext, ParseRawRes, Span};
use const_it::{expect_some, slice_strip_prefix};
use std::{
    borrow::Cow,
    collections::HashSet,
    fmt::{self, Debug},
    sync::OnceLock,
};

macro_rules! def_kws {
    ($($kw:ident),* $(,)*) => {
        pub const KEYWORDS: &[&str] = &[ $( kwstr(stringify!($kw)) ),* ];
        static KEYWORDS_SET: OnceLock<HashSet<&str>> = OnceLock::new();

        pub fn is_keyword(str: impl AsRef<str>) -> bool {
            fn is(str: &str) -> bool {
                KEYWORDS_SET.get_or_init(|| HashSet::from_iter(KEYWORDS.iter().copied())).contains(str)
            }
            is(str.as_ref())
        }

        def_kws!(@ 0; $($kw,)*);
    };

    (@ $n:expr;) => {};

    (@ $n:expr; $kw:ident, $($rest:tt)*) => {
        #[allow(non_camel_case_types, unused)]
        pub type $kw = Keyword::<{$n}>;
        def_kws!(@ $n+1; $($rest)*);
    };
}

const fn kwstr(str: &str) -> &str {
    expect_some!(
        slice_strip_prefix!(str, "Kw_"),
        "ident didn't start with `Kw_`"
    )
}

def_kws! {
    Kw__Alignas,
    Kw__Alignof,
    Kw__Atomic,
    Kw__BitInt,
    Kw__Bool,
    Kw__Complex,
    Kw__Decimal128,
    Kw__Decimal32,
    Kw__Decimal64,
    Kw__Generic,
    Kw__Imaginary,
    Kw__Noreturn,
    Kw__Static_assert,
    Kw__Thread_local,
    Kw_alignas,
    Kw_alignof,
    Kw_auto,
    Kw_bool,
    Kw_break,
    Kw_case,
    Kw_char,
    Kw_const,
    Kw_constexpr,
    Kw_continue,
    Kw_default,
    Kw_do,
    Kw_double,
    Kw_else,
    Kw_enum,
    Kw_extern,
    Kw_false,
    Kw_float,
    Kw_for,
    Kw_goto,
    Kw_if,
    Kw_inline,
    Kw_int,
    Kw_long,
    Kw_nullptr,
    Kw_register,
    Kw_restrict,
    Kw_return,
    Kw_short,
    Kw_signed,
    Kw_sizeof,
    Kw_static_assert,
    Kw_static,
    Kw_struct,
    Kw_switch,
    Kw_thread_local,
    Kw_true,
    Kw_typedef,
    Kw_typeof_unqual,
    Kw_typeof,
    Kw_union,
    Kw_unsigned,
    Kw_void,
    Kw_volatile,
    Kw_while,
}

#[derive(Clone)]
pub struct Keyword<const KW_INDEX: usize> {
    pub span: Span,
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(ident)) =
            IdentOrKw::try_parse_raw_if(ctx, input, |i| i.as_str() == KEYWORDS[KW_INDEX])?
        {
            Ok((rest, Some(Keyword { span: ident.span })))
        } else {
            Ok((input.clone(), None))
        }
    }
}
