use super::Span;
use std::{
    backtrace::Backtrace,
    borrow::Cow,
    error,
    fmt::{self, Debug, Display},
    panic::Location,
};

pub type ParseRawRes<T> = Result<ParseRawOk<T>, ParseErr>;
pub type ParseRes<T> = Result<T, ParseErr>;

pub type ParseRawOk<T> = (Span, T);

pub struct ParseErr {
    span: Span,
    pub message: Cow<'static, str>,
    location: &'static Location<'static>,
    backtrace: Box<Backtrace>,
}

impl ParseErr {
    #[track_caller]
    pub fn new(span: Span, message: impl Into<Cow<'static, str>>) -> Self {
        let backtrace = Box::new(Backtrace::capture());
        Self {
            span,
            message: message.into(),
            location: Location::caller(),
            backtrace,
        }
    }

    pub fn map_msg(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = message.into();
        self
    }
}

impl error::Error for ParseErr {}

impl Debug for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)?;
        writeln!(f)?;
        writeln!(f)?;
        writeln!(
            f,
            "error at {}:{}:{}",
            self.location.file(),
            self.location.line(),
            self.location.column()
        )?;
        writeln!(f)?;
        write!(f, "{}", self.backtrace)
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let src = self.span.source();
        let str = src.text.as_str();
        let pos = self.span.start_pos();
        let mut line = 1;
        let mut col = 1;
        let mut col_1_idx = 0;
        for (i, ch) in str.char_indices() {
            if i >= pos {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
                col_1_idx = i + 1;
            } else {
                col += 1;
            }
        }
        let (col_normal, col_error, col_msg, col_sidebar) = if f.alternate() {
            ("\x1b[0m", "\x1b[1;31m", "\x1b[1;37m", "\x1b[0;34m")
        } else {
            ("", "", "", "")
        };
        let message = &self.message;
        let sidebar = format!("{line}");
        let sidebar_empty = sidebar.replace(|_| true, " ");
        let line_str = &str[col_1_idx..];
        let (line_str, _) = line_str.split_once('\n').unwrap_or((line_str, ""));
        let indent = line_str[..(col - 1).min(line_str.len())].replace(|c| c != '\t', " ");
        let arrows = "^".repeat(self.span.len().min(line_str.len() - indent.len()).max(1));
        writeln!(f, "{col_error}error{col_msg}: {message}{col_normal}",)?;
        writeln!(
            f,
            "{col_sidebar}{sidebar_empty}--> {col_normal}{}:{}:{}",
            src.name, line, col
        )?;
        writeln!(f, "{col_sidebar}{sidebar_empty} |{col_normal}")?;
        writeln!(f, "{col_sidebar}{sidebar} | {col_normal}{line_str}")?;
        if indent.len() > message.len() {
            let indent = &indent[..indent.len() - message.len() - 1];
            write!(
                f,
                "{col_sidebar}{sidebar_empty} | {col_error}{indent}{message} {arrows}{col_normal}",
            )
        } else if sidebar_empty.len() + 2 + 1 + indent.len() + arrows.len() + 1 + message.len()
            > 100
        {
            writeln!(
                f,
                "{col_sidebar}{sidebar_empty} | {col_error}{indent}{arrows}{col_normal}",
            )?;
            write!(
                f,
                "{col_sidebar}{sidebar} | {col_error}{message}{col_normal}"
            )
        } else {
            write!(
                f,
                "{col_sidebar}{sidebar_empty} | {col_error}{indent}{arrows} {message}{col_normal}",
            )
        }
    }
}
