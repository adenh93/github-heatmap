mod contribution_week;
mod contribution;

use contribution_week::ContributionWeek;
use contribution::Contribution;

use scraper::{Selector, Html};
use crate::ColorValues;

const WEEK_SELECTOR: &str = "svg.js-calendar-graph-svg g g";
const DAY_SELECTOR: &str = "rect.ContributionCalendar-day";
const DAYS_IN_WEEK: usize = 7;

#[derive(Debug)]
pub struct Heatmap {
    pub contribution_weeks: Vec<ContributionWeek>,
}

impl Heatmap {
    pub fn from_document(document: &Html) -> Self {
        let contribution_week_selector = Selector::parse(WEEK_SELECTOR).unwrap();
        let day_selector = Selector::parse(DAY_SELECTOR).unwrap();

        let contribution_weeks = document.select(&contribution_week_selector)
            .map(|el| ContributionWeek::from_el(&el, &day_selector)).collect();

        Heatmap { contribution_weeks }
    }

    pub fn generate(&self, color: &ColorValues) {
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
}
