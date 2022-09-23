use colored::{Color, Colorize};
use scraper::ElementRef;
use crate::{ColorValues, HeatmapError};

const LEVEL_ATTR: &str = "data-level";

#[derive(Debug, Clone)]
pub struct Contribution {
    pub heat_level: usize
}

impl Contribution {
    pub fn from_el(el: &ElementRef) -> Result<Self, HeatmapError> {
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
}
