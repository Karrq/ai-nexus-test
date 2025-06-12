use clap::Parser;
use std::fs;
use std::path::Path;

/// A simple program to copy files
#[derive(Parser, Debug)]
#[clap(author = "Your Name", version = "1.0", about = "Copies files from source to destination", long_about = None)]
struct Args {
    /// Source file path
    #[clap(short, long, value_parser)]
    source: String,

    /// Destination file path
    #[clap(short, long, value_parser)]
    destination: String,

    /// Force overwrite if the destination file exists
    #[clap(short, long, action)]
    force: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_path = Path::new(&args.source);
    let dest_path = Path::new(&args.destination);

    if !source_path.exists() {
        eprintln!("Error: Source file does not exist.");
        return Err(From::from("Source file not found"));
    }

    if dest_path.exists() && !args.force {
        eprintln!("Error: Destination file already exists. Use --force to overwrite.");
        return Err(From::from("Destination file exists"));
    }

    fs::copy(&args.source, &args.destination)?;

    println!("File copied successfully from {} to {}", args.source, args.destination);

    Ok(())
}
