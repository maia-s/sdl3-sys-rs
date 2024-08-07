use super::{
    Define, DocComment, DocCommentFile, Enum, FnCall, Function, GetSpan, Ident, Include, Parse,
    ParseErr, ParseRawRes, PreProcBlock, PreProcLine, PreProcLineKind, Span, StructOrUnion,
    Terminated, TypeDef, WsAndComments,
};
use std::borrow::Cow;

pub enum Item {
    PreProcBlock(PreProcBlock),
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
    FnCall(FnCall),
    TypeDef(TypeDef),
}

impl Parse for Item {
    fn desc() -> Cow<'static, str> {
        "item".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let input = &input.trim_wsc_start()?;
        if let (rest, Some(pp)) = PreProcLine::try_parse_raw(input)? {
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
        } else if let (
            rest,
            Some(Terminated {
                value: call,
                term: _,
            }), // hack: optional `;` bc global macro calls are inconsistent about that
        ) = Terminated::<FnCall, Option<Op![;]>>::try_parse_raw(input)?
        {
            Ok((rest, Some(Item::FnCall(call))))
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
        } else if let (rest, Some(dc)) = DocCommentFile::try_parse_raw(input)? {
            Ok((rest, Some(Item::FileDoc(dc.into()))))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub type Items = Vec<Item>;

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
