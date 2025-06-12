use clap::Parser;
use std::fs;
use std::path::Path;

/// A simple file copier CLI
#[derive(Parser, Debug)]
#[clap(version, about = "Copies files from source to destination", long_about = None)]
struct Args {
    /// Source file path
    #[clap(value_parser)]
    source: String,

    /// Destination file path
    #[clap(value_parser)]
    destination: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_path = Path::new(&args.source);
    let dest_path = Path::new(&args.destination);

    if !source_path.exists() {
        eprintln!("Error: Source file '{}' does not exist.", args.source);
        std::process::exit(1);
    }

    // Perform the file copy
    match fs::copy(&args.source, &args.destination) {
        Ok(_) => {
            println!("File copied successfully from '{}' to '{}'.", args.source, args.destination);
        }
        Err(e) => {
            eprintln!("Error copying file: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
