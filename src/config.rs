use std::{
    path::PathBuf,
    fs::File,
    error::Error,
    io::prelude::*,
};

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::keybind::Keybinds;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub keybinds: Keybinds,
    pub target: String
}

impl Config {
    pub fn new(filename: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let contents = Self::read(filename)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)

    }
    fn read(filename: &PathBuf) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

