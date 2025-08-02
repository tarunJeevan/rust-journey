use snapbox::cmd::{Command, cargo_bin};

#[test]
fn zip_single_file() {
    Command::new(cargo_bin!("ezarc"));
}
