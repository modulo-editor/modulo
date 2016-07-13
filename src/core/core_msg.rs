use ::file::file_msg::{FileId, ToFileThreadMsg};

/// Messages that can be sent to the core thread.
#[derive(Debug)]
pub enum ToCoreThreadMsg {
    /// Sends file thread message to the file thread matching the `FileId`
    FileThreadMsg(FileId, ToFileThreadMsg),
    /// Spawns a new file thread
    SpawnFileThread,
}
