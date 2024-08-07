use super::{
    Delimited, DocComment, GetSpan, Ident, Kw_struct, Kw_union, Op, Parse, ParseErr, ParseRawRes,
    Span, Spanned, Type, TypeWithReqIdent, WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct StructOrUnion {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub kw_struct: Option<Kw_struct>,
    pub kw_union: Option<Kw_union>,
    pub ident: Option<Ident>,
    pub fields: Option<StructFields>,
}

impl GetSpan for StructOrUnion {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for StructOrUnion {
    fn desc() -> Cow<'static, str> {
        "struct or union".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        let kw_struct = Kw_struct::try_parse(&mut rest)?;
        let kw_union = if kw_struct.is_none() {
            Kw_union::try_parse(&mut rest)?
        } else {
            None
        };
        if kw_struct.is_none() && kw_union.is_none() {
            return Ok((input.clone(), None));
        }
        WsAndComments::try_parse(&mut rest)?;
        let ident = Ident::try_parse(&mut rest)?;
        WsAndComments::try_parse(&mut rest)?;
        let fields = StructFields::try_parse(&mut rest)?;
        let span = input.start().join(&rest.start());
        Ok((
            rest,
            Some(Self {
                span,
                doc,
                kw_struct,
                kw_union,
                ident,
                fields,
            }),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct StructField {
    span: Span,
    doc: Option<DocComment>,
    ident: Ident,
    ty: Type,
}

#[derive(Debug)]
pub struct StructFieldGroup {
    span: Span,
    doc: Option<DocComment>,
    idents: Vec<Ident>,
    ty: Type,
}

impl Parse for StructFieldGroup {
    fn desc() -> Cow<'static, str> {
        "struct field".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        if let Some(twi) = TypeWithReqIdent::try_parse(&mut rest)? {
            let mut idents = vec![twi.ident.unwrap()];
            WsAndComments::try_parse(&mut rest)?;
            while let Some(comma) = <Op![,]>::try_parse(&mut rest)? {
                if !twi.ty.strictly_left_aligned() {
                    return Err(ParseErr::new(
                        comma.span,
                        "multiple declaration for pointer and array types isn't supported",
                    ));
                }
                WsAndComments::try_parse(&mut rest)?;
                idents.push(Ident::parse(&mut rest)?);
                WsAndComments::try_parse(&mut rest)?;
            }
            let semi = <Op![;]>::parse(&mut rest)?;
            let span = input.start().join(&semi.span);
            let doc = DocComment::try_parse_combine_postfix(doc, &mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            Ok((
                rest,
                Some(Self {
                    span,
                    doc,
                    idents,
                    ty: twi.ty,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct StructFields {
    span: Span,
    open_brace: Op<'{'>,
    fields: Vec<StructField>,
    close_brace: Op<'}'>,
}

impl Parse for StructFields {
    fn desc() -> Cow<'static, str> {
        "struct fields".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(f)) =
            Spanned::<Delimited<Op<'{'>, Vec<StructFieldGroup>, Op<'}'>>>::try_parse_raw(input)?
        {
            let fields = f
                .value
                .value
                .into_iter()
                .flat_map(|f| {
                    f.idents.into_iter().map(move |ident| StructField {
                        span: f.span.clone(),
                        doc: f.doc.clone(),
                        ident,
                        ty: f.ty.clone(),
                    })
                })
                .collect();
            Ok((
                rest,
                Some(Self {
                    span: f.span,
                    open_brace: f.value.open,
                    fields,
                    close_brace: f.value.close,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
