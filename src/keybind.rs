use core::fmt;

use serde::{Serialize, Deserialize};

use crate::style::Style;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keybind {
    pub key: char, 
    #[serde(flatten)]
    pub style: Style,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keybinds {
    pub keybinds: Vec<Keybind>
}

impl Keybinds {
    pub fn get(&self, letter: Option<char>) -> Option<&Keybind> {
        self.keybinds
            .iter()
            .find(|keybind| Some(keybind.key) == letter)
    }
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
    use serde_yaml;
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
        let keybinds: Keybinds = serde_yaml::from_str(test_yaml).unwrap();
        println!("{}", keybinds);
        assert_eq!(keybinds.keybinds[0].style.style, "fill");
    }
    #[test]
    fn grab_key_from_config() {
        let test_yaml = r#"
keybinds:
- key: a
  style: fill
  value: 000000
- key: s
  style: fill
  value: aaaaaa
            "#;
        let keybinds: Keybinds = serde_yaml::from_str(test_yaml).unwrap();
        assert_eq!(
            keybinds.get(Some('a')).unwrap().style.style,
            String::from("fill")
        );
        assert_eq!(
            keybinds.get(Some('a')).unwrap().style.value,
            "000000"
            );
    }
}
