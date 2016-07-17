use std::ops::{Index, Range, RangeTo, RangeFrom};

/// Holds data for a single line of text.
#[derive(Clone, Debug)]
pub struct Line {
    pub text: String,
}

impl Line {
    pub fn new(text: String) -> Line {
        Line {
            text: text,
        }
    }
}

impl Index<RangeTo<usize>> for Line {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &str {
        self.text.index(index)
    }
}

impl Index<RangeFrom<usize>> for Line {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &str {
        self.text.index(index)
    }
}

impl Index<Range<usize>> for Line {
    type Output = str;

    #[inline]
    fn index(&self, index: Range<usize>) -> &str {
        self.text.index(index)
    }
}

/// Represents a cursor in text. Stores the line index and the column index.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

impl Point {
    pub fn new(line: usize, column: usize) -> Point {
        Point {
            line: line,
            column: column,
        }
    }
}

/// Repesents a cursor location or selection. If `end_point` is `None`, this represents a cursor
/// location, otherwise this represents a selection.
#[derive(Debug)]
pub struct SelectionRange {
    pub begin_point: Point,
    pub end_point: Option<Point>,
}

impl SelectionRange {
    pub fn new(begin: Point, end: Option<Point>) -> SelectionRange {
        SelectionRange {
            begin_point: begin,
            end_point: end,
        }
    }
}
