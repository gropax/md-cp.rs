use clap::{ArgAction, Parser, ColorChoice};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "md-cp",
    version,
    author,
    about = "Batch replace markdown file content with template without erasing frontmatter.",
    long_about = None,
    color = ColorChoice::Auto,
)]
pub struct Cli {
    #[arg(
        value_name = "TEMPLATE",
        help = "Path to the template file in Markdown.",
    )]
    pub template_file: PathBuf,

    #[arg(
        value_name = "DOCUMENTS",
        help = "Paths to the markdown documents to modify. Glob patterns are supported.",
    )]
    pub document_patterns: Vec<String>,

    #[arg(
        short,
        long,
        action = ArgAction::Count,
        long_help = "Control the amount of logging:\n\
                     - no flag: warnings\n\
                     - -v     : info\n\
                     - -vv    : debug"
    )]
    pub verbose: u8,

    #[arg(
        short,
        long,
        help = "Only show errors, remove warnings",
    )]
    pub quiet: bool,
}
