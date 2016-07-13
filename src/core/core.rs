use ::core::core_msg::ToCoreThreadMsg;
use ::file::file_msg::{FileId, ToFileThreadMsg};
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Core {
    file_threads: HashMap<FileId, Sender<ToFileThreadMsg>>,
    file_receiver: Receiver<ToCoreThreadMsg>,
    file_sender: Sender<ToCoreThreadMsg>,
}

impl Core {
    pub fn new() -> Core {
        let (sender, receiver) = mpsc::channel();
        Core {
            file_threads: HashMap::new(),
            file_receiver: receiver,
            file_sender: sender,
        }
    }

    /// Runs the event loop for the `CoreThread`
    pub fn run(&mut self) {
        while let Ok(msg) = self.file_receiver.recv() {
            match msg {
                ToCoreThreadMsg::FileThreadMsg(id, msg) => {
                    let file_thread = match self.file_threads.get(&id) {
                        Some(file_thread) => file_thread,
                        None => return warn!("Received message for closed file thread: {:?}", id),
                    };

                    let _ = file_thread.send(msg);
                }
            }
        }
    }
}
