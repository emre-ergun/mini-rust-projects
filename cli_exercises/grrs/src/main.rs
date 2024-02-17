use clap::Parser;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path).expect("Could not read the file");
    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            let i = i + 1;
            println!("{i}. {line}");
        }
    }

    // alternative to "read_to_string" function
    let f = File::open(args.path).expect("File could not be opened!");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    while let Ok(len) = reader.read_line(&mut line) {
        if line.contains(&args.pattern) {
            line = String::from(line.trim());
            println!("{line}");
        }
        line = String::from("");
        if len == 0 {
            break;
        }
    }

}
