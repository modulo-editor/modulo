use file_msg::{FileThreadId, ToFileThreadMsg};
use std::path::PathBuf;

/// Messages that can be sent to the core thread.
#[derive(Debug)]
pub enum ToCoreThreadMsg {
    /// Sends file thread message to the file thread matching the `FileThreadId`
    FileThreadMsg(FileThreadId, ToFileThreadMsg),
    /// Spawns a new file thread
    SpawnFileThread(Option<PathBuf>),
}
