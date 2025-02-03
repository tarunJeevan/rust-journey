#![allow(clippy::collapsible_else_if)]
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use colored::Colorize;
use regex::Regex;

pub fn run(
    query: String,
    filepath: String,
    ignore_case: bool,
    line_numbers: bool,
) -> Result<(), Box<dyn Error>> {
    // Get content from file
    let content = read_lines(filepath)?;

    // Get results from search function
    let results = search(query, content, ignore_case, line_numbers);

    // Print search results
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Custom function to read lines from file
fn read_lines<P>(filename: P) -> Result<io::BufReader<File>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

// Search function
fn search<R: BufRead>(
    query: String,
    content: R,
    ignore_case: bool,
    line_numbers: bool,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for (line_number, line) in content.lines().enumerate() {
        if let Ok(line_content) = line {
            // Pattern matching
            let matched_line: String = if ignore_case {
                // Case insensitive matching

                // Create regex for case insensitive pattern matching
                let pattern = format!(r"(?i){}", regex::escape(&query));
                let re = Regex::new(&pattern).unwrap();

                if line_content.to_lowercase().contains(&query.to_lowercase()) {
                    // Create mutable string to hold modified line
                    let mut mod_line = line_content.clone();

                    // Find all matches in line
                    let matches: Vec<&str> =
                        re.find_iter(&line_content).map(|m| m.as_str()).collect();

                    // For every match, replace the matched portion with a colored version
                    for n in matches.iter() {
                        mod_line = re
                            .replace(&line_content, n.red().bold().to_string())
                            .to_string();
                    }
                    mod_line
                } else {
                    String::new()
                }
            } else {
                // Case sensitive matching
                if line_content.contains(&query) {
                    line_content.replace(&query, &query.red().bold().to_string())
                } else {
                    String::new()
                }
            };

            // Push matched lines to result vector. Set line numbers if flag is enabled
            if !matched_line.is_empty() {
                if line_numbers {
                    result.push(format!("{}: {}", line_number, matched_line));
                } else {
                    result.push(matched_line);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let ignore_case = false;
        let line_numbers = false;
        let query = "duct".to_string();
        let matched_string =
            "safe, fast, productive.".replace("duct", &"duct".red().bold().to_string());
        let content = read_lines("test.txt").unwrap();

        assert_eq!(
            vec![matched_string],
            search(query, content, ignore_case, line_numbers)
        );
    }

    #[test]
    fn case_insensitive() {
        let ignore_case = true;
        let line_numbers = false;
        let query = "rUSt".to_string();
        let content = read_lines("test.txt").unwrap();

        assert_eq!(
            vec![
                "Rust:".replace("Rust", &"Rust".red().bold().to_string()),
                "Trust me.".replace("rust", &"rust".red().bold().to_string())
            ],
            search(query, content, ignore_case, line_numbers)
        )
    }
}
