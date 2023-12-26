use crate::ast::*;
use std::fmt;

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LiteralKind::*;

        write!(
            f,
            "{}",
            match self {
                Character => "[char]",
                String => "[str]",
                Integer => "[int]",
                Float => "[float]",
                Boolean => "[bool]",
            }
        )
    }
}

impl fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UnaryOpKind::*;

        write!(
            f,
            "{}",
            match self {
                Neg => '-',
                LogNot => '!',
                BwNot => '~',
            }
        )
    }
}

impl fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOpKind::*;

        write!(
            f,
            "{}",
            match self {
                Equal => "=",
                EqualEqual => "==",
                Plus => "+",
                PlusEqual => "+=",
                Minus => "-",
                MinusEqual => "-=",
                Mul => "*",
                MulEqual => "*=",
                Div => "/",
                DivEqual => "/=",
                Mod => "%",
                ModEqual => "%=",
                BwAnd => "&",
                BwAndEqual => "&=",
                LogAnd => "&&",
                BwOr => "|",
                BwOrEqual => "|=",
                LogOr => "||",
                NotEqual => "!=",
                Lt => "<",
                LtEqual => "<=",
                Gt => ">",
                GtEqual => ">=",
                Shl => "<<",
                ShlEqual => "<<=",
                Shr => ">>",
                ShrEqual => ">>=",
            }
        )
    }
}
