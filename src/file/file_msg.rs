use ::file::text::Point;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct FileThreadId(pub usize);

/// Messages that can be sent to the file thread to manipulate text
#[derive(Debug)]
pub enum ToFileThreadMsg {
    /// Replace text between the start point and end point with the data from the string.
    /// If the end point is `None`, the text is just inserted at the start point.
    /// If the string is the empty string, the text between the start point and end point is
    /// deleted.
    ReplaceText(Point, Option<Point>, String),
}
