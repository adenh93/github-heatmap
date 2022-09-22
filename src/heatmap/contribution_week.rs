use scraper::{Selector, ElementRef};
use super::Contribution;

#[derive(Debug)]
pub struct ContributionWeek {
    pub contributions: Vec<Option<Contribution>>
}

impl ContributionWeek {
    pub fn from_el(el: &ElementRef, selector: &Selector) -> ContributionWeek {
        let mut contributions: Vec<Option<Contribution>> = vec![None; 7];

        el.select(selector).for_each(|day| {
            let y_value: usize = day.value().attr("y").unwrap().parse().unwrap();
            let contribution = Contribution::from_el(&day);

            // To my knowledge, Github uses either a y attribute of 13px or 15px while rendering 
            // the heatmap nodes, depending on the size of the heatmap on the profile.
            let day_index = match y_value {
                0 => 0,
                y if y % 13 == 0 => y / 13,
                y if y % 15 == 0 => y / 15,
                _ => panic!("Encountered an unexpected y_value: {y_value}"),
            };

            contributions[day_index] = Some(contribution);
        });

        ContributionWeek { contributions }
    }
}
