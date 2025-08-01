use snapbox::cmd::{cargo_bin, Command};

#[test]
/// Tests that the program succeeds when provided a valid number of file paths as input and output arguments 
fn compress_success_with_output() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-o", "out1.zip", "out2.zip"])
        .assert()
        .success();
}

#[test]
/// Tests that the program succeeds when providing a valid directory path with the `-d` flag
fn compress_success_with_directory() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .assert()
        .success();
}

#[test]
/// Tests that the program succeeds when provided the `-x` flag to indicate file extraction
fn extract_success() {
    Command::new(cargo_bin!("ezarc"))
        .arg("-x")
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .assert()
        .success();
}

#[test]
/// Tests that the program fails when an unequal number of file paths are provided as input and output arguments
fn compress_failure_with_output() {
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
fn compress_failure_with_directory() {
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
fn output_and_directory_failure() {
    Command::new(cargo_bin!("ezarc"))
        .args(&["in1.txt", "in2.txt"])
        .args(&["-d", "./"])
        .args(&["-o", "out2.zip"])
        .assert()
        .failure();
}