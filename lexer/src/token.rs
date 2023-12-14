use std::{fmt, ops::Range};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub(crate) fn of_token(token_len: usize, pos: usize) -> Self {
        Self {
            start: pos - token_len,
            end: pos,
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
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Proc,
    Let,
    Void,
    Int,
    Ret,
    Float,
    If,
    Elif,
    Else,
    For,
    While,
    Do,
    Bool,
    Str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentKind {
    /// A non-reserved identifier.
    NonReserved,

    /// A keyword or reserved identifier.
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerBase {
    /// Integer literals starting with "0b".
    Binary = 2,

    /// Integer literals starting with "0o".
    Octal = 8,

    /// Integer literals with no prefix.
    Decimal = 10,

    /// Integer literals starting with "0x".
    Hexadecimal = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    /// Character literals.
    Character,

    /// String literals.
    String,

    /// Integer literals.
    Integer { base: IntegerBase },

    /// Float literals.
    Float,
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

    /// ,
    Comma,

    /// =
    Equal,

    /// ==
    EqualEqual,

    /// +
    Plus,

    /// +=
    PlusEqual,

    /// -
    Minus,

    /// -=
    MinusEqual,

    /// *
    Star,

    /// *=
    StarEqual,

    /// /
    Slash,

    /// /=
    SlashEqual,

    /// %
    Percent,

    /// %=
    PercentEqual,

    /// &
    Ampersand,

    /// &=
    AmpersandEqual,

    /// &&,
    AmpAmp,

    /// |
    Bar,

    /// |=
    BarEqual,

    /// ||
    BarBar,

    /// !
    Bang,

    /// !=
    BangEqual,

    /// <
    Lt,

    /// <=
    LtEqual,

    /// >
    Gt,

    /// >=
    GtEqual,

    /// Any identifiers, including keywords.
    Ident(IdentKind),

    /// Literals.
    Literal(LiteralKind),

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
