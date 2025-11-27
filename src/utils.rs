use std::path::PathBuf;
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

pub fn front_matter(input: &str) -> String {
    // Must start with front matter delimiter
    if !input.starts_with("---\n") && !input.starts_with("---\r\n") {
        return String::new();
    }

    // Look for the second delimiter
    let mut lines = input.lines();

    // Skip the first "---"
    lines.next();

    let mut front = String::from("---\n");

    for line in lines {
        front.push_str(line);
        front.push('\n');

        if line.trim() == "---" {
            // Found the closing delimiter → valid FM
            return front;
        }
    }

    // No closing "---" → invalid front matter
    String::new()
}
