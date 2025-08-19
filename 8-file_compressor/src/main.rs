use anyhow::Result;
use clap::{Args, Parser};

use std::path::PathBuf;
use walkdir::WalkDir;
use crate::commands::{compress_to_zip, extract_zip};

mod commands;

const SUPPORTED_ARCHIVE_FORMATS: &[&str] = &["tar.gz", "tar.xz", "zip", "zip64", "bz2", "gz", "xz"];

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

    /// Output archive path. Extension must match archive format, e.g., ".zip" for ZIP format
    #[arg(num_args = 1)]
    output: PathBuf,

    /// Input file path(s)
    #[arg(num_args = 1..)]
    input: Vec<PathBuf>,

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get and validate input files
    let input_files = validate_input_files(cli.input, cli.extract);

    // Calculate compression scheme
    let schema = if let Some(s) = cli.schema {
        match s {
            Schema { zip: true, .. } => "zip",
            Schema { bzip2: true, .. } => "bzip2",
            Schema { gzip: true, .. } => "gzip",
            _ => "zip",
        }
    } else {
        "zip"
    };
    
    // Get and validate output
    let output = validate_output(cli.output, cli.extract, schema);

    if cli.extract {
        for file in &input_files {
            // FIXME: Fix file permissions for extracted files. Save file permission metadata in archive when compressing?
            extract_zip(file, &output)?;
        }
    } else {
        for file in &input_files {
            compress_to_zip(file, &output)?;
        }
    }

    Ok(())
}

/// Helper function to validate the input files passed to the program, such as checking that they exist
/// 
/// `input` is a vector of the input file paths. `extract` indicates extraction operation if true
/// 
/// Returns the vector in `input`
fn validate_input_files(input: Vec<PathBuf>, extract: bool) -> Vec<PathBuf> {
    // Ensure the input isn't empty
    if input.is_empty() {
        eprintln!("Error: No input files specified. Use --help for more information.");
        std::process::exit(1);
    }

    // Validate that all input files are valid
    for file in &input {
        // Create a recursive directory iterator using WalkDir
        for entry in WalkDir::new(file) {
            if let Ok(entry) = entry {
                if !entry.path().exists() {
                    eprintln!("{:?} is not a valid file", entry.path());
                    std::process::exit(1);
                }
            } else {
                eprintln!(
                    "Error: Encountered an error reading file: {}",
                    file.display()
                );
                std::process::exit(1);
            }
        }
    }

    // If extraction is enabled, validate input files to ensure that they are archives
    if extract {
        for file in &input {
            // Create a recursive directory iterator using WalkDir
            for entry in WalkDir::new(file) {
                if let Ok(entry) = entry {
                    // Check file extension to ensure it's an archive format
                    if let Some(ext) = entry.path().extension() {
                        if SUPPORTED_ARCHIVE_FORMATS.contains(&ext.to_str().unwrap_or("")) {
                            continue;
                        }
                    } else {
                        eprintln!(
                            "Error: File must be an archive of a supported format: {}",
                            file.display()
                        );
                        std::process::exit(1);
                    }
                } else {
                    eprintln!(
                        "Error: Encountered an error reading file: {}",
                        file.display()
                    );
                    std::process::exit(1);
                }
            }
        }
    }
    // Return input files if all checks pass
    input
}

/// Helper function to validate output path when compressing, ensuring correct file extension
/// 
/// `output` is the output file path. `extract` indicates extraction operation if true. `schema` is the compression schema
/// 
/// Returns the output path as a `PathBuf`  
fn validate_output(output: PathBuf, extract: bool, schema: &str) -> PathBuf {
    if !extract {
        match output.extension() {
            Some(ext) => {
                match schema {
                    "zip" => {
                        if ext != "zip" {
                            eprintln!("Error: Invalid file extension. Use `--help` for more info: {}", output.display());
                            std::process::exit(1);
                        }
                    },
                    "gzip" => {
                        if ext != "gz" {
                            eprintln!("Error: Invalid file extension. Use `--help` for more info: {}", output.display());
                            std::process::exit(1);
                        }
                    },
                    "bzip2" => {
                        if ext != "bz2" {
                            eprintln!("Error: Invalid file extension. Use `--help` for more info: {}", output.display());
                            std::process::exit(1);
                        }
                    },
                    _ => {
                        eprintln!("Error: Unsupported archive format: {}", output.display());
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("Error: Output must be an archive with an appropriate extension: {}", output.display());
                std::process::exit(1);
            }
        }
    }
    output
}
