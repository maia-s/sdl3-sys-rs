use crate::parse::{Kw_else, Kw_while};

use super::{
    Define, Delimited, DocComment, DocCommentFile, Enum, Expr, ExprNoComma, FnCall, Function,
    GetSpan, Ident, Include, Kw_do, Kw_if, Op, Parse, ParseErr, ParseRawRes, PreProcBlock,
    PreProcLine, PreProcLineKind, Span, StructOrUnion, Terminated, Type, TypeDef, TypeWithReqIdent,
    WsAndComments,
};
use std::borrow::Cow;

pub enum Item {
    PreProcBlock(PreProcBlock),
    Block(Block),
    Skipped(Span),
    Define(Define),
    Undef(Ident),
    Include(Include),
    Pragma(Span),
    Error(Span),
    FileDoc(DocComment),
    StructOrUnion(StructOrUnion),
    Enum(Enum),
    Function(Function),
    Expr(Expr),
    FnCall(FnCall),
    TypeDef(TypeDef),
    VarDecl(VarDecl),
    DoWhile(DoWhile),
    IfElse(IfElse),
}

impl Parse for Item {
    fn desc() -> Cow<'static, str> {
        "item".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let input = &input.trim_wsc_start()?;
        if let (rest, Some(block)) = Block::try_parse_raw(input)? {
            Ok((rest, Some(Self::Block(block))))
        } else if let (rest, Some(pp)) = PreProcLine::try_parse_raw(input)? {
            Ok((
                rest,
                Some(match pp.kind {
                    PreProcLineKind::Define(d) => Item::Define(d),
                    PreProcLineKind::Undef(u) => Item::Undef(u),
                    PreProcLineKind::Include(i) => Item::Include(i),
                    PreProcLineKind::Pragma(p) => Item::Pragma(p),
                    PreProcLineKind::Error(e) => Item::Error(e),
                    _ => {
                        return if let (rest, Some(block)) = PreProcBlock::try_parse_raw(input)? {
                            Ok((rest, Some(Item::PreProcBlock(block))))
                        } else {
                            Ok((input.clone(), None))
                        }
                    }
                }),
            ))
        } else if let (rest, Some(s)) = DoWhile::try_parse_raw(input)? {
            Ok((rest, Some(Item::DoWhile(s))))
        } else if let (rest, Some(s)) = IfElse::try_parse_raw(input)? {
            Ok((rest, Some(Item::IfElse(s))))
        } else if let (rest, Some(f)) = Function::try_parse_raw(input)? {
            Ok((rest, Some(Item::Function(f))))
        } else if let (rest, Some(t)) = TypeDef::try_parse_raw(input)? {
            Ok((rest, Some(Item::TypeDef(t))))
        } else if let (mut rest, Some(e)) = Enum::try_parse_raw(input)? {
            WsAndComments::try_parse(&mut rest)?;
            <Op![;]>::parse(&mut rest)?;
            Ok((rest, Some(Item::Enum(e))))
        } else if let (rest, Some(s)) = StructOrUnionItem::try_parse_raw(input)? {
            Ok((rest, Some(Item::StructOrUnion(s.0))))
        } else if let (rest, Some(decl)) = VarDecl::try_parse_raw(input)? {
            Ok((rest, Some(Item::VarDecl(decl))))
        } else if let (rest, Some(expr)) = Terminated::<Expr, Op![;]>::try_parse_raw(input)? {
            Ok((rest, Some(Item::Expr(expr.value))))
        } else if let (rest, Some(call)) = FnCall::try_parse_raw(input)? {
            // fn call with ident and no trailing `;` (macro call)
            Ok((rest, Some(Item::FnCall(call))))
        } else if let (rest, Some(dc)) = DocCommentFile::try_parse_raw(input)? {
            Ok((rest, Some(Item::FileDoc(dc.into()))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub type Items = Vec<Item>;

pub struct Block {
    span: Span,
    items: Items,
}

impl Parse for Block {
    fn desc() -> Cow<'static, str> {
        "block".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(block)) =
            Delimited::<Op<'{'>, Option<Items>, Op<'}'>>::try_parse_raw(input)?
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

pub struct DoWhile {
    span: Span,
    block: Block,
    cond: Expr,
}

impl Parse for DoWhile {
    fn desc() -> Cow<'static, str> {
        "do while".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(do_kw)) = Kw_do::try_parse_raw(input)? {
            WsAndComments::try_parse(&mut rest)?;
            let block = Block::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            Kw_while::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            let cond = Delimited::<Op<'('>, Expr, Op<')'>>::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            // macros leave out the `;`
            let span = if let Some(semi) = <Op![;]>::try_parse(&mut rest)? {
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

pub struct IfElse {
    span: Span,
    cond: Expr,
    on_true: Block,
    on_false: Option<Block>,
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

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(if_kw)) = Kw_if::try_parse_raw(input)? {
            WsAndComments::try_parse(&mut rest)?;
            let cond = Delimited::<Op<'('>, Expr, Op<')'>>::parse(&mut rest)?.value;
            WsAndComments::try_parse(&mut rest)?;
            let on_true = Block::parse(&mut rest)?;
            let mut span = if_kw.span.join(&on_true.span);
            let (mut rest2, _) = WsAndComments::try_parse_raw(&rest)?;
            let on_false = if Kw_else::try_parse(&mut rest2)?.is_some() {
                rest = rest2;
                WsAndComments::try_parse(&mut rest)?;
                let on_false = if let (_, Some(_)) = Kw_if::try_parse_raw(&rest)? {
                    let elseif = IfElse::parse(&mut rest)?;
                    Block {
                        span: elseif.span(),
                        items: vec![Item::IfElse(elseif)],
                    }
                } else {
                    Block::parse(&mut rest)?
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

pub struct StructOrUnionItem(StructOrUnion);

impl Parse for StructOrUnionItem {
    fn desc() -> Cow<'static, str> {
        "struct or union item".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(Terminated { value: s, term: _ })) =
            Terminated::<StructOrUnion, Op![;]>::try_parse_raw(input)?
        {
            if s.ident.is_none() {
                return Err(ParseErr::new(
                    s.span(),
                    format!(
                        "top level anonymous {}",
                        if s.kw_struct.is_some() {
                            "struct"
                        } else {
                            "union"
                        }
                    ),
                ));
            }
            Ok((rest, Some(Self(s))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub struct VarDecl {
    span: Span,
    ident: Ident,
    ty: Type,
    init: Option<Expr>,
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

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(ty)) = TypeWithReqIdent::try_parse_raw(input)? {
            WsAndComments::try_parse(&mut rest)?;
            let init = if <Op![=]>::try_parse(&mut rest)?.is_some() {
                WsAndComments::try_parse(&mut rest)?;
                let init = Some(ExprNoComma::parse(&mut rest)?.0);
                WsAndComments::try_parse(&mut rest)?;
                init
            } else {
                None
            };
            let semi = if init.is_some() {
                Some(<Op![;]>::parse(&mut rest)?)
            } else {
                <Op![;]>::try_parse(&mut rest)?
            };
            if let Some(semi) = semi {
                let span = ty.span().join(&semi.span);
                return Ok((
                    rest,
                    Some(Self {
                        span,
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
