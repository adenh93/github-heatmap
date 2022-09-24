use thiserror::Error;
use reqwest::StatusCode;

/// A collection of error variants related to making a request
/// to a Github profile page prior to scraping.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum GithubError {
    /// Represents a generic failure while trying to make a GET request
    /// to the specified Github profile page.
    #[error("Unable to reach Github services. Try again later!")]
    BadRequest,
    
    /// Represents a failure caused while attempting to scrape the 
    /// specified Github profile page. This is typically caused by either
    /// a Github service outage, or any other kind of rejection (e.g rate-limiting, etc).
    #[error("Unable to scrape Github profile: '{0}'")]
    ScrapeFailure(StatusCode),

    /// Represents a 404 response caused by attempting to scrape a non-existing
    /// Github profile page.
    #[error("Unable to find Github profile at url: '{0}'")]
    ProfileNotFound(String),
}

/// A collection of error variants related to parsing a Github contribution
/// heatmap.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum HeatmapError {
    /// Represents a failure to query an element in the scraped document.
    /// This is usually caused by an update to the Github front end.
    #[error("Failed to query element '{alias}' with selector: '{selector}'")]
    QueryElement { 
        /// User-friendly alias related to element.
        alias: String, 
        /// CSS Selector used while attempting to query element.
        selector: String 
    },

    /// Represents a failure to select an attribute on an HTML Element. This
    /// is usually caused by an update to the Github front end. 
    #[error("Failed to query attribute '{attr}' on '{on_alias}'!")]
    QueryAttribute { 
        /// Attribute name used while attempting to select attribute.
        attr: String, 
        /// User-friendly alias related to element that attribute purportedly belongs to.
        on_alias: String 
    },

    /// Represents a failure to parse an attribute on an HTML Element. For example,
    /// attempting to parse a Rect element's y attribute as a usize.
    /// This is usually caused by an update to the Github front end.
    #[error("Failed to parse attribute '{attr}' on '{on_alias}'!")]
    ParseAttribute { 
        /// Attribute name used while attempting to parse attribute.
        attr: String, 
        /// User-friendly alias related to element that attribute purportedly belongs to.
        on_alias: String 
    },

    /// Represents a failure to parse Heatmap node sizes from the SVG element belonging
    /// to the Github profile page. Elements are typically distanced either 13px or 15px
    /// depending on the density of the profile page (contains README.md, etc).
    #[error("Failed to parse Heatmap nodes. Unknown node size scraped from Github frontend.")]
    UnknownNodeFormat
}
