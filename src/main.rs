#[macro_use]
extern crate log;

pub mod core;
pub mod file;

use core::core_msg::ToCoreThreadMsg;
use core::core_thread::Core;

fn main() {
    let (core, handle) = Core::start();
    let _ = core.send(ToCoreThreadMsg::SpawnFileThread);

    handle.join();
}
