use clap::Parser;
use std::process;
use github_heatmap::{run, Args};

fn main() {
    let args = Args::parse();  
    println!("{args:?}");

    if let Err(e) = run() {
        eprintln!("An error occurred: {e:?}");
        process::exit(1);
    }
}
