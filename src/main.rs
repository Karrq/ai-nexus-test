use clap::Parser;
use std::fs;
use std::path::Path;

/// A simple program to copy files
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "Copies files from source to destination", long_about = None)]
struct Args {
    /// Source file path
    #[arg(short, long)]
    source: String,

    /// Destination file path
    #[arg(short, long)]
    destination: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_path = Path::new(&args.source);
    let destination_path = Path::new(&args.destination);

    // Check if the source file exists
    if !source_path.exists() {
        return Err(From::from(format!("Source file \"{}\" does not exist", args.source)));
    }

    // Perform the file copy operation
    fs::copy(source_path, destination_path).map_err(|e| {
        format!("Error copying file: {}", e)
    })?;

    println!("File copied successfully from \"{}\" to \"{}\"", args.source, args.destination);

    Ok(())
}
