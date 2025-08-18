use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use anyhow::{Context, Result};
use zip::ZipArchive;

pub fn extract_zip(zip_path: &Path, out_dir: &Path) -> Result<()> {
    // Create archive reader
    let file = File::open(zip_path)
        .with_context(|| format!("Failed to open zip file: {}", zip_path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("Failed to read zip file: {}", zip_path.display()))?;
    
    // Create output directory
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("Failed to create output directory: {}", out_dir.display()))?;
    
    // Iterate through archive reader and extract its contents
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .with_context(|| format!("Failed to access file at index {i}"))?;
        let out_path = out_dir.join(file.name());
        
        if file.name().ends_with("/") {
            // This is a directory entry
            fs::create_dir_all(&out_path)
                .with_context(|| format!("Failed to create directory: {}", out_path.display()))?;
        } else {
            // Create parent directories if necessary
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
            }
            
            let mut out_file = File::create(&out_path)
                .with_context(|| format!("Failed to create file: {}", out_path.display()))?;
            copy(&mut file, &mut out_file)
                .with_context(|| format!("Failed to copy file: {}", out_path.display()))?;
        }
        
        // Optionally restore permissions on Unix machines
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))
                    .with_context(|| format!("Failed to set permissions for: {}", out_path.display()))?;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    
    use std::io::Write;
    use std::path::Path;
    
    use anyhow::Result;
    use tempfile::tempdir;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    /// Helper function to create a test zip archive
    fn create_test_zip(zip_path: &Path, files: &[(&str, &str)]) -> Result<()> {
        let file  = File::create(zip_path)
            .with_context(|| format!("Failed to create test zip file: {}", zip_path.display()))?;
        let mut writer = ZipWriter::new(file);
        
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o644);
        
        for (name, content) in files {
            writer.start_file(name, options)
                .with_context(|| format!("Failed to start file: {}", name))?;
            writer.write_all(content.as_bytes())
                .with_context(|| format!("Failed to write file content: {}", content))?;
        }
        writer.finish()
            .with_context(|| format!("Failed to finalize zip: {}", zip_path.display()))?;
        
        Ok(())
    }

    #[test]
    fn test_zip_extraction() -> Result<()> {
        let tmp_dir = tempdir()?;
        let tmp_path = tmp_dir.path();
        
        // Create zip with two files
        let zip_path = tmp_path.join("test.zip");
        create_test_zip(&zip_path, &[("file1.txt", "Hello"), ("nested/file2.log", "World")])?;
        
        // Extract
        let out_dir = tmp_path.join("extract_out");
        extract_zip(&zip_path, &out_dir)?;
        
        // Validate extraction
        let file_1_path = out_dir.join("file1.txt");
        let file_2_path = out_dir.join("nested/file2.log");
        
        assert!(file_1_path.exists(), "file1.txt not found after extraction");
        assert!(file_2_path.exists(), "nested/file2.log not found after extraction");
        
        // Validate extracted file contents
        let file_1_content = fs::read_to_string(file_1_path)?;
        let file_2_content = fs::read_to_string(file_2_path)?;
        
        assert_eq!(file_1_content, "Hello");
        assert_eq!(file_2_content, "World");
        
        Ok(())
    }
}
