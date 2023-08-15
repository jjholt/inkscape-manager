mod connection;
mod event_handler;
mod key;

mod config;
mod clipboard;

use config::Config;
use connection::Connections;

use event_handler::EventHandler;

use std::{
    path::PathBuf,
    fs::File,
    error::Error,
    io::prelude::*,
};

fn read(filename: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = PathBuf::from("config.yaml");
    let contents = read(&config_path)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    println!("Keybinds:\n{}", &config.keybinds);
    if let Some(target) = &config.target {
        println!("Target:\n{}", target);
    }

    // let (xcb, screen_num) = xcb::Connection::connect(None)?;
    let (xcb, screen_num) = xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Test], &[])?;
    let connections = Connections::setup(&xcb, screen_num)?;
    let event_handler = EventHandler::new(&connections, &config.keybinds, &config.target);

    loop {
        event_handler.listen()?;
    }
}

