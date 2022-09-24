mod heatmap;
mod utils;

use std::error;
use reqwest::StatusCode;
use scraper::Html;
pub use heatmap::{Heatmap, ContributionWeek, Contribution};
pub use utils::args::{Args, ColorValues};
pub use utils::errors::{HeatmapError, GithubError};
pub use utils::parsers::{parse_slug, parse_year};

const PROFILE_URL: &str = "https://github.com";

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let profile_url = match &args.year {
        Some(year) => format!("{PROFILE_URL}/{slug}?from={year}-01-01", slug = args.slug),
        None => format!("{PROFILE_URL}/{slug}", slug = args.slug)
    };

    let profile = get_github_profile(&profile_url)?;
    let heatmap = Heatmap::from_document(&profile)?;
    heatmap.render(&args.color);

    Ok(())
}

fn get_github_profile(profile_url: &str) -> Result<Html, GithubError> {
    let response = reqwest::blocking::get(profile_url).unwrap();

    let body = match response.status() {
        StatusCode::OK => Ok(response.text().map_err(|_| GithubError::BadRequest)?),
        StatusCode::NOT_FOUND => Err(GithubError::ProfileNotFound(profile_url.to_string())),
        status => Err(GithubError::ScrapeFailure(status))
    }?;

    Ok(Html::parse_document(&body))
}
