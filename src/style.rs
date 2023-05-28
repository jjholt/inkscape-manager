use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Style<'a> {
    pub style: &'a str,
    pub value: &'a str,
}

impl <'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.style, self.value)
    }
}

pub trait DisplayVec {
    fn display(&self) -> String; 
}

impl <'a> DisplayVec for Vec<Style<'a>> {
    fn display(&self) -> String {
        self.iter().map(|style| style.to_string()).collect()
    }
}
