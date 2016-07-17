//! This module handles storing and manipulating file data.

use modulo_traits::text::{Line, Point, SelectionRange};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Stores data for a file.
pub struct FileData {
    /// Stores the selection ranges in the entire file.
    pub selections: Vec<SelectionRange>,
    /// Stores the text data in the entire file.
    pub lines: Vec<Line>,
}

impl FileData {
    /// Creates file data with a single new line.
    pub fn new() -> FileData {
        FileData {
            selections: Vec::new(),
            lines: vec!(Line::new(String::new())),
        }
    }

    /// Loads file data from a file on the file system.
    pub fn load_from_file(&mut self, path: &Path) {
        assert!(path.exists() && path.is_file());
        let mut file = File::open(path).expect("Failed to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file to string.");
        self.load_from_string(&data);
    }

    /// Loads file data from a string.
    pub fn load_from_string(&mut self, string: &str) {
        self.lines.clear();
        for line in string.lines() {
            self.lines.push(Line::new(line.into()));
        }
    }

    /// Replaces all the text in a selection range. Returns the new vec of selection ranges.
    pub fn replace_text(&mut self, text: &str) {
        // TODO(Connor): Check for overlapping ranges and merge them (maybe this should be done when ranges are edited)
        self.selections.sort();

        let mut buffer = Vec::new();
        let mut new_selections = Vec::new();
        let mut last_line = 0;

        for selection in &self.selections {
            // Make sure begin is always before end
            let (begin, end) = match selection.end_point {
                Some(end) => if selection.begin_point < end {
                    (selection.begin_point, end)
                } else {
                    (end, selection.begin_point)
                },
                None => (selection.begin_point, selection.begin_point),
            };

            let prefix = &self.lines[begin.line][..begin.column];
            let postfix = &self.lines[end.line][end.column..];

            buffer.extend_from_slice(&self.lines[last_line..begin.line]);
            // TODO(Connor): Avoid this Vec
            let lines = text.lines().collect::<Vec<_>>();
            let new_line = buffer.len() + lines.len() - 1;
            let new_column = prefix.len() + lines.last().map(|last| last.len()).unwrap_or(0);
            new_selections.push(SelectionRange::new(Point::new(new_line, new_column), None));
            let text = format!("{}{}{}", prefix, text, postfix);
            buffer.extend(text.lines().map(|line| Line::new(line.into())));
            last_line = end.line + 1;
        }
        buffer.extend_from_slice(&self.lines[last_line..]);
        self.lines = buffer;
        self.selections = new_selections;
    }

    /// Clears all the data in the file;
    pub fn clear_all_text(&mut self) {
        self.selections.clear();
        self.lines.clear();
        self.lines.push(Line::new(String::new()));
    }

    pub fn add_selection(&mut self, range: SelectionRange) {
        self.selections.push(range);
    }

    pub fn clear_all_selections(&mut self) {
        self.selections.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modulo_traits::text::{Point, SelectionRange};

    #[test]
    fn test_new_file_single_new_line_no_selections() {
        let file_data = FileData::new();
        assert!(file_data.selections.is_empty());
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::new());
    }

    #[test]
    fn test_load_file_from_string() {
        let mut file_data = FileData::new();
        file_data.load_from_string("test\ntest\ntest");
        assert_eq!(file_data.lines.len(), 3);
    }

    #[test]
    fn test_clear_file_single_new_line_no_selections() {
        let mut file_data = FileData::new();
        file_data.load_from_string("test\ntest\ntest");
        file_data.clear_all_text();
        assert!(file_data.selections.is_empty());
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::new());
    }

    #[test]
    fn test_insert_text() {
        let mut file_data = FileData::new();
        file_data.add_selection(SelectionRange::new(Point::new(0, 0), None));
        file_data.replace_text("hi");
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::from("hi"));
    }

    #[test]
    fn test_replace_text() {
        let mut file_data = FileData::new();
        file_data.add_selection(SelectionRange::new(Point::new(0, 0), None));
        file_data.replace_text("hi\nhi");
        assert_eq!(file_data.lines.len(), 2);
        assert_eq!(file_data.selections.len(), 1);
        assert_eq!(file_data.lines[0].text, String::from("hi"));
        assert_eq!(file_data.lines[1].text, String::from("hi"));
        assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(1, 2), None));
        file_data.clear_all_selections();

        file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(0, 2))));
        file_data.replace_text("no");
        assert_eq!(file_data.lines.len(), 2);
        assert_eq!(file_data.selections.len(), 1);
        assert_eq!(file_data.lines[0].text, String::from("no"));
        assert_eq!(file_data.lines[1].text, String::from("hi"));
        assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
        file_data.clear_all_selections();

        file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(0,2))));
        file_data.add_selection(SelectionRange::new(Point::new(1, 0), Some(Point::new(1,2))));
        file_data.replace_text("so");
        assert_eq!(file_data.lines.len(), 2);
        assert_eq!(file_data.selections.len(), 2);
        assert_eq!(file_data.lines[0].text, String::from("so"));
        assert_eq!(file_data.lines[1].text, String::from("so"));
        assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
        assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(1, 2), None));
        file_data.clear_all_selections();

        file_data.clear_all_text();
        assert!(file_data.selections.is_empty());
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::new());
        file_data.load_from_string("no\nyes\nnot");
        file_data.add_selection(SelectionRange::new(Point::new(0, 1), Some(Point::new(0,2))));
        file_data.add_selection(SelectionRange::new(Point::new(2, 1), Some(Point::new(2,2))));
        file_data.replace_text("j");
        assert_eq!(file_data.lines.len(), 3);
        assert_eq!(file_data.selections.len(), 2);
        assert_eq!(file_data.lines[0].text, String::from("nj"));
        assert_eq!(file_data.lines[1].text, String::from("yes"));
        assert_eq!(file_data.lines[2].text, String::from("njt"));
        assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
        assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(2, 2), None));
        file_data.clear_all_selections();

        file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(1,3))));
        file_data.add_selection(SelectionRange::new(Point::new(2, 0), Some(Point::new(2,3))));
        file_data.replace_text("hi");
        assert_eq!(file_data.lines.len(), 2);
        assert_eq!(file_data.selections.len(), 2);
        assert_eq!(file_data.lines[0].text, String::from("hi"));
        assert_eq!(file_data.lines[1].text, String::from("hi"));
        assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
        assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(1, 2), None));
        file_data.clear_all_selections();
    }
}
