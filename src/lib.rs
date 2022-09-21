mod args;

pub use args::Args;
use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Ok(())
}
