mod args;
mod heatmap;

pub use args::Args;
use std::error;
use scraper::Html;
use heatmap::ContributionHeatmap;

const PROFILE_URL: &str = "https://github.com";

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let profile_url = format!("{PROFILE_URL}/{slug}", slug = args.slug);
    let body = reqwest::blocking::get(profile_url)?.text()?;
    let document = Html::parse_document(&body);

    let heatmap = ContributionHeatmap::from_document(&document);
    heatmap.render();

    Ok(())
}
