use scraper::ElementRef;
use crate::HeatmapError;
use super::Contribution;

const Y_ATTR: &str = "y";

/// A `ContributionWeek` instance represents an entire week of contributions
/// in a Github contribution heatmap. Typically visible as a column of heatmap
/// nodes on a Github profile page.
///
/// `ContributionWeek` instances are typically not constructed explicitly, rather created
/// implicitly by the higher level `Heatmap` struct via the `from_days` associated method.
///
#[derive(Debug, Eq, PartialEq)]
pub struct ContributionWeek {
    /// A vector of [`Contribution`] instances belonging to the week. 
    ///
    /// Contributions are wrapped in an Option, as years won't necessarily begin
    /// and/or end on a Sunday, meaning that certain days may not be included in the
    /// heatmap during any given week.
    pub contributions: Vec<Option<Contribution>>
}

impl ContributionWeek {
    /// Contructs a new `ContributionWeek` instance from a vector of HTML elements.
    /// Provided vector corresponds to a collection of Github heatmap nodes.
    ///
    /// For each valid day of the week with contributions, a [`Contribution`] instance
    /// will be constructed and pushed to the `contributions` vector.
    ///
    /// # Errors
    /// - [`HeatmapError::QueryAttribute`] fails to query y attribute
    /// - [`HeatmapError::ParseAttribute`] fails to parse y attribute
    /// - [`HeatmapError::UnknownNodeFormat`] encounters unexpected heatmap node size while
    /// determining day of week for contributions
    ///
    /// See [`Contribution`] for possible errors related to constructing a ['Contribution'].
    ///
    pub fn from_days(days: &Vec<ElementRef>) -> Result<Self, HeatmapError> {
        let mut contributions: Vec<Option<Contribution>> = vec![None; 7]; 

        for day in days {
            let y_value = Self::parse_y_attr(day)?;
            let day_index = Self::get_day_index(y_value)?;
            let contribution = Contribution::from_el(day)?;

            contributions[day_index] = Some(contribution);
        }

        Ok(ContributionWeek { contributions })
    }

    fn parse_y_attr(day_el: &ElementRef) -> Result<usize, HeatmapError> {
        let result = day_el
            .value()
            .attr(Y_ATTR)
            .ok_or_else(|| HeatmapError::QueryAttribute { 
                attr: Y_ATTR.to_string(), 
                on_alias: "heatmap node".to_string(),
            })?
            .parse()
            .map_err(|_| HeatmapError::ParseAttribute { 
                attr: Y_ATTR.to_string(), 
                on_alias: "heatmap node".to_string(),
            })?;

        Ok(result)
    }

    fn get_day_index(y_value: usize) -> Result<usize, HeatmapError> {
        // To my knowledge, Github uses either a y attribute of 13px or 15px while rendering 
        // the heatmap nodes, depending on the size of the heatmap on the profile.
        match y_value {
            0 => Ok(0),
            y if y % 13 == 0 => Ok(y / 13),
            y if y % 15 == 0 => Ok(y / 15),
            _ => Err(HeatmapError::UnknownNodeFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::{Html, Selector};

    #[test]
    fn constructs_contribution_week() {
        let fragment = Html::parse_fragment(r#"
            <rect y='0' data-level='1' />
            <rect y='15' data-level='2' />
            <rect y='30' data-level='3' />
            <rect y='45' data-level='4' />
            <rect y='60' data-level='4' />
            <rect y='75' data-level='4' />
            <rect y='90' data-level='4' />
        "#);

        let selector = Selector::parse("rect").unwrap();
        let rects: Vec<_> = fragment.select(&selector).collect();
        let contribution_week = ContributionWeek::from_days(&rects).unwrap();

        let expected = ContributionWeek {
            contributions: vec![
                Some(Contribution { heat_level: 1 }),
                Some(Contribution { heat_level: 2 }),
                Some(Contribution { heat_level: 3 }),
                Some(Contribution { heat_level: 4 }),
                Some(Contribution { heat_level: 4 }), 
                Some(Contribution { heat_level: 4 }),
                Some(Contribution { heat_level: 4 }),
            ]
        };

        assert_eq!(contribution_week, expected) 
    }

    #[test]
    fn constructs_partial_contribution_week() {
        let fragment = Html::parse_fragment(r#"
            <rect y='60' data-level='1' />
            <rect y='75' data-level='2' />
            <rect y='90' data-level='3' />
        "#);

        let selector = Selector::parse("rect").unwrap();
        let rects: Vec<_> = fragment.select(&selector).collect();
        let contribution_week = ContributionWeek::from_days(&rects).unwrap();

        let expected = ContributionWeek {
            contributions: vec![
                None,
                None,
                None,
                None,
                Some(Contribution { heat_level: 1 }), 
                Some(Contribution { heat_level: 2 }),
                Some(Contribution { heat_level: 3 }),
            ]
        };

        assert_eq!(contribution_week, expected) 
    }

    #[test]
    fn parses_y_attribute() {
        let fragment = Html::parse_fragment("<rect y='15' data-level='3' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let y_value = ContributionWeek::parse_y_attr(&rect_el).unwrap();

        assert_eq!(y_value, 15)
    }

    #[test]
    fn error_if_no_y_attribute() {
        let fragment = Html::parse_fragment("<rect data-level='3' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let y_value = ContributionWeek::parse_y_attr(&rect_el);

        assert_eq!(
            y_value, 
            Err(HeatmapError::QueryAttribute { attr: Y_ATTR.to_string(), on_alias: "heatmap node".to_string() })
        )
    } 

    #[test]
    fn error_if_invalid_y_attribute() {
        let fragment = Html::parse_fragment("<rect y='fifteen' data-level='three' />");
        let selector = Selector::parse("rect").unwrap();
        let rect_el = fragment.select(&selector).next().unwrap();
        let contribution = ContributionWeek::parse_y_attr(&rect_el);

        assert_eq!(
            contribution, 
            Err(HeatmapError::ParseAttribute { attr: Y_ATTR.to_string(), on_alias: "heatmap node".to_string() })
        )
    }

    #[test]
    fn gets_first_day_index() {
        let day_index = ContributionWeek::get_day_index(0).unwrap();
        assert_eq!(day_index, 0)
    }

    #[test]
    fn gets_large_day_index() {
        let day_index = ContributionWeek::get_day_index(30).unwrap();
        assert_eq!(day_index, 2)
    }

    #[test]
    fn gets_small_day_index() {
        let day_index = ContributionWeek::get_day_index(26).unwrap();
        assert_eq!(day_index, 2)
    }

    #[test]
    fn error_if_unknown_node_format() {
        let result = ContributionWeek::get_day_index(20);
        assert_eq!(result, Err(HeatmapError::UnknownNodeFormat))
    }
}
