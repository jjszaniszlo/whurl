mod cli;

use anyhow::Result;

use clap::Parser;

use crate::cli::Opts;

fn main() -> Result<()> {
    let opts = Opts::parse();

    Ok(())
}
