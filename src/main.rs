mod cli;
mod commands;
mod schema;
mod document;
mod validation;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::conform;

fn main() -> Result<()> {
    let cli = Cli::parse();

    utils::init_logging(cli.quiet, cli.verbose);

    Ok(())
}
