use std::fs::File;
use std::io::{BufWriter, copy};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use zip::{ZipWriter, write::SimpleFileOptions};

/// Compresses a single input file into a zip archive at the specified output path
///
/// `input_file` is the Path of the file to archive. `output_zip` is the Path of the resulting archive
///
/// Returns `()` on success and an error with context on Error
fn compress_single_file_to_zip(input_file: &Path, output_zip: &Path) -> Result<()> {
    let input_name = input_file
        .file_name()
        .context("Missing input file name")?
        .to_string_lossy();

    // Get input file contents
    let mut input =
        File::open(input_file).context(format!("Failed to open input file: {input_file:?}"))?;

    // Create the output file
    let output_file =
        File::create(output_zip).context(format!("Failed to create output file: {output_zip:?}"))?;

    // Create zip writer
    let writer = BufWriter::new(output_file);
    let mut zip = ZipWriter::new(writer);

    // Define file options
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    // Prep archive for writing
    zip.start_file(input_name.as_ref(), options)
        .context("Failed to start zip")?;

    // Copy and archive input file into archive
    copy(&mut input, &mut zip).context("Failed to copy data into zip archive")?;
    // Finish writing
    zip.finish()
        .context("Failed to finish writing zip archive")?;

    Ok(())
}

/// Compresses each input file into a zip archive at the specified output path, with a 1-to-1 mapping between input and output files
///
/// `input_files` is the list of file paths to archive. `output_files` is the list of Paths for the resulting archives
///
/// Returns `()` on success and an error with context on Error
pub fn compress_files_to_zip(input_files: &[PathBuf], output_files: &[PathBuf]) -> Result<()> {
    for (input, output) in input_files.iter().zip(output_files.iter()) {
        compress_single_file_to_zip(input, output)?;
    }

    Ok(())
}

pub fn compress_directory_to_zip_directory(in_dir: &Path, out_dir: &Path) -> Result<()> {
    Ok(())
}

pub fn compress_files_to_zip_directory(input_files: &[PathBuf], out_dir: &Path) -> Result<()> {
    for input_file in input_files {
        let input_name = input_file
            .file_name()
            .context("Missing input file name")?
            .to_string_lossy();

        // Set the archive name to be the same as the input file name
        let zip_name = format!("{input_name}.zip");
        // Get the updated output file path
        let output_path = out_dir.join(zip_name);

        compress_single_file_to_zip(input_file, &output_path)?;
    }

    Ok(())
}
