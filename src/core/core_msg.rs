use ::file::file_msg::{FileId, ToFileThreadMsg};

#[derive(Debug)]
pub enum ToCoreThreadMsg {
    FileThreadMsg(FileId, ToFileThreadMsg),
    SpawnFileThread,
}
