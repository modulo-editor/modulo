#[macro_use]
extern crate log;

pub mod core;
pub mod file;

use core::core_msg::ToCoreThreadMsg;
use core::core_thread::Core;
use std::path::PathBuf;

fn main() {
    let (core, handle) = Core::start();
    let _ = core.send(ToCoreThreadMsg::SpawnFileThread(Some(PathBuf::from("Cargo.toml"))));

    handle.join();
}
