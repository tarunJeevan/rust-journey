use std::{error::Error, fs};

use colored::Colorize;
use regex::Regex;

pub fn run(
    query: String,
    filepath: String,
    ignore_case: bool,
    // line_numbers: bool,
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filepath)?;

    let results = if ignore_case {
        search_ci(&query, &content)
    } else {
        search(&query, &content)
    };

    // TODO: Add line numbers if associated bool is true
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
