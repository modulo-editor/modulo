use ::core::core_msg::ToCoreThreadMsg;
use ::file::file_msg::{FileId, ToFileThreadMsg};
use ::file::text::{Line, Point};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

/// A file thread represents one open file. It contains all the information about the data within
/// that file and listens for messages to manipulate the data within the file.
pub struct FileThread {
    id: FileId,
    core_sender: Sender<ToCoreThreadMsg>,
    core_receiver: Receiver<ToFileThreadMsg>,
    data: Vec<Line>,
}

impl FileThread {
    pub fn start(id: FileId, sender: Sender<ToCoreThreadMsg>, receiver: Receiver<ToFileThreadMsg>) {
        thread::spawn(move || {
            println!("Spawning file thread.");
            let mut file_thread = FileThread {
                id: id,
                core_sender: sender,
                core_receiver: receiver,
                data: Vec::new(),
            };
            file_thread.run();
        });
    }

    /// Runs the event loop for the `FileThread`
    pub fn run(&mut self) {
        while let Ok(msg) = self.core_receiver.recv() {
            match msg {
                ToFileThreadMsg::ReplaceText(begin, end, text) =>
                    self.handle_replace_text(begin, end, text),
            }
        }
    }

    fn handle_replace_text(&mut self, begin: Point, end: Option<Point>, text:String) {
        println!("Replacing Text.");
    }
}
