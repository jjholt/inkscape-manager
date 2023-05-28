use crate::style::{Style, StyleList} ;

pub struct SVG <'a, 'b> {
    styles: StyleList<'a,'b>
}

impl <'a, 'b> SVG<'a, 'b> {
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
        let my_style = Style { style: "fill", value: "#000" };
        assert_eq!(my_style.to_string(), "fill:#000;");
    }
    #[test]
    fn iterates() {
        let my_styles: StyleList = StyleList { styles: &vec![
            Style { style: "fill", value: "#000" },
            Style { style: "stroke", value: "#000" },
            Style { style: "stroke-width", value: "0.7" },
        ]};
        assert_eq!(
            format!("{}", my_styles),
           "fill:#000;stroke:#000;stroke-width:0.7;"
       );
    }
    #[test]
    fn correct_output() {
        let styles = StyleList {
            styles: &vec![Style { style: "fill", value: "#000" }]
        };
        let my_svg = SVG {styles};

        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg><defs/><g></g>\
            <inkscape:clipboard style=\"fill:#000;\" /> </svg>".to_string(),
            my_svg.output()
        );
    }
}
