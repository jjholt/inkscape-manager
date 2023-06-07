use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    pub style: String, 
    pub value: String,
}

// pub struct StyleList<'a> {
//     pub styles: &'a [Style]
// }

pub struct StyleList <'a,'b>{
    pub styles: &'a [&'b Style]
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.style, self.value)
    }
}

impl <'a, 'b>fmt::Display for StyleList <'a, 'b>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let concat_strings: String = self.styles.iter().map(|style| style.to_string()).collect();
        write!(f, "{}", concat_strings)
    }
}

