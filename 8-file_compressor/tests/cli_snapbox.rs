use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use snapbox::cmd::{Command, cargo_bin};
use tempfile::{tempdir, TempDir};

/// Helper function to create a temp file for testing
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

#[test]
/// Tests that the program succeeds when provided multiple input files
fn success_with_multiple_files() -> Result<()> {
    let tmp_dir = tempdir()?;
    let input_dir = tmp_dir.path();

    let input_1 = create_temp_file(input_dir, "test_input_1.txt", "Hello!");
    let input_2 = create_temp_file(input_dir, "test_input_2.txt", "World!");

    Command::new(cargo_bin!("ezarc"))
        .arg("out")
        .args([
            input_1.canonicalize()?.display().to_string(),
            input_2.canonicalize()?.display().to_string(),
        ])
        .assert()
        .success();

    Ok(())
}

#[test]
/// Tests that the program succeeds when provided directories as input
fn success_with_input_directories() -> Result<()> {
    let tmp_dir_1 = TempDir::with_prefix("test_input_dir_1")?;
    let tmp_dir_2 = TempDir::with_prefix("test_input_dir_2")?;
    
    Command::new(cargo_bin!("ezarc"))
        .arg("out")
        .args([tmp_dir_1.path().canonicalize()?.display().to_string(), tmp_dir_2.path().canonicalize()?.display().to_string()])
        .assert()
        .success();
    
    Ok(())
}

#[test]
/// Tests that the program succeeds when provided input arguments of multiple types, such as files and directories
fn success_with_multiple_args() -> Result<()> {
    let tmp_dir_1 = TempDir::with_prefix("test_input_dir")?;
    let tmp_dir_2 = tempdir()?;
    let input_dir = tmp_dir_2.path();

    let input_1 = create_temp_file(input_dir, "test_input_1.txt", "Hello!");
    
    Command::new(cargo_bin!("ezarc"))
        .arg("out")
        .args([input_1.canonicalize()?.display().to_string(), tmp_dir_1.path().canonicalize()?.display().to_string()])
        .assert()
        .success();
    
    Ok(())
}

#[test]
/// Tests that the program fails when provided invalid input arguments, such as non-existent files or directories
fn failure_with_invalid_args() {
    Command::new(cargo_bin!("ezarc"))
        .arg("out")
        .args(["invalid_input_1.txt", "invalid_input_2.txt"])
        .assert()
        .failure()
        .code(1)
        .stderr_eq("Error: [..]");
}

#[test]
/// Tests that the program succeeds when provided the `-x` flag to indicate file extraction
fn success_with_extract_flag() -> Result<()> {
    let tmp_dir_2 = tempdir()?;
    let input_dir = tmp_dir_2.path();

    let input_1 = create_temp_file(input_dir, "test_input_1.zip", "Hello!");

    Command::new(cargo_bin!("ezarc"))
        .arg("-x")
        .arg("out")
        .args([input_1.canonicalize()?.display().to_string()])
        .assert()
        .success();
    
    Ok(())
}

#[test]
/// Tests that the program fails when provided the `-x` flag to indicate file extraction but has invalid input arguments
fn failure_with_extract_flag() -> Result<()> {
    let tmp_dir_1 = TempDir::with_prefix("test_input_dir")?;
    let tmp_dir_2 = tempdir()?;
    let input_dir = tmp_dir_2.path();

    // File has a `.txt` extension instead of an archive format
    let input_1 = create_temp_file(input_dir, "test_input_1.txt", "Hello!");

    Command::new(cargo_bin!("ezarc"))
        .arg("-x")
        .arg("out")
        .args([input_1.canonicalize()?.display().to_string(), tmp_dir_1.path().canonicalize()?.display().to_string()])
        .assert()
        .failure()
        .code(1)
        .stderr_eq("Error: [..]");

    Ok(())
}
