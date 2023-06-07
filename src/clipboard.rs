use std::{process::{Command, Stdio}, io::Write};

pub trait Clipboard {
    fn copy(&self, target: Option<&str>);


    fn paste() {
        unimplemented!();
    }
}

impl Clipboard for String {
    fn copy(&self, target: Option<&str>) {
        let mut command = Command::new("xclip");
        command.args(["-o", "-selection", "clipboard"]);
        if let Some(target_val) = target {
            command.args(["-t", target_val]);
        }
        let child = command
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        child
            .stdin
            .unwrap()
            .write_all(self.as_bytes())
            .unwrap();

    }
}
