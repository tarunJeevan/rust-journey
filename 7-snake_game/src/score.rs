extern crate chrono;
extern crate serde;

use std::{fs, path::Path};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Leaderboard {
    pub scores: Vec<ScoreEntry>,
}

impl Leaderboard {
    /// Loads leaderboard from file
    ///
    /// `path` is the filepath to the savefile where scores are stored
    ///
    /// Returns an instance of itself containing the deserialized contents of the savefile
    pub fn load(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(leaderboard) = serde_json::from_str(&contents) {
                leaderboard
            }
        }
        Leaderboard::default()
    }

    /// Save current leaderboard to a savefile, overwriting its contents if it already exists
    ///
    /// `path` is the path to the savefile
    ///
    /// Returns an IO Result containing the Unit type on success and an IO error on failure
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)
    }

    pub fn try_add_score(&mut self, name: &str, score: u32) -> bool {
        let entry = ScoreEntry {
            name: name.chars().take(3).collect::<String>().to_uppercase(),
            score,
            date: Utc::now(),
        };

        // Logic to determine how to update leaderboard
        // Update if there are less than 10 scores or new score > last score on leaderboard
        if self.scores.len() < 10 || score > self.scores.last().unwrap().score {
            // Add score to leaderboard
            self.scores.push(entry);
            // Sort scores in descending order
            self.scores.sort_by(|a, b| b.score.cmp(&a.score));
            // Ensure leaderboard only has 10 scores
            if self.scores.len() > 10 {
                self.scores.pop();
            }
            // Update successful so return true
            true
        } else {
            // Leaderboard is full AND new score isn't high enough so don't add
            false
        }
    }
}
