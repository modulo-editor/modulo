/// Holds data for a single line of text.
#[derive(Debug)]
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

/// Represents a cursor in text.
#[derive(Debug)]
pub struct Point {
    pub line: usize,
    pub index: usize,
}

impl Point {
    pub fn new(line: usize, index: usize) -> Point {
        Point {
            line: line,
            index: index,
        }
    }
}
