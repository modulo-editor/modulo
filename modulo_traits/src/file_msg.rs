use std::sync::mpsc::Sender;
use text::Point;

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
    /// Clears all the text in the file
    ClearAllText,
    /// Saves the file to the OS file system. If the path is `None`, the file will not be saved.
    /// The front end is responsible for making sure the file thread has a proper path to a file.
    /// This may in the future accept a channel that
    Save(Sender<SaveResult>),
}

/// Used to output the result of the save message. The frontend can use this information to know if
/// a file saved successfully.
pub enum SaveResult {
    Ok,
    Err,
    PromptForPath,
}
