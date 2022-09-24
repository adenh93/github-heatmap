mod contribution_week;
mod contribution;

pub use contribution_week::ContributionWeek;
pub use contribution::Contribution;
use scraper::{Selector, Html, ElementRef};
use crate::{ColorValues, HeatmapError};

const WEEK_SELECTOR: &str = "svg.js-calendar-graph-svg g g";
const DAY_SELECTOR: &str = "rect.ContributionCalendar-day";
const DAYS_IN_WEEK: usize = 7;

/// A `Heatmap` instance represents a fully scraped and parsed Github
/// constribution heatmap.
///
/// A `Heatmap` is constructed from a reference to a parsed HTML document
/// corresponding to a Github profile.
///
#[derive(Debug, Eq, PartialEq)]
pub struct Heatmap {
    /// A vector of [`ContributionWeek`] instances spanning across the entire
    /// year of contributions.
    pub contribution_weeks: Vec<ContributionWeek>,
}

impl Heatmap {
    /// Constructs a new `Heatmap` instance from a parsed HTML document.
    /// Provided reference to HTML document corresponds to the markup of a
    /// Github profile page (e.g. <https://github.com/torvalds>).
    ///
    /// [`ContributionWeek`] instances will be constructed and pushed to the 
    /// `contribution_weeks` vector for as many columns are evident in the parsed
    /// markup.
    ///
    /// # Errors
    /// - [`HeatmapError::QueryElement`] fails to query Heatmap SVG element.
    /// - [`HeatmapError::QueryElement`] fails to query Heatmap node elements
    /// 
    /// See [`ContributionWeek`] for errors related to constructing [`ContributionWeek`]
    /// instances.
    ///
    /// # Panics
    /// A panic will occur in the unlikely event that `Selector::parse` fails to parse 
    /// CSS selector constants.
    ///
    pub fn from_document(document: &Html) -> Result<Self, HeatmapError> {
        let contribution_week_selector = Selector::parse(WEEK_SELECTOR).unwrap();
        let day_selector = Selector::parse(DAY_SELECTOR).unwrap();
        let mut contribution_weeks = vec![];

        for el in document.select(&contribution_week_selector) {
            let week = Self::get_contribution_week(&el, &day_selector)?;
            contribution_weeks.push(week);
        }

        match &contribution_weeks.is_empty() {
            false => Ok(Heatmap { contribution_weeks }),
            true => Err(HeatmapError::QueryElement {
                alias: "heatmap".to_string(),
                selector: WEEK_SELECTOR.to_string()
            })
        }
    }

    /// Generates visual representation of Heatmap data structure,
    /// and writes it to standard output.
    ///
    /// Resulting Unicode will have a fill color depending on provided
    /// [`ColorValues`] color variant.
    ///
    pub fn render(&self, color: &ColorValues) {
        for day in 0..DAYS_IN_WEEK {
            let week: String = self.contribution_weeks
                .iter()
                .map(|week| match &week.contributions[day] {
                    Some(day) => day.render(color),
                    None => String::from("  ")
                })
                .collect();

            println!("{week}");
        };
    }

    fn get_contribution_week(el: &ElementRef, selector: &Selector) -> Result<ContributionWeek, HeatmapError> {
        let day_els: Vec<_> = el.select(selector).collect();
        
        if day_els.is_empty() {
            return Err(HeatmapError::QueryElement {
                alias: "heatmap node".to_string(),
                selector: DAY_SELECTOR.to_string()
            });
        }

        ContributionWeek::from_days(&day_els)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn constructs_heatmap() {
        let fragment = Html::parse_fragment(r#"
            <svg class="js-calendar-graph-svg">
                <g>
                    <g>
                        <rect y='45' data-level='1' class="ContributionCalendar-day" />
                        <rect y='60' data-level='2' class="ContributionCalendar-day" />
                        <rect y='75' data-level='3' class="ContributionCalendar-day" />
                        <rect y='90' data-level='4' class="ContributionCalendar-day" />
                    </g>
                    <g>
                        <rect y='0' data-level='1' class="ContributionCalendar-day" />
                        <rect y='15' data-level='2' class="ContributionCalendar-day" />
                        <rect y='30' data-level='3' class="ContributionCalendar-day" />
                        <rect y='45' data-level='4' class="ContributionCalendar-day" />
                        <rect y='60' data-level='4' class="ContributionCalendar-day" />
                        <rect y='75' data-level='4' class="ContributionCalendar-day" />
                        <rect y='90' data-level='4' class="ContributionCalendar-day" />
                    </g>
               </g> 
            </svg>
        "#);

        let heatmap = Heatmap::from_document(&fragment).unwrap();
        let expected = Heatmap { 
            contribution_weeks: vec![
                ContributionWeek {
                    contributions: vec![
                        None,
                        None,
                        None,
                        Some(Contribution { heat_level: 1 }),
                        Some(Contribution { heat_level: 2 }),
                        Some(Contribution { heat_level: 3 }),
                        Some(Contribution { heat_level: 4 }),
                    ]
                },
                ContributionWeek {
                    contributions: vec![
                        Some(Contribution { heat_level: 1 }),
                        Some(Contribution { heat_level: 2 }),
                        Some(Contribution { heat_level: 3 }),
                        Some(Contribution { heat_level: 4 }),
                        Some(Contribution { heat_level: 4 }),
                        Some(Contribution { heat_level: 4 }),
                        Some(Contribution { heat_level: 4 }),
                    ]
                }
            ]
        };

        assert_eq!(heatmap, expected) 
    }

    #[test]
    fn gets_contribution_week() {
        let fragment = Html::parse_fragment(r#"
            <g>
                <rect y='0' data-level='1' class="ContributionCalendar-day" />
                <rect y='15' data-level='2' class="ContributionCalendar-day" />
                <rect y='30' data-level='3' class="ContributionCalendar-day" />
                <rect y='45' data-level='4' class="ContributionCalendar-day" />
                <rect y='60' data-level='4' class="ContributionCalendar-day" />
                <rect y='75' data-level='4' class="ContributionCalendar-day" />
                <rect y='90' data-level='4' class="ContributionCalendar-day" />
            </g>
        "#);

        let el = fragment.root_element();
        let selector = Selector::parse(DAY_SELECTOR).unwrap();
        let contribution_week = Heatmap::get_contribution_week(&el, &selector).unwrap();

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
    fn error_if_cannot_parse_contribution_week() {
        let fragment = Html::parse_fragment(r#"
            <rect y='0' data-level='1' class="InvalidClass" />
            <rect y='15' data-level='2' class="InvalidClass" />
            <rect y='30' data-level='3' class="InvalidClass" />
            <rect y='45' data-level='4' class="InvalidClass" />
            <rect y='60' data-level='4' class="InvalidClass" />
            <rect y='75' data-level='4' class="InvalidClass" />
            <rect y='90' data-level='4' class="InvalidClass" />
        "#);

        let el = fragment.root_element();
        let selector = Selector::parse(DAY_SELECTOR).unwrap();
        let contribution_week = Heatmap::get_contribution_week(&el, &selector);
        let expected = Err(HeatmapError::QueryElement { 
            alias: "heatmap node".to_string(), 
            selector: DAY_SELECTOR.to_string(), 
        });

        assert_eq!(contribution_week, expected)
    }
}


