use grep_cli_tool::Config;
use std::{env, process};

use clap::{ArgAction, Parser};

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
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let clap_args = Cli::parse();
    println!("{:?}", clap_args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_cli_tool::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
