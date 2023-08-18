use serde::{Deserialize, Serialize};

use styler::keybind::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Config<'a> {
    pub keybinds: Keybinds<'a>,
    pub target: Option<&'a str>,
}
#[cfg(test)]
mod tests {
    use super::*;
    // const TEST_STR: &str = "keybinds:\n- key: a\n  style: fill\n  value: 000\n- key: b\n  rebind_to: q\ntarget: image/x-inkscape-svg";
    const TEST_STR: &str = r#"
target: image/x-inkscape-svg
keybinds:
- key: a
  style: fill
  value: 000
- key: b
  rebind_to: q
"#;

    #[test]
    fn build_config_from_yaml() {
        let yaml: &str = "- key: a\n  style: fill\n  value: 000\n- key: b\n  rebind_to: q";
        // let keybinds: Vec<Keybind> = serde_yaml::from_str(yaml).unwrap();
        let keybinds: Keybinds = serde_yaml::from_str(yaml).unwrap();
        let config = Config {
            keybinds,
            target: Some("image/x-inkscape-svg"),
        };
        println!("{}", serde_yaml::to_string(&config).unwrap());

        let config_from_yaml: Config = serde_yaml::from_str(TEST_STR).unwrap();
        assert_eq!(config, config_from_yaml);
    }
}
