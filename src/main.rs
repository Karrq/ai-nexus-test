use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1.0", about = "A simple file copy CLI", long_about = None)]
struct Args {
    /// Source file path
    #[arg(short, long)]
    source: String,

    /// Destination file path
    #[arg(short, long)]
    destination: String,

    /// Enable verbose output
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.verbose {
        println!("Source: {}", args.source);
        println!("Destination: {}", args.destination);
    }

    let source_path = Path::new(&args.source);
    let destination_path = Path::new(&args.destination);

    if !source_path.exists() {
        eprintln!("Error: Source file does not exist.");
        std::process::exit(1);
    }

    let dest_path = if destination_path.is_dir() {
        destination_path.join(source_path.file_name().unwrap())
    } else {
        destination_path.to_path_buf()
    };

    if args.verbose {
        println!("Copying {} to {}", args.source, dest_path.display());
    }

    fs::copy(&args.source, &dest_path)?;

    if args.verbose {
        println!("File copied successfully!");
    }

    Ok(())
}
