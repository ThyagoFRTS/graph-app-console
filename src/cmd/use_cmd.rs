use std::process::Command;

pub fn clear_console() {
    let _ = Command::new("cmd")
    .args(["/C", "cls"])
    .status();
}