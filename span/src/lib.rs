use miette::SourceSpan;
use std::{fmt, ops::Range};

/// An exclusive range representing a part of source code.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new span given a length and a position at the end of the span.     
    pub fn new(len: usize, pos: usize) -> Self {
        Self {
            start: pos - len,
            end: pos,
        }
    }

    /// Coalesce adjacent spans.
    pub fn coalesce_adjacent(self, other: Self) -> Self {
        let start = std::cmp::min(self.start, other.start);
        let end = std::cmp::max(self.end, other.end);

        Self { start, end }
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

#[allow(clippy::from_over_into)]
impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.start.into(), (self.end - self.start).into())
    }
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn test_coalesce_adjacent_spans() {
        let first = Span::from(1..2);
        let second = Span::from(2..5);
        assert_eq!(first.coalesce_adjacent(second), Span::from(1..5));

        let third = Span::from(2..5);
        let fourth = Span::from(1..5);
        assert_eq!(third.coalesce_adjacent(fourth), Span::from(1..5));

        let fifth = Span::from(1..2);
        let sixth = Span::from(3..5);
        assert_eq!(fifth.coalesce_adjacent(sixth), Span::from(1..5));

        let seventh = Span::from(1..5);
        let eigth = Span::from(2..4);
        assert_eq!(seventh.coalesce_adjacent(eigth), Span::from(1..5));
    }
}
