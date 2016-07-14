#[macro_use]
extern crate log;

pub mod core;
pub mod file;

use core::core_msg::ToCoreThreadMsg;
use core::core_thread::Core;
use file::file_msg::{FileThreadId, ToFileThreadMsg};
use file::text::Point;
use std::path::PathBuf;
use std::sync::mpsc;

fn main() {
    // TODO(Connor): Some basic testing. This should be removed and this will be moved to a library.
    let (core, handle) = Core::start();
    let _ = core.send(ToCoreThreadMsg::SpawnFileThread(Some(PathBuf::from("test.txt"))));
    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::ClearAllText));

    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::ReplaceText(Point::new(0,0), None, "Nope".into())));

    let (s, r) = mpsc::channel();
    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::Save(s)));

    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::ReplaceText(Point::new(0,0), Some(Point::new(0,4)), "Yup".into())));
    let (s, r) = mpsc::channel();

    let _ = core.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::Save(s)));

    handle.join();
}
