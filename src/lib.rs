mod args;
mod heatmap;

pub use args::{Args, ColorValues};
use std::error;
use scraper::Html;
use heatmap::Heatmap;

const PROFILE_URL: &str = "https://github.com";

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let profile_url = match &args.year {
        Some(year) => format!("{PROFILE_URL}/{slug}?from={year}-01-01", slug = args.slug),
        None => format!("{PROFILE_URL}/{slug}", slug = args.slug)
    };

    let body = reqwest::blocking::get(profile_url)?.text()?;
    let document = Html::parse_document(&body);

    let heatmap = Heatmap::from_document(&document);
    heatmap.generate(&args.color);

    Ok(())
}
