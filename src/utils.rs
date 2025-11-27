use directories_next::ProjectDirs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};
use glob::glob;

pub fn init_logging(quiet: bool, level: u8) {
    let filter = if quiet {
        "error"
    } else {
        match level {
            0 => "warn",
            1 => "info",
            _ => "debug",
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .without_time()
        .with_target(false)
        .init();
}

pub fn expand_globs(patterns: &Vec<String>) -> Result<Vec<PathBuf>, glob::GlobError> {
    let mut files = Vec::new();

    for pattern in patterns {
        for entry in glob(pattern).expect("Invalid glob pattern") {
            let path = entry?;
            files.push(path);
        }
    }

    Ok(files)
}

