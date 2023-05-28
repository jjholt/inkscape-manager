use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Style<'a> {
    pub style: &'a str,
    pub value: &'a str,
}

pub struct StyleList<'a,'b> {
    styles: &'a [Style<'b>]
}

impl <'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.style, self.value)
    }
}

impl <'a,'b> fmt::Display for StyleList<'a,'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let concat_strings: String = self.styles.iter().map(|style| style.to_string()).collect();
        write!(f, "{}", concat_strings)
    }
}

