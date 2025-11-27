mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    utils::init_logging(cli.quiet, cli.verbose);

    Ok(())
}
