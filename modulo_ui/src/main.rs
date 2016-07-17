//! The modulo editor

use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    // TODO(Connor): Unhardcode this path.
    let mut p = Command::new("../modulo/target/debug/modulo")
        .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn().expect("Failed to start Modulo");
    {
        let mut p_stdin = p.stdin.as_mut().expect("Failed to open stdin.");
        p_stdin.write("test".as_bytes()).unwrap();
    }
    p.wait().unwrap().success();

    // TODO(Connor): Make modulo a library. modulo-core and modulo-ui can both add the modulo crate
    // to have access to the same structs for usage with serde(serialize/deserialize).
    p.stdout.as_mut().unwrap();
    p.wait().unwrap();
}
