mod connection;
mod event_handler;
mod key;

mod clipboard;
mod config;

use config::Config;
use connection::Connections;

use event_handler::EventHandler;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let target = Some("image/x-inkscape-svg");
    let config_str = Config::get_raw_string()?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    println!("Keybinds:\n{}", &config.keybinds);

    let (xcb, screen_num) = xcb::Connection::connect(None)?;
    let connections = Connections::setup(&xcb, screen_num)?;
    let event_handler = EventHandler::new(&connections, &config.keybinds);

    loop {
        event_handler
            .listen()
            .unwrap_or(None)
            .map(|f| f.send(target));
    }
}
