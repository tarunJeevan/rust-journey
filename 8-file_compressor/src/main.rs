use anyhow::Result;
use clap::{Args, Parser}; // NOTE: Use context() for more detailed error messages

use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(
    version,
    about = "A CLI utility for archiving and extracting files. It archives by default but can also be used to extract archives.",
    long_about = None,
)]
struct Cli {
    /// Extract provided input file(s)
    #[arg(short = 'x', long)]
    extract: bool,

    /// Input file path(s)
    #[arg(num_args = 1..)]
    input: Vec<PathBuf>,

    /// Output file path(s)
    #[arg(short, long, num_args = 1..)]
    output: Vec<PathBuf>,

    /// Output directory
    #[arg(short, long, num_args = 1, conflicts_with = "output")]
    directory: Vec<PathBuf>,

    #[command(flatten)]
    schema: Option<Schema>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct Schema {
    /// Use ZIP compression (Default)
    #[arg(long)]
    zip: bool,

    /// Use BZIP2 compression
    #[arg(long)]
    bzip2: bool,

    /// Use GZIP compression
    #[arg(long)]
    gzip: bool,
}

// Enum to store which output mode is active
enum OutputMode {
    Directory(PathBuf),
    Files(Vec<PathBuf>),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get flag to determine extraction or compression
    let extract = cli.extract;

    // Get and validate input files
    let input_files = if extract {
        // Ensure that all input file extensions are compressed formats
        for file in &cli.input {
            match file.extension() {
                Some(ext) => {
                    if ["zip", "bz2", "gz"].contains(&ext.to_str().unwrap_or("")) {
                        continue;
                    }
                }
                None => {
                    eprintln!("Error: Invalid file path: {}", file.display());
                    std::process::exit(1);
                }
            }
        }
        cli.input
    } else {
        cli.input
    };

    // Get and validate output files
    let output = if !cli.directory.is_empty() {
        let out = cli.directory[0].clone();
        // Exit with error if the provided path is not a directory
        if !out.is_dir() {
            eprintln!("Error: Value of --directory must be a valid path to a directory.");
            std::process::exit(1);
        }
        // TODO: Instead of erroring out, create a new directory if the provided one doesn't exist
        // Return output directory
        OutputMode::Directory(out)
    } else if !cli.output.is_empty() {
        let out = cli.output.clone();
        // Exit with error if the number of output files doesn't match the number of input files
        if out.len() != input_files.len() {
            eprintln!("Error: Number of output files must equal the number of input files.");
            std::process::exit(1);
        }
        // Return output files
        OutputMode::Files(cli.output.clone())
    } else {
        // Exit with error if neither output files nor output directory is specified
        eprintln!("Error: Must specify output via --output or --directory");
        std::process::exit(1);
    };

    // Calculate compression scheme
    let _schema = if let Some(s) = cli.schema {
        match s {
            Schema { zip: true, .. } => "zip",
            Schema { bzip2: true, .. } => "bzip2",
            Schema { gzip: true, .. } => "gzip",
            _ => "zip",
        }
    } else {
        "zip"
    };

    match output {
        OutputMode::Files(files) => {
            // NOTE: Placeholder
            println!("Output files: {files:?}");
        }
        OutputMode::Directory(dir) => {
            // NOTE: Placeholder
            println!("Output directory: {dir:?}");
        }
    }

    Ok(())
}
