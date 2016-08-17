//! The file thread handles receiving messages related to file manipulation

use file::file_data::FileData;
use modulo_traits::core_msg::ToCoreThreadMsg;
use modulo_traits::file_msg::{FileThreadId, ToFileThreadMsg, SaveResult};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

/// A file thread represents one open file. It contains all the information about the data within
/// that file and listens for messages to manipulate the data within the file.
pub struct FileThread {
    id: FileThreadId,
    core_sender: Sender<ToCoreThreadMsg>,
    core_receiver: Receiver<ToFileThreadMsg>,
    file: FileData,
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
                file: FileData::new(),
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
                ToFileThreadMsg::ReplaceText(text) => {
                    info!("Received replace text file message.");
                    self.file.replace_text(&text);
                },
                ToFileThreadMsg::ClearAllText => {
                    info!("Received clear all text message.");
                    self.file.clear_all_text();
                },
                ToFileThreadMsg::SetSelections(selection_ranges) => {
                    info!("Received set selections message");
                    self.file.set_selections(selection_ranges);
                }
                ToFileThreadMsg::Save(sender) => {
                    info!("Received save file message.");
                    self.handle_save(sender);
                },
            }
        }
    }

    fn load_file(&mut self) {
        if let Some(ref path) = self.path {
            let path = path.as_path();

            if !path.exists() || !path.is_file() {
                return warn!("Illegal path, cannot load file.");
            }

            self.file.load_from_file(path);
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
                for line in &self.file.lines {
                    data.push_str(&line.text);
                    data.push('\n');
                }
                // TODO(Connor): Handle file writing failure.
                file.write_all(data.as_bytes()).expect("Failed to write.");
                let _ = sender.send(SaveResult::Ok);
            },
            None => {
                warn!("Could not save file. Path is not set.");
                let _ = sender.send(SaveResult::PromptForPath);
            },
        }
    }
}
