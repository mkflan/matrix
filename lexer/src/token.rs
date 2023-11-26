use std::{fmt, ops::Range};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub(crate) fn of_token(token_len: usize, cursor_y: usize) -> Self {
        Span {
            start: cursor_y - token_len,
            end: cursor_y,
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl From<Range<usize>> for Span {
    fn from(Range { start, end }: Range<usize>) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// (
    OpenParen,

    /// )
    ClosingParen,

    /// {
    OpenCurly,

    /// }
    ClosingCurly,

    /// [
    OpenSquare,

    /// ]
    ClosingSquare,

    /// :
    Colon,

    /// ;
    Semicolon,

    /// .
    Period,

    /// '
    SingleQuote,

    /// "
    DoubleQuote,

    /// ,
    Comma,

    /// =
    Equals,

    /// +
    Plus,

    /// -
    Minus,

    /// *
    Star,

    /// /
    Slash,

    /// %
    Percent,

    /// &
    Ampersand,

    /// |
    Bar,

    /// <
    Lt,

    /// >
    Gt,

    /// Any identifiers, including keywords.
    Ident,

    /// End of file.
    EoF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
