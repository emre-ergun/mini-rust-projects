use clap::Parser;
use std::fs;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read the file: {}", args.path.display()))?;

    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            let i = i + 1;
            println!("{i}. {line}");
        }
    }

    Ok(())
}
