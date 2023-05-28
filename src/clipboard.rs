use std::process::Command;

fn copy(text: &str, target: Option<&str>) {
    let mut command = Command::new("xclip");
    command.args(["-selection", "clipboard"]);
    if let Some(target_val) = target {
        command.args(["-t", target_val]);
    }
    command.spawn().unwrap();
}
