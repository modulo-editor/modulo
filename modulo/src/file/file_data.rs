//! This module handles storing and manipulating file data.

use modulo_traits::text::{Line, SelectionRange};
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

    /// Replaces all the text in a selection range.
    pub fn replace_text_in_range(&mut self, range: SelectionRange, text: &str) {
        let begin = range.begin_point;
        let end = match range.end_point {
            Some(end) => end,
            None => begin,
        };

        let lines = {
            let before_text = &self.lines[begin.line][..begin.column];
            let after_text = &self.lines[end.line][end.column..];

            let text = format!("{}{}{}", before_text, text, after_text);
            let lines: Vec<Line> = text.lines().map(|line| Line::new(line.into())).collect();
            lines
        };

        self.lines.drain(begin.line..end.line);
        let mut line_index = begin.line;
        for line in &lines {
            // TODO(Connor): Remove this clone somehow.
            self.lines.insert(line_index, line.clone());
            line_index += 1;
        }

        self.lines = lines;
    }

    /// Clears all the data in the file;
    pub fn clear_all_text(&mut self) {
        self.selections.clear();
        self.lines.clear();
        self.lines.push(Line::new(String::new()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_single_new_line_no_selections() {
        let file_data = FileData::new();
        assert!(file_data.selections.is_empty());
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::new());
    }

    #[test]
    fn load_file_from_string() {
        let mut file_data = FileData::new();
        file_data.load_from_string("test\ntest\ntest");
        assert_eq!(file_data.lines.len(), 3);
    }

    #[test]
    fn clear_file_single_new_line_no_selections() {
        let mut file_data = FileData::new();
        file_data.load_from_string("test\ntest\ntest");
        file_data.clear_all_text();
        assert!(file_data.selections.is_empty());
        assert_eq!(file_data.lines.len(), 1);
        assert_eq!(file_data.lines[0].text, String::new());
    }
}
