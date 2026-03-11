//! Source positions and spans used throughout lexing, parsing, and diagnostics.
//!
//! Spans are carried from tokens through compiled instructions so failures can
//! be traced back to the original source text.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn merge(self, other: Span) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Span};

    #[test]
    fn position_new_sets_all_fields() {
        let position = Position::new(7, 3, 9);
        assert_eq!(position.offset, 7);
        assert_eq!(position.line, 3);
        assert_eq!(position.column, 9);
    }

    #[test]
    fn span_merge_keeps_left_start_and_right_end() {
        let left = Span::new(Position::new(1, 1, 1), Position::new(4, 1, 4));
        let right = Span::new(Position::new(5, 2, 1), Position::new(9, 2, 5));
        let merged = left.merge(right);
        assert_eq!(merged.start, left.start);
        assert_eq!(merged.end, right.end);
    }
}
