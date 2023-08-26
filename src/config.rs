use std::fs;
use std::path::Path;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use styler::keybind::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Config<'a> {
    #[serde(borrow)]
    pub keybinds: Keybinds<'a>,
}

impl<'a> Config<'a> {
    pub fn get_raw_string() -> Result<String, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let conf_path = home + "/.config";
        let valid_files = [
            PathBuf::from(conf_path.clone() + "/inkscape-manager/config.yaml"),
            PathBuf::from(conf_path + "/.inkscape-manager.config.yaml"),
        ];

        let path = valid_files
            .iter()
            .find(|&f| Path::is_file(f))
            .map_or_else(|| Self::init_config(&valid_files[0]), |f| Ok(f))?;

        let config_str = fs::read_to_string(path)?;
        Ok(config_str)
    }
    fn init_config(path: &Path) -> Result<&Path, std::io::Error>{
        println!("Config file not found.", );
        let contents = &fs::read_to_string("config.yaml")?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, contents)?;
        println!("New config files created at: {:?}", path.parent().unwrap());
        Ok(path)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    const TEST_STR: &str = r#"
target: image/x-inkscape-svg
keybinds:
- key: a
  style: fill:000;
- key: b
  rebind_to: q
"#;

    #[test]
    fn build_config_from_yaml() {
        let yaml = "- key: a\n  style: fill:000\n- key: b\n  rebind_to: q";
        let keybinds: Keybinds = serde_yaml::from_str(yaml).unwrap();
        let config = Config {
            keybinds,
            // target: Some("image/x-inkscape-svg"),
        };
        println!("{}", serde_yaml::to_string(&config).unwrap());

        let config_from_yaml: Config = serde_yaml::from_str(TEST_STR).unwrap();
        assert_eq!(config, config_from_yaml);
    }
    #[test]
    fn load_from_default_locations() {
        let home = std::env::var("HOME").unwrap();
        let conf_path = home + "/.config";
        let config_dir = PathBuf::from(conf_path + "/inkscape-manager/config.yaml");

        let config_str = fs::read_to_string(config_dir).unwrap();
        let _: Config = serde_yaml::from_str(&config_str).unwrap();
    }
}
