use span::Span;

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

    /// Boolean literals.
    Boolean,
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

    /// ~
    Tilde,

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

    /// <<
    Shl,

    /// <<=
    ShlEqual,

    /// >>
    Shr,

    /// >>=
    ShrEqual,

    /// Any identifiers, including keywords.
    Ident(IdentKind),

    /// Literals.
    Literal(LiteralKind),

    /// End of file.
    EoF,
}

impl TokenKind {
    /// Return if this token kind is a unary operator or not.
    pub fn is_unary_op(self) -> bool {
        use TokenKind::{Bang, Minus, Tilde};

        matches!(self, Bang | Minus | Tilde)
    }

    /// Return if this token kind is a binary operator or not.
    pub fn is_binary_op(self) -> bool {
        use TokenKind::{
            AmpAmp, Ampersand, AmpersandEqual, BangEqual, Bar, BarBar, BarEqual, Equal, EqualEqual,
            Gt, GtEqual, Lt, LtEqual, Minus, MinusEqual, Percent, PercentEqual, Plus, PlusEqual,
            Shl, ShlEqual, Shr, ShrEqual, Slash, SlashEqual, Star, StarEqual,
        };

        matches!(
            self,
            EqualEqual
                | Plus
                | PlusEqual
                | Minus
                | MinusEqual
                | Star
                | StarEqual
                | Slash
                | SlashEqual
                | Percent
                | PercentEqual
                | Ampersand
                | AmpersandEqual
                | AmpAmp
                | Bar
                | BarEqual
                | BarBar
                | BangEqual
                | Lt
                | LtEqual
                | Gt
                | GtEqual
                | Shl
                | ShlEqual
                | Shr
                | ShrEqual
        )
    }

    /// Returns if this token kind is a comparison operator or not.
    pub fn is_comparison_op(self) -> bool {
        use TokenKind::{Gt, GtEqual, Lt, LtEqual};

        matches!(self, Lt | LtEqual | Gt | GtEqual)
    }

    /// Returns if this token kind is an equality operator or not.
    pub fn is_equality_op(self) -> bool {
        use TokenKind::{BangEqual, EqualEqual};

        matches!(self, BangEqual | EqualEqual)
    }
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
