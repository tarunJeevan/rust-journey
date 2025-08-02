mod compress_tests;
mod extract_tests;

use compress_tests::*;
use extract_tests::*;

use snapbox::cmd::{Command, cargo_bin};

#[test]
/// Tests that the program succeeds when provided a valid number of file paths as input and output arguments
fn success_with_output_flag() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-o", "out1.zip", "out2.zip"])
        .assert()
        .success();
}

#[test]
/// Tests that the program succeeds when providing a valid directory path with the `-d` flag
fn success_with_directory_flag() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .assert()
        .success();
}

#[test]
/// Tests that the program succeeds when provided the `-x` flag to indicate file extraction
fn success_with_extract_flag() {
    Command::new(cargo_bin!("ezarc"))
        .arg("-x")
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .assert()
        .success();
}

#[test]
/// Tests that the program fails when an unequal number of file paths are provided as input and output arguments
fn failure_with_output_flag() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-o", "out1.zip"])
        .assert()
        .failure()
        .code(1)
        .stderr_eq("Error: [..]");
}

#[test]
/// Tests that the program fails when a non-directory path is provided with the `-d` flag
fn failure_with_directory_flag() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "out1.zip"])
        .assert()
        .failure()
        .code(1)
        .stderr_eq("Error: [..]");
}

#[test]
/// Tests that the `-d` and -o flags cannot be used simultaneously
fn failure_with_output_and_directory_flags() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .args(&["-o", "out2.zip"])
        .assert()
        .failure();
}
