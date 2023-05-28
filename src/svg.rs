use crate::style::{Style, StyleList} ;

pub struct SVG <'a> {
    pub styles: StyleList<'a>
}

impl <'a> SVG<'a> {
    pub fn output(&self) -> String {
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg><defs/><g></g>\
                <inkscape:clipboard style=\"{}\" /> </svg>",
                self.styles
               )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_style() {
        let my_style = Style { style: "fill".to_string(), value: "#000".to_string() };
        assert_eq!(my_style.to_string(), "fill:#000;");
    }
    #[test]
    fn iterates() {
        let my_styles: StyleList = StyleList { styles: &vec![
            Style { style: "fill".to_string(), value: "#000".to_string() },
            Style { style: "stroke".to_string(), value: "#000".to_string() },
            Style { style: "stroke-width".to_string(), value: "0.7".to_string() },
        ]};
        assert_eq!(
            format!("{}", my_styles),
           "fill:#000;stroke:#000;stroke-width:0.7;"
       );
    }
    #[test]
    fn correct_output() {
        let styles = StyleList {
            styles: &vec![Style { style: "fill".to_string(), value: "#000".to_string() }]
        };
        let my_svg = SVG {styles};

        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg><defs/><g></g>\
            <inkscape:clipboard style=\"fill:#000;\" /> </svg>",
            &my_svg.output()
        );
    }
}
