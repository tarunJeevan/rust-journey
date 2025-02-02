use std::process;

use clap::{ArgAction, Parser};
use mygrep::run;

#[derive(Parser, Debug)]
#[command(name = "mygrep", about = "A grep-like CLI tool")]
struct Cli {
    /// The query string to search for
    query: String,

    /// The file to search in
    file: String,

    /// Turn on for case insensitive search
    #[arg(short, long, action = ArgAction::SetTrue)]
    ignore_case: bool,

    /// Turn on to show line numbers
    #[arg(short, long, action = ArgAction::SetTrue)]
    line_numbers: bool,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    if let Err(e) = run(args.query, args.file, args.ignore_case) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
