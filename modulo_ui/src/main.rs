//! The modulo editor
extern crate modulo;
extern crate modulo_traits;

use modulo::core::core_thread::Core;
use modulo_traits::core_msg::ToCoreThreadMsg;
use modulo_traits::file_msg::{FileThreadId, ToFileThreadMsg};
use std::path::PathBuf;
use std::sync::mpsc;

fn main() {
    let (sender, handle) = Core::start();
    sender.send(ToCoreThreadMsg::SpawnFileThread(Some(PathBuf::from("test.txt"))));
    let (save_sender, save_receiver) = mpsc::channel();
    sender.send(ToCoreThreadMsg::FileThreadMsg(FileThreadId(0), ToFileThreadMsg::Save(save_sender)));
    save_receiver.recv().unwrap();

    handle.join();
}
