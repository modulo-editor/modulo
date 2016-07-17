use std::ops::{Index, Range, RangeTo, RangeFrom};
use std::cmp::{Ord, Ordering};

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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
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

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.line < other.line {
            Ordering::Less
        } else if self.line > other.line {
            Ordering::Greater
        } else {
            if self.column < other.column {
                Ordering::Less
            } else if self.column > other.column {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

/// Repesents a cursor location or selection. If `end_point` is `None`, this represents a cursor
/// location, otherwise this represents a selection.
#[derive(Debug, PartialOrd, PartialEq, Eq)]
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

impl Ord for SelectionRange {
    fn cmp(&self, other: &SelectionRange) -> Ordering {
        let smaller_a = match self.end_point {
            Some(end) => {
                if self.begin_point < end {
                    self.begin_point
                } else {
                    end
                }
            },
            None => self.begin_point
        };

        let smaller_b = match other.end_point {
            Some(end) => {
                if other.begin_point < end {
                    other.begin_point
                } else {
                    end
                }
            },
            None => other.begin_point
        };

        smaller_a.cmp(&smaller_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_sorting() {
        let point_a = Point::new(0, 0);
        let point_b = Point::new(1, 0);
        assert!(point_a < point_b);
        let point_a = Point::new(0, 0);
        let point_b = Point::new(0, 1);
        assert!(point_a < point_b);
        let point_a = Point::new(0, 1);
        let point_b = Point::new(0, 1);
        assert!(point_a == point_b);
        let point_a = Point::new(1, 0);
        let point_b = Point::new(1, 0);
        assert!(point_a == point_b);
        let point_a = Point::new(1, 0);
        let point_b = Point::new(0, 0);
        assert!(point_a > point_b);
        let point_a = Point::new(0, 1);
        let point_b = Point::new(0, 0);
        assert!(point_a > point_b);
    }

    #[test]
    fn test_range_sorting() {
        let range_a = SelectionRange::new(Point::new(0, 0), None);
        let range_b = SelectionRange::new(Point::new(1, 0), None);
        assert!(range_a < range_b);
        let range_a = SelectionRange::new(Point::new(1, 0), None);
        let range_b = SelectionRange::new(Point::new(1, 0), None);
        assert!(range_a == range_b);
        let range_a = SelectionRange::new(Point::new(1, 0), None);
        let range_b = SelectionRange::new(Point::new(0, 0), None);
        assert!(range_a > range_b);
        let range_a = SelectionRange::new(Point::new(0, 0), Some(Point::new(1, 0)));
        let range_b = SelectionRange::new(Point::new(0, 0), None);
        assert!(range_a > range_b);
        let range_a = SelectionRange::new(Point::new(0, 0), Some(Point::new(1, 0)));
        let range_b = SelectionRange::new(Point::new(0, 0), Some(Point::new(2, 0)));
        assert!(range_a < range_b);
        let range_a = SelectionRange::new(Point::new(0, 0), Some(Point::new(2, 0)));
        let range_b = SelectionRange::new(Point::new(0, 0), Some(Point::new(2, 0)));
        assert!(range_a == range_b);
    }
}
