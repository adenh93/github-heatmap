use scraper::{Selector, ElementRef};
use super::Contribution;

#[derive(Debug)]
pub struct ContributionWeek {
    pub contributions: Vec<Contribution>
}

impl ContributionWeek {
    pub fn from_el(el: &ElementRef, selector: &Selector) -> ContributionWeek {
        let contributions = el.select(selector)
             .map(|day| Contribution::from_el(&day)).collect();

        ContributionWeek { contributions }
    }
}
