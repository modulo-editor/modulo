pub mod core;

use std::thread;
use std::sync::mpsc;
use core::Core;
use core::messages::Message;

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();

    let main_loop = thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            match message {
                Message::SendText(text) => println!("{}", text),
                Message::AddOneAndPrint(num) => println!("{:?}", num + 1),
            }
        }
    });

    let tx_clone = tx.clone();

    let plugin = thread::spawn(move || {
        let mut i = 0;
        loop {
            let _ = tx_clone.send(Message::AddOneAndPrint(i));
            i += 1;
        }
    });

    let plugin = thread::spawn(move || {
        let mut i = 0;
        loop {
            let _ = tx.send(Message::SendText("Yo".into()));
            i += 1;
        }
    });

    // Wait for threads to finish running before closing
    let _ = plugin.join();
    let _ = main_loop.join();

    println!("Hello, world!");
}
