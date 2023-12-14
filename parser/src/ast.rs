#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BinOpKind {
    /// ==
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
}
