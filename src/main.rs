mod clipboard;
mod svg;
mod config;
mod style;
mod keybind;
mod connection;

use std::path::PathBuf;

use clipboard::Clipboard;
use config::Config;
use svg::SVG;

use crate::style::StyleList;

fn main() {
    let config_path = PathBuf::from("config.yaml");
    let config = Config::new(&config_path).expect("poop");
    let keybinds = config.keybinds;
    // let xlib_input: Option<char>;
    let target = config.target;

    // xlib_input = Some('a');

   let my_handler = |key| {
        if let Some(keybind) = keybinds.get(key) {
            let style_list = StyleList {styles: &[&keybind.style]};
            SVG::new(style_list)
                .generate_output()
                .copy(Some(&target));
        }
        Ok(())
    };
    connection::connect(&my_handler).unwrap();

}


#[cfg(test)]
mod implementation {
    use super::*;
    #[test]
    fn check_inkscape_is_active() {
        unimplemented!();
    }
    #[test]
    fn take_input() {
        unimplemented!();
    }
    #[test]
    fn produce_svg() {
        unimplemented!();
    }
    #[test]
    fn add_to_clipboard() {
        unimplemented!();
    }
    #[test]
    fn automatically_paste_clipboard() {
        unimplemented!();
    }
}
