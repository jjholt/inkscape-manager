use crate::style::{Style, DisplayVec} ;

pub struct SVG <'a> {
    styles: Vec<Style<'a>>,
}

impl <'a> SVG<'a> {
    pub fn output(&self) -> String {
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg><defs/><g></g>\
                <inkscape:clipboard style=\"{}\" /> </svg>",
                self.styles.display()
               )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_style() {
        let my_style = Style { key: "fill", value: "#000" };
        assert_eq!(my_style.to_string(), "fill:#000;");
    }
    #[test]
    fn iterates() {
        let my_styles = vec![
            Style { key: "fill", value: "#000" },
            Style { key: "stroke", value: "#000" },
            Style { key: "stroke-width", value: "0.7" },
        ];
        assert_eq!(
            format!("{}", my_styles.display()),
           "fill:#000;stroke:#000;stroke-width:0.7;"
       );
    }
    #[test]
    fn correct_output() {
        let styles = vec![Style { key: "fill", value: "#000" }];
        let my_svg = SVG { styles };

        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg><defs/><g></g>\
            <inkscape:clipboard style=\"fill:#000;\" /> </svg>".to_string(),
            my_svg.output()
        );
    }
}
