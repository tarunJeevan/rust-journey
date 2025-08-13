use anyhow::Result;
use clap::{Args, Parser};
use std::fs::DirBuilder; // NOTE: Use context() for more detailed error messages

use std::path::PathBuf;

mod commands;

const SUPPORTED_ARCHIVE_FORMATS: &[&str] = &["tar.gz", "tar.xz", "zip", "bz2", "gz"];

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

    /// Allow directory as input/output
    #[arg(short, long, num_args = 1)]
    directory: bool,

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
    let _extract = cli.extract;

    // Get and validate input files
    let input_files = get_input_files(cli.input, cli.extract, cli.directory);

    // Get and validate output files
    let output = get_output_files(&input_files, cli.output, cli.directory);

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

fn get_input_files(input: Vec<PathBuf>, extract: bool, directory: bool) -> Vec<PathBuf> {
    let mut input_files: Vec<PathBuf> = Vec::new();
    // If `directory` is true, check if the input is a valid directory
    if directory {
        if input.len() == 1 {
            if !input[0].is_dir() {
                eprintln!("Error: Input must be a valid directory.");
                std::process::exit(1);
            }
            // Walk through the directory and add all files found to input_files
            if let Ok(entries) = input[0].read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        input_files.push(entry.path());
                    } else {
                        eprintln!("Error: Encountered an IO error reading file: {entry:?}.");
                        continue;
                    }
                }
            }
        } else {
            input_files.extend(input);
        }
    } else {
        // Exit with error if the user passes in a directory without the `-d` flag
        if input.len() == 1 && input[0].is_dir() {
            eprintln!(
                "Error: Invalid input. To pass a directory as input, use the '-d' flag. Use '--help' for more info."
            );
            std::process::exit(1);
        }
        // Add files from input into input_files
        input_files.extend(input);
    }

    // If `extract` is true, check file extensions to make sure that all files are archives
    if extract {
        for file in &input_files {
            if file.is_dir() {
                // Walk through the directory and add all files found to input_files
                if let Ok(entries) = input_files[0].read_dir() {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(ext) = entry.path().extension() {
                                if SUPPORTED_ARCHIVE_FORMATS.contains(&ext.to_str().unwrap_or("")) {
                                    continue;
                                }
                            } else {
                                eprintln!("Error: Invalid file path: {}", file.display());
                                std::process::exit(1);
                            }
                        } else {
                            eprintln!("Error: Encountered an IO error reading file: {entry:?}.");
                            continue;
                        }
                    }
                }
            } else {
                match file.extension() {
                    Some(ext) => {
                        if SUPPORTED_ARCHIVE_FORMATS.contains(&ext.to_str().unwrap_or("")) {
                            continue;
                        }
                    }
                    None => {
                        eprintln!("Error: Invalid file path: {}", file.display());
                        std::process::exit(1);
                    }
                }
            }
        }
        input_files
    } else {
        input_files
    }
}

fn get_output_files(input: &[PathBuf], output: Vec<PathBuf>, directory: bool) -> OutputMode {
    // NOTE: The `directory` flag might have been to extract an input directory into individual output files
    if directory {
        if output.len() == 1 {
            let out = output[0].clone();
            // If the provided path is not a valid directory, try to create one there
            if !out.is_dir() {
                if let Err(error) = DirBuilder::new().recursive(true).create(&out) {
                    eprintln!(
                        "Error: Failed to create the specified output directory with error: {error}"
                    );
                    std::process::exit(1);
                }
            }
            // Return output directory
            OutputMode::Directory(out)
        } else {
            // Exit with error if the number of input files don't equal the number of output files
            if output.len() != input.len() {
                eprintln!("Error: Expected a single output directory.");
                std::process::exit(1);
            }
            // Return output files
            OutputMode::Files(output)
        }
    } else {
        let out = output.clone();
        // Exit with error if the number of output files doesn't match the number of input files
        if out.len() != input.len() {
            eprintln!("Error: Number of output files must equal the number of input files.");
            std::process::exit(1);
        }
        // Return output files
        OutputMode::Files(out)
    }
}
