use std::fs::File;
use std::io::{BufWriter, copy};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

/// Compresses a single input file into a zip archive at the specified output path
///
/// `input_file` is the Path of the archive target. `output_zip` is the Path of the resulting archive. `permissions` is the unix file permissions
///
/// Returns `()` on success and an error with context on Error
pub fn compress_single_file_to_zip(input_file: &Path, output_zip: &Path) -> Result<()> {
    let input_name = input_file
        .file_name()
        .context("Missing input file name")?
        .to_string_lossy();

    // Get input file contents
    let mut input =
        File::open(input_file).context(format!("Failed to open input file: {input_file:?}"))?;

    // Create the output file
    let output_file = File::create(output_zip)
        .context(format!("Failed to create output file: {output_zip:?}"))?;

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

/// Recursively compresses the contents of an entire directory into a zip archive
///
/// `in_dir` is the input directory Path and `out_dir` is the output archive Path
///
/// Returns `()` on success and an error with context on Error
pub fn compress_directory_to_zip_directory(in_dir: &Path, out_dir: &Path) -> Result<()> {
    let zip_file = File::create(out_dir)
        .with_context(|| format!("Error: Failed to create zip archive: {:?}", out_dir.display()))?;
    let writer = BufWriter::new(zip_file);
    let mut zip = ZipWriter::new(writer);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let base_path = in_dir
        .canonicalize()
        .context("Error: Failed to canonicalize base input path.")?;

    for entry in WalkDir::new(in_dir) {
        let entry = entry?;
        let path = entry.path();

        // Skip the root directory
        if path == in_dir {
            continue;
        }

        let rel_path = path
            .strip_prefix(&base_path)
            .context("Error: Failed to compute relative path.")?;

        if path.is_file() {
            zip.start_file(rel_path.to_string_lossy(), options)
                .with_context(|| format!("Error: Failed to start entry file: {rel_path:?}"))?;
            let mut f = File::open(path)
                .with_context(|| format!("Error: Failed to open file: {path:?}"))?;

            copy(&mut f, &mut zip)
                .with_context(|| format!("Error: Failed to copy file into archive: {path:?}"))?;
        } else if path.is_dir() {
            // Append slash to signal directory in zip
            let dir_name = format!("{}/", rel_path.to_string_lossy());
            zip.add_directory(dir_name, options)
                .with_context(|| format!("Error: Failed to add directory: {rel_path:?}"))?;
        }
    }
    zip.finish()
        .context("Error: Failed to finalize zip archive.")?;
    Ok(())
}

/// Compresses multiple input files into a zip archive
///
/// `input_files` is a list of file paths to archive and `out_dir` is the output archive Path
///
/// Returns `()` on success and an error with context on Error
pub fn compress_files_to_zip_directory(input_files: &[PathBuf], out_dir: &Path) -> Result<()> {
    let zip_file = File::create(out_dir)
        .with_context(|| format!("Error: Failed to create zip file: {}", out_dir.display()))?;
    let writer = BufWriter::new(zip_file);
    let mut zip = ZipWriter::new(writer);
    
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    for input_file in input_files {
        let input_name = input_file
            .file_name()
            .context("Missing input file name.")?
            .to_string_lossy();
        
        zip.start_file(input_name.as_ref(), options)
            .with_context(|| format!("Error: Failed to start file entry for {input_file:?}"))?;
        
        let mut input = File::open(input_file)
            .with_context(|| format!("Error: Failed to open input file: {input_file:?}"))?;
        
        copy(&mut input, &mut zip)
            .with_context(|| format!("Error: Failed to copy file into archive: {input_file:?}"))?;
    }
    zip.finish()
        .context("Error: Failed to finalize zip archive")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::fs;
    use std::io::Write;
    
    use anyhow::Result;
    use tempfile::tempdir;
    use zip::ZipArchive;

    /// Helper function to create a temp file to compress
    ///
    /// `dir` is the directory where the file is to be created. `name` is the file name. `contents` is the file contents
    ///
    /// Returns a PathBuf containing the path to the created file
    fn create_temp_file(dir: &Path, name: &str, contents: &str) -> PathBuf {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, "{contents}").unwrap();
        file_path
    }

    /// Helper function to get all the file names in a directory
    ///
    /// `zip_path` is the directory to go through
    ///
    /// Returns a vector containing the file names
    fn extract_zip_file_names(zip_path: &Path) -> Vec<String> {
        let dir = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(dir).unwrap();

        let mut names = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            names.push(String::from(file.name()));
        }
        names
    }

    #[test]
    fn test_single_file_compression() -> Result<()> {
        // Create temp directory
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path();

        // Create dummy input file
        let input_1 = create_temp_file(input_dir, "file1.txt", "Hello world!");

        // Create output directory and compress input files into it
        let output_path = match input_1.parent() {
            Some(parent) => {
                let stem = input_1.file_stem().unwrap().to_str().unwrap_or("");
                let out_name = format!("{stem}.zip");
                PathBuf::from(parent).join(Path::new(&out_name))
            }
            None => {
                let name = format!(
                    "{}.zip",
                    input_1.file_stem().unwrap().to_str().unwrap_or("")
                );
                PathBuf::from(name)
            }
        };
        compress_single_file_to_zip(&input_1, &output_path)?;

        // Confirm that the output file was created
        assert!(output_path.exists());

        Ok(())
    }

    #[test]
    fn test_multi_file_to_archive_compression() -> Result<()> {
        // Create temp directory
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path();

        // Create dummy input files
        let input_1 = create_temp_file(input_dir, "file1.txt", "Hello world!");
        let input_2 = create_temp_file(input_dir, "file2.log", "I will be back...");

        // Create output directory and compress input files into it
        let output_zip = input_dir.join("combined.zip");
        compress_files_to_zip_directory(&[input_1.clone(), input_2.clone()], &output_zip)?;

        // Confirm that the output file was created
        assert!(output_zip.exists());

        // Confirm archive contents match what is expected
        let zip_contents = extract_zip_file_names(&output_zip);
        assert_eq!(zip_contents, vec!["file1.txt", "file2.log"]);

        Ok(())
    }

    #[test]
    fn test_directory_to_archive_compression() -> Result<()> {
        // Create temporary nested directories
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let sub_dir = input_dir.join("sub");
        fs::create_dir_all(&sub_dir)?;

        // Create dummy input files
        create_temp_file(&input_dir, "file1.txt", "Hello world!");
        create_temp_file(&sub_dir, "file2.log", "I will be back...");
        let output_zip = temp_dir.path().join("output.zip");
        compress_directory_to_zip_directory(&input_dir, &output_zip)?;

        // Confirm that the output folder was directed
        assert!(output_zip.exists());

        // Confirm archive contents match what is expected
        let zip_contents = extract_zip_file_names(&output_zip);
        assert_eq!(zip_contents, vec!["file1.txt", "sub/", "sub/file2.log"]);

        Ok(())
    }
}
