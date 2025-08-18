use std::fs::{File, OpenOptions, create_dir_all};
use std::io::copy;
use std::path::Path;

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

/// Helper function to safely open a zip for appending or create a new one
///
/// `path` is the file to open
///
/// Returns a writer for the file on Ok and an error with context on Err 
fn open_zip_for_write(path: &Path) -> Result<ZipWriter<File>> {
    if path.exists() {
        // Try to validate archive
        let f = File::open(path)?;
        match ZipArchive::new(f) {
            Ok(_) => {
                // Valid archive -> open for append
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(path)
                    .with_context(|| format!("Failed to open existing zip file: {}", path.display()))?;
                Ok(ZipWriter::new_append(file)?)
            }
            Err(_) => {
                // Corrupted archive -> recreate
                let file = File::create(path)
                    .with_context(|| format!("Failed to recreate corrupted zip file: {}", path.display()))?;
                Ok(ZipWriter::new(file))
            }
        }
    } else {
        // Create new archive
        let file = File::create(path)
            .with_context(|| format!("Failed to create new file: {}", path.display()))?;
        Ok(ZipWriter::new(file))
    }
}

/// Compresses all files/directories in `input_path` into `out_dir`, preserving existing file structure
///
/// `input_path` is the file/directory to be compressed. `out_dir` is the output location
///
/// Returns `()` on success and an error with context on Error
pub fn compress_to_zip(input_path: &Path, out_zip: &Path) -> Result<()> {
    // Ensure parent directory for output zip path exists
    if let Some(parent) = out_zip.parent() {
        create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }
    
    let mut writer = open_zip_for_write(out_zip)?;

    // Create zip file options
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // File compression logic
    if input_path.is_file() {
        // Add the single file to the archive
        let file_name = input_path
            .file_name()
            .context("Failed to get input file name")?
            .to_string_lossy()
            .into_owned();

        writer.start_file(file_name, options)?;
        let mut input_file = File::open(input_path)?;
        copy(&mut input_file, &mut writer)?;
    } else if input_path.is_dir() {
        // Recursively add the entire directory to the archive
        let base = input_path
            .canonicalize()
            .context("Failed to canonicalize base directory of input file")?;

        for entry in WalkDir::new(&base) {
            let entry = entry?;
            let path = entry.path();
            let rel_path = path
                .strip_prefix(&base)
                .context("Failed to compute relative path from base directory")?;

            // Skip the root directory
            if rel_path.as_os_str().is_empty() {
                continue;
            }

            if path.is_file() {
                writer.start_file(rel_path.to_string_lossy(), options)?;
                let mut input_file = File::open(path)?;
                copy(&mut input_file, &mut writer)?;
            } else if path.is_dir() {
                let dir_name = format!("{}/", rel_path.to_string_lossy());
                writer.add_directory(dir_name, options)?;
            }
        }
    } else {
        bail!(
            "Input is neither file nor directory: {}",
            input_path.display()
        );
    }

    // Finalize archive
    writer.finish().context("Failed to finalize zip archive")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::create_dir_all;
    use std::io::Write;
    use std::path::PathBuf;

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
    fn extract_zip_file_names(zip_path: &Path) -> Result<Vec<String>> {
        let dir = File::open(zip_path)?;
        let mut archive = ZipArchive::new(dir)?;

        let mut names = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            names.push(file.name().to_string());
        }
        Ok(names)
    }

    #[test]
    fn test_zip_single_file() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path();

        let input_file = create_temp_file(input_dir, "test.txt", "Hello World!");
        let output_zip = input_dir.join("zip_out.zip");

        compress_to_zip(&input_file, &output_zip)?;

        assert!(output_zip.exists());

        let contents = extract_zip_file_names(&output_zip)?;
        assert_eq!(contents, vec!["test.txt"]);

        Ok(())
    }

    #[test]
    fn test_zip_directory() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("test");
        let sub_dir = input_dir.join("sub");

        create_dir_all(&sub_dir)?;
        create_temp_file(&input_dir, "test.txt", "Hello World!");
        create_temp_file(&sub_dir, "input.txt", "Hello from sub-directory!");
        let output_zip = input_dir.join("zip_out.zip");

        compress_to_zip(&input_dir, &output_zip)?;

        assert!(output_zip.exists());

        // Get and sort contents to ensure a particular order to match against
        let mut contents = extract_zip_file_names(&output_zip)?;
        contents.sort();
        assert_eq!(contents, vec!["sub/", "sub/input.txt", "test.txt", "zip_out.zip"]);

        Ok(())
    }

    #[test]
    fn test_append_to_zip() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path();

        let file1 = create_temp_file(input_dir, "test1.txt", "Hello World!");
        let file2 = create_temp_file(input_dir, "test2.txt", "I will be back...");

        let output_zip = input_dir.join("append_zip_out.zip");

        // Compress file1 first
        compress_to_zip(&file1, &output_zip)?;
        // Compress file2 to same output_zip
        compress_to_zip(&file2, &output_zip)?;

        // Get and sort contents to ensure a particular order to match against
        let mut contents = extract_zip_file_names(&output_zip)?;
        contents.sort();
        assert_eq!(contents, vec!["test1.txt", "test2.txt"]);

        Ok(())
    }
}
