mod connection;
mod event_handler;
mod key;

mod clipboard;
mod config;

use config::Config;
use connection::Connections;

use event_handler::EventHandler;

use std::error::Error;

// fn read(filename: &Path) -> Result<String, Box<dyn Error>> {
//     let mut file = File::open(filename)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

fn main() -> Result<(), Box<dyn Error>> {
    let target = Some("image/x-inkscape-svg");
    let config_str = Config::get_raw_string()?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    println!("Keybinds:\n{}", &config.keybinds);
    // if let Some(target) = &config.target {
    //     println!("Target:\n{}", target);
    // }

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
