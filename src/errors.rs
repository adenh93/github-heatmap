use thiserror::Error;
use reqwest::StatusCode;

#[derive(Error, Debug, PartialEq)]
pub enum GithubError {
    #[error("Unable to reach Github services. Try again later!")]
    BadRequest,
    
    #[error("Unable to scrape Github profile: '{0}'")]
    ScrapeFailure(StatusCode),

    #[error("Unable to find Github profile at url: '{0}'")]
    ProfileNotFound(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum HeatmapError {
    #[error("Failed to query element '{alias}' with selector: '{selector}'")]
    QueryElement { alias: String, selector: String },

    #[error("Failed to query attribute '{attr}' on '{on_alias}'!")]
    QueryAttribute { attr: String, on_alias: String },

    #[error("Failed to parse attribute '{attr}' on '{on_alias}'!")]
    ParseAttribute { attr: String, on_alias: String },

    #[error("Failed to parse Heatmap nodes. Unknown node size scraped from Github frontend.")]
    UnknownNodeFormat
}
