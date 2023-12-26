use lexer::token::{self, Token};
use span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    /// Character literals.
    Character,

    /// String literals.
    String,

    /// Integer literals.
    Integer,

    /// Float literals.
    Float,

    /// Boolean literals.
    Boolean,
}

impl Into<LiteralKind> for token::LiteralKind {
    fn into(self) -> LiteralKind {
        match self {
            Self::Character => LiteralKind::Character,
            Self::String => LiteralKind::String,
            Self::Integer { base: _ } => LiteralKind::Integer,
            Self::Float => LiteralKind::Float,
            Self::Boolean => LiteralKind::Boolean,
        }
    }
}

/// Unary (prefix) operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpKind {
    /// -
    Neg,

    /// !
    LogNot,

    /// ~
    BwNot,
}

impl Into<UnaryOpKind> for token::TokenKind {
    fn into(self) -> UnaryOpKind {
        match self {
            Self::Minus => UnaryOpKind::Neg,
            Self::Bang => UnaryOpKind::LogNot,
            Self::Tilde => UnaryOpKind::BwNot,
            _ => unreachable!(
                "this implementation is only called when a unary operator has been reached"
            ),
        }
    }
}

/// Binary (infix) operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
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
    Mul,

    /// *=
    MulEqual,

    /// /
    Div,

    /// /=
    DivEqual,

    /// %
    Mod,

    /// %=
    ModEqual,

    /// &
    BwAnd,

    /// &=
    BwAndEqual,

    /// &&,
    LogAnd,

    /// |
    BwOr,

    /// |=
    BwOrEqual,

    /// ||
    LogOr,

    /// !=
    NotEqual,

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
}

impl Into<BinaryOpKind> for token::TokenKind {
    fn into(self) -> BinaryOpKind {
        match self {
            Self::Equal => BinaryOpKind::Equal,
            Self::EqualEqual => BinaryOpKind::EqualEqual,
            Self::Plus => BinaryOpKind::Plus,
            Self::PlusEqual => BinaryOpKind::PlusEqual,
            Self::Minus => BinaryOpKind::Minus,
            Self::MinusEqual => BinaryOpKind::MinusEqual,
            Self::Star => BinaryOpKind::Mul,
            Self::StarEqual => BinaryOpKind::MulEqual,
            Self::Slash => BinaryOpKind::Div,
            Self::SlashEqual => BinaryOpKind::DivEqual,
            Self::Percent => BinaryOpKind::Mod,
            Self::PercentEqual => BinaryOpKind::ModEqual,
            Self::Ampersand => BinaryOpKind::BwAnd,
            Self::AmpersandEqual => BinaryOpKind::BwAndEqual,
            Self::AmpAmp => BinaryOpKind::LogAnd,
            Self::Bar => BinaryOpKind::BwOr,
            Self::BarEqual => BinaryOpKind::BwOrEqual,
            Self::BarBar => BinaryOpKind::LogOr,
            Self::BangEqual => BinaryOpKind::NotEqual,
            Self::Lt => BinaryOpKind::Lt,
            Self::LtEqual => BinaryOpKind::LtEqual,
            Self::Gt => BinaryOpKind::Gt,
            Self::GtEqual => BinaryOpKind::GtEqual,
            Self::Shl => BinaryOpKind::Shl,
            Self::ShlEqual => BinaryOpKind::ShlEqual,
            Self::Shr => BinaryOpKind::Shr,
            Self::ShrEqual => BinaryOpKind::ShrEqual,
            _ => unreachable!("this implementation is only called when it is determined a binary operator has been reached")
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    /// A literal ("hello", 123, 20.4).
    Literal(LiteralKind),

    /// A unary expression (!false, -10).
    Unary {
        operator: UnaryOpKind,
        operand: Box<ExpressionKind>,
    },

    /// A binary expression (1 + 2, 5 > 3, 2 / 3).
    Binary {
        lhs: Box<ExpressionKind>,
        operator: BinaryOpKind,
        rhs: Box<ExpressionKind>,
    },

    /// A grouping ( (1 + 2), ((1 + 2) + (3 + 4)) ).
    Grouping(Box<ExpressionKind>),
}

// #[derive(Debug, Clone)]
// pub struct Expression {
//     pub kind: ExpressionKind,
//     pub span: Span,
// }
