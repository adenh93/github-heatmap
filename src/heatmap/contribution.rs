use colored::{Color, Colorize};
use scraper::ElementRef;
use crate::{ColorValues, HeatmapError};

const LEVEL_ATTR: &str = "data-level";

#[derive(Debug, Clone, PartialEq)]
pub struct Contribution {
    pub heat_level: usize
}

impl Contribution {
    pub fn from_el(el: &ElementRef) -> Result<Self, HeatmapError> {
       let heat_level = Self::parse_heat_level(el)?;
       Ok(Contribution { heat_level })
    }

    pub fn render(&self, color: &ColorValues) -> String {
       let shade = match self.heat_level {
           0 => 0,
           1 => 64,
           2 => 127,
           3 => 191,
           _ => 255,
       };

       let fill = match color {
           ColorValues::Red => Color::TrueColor { r: shade, g: 0, b: 0 },
           ColorValues::Green => Color::TrueColor { r: 0, g: shade, b: 0 },
           ColorValues::Blue => Color::TrueColor { r: 0, g: 0, b: shade },
       };

       "\u{025A0} ".color(fill).to_string()
    }

    fn parse_heat_level(el: &ElementRef) -> Result<usize, HeatmapError> {
        let heat_level = el
           .value()
           .attr(LEVEL_ATTR)
           .ok_or_else(|| HeatmapError::QueryAttribute { 
               attr: LEVEL_ATTR.to_string(), 
               on_alias: "heatmap node".to_string()
           })?
           .parse()
           .map_err(|_| HeatmapError::ParseAttribute { 
               attr: LEVEL_ATTR.to_string(),
               on_alias: "heatmap node".to_string()
           })?;

        Ok(heat_level)
    }
}

#[cfg(test)] 
mod tests {
    use super::*;
    use scraper::{Html, Selector};

    #[test]
    fn constructs_contribution() {
        let fragment = Html::parse_fragment("<rect y='15' data-level='3' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let contribution = Contribution::from_el(&rect_el).unwrap();

        assert_eq!(contribution, Contribution { heat_level: 3 })
    }

    #[test]
    fn parses_level_attribute() {
        let fragment = Html::parse_fragment("<rect y='15' data-level='3' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let heat_level = Contribution::parse_heat_level(&rect_el).unwrap();

        assert_eq!(heat_level, 3)
    }

    #[test]
    fn error_if_no_level_attribute() {
        let fragment = Html::parse_fragment("<rect y='15' data-heat-level='3' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let heat_level = Contribution::parse_heat_level(&rect_el);

        assert_eq!(
            heat_level, 
            Err(HeatmapError::QueryAttribute { attr: LEVEL_ATTR.to_string(), on_alias: "heatmap node".to_string() })
        )
    } 

    #[test]
    fn error_if_invalid_level_attribute() {
        let fragment = Html::parse_fragment("<rect y='15' data-level='three' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let heat_level = Contribution::parse_heat_level(&rect_el);

        assert_eq!(
            heat_level, 
            Err(HeatmapError::ParseAttribute { attr: LEVEL_ATTR.to_string(), on_alias: "heatmap node".to_string() })
        )
    }

    #[test]
    fn renders_heatmap_node_unfilled() {
        let contribution = Contribution { heat_level: 0 };
        let color = ColorValues::Green;
        let expected = "\u{025A0} ".color(Color::TrueColor { r: 0, g: 0, b: 0 }).to_string();

        assert_eq!(contribution.render(&color), expected);
    }
    
    #[test]
    fn renders_heatmap_node_red() {
        let contribution = Contribution { heat_level: 1 };
        let color = ColorValues::Red;
        let expected = "\u{025A0} ".color(Color::TrueColor { r: 64, g: 0, b: 0 }).to_string();

        assert_eq!(contribution.render(&color), expected);
    }


    #[test]
    fn renders_heatmap_node_green() {
        let contribution = Contribution { heat_level: 2 };
        let color = ColorValues::Green;
        let expected = "\u{025A0} ".color(Color::TrueColor { r: 0, g: 127, b: 0 }).to_string();

        assert_eq!(contribution.render(&color), expected);
    }

    #[test]
    fn renders_heatmap_node_blue() {
        let contribution = Contribution { heat_level: 3 };
        let color = ColorValues::Blue;
        let expected = "\u{025A0} ".color(Color::TrueColor { r: 0, g: 0, b: 191 }).to_string();

        assert_eq!(contribution.render(&color), expected);
    }
}
