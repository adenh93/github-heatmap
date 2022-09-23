mod contribution_week;
mod contribution;

use contribution_week::ContributionWeek;
use contribution::Contribution;

use scraper::{Selector, Html};
use crate::{ColorValues, HeatmapError};

const WEEK_SELECTOR: &str = "svg.js-calendar-graph-svg g g";
const DAY_SELECTOR: &str = "rect.ContributionCalendar-day";
const DAYS_IN_WEEK: usize = 7;

#[derive(Debug)]
pub struct Heatmap {
    pub contribution_weeks: Vec<ContributionWeek>,
}

impl Heatmap {
    pub fn from_document(document: &Html) -> Result<Self, HeatmapError> {
        let contribution_week_selector = Selector::parse(WEEK_SELECTOR).unwrap();
        let day_selector = Selector::parse(DAY_SELECTOR).unwrap();
        let mut contribution_weeks = vec![];

        for el in document.select(&contribution_week_selector) {
            let day_els: Vec<_> = el.select(&day_selector).collect();
        
            if day_els.is_empty() {
                return Err(HeatmapError::QueryElement {
                    alias: "heatmap node".to_string(),
                    selector: DAY_SELECTOR.to_string()
                });
            }

            let week = ContributionWeek::from_days(&day_els)?;
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
