use core::fmt;

use serde::{Serialize, Deserialize};
use serde_yaml;

use crate::style::Style;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keybind {
    pub key: String, 
    #[serde(flatten)]
    pub style: Style,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Keybinds {
    pub keybinds: Vec<Keybind>
}

impl fmt::Display for Keybinds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.keybinds.iter()
            .map(|k| {
                write!(f, "{} => {}:{}\n", k.key, k.style.style, k.style.value) })
            .collect::<fmt::Result>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_yaml() {
        let test_yaml = r#"
keybinds:
- key: a
  style: fill
  value: 000000
- key: s
  style: fill
  value: aaaaaa
            "#;
        let keybinds: Keybinds = super::serde_yaml::from_str(test_yaml).unwrap();
        println!("{}", keybinds);
        assert_eq!(keybinds.keybinds[0].style.style, "fill");
    }
}
