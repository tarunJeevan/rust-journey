// use colored::Colorize;
use std::{env, error::Error, fs};

use colored::Colorize;

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

    // TODO: Stylize matched portion of line with underline or color
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

pub fn search(query: &str, content: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            let mod_line = line.replace(query, &query.red().bold().to_string());
            result.push(mod_line);
        }
    }
    result
}

pub fn search_ci(query: &str, content: &str) -> Vec<String> {
    let mod_query = query.to_lowercase();
    let mut result: Vec<String> = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&mod_query) {
            // FIXME: String is only colored if query perfectly matches line text. Ex: If query is 'to' then 'To' in line isn't colored
            let mod_line = line.replace(query, &query.red().bold().to_string());
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
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_ci(query, content))
    }
}
