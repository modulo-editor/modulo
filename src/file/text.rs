/// Holds data for a single line of text.
#[derive(Debug)]
pub struct Line {
    pub text: String,
}

/// Represents a cursor in text.
#[derive(Debug)]
pub struct Point {
    pub line: usize,
    pub index: usize,
}
