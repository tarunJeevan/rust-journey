use std::{env, error::Error, fs};

use colored::Colorize;
use regex::Regex;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments. Expected at least 2 arguments.");
        }

        // TODO: Make order of command line arguments more flexible

        // NOTE: Environment variable approach to ignore case
        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        // FIXME: Prettify the following into more clean and idiomatic code
        // No ignore case flag
        if args.len() == 3 {
            let query = args[1].clone();
            let filepath = args[2].clone();

            return Ok(Config {
                query,
                filepath,
                ignore_case: ignore_case_env,
            });
        }
        // There is an ignore case flag
        else if args.len() == 4 {
            let query = args[2].clone();
            let filepath = args[3].clone();

            // NOTE: Command line argument approach to ignore case
            let ignore_case_cli = matches!(args[1].clone().as_str(), "--ignore-case" | "-I");

            return Ok(Config {
                query,
                filepath,
                ignore_case: ignore_case_cli || ignore_case_env,
            });
        }

        Err("Command should be formatted 'minigrep [-I | --ignore-case] query filepath'")
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filepath)?;

    // TODO: Add line numbers to each line

    let results = if config.ignore_case {
        search_ci(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Regular search
pub fn search(query: &str, content: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            // Create modified line by stylizing matched portions of original line
            let mod_line = line.replace(query, &query.red().bold().to_string());
            result.push(mod_line);
        }
    }
    result
}

// Case insensitive search
pub fn search_ci(query: &str, content: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    // Create regex for case insensitive pattern matching
    let pattern = format!(r"(?i){}", regex::escape(query));
    let re = Regex::new(&pattern).unwrap();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            // Create mutable string to hold modified line
            let mut mod_line = line.to_string();

            // Find all matches in line
            let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();

            // For every match, replace the matched portion with a colored version
            for n in matches.iter() {
                mod_line = re.replace(line, n.red().bold().to_string()).to_string();
            }
            result.push(mod_line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let matched_string =
            "safe, fast, productive.".replace("duct", &"duct".red().bold().to_string());
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![matched_string], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                "Rust:".replace("Rust", &"Rust".red().bold().to_string()),
                "Trust me.".replace("rust", &"rust".red().bold().to_string())
            ],
            search_ci(query, content)
        )
    }
}
