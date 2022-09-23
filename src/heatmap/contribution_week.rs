use scraper::ElementRef;
use crate::HeatmapError;
use super::Contribution;

const Y_ATTR: &str = "y";

#[derive(Debug)]
pub struct ContributionWeek {
    pub contributions: Vec<Option<Contribution>>
}

impl ContributionWeek {
    pub fn from_days(days: &Vec<ElementRef>) -> Result<ContributionWeek, HeatmapError> {
        let mut contributions: Vec<Option<Contribution>> = vec![None; 7]; 

        for day in days {
            let y_value = day
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

            let contribution = Contribution::from_el(day)?;

            // To my knowledge, Github uses either a y attribute of 13px or 15px while rendering 
            // the heatmap nodes, depending on the size of the heatmap on the profile.
            let day_index = match y_value {
                0 => 0,
                y if y % 13 == 0 => y / 13,
                y if y % 15 == 0 => y / 15,
                _ => return Err(HeatmapError::UnknownNodeFormat)
            };

            contributions[day_index] = Some(contribution);
        }

        Ok(ContributionWeek { contributions })
    }
}
