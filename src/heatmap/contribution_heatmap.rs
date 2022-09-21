use scraper::{Selector, Html};
use super::ContributionWeek;

const WEEK_SELECTOR: &str = "svg.js-calendar-graph-svg g g";
const DAY_SELECTOR: &str = "rect.ContributionCalendar-day";

#[derive(Debug)]
pub struct ContributionHeatmap {
    pub contribution_weeks: Vec<ContributionWeek>
}

impl ContributionHeatmap {
    pub fn from_document(document: &Html) -> Self {
        let contribution_week_selector = Selector::parse(WEEK_SELECTOR).unwrap();
        let day_selector = Selector::parse(DAY_SELECTOR).unwrap();

        let contribution_weeks = document.select(&contribution_week_selector)
            .map(|el| ContributionWeek::from_el(&el, &day_selector)).collect();

        ContributionHeatmap { contribution_weeks }
    }

    pub fn render(&self) {
        todo!();
    }
}
