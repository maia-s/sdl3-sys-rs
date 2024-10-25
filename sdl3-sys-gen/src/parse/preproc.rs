use crate::parse::is_keyword;

use super::{
    patch_parsed_define, Ambiguous, Cast, Delimited, DocComment, DocCommentPost, Expr, GetSpan,
    Ident, IdentOrKw, IntegerLiteral, Item, Items, Literal, Op, Parse, ParseContext, ParseErr,
    ParseRawRes, Punctuated, Span, Type, WsAndComments,
};
use core::mem;
use std::borrow::Cow;

fn skip_ifdef(str: &str) -> bool {
    matches!(str, "__cplusplus" | "SDL_THREAD_SAFETY_ANALYSIS")
}

#[derive(Clone, Debug)]
pub struct Define {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub ident: IdentOrKw,
    pub args: Option<Vec<DefineArg>>,
    pub value: DefineValue,
}

impl GetSpan for Define {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug)]
pub struct DefineArg {
    pub ident: IdentOrKw,
    pub ty: Type,
}

impl DefineArg {
    pub fn new(ident: IdentOrKw, ty: Type) -> Self {
        Self { ident, ty }
    }
}

#[derive(Clone, Debug)]
pub enum DefineValue {
    Expr(Expr),
    Type(Type),
    Items(Items),
    Other(Span),
    TargetDependent,
    Empty,
    ExprFollowedBy(Expr, Box<DefineValue>),
}

impl DefineValue {
    pub fn one() -> Self {
        Self::Expr(Expr::Literal(Literal::Integer(IntegerLiteral::one())))
    }

    pub fn parse_expr(s: &str) -> Result<Self, ParseErr> {
        let s = Span::new_inline(s.to_string());
        Ok(Self::Expr(Expr::parse_all(
            &ParseContext::new(None),
            s.trim_wsc()?,
        )?))
    }

    pub const fn is_target_dependent(&self) -> bool {
        matches!(self, Self::TargetDependent)
    }

    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    #[must_use]
    pub fn cast_expr(&self, ty: Type) -> Self {
        match self {
            DefineValue::Expr(expr) => DefineValue::Expr(Expr::Cast(Box::new(Cast {
                span: Span::none(),
                ty,
                expr: expr.clone(),
            }))),
            _ => todo!(),
        }
    }
}

impl Parse for DefineValue {
    fn desc() -> Cow<'static, str> {
        "define value".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if input.is_empty() {
            Ok((input.end(), Some(Self::Empty)))
        } else if input.contains("##") || input.contains("_cast<") {
            Ok((input.end(), Some(Self::Other(input.clone()))))
        } else {
            let mut items = Items::try_parse_try_all(ctx, input)?;
            let expr = Expr::try_parse_try_all(ctx, input)?;
            let ty = Type::try_parse_try_all(ctx, input)?;
            if let Some(i) = &items {
                if i.0.len() == 1 && matches!(i.0[0], Item::FnCall(_)) {
                    // skip FnCall item as it'll have been parsed as Expr::FnCall too
                    items = None;
                }
            }
            if items.is_some() as usize + expr.is_some() as usize + ty.is_some() as usize > 1 {
                let mut ambiguous = Ambiguous::new(input.clone());
                if let Some(items) = items {
                    ambiguous.push_items(items);
                }
                if let Some(expr) = expr {
                    ambiguous.push_expr(expr);
                }
                if let Some(ty) = ty {
                    ambiguous.push_ty(ty);
                }
                Ok((input.end(), Some(Self::Expr(Expr::Ambiguous(ambiguous)))))
            } else if let Some(items) = items {
                Ok((input.end(), Some(Self::Items(items))))
            } else if let Some(expr) = expr {
                Ok((input.end(), Some(Self::Expr(expr))))
            } else if let Some(ty) = ty {
                Ok((input.end(), Some(Self::Type(ty))))
            } else {
                if let (mut rest, Some(expr)) = Expr::try_parse_raw(ctx, input)? {
                    WsAndComments::try_parse(ctx, &mut rest)?;
                    if let Some(follow) = Self::try_parse(ctx, &mut rest)? {
                        return Ok((rest, Some(Self::ExprFollowedBy(expr, Box::new(follow)))));
                    }
                }
                Err(ParseErr::new(input.start(), "can't parse define value"))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Include {
    pub span: Span,
    pub kind: IncludeKind,
    pub path: Span,
}

impl GetSpan for Include {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug)]
pub enum IncludeKind {
    Local,
    System,
}

pub struct Line {
    span: Span,
}

impl Parse for Line {
    fn desc() -> Cow<'static, str> {
        "line".into()
    }

    fn parse_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Self> {
        let input = &input.trim_wsc_start()?;
        let mut escaped = false;
        let (rest, span) = 'parse: {
            for (i, ch) in input.char_indices() {
                if ch == '\n' && !escaped {
                    let rest = input.slice(i + 1..);
                    let span = input.slice(..i);
                    break 'parse (rest, span);
                }
                escaped = ch == '\\';
            }
            break 'parse (input.end(), input.clone());
        };
        let span = span.trim_wsc_end()?;
        Ok((rest, Line { span }))
    }
}

#[derive(Clone, Debug)]
pub struct PreProcBlock<const ALLOW_INITIAL_ELSE: bool = false> {
    pub span: Span,
    pub cond_expr: Option<ConditionalExpr>,
    pub block: Items,
    pub else_block: Option<Box<PreProcBlock<true>>>,
}

impl<const ALLOW_INITIAL_ELSE: bool> GetSpan for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Conditional {
    pub not: Vec<ConditionalExpr>,
    pub require: Option<ConditionalExpr>,
}

impl Conditional {
    pub fn new() -> Self {
        Self {
            not: Vec::new(),
            require: None,
        }
    }

    pub fn push(&mut self, mut cond_expr: Option<ConditionalExpr>) {
        mem::swap(&mut self.require, &mut cond_expr);
        if let Some(req) = cond_expr {
            self.not.push(req);
        } else {
            assert!(self.not.is_empty())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.not.is_empty() && self.require.is_none()
    }
}

#[derive(Clone, Debug)]
pub enum ConditionalExpr {
    If(Expr),
    IfDef(Ident),
    IfNDef(Ident),
}

impl<const ALLOW_INITIAL_ELSE: bool> Parse for PreProcBlock<ALLOW_INITIAL_ELSE> {
    fn desc() -> Cow<'static, str> {
        "preprocessor block".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(line)) = PreProcLine::try_parse_raw(ctx, input)? {
            let span0 = line.span;

            let (cond_expr, is_else) = match line.kind {
                PreProcLineKind::If(expr) => (Some(ConditionalExpr::If(expr)), false),
                PreProcLineKind::IfDef(ident) => (Some(ConditionalExpr::IfDef(ident)), false),
                PreProcLineKind::IfNDef(ident) => (Some(ConditionalExpr::IfNDef(ident)), false),

                PreProcLineKind::ElIf(expr) => (Some(ConditionalExpr::If(expr)), true),
                PreProcLineKind::ElIfDef(ident) => (Some(ConditionalExpr::IfDef(ident)), true),
                PreProcLineKind::ElIfNDef(ident) => (Some(ConditionalExpr::IfNDef(ident)), true),
                PreProcLineKind::Else => (None, true),

                PreProcLineKind::EndIf
                | PreProcLineKind::Define(_)
                | PreProcLineKind::Undef(_)
                | PreProcLineKind::Include(_)
                | PreProcLineKind::Pragma(_)
                | PreProcLineKind::Error(_)
                | PreProcLineKind::Warning(_) => return Ok((input.clone(), None)),
            };

            if !ALLOW_INITIAL_ELSE && is_else {
                return Ok((input.clone(), None));
            }

            let block_start = rest.start();

            while !rest.is_empty() {
                // skip doc comments so the end of one doesn't break Line
                if DocComment::try_parse(ctx, &mut rest)?.is_some() {
                } else if let (rest_, Some(line)) = Line::try_parse_raw(ctx, &rest)? {
                    let span = line.span.clone();
                    let mut line = line.span;
                    if !line.as_str().contains('#') && line.as_str().contains("/**<") {
                        // skip postfix doc comment
                        let pos = rest
                            .as_str()
                            .as_bytes()
                            .windows(4)
                            .position(|b| b == b"/**<")
                            .unwrap();
                        rest = rest.slice(pos..);
                        DocCommentPost::parse(ctx, &mut rest)?;
                        continue;
                    }
                    if let Some(pp) = PreProcLine::try_parse(ctx, &mut line)? {
                        match pp.kind {
                            PreProcLineKind::If(_)
                            | PreProcLineKind::IfDef(_)
                            | PreProcLineKind::IfNDef(_) => {
                                // skip nested if block
                                PreProcBlock::<false>::parse(ctx, &mut rest)?;
                                continue;
                            }

                            PreProcLineKind::ElIf(_)
                            | PreProcLineKind::ElIfDef(_)
                            | PreProcLineKind::ElIfNDef(_)
                            | PreProcLineKind::Else => {
                                let block = block_start.join(&rest.start());
                                let block = match &cond_expr {
                                    None => {
                                        return Err(ParseErr::new(
                                            pp.span,
                                            "expected `#endif` after `#else`, got another else",
                                        ))
                                    }

                                    Some(ConditionalExpr::IfDef(i)) if skip_ifdef(i.as_str()) => {
                                        Items(vec![Item::Skipped(block)])
                                    }

                                    _ => Items::try_parse_all(ctx, block.trim_wsc()?)?
                                        .unwrap_or_default(),
                                };
                                let (rest, else_block) =
                                    PreProcBlock::<true>::parse_raw(ctx, &rest)?;
                                let span1 = else_block.span();
                                return Ok((
                                    rest,
                                    Some(Self {
                                        span: span0.start().join(&span1.end()),
                                        cond_expr,
                                        block,
                                        else_block: Some(Box::new(else_block)),
                                    }),
                                ));
                            }

                            PreProcLineKind::EndIf => {
                                let block = block_start.join(&rest.start());
                                let block = match &cond_expr {
                                    Some(ConditionalExpr::IfDef(i)) if skip_ifdef(i.as_str()) => {
                                        Items(vec![Item::Skipped(block)])
                                    }

                                    _ => Items::try_parse_all(ctx, block.trim_wsc()?)?
                                        .unwrap_or_default(),
                                };
                                let rest = rest_;
                                return Ok((
                                    rest,
                                    Some(Self {
                                        span: span0.start().join(&span.end()),
                                        cond_expr,
                                        block,
                                        else_block: None,
                                    }),
                                ));
                            }

                            PreProcLineKind::Define(_)
                            | PreProcLineKind::Undef(_)
                            | PreProcLineKind::Include(_)
                            | PreProcLineKind::Pragma(_)
                            | PreProcLineKind::Error(_)
                            | PreProcLineKind::Warning(_) => {
                                rest = rest_;
                                continue;
                            }
                        }
                    } else {
                        rest = rest_;
                    }
                } else {
                    break;
                }
            }

            Err(ParseErr::new(span0, "unterminated #if"))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Debug)]
pub struct PreProcLine {
    pub span: Span,
    pub kind: PreProcLineKind,
}

#[derive(Debug)]
pub enum PreProcLineKind {
    If(Expr),
    IfDef(Ident),
    IfNDef(Ident),
    ElIf(Expr),
    ElIfDef(Ident),
    ElIfNDef(Ident),
    Else,
    EndIf,
    Define(Define),
    Undef(Ident),
    Include(Include),
    Pragma(Span),
    Error(Span),
    Warning(#[allow(dead_code)] Span),
}

impl Parse for PreProcLine {
    fn desc() -> Cow<'static, str> {
        "preprocessor directive".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let (rest, doc) = DocComment::try_parse_raw(ctx, input)?;
        if doc.is_some() && !rest.starts_with_ch('#') {
            // detached doc comment
            return Ok((input.clone(), None));
        }
        let (rest, line) = Line::parse_raw(ctx, &rest)?;
        let span = line.span;
        let line = span.trim_wsc()?;
        if let Some(i) = line.strip_prefix_ch('#') {
            let mut i = i.trim_wsc_start()?;
            let ident = IdentOrKw::parse(ctx, &mut i)
                .map_err(|e| e.map_msg("expected preprocessor directive"))?;
            WsAndComments::try_parse(ctx, &mut i)?;

            let kind = match ident.as_str() {
                "if" => PreProcLineKind::If(Expr::parse_all(ctx, i)?),
                "ifdef" => PreProcLineKind::IfDef(Ident::parse_all(ctx, i)?),
                "ifndef" => PreProcLineKind::IfNDef(Ident::parse_all(ctx, i)?),

                "elif" => PreProcLineKind::ElIf(Expr::parse_all(ctx, i)?),
                "elifdef" => PreProcLineKind::ElIfDef(Ident::parse_all(ctx, i)?),
                "enifndef" => PreProcLineKind::ElIfNDef(Ident::parse_all(ctx, i)?),

                "else" => PreProcLineKind::Else,
                "endif" => PreProcLineKind::EndIf,

                "define" => {
                    let ident = IdentOrKw::parse(ctx, &mut i)?;
                    if let Some(del) = Delimited::<
                        Op<'('>,
                        Option<Punctuated<IdentOrKw, Op![,]>>,
                        Op<')'>,
                    >::try_parse(ctx, &mut i)?
                    {
                        let _pi = ctx.patch_idents_state_guard();
                        let args = del
                            .value
                            .unwrap_or_default()
                            .0
                            .into_iter()
                            .map(|(ident, _)| {
                                let ident = if is_keyword(ident.as_str()) {
                                    let replacement = IdentOrKw::new_inline(format!("{}_", ident));
                                    ctx.add_patch_ident(ident, replacement.clone());
                                    replacement
                                } else {
                                    ident
                                };
                                DefineArg {
                                    ident,
                                    ty: Type::infer(),
                                }
                            })
                            .collect();
                        WsAndComments::try_parse(ctx, &mut i)?;
                        let doc = DocComment::try_parse_rev_combine_postfix(ctx, &mut i, doc)?;
                        let value = DefineValue::parse_all(ctx, i.trim_wsc_end()?)?;
                        let mut define = Define {
                            span: span.clone(),
                            doc,
                            ident,
                            args: Some(args),
                            value,
                        };
                        patch_parsed_define(ctx, &mut define)?;
                        PreProcLineKind::Define(define)
                    } else {
                        WsAndComments::try_parse(ctx, &mut i)?;
                        let mut value_span = i.trim_wsc_start()?;
                        let doc =
                            DocComment::try_parse_rev_combine_postfix(ctx, &mut value_span, doc)?;
                        let mut value = DefineValue::parse_all(ctx, value_span.trim_wsc_end()?)?;
                        if let Some(td) = &mut *ctx.active_typedef.borrow_mut() {
                            let mut associate = true;
                            if let Some(prefix) = td.use_for_defines {
                                if !ident.as_str().starts_with(prefix) {
                                    associate = false;
                                }
                            }
                            if associate {
                                value = value.cast_expr(Type::ident(td.ident.clone()));
                                td.associated_defines
                                    .borrow_mut()
                                    .push((ident.clone().try_into().unwrap(), doc.clone()));
                            }
                        }
                        let mut define = Define {
                            span: span.clone(),
                            doc,
                            ident,
                            args: None,
                            value,
                        };
                        patch_parsed_define(ctx, &mut define)?;
                        PreProcLineKind::Define(define)
                    }
                }

                "undef" => PreProcLineKind::Undef(Ident::parse_all(ctx, i)?),

                "include" => {
                    let i = i.trim_wsc_start()?;
                    let e = i.as_bytes()[i.len() - 1];
                    let kind = match i.as_bytes()[0] {
                        b'<' => {
                            if e != b'>' {
                                return Err(ParseErr::new(i.slice(i.len() - 1..), "expected `>`"));
                            }
                            IncludeKind::System
                        }
                        b'"' => {
                            if e != b'"' {
                                return Err(ParseErr::new(i.slice(i.len() - 1..), "expected `\"`"));
                            }
                            IncludeKind::Local
                        }
                        _ => return Err(ParseErr::new(i, "malformed include path")),
                    };
                    PreProcLineKind::Include(Include {
                        span: span.clone(),
                        kind,
                        path: i.slice(1..i.len() - 1),
                    })
                }

                "pragma" => PreProcLineKind::Pragma(i),
                "error" => PreProcLineKind::Error(i),
                "warning" => PreProcLineKind::Warning(i),

                _ => return Ok((input.clone(), None)),
            };
            Ok((rest, Some(Self { span, kind })))
        } else {
            Ok((input.clone(), None))
        }
    }
}
