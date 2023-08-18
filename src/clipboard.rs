use std::{
    io,
    process::{Command, Stdio},
};

pub trait Clipboard {
    fn to_clipboard(&self, target: Option<&str>) -> io::Result<&String>;
}

impl Clipboard for String {
    fn to_clipboard(&self, target: Option<&str>) -> io::Result<&Self> {
        // println!("{}", self);
        let echo_child = Command::new("echo")
            .arg(self)
            .stdout(Stdio::piped())
            .spawn()?;
        let echo_out = echo_child.stdout.expect("Failed to open echo out");

        let mut command = Command::new("xclip");
        command
            .stdin(Stdio::from(echo_out))
            .args(["-selection", "clipboard"]);

        if let Some(target) = target {
            command.args(["-t", target]);
        }

        command.spawn()?.wait_with_output()?;
        // let child = command.stdin(Stdio::piped()).spawn()?;
        //
        // child.stdin.unwrap().write_all(self.as_bytes())?;
        Ok(self)
    }
}
