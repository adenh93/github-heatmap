use colored::{Color, Colorize};
use scraper::ElementRef;
use crate::{ColorValues, HeatmapError};

const LEVEL_ATTR: &str = "data-level";

/// A `Contribution` instance represents an invidividual heatmap node, with
/// a heat level corresponding to the data-level attribute set on the scraped
/// SVG Rect element.
///
/// `Contribution` instances are typically not constructed explicitly, rather created
/// implicitly by the higher level `Heatmap` struct via the `from_el` associated method.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Contribution {
    /// The `heat_level` property corresponds to the Rect element's data-level attribute,
    /// which Github uses to determine the intensity when shading the Rect element on
    /// the front end. 
    ///
    /// The `heat_level` property is utilised in the same way when deciding
    /// on the intensity of the filled Unicode box character.
    pub heat_level: usize
}

impl Contribution {
    /// Constructs a new `Contribution` instance from an HTML element.
    /// Provided element reference corresponds to scraped Github heatmap
    /// node.
    ///
    /// # Errors
    /// - [`HeatmapError::QueryAttribute`] fails to query heat level attribute
    /// - [`HeatmapError::ParseAttribute`] fails to parse heat level attribute
    ///
    pub fn from_el(el: &ElementRef) -> Result<Self, HeatmapError> {
       let heat_level = Self::parse_heat_level(el)?;
       Ok(Contribution { heat_level })
    }

    /// Renders a contribution node. 
    ///
    /// Returns a formatted string containing a Unicode box character, 
    /// with a fill color depending on the provided [`ColorValues`] variant, 
    /// and the `heat_level` property of the `Contribution` instance.
    ///
    pub fn render(&self, color: &ColorValues) -> String {
       let intensity = match self.heat_level {
           0 => 0,
           1 => 64,
           2 => 127,
           3 => 191,
           _ => 255,
       };

       let fill = match color {
           ColorValues::Red => Color::TrueColor { r: intensity, g: 0, b: 0 },
           ColorValues::Green => Color::TrueColor { r: 0, g: intensity, b: 0 },
           ColorValues::Blue => Color::TrueColor { r: 0, g: 0, b: intensity },
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
