use clap::Parser;
use std::process;
use github_heatmap::{run, Args};

fn main() {
    let args = Args::parse();  

    if let Err(e) = run(&args) {
        eprintln!("An error occurred: {e}");
        process::exit(1);
    }
}
