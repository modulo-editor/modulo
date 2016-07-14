#[macro_use]
extern crate log;

pub mod core;
pub mod file;

use core::core_msg::ToCoreThreadMsg;
use core::core_thread::Core;
use file::file_msg::{FileThreadId, ToFileThreadMsg};
use std::path::PathBuf;
use std::sync::mpsc;

fn main() {
    let (core, handle) = Core::start();
    let _ = core.send(ToCoreThreadMsg::SpawnFileThread(Some(PathBuf::from("test.txt"))));
    let (s, r) = mpsc::channel();
    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::Save(s)));

    handle.join();
}
