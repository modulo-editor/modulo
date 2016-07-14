use ::core::core_msg::ToCoreThreadMsg;
use ::file::file_msg::{FileThreadId, ToFileThreadMsg, SaveResult};
use ::file::text::{Line, Point};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

/// A file thread represents one open file. It contains all the information about the data within
/// that file and listens for messages to manipulate the data within the file.
pub struct FileThread {
    id: FileThreadId,
    core_sender: Sender<ToCoreThreadMsg>,
    core_receiver: Receiver<ToFileThreadMsg>,
    data: Vec<Line>,
    path: Option<PathBuf>,
}

impl FileThread {
    /// Call to open a new file thread. The `path` parameter is the `Path` to the file to edit.
    /// If `path` is `None`, an empty, untitled file is opened.
    /// If the file at the path does not exist, the file is created when the file is saved.
    pub fn start(id: FileThreadId,
                 path: Option<PathBuf>,
                 sender: Sender<ToCoreThreadMsg>,
                 receiver: Receiver<ToFileThreadMsg>) {
        thread::spawn(move || {
            println!("Spawning file thread.");
            let mut file_thread = FileThread {
                id: id,
                core_sender: sender,
                core_receiver: receiver,
                data: Vec::new(),
                path: path,
            };
            file_thread.load_file();
            file_thread.run();
        });
    }

    /// Runs the event loop for the `FileThread`
    pub fn run(&mut self) {
        while let Ok(msg) = self.core_receiver.recv() {
            match msg {
                ToFileThreadMsg::ReplaceText(begin, end, text) =>
                    self.handle_replace_text(begin, end, text),
                ToFileThreadMsg::Save(sender) =>
                    self.handle_save(sender),
            }
        }
    }

    fn load_file(&mut self) {
        if let Some(ref path) = self.path {
            let path = path.as_path();

            if !path.exists() || !path.is_file() {
                self.data.push(Line::new("".into()));
                return warn!("Illegal path, cannot load file.");
            }

            // TODO(Connor): Handle file opening failure.
            let mut file = File::open(path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data);

            for line in data.lines() {
                self.data.push(Line::new(line.into()));
            }
            info!("Loaded file from path: {:?}", path);
        } else {
            self.data.push(Line::new("".into()));
        }
    }

    fn handle_save(&self, sender: Sender<SaveResult>) {
        match self.path {
            Some(ref path) => {
                info!("Saving file at path: {:?}", path);
                let path = path.as_path();
                let mut file = if path.exists() && path.is_file() {
                    // TODO(Connor): Handle file opening failure.
                    File::create(path).unwrap()
                } else {
                    // TODO(Connor): Handle file opening failure.
                    File::create(path).unwrap()
                };
                let mut data = String::new();
                for line in &self.data {
                    data.push_str(&line.text);
                    data.push('\n');
                }
                // TODO(Connor): Handle file writing failure.
                file.write_all(data.as_bytes()).expect("Failed to write.");
            },
            None => {
                info!("Could not save file. Path is not set.");
                let _ = sender.send(SaveResult::PromptForPath);
            },
        }
    }

    fn handle_replace_text(&mut self, begin: Point, end: Option<Point>, text: String) {
        info!("Replacing text between {:?} and {:?} with {:?}", begin, end, text);

        let end = match end {
            Some(end) => end,
            None => begin,
        };

        let lines = {
            let before_lines = &self.data[..begin.line];
            let before_text = &self.data[begin.line][..begin.index];
            let after_text = &self.data[end.line][end.index..];
            let after_lines = &self.data[end.line + 1..];

            let text = format!("{}{}{}", before_text, text, after_text);
            let mut lines = Vec::new();

            lines.extend_from_slice(before_lines);
            for line in text.lines() {
                lines.push(Line::new(line.into()));
            }
            lines.extend_from_slice(after_lines);
            lines
        };

        self.data = lines;
    }
}
