use super::{GetSpan, Parse, ParseContext, ParseErr, ParseRawRes, ParseRev, Span, WsAndComments};
use crate::common_doc_prefix;
use std::{
    borrow::Cow,
    fmt::{self, Display},
};

#[derive(Clone, Debug)]
pub struct DocComment {
    pub span: Span,
    pub doc: Span,
    pub trailing: bool,
}

impl Display for DocComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! pass {
            ($lines:expr) => {{
                let mut lines = $lines;
                let Some(first) = lines.next() else {
                    // no non-empty lines
                    return Ok(());
                };
                let Some(second) = lines.next() else {
                    // one non-empty line
                    return f.write_str(first.trim());
                };
                let i = second
                    .find(|c: char| !c.is_whitespace() && c != '*')
                    .unwrap_or(second.len());
                let mut prefix = &second[..i];
                for line in lines {
                    prefix = common_doc_prefix(prefix, line);
                }
                prefix
            }};
        }

        let prefix = pass!(self
            .doc
            .as_str()
            .lines()
            .filter(|line| !line.trim().is_empty()));

        let prefix2 = pass!(self.doc.as_str().lines().filter_map(|line| {
            let line = line.strip_prefix(prefix).unwrap_or(line);
            (!line.trim().is_empty()).then_some(line)
        }));

        let mut lines = self
            .doc
            .as_str()
            .lines()
            .map(|line| {
                if let Some(line) = line.strip_prefix(prefix) {
                    line.strip_prefix(prefix2).unwrap_or(line)
                } else {
                    line
                }
                .trim_end()
            })
            .skip_while(|line| line.trim().is_empty())
            .peekable();

        if let Some(line) = lines.peek() {
            if line.trim().starts_with("# Category") {
                lines.next();
                if let Some(line) = lines.peek() {
                    if line.trim().is_empty() {
                        lines.next();
                    }
                }
            }
        }

        let mut empties = 0;
        for line in lines {
            if line.trim().is_empty() {
                empties += 1;
            } else {
                for _ in 0..empties {
                    writeln!(f)?;
                }
                empties = 0;
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

impl GetSpan for DocComment {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl DocComment {
    pub fn try_parse_combine_postfix(
        ctx: &ParseContext,
        input: &mut Span,
        pre: Option<DocComment>,
    ) -> Result<Option<DocComment>, ParseErr> {
        if let Some(post) = DocCommentPost::try_parse(ctx, input)? {
            if pre.is_some() {
                // item has both prefix and postfix documentation comment; ignore postfix
                Ok(pre)
            } else {
                Ok(Some(post.into()))
            }
        } else {
            Ok(pre)
        }
    }

    pub fn try_parse_rev_combine_postfix(
        ctx: &ParseContext,
        input: &mut Span,
        pre: Option<DocComment>,
    ) -> Result<Option<DocComment>, ParseErr> {
        if let Some(post) = DocCommentPost::try_parse_rev(ctx, input)? {
            if pre.is_some() {
                // item has both prefix and postfix documentation comment; ignore postfix
                Ok(pre)
            } else {
                Ok(Some(post.into()))
            }
        } else {
            Ok(pre)
        }
    }
}

impl Parse for DocComment {
    fn desc() -> Cow<'static, str> {
        "documentation comment".into()
    }

    fn try_parse_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_start()?;
        if rest.starts_with("/**") && !rest.starts_with("/**<") {
            let start = rest.start();
            if let Some(end) = rest.as_bytes().windows(2).position(|b| b == b"*/") {
                let doc = rest.slice(3..end);
                // strip whitespace after the doc comment, up to max one newline
                let mut rest = rest.slice(end + 2..);
                let mut got_cr = false;
                for (i, ch) in rest.char_indices() {
                    if ch == '\n' {
                        if got_cr {
                            rest = rest.slice(i..);
                            let span = start.join(&rest.start());
                            return Ok((
                                rest,
                                Some(Self {
                                    span,
                                    doc,
                                    trailing: false,
                                }),
                            ));
                        }
                        got_cr = true;
                    } else if !ch.is_ascii_whitespace() {
                        rest = rest.slice(i..);
                        let span = start.join(&rest.start());
                        return Ok((
                            rest,
                            Some(Self {
                                span,
                                doc,
                                trailing: false,
                            }),
                        ));
                    }
                }
                let span = start.join(&rest.start());
                Ok((
                    rest.end(),
                    Some(Self {
                        span,
                        doc,
                        trailing: false,
                    }),
                ))
            } else {
                Err(ParseErr::new(rest.slice(..4), "doc comment with no end"))
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocCommentFile(pub DocComment);

impl Parse for DocCommentFile {
    fn desc() -> Cow<'static, str> {
        "doc comment for file".into()
    }

    fn try_parse_raw(ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(dc)) = DocComment::try_parse_raw(ctx, input)? {
            if WsAndComments::try_parse(ctx, &mut rest)?.is_some() {
                // empty line after doc comment, so it's not attached to anything
                return Ok((rest, Some(Self(dc))));
            }
        }
        Ok((input.clone(), None))
    }
}

pub struct DocCommentPost {
    span: Span,
    doc: Span,
}

impl Parse for DocCommentPost {
    fn desc() -> Cow<'static, str> {
        "documentation comment (postfix)".into()
    }

    fn try_parse_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_start()?;
        if rest.starts_with("/**<") {
            if let Some(end) = rest.as_bytes().windows(2).position(|b| b == b"*/") {
                let (span, rest) = rest.split_at(end + 2);
                let doc = span.slice(4..span.len() - 2).trim();
                Ok((rest, Some(Self { span, doc })))
            } else {
                todo!()
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl ParseRev for DocCommentPost {
    fn try_parse_rev_raw(_ctx: &ParseContext, input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_end()?;
        if rest.ends_with("*/") {
            if let Some(start) = rest.as_bytes().windows(4).rev().position(|b| b == b"/**<") {
                let (rest, span) = rest.split_at(rest.len() - start - 4);
                let doc = span.slice(4..span.len() - 2).trim();
                Ok((rest, Some(Self { span, doc })))
            } else {
                todo!()
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl From<DocCommentPost> for DocComment {
    fn from(value: DocCommentPost) -> Self {
        Self {
            span: value.span,
            doc: value.doc,
            trailing: true,
        }
    }
}
