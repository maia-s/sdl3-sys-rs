use super::{
    Asm, Define, Delimited, DocCommentFile, Enum, EnumVariant, Expr, ExprNoComma, FnCall, Function,
    GetSpan, Ident, Include, Kw_break, Kw_continue, Kw_do, Kw_for, Kw_if, Kw_return, Kw_static, Op,
    Parse, ParseContext, ParseErr, ParseRawRes, PreProcBlock, PreProcLine, PreProcLineKind,
    Punctuated, Span, StructOrUnion, Terminated, Type, TypeDef, TypeWithReqIdent, WsAndComments,
};
use crate::{
    parse::{Kw_else, Kw_while},
    Defer,
};
use core::iter::FusedIterator;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct RustCode {
    pub value: String,
    pub ty: Type,
    pub is_const: bool,
    pub is_unsafe: bool,
}

impl RustCode {
    pub fn boxed(value: String, ty: Type, is_const: bool, is_unsafe: bool) -> Box<Self> {
        Box::new(Self {
            value,
            ty,
            is_const,
            is_unsafe,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Item {
    PreProcBlock(PreProcBlock),
    Block(Block),
    Skipped(Span),
    Define(Define),
    Undef(Ident),
    Include(Include),
    Pragma(Span),
    Error(Span),
    Warning(Span),
    FileDoc(DocCommentFile),
    StructOrUnion(StructOrUnion),
    Enum(Enum),
    Function(Function),
    Expr(Expr),
    FnCall(FnCall),
    TypeDef(TypeDef),
    VarDecl(VarDecl),
    DoWhile(DoWhile),
    While(While),
    For(For),
    IfElse(IfElse),
    Return(Return),
    EnumVariant(EnumVariant),
    Break(Span),
    Continue(Span),
}

impl Parse for Item {
    fn desc() -> Cow<'static, str> {
        "item".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut clear_td = Defer::new(|| {
            ctx.active_typedef.take();
        });
        let input = &input.trim_wsc_start()?;
        if let (rest, Some(block)) = Block::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Self::Block(block))))
        } else if let (rest, Some(pp)) = PreProcLine::try_parse_raw(ctx, input)? {
            clear_td.disable();
            Ok((
                rest,
                Some(match pp.kind {
                    PreProcLineKind::Define(d) => Item::Define(d),
                    PreProcLineKind::Undef(u) => Item::Undef(u),
                    PreProcLineKind::Include(i) => Item::Include(i),
                    PreProcLineKind::Pragma(p) => Item::Pragma(p),
                    PreProcLineKind::Error(e) => Item::Error(e),
                    PreProcLineKind::Warning(w) => Item::Warning(w),
                    _ => {
                        return if let (rest, Some(block)) = PreProcBlock::try_parse_raw(ctx, input)?
                        {
                            Ok((rest, Some(Item::PreProcBlock(block))))
                        } else {
                            Ok((input.clone(), None))
                        }
                    }
                }),
            ))
        } else if let (rest, Some(s)) = Terminated::<Kw_break, Op![;]>::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::Break(s.value.span))))
        } else if let (rest, Some(s)) =
            Terminated::<Kw_continue, Op![;]>::try_parse_raw(ctx, input)?
        {
            Ok((rest, Some(Item::Continue(s.value.span))))
        } else if let (rest, Some(s)) = DoWhile::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::DoWhile(s))))
        } else if let (rest, Some(s)) = While::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::While(s))))
        } else if let (rest, Some(s)) = For::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::For(s))))
        } else if let (rest, Some(s)) = IfElse::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::IfElse(s))))
        } else if let (rest, Some(s)) = Return::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::Return(s))))
        } else if let (rest, Some(f)) = Function::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::Function(f))))
        } else if let (rest, Some(t)) = TypeDef::try_parse_raw(ctx, input)? {
            if t.use_for_defines.is_some() {
                clear_td.disable();
                *ctx.active_typedef.borrow_mut() = Some(t.clone());
            }
            Ok((rest, Some(Item::TypeDef(t))))
        } else if let (mut rest, Some(e)) = Enum::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            <Op![;]>::parse(ctx, &mut rest)?;
            Ok((rest, Some(Item::Enum(e))))
        } else if let (rest, Some(s)) = StructOrUnionItem::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::StructOrUnion(s.0))))
        } else if let (rest, Some(decl)) = VarDecl::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::VarDecl(decl))))
        } else if let (rest, Some(asm)) =
            Asm::try_parse_raw_if(ctx, input, |a| a.kind.as_str() == "_asm")?
        {
            Ok((rest, Some(Item::Expr(Expr::Asm(asm)))))
        } else if let (rest, Some(expr)) = Terminated::<Expr, Op![;]>::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::Expr(expr.value))))
        } else if let (rest, Some(call)) = FnCall::try_parse_raw(ctx, input)? {
            // fn call with ident and no trailing `;` (macro call)
            Ok((rest, Some(Item::FnCall(call))))
        } else if let (rest, Some(dc)) = DocCommentFile::try_parse_raw(ctx, input)? {
            Ok((rest, Some(Item::FileDoc(dc))))
        } else if let (rest, Some(ev)) =
            EnumVariant::try_parse_raw_if(ctx, input, |ev| ev.expr.is_some())?
        {
            Ok((rest, Some(Item::EnumVariant(ev))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Items(pub Vec<Item>);

impl Items {
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = &Item> + ExactSizeIterator + FusedIterator {
        self.0.iter()
    }
}

impl Parse for Items {
    fn desc() -> Cow<'static, str> {
        "items".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let active_typedef = ctx.active_typedef.take();
        let _restore = Defer::new(|| *ctx.active_typedef.borrow_mut() = active_typedef);
        let (rest, parsed) = Vec::try_parse_raw(ctx, input)?;
        Ok((rest, parsed.map(Items)))
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub span: Span,
    pub items: Items,
}

impl GetSpan for Block {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Block {
    fn desc() -> Cow<'static, str> {
        "block".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(block)) =
            Delimited::<Op<'{'>, Option<Items>, Op<'}'>>::try_parse_raw(ctx, input)?
        {
            Ok((
                rest,
                Some(Self {
                    span: block.open.span.join(&block.close.span),
                    items: block.value.unwrap_or_default(),
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct DoWhile {
    pub span: Span,
    pub block: Block,
    pub cond: Expr,
}

impl GetSpan for DoWhile {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for DoWhile {
    fn desc() -> Cow<'static, str> {
        "do while".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(do_kw)) = Kw_do::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let block = Block::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            Kw_while::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let cond = Delimited::<Op<'('>, Expr, Op<')'>>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            // macros leave out the `;`
            let span = if let Some(semi) = <Op![;]>::try_parse(ctx, &mut rest)? {
                do_kw.span.join(&semi.span)
            } else {
                do_kw.span.join(&cond.close.span)
            };
            Ok((
                rest,
                Some(Self {
                    span,
                    block,
                    cond: cond.value,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct For {
    pub span: Span,
    pub init: Expr,
    pub cond: Expr,
    pub iter: Expr,
    pub block: Block,
}

impl Parse for For {
    fn desc() -> Cow<'static, str> {
        "for".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(for_kw)) = Kw_for::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            Op::<'('>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let init = Expr::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            <Op![;]>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let cond = Expr::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            <Op![;]>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let iter = Expr::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            Op::<')'>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let block = Block::parse(ctx, &mut rest)?;
            let span = for_kw.span.join(&block.span);
            Ok((
                rest,
                Some(Self {
                    span,
                    init,
                    cond,
                    iter,
                    block,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfElse {
    pub span: Span,
    pub cond: Expr,
    pub on_true: Block,
    pub on_false: Option<Block>,
}

impl GetSpan for IfElse {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for IfElse {
    fn desc() -> Cow<'static, str> {
        "if".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(if_kw)) = Kw_if::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let cond = Delimited::<Op<'('>, Expr, Op<')'>>::parse(ctx, &mut rest)?.value;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let on_true = Block::parse(ctx, &mut rest)?;
            let mut span = if_kw.span.join(&on_true.span);
            let (mut rest2, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
            let on_false = if Kw_else::try_parse(ctx, &mut rest2)?.is_some() {
                rest = rest2;
                WsAndComments::try_parse(ctx, &mut rest)?;
                let on_false = if let (_, Some(_)) = Kw_if::try_parse_raw(ctx, &rest)? {
                    let elseif = IfElse::parse(ctx, &mut rest)?;
                    Block {
                        span: elseif.span(),
                        items: Items(vec![Item::IfElse(elseif)]),
                    }
                } else {
                    Block::parse(ctx, &mut rest)?
                };
                span = span.join(&on_false.span);
                Some(on_false)
            } else {
                None
            };
            Ok((
                rest,
                Some(Self {
                    span,
                    cond,
                    on_true,
                    on_false,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Return {
    pub span: Span,
    pub expr: Expr,
}

impl GetSpan for Return {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for Return {
    fn desc() -> Cow<'static, str> {
        "return".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(return_kw)) = Kw_return::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let expr = Expr::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let semi = <Op![;]>::parse(ctx, &mut rest)?;
            Ok((
                rest,
                Some(Self {
                    span: return_kw.span.join(&semi.span),
                    expr,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub struct StructOrUnionItem(StructOrUnion);

impl Parse for StructOrUnionItem {
    fn desc() -> Cow<'static, str> {
        "struct or union item".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(Terminated { value: s, term: _ })) =
            Terminated::<StructOrUnion, Op![;]>::try_parse_raw(ctx, input)?
        {
            if s.ident.is_none() {
                return Err(ParseErr::new(
                    s.span(),
                    format!(
                        "top level anonymous {}",
                        if s.is_struct() { "struct" } else { "union" }
                    ),
                ));
            }
            Ok((rest, Some(Self(s))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub span: Span,
    pub kw_static: Option<Kw_static>,
    pub ident: Ident,
    pub ty: Type,
    pub init: Option<Expr>,
}

impl GetSpan for VarDecl {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for VarDecl {
    fn desc() -> Cow<'static, str> {
        "variable declaration".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let kw_static = Kw_static::try_parse(ctx, &mut rest)?;
        WsAndComments::try_parse(ctx, &mut rest)?;
        if let Some(ty) = TypeWithReqIdent::try_parse(ctx, &mut rest)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let init = if <Op![=]>::try_parse(ctx, &mut rest)?.is_some() {
                WsAndComments::try_parse(ctx, &mut rest)?;
                let init = if let Some(values) = Delimited::<
                    Op<'{'>,
                    Option<Punctuated<ExprNoComma, Op![,]>>,
                    Op<'}'>,
                >::try_parse(ctx, &mut rest)?
                {
                    let span = values.span();
                    let values: Vec<ExprNoComma> =
                        values.value.map(|v| v.into()).unwrap_or_default();
                    let values = values.into_iter().map(|i| i.0).collect();
                    Some(Expr::ArrayValues { span, values })
                } else {
                    Some(ExprNoComma::parse(ctx, &mut rest)?.0)
                };
                WsAndComments::try_parse(ctx, &mut rest)?;
                init
            } else {
                None
            };
            let semi = if init.is_some() {
                Some(<Op![;]>::parse(ctx, &mut rest)?)
            } else {
                <Op![;]>::try_parse(ctx, &mut rest)?
            };
            if let Some(semi) = semi {
                let span = ty.span().join(&semi.span);
                return Ok((
                    rest,
                    Some(Self {
                        span,
                        kw_static,
                        ident: ty.ident.unwrap(),
                        ty: ty.ty,
                        init,
                    }),
                ));
            }
        }
        Ok((input.clone(), None))
    }
}

#[derive(Clone, Debug)]
pub struct While {
    pub span: Span,
    pub block: Block,
    pub cond: Expr,
}

impl GetSpan for While {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for While {
    fn desc() -> Cow<'static, str> {
        "while".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(kw_while)) = Kw_while::try_parse_raw(ctx, input)? {
            WsAndComments::try_parse(ctx, &mut rest)?;
            let cond = Delimited::<Op<'('>, Expr, Op<')'>>::parse(ctx, &mut rest)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
            let block = Block::parse(ctx, &mut rest)?;
            let span = kw_while.span.join(&block.span);
            Ok((
                rest,
                Some(Self {
                    span,
                    block,
                    cond: cond.value,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
