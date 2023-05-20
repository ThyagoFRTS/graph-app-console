use std::process::Command;

pub fn clear_console() {
    Command::new("cmd")
    .args(["/C", "cls"])
    .output()
    .expect("falha ao acessar cmd");
}