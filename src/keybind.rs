use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::style::Style;

#[derive(Serialize, Deserialize)]
struct Keybind <'a> {
    key: &'a str,
    style: Style<'a>,
}

