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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Leaderboard {
    pub scores: Vec<ScoreEntry>,
}

impl Leaderboard {
    /// Loads leaderboard from file
    ///
    /// `path` is the filepath to the savefile where scores are stored
    ///
    /// Returns an instance of itself containing the deserialized contents of the savefile or a default instance
    pub fn load(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            return serde_json::from_str(&contents).unwrap_or(Leaderboard::default());
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

    /// Check if a given score can be added to the leaderboard
    ///
    /// `score` is the score to compare against the leaderboard
    ///
    /// Returns `true` if score can be added to leaderboard and `false` otherwise
    pub fn can_add_score(&self, score: u32) -> bool {
        // Check if score is eligible to go on leaderboard
        if self.scores.len() < 10 || score > self.scores.last().unwrap().score {
            true
        } else {
            // Leaderboard is full AND new score isn't high enough so don't add
            false
        }
    }

    /// Adds a new score entry to the leaderboard
    ///
    /// `name` is the 3-letter name of the entry and `score` is the attached score
    pub fn add_score(&mut self, name: &str, score: u32) {
        // Create score entry
        let entry = ScoreEntry {
            name: name.to_uppercase(),
            score,
            date: Utc::now(), // NOTE: Change method depending on date format
        };

        // Add score to leaderboard
        self.scores.push(entry);

        // Sort scores in descending order
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));

        // Ensure leaderboard only has 10 scores
        if self.scores.len() > 10 {
            self.scores.pop();
        }
    }
}
