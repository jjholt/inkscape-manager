use std::{process::{Command, Stdio}, io::{Write, self}};

pub trait Clipboard {
    fn to_clipboard(&self, target: Option<&str>) -> io::Result<&String>;

    fn paste(&self, target: Option<&str>) -> io::Result<&String>;
}

impl Clipboard for String {
    fn to_clipboard(&self, target: Option<&str>) -> io::Result<&Self> {
        let mut command = Command::new("xclip");
        command.args(["-selection", "clipboard"]);
        if let Some(target_val) = target {
            command.args(["-t", target_val]);
        }
        // command.output()?;
        let child = command
            .stdin(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .unwrap()
            .write_all(self.as_bytes())?;
        Ok(self)
    }

    fn paste(&self, target: Option<&str>) -> io::Result<&Self> {
        let mut command = Command::new("xclip");
        command.args(["-selection", "clipboard", "-o"]);
        if let Some(target_val) = target {
            command.args(["-t", target_val]);
        }
        let child = command
            .stdin(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .unwrap()
            .write_all(self.as_bytes())?;
        Ok(self)
    }

}
