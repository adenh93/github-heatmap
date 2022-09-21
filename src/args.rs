use clap::{Parser, ValueEnum};

/// Scrapes a Github profile, and generates a contributions heatmap in Unicode
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Github profile slug, e.g. adenh93
    #[clap(value_parser)]
    pub slug: String,

    /// Heatmap color scheme. Nodes will be shaded depending on heat level.
    #[clap(short, long, value_enum, default_value_t = ColorValues::Green)]
    pub color: ColorValues,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ColorValues {
    Red,
    Green,
    Blue,
}
