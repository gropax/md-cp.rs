mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use std::fs;

fn main() -> Result<()> {
    let cli = Cli::parse();

    utils::init_logging(cli.quiet, cli.verbose);

    let template_content = fs::read_to_string(&cli.template_file)?;

    let document_files = utils::expand_globs(&cli.document_patterns)?;
    for document_file in &document_files {
        let document_content = fs::read_to_string(document_file)?;

        let mut new_content = utils::front_matter(&document_content);
        new_content.push_str(&template_content);

        fs::write(document_file, new_content)?;
    }

    Ok(())
}
