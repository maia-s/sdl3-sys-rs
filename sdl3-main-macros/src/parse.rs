use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub trait IntoTokenTrees: Sized {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>);

    fn into_token_stream(self) -> TokenStream {
        let mut ts = TokenStream::new();
        self.into_token_trees(&mut ts);
        ts
    }
}

impl<T: IntoTokenTrees + Clone> IntoTokenTrees for &T {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        self.clone().into_token_trees(out);
    }
}

impl<T: IntoTokenTrees + Clone> IntoTokenTrees for &mut T {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        self.clone().into_token_trees(out);
    }
}

impl<T: IntoTokenTrees> IntoTokenTrees for Option<T> {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        if let Some(value) = self {
            value.into_token_trees(out);
        }
    }
}

impl<T: IntoTokenTrees> IntoTokenTrees for Vec<T> {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        for i in self {
            i.into_token_trees(out);
        }
    }
}

impl IntoTokenTrees for &str {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        Literal::string(self).into_token_trees(out);
    }
}

impl IntoTokenTrees for u8 {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        Literal::u8_unsuffixed(self).into_token_trees(out);
    }
}

pub struct Error {
    span: Option<Span>,
    message: String,
}

impl Error {
    pub fn new(span: Option<Span>, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl IntoTokenTrees for Error {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        let span = self.span.unwrap_or(Span::call_site());
        for mut tt in format!("::core::compile_error!({:?});", self.message.as_str())
            .parse::<TokenStream>()
            .unwrap()
            .into_iter()
        {
            tt.set_span(span);
            out.extend([tt]);
        }
    }
}

pub trait Parse: Sized {
    fn desc() -> &'static str;

    fn parse(input: &mut &[TokenTree]) -> Result<Self, Error> {
        if let Some(parsed) = Self::try_parse(input)? {
            Ok(parsed)
        } else {
            Err(Error::new(
                input.first().map(|t| t.span()),
                format!("expected {}", Self::desc()),
            ))
        }
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        Self::parse(input).map(Some)
    }

    fn try_parse_required(input: &mut &[TokenTree], required: bool) -> Result<Option<Self>, Error> {
        if required {
            Ok(Some(Self::parse(input)?))
        } else {
            Ok(Self::try_parse(input)?)
        }
    }

    fn parse_all(input: &mut &[TokenTree]) -> Result<Self, Error> {
        let parsed = Self::parse(input)?;
        if input.is_empty() {
            Ok(parsed)
        } else {
            Err(Error::new(
                Some(input.first().unwrap().span()),
                format!("unexpected input after {}", Self::desc()),
            ))
        }
    }

    fn try_parse_all(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if input.is_empty() {
            return Ok(None);
        }
        Self::parse_all(input).map(Some)
    }
}

impl<T: Parse> Parse for Option<T> {
    fn desc() -> &'static str {
        T::desc()
    }

    fn parse(input: &mut &[TokenTree]) -> Result<Self, Error> {
        T::try_parse(input)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn desc() -> &'static str {
        T::desc()
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let mut vec = Vec::new();
        while let Some(parsed) = T::try_parse(input)? {
            vec.push(parsed);
        }
        Ok((!vec.is_empty()).then_some(vec))
    }
}

pub fn flatten_token_tree(tt: TokenTree) -> TokenTree {
    if let TokenTree::Group(group) = &tt {
        if group.delimiter() == Delimiter::None {
            let mut ts = group.stream().into_iter();
            if let Some(tt) = ts.next() {
                if ts.next().is_none() {
                    return flatten_token_tree(tt);
                }
            }
        }
    }
    tt
}

pub fn into_input(ts: TokenStream) -> Vec<TokenTree> {
    ts.into_iter().map(flatten_token_tree).collect()
}

fn parse_group(input: &mut &[TokenTree], delimiter: Delimiter) -> Result<Group, Error> {
    let expected = match delimiter {
        Delimiter::None => "expected none-delimited group",
        Delimiter::Brace => "expected `{` .. `}`",
        Delimiter::Bracket => "expected `[` .. `]`",
        Delimiter::Parenthesis => "expected `(` .. `)`",
    };
    let Some(TokenTree::Group(group)) = input.first() else {
        return Err(Error::new(input.first().map(|t| t.span()), expected));
    };
    if group.delimiter() != Delimiter::Brace {
        return Err(Error::new(Some(group.span()), expected));
    }
    *input = &input[1..];
    Ok(group.clone())
}

fn try_parse_kw(input: &mut &[TokenTree], kw: &str) -> Option<Ident> {
    if let Some(TokenTree::Ident(ident)) = input.first() {
        if ident.to_string() == kw {
            *input = &input[1..];
            return Some(ident.clone());
        }
    }
    None
}

/*
fn parse_kw(input: &mut &[TokenTree], kw: &str) -> Result<Ident, Error> {
    try_parse_kw(input, kw)
        .ok_or_else(|| Error::new(input.first().map(|t| t.span()), format!("expected `{kw}`")))
}
*/

fn try_parse_op(input: &mut &[TokenTree], op: &str) -> Option<Vec<Punct>> {
    let mut rest = *input;
    let mut op = op.as_bytes();
    let mut parsed = Vec::new();
    while let Some(TokenTree::Punct(punct)) = rest.first() {
        if punct.as_char() == char::from(op[0]) {
            parsed.push(punct.clone());
            rest = &rest[1..];
            op = &op[1..];
            if op.is_empty() {
                *input = rest;
                return Some(parsed);
            } else if punct.spacing() == Spacing::Alone {
                break;
            }
        } else {
            break;
        }
    }
    None
}

fn parse_op(input: &mut &[TokenTree], op: &str) -> Result<Vec<Punct>, Error> {
    try_parse_op(input, op)
        .ok_or_else(|| Error::new(input.first().map(|t| t.span()), format!("expected `{op}`")))
}

fn read_until_inclusive(
    input: &mut &[TokenTree],
    last: impl Fn(&TokenTree) -> bool,
) -> Result<Vec<TokenTree>, Error> {
    let mut vec = Vec::new();
    while let Some(tt) = input.first() {
        vec.push(tt.clone());
        if last(tt) {
            return Ok(vec);
        }
    }
    Err(Error::new(None, "unexpected end of input"))
}

fn read_balanced_angle_brackets_while(
    input: &mut &[TokenTree],
    accept: impl Fn(&TokenTree) -> bool,
) -> Result<Vec<TokenTree>, Error> {
    let mut depth = 0;
    let mut tts = Vec::new();
    let mut first_angle = None;
    while let Some(tt) = input.first() {
        if let TokenTree::Punct(punct) = tt {
            match punct.as_char() {
                '-' | '=' => {
                    if let Some(TokenTree::Punct(p2)) = input.get(1) {
                        if punct.spacing() == Spacing::Joint && p2.as_char() == '>' {
                            // don't count > in -> or => ops as angle bracket
                            *input = &input[2..];
                            tts.push(tt.clone());
                            tts.push(TokenTree::Punct(p2.clone()));
                            continue;
                        }
                    }
                }
                '<' => {
                    if depth == 0 && first_angle.is_some() && !accept(tt) {
                        break;
                    }
                    first_angle = Some(tt.span());
                    depth += 1;
                    *input = &input[1..];
                    tts.push(tt.clone());
                    continue;
                }
                '>' => {
                    depth -= 1;
                    *input = &input[1..];
                    tts.push(tt.clone());
                    continue;
                }
                _ => (),
            }
        }
        if depth != 0 || accept(tt) {
            *input = &input[1..];
            tts.push(tt.clone());
        } else {
            break;
        }
    }
    if depth == 0 {
        Ok(tts)
    } else {
        Err(Error::new(first_angle, "unterminated `<`"))
    }
}

#[derive(Clone)]
pub struct Attribute {
    meta: TokenStream,
}

impl IntoTokenTrees for Attribute {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #[#{self.meta}]);
    }
}

impl Parse for Attribute {
    fn desc() -> &'static str {
        "outer attribute"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if try_parse_op(input, "#").is_none() {
            return Ok(None);
        }
        let bracketed = parse_group(input, Delimiter::Bracket)?.stream();
        Ok(Some(Self { meta: bracketed }))
    }
}

#[derive(Clone)]
pub struct InnerAttribute {
    meta: TokenStream,
}

impl IntoTokenTrees for InnerAttribute {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #![#{self.meta}]);
    }
}

impl Parse for InnerAttribute {
    fn desc() -> &'static str {
        "inner attribute"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let try_input = &mut { *input };
        if try_parse_op(try_input, "#").is_none() {
            return Ok(None);
        }
        if try_parse_op(try_input, "!").is_none() {
            return Ok(None);
        }
        *input = try_input;
        let bracketed = parse_group(input, Delimiter::Bracket)?.stream();
        Ok(Some(Self { meta: bracketed }))
    }
}

#[derive(Clone)]
pub struct ConstItem {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub rest: Vec<TokenTree>,
}

impl IntoTokenTrees for ConstItem {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.attrs} #{self.vis} #{self.ident} #{self.rest})
    }
}

impl Parse for ConstItem {
    fn desc() -> &'static str {
        "const item"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let try_input = &mut { *input };
        let attrs = Vec::<Attribute>::try_parse(try_input)?.unwrap_or_default();
        let vis = Visibility::parse(try_input)?;
        if try_parse_kw(try_input, "const").is_some() {
            *input = try_input;
            let ident = Ident::parse(input)?;
            let rest = read_until_inclusive(
                input,
                |tt| matches!(tt, TokenTree::Punct(p) if p.as_char() == ';'),
            )?;
            Ok(Some(Self {
                attrs,
                vis,
                ident,
                rest,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct ExternAbi {
    pub span: Span,
    pub abi: String,
}

impl IntoTokenTrees for ExternAbi {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => extern #{self.abi});
    }
}

impl Parse for ExternAbi {
    fn desc() -> &'static str {
        "extern abi"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(extern_kw) = try_parse_kw(input, "extern") {
            let err = || {
                Err(Error::new(
                    Some(input.first().map(|t| t.span()).unwrap_or(extern_kw.span())),
                    "expected ABI string literal after `extern`",
                ))
            };
            let Some(TokenTree::Literal(lit)) = input.first() else {
                return err();
            };
            let mut lit = lit.to_string();
            if lit.len() < 2 || lit.as_bytes()[0] != b'"' || lit.as_bytes()[lit.len() - 1] != b'"' {
                return err();
            }
            lit.pop();
            lit.remove(0);
            *input = &input[1..];
            Ok(Some(Self {
                span: extern_kw.span(),
                abi: lit,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub unsafe_kw: Option<Ident>,
    pub abi: Option<ExternAbi>,
    pub params: Vec<Type>,
    pub return_type: Type,
}

impl IntoTokenTrees for FunctionSignature {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        let mut params = TokenStream::new();
        for param in self.params {
            miniquote_to!(&mut params => #param,);
        }
        miniquote_to!(out => #{self.unsafe_kw} #{self.abi} fn(#params));
        if !matches!(&self.return_type, Type::Tuple(v) if v.is_empty()) {
            miniquote_to!(out => -> #{self.return_type});
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafe_kw: Option<Ident>,
    pub abi: Option<ExternAbi>,
    pub ident: Ident,
    pub params: FunctionParams,
    pub return_type: Option<Type>,
    pub body: TokenTree,
}

impl Function {
    pub fn new(ident: Ident) -> Self {
        Self {
            attrs: Vec::new(),
            vis: Visibility::default(),
            unsafe_kw: None,
            abi: None,
            ident,
            params: FunctionParams::default(),
            return_type: None,
            body: TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new())),
        }
    }

    pub fn signature(&self) -> FunctionSignature {
        FunctionSignature {
            unsafe_kw: self.unsafe_kw.clone(),
            abi: self.abi.clone(),
            params: self.params.0.iter().map(|p| p.ty.clone()).collect(),
            return_type: self.return_type.clone().unwrap_or(Type::unit()),
        }
    }
}

impl IntoTokenTrees for Function {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.attrs} #{self.vis} #{self.unsafe_kw} #{self.abi} fn #{self.ident} #{self.params});
        if let Some(return_type) = self.return_type {
            miniquote_to!(out => -> #return_type);
        }
        self.body.into_token_trees(out);
    }
}

impl Parse for Function {
    fn desc() -> &'static str {
        "function"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let try_input = &mut *input;

        let attrs = Vec::<Attribute>::try_parse(try_input)?.unwrap_or_default();
        let vis = Visibility::parse(try_input)?;
        let unsafe_kw = try_parse_kw(try_input, "unsafe");
        let abi = ExternAbi::try_parse(try_input)?;
        if try_parse_kw(try_input, "fn").is_none() {
            return Ok(None);
        }

        *input = try_input;

        let ident = Ident::parse(input)?;
        let params = FunctionParams::parse(input)?;
        let return_type = if try_parse_op(input, "->").is_some() {
            let rtype = Type::parse(input)?;
            if matches!(&rtype, Type::Tuple(v) if v.is_empty()) {
                // unit return
                None
            } else {
                Some(rtype)
            }
        } else {
            None
        };
        let body = if let Some(body @ TokenTree::Group(group)) = input.first() {
            if group.delimiter() == Delimiter::Brace {
                *input = &input[1..];
                body.clone()
            } else {
                return Err(Error::new(Some(group.span()), "expected `{`"));
            }
        } else {
            return Err(Error::new(input.first().map(|t| t.span()), "expected `{`"));
        };
        Ok(Some(Self {
            attrs,
            vis,
            unsafe_kw,
            abi,
            ident,
            params,
            return_type,
            body,
        }))
    }
}

#[derive(Clone)]
pub struct FunctionParam {
    pub mut_kw: Option<Ident>,
    pub ident: Ident,
    pub ty: Type,
}

impl IntoTokenTrees for FunctionParam {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        if self.ident.to_string() == "self" {
            match self.ty {
                Type::SelfTy => return miniquote_to!(out => #{self.mut_kw} #{self.ident}),
                Type::Ref(lt, ty) if matches!(*ty, Type::SelfTy) => {
                    return miniquote_to!(out => & #lt #{self.ident})
                }
                Type::RefMut(lt, ty) if matches!(*ty, Type::SelfTy) => {
                    return miniquote_to!(out => & #lt mut #{self.ident})
                }
                _ => (),
            }
        }
        miniquote_to!(out => #{self.mut_kw} #{self.ident}: #{self.ty})
    }
}

impl Parse for FunctionParam {
    fn desc() -> &'static str {
        "function parameter"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let self_ref = try_parse_op(input, "&");
        let self_lt = if self_ref.is_some() {
            Lifetime::try_parse(input)?
        } else {
            None
        };
        let mut_kw = try_parse_kw(input, "mut");
        if let Some(ident) = Ident::try_parse(input)? {
            let ty = if ident.to_string() == "self" {
                if self_ref.is_some() {
                    if mut_kw.is_some() {
                        Type::RefMut(self_lt, Box::new(Type::SelfTy))
                    } else {
                        Type::Ref(self_lt, Box::new(Type::SelfTy))
                    }
                } else {
                    Type::SelfTy
                }
            } else {
                if self_ref.is_some() {
                    let span = Some(ident.span());
                    return if mut_kw.is_some() {
                        Err(Error::new(span, "expected `self` after `mut`"))
                    } else {
                        Err(Error::new(span, "expected `self` after `&`"))
                    };
                }
                parse_op(input, ":")?;
                Type::parse(input)?
            };
            Ok(Some(Self { mut_kw, ident, ty }))
        } else if let Some(mut_kw) = mut_kw {
            Err(Error::new(
                Some(mut_kw.span()),
                "expected ident after this `mut`",
            ))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct FunctionArgs(Vec<Ident>);

impl IntoTokenTrees for FunctionArgs {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        let mut ts = TokenStream::new();
        for arg in self.0 {
            miniquote_to!(&mut ts => #arg,);
        }
        Group::new(Delimiter::Parenthesis, ts).into_token_trees(out);
    }
}

#[derive(Clone, Default)]
pub struct FunctionParams(Vec<FunctionParam>);

impl FunctionParams {
    pub fn to_args(&self) -> FunctionArgs {
        FunctionArgs(self.iter().map(|p| p.ident.clone()).collect())
    }
}

impl Deref for FunctionParams {
    type Target = [FunctionParam];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FunctionParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoTokenTrees for FunctionParams {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        let mut ts = TokenStream::new();
        for param in self.0 {
            miniquote_to!(&mut ts => #param,);
        }
        Group::new(Delimiter::Parenthesis, ts).into_token_trees(out);
    }
}

impl Parse for FunctionParams {
    fn desc() -> &'static str {
        "function parameters"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(TokenTree::Group(params)) = input.first() {
            if params.delimiter() == Delimiter::Parenthesis {
                *input = &input[1..];
                let input = input!(params.stream());
                let mut params = Vec::new();
                while let Some(param) = FunctionParam::try_parse(input)? {
                    params.push(param);
                    if try_parse_op(input, ",").is_none() {
                        break;
                    }
                }
                if !input.is_empty() {
                    return Err(Error::new(
                        Some(input.first().unwrap().span()),
                        "expected function parameter",
                    ));
                }
                return Ok(Some(Self(params)));
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub enum GenericArg {
    Lifetime(Lifetime),
    Type(Type), // or constant
    Expr(TokenTree),
}

impl GenericArg {
    #[must_use]
    fn replace_self(&self, new_self: Type) -> Self {
        match self {
            Self::Lifetime(lt) => Self::Lifetime(lt.clone()),
            Self::Type(t) => Self::Type(t.replace_self(new_self)),
            Self::Expr(e) => Self::Expr(e.clone()),
        }
    }
}

impl IntoTokenTrees for GenericArg {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        match self {
            Self::Lifetime(lt) => lt.into_token_trees(out),
            Self::Type(t) => t.into_token_trees(out),
            Self::Expr(expr) => miniquote_to!(out => { #expr }),
        }
    }
}

impl Parse for GenericArg {
    fn desc() -> &'static str {
        "generic argument"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(lt) = Lifetime::try_parse(input)? {
            return Ok(Some(Self::Lifetime(lt)));
        } else if let Some(t) = Type::try_parse(input)? {
            return Ok(Some(Self::Type(t)));
        } else if let Some(tt @ TokenTree::Group(group)) = input.first() {
            if group.delimiter() == Delimiter::Brace {
                *input = &input[1..];
                return Ok(Some(Self::Expr(tt.clone())));
            }
        } else if let Some(tt @ TokenTree::Literal(_)) = input.first() {
            *input = &input[1..];
            return Ok(Some(Self::Expr(tt.clone())));
        }
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct GenericArgs {
    pub args: Vec<GenericArg>,
}

impl GenericArgs {
    #[must_use]
    fn replace_self(&self, new_self: Type) -> Self {
        Self {
            args: self
                .args
                .iter()
                .map(|arg| arg.replace_self(new_self.clone()))
                .collect(),
        }
    }
}

impl Deref for GenericArgs {
    type Target = [GenericArg];

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl IntoTokenTrees for GenericArgs {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => ::<);
        for arg in self.args {
            miniquote_to!(out => #arg,);
        }
        miniquote_to!(out => >);
    }
}

impl Parse for GenericArgs {
    fn desc() -> &'static str {
        "generic arguments"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if try_parse_op(input, "<").is_some() {
            let mut args = Vec::new();
            while let Some(arg) = GenericArg::try_parse(input)? {
                args.push(arg);
                if try_parse_op(input, ",").is_none() {
                    break;
                }
            }
            parse_op(input, ">")?;
            Ok(Some(Self { args }))
        } else {
            Ok(None)
        }
    }
}

impl IntoTokenTrees for Group {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend([TokenTree::Group(self)]);
    }
}

impl IntoTokenTrees for Ident {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend([TokenTree::Ident(self)]);
    }
}

impl Parse for Ident {
    fn desc() -> &'static str {
        "ident"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(TokenTree::Ident(ident)) = input.first() {
            *input = &input[1..];
            Ok(Some(ident.clone()))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct ImplBlock {
    pub attrs: Vec<Attribute>,
    pub generic_defs: Option<GenericArgs>,
    pub ty: Type,
    pub inner_attrs: Vec<InnerAttribute>,
    pub items: Vec<Item>,
}

impl IntoTokenTrees for ImplBlock {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to! { out =>
            #{self.attrs}
            impl #{self.generic_defs} #{self.ty} {
                #{self.inner_attrs}
                #{self.items}
            }
        }
    }
}

impl Parse for ImplBlock {
    fn desc() -> &'static str {
        "impl block"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let try_input = &mut { *input };
        let attrs = Vec::<Attribute>::try_parse(try_input)?.unwrap_or_default();
        if try_parse_kw(try_input, "impl").is_some() {
            *input = try_input;
            let generic_defs = GenericArgs::try_parse(input)?;
            let ty = Type::parse(input)?;
            let braced = input!(parse_group(input, Delimiter::Brace)?.stream());
            let inner_attrs = Vec::<InnerAttribute>::try_parse(braced)?.unwrap_or_default();
            let items = Vec::<Item>::try_parse_all(braced)?.unwrap_or_default();
            Ok(Some(Self {
                attrs,
                generic_defs,
                ty,
                inner_attrs,
                items,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub enum Item {
    Const(ConstItem),
    Function(Function),
    TypeAlias(TypeAlias),
}

impl IntoTokenTrees for Item {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        match self {
            Self::Const(item) => item.into_token_trees(out),
            Self::Function(item) => item.into_token_trees(out),
            Self::TypeAlias(item) => item.into_token_trees(out),
        }
    }
}

impl Parse for Item {
    fn desc() -> &'static str {
        "item"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(function) = Function::try_parse(input)? {
            Ok(Some(Self::Function(function)))
        } else if let Some(const_item) = ConstItem::try_parse(input)? {
            Ok(Some(Self::Const(const_item)))
        } else if let Some(alias) = TypeAlias::try_parse(input)? {
            Ok(Some(Self::TypeAlias(alias)))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lifetime {
    apo: Punct,
    ident: Ident,
}

impl IntoTokenTrees for Lifetime {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.apo}#{self.ident});
    }
}

impl Parse for Lifetime {
    fn desc() -> &'static str {
        "lifetime"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        if let Some(mut apo) = try_parse_op(input, "'") {
            let ident = Ident::parse(input)?;
            Ok(Some(Self {
                apo: apo.remove(0),
                ident,
            }))
        } else {
            Ok(None)
        }
    }
}

impl IntoTokenTrees for Literal {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend([TokenTree::Literal(self)]);
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub global: bool,
    pub segments: Vec<PathSeg>,
}

impl Path {
    pub fn last_segment_generics(&self) -> Option<&GenericArgs> {
        self.segments.last().unwrap().generics.as_ref()
    }

    pub fn last_segment_eq_no_gen(&self, cmp: &str) -> bool {
        let seg = &self.segments.last().unwrap();
        seg.generics.is_none() && seg.ident.to_string() == cmp
    }

    #[must_use]
    pub fn replace_self(&self, new_self: Type) -> Self {
        Self {
            global: self.global,
            segments: self
                .segments
                .iter()
                .map(|s| s.replace_self(new_self.clone()))
                .collect(),
        }
    }
}

impl FromStr for Path {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();
        let mut global = false;
        let mut segments = Vec::new();
        if let Some(s2) = s.strip_prefix("::") {
            global = true;
            s = s2.trim_start();
        }
        for s in s.split("::") {
            let s = s.trim();
            segments.push(PathSeg {
                ident: Ident::new(s, Span::mixed_site()),
                generics: None,
            })
        }
        Ok(Self { global, segments })
    }
}

impl IntoTokenTrees for Path {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        let mut sep = self.global;
        for seg in self.segments {
            if sep {
                miniquote_to!(out => ::);
            }
            sep = true;
            miniquote_to!(out => #seg);
        }
    }
}

impl Parse for Path {
    fn desc() -> &'static str {
        "path"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let global = try_parse_op(input, "::").is_some();
        if let Some(ident) = Ident::try_parse_required(input, global)? {
            let mut segments = vec![PathSeg {
                ident,
                generics: None,
            }];
            loop {
                let mut sep = try_parse_op(input, "::").is_some();
                if matches!(input.first(), Some(TokenTree::Punct(p)) if p.as_char() == '<') {
                    segments.last_mut().unwrap().generics = Some(GenericArgs::parse(input)?);
                    sep = try_parse_op(input, "::").is_some();
                }
                if !sep {
                    break;
                }
                segments.push(PathSeg {
                    ident: Ident::parse(input)?,
                    generics: None,
                });
            }
            Ok(Some(Self { global, segments }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathSeg {
    pub ident: Ident,
    pub generics: Option<GenericArgs>,
}

impl PathSeg {
    #[must_use]
    fn replace_self(&self, new_self: Type) -> Self {
        Self {
            ident: self.ident.clone(),
            generics: self.generics.as_ref().map(|g| g.replace_self(new_self)),
        }
    }
}

impl IntoTokenTrees for PathSeg {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.ident} #{self.generics});
    }
}

impl IntoTokenTrees for Punct {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend([TokenTree::Punct(self)]);
    }
}

impl IntoTokenTrees for TokenStream {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend(self);
    }

    fn into_token_stream(self) -> TokenStream {
        self
    }
}

impl IntoTokenTrees for TokenTree {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend([self]);
    }
}

impl IntoTokenTrees for &[TokenTree] {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend(self.iter().cloned());
    }
}

impl IntoTokenTrees for &mut [TokenTree] {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        out.extend(self.iter().cloned());
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum TypeClass {
    Val = 1,
    Ref = 2,
    RefMut = 3,
    OptVal = 4 | 1,
    OptRef = 4 | 2,
    OptRefMut = 4 | 3,
}

#[derive(Clone, Debug)]
pub enum Type {
    Ptr(Box<Type>),
    PtrMut(Box<Type>),
    Ref(Option<Lifetime>, Box<Type>),
    RefMut(Option<Lifetime>, Box<Type>),
    Tuple(Vec<Type>),
    SelfTy,
    Path(Path),
    Other(Vec<TokenTree>),
}

impl Type {
    pub fn unit() -> Self {
        Self::Tuple(Vec::new())
    }

    pub fn classify(&self) -> Result<TypeClass, Error> {
        match self {
            Type::Ref(_, _) => Ok(TypeClass::Ref),
            Type::RefMut(_, _) => Ok(TypeClass::RefMut),
            Type::Path(path) => {
                let seg = path.segments.last().unwrap();
                if seg.ident.to_string() == "Option" {
                    let Some(generics) = &seg.generics else {
                        return Err(Error::new(
                            Some(seg.ident.span()),
                            "`Option` is missing generics",
                        ));
                    };
                    let generics = &generics.args;
                    if generics.len() != 1 {
                        return Err(Error::new(
                            Some(seg.ident.span()),
                            format!(
                                "`Option` has too {} generic arguments",
                                if generics.len() > 1 { "many" } else { "few" }
                            ),
                        ));
                    }
                    let GenericArg::Type(t) = &generics[0] else {
                        return Err(Error::new(
                            Some(seg.ident.span()),
                            "the generic argument to `Option` isn't a type",
                        ));
                    };
                    match t {
                        Type::Ref(_, _) => Ok(TypeClass::OptRef),
                        Type::RefMut(_, _) => Ok(TypeClass::OptRefMut),
                        _ => Ok(TypeClass::OptVal),
                    }
                } else {
                    Ok(TypeClass::Val)
                }
            }
            _ => Ok(TypeClass::Val),
        }
    }

    pub fn path_generics(&self) -> Option<&GenericArgs> {
        let Type::Path(t) = self else { return None };
        t.last_segment_generics()
    }

    pub fn is_ident_no_gen(&self, ident: &str) -> bool {
        let Type::Path(t) = self else { return false };
        t.last_segment_eq_no_gen(ident)
    }

    #[must_use]
    pub fn replace_self(&self, new_self: Type) -> Type {
        match self {
            Self::Ptr(t) => Self::Ptr(Box::new(t.replace_self(new_self))),
            Self::PtrMut(t) => Self::PtrMut(Box::new(t.replace_self(new_self))),
            Self::Ref(lt, t) => Self::Ref(lt.clone(), Box::new(t.replace_self(new_self))),
            Self::RefMut(lt, t) => Self::RefMut(lt.clone(), Box::new(t.replace_self(new_self))),
            Self::Tuple(t) => {
                Self::Tuple(t.iter().map(|t| t.replace_self(new_self.clone())).collect())
            }
            Self::SelfTy => new_self,
            Self::Path(p) => Self::Path(p.replace_self(new_self.clone())),
            Self::Other(_) => self.clone(),
        }
    }
}

impl IntoTokenTrees for Type {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        match self {
            Self::Ptr(t) => miniquote_to!(out => *const #t),
            Self::PtrMut(t) => miniquote_to!(out => *mut #t),
            Self::Ref(lt, t) => miniquote_to!(out => & #lt #t),
            Self::RefMut(lt, t) => miniquote_to!(out => & #lt mut #t),
            Self::Tuple(t) => {
                let mut ts = TokenStream::new();
                for t in t {
                    miniquote_to!(&mut ts => #t,);
                }
                Group::new(Delimiter::Parenthesis, ts).into_token_trees(out);
            }
            Self::SelfTy => miniquote_to!(out => Self),
            Self::Path(t) => t.into_token_trees(out),
            Self::Other(t) => t.into_token_trees(out),
        }
    }
}

impl Parse for Type {
    fn desc() -> &'static str {
        "type"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let Some(tt) = input.first() else {
            return Ok(None);
        };

        match tt {
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Parenthesis => {
                        *input = &input[1..];
                        let input = input!(group.stream());
                        let mut types = Vec::new();
                        let mut trailing_comma = false;
                        while let Some(ty) = Type::try_parse(input)? {
                            types.push(ty);
                            trailing_comma = try_parse_op(input, ",").is_some();
                            if !trailing_comma {
                                break;
                            }
                        }
                        if !input.is_empty() {
                            return Err(Error::new(
                                Some(input.first().unwrap().span()),
                                if trailing_comma {
                                    "expected type or `)`"
                                } else {
                                    "expected type, `,` or `)`"
                                },
                            ));
                        }
                        if types.len() == 1 && !trailing_comma {
                            // this is just a type in parens
                            return Ok(Some(types.remove(0)));
                        } else {
                            return Ok(Some(Type::Tuple(types)));
                        }
                    }
                    Delimiter::Brace => return Ok(None),
                    _ => (),
                }
            }

            TokenTree::Ident(i) => {
                if i.to_string() == "Self" {
                    *input = &input[1..];
                    return Ok(Some(Type::SelfTy));
                } else {
                    return Ok(Some(Type::Path(Path::parse(input)?)));
                }
            }

            TokenTree::Punct(punct) => match punct.as_char() {
                '*' => {
                    *input = &input[1..];
                    if try_parse_kw(input, "const").is_some() {
                        return Ok(Some(Type::Ptr(Box::new(Type::parse(input)?))));
                    } else if try_parse_kw(input, "mut").is_some() {
                        return Ok(Some(Type::PtrMut(Box::new(Type::parse(input)?))));
                    } else {
                        return Err(Error::new(
                            Some(input.first().map(|t| t.span()).unwrap_or(punct.span())),
                            "expected `const` or `mut` after `*`",
                        ));
                    }
                }
                '&' => {
                    *input = &input[1..];
                    let lifetime = Lifetime::try_parse(input)?;
                    if try_parse_kw(input, "mut").is_some() {
                        return Ok(Some(Type::RefMut(lifetime, Box::new(Type::parse(input)?))));
                    } else {
                        return Ok(Some(Type::Ref(lifetime, Box::new(Type::parse(input)?))));
                    }
                }
                ':' => {
                    if punct.spacing() == Spacing::Joint
                        && matches!(input.get(1), Some(TokenTree::Punct(p2)) if p2.as_char() == ':')
                    {
                        return Ok(Some(Type::Path(Path::parse(input)?)));
                    }
                }
                ',' | ';' | '>' => return Ok(None),
                _ => (),
            },

            _ => (),
        }

        let Ok(tts) = read_balanced_angle_brackets_while(input, |tt| match tt {
            TokenTree::Group(group) => group.delimiter() != Delimiter::Brace,
            TokenTree::Punct(punct) => !matches!(punct.as_char(), ',' | ';' | '<' | '>'),
            _ => true,
        }) else {
            return Ok(None);
        };
        Ok(Some(Type::Other(tts)))
    }
}

#[derive(Clone)]
pub struct TypeAlias {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub rest: Vec<TokenTree>,
}

impl IntoTokenTrees for TypeAlias {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.attrs} #{self.vis} #{self.ident} #{self.rest})
    }
}

impl Parse for TypeAlias {
    fn desc() -> &'static str {
        "type alias"
    }

    fn try_parse(input: &mut &[TokenTree]) -> Result<Option<Self>, Error> {
        let try_input = &mut { *input };
        let attrs = Vec::<Attribute>::try_parse(try_input)?.unwrap_or_default();
        let vis = Visibility::parse(try_input)?;
        if try_parse_kw(try_input, "type").is_some() {
            *input = try_input;
            let ident = Ident::parse(input)?;
            let rest = read_until_inclusive(
                input,
                |tt| matches!(tt, TokenTree::Punct(p) if p.as_char() == ';'),
            )?;
            Ok(Some(Self {
                attrs,
                vis,
                ident,
                rest,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Default)]
pub struct Visibility {
    pub pub_kw: Option<Ident>,
    args: Option<Group>,
}

impl IntoTokenTrees for Visibility {
    fn into_token_trees(self, out: &mut impl Extend<TokenTree>) {
        miniquote_to!(out => #{self.pub_kw}#{self.args});
    }
}

impl Parse for Visibility {
    fn desc() -> &'static str {
        "visibility"
    }

    fn parse(input: &mut &[TokenTree]) -> Result<Self, Error> {
        if let Some(pub_kw) = try_parse_kw(input, "pub") {
            *input = &input[1..];
            let args = if let Some(TokenTree::Group(args)) = input.first() {
                if args.delimiter() == Delimiter::Parenthesis {
                    *input = &input[1..];
                    Some(args)
                } else {
                    None
                }
            } else {
                None
            };
            Ok(Self {
                pub_kw: Some(pub_kw.clone()),
                args: args.cloned(),
            })
        } else {
            Ok(Self {
                pub_kw: None,
                args: None,
            })
        }
    }
}
