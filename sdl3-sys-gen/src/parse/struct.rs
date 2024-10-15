use super::{
    patch_parsed_struct, CanCopy, Delimited, DocComment, GetSpan, Ident, Kw_struct, Kw_union, Op,
    Parse, ParseContext, ParseErr, ParseRawRes, Span, Spanned, Type, TypeWithReqIdent,
    WsAndComments,
};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructKind {
    Struct,
    Union,
}

#[derive(Clone, Debug)]
pub struct StructOrUnion {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub kind: StructKind,
    pub ident: Option<Ident>,
    pub generated_ident: Ident,
    pub fields: Option<StructFields>,
    pub hidden: bool,
    pub can_copy: CanCopy,
    pub can_construct: bool,
}

impl StructOrUnion {
    pub const fn is_struct(&self) -> bool {
        matches!(self.kind, StructKind::Struct)
    }
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        let kw_struct = Kw_struct::try_parse(ctx, &mut rest)?;
        let kw_union = if kw_struct.is_none() {
            Kw_union::try_parse(ctx, &mut rest)?
        } else {
            None
        };
        if kw_struct.is_none() && kw_union.is_none() {
            return Ok((input.clone(), None));
        }
        WsAndComments::try_parse(ctx, &mut rest)?;
        let ident = Ident::try_parse(ctx, &mut rest)?;
        let (mut rest2, _) = WsAndComments::try_parse_raw(ctx, &rest)?;
        let generated_ident = if let Some(ident) = &ident {
            ident.clone()
        } else {
            let sibling = *ctx.sibling_struct_index.borrow();
            *ctx.sibling_struct_index.borrow_mut() += 1;
            Ident::new_inline(format!(
                "{}__Anon{}{}",
                ctx.parent_struct_ident
                    .borrow()
                    .as_ref()
                    .map(|p| p.as_str())
                    .unwrap_or(""),
                if kw_struct.is_some() {
                    "Struct"
                } else {
                    "Union"
                },
                sibling
            ))
        };
        let _guard = ctx.with_parent_struct_guard(Some(generated_ident.clone()));
        let fields = StructFields::try_parse(ctx, &mut rest2)?;
        let mut has_refcount = false;
        if let Some(fields) = &fields {
            rest = rest2;
            for field in fields.fields.iter() {
                if field.ident.as_str() == "refcount" {
                    has_refcount = true;
                    break;
                }
            }
        }
        let span = kw_struct
            .as_ref()
            .map(|s| s.span())
            .unwrap_or_else(|| kw_union.as_ref().unwrap().span())
            .start()
            .join(&rest.start());
        let kind = if kw_struct.is_some() {
            StructKind::Struct
        } else {
            StructKind::Union
        };
        let mut this = Self {
            span,
            doc,
            kind,
            ident,
            generated_ident,
            fields,
            hidden: false,
            can_copy: if has_refcount {
                CanCopy::Never
            } else {
                CanCopy::Default
            },
            can_construct: !has_refcount,
        };
        patch_parsed_struct(ctx, &mut this)?;
        Ok((rest, Some(this)))
    }
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub span: Span,
    pub doc: Option<DocComment>,
    pub ident: Ident,
    pub ty: Type,
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

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(ctx, &mut rest)?;
        if let Some(twi) = TypeWithReqIdent::try_parse(ctx, &mut rest)? {
            let mut idents = vec![twi.ident.unwrap()];
            WsAndComments::try_parse(ctx, &mut rest)?;
            while let Some(comma) = <Op![,]>::try_parse(ctx, &mut rest)? {
                if !twi.ty.strictly_left_aligned() {
                    return Err(ParseErr::new(
                        comma.span,
                        "multiple declaration for pointer and array types isn't supported",
                    ));
                }
                WsAndComments::try_parse(ctx, &mut rest)?;
                idents.push(Ident::parse(ctx, &mut rest)?);
                WsAndComments::try_parse(ctx, &mut rest)?;
            }
            let semi = <Op![;]>::parse(ctx, &mut rest)?;
            let span = input.start().join(&semi.span);
            let doc = DocComment::try_parse_combine_postfix(ctx, &mut rest, doc)?;
            WsAndComments::try_parse(ctx, &mut rest)?;
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
    pub span: Span,
    pub open_brace: Op<'{'>,
    pub fields: Vec<StructField>,
    pub close_brace: Op<'}'>,
}

impl Parse for StructFields {
    fn desc() -> Cow<'static, str> {
        "struct fields".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(f)) =
            Spanned::<Delimited<Op<'{'>, Vec<StructFieldGroup>, Op<'}'>>>::try_parse_raw(
                ctx, input,
            )?
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
